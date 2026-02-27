# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Gitdot is a GitHub alternative for open-source maintainers. It's a full-stack application with a Rust backend (Axum), TypeScript frontend (Next.js), and CLI tool.

## Build & Run Commands

### Backend (Rust)
```bash
cargo check                          # Type check all crates
cargo build -p gitdot-server         # Build backend server
cargo run -p gitdot-server           # Run backend (reads gitdot-server/.env)
cargo test -p gitdot-core            # Run core tests
cargo +nightly fmt                   # Format code with rustfmt
```

### Frontend (TypeScript)
```bash
cd gitdot-web
pnpm dev                             # Dev server
pnpm build                           # Production build
pnpm test                            # Jest tests
pnpm biome check .                   # Lint & format check
pnpm biome check . --write           # Auto-fix lint & format
```

### Full Dev Environment
```bash
just dev                             # Starts tmux with frontend + backend + s2-lite
just web                             # Frontend only
just server                          # Backend only
```

## Workspace Structure

Rust crates in the workspace:

- **`gitdot-api`** — Shared API resource types and endpoint request/response definitions.
- **`gitdot-api/derive`** (`api_derive`) — Proc macro crate providing `#[derive(ApiResource)]`.
- **`gitdot-core`** — Business logic, services, repositories, models, DB migrations. The bulk of backend logic lives here.
- **`gitdot-server`** — Axum HTTP handlers, routing, auth middleware. Thin layer that delegates to core services.
- **`gitdot-cli`** — CLI tool (clap-based).
- **`gitdot-config`** — Shared configuration types.
- **`s2-api`**, **`s2-common`**, **`s2-lite`**, **`s2-sdk`** — S2 durable streams library crates.

TypeScript packages (pnpm workspace):

- **`gitdot-web`** — Next.js 16, React 19, App Router.
- **`gitdot-api-ts`** — TypeScript/Zod mirror of `gitdot-api` for frontend type safety.

## Architecture

### Backend Layered Architecture
```
Handler (backend) → Service (core) → Repository (core) → PostgreSQL (sqlx)
```

- **Handlers** receive HTTP requests, extract params, call services, map responses via `IntoApi` trait
- **Services** contain business logic, defined as traits with `Impl` structs
- **Repositories** are the data access layer, also trait-based
- **DTOs** flow between layers; `IntoApi` converts core DTOs to API resource types

### Git HTTP Protocol
The backend implements smart HTTP git protocol by shelling out to `git http-backend` CGI. Repos are stored as bare git repos under `GIT_PROJECT_ROOT`.

### Frontend Patterns
- Server components and server actions for data fetching (`app/actions/`, `app/dal/`)
- Supabase for auth, custom backend API for application data
- `@/ui/link.tsx` wraps Next.js Link — use it instead of `next/link` directly (enforced by Biome)
- Radix UI primitives + Tailwind for components
- `gitdot-api-ts` package provides Zod schemas matching the Rust API types

## Code Conventions

### Rust Import Ordering (enforced by rustfmt.toml)
```rust
// 1. mod declarations
// 2. std imports
// 3. 3rd-party crate imports
// 4. Workspace imports (gitdot-api, gitdot-core)
// 5. crate/super imports
// 6. pub use re-exports
// 7. Logic
```

Use `imports_granularity = "Crate"` — merge imports from the same crate.

### Database
PostgreSQL via sqlx with compile-time checked queries. Migrations in `gitdot-core/migrations/`.

### Environment
Backend config via `gitdot-server/.env` — key vars: `GIT_PROJECT_ROOT`, `DATABASE_URL`, `SUPABASE_JWT_PUBLIC_KEY`.
