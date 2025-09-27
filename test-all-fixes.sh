#!/bin/bash

echo "🧪 Testing All Phase 1 Fixes"
echo "============================="

echo ""
echo "📦 Testing SSR Demo..."
cd demos/ssr-demo
echo "Running: cargo check"
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ SSR Demo still has errors"
    echo "Error details:"
    cargo check
else
    echo "✅ SSR Demo compiles successfully"
fi

echo ""
echo "📦 Testing CSR Demo..."
cd ../csr-demo
echo "Running: cargo check"
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ CSR Demo still has errors"
    cargo check
else
    echo "✅ CSR Demo compiles successfully"
fi

echo ""
echo "📦 Testing Native Demo..."
cd ../native-test
echo "Running: cargo check"
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ Native Demo still has errors"
    cargo check
else
    echo "✅ Native Demo compiles successfully"
fi

echo ""
echo "📦 Testing Workspace..."
cd ../..
echo "Running: cargo check --workspace"
if cargo check --workspace 2>&1 | grep -q "error:"; then
    echo "❌ Workspace has errors"
    echo "Error details:"
    cargo check --workspace
else
    echo "✅ Workspace compiles successfully"
fi

echo ""
echo "🎯 Phase 1 Test Complete!"
echo "========================="
echo ""
echo "📊 Summary:"
echo "- CSR Demo: $(if cargo check --package csr-demo >/dev/null 2>&1; then echo "✅ Working"; else echo "❌ Issues"; fi)"
echo "- SSR Demo: $(if cargo check --package ssr-demo >/dev/null 2>&1; then echo "✅ Working"; else echo "❌ Issues"; fi)"
echo "- Native Demo: $(if cargo check --package native-test >/dev/null 2>&1; then echo "✅ Working"; else echo "❌ Issues"; fi)"
echo ""
echo "🚀 Next Steps:"
echo "If all demos compile successfully, we can move to Phase 2!"
echo "If there are still errors, we need to fix them first."
