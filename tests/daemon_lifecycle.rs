use std::time::Duration;

/// Verify that kill_process_group kills the entire process group,
/// including grandchild processes (simulating signal-cli spawning Java).
#[tokio::test]
async fn drop_kills_entire_process_group() {
    let marker = format!("/tmp/signal-cli-api-test-{}", std::process::id());

    // Spawn a shell in its own process group (via setsid, matching daemon.rs behavior).
    // The shell spawns a backgrounded sleep (simulating signal-cli → Java).
    let mut child = unsafe {
        tokio::process::Command::new("sh")
            .arg("-c")
            .arg(format!(
                "sleep 300 & echo $! > {marker}.child; echo $$ > {marker}.parent; wait"
            ))
            .kill_on_drop(true)
            .pre_exec(|| {
                let ret = libc::setsid();
                if ret == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            })
            .spawn()
            .unwrap()
    };

    // Wait for PID files to appear
    tokio::time::sleep(Duration::from_millis(500)).await;

    let parent_pid: i32 = std::fs::read_to_string(format!("{marker}.parent"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let child_pid: i32 = std::fs::read_to_string(format!("{marker}.child"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Both should be alive
    assert!(is_alive(parent_pid), "parent should be alive before drop");
    assert!(is_alive(child_pid), "child should be alive before drop");

    // Use the process-group kill logic from daemon.rs
    let pid = child.id().expect("child should have a PID") as i32;
    signal_cli_api::daemon::kill_process_group(pid);

    // Give time for processes to die
    tokio::time::sleep(Duration::from_millis(500)).await;
    let _ = child.wait().await; // reap zombie

    // Both should be dead
    assert!(
        !is_alive(parent_pid),
        "parent should be dead after group kill"
    );
    assert!(
        !is_alive(child_pid),
        "grandchild should be dead after group kill"
    );

    // Cleanup
    let _ = std::fs::remove_file(format!("{marker}.parent"));
    let _ = std::fs::remove_file(format!("{marker}.child"));
}

/// Verify that without process group kill, the grandchild SURVIVES.
/// This test documents the bug we're fixing.
#[tokio::test]
async fn without_group_kill_grandchild_survives() {
    let marker = format!(
        "/tmp/signal-cli-api-test-nogroupkill-{}",
        std::process::id()
    );

    // Spawn WITHOUT setsid — child inherits our process group.
    // This means kill_on_drop only kills the shell, not the sleep.
    let mut child = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sleep 300 & echo $! > {marker}.child; echo $$ > {marker}.parent; wait"
        ))
        .kill_on_drop(true)
        .spawn()
        .unwrap();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let parent_pid: i32 = std::fs::read_to_string(format!("{marker}.parent"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let child_pid: i32 = std::fs::read_to_string(format!("{marker}.child"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Kill ONLY the direct child (the old behavior)
    child.start_kill().unwrap();
    let _ = child.wait().await;

    tokio::time::sleep(Duration::from_millis(500)).await;

    assert!(
        !is_alive(parent_pid),
        "parent should be dead after direct kill"
    );
    // The grandchild SURVIVES — this is the bug!
    assert!(
        is_alive(child_pid),
        "grandchild survives direct kill (this is the bug)"
    );

    // Cleanup: kill the orphan we just created
    unsafe {
        libc::kill(child_pid, libc::SIGKILL);
    }
    let _ = std::fs::remove_file(format!("{marker}.parent"));
    let _ = std::fs::remove_file(format!("{marker}.child"));
}

fn is_alive(pid: i32) -> bool {
    unsafe { libc::kill(pid, 0) == 0 }
}
