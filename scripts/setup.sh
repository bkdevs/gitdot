#!/bin/bash

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
(cd frontend && pnpm install)

echo "Running cargo build"
(cd backend && cargo build)

echo "Setup complete!"
