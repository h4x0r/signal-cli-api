use axum::extract::{Path, State};
use axum::response::Response;
use axum::routing::{get, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::rpc_ok;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/identities/{number}", get(list_identities))
        .route("/v1/identities/{number}/trust/{number_to_trust}", put(trust_identity))
}

async fn list_identities(
    State(st): State<AppState>,
    Path(number): Path<String>,
) -> Response {
    rpc_ok(&st, "listIdentities", json!({ "account": number })).await
}

#[derive(Deserialize)]
struct TrustBody {
    #[serde(default)]
    trust_all_known_keys: Option<bool>,
    verified_safety_number: Option<String>,
}

async fn trust_identity(
    State(st): State<AppState>,
    Path((number, number_to_trust)): Path<(String, String)>,
    Json(body): Json<TrustBody>,
) -> Response {
    let mut params = json!({
        "account": number,
        "recipient": [number_to_trust],
    });
    if let Some(true) = body.trust_all_known_keys {
        params["trust-all-known-keys"] = json!(true);
    }
    if let Some(safety_number) = &body.verified_safety_number {
        params["verified-safety-number"] = json!(safety_number);
    }
    rpc_ok(&st, "trust", params).await
}
