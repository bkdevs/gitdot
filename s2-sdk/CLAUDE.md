# CLAUDE.md — s2-sdk

Rust SDK for [S2](https://s2.dev/), a durable streams platform. Provides async Rust bindings for managing basins, streams, and records.

## Build & Test

```bash
cargo check -p s2-sdk           # Type check
cargo build -p s2-sdk           # Build
cargo test -p s2-sdk            # Run tests (requires S2 credentials)
cargo +nightly fmt              # Format (nightly rustfmt)
```

## Module Overview

| Module | Role |
|--------|------|
| `ops.rs` | Public API: `S2`, `S2Basin`, `S2Stream` |
| `client.rs` | HTTP transport, connection pooling, auth |
| `api.rs` | `AccountClient` / `BasinClient`; protobuf + S2S protocol |
| `types.rs` | `S2Config`, `S2Error`, input/output DTOs |
| `producer.rs` | High-level `Producer`: per-record submission with auto-batching |
| `batching.rs` | `BatchingConfig`, `AppendInputs` — linger, byte/record limits |
| `session/append.rs` | `AppendSession`: pipelined HTTP/2 appends with backpressure |
| `session/read.rs` | `read_session`: streaming reads with retry + heartbeat |
| `retry.rs` | `RetryBackoff` with exponential backoff + jitter |

## Architecture

```
S2 / S2Basin / S2Stream  (ops.rs — public API)
        ↓
AccountClient / BasinClient  (api.rs — S2S protocol, protobuf)
        ↓
BaseClient  (client.rs — hyper, rustls, compression)
        ↓
S2 HTTPS endpoints
```

- **S2Config**: holds `account_endpoint` and `basin_endpoint`; `endpoint()` expands both
- **AppendSession**: multiplexed HTTP/2, semaphore-based backpressure on `max_unacked_bytes`, returns `BatchSubmitTicket` futures
- **Producer**: wraps AppendSession; batches individual records via `tokio-muxt`, returns `RecordSubmitTicket` per record
- **ReadSession**: auto-reconnects on timeout; heartbeat frames detect stalled servers
- **Pagination**: `list_all_streams()` returns `Streaming<StreamInfo>` using cursor-based `start_after` paging

## Key Patterns

- Builder methods: `.with_*()` on config types (`S2Config`, `ProducerConfig`, `AppendSessionConfig`)
- Error handling: `S2Error` enum; sub-errors (`ApiError`, `AppendSessionError`) convert into it
- Async streams: `async_stream::try_stream!` for generators; `Streaming<T>` = `Pin<Box<dyn Stream<Item = Result<T, _>>>>`
- Idempotency: create operations send `S2_REQUEST_TOKEN` (UUID) header
- Serialization: `prost` for protobuf; `serde`/`serde_json` for REST

## Tests

Integration tests in `tests/` (`account_ops.rs`, `basin_ops.rs`, `stream_ops.rs`). Require live S2 credentials. Use `test_context` fixtures and `tokio_shared_rt` for a shared Tokio runtime.

## Dependencies (notable)

- Transport: `hyper` 1.x, `hyper-rustls` (aws-lc-rs), `hyper-util`
- Async: `tokio`, `futures`, `async-stream`, `async-compression`
- Serialization: `prost`, `serde`, `serde_urlencoded`
- Workspace: `s2-api` (shared protobuf types), `s2-common` (validation, caps)
- Time/IDs: `time` (RFC 3339), `uuid`
