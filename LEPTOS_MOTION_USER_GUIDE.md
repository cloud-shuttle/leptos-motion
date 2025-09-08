# Leptos Motion User Guide 🎬

**Version**: 0.4.0  
**Status**: Production Ready  
**Bundle Size**: 30KB-85KB (optimized)

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Animation Components](#animation-components)
4. [Animation Values](#animation-values)
5. [Transitions](#transitions)
6. [Gestures](#gestures)
7. [Layout Animations](#layout-animations)
8. [Common Patterns](#common-patterns)
9. [Troubleshooting](#troubleshooting)
10. [API Reference](#api-reference)

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.8"
leptos-motion = "0.4"
```

### Basic Example

```rust
use leptos::*;
use leptos_motion::*;
use std::collections::HashMap;

#[component]
fn App() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);
    
    // Create animation targets
    let initial = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(0.5));
        map
    };
    
    let animate = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    };
    
    view! {
        <button on:click=move |_| set_is_visible.set(!is_visible.get())>
            "Toggle Animation"
        </button>
        
        <MotionDiv
            class=Some("animated-box".to_string())
            initial=Some(initial)
            animate=Some(animate)
            _transition=Some(Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                delay: None,
                repeat: RepeatConfig::Never,
                stagger: None,
            })
        >
            "Hello Leptos Motion!"
        </MotionDiv>
    }
}
```

## Core Concepts

### 1. Animation Targets

Animation targets are `HashMap<String, AnimationValue>` that define what properties to animate:

```rust
let mut target = HashMap::new();
target.insert("opacity".to_string(), AnimationValue::Number(1.0));
target.insert("x".to_string(), AnimationValue::Pixels(100.0));
target.insert("rotate".to_string(), AnimationValue::Degrees(45.0));
```

### 2. Motion Components

Motion components are Leptos components that accept animation props:

- `MotionDiv` - Animated div element
- `MotionSpan` - Animated span element
- `AnimatePresence` - Enter/exit animations

### 3. Animation Lifecycle

1. **Initial** - Starting state when component mounts
2. **Animate** - Target state to animate to
3. **Exit** - State when component unmounts (with AnimatePresence)

## Animation Components

### MotionDiv

The primary animation component for div elements:

```rust
<MotionDiv
    class=Some("my-class".to_string())
    initial=Some(initial_state)
    animate=Some(target_state)
    _transition=Some(transition_config)
    _while_hover=Some(hover_state)
    _while_tap=Some(tap_state)
>
    "Content"
</MotionDiv>
```

**Props:**
- `class: Option<String>` - CSS class name
- `initial: Option<AnimationTarget>` - Initial animation state
- `animate: Option<AnimationTarget>` - Target animation state
- `_transition: Option<Transition>` - Transition configuration
- `_while_hover: Option<AnimationTarget>` - Hover animation state
- `_while_tap: Option<AnimationTarget>` - Tap animation state

### AnimatePresence

Manages enter/exit animations:

```rust
<AnimatePresence mode=Some(PresenceMode::Sync)>
    {move || is_visible.get().then(|| view! {
        <MotionDiv
            initial=Some(exit_state)
            animate=Some(enter_state)
            exit=Some(exit_state)
        >
            "Animated content"
        </MotionDiv>
    })}
</AnimatePresence>
```

## Animation Values

### Supported Types

```rust
// Numeric values (unitless)
AnimationValue::Number(1.0)

// Pixel values
AnimationValue::Pixels(100.0)

// Percentage values
AnimationValue::Percentage(50.0)

// Degree values for rotations
AnimationValue::Degrees(45.0)

// Radian values for rotations
AnimationValue::Radians(1.57)

// Color values (CSS format)
AnimationValue::Color("linear-gradient(45deg, #ff6b6b, #4ecdc4)".to_string())

// String values for non-numeric properties
AnimationValue::String("block".to_string())

// Transform matrix
AnimationValue::Transform(Transform {
    x: Some(100.0),
    y: Some(50.0),
    scale: Some(1.5),
    rotate_z: Some(45.0),
    ..Default::default()
})
```

### Common Properties

```rust
// Transform properties
"x" -> AnimationValue::Pixels(100.0)
"y" -> AnimationValue::Pixels(50.0)
"scale" -> AnimationValue::Number(1.5)
"rotate" -> AnimationValue::Degrees(45.0)
"skewX" -> AnimationValue::Degrees(10.0)

// Style properties
"opacity" -> AnimationValue::Number(0.8)
"background" -> AnimationValue::Color("red".to_string())
"borderRadius" -> AnimationValue::Pixels(10.0)
"width" -> AnimationValue::Pixels(200.0)
"height" -> AnimationValue::Pixels(100.0)
```

## Transitions

### Transition Configuration

```rust
let transition = Transition {
    duration: Some(0.5),           // Duration in seconds
    ease: Easing::EaseOut,         // Easing function
    delay: Some(0.1),              // Delay in seconds
    repeat: RepeatConfig::Never,   // Repeat configuration
    stagger: None,                 // Stagger configuration
};
```

### Easing Functions

```rust
// Basic easing
Easing::Linear
Easing::EaseIn
Easing::EaseOut
Easing::EaseInOut

// Spring physics
Easing::Spring(SpringConfig {
    stiffness: 100.0,
    damping: 10.0,
    mass: 1.0,
    velocity: 0.0,
    rest_delta: 0.01,
    rest_speed: 0.01,
})

// Cubic bezier
Easing::Bezier(0.25, 0.1, 0.25, 1.0)
```

### Repeat Configuration

```rust
RepeatConfig::Never              // No repetition
RepeatConfig::Count(3)           // Repeat 3 times
RepeatConfig::Infinite           // Infinite repetition
RepeatConfig::InfiniteReverse    // Infinite with alternating direction
```

## Gestures

### Drag Configuration

```rust
let drag_config = DragConfig {
    axis: Some(DragAxis::X),
    constraints: Some(DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: None,
        bottom: None,
    }),
};

<MotionDiv
    _drag=Some(drag_config)
    _while_drag=Some(drag_state)
>
    "Draggable content"
</MotionDiv>
```

### Gesture States

```rust
// While dragging
let while_drag = {
    let mut map = HashMap::new();
    map.insert("scale".to_string(), AnimationValue::Number(1.1));
    map.insert("rotate".to_string(), AnimationValue::Degrees(5.0));
    map
};

// While hovering
let while_hover = {
    let mut map = HashMap::new();
    map.insert("y".to_string(), AnimationValue::Pixels(-5.0));
    map.insert("boxShadow".to_string(), AnimationValue::String("0 10px 20px rgba(0,0,0,0.2)".to_string()));
    map
};
```

## Layout Animations

Layout animations automatically animate position and size changes:

```rust
<MotionDiv
    _layout=Some(true)
    style="position: absolute; left: {x}px; top: {y}px;"
>
    "Layout animated content"
</MotionDiv>
```

## Common Patterns

### 1. Fade In Animation

```rust
let fade_in = {
    let mut map = HashMap::new();
    map.insert("opacity".to_string(), AnimationValue::Number(0.0));
    map.insert("y".to_string(), AnimationValue::Pixels(20.0));
    map
};

let fade_in_target = {
    let mut map = HashMap::new();
    map.insert("opacity".to_string(), AnimationValue::Number(1.0));
    map.insert("y".to_string(), AnimationValue::Pixels(0.0));
    map
};
```

### 2. Scale Animation

```rust
let scale_up = {
    let mut map = HashMap::new();
    map.insert("scale".to_string(), AnimationValue::Number(1.1));
    map
};
```

### 3. Slide Animation

```rust
let slide_in = {
    let mut map = HashMap::new();
    map.insert("x".to_string(), AnimationValue::Pixels(-100.0));
    map
};

let slide_target = {
    let mut map = HashMap::new();
    map.insert("x".to_string(), AnimationValue::Pixels(0.0));
    map
};
```

### 4. Staggered Animations

```rust
let stagger_transition = Transition {
    duration: Some(0.3),
    ease: Easing::EaseOut,
    delay: None,
    repeat: RepeatConfig::Never,
    stagger: Some(StaggerConfig {
        delay: 0.1,
        from: StaggerFrom::First,
    }),
};
```

## Troubleshooting

### Common Issues

#### 1. Animations Not Working

**Problem**: Animations don't trigger or are not smooth.

**Solutions**:
- Ensure you're using `Some()` wrappers for optional props
- Check that animation values are correct types
- Verify transition configuration is valid
- Make sure the component is properly mounted

#### 2. Type Mismatches

**Problem**: Compilation errors about type mismatches.

**Solutions**:
- Use correct `AnimationValue` variants
- Ensure `HashMap<String, AnimationValue>` for animation targets
- Use `Some()` for optional props
- Import types from `leptos_motion_core`

#### 3. Performance Issues

**Problem**: Animations are choppy or slow.

**Solutions**:
- Use `will-change` CSS property
- Avoid animating too many properties simultaneously
- Use hardware-accelerated properties (transform, opacity)
- Consider using `layout=false` for non-layout animations

### Debug Tips

1. **Console Logging**: Add logging to track animation state changes
2. **Browser DevTools**: Use performance profiler to identify bottlenecks
3. **Animation Inspector**: Check if animations are actually running
4. **CSS Validation**: Ensure CSS properties are valid

## API Reference

### Core Types

```rust
// Animation target - HashMap of property names to values
pub type AnimationTarget = HashMap<String, AnimationValue>;

// Animation value - represents different animatable types
pub enum AnimationValue {
    Number(f64),
    Pixels(f64),
    Percentage(f64),
    Degrees(f64),
    Radians(f64),
    Color(String),
    Transform(Transform),
    String(String),
    Complex(ComplexValue),
}

// Transition configuration
pub struct Transition {
    pub duration: Option<f64>,
    pub ease: Easing,
    pub delay: Option<f64>,
    pub repeat: RepeatConfig,
    pub stagger: Option<StaggerConfig>,
}

// Easing functions
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring(SpringConfig),
    Bezier(f64, f64, f64, f64),
}

// Repeat configuration
pub enum RepeatConfig {
    Never,
    Count(u32),
    Infinite,
    InfiniteReverse,
}
```

### Component Props

```rust
// MotionDiv props
pub struct MotionDivProps {
    pub class: Option<String>,
    pub initial: Option<AnimationTarget>,
    pub animate: Option<AnimationTarget>,
    pub _transition: Option<Transition>,
    pub _while_hover: Option<AnimationTarget>,
    pub _while_tap: Option<AnimationTarget>,
    pub _layout: Option<bool>,
    pub _drag: Option<DragConfig>,
    pub _drag_constraints: Option<DragConstraints>,
}
```

### Utility Functions

```rust
// Create animation target helper
pub fn create_animation_target() -> AnimationTarget {
    HashMap::new()
}

// Add property to animation target
pub fn add_property(
    target: &mut AnimationTarget,
    name: &str,
    value: AnimationValue,
) {
    target.insert(name.to_string(), value);
}
```

---

## Next Steps

1. **Explore Examples**: Check the `examples/` directory for working demos
2. **Read Architecture**: Understand the underlying animation engine
3. **Join Community**: Get help and share your animations
4. **Contribute**: Help improve the library

**Happy Animating! 🎬✨**
