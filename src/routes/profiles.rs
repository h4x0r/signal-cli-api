use axum::extract::{Path, State};
use axum::response::Response;
use axum::routing::put;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::rpc_ok;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/profiles/{number}", put(update_profile))
}

#[derive(Deserialize)]
struct UpdateProfileBody {
    name: Option<String>,
    about: Option<String>,
    base64_avatar: Option<String>,
}

async fn update_profile(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<UpdateProfileBody>,
) -> Response {
    let mut params = json!({ "account": number });
    if let Some(name) = &body.name {
        params["given-name"] = json!(name);
    }
    if let Some(about) = &body.about {
        params["about"] = json!(about);
    }
    if let Some(avatar) = &body.base64_avatar {
        params["avatar"] = json!(avatar);
    }
    rpc_ok(&st, "updateProfile", params).await
}
