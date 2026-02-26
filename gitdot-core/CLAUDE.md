# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-core` contains all business logic, data access, and external client integrations. The backend server depends on this crate and calls its services.

## Structure

```
src/
├── service/       Trait + Impl per domain (business logic)
├── repository/    Trait + Impl per domain (sqlx data access)
├── client/        Trait + Impl for external services (git2, git http-backend, difftastic)
├── dto/           Request/response DTOs + validated types
├── model/         Database model structs (#[derive(FromRow)])
├── error/         Domain-specific error enums (thiserror)
└── util/          Internal helpers (reserved names, token generation)
```

Migrations live in `migrations/` (sqlx `.up.sql` / `.down.sql`).

## Rust Import Ordering

```rust
// 1. mod declarations
mod user;

// 2. std imports
use std::sync::Arc;

// 3. 3rd-party crate imports
use async_trait::async_trait;
use sqlx::PgPool;

// 4. Workspace crate imports (none currently — core has no workspace deps)

// 5. crate/super/self imports
use crate::{dto::UserResponse, error::UserError, model::User};

// 6. pub use re-exports
pub use user::*;
```

Separate each group with a blank line. Merge imports from the same crate (`imports_granularity = "Crate"`). All imports and re-exports must come before any declarations or logic (structs, fns, impls, traits).

## Key Patterns

### Service Layer
```rust
#[async_trait]
pub trait UserService: Send + Sync + 'static { ... }

pub struct UserServiceImpl<U: UserRepository, R: RepositoryRepository> { ... }
```
Services are generic over repository traits for testability. Concrete constructors accept `*RepositoryImpl` types.

### Repository Layer
```rust
#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static { ... }

pub struct UserRepositoryImpl { pool: PgPool }
```
Methods return `Result<T, sqlx::Error>`. Use raw SQL with `sqlx::query_as`. Transactions via `pool.begin()`.

### Validated Types (nutype)
`dto/common.rs` defines `OwnerName`, `RepositoryName`, `RunnerName` using the `nutype` crate:
- Auto-sanitize: trim, lowercase, strip `.git` suffix
- Validate: slug format (lowercase alphanumeric + hyphens/underscores, 2-32 chars)
- Request DTO constructors call `try_new()` and map errors to domain errors

### Error Handling
Each domain has an error enum in `error/`. Pattern:
```rust
#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
```

### Data Flow
Request DTO (validated) → Service (business logic) → Repository (SQL) → Model → Response DTO (via `From<Model>`)

## Adding a New Domain

1. **Model**: `model/{domain}.rs` — `#[derive(FromRow)]` struct
2. **Error**: `error/{domain}.rs` — thiserror enum with DatabaseError variant
3. **Repository**: `repository/{domain}.rs` — trait + impl with sqlx queries
4. **DTOs**: `dto/{domain}/` — request structs (with `OwnerName`/etc validation), response struct with `From<Model>` impl
5. **Service**: `service/{domain}.rs` — trait + impl, inject repositories
6. **Migration**: `migrations/` — create table SQL
7. Re-export from parent module files (`model.rs`, `error.rs`, etc.)

## Git Operations

`client/git.rs` (`Git2Client`) wraps the `git2` crate for bare repo operations. Blocking git2 calls are wrapped in `tokio::task::spawn_blocking`. Repos stored at `{GIT_PROJECT_ROOT}/{owner}/{repo}.git`.

`client/git_http.rs` shells out to `git http-backend` for clone/push protocol support.
