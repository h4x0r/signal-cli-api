use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, get},
};
use serde_json::json;

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_no_content};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/attachments", get(list_attachments))
        .route("/v1/attachments/{attachment}", get(get_attachment))
        .route("/v1/attachments/{attachment}", delete(delete_attachment))
}

/// GET /v1/attachments — list all locally cached attachments.
async fn list_attachments(State(st): State<AppState>) -> Response {
    rpc_ok(&st, "listAttachments", json!({})).await
}

/// GET /v1/attachments/{attachment} — retrieve a specific attachment.
async fn get_attachment(
    State(st): State<AppState>,
    Path(attachment): Path<String>,
) -> Response {
    rpc_ok(&st, "getAttachment", json!({ "id": attachment })).await
}

/// DELETE /v1/attachments/{attachment} — delete a locally cached attachment.
async fn delete_attachment(
    State(st): State<AppState>,
    Path(attachment): Path<String>,
) -> Response {
    rpc_no_content(&st, "deleteAttachment", json!({ "id": attachment })).await
}
