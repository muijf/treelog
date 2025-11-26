#!/bin/bash
# Script to run the same checks as CI locally
# Usage: ./scripts/ci-check.sh [stable|beta|nightly]

set -e

RUST_VERSION="${1:-stable}"

echo "========================================="
echo "Running CI checks with Rust $RUST_VERSION"
echo "========================================="
echo ""

# Install/select toolchain
echo "[1/7] Setting up Rust toolchain..."
rustup toolchain install "$RUST_VERSION" --component rustfmt,clippy 2>/dev/null || true
rustup override set "$RUST_VERSION" 2>/dev/null || rustup default "$RUST_VERSION"
echo "Using: $(rustc --version)"
echo ""

# Format check
echo "[2/7] Checking code formatting..."
cargo fmt --all -- --check
echo "Format check passed"
echo ""

# Clippy
echo "[3/7] Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
echo "Clippy check passed"
echo ""

# Build
echo "[4/7] Building project..."
cargo build --verbose --all-features
echo "Build successful"
echo ""

# Library tests
echo "[5/7] Running library tests..."
cargo test --lib --verbose --all-features
echo "Library tests passed"
echo ""

# Doc tests
echo "[6/7] Running documentation tests..."
cargo test --doc --verbose --all-features
echo "Documentation tests passed"
echo ""

# Build examples
echo "[7/7] Building examples..."
cargo build --examples --verbose --all-features
echo "Examples built successfully"
echo ""

echo "========================================="
echo "All CI checks passed successfully!"
echo "========================================="
