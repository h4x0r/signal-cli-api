# signal-cli-api

A fast, native REST + WebSocket API for [signal-cli](https://github.com/AsamK/signal-cli). Send and receive Signal messages from any language with a simple REST API.

- **No Docker.** Single static binary, ~7 MB. Runs anywhere.
- **Real-time.** WebSocket and SSE streams for incoming messages.
- **Observable.** Prometheus metrics and webhooks built in.
- **Tested.** 215 regression tests against a mock signal-cli. Ships with confidence.
- **Fast.** Written in Rust (axum). Sub-millisecond request overhead.

## What it does

You run `signal-cli` in daemon mode. This binary sits in front of it and gives you a clean HTTP API to send and receive Signal messages from any language.

```
Your app  -->  signal-cli-api (REST/WS)  -->  signal-cli daemon  -->  Signal network
```

## Quick start

### 1. Install signal-cli and register your number

```bash
brew install signal-cli

# Register (you'll get an SMS code)
signal-cli -u +1234567890 register
signal-cli -u +1234567890 verify 123-456
```

### 2. Start signal-cli in daemon mode

```bash
signal-cli -u +1234567890 daemon --tcp 127.0.0.1:7583
```

### 3. Start signal-cli-api

```bash
# From source
cargo run

# Or with a release binary
./signal-cli-api
```

That's it. API is live at `http://localhost:8080`.

### Options

```
--signal-cli  signal-cli daemon address  [default: 127.0.0.1:7583]
--listen      HTTP API listen address    [default: 127.0.0.1:8080]
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

Response:
```json
{"timestamp": 1234567890}
```

## Receive messages (WebSocket)

Connect to `ws://localhost:8080/v1/receive/+1234567890` and incoming messages stream in as JSON:

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

Works with any WebSocket client — Python, Node, Go, whatever.

## Receive messages (SSE)

Prefer Server-Sent Events? Same data, different transport:

```bash
curl -N http://localhost:8080/v1/events/+1234567890
```

## API reference

### Messages

| Method | Endpoint | What it does |
|--------|----------|--------------|
| POST | `/v2/send` | Send a message (text, attachments, mentions, quotes) |
| POST | `/v1/send` | Send a message (v1, same thing) |
| GET | `/v1/receive/{number}` | WebSocket stream of incoming messages |
| DELETE | `/v1/remote-delete/{number}` | Delete a sent message |

### Typing & Reactions

| Method | Endpoint | What it does |
|--------|----------|--------------|
| PUT | `/v1/typing-indicator/{number}` | Show "typing..." |
| DELETE | `/v1/typing-indicator/{number}` | Stop "typing..." |
| POST | `/v1/reactions/{number}` | React to a message |
| DELETE | `/v1/reactions/{number}` | Remove a reaction |
| POST | `/v1/receipts/{number}` | Send read/delivery receipt |

### Groups

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/groups/{number}` | List all groups |
| POST | `/v1/groups/{number}` | Create a group |
| GET | `/v1/groups/{number}/{groupid}` | Get group details |
| PUT | `/v1/groups/{number}/{groupid}` | Update group (name, description, expiration) |
| DELETE | `/v1/groups/{number}/{groupid}` | Delete/leave group |
| POST | `/v1/groups/{number}/{groupid}/members` | Add members |
| DELETE | `/v1/groups/{number}/{groupid}/members` | Remove members |
| POST | `/v1/groups/{number}/{groupid}/admins` | Add admins |
| DELETE | `/v1/groups/{number}/{groupid}/admins` | Remove admins |
| POST | `/v1/groups/{number}/{groupid}/join` | Join group |
| POST | `/v1/groups/{number}/{groupid}/quit` | Quit group |
| POST | `/v1/groups/{number}/{groupid}/block` | Block group |

### Contacts

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/contacts/{number}` | List contacts |
| GET | `/v1/contacts/{number}/{recipient}` | Get one contact |
| PUT | `/v1/contacts/{number}` | Update contact |
| POST | `/v1/contacts/{number}/sync` | Sync contacts |

### Accounts

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/accounts` | List registered accounts |
| POST | `/v1/register/{number}` | Register a number |
| POST | `/v1/register/{number}/verify/{token}` | Verify registration |
| POST | `/v1/unregister/{number}` | Unregister |
| POST | `/v1/accounts/{number}/rate-limit-challenge` | Submit rate-limit challenge |
| PUT | `/v1/accounts/{number}/settings` | Update trust settings |
| POST | `/v1/accounts/{number}/pin` | Set PIN |
| DELETE | `/v1/accounts/{number}/pin` | Remove PIN |
| POST | `/v1/accounts/{number}/username` | Set username |
| DELETE | `/v1/accounts/{number}/username` | Remove username |

### Devices

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/devices/{number}` | List linked devices |
| POST | `/v1/devices/{number}` | Link a new device |
| DELETE | `/v1/devices/{number}/{device_id}` | Remove a device |
| DELETE | `/v1/devices/{number}/local-data` | Delete local account data |
| GET | `/v1/qrcodelink` | Get QR code link URI |
| GET | `/v1/qrcodelink/raw` | Get raw link URI (plain text) |

### Identities

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/identities/{number}` | List identity keys |
| PUT | `/v1/identities/{number}/trust/{number_to_trust}` | Trust an identity |

### Profiles

| Method | Endpoint | What it does |
|--------|----------|--------------|
| PUT | `/v1/profiles/{number}` | Update your profile (name, about, avatar) |

### Polls

| Method | Endpoint | What it does |
|--------|----------|--------------|
| POST | `/v1/polls/{number}` | Create and send a poll |
| POST | `/v1/polls/{number}/vote` | Vote on a poll |
| DELETE | `/v1/polls/{number}` | Close a poll |

### Stickers

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/sticker-packs/{number}` | List installed sticker packs |
| POST | `/v1/sticker-packs/{number}` | Install a sticker pack |

### Attachments

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/attachments` | List cached attachments |
| GET | `/v1/attachments/{id}` | Get an attachment |
| DELETE | `/v1/attachments/{id}` | Delete an attachment |

### Configuration

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/configuration` | Get global config |
| POST | `/v1/configuration` | Set global config |
| GET | `/v1/configuration/{number}/settings` | Get account config |
| POST | `/v1/configuration/{number}/settings` | Set account config |

### Search

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/search/{number}?numbers=+111,+222` | Check if numbers are on Signal |

### Webhooks

Register HTTP callbacks to get notified of incoming messages:

| Method | Endpoint | What it does |
|--------|----------|--------------|
| POST | `/v1/webhooks` | Register a webhook |
| GET | `/v1/webhooks` | List webhooks |
| DELETE | `/v1/webhooks/{id}` | Remove a webhook |

```bash
# Register a webhook
curl -X POST http://localhost:8080/v1/webhooks \
  -H 'Content-Type: application/json' \
  -d '{"url": "https://your-app.com/signal-hook"}'
```

### Events (SSE)

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/events/{number}` | Server-Sent Events stream |

### System

| Method | Endpoint | What it does |
|--------|----------|--------------|
| GET | `/v1/health` | Health check (returns 204) |
| GET | `/v1/about` | Version info |
| GET | `/v1/openapi.json` | OpenAPI 3.0 spec |
| GET | `/metrics` | Prometheus metrics |

## Monitoring

Prometheus metrics at `/metrics`:

```
signal_messages_sent_total 42
signal_messages_received_total 108
signal_rpc_calls_total 312
signal_rpc_errors_total 0
signal_ws_clients_active 2
```

## Building from source

```bash
git clone <this-repo>
cd signal-cli-api
cargo build --release
# Binary at target/release/signal-cli-api
```

## Running tests

```bash
cargo test
```

215 tests run against a mock signal-cli server. No Signal account needed.

## License

MIT
