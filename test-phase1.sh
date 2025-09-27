#!/bin/bash

echo "🧪 Testing Phase 1 Fixes"
echo "========================"

echo ""
echo "📦 Testing SSR Demo..."
cd demos/ssr-demo
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ SSR Demo still has errors"
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
echo "🎯 Phase 1 Test Complete!"
cd ../..
