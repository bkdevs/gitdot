# syntax=docker/dockerfile:1.7

# ---- chef base ----
# cargo-chef separates dependency compilation into its own cached layer so
# `docker build` only recompiles deps when Cargo.toml/Cargo.lock changes.
FROM rust:1.92.0-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

# ---- planner: compute the recipe (cheap, just inspects Cargo.toml's) ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- builder ----
FROM chef AS builder

# Install build dependencies (cmake/perl needed for rdkafka + vendored openssl)
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev cmake perl && \
  rm -rf /var/lib/apt/lists/*

# 1. Bake all dependencies into a separate cached layer. This is the slow step
#    (librdkafka + openssl + everything else); cached until Cargo.* changes.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json -p gitdot-server

# 2. Copy real source and build only workspace crates.
COPY . .
RUN cargo build --release -p gitdot-server

# ---- runtime ----
FROM debian:bookworm-slim

WORKDIR /app

# Install git and ca-certificates
RUN apt-get update && \
  apt-get install -y git ca-certificates && \
  rm -rf /var/lib/apt/lists/*

# Configure git globally
RUN git config --system user.name "Git Server" && \
  git config --system user.email "git@gitdot.io" && \
  git config --system http.receivepack true && \
  git config --system uploadpack.allowAnySHA1InWant true

COPY --from=builder /app/target/release/gitdot-server /app/gitdot-server

ENV GIT_PROJECT_ROOT="/srv/git"
ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT="8080"

EXPOSE 8080

CMD ["/app/gitdot-server"]
