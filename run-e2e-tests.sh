#!/bin/bash

# Leptos Motion E2E Testing Script
echo "🚀 Starting Leptos Motion E2E Tests..."

# Install Playwright if not already installed
if ! command -v playwright &> /dev/null; then
    echo "📦 Installing Playwright..."
    npm install
    npx playwright install
fi

# Kill any existing servers
echo "🧹 Cleaning up existing servers..."
pkill -f "trunk serve" || true
pkill -f "python3 -m http.server" || true

# Start the Leptos demo with Trunk
echo "🦀 Starting Leptos demo with Trunk..."
cd examples/comprehensive-showcase
trunk serve --port 9000 &
TRUNK_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 10

# Check if server is running
if ! curl -s http://localhost:9000/ > /dev/null; then
    echo "❌ Server failed to start"
    kill $TRUNK_PID 2>/dev/null || true
    exit 1
fi

echo "✅ Server is running on http://localhost:9000/"

# Run E2E tests
echo "🧪 Running E2E tests..."
cd ../..
npx playwright test

# Capture test results
TEST_EXIT_CODE=$?

# Cleanup
echo "🧹 Cleaning up..."
kill $TRUNK_PID 2>/dev/null || true

# Report results
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo "✅ All E2E tests passed!"
else
    echo "❌ Some E2E tests failed"
fi

exit $TEST_EXIT_CODE
