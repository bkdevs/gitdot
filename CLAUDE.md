# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Gitdot is a GitHub alternative for open-source maintainers. It's a full-stack application with a Rust backend (Axum), TypeScript frontend (Next.js), and CLI tool.

## Build & Run Commands

### Backend (Rust)
```bash
cargo check                          # Type check all crates
cargo build -p gitdot_server         # Build backend server
cargo run -p gitdot_server           # Run backend (reads backend/.env)
cargo test -p gitdot_core            # Run core tests
cargo +nightly fmt                   # Format code with rustfmt
```

### Frontend (TypeScript)
```bash
cd frontend
pnpm dev                             # Dev server
pnpm build                           # Production build
pnpm test                            # Jest tests
pnpm biome check .                   # Lint & format check
pnpm biome check . --write           # Auto-fix lint & format
```

### Full Dev Environment
```bash
./scripts/dev.sh                     # Starts tmux with frontend + backend
```

## Workspace Structure

Six Rust crates in the workspace:

- **`core`** (`gitdot_core`) — Business logic, services, repositories, models, DB migrations. The bulk of backend logic lives here.
- **`backend`** (`gitdot_server`) — Axum HTTP handlers, routing, auth middleware. Thin layer that delegates to core services.
- **`api`** (`gitdot_api`) — Shared API resource types and endpoint request/response definitions.
- **`api_derive`** — Proc macro crate providing `#[derive(ApiResource)]`.
- **`cli`** (`gitdot_cli`) — CLI tool (clap-based).
- **`runner`** (`gitdot_runner`) — CI/CD task runner.

Plus `frontend/` — Next.js 16, React 19, App Router.

## Architecture

### Backend Layered Architecture
```
Handler (backend) → Service (core) → Repository (core) → PostgreSQL (sqlx)
```

- **Handlers** receive HTTP requests, extract params, call services, map responses via `IntoApi` trait
- **Services** contain business logic, defined as traits with `Impl` structs
- **Repositories** are the data access layer, also trait-based
- **DTOs** flow between layers; `IntoApi` converts core DTOs to API resource types

### Feature Flags
The backend uses cargo features `main` and `ci` (both on by default):
- `main` — Core platform: git HTTP, repos, users, orgs, questions, oauth
- `ci` — CI/CD: runners, DAGs, tasks

Feature gates only live in `app.rs` (routing) and `app_state.rs` (service construction).

### Git HTTP Protocol
The backend implements smart HTTP git protocol by shelling out to `git http-backend` CGI. Repos are stored as bare git repos under `GIT_PROJECT_ROOT`.

### Frontend Patterns
- Server components and server actions for data fetching (`app/actions.ts`, `app/lib/dal/`)
- Supabase for auth, custom backend API for application data
- `@/ui/link.tsx` wraps Next.js Link — use it instead of `next/link` directly (enforced by Biome)
- Radix UI primitives + Tailwind for components

## Code Conventions

### Rust Import Ordering (enforced by rustfmt.toml)
```rust
// 1. mod declarations
// 2. std imports
// 3. 3rd-party crate imports
// 4. Workspace imports (gitdot_api, gitdot_core)
// 5. crate/super imports
// 6. pub use re-exports
// 7. Logic
```

Use `imports_granularity = "Crate"` — merge imports from the same crate.

### Database
PostgreSQL via sqlx with compile-time checked queries. Migrations in `core/migrations/`.

### Environment
Backend config via `backend/.env` — key vars: `GIT_PROJECT_ROOT`, `DATABASE_URL`, `SUPABASE_JWT_PUBLIC_KEY`.
