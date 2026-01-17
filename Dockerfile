# Build stage
FROM rust:1.92.0-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY core/Cargo.toml ./core/
COPY cli/Cargo.toml ./cli/

# Create dummy source files to cache dependencies
RUN mkdir -p backend/src core/src cli/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "fn main() {}" > cli/src/main.rs && \
    echo "" > core/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release -p gitdot_server

# Remove dummy files
RUN rm -rf backend/src core/src cli/src

# Copy actual source code
COPY backend/src ./backend/src
COPY core/src ./core/src
COPY cli/src ./cli/src

# Build the application
RUN cargo build --release -p gitdot_server

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
COPY --from=builder /app/target/release/gitdot_server /app/gitdot_server

# Set environment variables with defaults for Cloud Run
ENV GIT_PROJECT_ROOT="/srv/git"
ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT="8080"

# Expose port 8080 (Cloud Run default)
EXPOSE 8080

# Run the binary
CMD ["/app/gitdot_server"]
