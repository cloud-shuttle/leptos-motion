#!/bin/bash

echo "🧪 Testing MotionDiv Demos"
echo "========================="

echo ""
echo "📦 Testing Native Demo..."
cd demos/native-test
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ Native Demo compilation failed"
    cargo check
else
    echo "✅ Native Demo compiles successfully"
fi

echo ""
echo "📦 Testing CSR Demo..."
cd ../csr-demo
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ CSR Demo compilation failed"
    cargo check
else
    echo "✅ CSR Demo compiles successfully"
fi

echo ""
echo "📦 Testing SSR Demo..."
cd ../ssr-demo
if cargo check 2>&1 | grep -q "error:"; then
    echo "❌ SSR Demo compilation failed"
    cargo check
else
    echo "✅ SSR Demo compiles successfully"
fi

echo ""
echo "🎯 Summary:"
echo "==========="
echo "Run 'chmod +x test-motion-div.sh && ./test-motion-div.sh' to test all demos"
echo ""
echo "To run demos:"
echo "- Native: cd demos/native-test && trunk serve"
echo "- CSR: cd demos/csr-demo && trunk serve"  
echo "- SSR: cd demos/ssr-demo && cargo run"

cd ../..
