# List available commands
default:
    @just --list

alias b := build-all
alias l:= lint-all
alias t:= test-all


# ── Setup ───────────────────────────────────────────────────────────────────

# Install dependencies and build (requires pnpm and cargo)
setup:
    #!/usr/bin/env bash
    set -e
    if ! command -v pnpm &> /dev/null; then
        echo "Error: pnpm is not installed."
        echo "Please install pnpm: https://pnpm.io/installation"
        exit 1
    fi
    if ! command -v cargo &> /dev/null; then
        echo "Error: cargo is not installed."
        echo "Please install Rust and cargo: https://rustup.rs/"
        exit 1
    fi
    echo "Running pnpm install"
    cd gitdot-web && pnpm install
    echo "Running cargo build"
    cd .. && cargo build
    echo "Setup complete!"

# ── Dev (run services) ──────────────────────────────────────────────────────

# Start frontend, backend, and s2-lite in a tmux session
dev:
    #!/usr/bin/env bash
    SESSION_NAME="gitdot"
    if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
        echo "Killing existing tmux session '$SESSION_NAME'..."
        tmux kill-session -t "$SESSION_NAME"
        sleep 0.2 # Give tmux a heartbeat to clear the process
    fi

    PROJECT_ROOT="{{justfile_directory()}}"
    echo "Starting tmux session '$SESSION_NAME'..."

    tmux new-session -d -s "$SESSION_NAME" -c "$PROJECT_ROOT/gitdot-web" -n "frontend" "pnpm run dev"
    tmux new-window -d -t "${SESSION_NAME}:" -c "$PROJECT_ROOT/gitdot-server" -n "gitdot-server" "cargo run"
    tmux new-window -d -t "${SESSION_NAME}:" -c "$PROJECT_ROOT/s2-lite" -n "s2" "cargo run -- --port 8081"

    tmux attach-session -t "$SESSION_NAME"

# Run frontend dev server
web:
    cd gitdot-web && pnpm dev

# Run backend server
server:
    cd gitdot-server && cargo run

# Run s2-lite server
s2:
    cargo run -p s2-lite -- --port 8081

# ── Build ───────────────────────────────────────────────────────────────────

# Build everything
build: build-all

# Build all Rust crates and TS packages
build-all: build-api build-api-derive build-cli build-config build-core build-server build-s2-api build-s2-common build-s2-lite build-s2-sdk build-web build-api-ts

# Build the backend server
build-server:
    cargo build -p gitdot-server

# Build the CLI
build-cli:
    cargo build -p gitdot-cli

# Build the config crate
build-config:
    cargo build -p gitdot-config

# Build the core crate
build-core:
    cargo build -p gitdot-core

# Build the API crate
build-api:
    cargo build -p gitdot-api

# Build the API derive crate
build-api-derive:
    cargo build -p gitdot-api-derive

# Build the s2-api crate
build-s2-api:
    cargo build -p s2-api

# Build the s2-common crate
build-s2-common:
    cargo build -p s2-common

# Build the s2-lite crate
build-s2-lite:
    cargo build -p s2-lite

# Build the s2-sdk crate
build-s2-sdk:
    cargo build -p s2-sdk

# Build web for production
build-web:
    cd gitdot-web && pnpm build

# Build gitdot-api-ts (typecheck only)
build-api-ts:
    cd gitdot-api-ts && pnpm typecheck

# ── Test ────────────────────────────────────────────────────────────────────

# Run all tests
test: test-all

# Run all tests (server + web)
test-all: test-server test-web

# Run server (core) tests
test-server:
    cargo test -p gitdot-core

# Run web tests
test-web:
    cd gitdot-web && pnpm test

# ── Lint & Format ──────────────────────────────────────────────────────────

# Lint and format everything
lint: lint-all

# Lint and format all Rust crates and TS packages
lint-all: lint-api lint-api-derive lint-cli lint-config lint-core lint-server lint-s2-api lint-s2-common lint-s2-lite lint-s2-sdk lint-web lint-api-ts

# Lint and format gitdot-api
lint-api: _ensure-nightly
    cargo +nightly fmt -p gitdot-api

# Lint and format gitdot-api-derive
lint-api-derive: _ensure-nightly
    cargo +nightly fmt -p gitdot-api-derive

# Lint and format gitdot-cli
lint-cli: _ensure-nightly
    cargo +nightly fmt -p gitdot-cli

# Lint and format gitdot-config
lint-config: _ensure-nightly
    cargo +nightly fmt -p gitdot-config

# Lint and format gitdot-core
lint-core: _ensure-nightly
    cargo +nightly fmt -p gitdot-core

# Lint and format gitdot-server
lint-server: _ensure-nightly
    cargo +nightly fmt -p gitdot-server

# Lint and format s2-api
lint-s2-api: _ensure-nightly
    cargo +nightly fmt -p s2-api

# Lint and format s2-common
lint-s2-common: _ensure-nightly
    cargo +nightly fmt -p s2-common

# Lint and format s2-lite
lint-s2-lite: _ensure-nightly
    cargo +nightly fmt -p s2-lite

# Lint and format s2-sdk
lint-s2-sdk: _ensure-nightly
    cargo +nightly fmt -p s2-sdk

# Lint and format web
lint-web:
    cd gitdot-web && pnpm biome check . --write

# Lint and format gitdot-api-ts
lint-api-ts:
    cd gitdot-api-ts && pnpm biome check . --write

# Type check all Rust crates
check:
    cargo check

# ── Lint & Format ──────────────────────────────────────────────────────────

# Run migrations
migrate:
    cd gitdot-server && sqlx migrate run --source ../gitdot-core/migrations

# ── Helpers (private) ──────────────────────────────────────────────────────

_ensure-nightly:
    @rustup toolchain list | grep -q nightly || (echo "Nightly toolchain required. Run: rustup toolchain install nightly" && exit 1)
