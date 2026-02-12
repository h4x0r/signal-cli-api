use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock, oneshot};

pub type RpcResponse = serde_json::Value;

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Metrics {
    pub messages_sent: AtomicU64,
    pub messages_received: AtomicU64,
    pub rpc_calls: AtomicU64,
    pub rpc_errors: AtomicU64,
    pub ws_clients: AtomicU64,
}

impl Metrics {
    pub fn inc_sent(&self) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
    }
    pub fn inc_received(&self) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
    }
    pub fn inc_rpc(&self) {
        self.rpc_calls.fetch_add(1, Ordering::Relaxed);
    }
    pub fn inc_rpc_error(&self) {
        self.rpc_errors.fetch_add(1, Ordering::Relaxed);
    }
    pub fn to_prometheus(&self) -> String {
        format!(
            "# HELP signal_messages_sent_total Total messages sent\n\
             # TYPE signal_messages_sent_total counter\n\
             signal_messages_sent_total {}\n\
             # HELP signal_messages_received_total Total messages received\n\
             # TYPE signal_messages_received_total counter\n\
             signal_messages_received_total {}\n\
             # HELP signal_rpc_calls_total Total JSON-RPC calls to signal-cli\n\
             # TYPE signal_rpc_calls_total counter\n\
             signal_rpc_calls_total {}\n\
             # HELP signal_rpc_errors_total Total JSON-RPC errors\n\
             # TYPE signal_rpc_errors_total counter\n\
             signal_rpc_errors_total {}\n\
             # HELP signal_ws_clients_active Active WebSocket clients\n\
             # TYPE signal_ws_clients_active gauge\n\
             signal_ws_clients_active {}\n",
            self.messages_sent.load(Ordering::Relaxed),
            self.messages_received.load(Ordering::Relaxed),
            self.rpc_calls.load(Ordering::Relaxed),
            self.rpc_errors.load(Ordering::Relaxed),
            self.ws_clients.load(Ordering::Relaxed),
        )
    }
}

// ---------------------------------------------------------------------------
// Webhook
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WebhookConfig {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>, // empty = all events
}

// ---------------------------------------------------------------------------
// AppState
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    pub writer_tx: tokio::sync::mpsc::Sender<String>,
    pub broadcast_tx: broadcast::Sender<String>,
    pub pending: Arc<DashMap<u64, oneshot::Sender<RpcResponse>>>,
    pub next_id: Arc<AtomicU64>,
    pub metrics: Arc<Metrics>,
    pub webhooks: Arc<RwLock<Vec<WebhookConfig>>>,
    pub rpc_timeout: Duration,
}

/// Sentinel error string returned when an RPC call times out.
pub const RPC_TIMEOUT_ERROR: &str = "RPC_TIMEOUT";

/// Map an RPC error string to the appropriate HTTP status code.
pub fn rpc_error_status(err: &str) -> axum::http::StatusCode {
    if err == RPC_TIMEOUT_ERROR {
        axum::http::StatusCode::GATEWAY_TIMEOUT
    } else {
        axum::http::StatusCode::BAD_REQUEST
    }
}

impl AppState {
    pub fn new(writer_tx: tokio::sync::mpsc::Sender<String>) -> Self {
        let (broadcast_tx, _) = broadcast::channel(256);
        Self {
            writer_tx,
            broadcast_tx,
            pending: Arc::new(DashMap::new()),
            next_id: Arc::new(AtomicU64::new(1)),
            metrics: Arc::new(Metrics::default()),
            webhooks: Arc::new(RwLock::new(Vec::new())),
            rpc_timeout: Duration::from_secs(30),
        }
    }

    /// Helper: make a JSON-RPC call to signal-cli.
    pub async fn rpc(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, String> {
        self.metrics.inc_rpc();
        let result = crate::jsonrpc::rpc_call(
            &self.writer_tx,
            &self.pending,
            &self.next_id,
            method,
            params,
            self.rpc_timeout,
        )
        .await;
        if result.is_err() {
            self.metrics.inc_rpc_error();
        }
        result
    }
}
