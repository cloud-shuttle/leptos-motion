#!/bin/bash

# Leptos Motion E2E Testing Script
echo "🧪 Starting Leptos Motion E2E Tests..."

# Install Playwright if not already installed
if ! command -v playwright &> /dev/null; then
    echo "📦 Installing Playwright..."
    npm install
    npx playwright install
fi

# Kill any existing servers
echo "🧹 Cleaning up existing servers..."
pkill -f "trunk serve" || true
pkill -f "cargo run" || true
pkill -f "python3 -m http.server" || true

# Start the demos
echo "🚀 Starting demo servers..."
./scripts/serve-demos.sh &
SERVER_PID=$!

# Wait for servers to start
echo "⏳ Waiting for servers to start..."
sleep 20

# Check if servers are running
echo "🔍 Checking server status..."

# Check CSR server
if ! curl -s http://localhost:9000/ > /dev/null; then
    echo "❌ CSR Demo server not responding"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

# Check SSR server
if ! curl -s http://localhost:9001/ > /dev/null; then
    echo "❌ SSR Demo server not responding"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

echo "✅ Both servers are running"

# Run E2E tests
echo "🧪 Running E2E tests..."
npx playwright test

# Capture test results
TEST_EXIT_CODE=$?

# Cleanup
echo "🧹 Cleaning up..."
kill $SERVER_PID 2>/dev/null || true

# Report results
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo "✅ All E2E tests passed!"
else
    echo "❌ Some E2E tests failed"
fi

exit $TEST_EXIT_CODE
