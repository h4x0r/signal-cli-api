pub mod accounts;
pub mod helpers;
pub mod attachments;
pub mod config;
pub mod contacts;
pub mod devices;
pub mod events;
pub mod groups;
pub mod identities;
pub mod messages;
pub mod metrics;
pub mod openapi;
pub mod polls;
pub mod profiles;
pub mod reactions;
pub mod receipts;
pub mod search;
pub mod stickers;
pub mod system;
pub mod typing;
pub mod webhook_routes;

use axum::Router;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(system::routes())
        .merge(accounts::routes())
        .merge(devices::routes())
        .merge(messages::routes())
        .merge(attachments::routes())
        .merge(contacts::routes())
        .merge(groups::routes())
        .merge(reactions::routes())
        .merge(receipts::routes())
        .merge(typing::routes())
        .merge(profiles::routes())
        .merge(identities::routes())
        .merge(polls::routes())
        .merge(search::routes())
        .merge(stickers::routes())
        .merge(config::routes())
        // Extras beyond bbernhard parity
        .merge(webhook_routes::routes())
        .merge(events::routes())
        .merge(metrics::routes())
        .merge(openapi::routes())
        .with_state(state)
}
