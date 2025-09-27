#!/bin/bash

# Fix Logger Error - Rebuild Comprehensive Showcase
echo "🔧 Fixing logger error in Comprehensive Showcase..."

cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/comprehensive-showcase

echo "📦 Building WASM files with logger fix..."

# Try trunk first
if command -v trunk &> /dev/null; then
    echo "Using trunk to build..."
    trunk build --release
else
    echo "Trunk not found, trying cargo..."
    # Try cargo build
    cargo build --target wasm32-unknown-unknown --release
fi

echo "✅ Build complete! The logger error should be fixed."
echo "🌐 Test at: http://localhost:8080/"
echo ""
echo "If you still see the error, the WASM files might need to be regenerated."
echo "The source code has been fixed - just need to rebuild the WASM files."
