# Quick Start Guide - Leptos Motion Examples

## 🚀 **Fastest Way to See Examples in Action**

### **Option 1: Try the Main Showcase (Recommended)**
```bash
cd examples/comprehensive-showcase
trunk serve
# Open http://localhost:8080
```

### **Option 2: Try CSS-Based Animations (Guaranteed to Work)**
```bash
cd examples/phase2-minimal-demo
trunk serve
# Open http://localhost:8080
```

### **Option 3: Try Simple Motion Library**
```bash
cd examples/simple-animation-demo
trunk serve
# Open http://localhost:8080
```

## 📋 **All 12 Examples (Quick Reference)**

| Example | Purpose | Status | Run Command |
|---------|---------|--------|-------------|
| `comprehensive-showcase` | Full library showcase | ✅ Built | `trunk serve` |
| `basic-leptos-demo` | Pure Leptos baseline | ✅ Built | `trunk serve` |
| `phase2-minimal-demo` | CSS animations | ✅ Working | `trunk serve` |
| `simple-animation-demo` | Simple motion | ✅ Built | `trunk serve` |
| `e-commerce-gallery` | Real-world use case | ✅ Built | `trunk serve` |
| `scroll-progress-demo` | Scroll animations | ✅ Built | `trunk serve` |
| `sidebar-menu-demo` | UI components | ✅ Built | `trunk serve` |
| `path-morphing-demo` | Advanced features | ✅ Built | `trunk serve` |
| `advanced-gestures` | Gesture handling | ✅ Compiles | `trunk serve` |
| `layout-animations` | Layout animations | ✅ Compiles | `trunk serve` |
| `puzzle-game-demo` | Interactive game | ✅ Built | `trunk serve` |
| `wasm-performance-demo` | Performance testing | ✅ Compiles | `trunk serve` |

## 🎯 **What Each Example Demonstrates**

- **`comprehensive-showcase`** → All motion library features
- **`basic-leptos-demo`** → Pure Leptos (no motion)
- **`phase2-minimal-demo`** → CSS-based animations
- **`simple-animation-demo`** → Basic motion library
- **`e-commerce-gallery`** → Real-world product gallery
- **`scroll-progress-demo`** → Scroll-triggered animations
- **`sidebar-menu-demo`** → UI component animations
- **`path-morphing-demo`** → SVG path morphing
- **`advanced-gestures`** → Touch/drag gestures
- **`layout-animations`** → Layout change animations
- **`puzzle-game-demo`** → Interactive game animations
- **`wasm-performance-demo`** → Performance benchmarks

## 🔧 **Troubleshooting**

### **If `trunk serve` fails:**
```bash
# Install trunk if needed
cargo install trunk

# Check compilation first
cargo check

# Try building manually
trunk build
```

### **If examples don't load:**
1. Check browser console for errors
2. Try a different browser
3. Clear browser cache
4. Check if port 8080 is available

### **If compilation fails:**
```bash
# Check workspace compilation
cd /path/to/leptos-motion
cargo check --workspace

# Check specific example
cd examples/example-name
cargo check
```

## 📊 **Example Status Summary**

- **✅ 12/12 examples compile successfully**
- **✅ 8/12 examples have built WASM files**
- **✅ 4/12 examples compile but need building**
- **✅ 0 broken or incomplete examples**

## 🎨 **Animation Types Available**

1. **CSS Transitions** (phase2-minimal-demo)
2. **Motion Library** (comprehensive-showcase)
3. **Scroll Animations** (scroll-progress-demo)
4. **Gesture Animations** (advanced-gestures)
5. **Layout Animations** (layout-animations)
6. **Path Morphing** (path-morphing-demo)
7. **UI Components** (sidebar-menu-demo)
8. **Game Animations** (puzzle-game-demo)
9. **Performance** (wasm-performance-demo)

## 🚀 **Next Steps**

1. **Try the examples** → Start with comprehensive-showcase
2. **Read the code** → Understand how animations work
3. **Modify examples** → Experiment with different values
4. **Create your own** → Build on the patterns you see
5. **Check documentation** → See full README.md for details

---

**Happy animating! 🎉**
