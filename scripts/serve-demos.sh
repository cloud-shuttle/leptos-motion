#!/bin/bash

# Leptos Motion Demo Server Script
echo "🚀 Starting Leptos Motion Demos..."

# Kill any existing servers
echo "🧹 Cleaning up existing servers..."
pkill -f "trunk serve" || true
pkill -f "cargo run" || true
pkill -f "python3 -m http.server" || true

# Start CSR Demo
echo "🦀 Starting CSR Demo on http://localhost:9000/"
cd demos/csr-demo
trunk serve --port 9000 &
CSR_PID=$!

# Start SSR Demo
echo "🦀 Starting SSR Demo on http://localhost:9001/"
cd ../ssr-demo
cargo run --bin ssr-demo &
SSR_PID=$!

# Wait for servers to start
echo "⏳ Waiting for servers to start..."
sleep 15

# Check if servers are running
echo "🔍 Checking server status..."

# Check CSR server
if curl -s http://localhost:9000/ > /dev/null; then
    echo "✅ CSR Demo is running on http://localhost:9000/"
else
    echo "❌ CSR Demo failed to start"
    kill $CSR_PID 2>/dev/null || true
    kill $SSR_PID 2>/dev/null || true
    exit 1
fi

# Check SSR server
if curl -s http://localhost:9001/ > /dev/null; then
    echo "✅ SSR Demo is running on http://localhost:9001/"
else
    echo "❌ SSR Demo failed to start"
    kill $CSR_PID 2>/dev/null || true
    kill $SSR_PID 2>/dev/null || true
    exit 1
fi

echo "🎉 Both demos are running successfully!"
echo "📱 CSR Demo: http://localhost:9000/"
echo "🖥️  SSR Demo: http://localhost:9001/"

# Keep servers running
echo "🔄 Servers are running. Press Ctrl+C to stop."
wait
