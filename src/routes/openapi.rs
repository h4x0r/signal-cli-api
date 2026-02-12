use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/v1/openapi.json", get(openapi_spec))
}

async fn openapi_spec() -> Response {
    let spec = json!({
        "openapi": "3.0.3",
        "info": {
            "title": "signal-cli REST API",
            "description": "REST API bridge for signal-cli",
            "version": env!("CARGO_PKG_VERSION")
        },
        "paths": {
            "/v2/send": {
                "post": {
                    "tags": ["Messages"],
                    "summary": "Send a message",
                    "operationId": "send",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/SendPayload" }
                            }
                        }
                    },
                    "responses": {
                        "201": { "description": "Message sent" },
                        "400": { "description": "Invalid request" }
                    }
                }
            },
            "/v1/receive/{number}": {
                "get": {
                    "tags": ["Messages"],
                    "summary": "Receive messages",
                    "operationId": "receive",
                    "parameters": [{
                        "name": "number",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" }
                    }],
                    "responses": {
                        "200": { "description": "Array of messages" }
                    }
                }
            },
            "/v1/health": {
                "get": {
                    "tags": ["System"],
                    "summary": "Health check",
                    "operationId": "health",
                    "responses": {
                        "204": { "description": "Healthy" }
                    }
                }
            },
            "/v1/about": {
                "get": {
                    "tags": ["System"],
                    "summary": "API version info",
                    "operationId": "about",
                    "responses": {
                        "200": { "description": "Version information" }
                    }
                }
            },
            "/v1/groups/{number}": {
                "get": {
                    "tags": ["Groups"],
                    "summary": "List groups for an account",
                    "operationId": "listGroups",
                    "parameters": [{
                        "name": "number",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" }
                    }],
                    "responses": {
                        "200": { "description": "Array of groups" }
                    }
                }
            },
            "/v1/webhooks": {
                "get": {
                    "tags": ["Webhooks"],
                    "summary": "List registered webhooks",
                    "operationId": "listWebhooks",
                    "responses": {
                        "200": { "description": "Array of webhook configs" }
                    }
                },
                "post": {
                    "tags": ["Webhooks"],
                    "summary": "Register a webhook",
                    "operationId": "createWebhook",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/WebhookConfig" }
                            }
                        }
                    },
                    "responses": {
                        "201": { "description": "Webhook registered" }
                    }
                }
            },
            "/v1/events/{number}": {
                "get": {
                    "tags": ["Events"],
                    "summary": "Server-Sent Events stream",
                    "operationId": "sseEvents",
                    "parameters": [{
                        "name": "number",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" }
                    }],
                    "responses": {
                        "200": { "description": "SSE stream of messages" }
                    }
                }
            },
            "/metrics": {
                "get": {
                    "tags": ["System"],
                    "summary": "Prometheus metrics",
                    "operationId": "metrics",
                    "responses": {
                        "200": {
                            "description": "Prometheus-formatted metrics",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string" }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "SendPayload": {
                    "type": "object",
                    "required": ["message", "number", "recipients"],
                    "properties": {
                        "message": { "type": "string", "description": "Message text" },
                        "number": { "type": "string", "description": "Sender account number" },
                        "recipients": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Recipient numbers"
                        },
                        "base64_attachments": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Base64-encoded attachments"
                        }
                    }
                },
                "WebhookConfig": {
                    "type": "object",
                    "required": ["url"],
                    "properties": {
                        "id": { "type": "string", "description": "Webhook ID (server-generated)" },
                        "url": { "type": "string", "description": "Callback URL" },
                        "events": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Event types to subscribe to (empty = all)"
                        }
                    }
                }
            }
        }
    });

    Json(spec).into_response()
}
