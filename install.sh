#!/usr/bin/env bash

echo "Building project."
cargo build --release

echo "Copying executable."
sudo install target/release/rgb-controller /usr/local/bin/

