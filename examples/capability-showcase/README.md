# 🎬 Leptos Motion Capability Showcase

A comprehensive demonstration of all the animation capabilities available in the Leptos Motion library.

## 🚀 Features Demonstrated

### 🎨 Basic Animations
- **Scale & Fade**: Smooth opacity and scale transitions
- **Rotate & Slide**: Rotation and translation animations
- **Color & Border**: Background color and border radius changes

### 🖱️ Gesture Interactions
- **Hover Effects**: Scale, rotate, and shadow changes on hover
- **Tap Animations**: Scale feedback on tap/click
- **Interactive Counters**: Real-time interaction tracking

### 📱 Layout Animations
- **FLIP Technique**: Smooth layout transitions when items are added/removed
- **Dynamic Lists**: Animated item management
- **Shuffle Effects**: Smooth reordering animations

### 🎬 Keyframe Animations
- **Multi-step Animations**: Complex sequences with multiple properties
- **Infinite Loops**: Continuous animation cycles
- **Precise Control**: Fine-tuned timing and easing

### ⚡ Stagger Animations
- **Sequential Delays**: Elements animate with configurable delays
- **Wave Effects**: Cascading animation patterns
- **Synchronized Timing**: Coordinated multi-element animations

### 🎯 Drag Constraints
- **Free Drag**: Unconstrained dragging with momentum
- **Axis Constraints**: X-only or Y-only movement
- **Boundary Constraints**: Elastic boundaries with bounce effects

### 🚀 Performance Demo
- **Concurrent Animations**: Multiple elements animating simultaneously
- **60fps Target**: Smooth performance with many animations
- **Scalable Count**: Adjustable number of concurrent animations

### 🔧 Advanced Features
- **Spring Physics**: Natural motion with spring-like behavior
- **3D Transforms**: Complex 3D rotation and translation
- **Color Transitions**: Smooth gradient and color changes

## 🛠️ Building and Running

### Prerequisites
- Rust 1.70+
- wasm-pack
- A modern web browser

### Build Commands

```bash
# Build the showcase
cd examples/capability-showcase
wasm-pack build --target web --out-dir pkg

# Serve locally (requires a local server)
python -m http.server 8000
# or
npx serve .
```

### Development

```bash
# Watch for changes and rebuild
wasm-pack build --target web --out-dir pkg --dev --watch
```

## 📊 Performance Metrics

The showcase demonstrates:
- **60 FPS** target performance
- **100+ concurrent animations** capability
- **500+ tests** passing in the library
- **Zero breaking changes** from previous versions

## 🎯 Use Cases

This showcase is perfect for:
- **Developers** evaluating Leptos Motion capabilities
- **Designers** understanding animation possibilities
- **Product Managers** seeing real-world performance
- **Teams** planning animation strategies

## 🔗 Integration

To use these patterns in your own project:

```toml
[dependencies]
leptos-motion = "0.6.0"
```

```rust
use leptos_motion::*;

// Basic animation
<MotionDiv
    while_hover=motion_target!("scale" => AnimationValue::Number(1.1))
    transition=Transition { duration: Some(0.2), ease: Easing::EaseOut }
>
    "Hover me!"
</MotionDiv>
```

## 📚 Documentation

- [Full API Documentation](https://docs.rs/leptos-motion)
- [GitHub Repository](https://github.com/cloud-shuttle/leptos-motion)
- [Examples Directory](../)

## 🎉 Version 0.6.0

This showcase demonstrates the latest features in Leptos Motion v0.6.0:
- Complete FLIP layout animations
- Advanced keyframe animations
- Stagger animation support
- Performance optimizations
- Cross-browser compatibility
