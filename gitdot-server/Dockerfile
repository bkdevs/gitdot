# Build stage
FROM rust:1.92.0-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Copy everything
COPY . .

# Build the application
RUN cargo build --release -p gitdot-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install git, ca-certificates, and curl for downloading difftastic
RUN apt-get update && \
  apt-get install -y git ca-certificates curl && \
  rm -rf /var/lib/apt/lists/*

# Install difftastic
RUN curl -L https://github.com/Wilfred/difftastic/releases/download/0.67.0/difft-x86_64-unknown-linux-gnu.tar.gz -o /tmp/difft.tar.gz && \
  tar -xzf /tmp/difft.tar.gz -C /tmp && \
  mv /tmp/difft /usr/local/bin/difft && \
  chmod +x /usr/local/bin/difft && \
  rm /tmp/difft.tar.gz

# Configure git globally
RUN git config --system user.name "Git Server" && \
  git config --system user.email "git@gitdot.io" && \
  git config --system http.receivepack true && \
  git config --system uploadpack.allowAnySHA1InWant true

# Copy the binary from builder
COPY --from=builder /app/target/release/gitdot-server /app/gitdot-server

# Set environment variables with defaults for Cloud Run
ENV GIT_PROJECT_ROOT="/srv/git"
ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT="8080"

# Expose port 8080 (Cloud Run default)
EXPOSE 8080

# Run the binary
CMD ["/app/gitdot-server"]
