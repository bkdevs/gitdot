# Build stage
FROM rust:1.92.0-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Copy everything
COPY . .

# Build the application
RUN cargo build --release -p gitdot-auth

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install ca-certificates for TLS
RUN apt-get update && \
  apt-get install -y ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gitdot-auth /app/gitdot-auth

ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT="8082"

EXPOSE 8082

CMD ["/app/gitdot-auth"]
