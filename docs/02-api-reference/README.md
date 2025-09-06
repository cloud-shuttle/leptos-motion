# API Reference 📚

Complete API documentation for Leptos Motion.

## 📖 What's in This Section

### 🔧 Core API
- **[API Reference](./API_REFERENCE.md)** - Complete API documentation
- **[API Stability Analysis](./API_STABILITY_ANALYSIS.md)** - API stability and versioning
- **[Changelog](./changelog.md)** - Version history and changes

## 🎯 Quick Reference

### Core Components
```rust
// MotionDiv - Animated div element
<MotionDiv initial={{ opacity: 0 }} animate={{ opacity: 1 }} />

// MotionSpan - Animated span element  
<MotionSpan while_hover={{ scale: 1.1 }} />

// AnimatePresence - Enter/exit animations
<AnimatePresence>
  {show && <MotionDiv exit={{ opacity: 0 }} />}
</AnimatePresence>
```

### Animation Values
```rust
// Numeric values
let x = MotionNumber::new(0.0);
let y = MotionNumber::new(100.0);

// Transform values
let transform = MotionTransform::new()
    .translate_x(x)
    .translate_y(y)
    .scale(1.0);
```

### Gestures
```rust
// Drag configuration
let drag_config = DragConfig::new()
    .axis(DragAxis::X)
    .constraints(DragConstraints::new().left(-100).right(100));

// Gesture detection
let gesture_detector = GestureDetector::new();
```

## 🔍 Finding What You Need

| Looking For? | Check Here |
|-------------|------------|
| **Component props** | [API Reference](./API_REFERENCE.md) |
| **Animation values** | [API Reference](./API_REFERENCE.md) |
| **Gesture APIs** | [API Reference](./API_REFERENCE.md) |
| **Version changes** | [Changelog](./changelog.md) |
| **API stability** | [API Stability Analysis](./API_STABILITY_ANALYSIS.md) |

## 🎯 Next Steps

- Explore [Guides](../03-guides/) for in-depth tutorials
- Check [Examples](../04-examples/) for working code
- Read [Architecture](../07-architecture/) for technical details

---

*Need help with a specific API? Check the complete reference above! 🔧*