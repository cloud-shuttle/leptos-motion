#!/bin/bash
# Run pre-commit hooks manually
# Usage: ./scripts/run-pre-commit.sh [hook-name]

set -e

echo "🔍 Running pre-commit hooks..."

if [ $# -eq 0 ]; then
    # Run all hooks
    echo "Running all pre-commit hooks..."
    pre-commit run --all-files
else
    # Run specific hook
    echo "Running pre-commit hook: $1"
    pre-commit run "$1" --all-files
fi

echo "✅ Pre-commit hooks completed!"
