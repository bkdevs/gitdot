# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-auth` is the standalone Axum HTTP server for authentication flows: device-code login, GitHub OAuth, session refresh, email verification, and Slack account linking. It's a thin layer — handlers extract request params, call `AuthenticationService` (defined in `gitdot-core`), and map responses. No business logic lives here.

## Structure

- `app.rs` — Router construction, middleware stack, `GitdotAuthServer` entrypoint
- `app/settings.rs` — `Settings` loaded via figment from env vars (all secrets included; no runtime Secret Manager fetches)
- `app/state.rs` — `AppState` with `AuthenticationService` and `Arc<Settings>`
- `app/bootstrap.rs` — thin startup orchestrator; delegates to `gitdot_axum::bootstrap` (env, crypto provider, tracing)
- `app/error.rs` — `AppError` enum mapping core errors to HTTP status codes
- `app/response.rs` — `AppResponse<T: ApiResource>` wrapper for typed JSON responses
- `extract/` — `Principal` JWT extractor, `ClientIp`, `UserAgent`
- `handler/` — One module per domain (device, github, email, slack, logout, refresh_session), one file per endpoint
- `dto.rs` — `IntoApi` trait + impls converting core DTOs to API resources
- `bin/main.rs` — Entry point: `GitdotAuthServer::new().await?.start().await`

## Configuration

All settings — including every secret — are loaded from environment variables at startup via figment (`Figment::new().merge(Env::raw()).extract()`). In production these are injected via Cloud Run's Secret Manager bindings; locally they come from `.env`.

- Required fields have no default — figment fails fast with "missing field X" if unset
- Optional fields use `#[serde(default = "fn_name")]`
- `port: u16` (not `String`) so figment's numeric auto-detect works with `PORT=8082`

See `.env.example` for the full var list.

## Adding a New Handler

1. Create `handler/{domain}/{action}.rs` with an async function
2. Use axum extractors: `State(state): State<AppState>`, `Path(...)`, `Query(...)`, `Json(...)`. Use `Principal` for JWT-authenticated routes.
3. Call the appropriate service method from `state.authentication_service`, map errors with `AppError::from`, wrap response with `AppResponse::new(StatusCode, dto.into_api())`
4. Register route in `handler/{domain}.rs` router function
5. Merge router in `handler.rs` `create_auth_router()`

## Rust Import Ordering

```rust
// 1. mod declarations
mod create_device_code;

// 2. std imports
use std::sync::Arc;

// 3. 3rd-party crate imports
use axum::{extract::{Path, State}, http::StatusCode, Json};

// 4. Workspace crate imports
use gitdot_api::endpoint::auth::device::create_device_code as api;
use gitdot_core::dto::DeviceCodeRequest;

// 5. crate/super/self imports
use crate::app::{AppError, AppResponse, AppState};

// 6. pub use re-exports
pub use device::*;
```

Separate each group with a blank line. Merge imports from the same crate (`imports_granularity = "Crate"`). All imports and re-exports must come before any declarations or logic.

## Conventions

- Handlers are one function per file, no business logic — delegate to `AuthenticationService`
- `IntoApi` converts core response DTOs to API resource types (in `dto.rs`)
- `Principal` is the JWT-authenticated extractor; verifies the bearer token against `settings.gitdot_public_key`
- CORS allow-origin is built from `settings.gitdot_web_url` — single origin per environment
- Tower governor rate-limits all routes (2 rps, burst 10)
