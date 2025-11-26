#!/bin/bash
# Pre-push hook for Rust project
# This runs tests before allowing pushes

set -e

echo "Running pre-push checks..."

echo "Running tests..."
cargo test --all-features || {
    echo "ERROR: Tests failed. Fix the failing tests before pushing."
    exit 1
}

echo "Pre-push checks passed!"
exit 0
