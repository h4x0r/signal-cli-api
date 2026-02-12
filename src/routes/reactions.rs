use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, post},
    Json,
};
use serde_json::{json, Value};

use crate::state::AppState;
use super::helpers::{rpc_created, rpc_no_content};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/reactions/{number}", post(send_reaction))
        .route("/v1/reactions/{number}", delete(remove_reaction))
}

/// POST /v1/reactions/{number} — send a reaction to a message.
async fn send_reaction(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_created(&st, "sendReaction", params).await
}

/// DELETE /v1/reactions/{number} — remove a reaction from a message.
async fn remove_reaction(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_no_content(&st, "removeReaction", params).await
}
