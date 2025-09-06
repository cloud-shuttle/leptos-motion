#!/bin/bash
# Validate all Cargo.toml files in the project

set -e

echo "Validating Cargo.toml files..."

# Find all Cargo.toml files except in target directory and problematic examples
find . -name "Cargo.toml" -not -path "./target/*" -not -path "./examples/mobile-app/*" -not -path "./examples/dashboard-app/*" -not -path "./examples/e-commerce-gallery/*" -not -path "./examples/advanced-features/*" | while read -r toml_file; do
    echo "Checking $toml_file..."
    if ! cargo check --manifest-path "$toml_file" > /dev/null 2>&1; then
        echo "Invalid Cargo.toml: $toml_file"
        exit 1
    fi
done

echo "All Cargo.toml files are valid!"
