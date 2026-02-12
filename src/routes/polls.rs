use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, post},
    Json,
};
use serde_json::{json, Value};

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_created};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/polls/{number}", post(create_poll))
        .route("/v1/polls/{number}/vote", post(vote_poll))
        .route("/v1/polls/{number}", delete(close_poll))
}

/// POST /v1/polls/{number} — create and send a poll.
async fn create_poll(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_created(&st, "sendPoll", params).await
}

/// POST /v1/polls/{number}/vote — vote on an existing poll.
async fn vote_poll(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_ok(&st, "sendPollVote", params).await
}

/// DELETE /v1/polls/{number} — close a poll.
async fn close_poll(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_ok(&st, "closePoll", params).await
}
