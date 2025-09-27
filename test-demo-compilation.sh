#!/bin/bash

echo "🧪 Testing MotionDiv Demo Compilation"
echo "====================================="

echo ""
echo "📦 Testing CSR Demo..."
cd demos/csr-demo
if cargo check --quiet 2>/dev/null; then
    echo "✅ CSR Demo compiles successfully"
else
    echo "❌ CSR Demo compilation failed"
    echo "Running cargo check to see errors:"
    cargo check
fi

echo ""
echo "📦 Testing SSR Demo..."
cd ../ssr-demo
if cargo check --quiet 2>/dev/null; then
    echo "✅ SSR Demo compiles successfully"
else
    echo "❌ SSR Demo compilation failed"
    echo "Running cargo check to see errors:"
    cargo check
fi

echo ""
echo "🎯 Summary:"
echo "- CSR Demo: $(if cargo check --quiet 2>/dev/null; then echo "✅ Working"; else echo "❌ Failed"; fi)"
echo "- SSR Demo: $(if cargo check --quiet 2>/dev/null; then echo "✅ Working"; else echo "❌ Failed"; fi)"

cd ../..
