#!/bin/bash

echo "🧪 Testing Phase 1 Fixes (Updated)"
echo "=================================="

echo ""
echo "📦 Testing SSR Demo..."
cd demos/ssr-demo
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ SSR Demo still has errors"
    echo "Running cargo check to see details:"
    cargo check
else
    echo "✅ SSR Demo compiles successfully"
fi

echo ""
echo "📦 Testing CSR Demo..."
cd ../csr-demo
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ CSR Demo still has errors"
    cargo check
else
    echo "✅ CSR Demo compiles successfully"
fi

echo ""
echo "📦 Testing Native Demo..."
cd ../native-test
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ Native Demo still has errors"
    cargo check
else
    echo "✅ Native Demo compiles successfully"
fi

echo ""
echo "📦 Testing Workspace..."
cd ../..
if cargo check --workspace 2>&1 | grep -q "error:"; then
    echo "❌ Workspace has errors"
    echo "Running cargo check --workspace to see details:"
    cargo check --workspace
else
    echo "✅ Workspace compiles successfully"
fi

echo ""
echo "🎯 Phase 1 Test Complete!"
echo "========================="
echo "If all demos compile successfully, we can move to Phase 2!"
echo "If there are still errors, we need to fix them first."
