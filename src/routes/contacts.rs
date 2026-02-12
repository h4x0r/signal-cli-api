use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::rpc_ok;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/contacts/{number}", get(list_contacts).put(update_contact))
        .route("/v1/contacts/{number}/{recipient}", get(get_contact))
        .route("/v1/contacts/{number}/sync", post(sync_contacts))
        .route("/v1/contacts/{number}/{recipient}/avatar", get(get_avatar))
}

async fn list_contacts(
    State(st): State<AppState>,
    Path(number): Path<String>,
) -> Response {
    rpc_ok(&st, "listContacts", json!({ "account": number })).await
}

async fn get_contact(
    State(st): State<AppState>,
    Path((number, recipient)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "listContacts", json!({ "account": number, "recipient": [recipient] })).await
}

#[derive(Deserialize)]
struct UpdateContactBody {
    name: Option<String>,
    expiration: Option<u64>,
    recipient: Option<String>,
}

async fn update_contact(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<UpdateContactBody>,
) -> Response {
    let mut params = json!({ "account": number });
    if let Some(name) = &body.name {
        params["name"] = json!(name);
    }
    if let Some(exp) = body.expiration {
        params["expiration"] = json!(exp);
    }
    if let Some(recipient) = &body.recipient {
        params["recipient"] = json!([recipient]);
    }
    rpc_ok(&st, "updateContact", params).await
}

async fn sync_contacts(
    State(st): State<AppState>,
    Path(number): Path<String>,
) -> Response {
    rpc_ok(&st, "sendContacts", json!({ "account": number })).await
}

async fn get_avatar(
    Path((_number, _recipient)): Path<(String, String)>,
) -> Response {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "error": "Avatar retrieval not yet implemented" }))).into_response()
}
