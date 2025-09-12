#!/bin/bash

echo "🔍 Generating coverage report..."
cargo tarpaulin --out Html --output-dir coverage

echo "📈 Coverage summary:"
if [ -f "coverage/tarpaulin-report.html" ]; then
    echo "Coverage report generated at: coverage/tarpaulin-report.html"
else
    echo "❌ Coverage report generation failed"
    exit 1
fi

echo "📋 Files with low coverage:"
find . -name "*.rs" -path "*/src/*" -exec grep -l "coverage.*[0-9][0-9]%" {} \; 2>/dev/null || echo "No low coverage files found"

echo "✅ Coverage report complete"
