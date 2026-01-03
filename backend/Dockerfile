# Build stage
FROM rust:1.92.0-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
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

# Copy the binary from builder
COPY --from=builder /app/target/release/gitdot /app/gitdot

# Create directory for git repositories
RUN mkdir -p /srv/git && chmod 755 /srv/git

# Set environment variables with defaults for Cloud Run
ENV DATABASE_URL=""
ENV GIT_PROJECT_ROOT="/srv/git"
ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT="8080"

# Expose port 8080 (Cloud Run default)
EXPOSE 8080

# Run the binary
CMD ["/app/gitdot"]
