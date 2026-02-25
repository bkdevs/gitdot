# List available commands
default:
    @just --list

# ── Gitdot setup & dev ───────────────────────────────────────────────────────

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

# Start frontend + backend in a tmux session
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
    tmux new-window -t "$SESSION_NAME" -c "$PROJECT_ROOT/backend" -n "backend" "cargo run -p gitdot_server"
    tmux select-window -t "$SESSION_NAME:0"
    tmux attach-session -t "$SESSION_NAME"

# ── Gitdot backend ───────────────────────────────────────────────────────────

# Type check all Rust crates
check:
    cargo check

# Build the backend server
build:
    cargo build -p gitdot_server

# Run backend server
backend:
    cargo run -p gitdot_server

# Run core tests
test:
    cargo test -p gitdot_core

# Format Rust code (requires nightly)
fmt: _ensure-nightly
    cargo +nightly fmt

# ── Gitdot frontend ──────────────────────────────────────────────────────────

# Run frontend dev server
frontend:
    cd frontend && pnpm dev

# Build frontend for production
frontend-build:
    cd frontend && pnpm build

# Run frontend tests
frontend-test:
    cd frontend && pnpm test

# Lint and format check frontend
lint:
    cd frontend && pnpm biome check .

# Auto-fix frontend lint and format issues
lint-fix:
    cd frontend && pnpm biome check . --write

# ── s2 ───────────────────────────────────────────────────────────────────────

# Sync git submodules
s2-sync:
    git submodule update --init --recursive

# Build the s2 CLI binary
s2-build *args: s2-sync
    cd s2 && cargo build --locked --release -p s2-cli {{args}}

# Run clippy on s2 workspace
s2-clippy *args: s2-sync
    cd s2 && cargo clippy --workspace --all-features --all-targets {{args}} -- -D warnings --allow deprecated

# Run cargo-deny checks on s2
s2-deny *args: _s2-ensure-deny
    cd s2 && cargo deny check {{args}}

# Format s2 code with rustfmt (requires nightly)
s2-fmt: _ensure-nightly
    cd s2 && cargo +nightly fmt

# Run s2 tests with nextest
s2-test *args: s2-sync _ensure-nextest
    cd s2 && cargo nextest run --workspace --all-features -E 'not (package(s2-cli) & binary(integration))' {{args}}

# Run s2 CLI integration tests (requires s2 lite server running)
s2-test-integration: s2-sync _ensure-nextest
    cd s2 && S2_ACCESS_TOKEN=test S2_ACCOUNT_ENDPOINT=http://localhost S2_BASIN_ENDPOINT=http://localhost \
    cargo nextest run -p s2-cli --test integration

# Verify s2 Cargo.lock is up-to-date
s2-check-locked:
    cd s2 && cargo metadata --locked --format-version 1 >/dev/null

# Clean s2 build artifacts
s2-clean:
    cd s2 && cargo clean

# Run s2-lite
s2-lite *args: s2-sync
    cd s2 && cargo run --release -p s2-cli -- lite {{args}}

# ── Helpers (private) ────────────────────────────────────────────────────────

_ensure-nightly:
    @rustup toolchain list | grep -q nightly || (echo "Nightly toolchain required. Run: rustup toolchain install nightly" && exit 1)

_ensure-nextest:
    @cargo nextest --version > /dev/null 2>&1 || cargo install cargo-nextest

_s2-ensure-deny:
    @cargo deny --version > /dev/null 2>&1 || cargo install cargo-deny
