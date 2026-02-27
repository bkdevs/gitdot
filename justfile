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

# Build all (server + cli + web)
build-all: build-server build-cli build-web

# Build the backend server
build-server:
    cargo build -p gitdot-server

# Build the CLI
build-cli:
    cargo build -p gitdot-cli

# Build web for production
build-web:
    cd gitdot-web && pnpm build

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

# Lint and format all (server + web)
lint-all: lint-server lint-web

# Lint and format Rust code
lint-server: _ensure-nightly
    cargo +nightly fmt

# Lint and format web
lint-web:
    cd gitdot-web && pnpm biome check . --write

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
