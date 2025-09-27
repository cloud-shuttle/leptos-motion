#!/bin/bash

# Comprehensive fix script for MotionDiv issues
# This script fixes the most common issues across all demo files

echo "🔧 Fixing MotionDiv issues across all demo files..."

# Find all Rust files in the comprehensive demo
find demos/showcase/comprehensive-demo/src -name "*.rs" -type f | while read file; do
    echo "Processing: $file"
    
    # Fix 1: Add node_ref to ReactiveMotionDiv and MotionDiv
    # Pattern: <ReactiveMotionDiv or <MotionDiv without node_ref
    sed -i '' 's/<ReactiveMotionDiv$/<ReactiveMotionDiv\
                        node_ref=NodeRef::new()/g' "$file"
    
    sed -i '' 's/<MotionDiv$/<MotionDiv\
                        node_ref=NodeRef::new()/g' "$file"
    
    # Fix 2: Fix animate_fn -> animate
    sed -i '' 's/animate_fn=/animate=/g' "$file"
    
    # Fix 3: Fix transition= -> _transition=
    sed -i '' 's/transition=/_transition=/g' "$file"
    
    # Fix 4: Fix closure calling - add () to Box::new closures
    # Pattern: animate=Box::new(move || something) -> animate=Box::new(move || something)()
    sed -i '' 's/animate=Box::new(move || \([^)]*\))/animate=Box::new(move || \1)()/g' "$file"
    
    echo "✅ Fixed: $file"
done

echo "🎯 All fixes applied!"
echo "Run 'cargo check --package comprehensive-demo' to verify fixes"
