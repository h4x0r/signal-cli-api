use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::atomic::{AtomicU64, Ordering};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Middleware that assigns a request ID and logs request/response details.
pub async fn request_tracing(request: Request, next: Next) -> Response {
    let request_id = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let start = std::time::Instant::now();

    let mut response = next.run(request).await;

    let latency_ms = start.elapsed().as_millis() as u64;
    let status = response.status().as_u16();

    tracing::info!(
        request_id,
        %method,
        path,
        status,
        latency_ms,
    );

    response.headers_mut().insert(
        "x-request-id",
        request_id.to_string().parse().unwrap(),
    );

    response
}
