# signal-cli-api

A native REST + WebSocket API for [signal-cli](https://github.com/AsamK/signal-cli). Build Signal bots, notification systems, and integrations in any language.

## Why this one

- **Zero config.** Run `signal-cli-api` and it works. Auto-spawns signal-cli, picks free ports, no containers or orchestration needed.
- **Real-time streaming.** WebSocket, SSE, and webhook delivery for incoming messages. No polling loops.
- **Observable.** Prometheus metrics, structured request tracing with `x-request-id`, per-RPC latency logging.
- **Native TLS.** Pass `--tls-cert` and `--tls-key`. No reverse proxy needed for HTTPS.
- **Fast.** Rust + axum. Sub-millisecond request overhead. Persistent JSON-RPC connection to signal-cli (no JVM restarts per request).
- **Tested.** 267 integration tests against a mock signal-cli daemon.

## Quick start

### 1. Install signal-cli and register

```bash
brew install signal-cli   # or your package manager

signal-cli -u +1234567890 register
signal-cli -u +1234567890 verify 123-456
```

### 2. Run

```bash
signal-cli-api
```

That's it. API is live at `http://localhost:8080`. signal-cli is managed automatically.

If you already run signal-cli as a daemon:

```bash
signal-cli-api --signal-cli 127.0.0.1:7583
```

### Options

```
--signal-cli <addr>   Connect to existing signal-cli daemon (default: auto-spawn)
--listen <addr>       HTTP listen address (default: 127.0.0.1:8080, falls back to random port if busy)
--tls-cert <path>     TLS certificate (PEM). Enables HTTPS.
--tls-key <path>      TLS private key (PEM). Required with --tls-cert.
```

## Send a message

```bash
curl -X POST http://localhost:8080/v2/send \
  -H 'Content-Type: application/json' \
  -d '{
    "message": "Hello from my app!",
    "number": "+1234567890",
    "recipients": ["+1987654321"]
  }'
```

```json
{"timestamp": 1234567890}
```

## Receive messages

### WebSocket (recommended for bots)

Connect to `ws://localhost:8080/v1/receive/+1234567890`. Messages stream as JSON:

```json
{
  "envelope": {
    "source": "+1987654321",
    "dataMessage": {
      "message": "Hey!",
      "timestamp": 1234567890
    }
  }
}
```

Works with any WebSocket client — Python, Node, Go, Rust, whatever.

### Server-Sent Events (SSE)

```bash
curl -N http://localhost:8080/v1/events/+1234567890
```

### Webhooks

Push incoming messages to your HTTP endpoint:

```bash
# Register
curl -X POST http://localhost:8080/v1/webhooks \
  -H 'Content-Type: application/json' \
  -d '{"url": "https://your-app.com/signal-hook"}'

# With event filtering
curl -X POST http://localhost:8080/v1/webhooks \
  -H 'Content-Type: application/json' \
  -d '{"url": "https://your-app.com/hook", "events": ["message", "receipt"]}'
```

## Monitoring

Prometheus-compatible metrics at `/metrics`:

```
signal_messages_sent_total 42
signal_messages_received_total 108
signal_rpc_calls_total 312
signal_rpc_errors_total 0
signal_ws_clients_active 2
```

Every request gets an `x-request-id` header and structured log entry:

```
INFO request_id=47 method=POST path="/v2/send" status=201 latency_ms=1152
INFO rpc_method="send" status=201 latency_ms=1150
```

## API reference

### Messages

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v2/send` | Send message (text, attachments, mentions, quotes) |
| POST | `/v1/send` | Send message (v1, deprecated) |
| GET | `/v1/receive/{number}` | WebSocket stream |
| DELETE | `/v1/remote-delete/{number}` | Delete a sent message |

### Typing, Reactions & Receipts

| Method | Endpoint | Description |
|--------|----------|-------------|
| PUT | `/v1/typing-indicator/{number}` | Show typing |
| DELETE | `/v1/typing-indicator/{number}` | Stop typing |
| POST | `/v1/reactions/{number}` | Send reaction |
| DELETE | `/v1/reactions/{number}` | Remove reaction |
| POST | `/v1/receipts/{number}` | Send read/delivery receipt |

### Groups

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/groups/{number}` | List groups |
| POST | `/v1/groups/{number}` | Create group |
| GET | `/v1/groups/{number}/{groupid}` | Get group |
| PUT | `/v1/groups/{number}/{groupid}` | Update group |
| DELETE | `/v1/groups/{number}/{groupid}` | Delete group |
| POST | `/v1/groups/{number}/{groupid}/members` | Add members |
| DELETE | `/v1/groups/{number}/{groupid}/members` | Remove members |
| POST | `/v1/groups/{number}/{groupid}/admins` | Add admins |
| DELETE | `/v1/groups/{number}/{groupid}/admins` | Remove admins |
| POST | `/v1/groups/{number}/{groupid}/join` | Join group |
| POST | `/v1/groups/{number}/{groupid}/quit` | Quit group |
| POST | `/v1/groups/{number}/{groupid}/block` | Block group |

### Contacts

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/contacts/{number}` | List contacts |
| GET | `/v1/contacts/{number}/{recipient}` | Get contact |
| PUT | `/v1/contacts/{number}` | Update contact |
| POST | `/v1/contacts/{number}/sync` | Sync contacts |

### Accounts

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/accounts` | List accounts |
| POST | `/v1/register/{number}` | Register |
| POST | `/v1/register/{number}/verify/{token}` | Verify |
| POST | `/v1/unregister/{number}` | Unregister |
| POST | `/v1/accounts/{number}/rate-limit-challenge` | Rate-limit challenge |
| PUT | `/v1/accounts/{number}/settings` | Update settings |
| POST | `/v1/accounts/{number}/pin` | Set PIN |
| DELETE | `/v1/accounts/{number}/pin` | Remove PIN |
| POST | `/v1/accounts/{number}/username` | Set username |
| DELETE | `/v1/accounts/{number}/username` | Remove username |

### Devices

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/devices/{number}` | List devices |
| POST | `/v1/devices/{number}` | Link device |
| DELETE | `/v1/devices/{number}/{device_id}` | Remove device |
| DELETE | `/v1/devices/{number}/local-data` | Delete local data |
| GET | `/v1/qrcodelink` | QR code link URI |
| GET | `/v1/qrcodelink/raw` | Raw link URI |

### Identities & Profiles

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/identities/{number}` | List identities |
| PUT | `/v1/identities/{number}/trust/{number_to_trust}` | Trust identity |
| PUT | `/v1/profiles/{number}` | Update profile |

### Polls & Stickers

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v1/polls/{number}` | Create poll |
| POST | `/v1/polls/{number}/vote` | Vote |
| DELETE | `/v1/polls/{number}` | Close poll |
| GET | `/v1/sticker-packs/{number}` | List sticker packs |
| POST | `/v1/sticker-packs/{number}` | Install sticker pack |

### Attachments & Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/attachments` | List attachments |
| GET | `/v1/attachments/{id}` | Get attachment |
| DELETE | `/v1/attachments/{id}` | Delete attachment |
| GET | `/v1/search/{number}?numbers=+111,+222` | Check registration status |

### Webhooks

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v1/webhooks` | Register webhook |
| GET | `/v1/webhooks` | List webhooks |
| DELETE | `/v1/webhooks/{id}` | Remove webhook |

### System

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/health` | Health check (204) |
| GET | `/v1/about` | Version and build info |
| GET | `/v1/openapi.json` | OpenAPI 3.0 spec |
| GET | `/v1/events/{number}` | SSE stream |
| GET | `/metrics` | Prometheus metrics |

## Building from source

```bash
git clone <this-repo>
cd signal-cli-api
cargo build --release
# Binary at target/release/signal-cli-api (~4 MB)
```

## Tests

```bash
cargo test   # 267 tests, no Signal account needed
```

## License

MIT
