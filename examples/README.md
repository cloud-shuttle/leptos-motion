# Leptos Motion Examples

This directory contains curated examples demonstrating the capabilities of the Leptos Motion library.

## 🎯 Overview

We've curated **12 high-quality examples** from the original 35+ examples, focusing on:
- ✅ **Working examples** that compile and run successfully
- 🎨 **Diverse use cases** covering different animation types
- 📚 **Clear demonstrations** of library features
- 🚀 **Real-world applications** and practical examples

## 📋 Examples by Category

### 🟢 **Core Examples (8)**

These are the essential examples that demonstrate the core functionality:

#### 1. **`comprehensive-showcase`** ⭐
- **Purpose**: Full motion library showcase
- **Features**: Complete demonstration of all motion capabilities
- **Uses**: `leptos_motion::*` - tests the full library
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd comprehensive-showcase && trunk serve`

#### 2. **`basic-leptos-demo`** ⭐
- **Purpose**: Basic Leptos functionality (no motion)
- **Features**: Pure Leptos reactive system baseline
- **Uses**: Pure Leptos - good baseline for comparison
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd basic-leptos-demo && trunk serve`

#### 3. **`phase2-minimal-demo`** ⭐
- **Purpose**: CSS-based animations (working approach)
- **Features**: CSS classes + Leptos signals for animations
- **Uses**: CSS transitions and transforms
- **Status**: ✅ Compiles, CSS-based approach
- **Run**: `cd phase2-minimal-demo && trunk serve`

#### 4. **`simple-animation-demo`** ⭐
- **Purpose**: Simple motion library demo
- **Features**: Basic motion library usage
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd simple-animation-demo && trunk serve`

#### 5. **`e-commerce-gallery`** ⭐
- **Purpose**: Real-world e-commerce use case
- **Features**: Product gallery with motion effects
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd e-commerce-gallery && trunk serve`

#### 6. **`scroll-progress-demo`** ⭐
- **Purpose**: Scroll-based animations
- **Features**: Animations triggered by scroll position
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd scroll-progress-demo && trunk serve`

#### 7. **`sidebar-menu-demo`** ⭐
- **Purpose**: UI component animations
- **Features**: Sidebar menu with motion effects
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd sidebar-menu-demo && trunk serve`

#### 8. **`path-morphing-demo`** ⭐
- **Purpose**: Advanced path morphing animations
- **Features**: SVG path morphing and complex animations
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd path-morphing-demo && trunk serve`

### 🔵 **Feature-Specific Examples (4)**

These examples demonstrate specific features and advanced capabilities:

#### 9. **`advanced-gestures`** ⭐
- **Purpose**: Advanced gesture handling
- **Features**: Touch, drag, and gesture-based animations
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles successfully
- **Run**: `cd advanced-gestures && trunk serve`

#### 10. **`layout-animations`** ⭐
- **Purpose**: Layout-based animations
- **Features**: Animations triggered by layout changes
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles successfully
- **Run**: `cd layout-animations && trunk serve`

#### 11. **`puzzle-game-demo`** ⭐
- **Purpose**: Interactive game with animations
- **Features**: Puzzle game with motion effects
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles, has built WASM
- **Run**: `cd puzzle-game-demo && trunk serve`

#### 12. **`wasm-performance-demo`** ⭐
- **Purpose**: WASM performance testing
- **Features**: Performance benchmarks and optimization demos
- **Uses**: `leptos_motion::*`
- **Status**: ✅ Compiles successfully
- **Run**: `cd wasm-performance-demo && trunk serve`

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- Trunk (for building WASM)
- Modern web browser

### Running Examples

1. **Navigate to an example directory**:
   ```bash
   cd examples/comprehensive-showcase
   ```

2. **Install dependencies** (if needed):
   ```bash
   cargo check
   ```

3. **Build and serve**:
   ```bash
   trunk serve
   ```

4. **Open in browser**: Navigate to `http://localhost:8080`

### Building for Production

```bash
trunk build --release
```

## 🧪 Testing Examples

All examples have been tested and verified to:
- ✅ **Compile successfully** with the current library
- ✅ **Use proper dependencies** and workspace configuration
- ✅ **Demonstrate real functionality** (not just stubs)
- ✅ **Follow best practices** for Leptos and WASM

## 📊 Example Statistics

- **Total Examples**: 12 (curated from 35+)
- **Working Examples**: 12/12 (100%)
- **Built WASM Examples**: 8/12 (67%)
- **Core Examples**: 8
- **Feature Examples**: 4

## 🔧 Development Notes

### What Was Removed

We removed 23 examples that were:
- **Development phases** (10 phase1/phase2/phase3 examples)
- **Non-workspace examples** (5 examples not in Cargo.toml)
- **Redundant examples** (6 similar feature examples)
- **Broken examples** (2 incomplete examples)

### What Was Kept

We kept examples that:
- ✅ **Compile successfully** in the workspace
- ✅ **Demonstrate unique features** or use cases
- ✅ **Have clear, working code** (not stubs)
- ✅ **Represent different animation types** and approaches
- ✅ **Include real-world applications** and practical examples

## 🎨 Animation Types Covered

- **CSS-based animations** (phase2-minimal-demo)
- **Motion library animations** (comprehensive-showcase)
- **Scroll-triggered animations** (scroll-progress-demo)
- **Gesture-based animations** (advanced-gestures)
- **Layout animations** (layout-animations)
- **Path morphing** (path-morphing-demo)
- **UI component animations** (sidebar-menu-demo)
- **Game animations** (puzzle-game-demo)
- **Performance demos** (wasm-performance-demo)

## 📚 Learning Path

### For Beginners
1. Start with `basic-leptos-demo` (pure Leptos)
2. Try `phase2-minimal-demo` (CSS approach)
3. Move to `simple-animation-demo` (motion library)

### For Intermediate Users
1. Explore `comprehensive-showcase` (full features)
2. Try `e-commerce-gallery` (real-world use case)
3. Experiment with `scroll-progress-demo` (scroll animations)

### For Advanced Users
1. Study `path-morphing-demo` (advanced features)
2. Explore `advanced-gestures` (gesture handling)
3. Benchmark with `wasm-performance-demo` (performance)

## 🤝 Contributing

When adding new examples:
1. Ensure they compile in the workspace
2. Add them to the workspace `Cargo.toml`
3. Include clear documentation
4. Test with `cargo check --workspace`
5. Update this README

## 📄 License

All examples are part of the Leptos Motion project and follow the same license terms.
