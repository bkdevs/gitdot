# Build stage
FROM rust:1.91-bookworm AS builder

WORKDIR /app

# cmake needed by aws-lc-rs (rustls crypto provider)
RUN apt-get update && \
    apt-get install -y cmake pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release -p s2-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# curl for health checks; ca-certificates for TLS to GCP
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/s2-server /app/s2-server

EXPOSE 8081

CMD ["/app/s2-server", "--port", "8081"]
