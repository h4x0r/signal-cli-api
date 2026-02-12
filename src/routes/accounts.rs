use axum::extract::{Path, State};
use axum::response::Response;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_no_content};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/accounts", get(list_accounts))
        .route("/v1/register/{number}", post(register))
        .route("/v1/register/{number}/verify/{token}", post(verify))
        .route("/v1/unregister/{number}", post(unregister))
        .route(
            "/v1/accounts/{number}/rate-limit-challenge",
            post(rate_limit_challenge),
        )
        .route("/v1/accounts/{number}/settings", put(update_settings))
        .route(
            "/v1/accounts/{number}/pin",
            post(set_pin).delete(remove_pin),
        )
        .route(
            "/v1/accounts/{number}/username",
            post(set_username).delete(remove_username),
        )
}

async fn list_accounts(State(st): State<AppState>) -> Response {
    rpc_ok(&st, "listAccounts", json!({})).await
}

#[derive(Deserialize)]
struct RegisterBody {
    #[serde(default)]
    captcha: Option<String>,
    #[serde(default)]
    voice: Option<bool>,
}

async fn register(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<RegisterBody>,
) -> Response {
    let mut params = json!({ "account": number });
    if let Some(captcha) = body.captcha {
        params["captcha"] = json!(captcha);
    }
    if let Some(voice) = body.voice {
        params["voice"] = json!(voice);
    }
    rpc_no_content(&st, "register", params).await
}

async fn verify(
    Path((number, token)): Path<(String, String)>,
    State(st): State<AppState>,
) -> Response {
    rpc_no_content(&st, "verify", json!({ "account": number, "verificationCode": token })).await
}

async fn unregister(Path(number): Path<String>, State(st): State<AppState>) -> Response {
    rpc_no_content(&st, "unregister", json!({ "account": number })).await
}

#[derive(Deserialize)]
struct RateLimitBody {
    challenge: String,
    captcha: String,
}

async fn rate_limit_challenge(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<RateLimitBody>,
) -> Response {
    rpc_no_content(&st, "submitRateLimitChallenge", json!({
        "account": number,
        "challenge": body.challenge,
        "captcha": body.captcha,
    })).await
}

#[derive(Deserialize)]
struct SettingsBody {
    #[serde(default)]
    trust_mode: Option<String>,
}

async fn update_settings(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<SettingsBody>,
) -> Response {
    let mut params = json!({ "account": number });
    if let Some(trust_mode) = body.trust_mode {
        params["trustMode"] = json!(trust_mode);
    }
    rpc_no_content(&st, "updateAccountSettings", params).await
}

#[derive(Deserialize)]
struct PinBody {
    pin: String,
}

async fn set_pin(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<PinBody>,
) -> Response {
    rpc_no_content(&st, "setPin", json!({ "account": number, "pin": body.pin })).await
}

async fn remove_pin(Path(number): Path<String>, State(st): State<AppState>) -> Response {
    rpc_no_content(&st, "removePin", json!({ "account": number })).await
}

#[derive(Deserialize)]
struct UsernameBody {
    username: String,
}

async fn set_username(
    Path(number): Path<String>,
    State(st): State<AppState>,
    Json(body): Json<UsernameBody>,
) -> Response {
    rpc_no_content(&st, "setUsername", json!({ "account": number, "username": body.username })).await
}

async fn remove_username(Path(number): Path<String>, State(st): State<AppState>) -> Response {
    rpc_no_content(&st, "removeUsername", json!({ "account": number })).await
}
