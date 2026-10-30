use axum::{
    Router,
    extract::{Path, Query, State},
    response::Response,
    routing::{delete, get},
};
use serde::Deserialize;
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

#[derive(Deserialize)]
struct GetAttachmentQuery {
    /// Account (phone number) the attachment belongs to. signal-cli's daemon
    /// needs this to pick a manager when more than one account is registered;
    /// without it `getAttachment` fails with "Method requires valid account
    /// parameter". Optional so single-account callers keep working unchanged.
    #[serde(default)]
    account: Option<String>,
}

/// GET /v1/attachments/{attachment}[?account=+1555...] — retrieve a specific attachment.
async fn get_attachment(
    State(st): State<AppState>,
    Path(attachment): Path<String>,
    Query(query): Query<GetAttachmentQuery>,
) -> Response {
    let mut params = json!({ "id": attachment });
    if let Some(account) = query.account {
        params["account"] = json!(account);
    }
    rpc_ok(&st, "getAttachment", params).await
}

/// DELETE /v1/attachments/{attachment} — delete a locally cached attachment.
async fn delete_attachment(
    State(st): State<AppState>,
    Path(attachment): Path<String>,
) -> Response {
    rpc_no_content(&st, "deleteAttachment", json!({ "id": attachment })).await
}
