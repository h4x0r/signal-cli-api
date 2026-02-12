use axum::{
    Router,
    extract::{Path, Query, State},
    response::Response,
    routing::get,
};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::rpc_ok;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/search/{number}", get(search_numbers))
}

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(default)]
    numbers: String,
}

/// GET /v1/search/{number}?numbers=... — check if phone numbers are registered on Signal.
async fn search_numbers(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Query(q): Query<SearchQuery>,
) -> Response {
    let recipients: Vec<&str> = q.numbers.split(',').filter(|s| !s.is_empty()).collect();
    rpc_ok(&st, "getUserStatus", json!({ "account": number, "recipient": recipients })).await
}
