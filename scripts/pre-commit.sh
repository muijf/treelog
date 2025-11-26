#!/bin/bash
# Pre-commit hook for Rust project
# This runs cargo fmt and cargo clippy before allowing commits

set -e

echo "Running pre-commit checks..."

echo "Checking formatting..."
cargo fmt --all -- --check || {
    echo "ERROR: Formatting check failed. Run 'cargo fmt --all' to fix."
    exit 1
}

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "ERROR: Clippy check failed. Fix the warnings above."
    exit 1
}

echo "Pre-commit checks passed!"
exit 0
