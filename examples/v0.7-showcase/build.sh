#!/bin/bash

# Build script for Leptos Motion v0.7.0 Showcase

echo "🚀 Building Leptos Motion v0.7.0 Showcase..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it first:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM package
echo "📦 Building WASM package..."
wasm-pack build --target web --out-dir pkg --dev

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build completed successfully!"
echo ""
echo "🌐 To serve the showcase:"
echo "   cd examples/v0.7-showcase"
echo "   python3 -m http.server 8000"
echo "   Then open http://localhost:8000"
echo ""
echo "🎬 Enjoy the Leptos Motion v0.7.0 showcase!"
