use crate::state::AppState;

/// Extract the event type from a Signal notification JSON.
/// Maps envelope fields to event type names:
///   dataMessage -> "message", receiptMessage -> "receipt",
///   typingMessage -> "typing", syncMessage -> "sync"
fn extract_event_type(msg: &str) -> Option<&'static str> {
    let parsed: serde_json::Value = serde_json::from_str(msg).ok()?;
    let envelope = parsed.get("envelope")?;
    if envelope.get("dataMessage").is_some() {
        Some("message")
    } else if envelope.get("receiptMessage").is_some() {
        Some("receipt")
    } else if envelope.get("typingMessage").is_some() {
        Some("typing")
    } else if envelope.get("syncMessage").is_some() {
        Some("sync")
    } else {
        None
    }
}

/// Subscribes to the broadcast channel and POSTs each incoming message
/// to all registered webhook URLs. Respects the `events` filter on each webhook.
pub async fn dispatch_loop(state: AppState) {
    let client = reqwest::Client::new();
    let mut rx = state.broadcast_tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let event_type = extract_event_type(&msg);
        let hooks = state.webhooks.read().await.clone();
        for hook in hooks.iter() {
            // Skip if the webhook has an event filter and this event doesn't match
            if !hook.events.is_empty() {
                if let Some(et) = event_type {
                    if !hook.events.iter().any(|e| e == et) {
                        continue;
                    }
                } else {
                    // Unknown event type and webhook has a filter — skip
                    continue;
                }
            }

            let client = client.clone();
            let url = hook.url.clone();
            let body = msg.clone();
            tokio::spawn(async move {
                if let Err(e) = client
                    .post(&url)
                    .header("content-type", "application/json")
                    .body(body)
                    .send()
                    .await
                {
                    tracing::warn!("Webhook delivery to {url} failed: {e}");
                }
            });
        }
    }
}
