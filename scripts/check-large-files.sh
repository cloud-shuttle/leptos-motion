#!/bin/bash

# Check for large files that shouldn't be committed
# This script is used by pre-commit hooks to prevent large files from being committed

set -e

# Maximum file size in KB (default: 1000KB = 1MB)
MAX_SIZE_KB=${MAX_SIZE_KB:-1000}

# Convert to bytes for comparison
MAX_SIZE_BYTES=$((MAX_SIZE_KB * 1024))

echo "🔍 Checking for files larger than ${MAX_SIZE_KB}KB..."

# Find all files in the repository that are larger than the limit
large_files=$(find . -type f -size +${MAX_SIZE_KB}k -not -path './.git/*' -not -path './target/*' -not -path './**/target/*' -not -path './node_modules/*' -not -path './dist/*' -not -path './**/dist/*' 2>/dev/null || true)

if [ -n "$large_files" ]; then
    echo "❌ Found files larger than ${MAX_SIZE_KB}KB:"
    echo ""

    # Show file sizes in a readable format
    for file in $large_files; do
        size_bytes=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0")
        size_kb=$((size_bytes / 1024))
        size_mb=$((size_kb / 1024))

        if [ $size_mb -gt 0 ]; then
            echo "  📁 $file (${size_mb}MB)"
        else
            echo "  📁 $file (${size_kb}KB)"
        fi
    done

    echo ""
    echo "💡 These files should not be committed to the repository."
    echo "   Consider:"
    echo "   - Adding them to .gitignore"
    echo "   - Using Git LFS for large binary files"
    echo "   - Storing them externally and downloading during build"
    echo "   - Compressing or optimizing the files"
    echo ""
    echo "🚫 Commit blocked due to large files."
    exit 1
else
    echo "✅ No files larger than ${MAX_SIZE_KB}KB found."
fi

# Also check for common problematic file types
echo "🔍 Checking for common problematic file types..."

# Check for .wasm files (should be in target/ or built, not committed)
wasm_files=$(find . -name "*.wasm" -not -path './.git/*' -not -path './target/*' -not -path './**/target/*' 2>/dev/null || true)

if [ -n "$wasm_files" ]; then
    echo "❌ Found .wasm files that should not be committed:"
    for file in $wasm_files; do
        echo "  📁 $file"
    done
    echo "💡 .wasm files should be built during CI/CD, not committed to the repository."
    exit 1
fi

# Check for .rlib files (Rust library files)
rlib_files=$(find . -name "*.rlib" -not -path './.git/*' -not -path './target/*' -not -path './**/target/*' 2>/dev/null || true)

if [ -n "$rlib_files" ]; then
    echo "❌ Found .rlib files that should not be committed:"
    for file in $rlib_files; do
        echo "  📁 $file"
    done
    echo "💡 .rlib files are build artifacts and should not be committed."
    exit 1
fi

# Check for .rmeta files (Rust metadata files)
rmeta_files=$(find . -name "*.rmeta" -not -path './.git/*' -not -path './target/*' -not -path './**/target/*' 2>/dev/null || true)

if [ -n "$rmeta_files" ]; then
    echo "❌ Found .rmeta files that should not be committed:"
    for file in $rmeta_files; do
        echo "  📁 $file"
    done
    echo "💡 .rmeta files are build artifacts and should not be committed."
    exit 1
fi

echo "✅ No problematic file types found."
echo "🎉 All file size checks passed!"
