# List available commands
default:
    @just --list

alias b := build-all
alias l:= lint-all
alias t:= test-all

DB_CONTAINER := "gitdot-db"
KAFKA_CONTAINER := "gitdot-kafka"


# ── Install ───────────────────────────────────────────────────────────────────

# Install dependencies and build (requires pnpm, cargo, and docker)
install:
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
    if ! command -v docker &> /dev/null; then
        echo "Error: docker is not installed."
        echo "Please install Docker: https://docs.docker.com/get-docker/"
        exit 1
    fi
    echo "Running pnpm install"
    cd gitdot-web && pnpm install
    echo "Running cargo build"
    cd .. && cargo build
    echo "Starting local Postgres..."
    just db
    echo "Running migrations..."
    just migrate
    echo "Starting local Kafka..."
    just kafka
    echo "Install complete!"

# ── Database ─────────────────────────────────────────────────────────────────

# Start local Postgres in Docker (idempotent)
db:
    #!/usr/bin/env bash
    set -e
    if ! docker info &>/dev/null; then
        echo "Error: Docker is not running."
        exit 1
    fi
    if docker ps -q -f name={{DB_CONTAINER}} | grep -q .; then
        echo "Postgres already running."
    elif docker ps -aq -f name={{DB_CONTAINER}} | grep -q .; then
        echo "Starting existing container '{{DB_CONTAINER}}'..."
        docker start {{DB_CONTAINER}}
        sleep 1
    else
        echo "Creating Postgres container '{{DB_CONTAINER}}'..."
        docker run -d \
            --name {{DB_CONTAINER}} \
            -e POSTGRES_USER=postgres \
            -e POSTGRES_PASSWORD=postgres \
            -e POSTGRES_DB=gitdot \
            -p 5432:5432 \
            postgres:16
        echo "Waiting for Postgres to be ready..."
        sleep 3
    fi

# Stop local Postgres container
db-stop:
    docker stop {{DB_CONTAINER}}

# ── Kafka ────────────────────────────────────────────────────────────────────

# Start local Kafka (KRaft, single broker) in Docker (idempotent)
kafka:
    #!/usr/bin/env bash
    set -e
    if ! docker info &>/dev/null; then
        echo "Error: Docker is not running."
        exit 1
    fi
    if docker ps -q -f name={{KAFKA_CONTAINER}} | grep -q .; then
        echo "Kafka already running."
    elif docker ps -aq -f name={{KAFKA_CONTAINER}} | grep -q .; then
        echo "Starting existing container '{{KAFKA_CONTAINER}}'..."
        docker start {{KAFKA_CONTAINER}}
        sleep 1
    else
        echo "Creating Kafka container '{{KAFKA_CONTAINER}}'..."
        docker run -d \
            --name {{KAFKA_CONTAINER}} \
            -e KAFKA_NODE_ID=1 \
            -e KAFKA_PROCESS_ROLES=broker,controller \
            -e KAFKA_LISTENERS=PLAINTEXT://:9092,CONTROLLER://:9093 \
            -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 \
            -e KAFKA_CONTROLLER_LISTENER_NAMES=CONTROLLER \
            -e KAFKA_CONTROLLER_QUORUM_VOTERS=1@localhost:9093 \
            -e KAFKA_LISTENER_SECURITY_PROTOCOL_MAP=CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT \
            -e KAFKA_INTER_BROKER_LISTENER_NAME=PLAINTEXT \
            -e KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1 \
            -e KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR=1 \
            -e KAFKA_TRANSACTION_STATE_LOG_MIN_ISR=1 \
            -e KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS=0 \
            -p 9092:9092 \
            apache/kafka:3.9.0
        echo "Waiting for Kafka to be ready..."
        sleep 5
    fi

# Stop local Kafka container
kafka-stop:
    docker stop {{KAFKA_CONTAINER}}

# ── Dev (run services) ──────────────────────────────────────────────────────

# Start frontend, backend, and s2-server in a tmux session
dev:
    #!/usr/bin/env bash
    set -e
    SESSION_NAME="gitdot"

    echo "Starting local Postgres..."
    just db

    if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
        echo "Killing existing tmux session '$SESSION_NAME'..."
        tmux kill-session -t "$SESSION_NAME"
        sleep 0.2 # Give tmux a heartbeat to clear the process
    fi

    PROJECT_ROOT="{{justfile_directory()}}"
    echo "Starting tmux session '$SESSION_NAME'..."

    tmux new-session -d -s "$SESSION_NAME" -c "$PROJECT_ROOT/gitdot-web" -n "gitdot-web" "pnpm run dev"
    tmux new-window -d -t "${SESSION_NAME}:" -c "$PROJECT_ROOT/gitdot-server" -n "gitdot-server" "cargo run"
    tmux new-window -d -t "${SESSION_NAME}:" -c "$PROJECT_ROOT/gitdot-auth" -n "gitdot-auth" "PORT=8082 cargo run"
    tmux new-window -d -t "${SESSION_NAME}:" -c "$PROJECT_ROOT/s2-server" -n "s2" "cargo run -- --port 8081"

    tmux attach-session -t "$SESSION_NAME"

# Run frontend dev server
web:
    cd gitdot-web && pnpm dev

# Run backend server
server:
    cd gitdot-server && cargo run

# Run auth server
auth:
    cd gitdot-auth && PORT=8082 cargo run

# Run s2-server
s2:
    cd s2-server && cargo run -- --port 8081

# Run gitdot CLI with arguments
cli *args:
    cd gitdot-cli && cargo run -- {{args}}

# Generate Ed25519 key pair and write to gitdot-server/.env and s2-server/.env
keygen:
    cargo run -p gitdot-server --bin gitdot-keygen

# ── Build ───────────────────────────────────────────────────────────────────

# Build everything
build: build-all

# Build all Rust crates and TS packages
build-all: build-api build-api-derive build-cli build-config build-core build-server build-s2-api build-s2-common build-s2-server build-s2-sdk build-web

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

# Build the s2-server crate
build-s2-server:
    cargo build -p s2-server

# Build the s2-sdk crate
build-s2-sdk:
    cargo build -p s2-sdk

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

# Lint and format all Rust crates and TS packages
lint-all: lint-api lint-api-derive lint-cli lint-config lint-core lint-server lint-s2-api lint-s2-common lint-s2-server lint-s2-sdk lint-web

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

# Lint and format s2-server
lint-s2-server: _ensure-nightly
    cargo +nightly fmt -p s2-server

# Lint and format s2-sdk
lint-s2-sdk: _ensure-nightly
    cargo +nightly fmt -p s2-sdk

# Lint and format web
lint-web:
    cd gitdot-web && pnpm biome check . --write --unsafe

# Type check all Rust crates
check:
    cargo check

# ── Lint & Format ──────────────────────────────────────────────────────────

# Run migrations
migrate:
    cd gitdot-server && sqlx migrate run --source ../gitdot-core/migrations

# ── Docker ─────────────────────────────────────────────────────────────────

REGISTRY := "us-central1-docker.pkg.dev/gitdot/gitdot"

# Configure Docker to authenticate with GCP Artifact Registry (one-time)
docker-auth:
    gcloud auth configure-docker us-central1-docker.pkg.dev

# Build and push server + auth Docker images
docker-push: (_docker-push "gitdot-server") (_docker-push "gitdot-auth")

_docker-push name:
    #!/usr/bin/env bash
    set -e
    SHA=$(git rev-parse --short HEAD)
    echo "Building {{name}} (${SHA})..."
    docker build --platform linux/amd64 \
        -t {{REGISTRY}}/{{name}}:latest \
        -t {{REGISTRY}}/{{name}}:${SHA} \
        -f {{name}}/Dockerfile .
    echo "Pushing {{name}}..."
    docker push {{REGISTRY}}/{{name}}:latest
    docker push {{REGISTRY}}/{{name}}:${SHA}

# ── Clean ──────────────────────────────────────────────────────────────────

clean:
    #!/usr/bin/env bash
    set -e
    echo "Running cargo clean..."
    cargo clean
    echo "Removing node_modules directory..."
    rm -rf gitdot-web/node_modules
    echo "Removing .next directory"
    rm -rf gitdot-web/.next
    echo "Clean complete."

# ── Helpers (private) ──────────────────────────────────────────────────────

_ensure-nightly:
    @rustup toolchain list | grep -q nightly || (echo "Nightly toolchain required. Run: rustup toolchain install nightly" && exit 1)
