# List available commands
default:
    @just --list

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
    cd frontend && pnpm install
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
    fi
    PROJECT_ROOT="{{justfile_directory()}}"
    echo "Starting tmux session '$SESSION_NAME'..."
    tmux new-session -d -s "$SESSION_NAME" -c "$PROJECT_ROOT/frontend" -n "frontend" "pnpm run dev"
    tmux new-window -t "$SESSION_NAME" -c "$PROJECT_ROOT/backend" -n "backend" "cargo run"
    tmux new-window -t "$SESSION_NAME" -c "$PROJECT_ROOT/s2/lite" -n "s2" "cargo run -- --port 8081"
    tmux attach-session -t "$SESSION_NAME"

# Run frontend dev server
frontend:
    cd frontend && pnpm dev

# Run backend server
backend:
    cargo run -p gitdot-server

# Run s2-lite server
s2:
    cargo run -p s2-lite -- --port 8081

# ── Build ───────────────────────────────────────────────────────────────────

# Build everything
build: build-all

# Build all (backend + cli + frontend)
build-all: build-backend build-cli build-frontend

# Build the backend server
build-backend:
    cargo build -p gitdot-server

# Build the CLI
build-cli:
    cargo build -p gitdot-cli

# Build frontend for production
build-frontend:
    cd frontend && pnpm build

# ── Test ────────────────────────────────────────────────────────────────────

# Run all tests
test: test-all

# Run all tests (backend + frontend)
test-all: test-backend test-frontend

# Run backend (core) tests
test-backend:
    cargo test -p gitdot-core

# Run frontend tests
test-frontend:
    cd frontend && pnpm test

# ── Lint & Format ──────────────────────────────────────────────────────────

# Lint and format everything
lint: lint-all

# Lint and format all (backend + frontend)
lint-all: lint-backend lint-frontend

# Lint and format Rust code
lint-backend: _ensure-nightly
    cargo +nightly fmt

# Lint and format frontend
lint-frontend:
    cd frontend && pnpm biome check . --write

# Type check all Rust crates
check:
    cargo check

# ── Helpers (private) ──────────────────────────────────────────────────────

_ensure-nightly:
    @rustup toolchain list | grep -q nightly || (echo "Nightly toolchain required. Run: rustup toolchain install nightly" && exit 1)
