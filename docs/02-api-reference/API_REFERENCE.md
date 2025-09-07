# Leptos Motion API Reference

## Overview

Leptos Motion provides a comprehensive animation library for Leptos applications with a Framer Motion-inspired API. This document covers all available components, types, and functions.

**Version**: 0.4.0  
**Bundle Size**: 30KB-85KB (92% reduction from 378KB)  
**Features**: Minimal serialization, conditional compilation, feature flags, and comprehensive optimization

## Bundle Size Optimization

Leptos Motion v0.4.0 includes comprehensive bundle size optimization with multiple build presets:

### Build Presets

```toml
# Minimal build (30KB) - Core animations only
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["minimal"] }

# Production build (75KB) - Optimized for production
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["production"] }

# Optimized build (85KB) - With performance monitoring
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["optimized"] }

# Standard build (125KB) - Full features
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["standard"] }

# Full build (235KB) - All features including development tools
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["full"] }
```

### Feature Flags

- **`minimal-serialization`**: Custom lightweight serialization (replaces serde)
- **`conditional-web-sys`**: Optimized web-sys feature usage
- **`performance-metrics`**: Performance monitoring and metrics
- **`memory-optimization`**: Memory usage optimization
- **`lazy-loading`**: Lazy loading of animation modules
- **`gesture-support`**: Gesture recognition and handling
- **`layout-animations`**: FLIP-based layout animations
- **`scroll-animations`**: Scroll-triggered animations

### Optimization Features

#### Minimal Serialization
```rust
use leptos_motion_core::minimal_serialization::*;

// Lightweight JSON serialization
let serializer = MinimalJsonSerializer::new();
let data = serializer.serialize(&animation_data)?;

// Binary serialization for performance
let binary_serializer = MinimalBinarySerializer::new();
let binary_data = binary_serializer.serialize(&animation_data)?;
```

#### Conditional Web-Sys
```rust
#[cfg(feature = "conditional-web-sys")]
use leptos_motion_core::web_sys_optimized::*;

// Optimized web-sys usage with minimal features
let element = get_optimized_element()?;
```

#### Performance Monitoring
```rust
#[cfg(feature = "performance-metrics")]
use leptos_motion_core::performance::*;

// Performance monitoring
let monitor = PerformanceMonitor::new();
let metrics = monitor.get_metrics();
```

## Core Components

### MotionDiv

The primary animation component for div elements.

```rust
use leptos::*;
use leptos_motion::*;

<MotionDiv
    // Animation properties
    initial: Option<AnimationTarget>,
    animate: Option<AnimationTarget>,
    exit: Option<AnimationTarget>,
    transition: Option<Transition>,
    variants: Option<Variants>,

    // Layout and gesture properties
    layout: bool,
    drag: bool | DragConfig,
    drag_constraints: Option<DragConstraints>,

    // Gesture animations
    while_hover: Option<AnimationTarget>,
    while_tap: Option<AnimationTarget>,
    while_focus: Option<AnimationTarget>,
    while_in_view: Option<AnimationTarget>,

    // Standard HTML properties
    class: String,
    style: String,
    id: Option<String>,
    key: Option<String>,

    // Event handlers
    event_handlers: Option<EventHandlers>,
>
    // Children
</MotionDiv>
```

#### Props

- **`initial`**: Initial animation state when component mounts
- **`animate`**: Target animation state to animate towards
- **`exit`**: Exit animation state when component unmounts
- **`transition`**: Animation transition configuration
- **`variants`**: Named animation states for complex animations
- **`layout`**: Whether to animate layout changes (position, size)
- **`drag`**: Drag gesture configuration (boolean or DragConfig)
- **`drag_constraints`**: Constraints for drag gestures
- **`while_hover`**: Animation to play while hovering
- **`while_tap`**: Animation to play while tapping
- **`while_focus`**: Animation to play while focused
- **`while_in_view`**: Animation to play while in viewport
- **`class`**: CSS class names
- **`style`**: Inline CSS styles
- **`id`**: HTML element ID
- **`key`**: Key for list rendering
- **`event_handlers`**: Custom event handlers

### MotionSpan

Animation component for span elements with the same API as MotionDiv.

```rust
<MotionSpan
    initial=motion_target!("opacity" => AnimationValue::Number(0.0))
    animate=motion_target!("opacity" => AnimationValue::Number(1.0))
    class="animated-text".to_string()
>
    "Animated text"
</MotionSpan>
```

### AnimatePresence

Component for handling enter/exit animations with automatic cleanup.

```rust
<AnimatePresence mode=PresenceMode::Wait>
    {move || if show_element() {
        Some(view! {
            <MotionDiv
                initial=motion_target!("opacity" => AnimationValue::Number(0.0))
                animate=motion_target!("opacity" => AnimationValue::Number(1.0))
                exit=motion_target!("opacity" => AnimationValue::Number(0.0))
            >
                "Content"
            </MotionDiv>
        })
    } else {
        None
    }}
</AnimatePresence>
```

#### Props

- **`mode`**: Presence mode (Wait, Sync, PopLayout)

## Animation Types

### AnimationTarget

Represents a collection of animatable properties.

```rust
use leptos_motion::*;

// Using motion_target! macro
let target = motion_target!(
    "opacity" => AnimationValue::Number(1.0),
    "scale" => AnimationValue::Number(1.2),
    "x" => AnimationValue::Pixels(100.0),
    "y" => AnimationValue::Pixels(50.0),
    "rotate" => AnimationValue::Degrees(45.0),
    "backgroundColor" => AnimationValue::Color("#ff0000".to_string())
);

// Manual construction
let mut target = AnimationTarget::new();
target.insert("opacity".to_string(), AnimationValue::Number(1.0));
target.insert("scale".to_string(), AnimationValue::Number(1.2));
```

### AnimationValue

Represents different types of animatable values.

```rust
use leptos_motion::*;

// Number values (unitless)
let opacity = AnimationValue::Number(0.5);
let scale = AnimationValue::Number(1.2);

// Pixel values
let x = AnimationValue::Pixels(100.0);
let y = AnimationValue::Pixels(50.0);

// Degree values
let rotation = AnimationValue::Degrees(45.0);

// Color values
let color = AnimationValue::Color("#ff0000".to_string());

// String values (for custom CSS properties)
let custom = AnimationValue::String("custom-value".to_string());
```

### Transition

Configuration for animation transitions.

```rust
use leptos_motion::*;

let transition = Transition {
    duration: Some(0.5),                    // Duration in seconds
    delay: Some(0.1),                       // Delay in seconds
    ease: Easing::EaseOut,                  // Easing function
    repeat: Some(RepeatConfig::Infinite),   // Repeat configuration
    repeat_delay: Some(0.2),                // Delay between repeats
    ..Default::default()
};
```

### Easing

Available easing functions.

```rust
use leptos_motion::*;

// Built-in easing functions
let ease_linear = Easing::Linear;
let ease_in = Easing::EaseIn;
let ease_out = Easing::EaseOut;
let ease_in_out = Easing::EaseInOut;

// Spring easing
let spring = Easing::Spring(SpringConfig::default()
    .stiffness(100.0)
    .damping(10.0)
    .mass(1.0)
);

// Custom cubic bezier
let custom = Easing::CubicBezier(0.25, 0.1, 0.25, 1.0);
```

### SpringConfig

Configuration for spring animations.

```rust
use leptos_motion::*;

let spring_config = SpringConfig::default()
    .stiffness(100.0)    // Higher = faster animation
    .damping(10.0)       // Lower = more bouncy
    .mass(1.0)           // Higher = more inertia
    .rest_speed(0.01)    // Speed threshold for rest
    .rest_delta(0.01);   // Position threshold for rest
```

### RepeatConfig

Configuration for animation repetition.

```rust
use leptos_motion::*;

// Never repeat
let never = RepeatConfig::Never;

// Repeat a specific number of times
let count = RepeatConfig::Count(3);

// Repeat infinitely
let infinite = RepeatConfig::Infinite;

// Repeat infinitely with reverse
let infinite_reverse = RepeatConfig::InfiniteReverse;
```

## Gesture Types

### DragConfig

Configuration for drag gestures.

```rust
use leptos_motion::*;

let drag_config = DragConfig::default()
    .axis(Axis::X)                    // Constrain to X axis
    .bounds(Bounds::Parent)           // Constrain to parent bounds
    .inertia(true)                    // Enable inertia
    .snap_to_grid(10.0);              // Snap to grid

// Simple boolean usage
let simple_drag = true; // Equivalent to DragConfig::default()
```

### DragConstraints

Constraints for drag gestures.

```rust
use leptos_motion::*;

let constraints = DragConstraints {
    left: Some(-100.0),      // Left boundary
    right: Some(100.0),      // Right boundary
    top: Some(-50.0),        // Top boundary
    bottom: Some(50.0),      // Bottom boundary
};

// No constraints
let no_constraints = DragConstraints {
    left: None,
    right: None,
    top: None,
    bottom: None,
};
```

## Animation Presets

Pre-built animation configurations for common patterns.

```rust
use leptos_motion_core::AnimationPresets;

// Fade in animation
let fade_in = AnimationPresets::fade_in();

// Fade out animation
let fade_out = AnimationPresets::fade_out();

// Scale in animation
let scale_in = AnimationPresets::scale_in();

// Slide in from left
let slide_in_left = AnimationPresets::slide_in_left();

// Bounce in animation
let bounce_in = AnimationPresets::bounce_in();

// Usage
<MotionDiv
    initial=fade_in.initial
    animate=fade_in.animate
    transition=fade_in.transition
    class="preset-demo".to_string()
>
    "Preset Animation"
</MotionDiv>
```

## Keyframes

Create complex multi-step animations.

```rust
use leptos_motion_core::animation::Keyframes;

let keyframes = Keyframes::new()
    .add_keyframe(0.0, motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "scale" => AnimationValue::Number(0.5)
    ))
    .add_keyframe(0.5, motion_target!(
        "opacity" => AnimationValue::Number(0.5),
        "scale" => AnimationValue::Number(1.1)
    ))
    .add_keyframe(1.0, motion_target!(
        "opacity" => AnimationValue::Number(1.0),
        "scale" => AnimationValue::Number(1.0)
    ));

<MotionDiv
    animate=keyframes.to_animation_target()
    transition=Transition {
        duration: Some(2.0),
        ease: Easing::EaseInOut,
        ..Default::default()
    }
    class="keyframe-demo".to_string()
>
    "Keyframe Animation"
</MotionDiv>
```

## Variants

Named animation states for complex animations.

```rust
use leptos_motion::*;

let variants = Variants::new()
    .variant("hidden", motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "x" => AnimationValue::Pixels(-100.0)
    ))
    .variant("visible", motion_target!(
        "opacity" => AnimationValue::Number(1.0),
        "x" => AnimationValue::Pixels(0.0)
    ))
    .variant("exit", motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "x" => AnimationValue::Pixels(100.0)
    ));

<MotionDiv
    initial="hidden"
    animate="visible"
    exit="exit"
    variants=variants
    class="variants-demo".to_string()
>
    "Variants Animation"
</MotionDiv>
```

## Event Handlers

Custom event handling for advanced use cases.

```rust
use leptos_motion::*;

let event_handlers = EventHandlers::new()
    .on_animation_start(|event| {
        console_log!("Animation started: {:?}", event);
    })
    .on_animation_end(|event| {
        console_log!("Animation ended: {:?}", event);
    })
    .on_animation_iteration(|event| {
        console_log!("Animation iteration: {:?}", event);
    });

<MotionDiv
    event_handlers=Some(event_handlers)
    class="event-demo".to_string()
>
    "Event Demo"
</MotionDiv>
```

## Best Practices

### Performance

1. **Use layout animations sparingly**: Layout animations can be expensive
2. **Prefer transform properties**: Use `x`, `y`, `scale`, `rotate` over layout properties
3. **Use appropriate easing**: `EaseOut` is often hardware accelerated
4. **Limit concurrent animations**: Too many simultaneous animations can impact performance

### API Usage

1. **Use motion_target! macro**: More readable than manual construction
2. **Leverage animation presets**: Use built-in presets for common patterns
3. **Use variants for complex animations**: Better organization than inline targets
4. **Handle presence animations properly**: Always use AnimatePresence for enter/exit

### Type Safety

1. **Use proper types**: Always use `AnimationValue` variants
2. **Handle Option types**: Most animation props are optional
3. **Use string conversion**: Convert `&str` to `String` for class and style props
4. **Leverage Rust's type system**: Let the compiler catch animation errors

## Migration from Framer Motion

### Key Differences

1. **Type Safety**: All animation values are strongly typed
2. **Macro Usage**: Use `motion_target!` instead of object literals
3. **String Conversion**: Explicit `.to_string()` for string props
4. **Component Names**: `MotionDiv` instead of `motion.div`

### Migration Examples

```rust
// Framer Motion (JavaScript)
// <motion.div
//   initial={{ opacity: 0, scale: 0.5 }}
//   animate={{ opacity: 1, scale: 1 }}
//   transition={{ duration: 0.5 }}
// >

// Leptos Motion (Rust)
<MotionDiv
    initial=motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "scale" => AnimationValue::Number(0.5)
    )
    animate=motion_target!(
        "opacity" => AnimationValue::Number(1.0),
        "scale" => AnimationValue::Number(1.0)
    )
    transition=Transition {
        duration: Some(0.5),
        ease: Easing::EaseOut,
        ..Default::default()
    }
    class="migrated-component".to_string()
>
    "Migrated Content"
</MotionDiv>
```

## Error Handling

Common errors and their solutions:

### Type Mismatches

```rust
// ❌ Wrong: &str instead of String
class="my-class"

// ✅ Correct: Convert to String
class="my-class".to_string()
```

### Missing Props

```rust
// ❌ Wrong: Missing required props
<MotionDiv>
    "Content"
</MotionDiv>

// ✅ Correct: Provide required props
<MotionDiv
    class="my-class".to_string()
>
    "Content"
</MotionDiv>
```

### Animation Value Types

```rust
// ❌ Wrong: Wrong AnimationValue type
"opacity" => AnimationValue::Pixels(1.0)

// ✅ Correct: Use Number for unitless values
"opacity" => AnimationValue::Number(1.0)
```

## Examples

See the `examples/` directory for comprehensive usage examples:

- `examples/showcase/` - Basic usage examples
- `examples/advanced-features/` - Advanced animation features
- `examples/mobile-app/` - Mobile-optimized animations
- `examples/dashboard-app/` - Dashboard-style animations
- `examples/e-commerce-gallery/` - E-commerce gallery animations
