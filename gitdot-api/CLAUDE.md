# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-api` defines the shared API contract — resource types (response shapes) and endpoint definitions (request/response pairs with HTTP metadata). Used by both the backend (to type handlers) and potentially clients.

## Structure

- `resource/` — Data structs returned by the API. One file per domain (user, repository, question, etc.).
- `endpoint/` — One submodule per endpoint. Each defines a ZST implementing the `Endpoint` trait plus request/response types.
- `ApiResource` — Marker trait (`Serialize + PartialEq`) with blanket impls for `Vec<T>`, `Option<T>`, and `()`. Use `#[derive(ApiResource)]` from `api_derive`.

## Adding a New Endpoint

1. Create resource struct in `resource/{domain}.rs` with `#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]`
2. Create `endpoint/{domain}/{action}.rs` with:
   - A ZST struct (e.g., `pub struct GetUser;`)
   - `impl Endpoint` with `PATH`, `METHOD`, `Request`, `Response`
   - Request struct with `Serialize + Deserialize`
   - Response type (usually a type alias to a resource)
3. Re-export from parent `endpoint/{domain}.rs` module

## Rust Import Ordering

```rust
// 1. mod declarations
mod foo;

// 2. std imports
use std::collections::HashMap;

// 3. 3rd-party crate imports
use serde::{Deserialize, Serialize};

// 4. Workspace crate imports (api_derive)
use api_derive::ApiResource;

// 5. crate/super/self imports
use crate::resource::user::UserResource;

// 6. pub use re-exports
pub use user::*;
```

Separate each group with a blank line. Merge imports from the same crate (`imports_granularity = "Crate"`). All imports and re-exports must come before any declarations or logic (structs, fns, impls, traits).

## Conventions

- Response types are type aliases to resources: `pub type GetUserResponse = UserResource;`
- Empty request bodies use an empty struct: `pub struct GetUserRequest {}`
- Default values for optional query params use serde defaults: `#[serde(default = "default_ref")]`
- IDs are `uuid::Uuid`, timestamps are `chrono::DateTime<Utc>`
