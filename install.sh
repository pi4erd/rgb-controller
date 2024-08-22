#!/usr/bin/env bash
set -e

echo "Running tests..."
cargo test

echo "Building project."
cargo build --release

echo "Copying executable."
sudo install target/release/rgb-controller /usr/local/bin/

