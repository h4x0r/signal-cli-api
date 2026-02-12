use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/about", get(about))
}

async fn health() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

async fn about() -> Response {
    let info = json!({
        "versions": {
            "signal-cli-api": env!("CARGO_PKG_VERSION"),
        },
        "build": {
            "target": std::env::consts::ARCH,
            "os": std::env::consts::OS,
        }
    });
    Json(info).into_response()
}
