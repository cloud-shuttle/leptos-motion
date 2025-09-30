#!/bin/bash

# Script to build and run WASM contract tests
set -e

echo "🧪 Building WASM Contract Tests..."

# Build the WASM package
cd tests/wasm
wasm-pack build --target web --out-dir pkg

# Copy the HTML file to the pkg directory
cp index.html pkg/

echo "✅ WASM build completed!"
echo ""
echo "🚀 To run the tests:"
echo "1. Start a local web server in the tests/wasm/pkg directory"
echo "2. Open http://localhost:8000/index.html in your browser"
echo "3. The contract tests will run automatically"
echo ""
echo "Example using Python:"
echo "cd tests/wasm/pkg && python3 -m http.server 8000"
echo ""
echo "Example using Node.js (if you have http-server):"
echo "cd tests/wasm/pkg && npx http-server -p 8000"
