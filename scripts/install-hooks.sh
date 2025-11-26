#!/bin/bash
# Install git hooks from scripts directory

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GIT_HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "Installing git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install pre-commit hook
ln -sf "$SCRIPT_DIR/pre-commit.sh" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"
echo "✅ Installed pre-commit hook"

# Install pre-push hook
ln -sf "$SCRIPT_DIR/pre-push.sh" "$GIT_HOOKS_DIR/pre-push"
chmod +x "$GIT_HOOKS_DIR/pre-push"
echo "✅ Installed pre-push hook"

echo "🎉 Git hooks installed successfully!"
echo ""
echo "Hooks will now run:"
echo "  - pre-commit: cargo fmt + cargo clippy"
echo "  - pre-push: cargo test"

