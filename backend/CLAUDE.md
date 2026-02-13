# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot_server` is the Axum HTTP server. It's a thin layer — handlers extract request params, call core services, and map responses. Business logic lives in `gitdot_core`.

## Structure

- `app.rs` — Router construction, middleware stack, `GitdotServer` entrypoint
- `app/app_state.rs` — `AppState` with all service instances. Feature-gated fields for `main` vs `ci`.
- `app/auth.rs` — `AuthenticatedUser<S>` extractor with JWT and token-based auth schemes (sealed trait pattern)
- `app/error.rs` — `AppError` enum mapping core errors to HTTP status codes
- `app/response.rs` — `AppResponse<T: ApiResource>` wrapper for typed JSON responses
- `handler/` — One module per domain, one file per endpoint
- `dto/` — `IntoApi` trait + impls converting core DTOs to API resources
- `dto/git_http.rs` — Git HTTP protocol response types

## Adding a New Handler

1. Create `handler/{domain}/{action}.rs` with an async function
2. Use axum extractors: `State(state): State<AppState>`, `Path(...)`, `Query(...)`, `Json(...)`
3. Call the appropriate service from `state`, map errors with `AppError::from`, wrap response with `AppResponse::new(StatusCode, dto.into_api())`
4. Register route in `handler/{domain}.rs` router function
5. Merge router in `app.rs` `create_router()`

## Features

- `main` (default) — Core platform: git HTTP, repos, users, orgs, questions, oauth
- `ci` (default) — CI/CD: runners, DAGs, tasks

Feature gates live only in `app.rs` (routing) and `app_state.rs` (fields/construction). Handler modules are always compiled.

## Rust Import Ordering

```rust
// 1. mod declarations
mod create_repository;

// 2. std imports
use std::sync::Arc;

// 3. 3rd-party crate imports
use axum::{extract::{Path, State}, http::StatusCode, Json};

// 4. Workspace crate imports
use gitdot_api::endpoint::create_repository as api;
use gitdot_core::dto::CreateRepositoryRequest;

// 5. crate/super/self imports
use crate::app::{AppError, AppResponse, AppState};

// 6. pub use re-exports
pub use repository::*;
```

Separate each group with a blank line. Merge imports from the same crate (`imports_granularity = "Crate"`). All imports and re-exports must come before any declarations or logic (structs, fns, impls, traits).

## Conventions

- Handlers are one function per file, no business logic — delegate to core services
- `IntoApi` converts core response DTOs to API resource types (in `dto/`)
- Auth: `AuthenticatedUser` (required), `Option<AuthenticatedUser>` (optional), `AuthenticatedUser<Jwt>` (JWT-only)
