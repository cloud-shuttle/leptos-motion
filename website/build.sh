#!/bin/bash

# Build script for Leptos Motion website
set -e

echo "🚀 Building Leptos Motion website..."

# Create dist directory if it doesn't exist
mkdir -p dist

# Copy the static HTML file
cp index.html dist/

echo "✅ Website built successfully!"
echo "📁 Output: dist/index.html"
echo ""
echo "To serve the website locally:"
echo "  cd dist && python3 -m http.server 8000"
echo "  # or"
echo "  cd dist && npx serve ."
