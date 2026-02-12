use std::convert::Infallible;

use axum::extract::{Path, State};
use axum::response::sse::{Event, Sse};
use axum::routing::get;
use axum::Router;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/v1/events/{number}", get(sse_events))
}

async fn sse_events(
    State(st): State<AppState>,
    Path(_number): Path<String>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = st.broadcast_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(msg) => Some(Ok(Event::default().event("message").data(msg))),
        Err(_) => None,
    });
    Sse::new(stream)
}
