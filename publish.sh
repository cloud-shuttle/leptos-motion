#!/bin/bash

# Publish crates in dependency order
echo "Publishing Leptos Motion v0.9.0 to crates.io..."

# 1. Publish core first (no internal dependencies)
echo "Publishing leptos-motion-core..."
cd crates/leptos-motion-core
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-core"
    exit 1
fi
cd ../..

# 2. Publish macros (depends on core)
echo "Publishing leptos-motion-macros..."
cd crates/leptos-motion-macros
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-macros"
    exit 1
fi
cd ../..

# 3. Publish gestures (depends on core)
echo "Publishing leptos-motion-gestures..."
cd crates/leptos-motion-gestures
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-gestures"
    exit 1
fi
cd ../..

# 4. Publish layout (depends on core)
echo "Publishing leptos-motion-layout..."
cd crates/leptos-motion-layout
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-layout"
    exit 1
fi
cd ../..

# 5. Publish scroll (depends on core)
echo "Publishing leptos-motion-scroll..."
cd crates/leptos-motion-scroll
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-scroll"
    exit 1
fi
cd ../..

# 6. Publish dom (depends on core, gestures, layout)
echo "Publishing leptos-motion-dom..."
cd crates/leptos-motion-dom
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-dom"
    exit 1
fi
cd ../..

# 7. Publish webgl (depends on core)
echo "Publishing leptos-motion-webgl..."
cd crates/leptos-motion-webgl
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-webgl"
    exit 1
fi
cd ../..

# 8. Publish studio (depends on core, dom)
echo "Publishing leptos-motion-studio..."
cd crates/leptos-motion-studio
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion-studio"
    exit 1
fi
cd ../..

# 9. Publish main crate (depends on all others)
echo "Publishing leptos-motion..."
cd crates/leptos-motion
cargo publish
if [ $? -ne 0 ]; then
    echo "Failed to publish leptos-motion"
    exit 1
fi
cd ../..

echo "✅ Successfully published all Leptos Motion v0.9.0 crates to crates.io!"
