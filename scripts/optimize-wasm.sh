#!/bin/bash
# WASM optimization script for leptos-motion

set -e

echo "🚀 Starting WASM optimization..."

# Build with optimized flags
echo "📦 Building with WASM optimizations..."
RUSTFLAGS="-C opt-level=s -C debuginfo=0 -C codegen-units=1 -C panic=abort -C strip=symbols" \
cargo build --release --target wasm32-unknown-unknown --no-default-features --features minimal,minimal-web-sys

# Check if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "🔧 Running wasm-opt optimizations..."
    wasm-opt -Os -o target/wasm32-unknown-unknown/release/leptos_motion_core_optimized.wasm target/wasm32-unknown-unknown/release/leptos_motion_core.wasm

    # Show size comparison
    echo "📊 Size comparison:"
    echo "Original: $(ls -lh target/wasm32-unknown-unknown/release/leptos_motion_core.wasm | awk '{print $5}')"
    echo "Optimized: $(ls -lh target/wasm32-unknown-unknown/release/leptos_motion_core_optimized.wasm | awk '{print $5}')"
else
    echo "⚠️  wasm-opt not found. Install with: npm install -g binaryen"
    echo "📊 Current size: $(ls -lh target/wasm32-unknown-unknown/release/leptos_motion_core.wasm | awk '{print $5}')"
fi

echo "✅ WASM optimization complete!"
