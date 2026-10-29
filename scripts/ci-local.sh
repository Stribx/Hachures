#!/usr/bin/env bash
# Runs the same contributor checks as the Linux CI quality job.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> cargo fmt --check"
cargo fmt --check

echo "==> cargo clippy"
cargo clippy --all-targets -- -D warnings

echo "==> cargo test"
cargo test -p game

echo "==> balance regression"
cargo test -p game balance -- --ignored

echo "==> WebAssembly check"
cargo check --target wasm32-unknown-unknown -p game

echo "Local CI checks passed."

