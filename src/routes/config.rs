use axum::extract::{Path, State};
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_no_content};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/configuration",
            get(get_global_config).post(set_global_config),
        )
        .route(
            "/v1/configuration/{number}/settings",
            get(get_account_config).post(set_account_config),
        )
}

async fn get_global_config(State(st): State<AppState>) -> Response {
    rpc_ok(&st, "getConfiguration", json!({})).await
}

async fn set_global_config(
    State(st): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    rpc_no_content(&st, "setConfiguration", body).await
}

async fn get_account_config(
    Path(number): Path<String>,
    State(st): State<AppState>,
) -> Response {
    rpc_ok(&st, "getAccountSettings", json!({ "account": number })).await
}

async fn set_account_config(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_no_content(&st, "setAccountSettings", params).await
}
