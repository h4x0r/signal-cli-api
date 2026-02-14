use std::time::Duration;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};

/// A managed signal-cli daemon child process.
/// Kills the entire process group on drop.
pub struct ManagedDaemon {
    child: Child,
    pid: i32,
    pub addr: String,
}

impl Drop for ManagedDaemon {
    fn drop(&mut self) {
        kill_process_group(self.pid);
        let _ = self.child.start_kill(); // belt and braces
    }
}

/// Kill an entire process group: SIGTERM first, then SIGKILL after 2s.
/// Public so integration tests can call it directly.
pub fn kill_process_group(pid: i32) {
    // Send SIGTERM to the process group (negative PID = group)
    unsafe {
        libc::kill(-pid, libc::SIGTERM);
    }
    // Give processes time to exit gracefully
    std::thread::sleep(Duration::from_secs(2));
    // Escalate to SIGKILL for anything still alive
    unsafe {
        libc::kill(-pid, libc::SIGKILL);
    }
}

/// Find signal-cli on $PATH.
fn find_signal_cli() -> anyhow::Result<String> {
    let output = std::process::Command::new("which")
        .arg("signal-cli")
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        anyhow::bail!(
            "signal-cli not found on $PATH. Install it or use --signal-cli <addr> to connect to an existing daemon"
        )
    }
}

/// Spawn signal-cli daemon on a random available port and wait until it's ready.
/// The child is placed in its own process group via setsid() so that
/// dropping ManagedDaemon kills the entire tree (including Java grandchildren).
pub async fn spawn() -> anyhow::Result<ManagedDaemon> {
    let bin = find_signal_cli()?;
    tracing::info!("Found signal-cli at {bin}");

    // Grab a random available port by binding then releasing.
    let port = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        listener.local_addr()?.port()
    };
    let addr = format!("127.0.0.1:{port}");

    tracing::info!("Spawning signal-cli daemon on {addr}");
    // SAFETY: pre_exec runs in the forked child before exec. setsid() is
    // async-signal-safe and creates a new session/process group, which lets
    // us kill the entire group (including Java grandchildren) on shutdown.
    let mut child = unsafe {
        Command::new(&bin)
            .args(["daemon", "--tcp", &addr])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .pre_exec(|| {
                let ret = libc::setsid();
                if ret == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            })
            .spawn()?
    };

    let pid = child.id().expect("child should have a PID") as i32;

    // Poll until the port is accepting connections (max ~30s — JVM startup is slow).
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    loop {
        if tokio::time::Instant::now() > deadline {
            // Try to read stderr for diagnostics before bailing.
            if let Some(stderr) = child.stderr.take() {
                use tokio::io::AsyncReadExt;
                let mut buf = vec![0u8; 4096];
                let mut stderr = stderr;
                if let Ok(n) = stderr.read(&mut buf).await {
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    anyhow::bail!("signal-cli daemon failed to start within 30s. stderr: {msg}");
                }
            }
            anyhow::bail!("signal-cli daemon failed to start within 30 seconds");
        }
        // Check if the child exited early (crash/error).
        if let Some(status) = child.try_wait()? {
            let mut msg = format!("signal-cli exited with {status}");
            if let Some(mut stderr) = child.stderr.take() {
                use tokio::io::AsyncReadExt;
                let mut buf = String::new();
                let _ = stderr.read_to_string(&mut buf).await;
                if !buf.is_empty() {
                    msg.push_str(": ");
                    msg.push_str(buf.trim());
                }
            }
            anyhow::bail!(msg);
        }
        match TcpStream::connect(&addr).await {
            Ok(_) => break,
            Err(_) => tokio::time::sleep(Duration::from_millis(200)).await,
        }
    }
    tracing::info!("signal-cli daemon ready on {addr}");

    Ok(ManagedDaemon { child, pid, addr })
}
