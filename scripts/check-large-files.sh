#!/bin/bash
# Check for large binary files that shouldn't be committed

set -e

echo "Checking for large binary files..."

# Find binary files and check if any are outside allowed directories
# Exclude build directories and package directories
if find . -name "*.wasm" -o -name "*.so" -o -name "*.dylib" -o -name "*.dll" | \
   grep -v target/ | \
   grep -v node_modules/ | \
   grep -v pkg/ | \
   grep -v dist/ | \
   grep -v examples/.*/pkg/ | \
   grep -v examples/.*/dist/ | \
   grep -q .; then
    echo "Found binary files that should not be committed:"
    find . -name "*.wasm" -o -name "*.so" -o -name "*.dylib" -o -name "*.dll" | \
    grep -v target/ | \
    grep -v node_modules/ | \
    grep -v pkg/ | \
    grep -v dist/ | \
    grep -v examples/.*/pkg/ | \
    grep -v examples/.*/dist/
    exit 1
fi

echo "No problematic binary files found."
