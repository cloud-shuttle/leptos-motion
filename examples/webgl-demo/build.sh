#!/bin/bash

# WebGL Advanced Features Demo Build Script
# This script builds the WebGL demo showcasing Phase 3 features

set -e

echo "🚀 Building WebGL Advanced Features Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it first:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WebGL demo
echo "📦 Building WebGL demo with wasm-pack..."
cd "$(dirname "$0")"

# Build for web target
wasm-pack build --target web --out-dir pkg --dev

echo "✅ WebGL demo built successfully!"
echo ""
echo "🎮 To run the demo:"
echo "   1. Start a local server in the examples/webgl-demo directory"
echo "   2. Open http://localhost:8000 in your browser"
echo ""
echo "   Example with Python:"
echo "   cd examples/webgl-demo && python -m http.server 8000"
echo ""
echo "   Example with Node.js:"
echo "   cd examples/webgl-demo && npx serve ."
echo ""
echo "🎯 Demo Features:"
echo "   • Post-processing effects (bloom, SSAO, tone mapping)"
echo "   • Shadow mapping with directional and point lights"
echo "   • Physics simulation with collision detection"
echo "   • Interactive real-time controls"
echo "   • Performance monitoring"
