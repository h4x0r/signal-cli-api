use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use axum::{Json, Router};
use serde::Deserialize;

use crate::state::{AppState, WebhookConfig};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/webhooks", post(create_webhook).get(list_webhooks))
        .route("/v1/webhooks/{id}", delete(delete_webhook))
}

#[derive(Deserialize)]
struct CreateWebhook {
    url: String,
    #[serde(default)]
    events: Vec<String>,
}

async fn create_webhook(
    State(st): State<AppState>,
    Json(body): Json<CreateWebhook>,
) -> Response {
    let id = format!(
        "{:016x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );

    let config = WebhookConfig {
        id,
        url: body.url,
        events: body.events,
    };

    st.webhooks.write().await.push(config.clone());

    (StatusCode::CREATED, Json(config)).into_response()
}

async fn list_webhooks(State(st): State<AppState>) -> Response {
    let hooks = st.webhooks.read().await;
    Json(hooks.clone()).into_response()
}

async fn delete_webhook(
    State(st): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut hooks = st.webhooks.write().await;
    let len_before = hooks.len();
    hooks.retain(|h| h.id != id);
    if hooks.len() < len_before {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
