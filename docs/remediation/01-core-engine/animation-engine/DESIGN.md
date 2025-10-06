# Animation Engine Design Document

## Overview

The Animation Engine is the core component responsible for managing and executing all animations in Leptos Motion. It provides a unified interface for different animation backends while ensuring consistent timing, performance, and developer experience.

## Architecture

### Core Components

```
AnimationEngine
├── AnimationController     # Main animation orchestration
├── AnimationTarget         # Individual animation state
├── TimingController        # Animation timing and scheduling
├── ValueInterpolation      # Property value interpolation
└── BackendSelector         # WAAPI vs RAF selection
```

### Animation Backends

#### Web Animations API (WAAPI)
- **Primary Backend**: Used for CSS properties and transforms
- **Advantages**: Hardware acceleration, browser optimization
- **Limitations**: Limited to CSS properties, no custom properties

#### RequestAnimationFrame (RAF)
- **Fallback Backend**: Used for non-CSS properties
- **Advantages**: Full control over animation loop, custom properties
- **Limitations**: Manual interpolation, potential performance overhead

## API Design

### Animation Creation

```rust
// Create animation target
let target = AnimationTarget::new(
    property: "opacity",
    from_value: AnimationValue::Number(0.0),
    to_value: AnimationValue::Number(1.0),
    timing: TimingConfig {
        duration: Some(0.3),
        easing: Some(Easing::EaseOut),
        delay: None,
    }
);

// Execute animation
let handle = animation_engine.animate(target).await?;
```

### Animation Control

```rust
// Animation handle provides control interface
handle.play();
handle.pause();
handle.stop();
handle.seek(0.5); // Seek to 50% completion

// Get animation state
let state = handle.state(); // Running, Paused, Completed, Cancelled
let progress = handle.progress(); // 0.0 to 1.0
```

### Reactive Integration

```rust
// Integrate with Leptos signals
let (progress, set_progress) = signal(0.0);

let handle = animation_engine.animate(target).await?;

// Update signal on animation progress
handle.on_progress(move |p| set_progress.set(p));

// React to signal changes
create_effect(move |_| {
    let p = progress.get();
    // Update UI based on animation progress
});
```

## Animation Types

### 1. Property Animations
- **Target**: Individual CSS properties or custom properties
- **Backend**: WAAPI (CSS) or RAF (custom)
- **Examples**: `opacity`, `transform`, `color`

### 2. Transform Animations
- **Target**: CSS transform properties
- **Backend**: WAAPI with hardware acceleration
- **Examples**: `translate`, `rotate`, `scale`, `skew`

### 3. Path Drawing Animations
- **Target**: SVG `stroke-dashoffset` property
- **Backend**: WAAPI with automatic path length calculation
- **Examples**: SVG path drawing effects

### 4. Layout Animations (Future)
- **Target**: Element dimensions and positioning
- **Backend**: RAF with layout observation
- **Examples**: `width`, `height`, `position` changes

## Timing System

### Timing Configuration

```rust
#[derive(Clone, Debug)]
pub struct TimingConfig {
    pub duration: Option<f64>,      // Animation duration in seconds
    pub delay: Option<f64>,         // Start delay in seconds
    pub easing: Option<Easing>,     // Easing function
    pub repeat: Option<Repeat>,     // Repeat configuration
    pub direction: Option<Direction>, // Animation direction
}
```

### Repeat Configuration

```rust
#[derive(Clone, Debug)]
pub enum Repeat {
    Count(u32),                    // Repeat N times
    Infinite,                      // Repeat forever
}

#[derive(Clone, Debug)]
pub enum Direction {
    Normal,                        // Forward only
    Reverse,                       // Backward only
    Alternate,                     // Forward then backward
    AlternateReverse,              // Backward then forward
}
```

## Value System

### Animation Values

```rust
#[derive(Clone, Debug)]
pub enum AnimationValue {
    Number(f64),                   // opacity: 0.5
    Pixels(f64),                   // width: 100px
    Degrees(f64),                  // rotate: 45deg
    Color(Color),                  // color: #ff0000
    Transform(Transform),          // transform: translate(10px, 20px)
    Custom(String),                // custom properties
}
```

### Interpolation System

```rust
trait Interpolate {
    fn interpolate(&self, other: &Self, progress: f64) -> Self;
}

// Implement for all animation value types
impl Interpolate for AnimationValue {
    fn interpolate(&self, other: &Self, progress: f64) -> Self {
        match (self, other) {
            (Number(a), Number(b)) => Number(a + (b - a) * progress),
            (Pixels(a), Pixels(b)) => Pixels(a + (b - a) * progress),
            // ... other interpolations
        }
    }
}
```

## Backend Selection Logic

### Automatic Backend Selection

```rust
fn select_backend(property: &str, value: &AnimationValue) -> AnimationBackend {
    match (property, value) {
        // CSS properties with WAAPI support
        ("opacity", Number(_)) => AnimationBackend::WAAPI,
        ("transform", Transform(_)) => AnimationBackend::WAAPI,

        // CSS properties requiring RAF
        ("width", Pixels(_)) => AnimationBackend::RAF,
        ("height", Pixels(_)) => AnimationBackend::RAF,

        // Custom properties
        (custom, _) if custom.starts_with("--") => AnimationBackend::RAF,

        // Default to WAAPI for known CSS properties
        _ if is_css_property(property) => AnimationBackend::WAAPI,

        // Fallback to RAF
        _ => AnimationBackend::RAF,
    }
}
```

## Performance Optimizations

### 1. Animation Pooling
- Reuse animation objects to reduce GC pressure
- Pool allocation for common animation patterns

### 2. RAF Optimization
- Single RAF loop for all RAF-based animations
- Batched updates to minimize DOM interactions

### 3. WAAPI Optimization
- Prefer WAAPI for hardware-accelerated properties
- Fallback to RAF only when necessary

### 4. Memory Management
- Automatic cleanup of completed animations
- Weak references to prevent memory leaks
- Efficient value interpolation caching

## Error Handling

### Animation Errors

```rust
#[derive(Debug, Clone)]
pub enum AnimationError {
    InvalidProperty(String),
    UnsupportedValue(String),
    BackendFailure(String),
    TimingError(String),
    InterpolationError(String),
}
```

### Error Recovery

```rust
// Graceful fallback on WAAPI failure
match waapi_animation {
    Ok(handle) => Ok(handle),
    Err(_) => {
        log::warn!("WAAPI failed, falling back to RAF");
        raf_animation.fallback()
    }
}
```

## Testing Strategy

### Unit Tests
- Individual component functionality
- Value interpolation accuracy
- Timing calculation correctness
- Error handling scenarios

### Integration Tests
- End-to-end animation execution
- Backend selection logic
- Cross-browser compatibility
- Performance benchmarks

### Performance Tests
- Animation frame rate consistency
- Memory usage monitoring
- CPU usage profiling
- Bundle size impact

## Browser Compatibility

### Supported Browsers
- **Chrome**: Full WAAPI + RAF support
- **Firefox**: Full WAAPI + RAF support
- **Safari**: Limited WAAPI, full RAF support
- **Edge**: Full WAAPI + RAF support

### Fallback Strategy
1. Attempt WAAPI first for supported properties
2. Fallback to RAF for unsupported scenarios
3. Graceful degradation for older browsers

## Future Extensions

### Planned Features
- **Keyframe Animations**: Multi-step animation sequences
- **Timeline Support**: Complex animation orchestration
- **Spring Physics**: Advanced easing with physics
- **Morphing**: Shape interpolation and morphing

### Performance Enhancements
- **WebGL Acceleration**: GPU-accelerated animations
- **Worker Threading**: Off-main-thread animation calculation
- **Animation Caching**: Reuse compiled animation sequences

## Dependencies

### External Dependencies
- `web-sys`: Web API bindings for WAAPI and RAF
- `wasm-bindgen`: JavaScript interop for browser APIs
- `js-sys`: JavaScript primitive type bindings

### Internal Dependencies
- `easing`: Easing function library
- `timing`: Animation timing utilities
- `interpolation`: Value interpolation system
- `memory`: Animation memory management

---

*This design document provides the technical foundation for the Animation Engine. Implementation details may evolve based on performance requirements and browser API changes.*
