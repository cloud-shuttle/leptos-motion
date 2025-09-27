#!/bin/bash

# Leptos Motion - Serve Rust/WASM Demos Only
# This script serves only the real Rust/WASM examples

echo "🚀 Starting Leptos Motion Rust/WASM Demos..."

# Kill any existing servers
echo "🔄 Stopping existing servers..."
pkill -f "python3 -m http.server 8080" 2>/dev/null || true
pkill -f "python3 -m http.server 8081" 2>/dev/null || true
pkill -f "python3 -m http.server 8082" 2>/dev/null || true
pkill -f "python3 -m http.server 8083" 2>/dev/null || true
pkill -f "python3 -m http.server 8084" 2>/dev/null || true
pkill -f "python3 -m http.server 8085" 2>/dev/null || true
pkill -f "python3 -m http.server 8086" 2>/dev/null || true
pkill -f "python3 -m http.server 8087" 2>/dev/null || true
pkill -f "python3 -m http.server 8088" 2>/dev/null || true

sleep 2

echo "🌐 Starting Rust/WASM demo servers..."

# Comprehensive Showcase (port 8080) - THE REAL LEPTOS-MOTION CRATE
echo "🎨 Starting Comprehensive Showcase on port 8080..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/comprehensive-showcase
python3 -m http.server 8080 > /dev/null 2>&1 &

# Basic Leptos Demo (port 8082)
echo "🔧 Starting Basic Leptos Demo on port 8082..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/basic-leptos-demo
python3 -m http.server 8082 > /dev/null 2>&1 &

# Simple Animation Demo (port 8083)
echo "⚡ Starting Simple Animation Demo on port 8083..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/simple-animation-demo
python3 -m http.server 8083 > /dev/null 2>&1 &

# Path Morphing Demo (port 8084)
echo "🔄 Starting Path Morphing Demo on port 8084..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/path-morphing-demo
python3 -m http.server 8084 > /dev/null 2>&1 &

# Puzzle Game Demo (port 8085)
echo "🧩 Starting Puzzle Game Demo on port 8085..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/puzzle-game-demo/dist
python3 -m http.server 8085 > /dev/null 2>&1 &

# Scroll Progress Demo (port 8086)
echo "📜 Starting Scroll Progress Demo on port 8086..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/scroll-progress-demo
python3 -m http.server 8086 > /dev/null 2>&1 &

# Sidebar Menu Demo (port 8087)
echo "📱 Starting Sidebar Menu Demo on port 8087..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/sidebar-menu-demo
python3 -m http.server 8087 > /dev/null 2>&1 &

# E-commerce Gallery (port 8088)
echo "🛒 Starting E-commerce Gallery on port 8088..."
cd /Users/peterhanssens/consulting/Leptos/leptos-motion/examples/e-commerce-gallery
python3 -m http.server 8088 > /dev/null 2>&1 &

sleep 3

echo ""
echo "🎉 All Rust/WASM demos are now running!"
echo ""
echo "🎨 LEPTOS-MOTION CRATE DEMO:"
echo "  • Comprehensive Showcase: http://localhost:8080/"
echo "    (This is the REAL leptos-motion crate - may need logger fix)"
echo ""
echo "🔧 LEPTOS FRAMEWORK DEMOS:"
echo "  • Basic Leptos Demo:      http://localhost:8082/"
echo "  • Simple Animation Demo:  http://localhost:8083/"
echo "  • Path Morphing Demo:     http://localhost:8084/"
echo "  • Puzzle Game Demo:       http://localhost:8085/"
echo "  • Scroll Progress Demo:   http://localhost:8086/"
echo "  • Sidebar Menu Demo:      http://localhost:8087/"
echo "  • E-commerce Gallery:     http://localhost:8088/"
echo ""
echo "🎯 FOCUS: Only real Rust/WASM examples are running!"
echo "✅ No CSS/JavaScript fallbacks - pure Rust demos only!"
echo ""
echo "To stop all servers, run: pkill -f 'python3 -m http.server'"
