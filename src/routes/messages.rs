use axum::{
    Router,
    extract::{Path, State, WebSocketUpgrade, ws},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json,
};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_created};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/send", post(send_v1))
        .route("/v2/send", post(send_v2))
        .route("/v1/receive/{number}", get(receive_ws))
        .route("/v1/remote-delete/{number}", delete(remote_delete))
}

/// POST /v1/send — send a message (v1, simple).
async fn send_v1(
    State(st): State<AppState>,
    Json(body): Json<Value>,
) -> Response {
    rpc_created(&st, "send", body).await
}

/// POST /v2/send — send a message (v2, extended). Increments sent counter.
async fn send_v2(
    State(st): State<AppState>,
    Json(body): Json<Value>,
) -> Response {
    let start = std::time::Instant::now();
    match st.rpc("send", body).await {
        Ok(result) => {
            st.metrics.inc_sent();
            tracing::info!(rpc_method = "send", status = 201, latency_ms = start.elapsed().as_millis() as u64);
            (axum::http::StatusCode::CREATED, Json(result)).into_response()
        }
        Err(e) => {
            let status = crate::state::rpc_error_status(&e);
            tracing::warn!(rpc_method = "send", status = status.as_u16(), error = %e, latency_ms = start.elapsed().as_millis() as u64);
            (status, Json(json!({ "error": e }))).into_response()
        }
    }
}

/// GET /v1/receive/{number} — WebSocket endpoint for real-time messages.
async fn receive_ws(
    State(st): State<AppState>,
    Path(_number): Path<String>,
    upgrade: WebSocketUpgrade,
) -> impl IntoResponse {
    upgrade.on_upgrade(move |socket| handle_ws(socket, st))
}

async fn handle_ws(mut socket: ws::WebSocket, st: AppState) {
    st.metrics.ws_clients.fetch_add(1, Ordering::Relaxed);
    let mut rx = st.broadcast_tx.subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(text) => {
                        if socket.send(ws::Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                }
            }
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(ws::Message::Close(_))) | None => break,
                    _ => {} // ignore client-sent frames
                }
            }
        }
    }

    st.metrics.ws_clients.fetch_sub(1, Ordering::Relaxed);
}

/// DELETE /v1/remote-delete/{number} — remotely delete a sent message.
async fn remote_delete(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_ok(&st, "remoteDelete", params).await
}
