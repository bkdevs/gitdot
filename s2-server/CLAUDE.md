# CLAUDE.md

This file provides guidance to Claude Code when working with code in the `s2-server` crate.

## Overview

`s2-server` is a lightweight HTTP server implementing the [S2](https://s2.dev) durable streams API, backed by object storage via SlateDB. It manages **basins** (logical containers) and **streams** (append-only logs), with pluggable storage backends (S3, GCP, local filesystem, or in-memory).

## Build & Run

```bash
cargo build -p s2-server          # Build
cargo run -p s2-server            # Run (reads .env for config)
cargo test -p s2-server           # Run tests
docker build -t s2-server .       # Docker build
```

Binary flags:
```
--local-root DIR         Local filesystem backend root
--port PORT              Listen port (default: 443 if TLS, else 80)
--tls-self               Self-signed TLS cert
--tls-cert / --tls-key   Custom TLS cert/key
--no-cors                Disable permissive CORS
--init-file FILE         JSON spec for declarative startup initialization
```

## Architecture

```
HTTP Handlers (src/handlers/v1/)
    ↓
Backend Services (src/backend/{basins,streams,append,read}.rs)
    ↓
KV Schema (src/backend/kv/)
    ↓
SlateDB (src/backend/store.rs)
    ↓
Object Storage (S3 / GCP / Local / Memory)
```

## Directory Structure

```
src/
├── bin/main.rs           Entry point (mimalloc, rustls, tracing, clap)
├── server.rs             Server init (storage backend detection, TLS, Axum setup)
├── auth.rs               JWT auth — Authenticator trait, Principal extractor
├── init.rs               Declarative JSON initialization (ResourcesSpec, BasinSpec, StreamSpec)
├── handlers/
│   └── v1/
│       ├── basins.rs     Basin CRUD HTTP handlers
│       ├── streams.rs    Stream CRUD HTTP handlers
│       ├── records.rs    Record append/read/tail-follow handlers
│       ├── paths.rs      URL path constants
│       └── error.rs      ServiceError → HTTP response mapping
└── backend/
    ├── core.rs           Backend struct (SlateDB, streamer slots, auth, background tasks)
    ├── store.rs          DB access layer (get, put, transactions)
    ├── basins.rs         Basin lifecycle logic
    ├── streams.rs        Stream lifecycle logic
    ├── append.rs         Record append (batching, transactions)
    ├── read.rs           Record read (filtering, pagination, timestamps)
    ├── streamer.rs       Streaming protocol (record encoding, pipelining, fencing)
    ├── error.rs          Domain errors with IntoResponse impl
    ├── stream_id.rs      StreamId type
    ├── kv/               KV schema modules (one per key type)
    └── bgtasks/          Background tasks (trim, delete-on-empty, basin deletion)
```

## Key Patterns

**KV Schema** — Each key type is a separate module in `src/backend/kv/` with `ser_key`, `ser_value`, `deser_key`, `deser_value` functions. Add new key types as new modules; register them in `kv/mod.rs`.

**Error Handling** — Domain errors in `backend/error.rs` implement `IntoResponse` and serialize directly to HTTP. HTTP-layer errors live in `handlers/v1/error.rs`.

**Auth** — `Authenticator` trait in `auth.rs` has two implementations: `S2Server` (internal) and `GitdotServer` (JWT with public key from `GITDOT_PUBLIC_KEY` env var).

**Background Tasks** — Triggered via a broadcast channel (`bgtask_trigger` in `core.rs`). Three tasks: stream trim, delete-on-empty, basin deletion.

**Streamer Slots** — `DashMap<StreamId, StreamerSlot>` in `core.rs` manages concurrent streaming sessions. Core streaming logic lives in `streamer.rs`.

## Environment Variables

| Variable | Purpose |
|---|---|
| `S3_BUCKET` | Use S3 backend |
| `GCP_BUCKET` | Use GCP backend |
| `GITDOT_PUBLIC_KEY` | JWT public key for GitdotServer auth |
| `S2LITE_INIT_FILE` | Path to JSON init spec (alternative to `--init-file`) |

## HTTP Routes

- `GET /health` (also `/ping` for backward compat)
- `/v1/basins` — basin CRUD
- `/v1/{basin}/streams` — stream CRUD
- `/v1/{basin}/{stream}/records` — record append/read

Middleware: gzip/zstd compression, permissive CORS (unless `--no-cors`).
