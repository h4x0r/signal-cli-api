use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Json,
};
use serde_json::{json, Value};

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_created};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/sticker-packs/{number}", get(list_sticker_packs))
        .route("/v1/sticker-packs/{number}", post(install_sticker_pack))
}

/// GET /v1/sticker-packs/{number} — list installed sticker packs.
async fn list_sticker_packs(
    State(st): State<AppState>,
    Path(number): Path<String>,
) -> Response {
    rpc_ok(&st, "listStickerPacks", json!({ "account": number })).await
}

/// POST /v1/sticker-packs/{number} — install a sticker pack.
async fn install_sticker_pack(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    let mut params = body;
    params["account"] = json!(number);
    rpc_created(&st, "uploadStickerPack", params).await
}
