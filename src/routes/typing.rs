use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, put},
    Json,
};
use serde_json::{json, Value};

use crate::state::AppState;
use super::helpers::rpc_no_content;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/typing-indicator/{number}", put(start_typing))
        .route("/v1/typing-indicator/{number}", delete(stop_typing))
}

/// PUT /v1/typing-indicator/{number} — start typing indicator.
async fn start_typing(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    params["stop"] = json!(false);
    rpc_no_content(&st, "sendTyping", params).await
}

/// DELETE /v1/typing-indicator/{number} — stop typing indicator.
async fn stop_typing(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    params["stop"] = json!(true);
    rpc_no_content(&st, "sendTyping", params).await
}
