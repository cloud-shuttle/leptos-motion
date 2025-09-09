#!/bin/bash

# Script to update all examples to use the latest local crates

echo "Updating all examples to use v0.7.0 local crates..."

# Find all Cargo.toml files in examples
find . -name "Cargo.toml" -path "*/examples/*" | while read -r file; do
    echo "Updating $file..."

    # Update leptos-motion dependencies to use local v0.7.0 versions
    sed -i '' 's|leptos-motion = { path = "../../crates/leptos-motion"|leptos-motion = { version = "0.7.0", path = "../../crates/leptos-motion"|g' "$file"
    sed -i '' 's|leptos-motion-dom = { path = "../../crates/leptos-motion-dom"|leptos-motion-dom = { version = "0.7.0", path = "../../crates/leptos-motion-dom"|g' "$file"
    sed -i '' 's|leptos-motion-gestures = { path = "../../crates/leptos-motion-gestures"|leptos-motion-gestures = { version = "0.7.0", path = "../../crates/leptos-motion-gestures"|g' "$file"
    sed -i '' 's|leptos-motion-layout = { path = "../../crates/leptos-motion-layout"|leptos-motion-layout = { version = "0.7.0", path = "../../crates/leptos-motion-layout"|g' "$file"
    sed -i '' 's|leptos-motion-scroll = { path = "../../crates/leptos-motion-scroll"|leptos-motion-scroll = { version = "0.7.0", path = "../../crates/leptos-motion-scroll"|g' "$file"
    sed -i '' 's|leptos-motion-macros = { path = "../../crates/leptos-motion-macros"|leptos-motion-macros = { version = "0.7.0", path = "../../crates/leptos-motion-macros"|g' "$file"
    sed -i '' 's|leptos-motion-core = { path = "../../crates/leptos-motion-core"|leptos-motion-core = { version = "0.7.0", path = "../../crates/leptos-motion-core"|g' "$file"

    # Add leptos-motion-dom dependency if it doesn't exist and leptos-motion exists
    if grep -q "leptos-motion = {" "$file" && ! grep -q "leptos-motion-dom = {" "$file"; then
        echo "Adding leptos-motion-dom dependency to $file"
        sed -i '' '/leptos-motion = {/a\
leptos-motion-dom = { version = "0.7.0", path = "../../crates/leptos-motion-dom" }
' "$file"
    fi
done

echo "All examples updated!"
