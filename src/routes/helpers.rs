use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::state::{rpc_error_status, AppState};

/// Make an RPC call and return 200 OK with the JSON result on success.
pub async fn rpc_ok(st: &AppState, method: &str, params: Value) -> Response {
    let start = std::time::Instant::now();
    match st.rpc(method, params).await {
        Ok(result) => {
            tracing::info!(rpc_method = method, status = 200, latency_ms = start.elapsed().as_millis() as u64);
            Json(result).into_response()
        }
        Err(e) => {
            let status = rpc_error_status(&e);
            tracing::warn!(rpc_method = method, status = status.as_u16(), error = %e, latency_ms = start.elapsed().as_millis() as u64);
            (status, Json(json!({ "error": e }))).into_response()
        }
    }
}

/// Make an RPC call and return 201 Created with the JSON result on success.
pub async fn rpc_created(st: &AppState, method: &str, params: Value) -> Response {
    let start = std::time::Instant::now();
    match st.rpc(method, params).await {
        Ok(result) => {
            tracing::info!(rpc_method = method, status = 201, latency_ms = start.elapsed().as_millis() as u64);
            (StatusCode::CREATED, Json(result)).into_response()
        }
        Err(e) => {
            let status = rpc_error_status(&e);
            tracing::warn!(rpc_method = method, status = status.as_u16(), error = %e, latency_ms = start.elapsed().as_millis() as u64);
            (status, Json(json!({ "error": e }))).into_response()
        }
    }
}

/// Make an RPC call and return 204 No Content on success.
pub async fn rpc_no_content(st: &AppState, method: &str, params: Value) -> Response {
    let start = std::time::Instant::now();
    match st.rpc(method, params).await {
        Ok(_) => {
            tracing::info!(rpc_method = method, status = 204, latency_ms = start.elapsed().as_millis() as u64);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            let status = rpc_error_status(&e);
            tracing::warn!(rpc_method = method, status = status.as_u16(), error = %e, latency_ms = start.elapsed().as_millis() as u64);
            (status, Json(json!({ "error": e }))).into_response()
        }
    }
}
