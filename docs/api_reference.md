# Leptos Motion API Reference

This document provides a comprehensive reference for the Leptos Motion API, including all public types, functions, and components.

## Table of Contents

- [Core Types](#core-types)
- [Animation Values](#animation-values)
- [Motion Values](#motion-values)
- [Transitions](#transitions)
- [Easing Functions](#easing-functions)
- [Spring Configuration](#spring-configuration)
- [Components](#components)
- [Hooks](#hooks)
- [Utilities](#utilities)

## Core Types

### AnimationValue

Represents a value that can be animated.

```rust
pub enum AnimationValue {
    Number(f64),
    Pixels(f64),
    Degrees(f64),
    Color(Rgba),
    Transform(Transform),
}
```

**Variants:**
- `Number(f64)` - A plain number value
- `Pixels(f64)` - A value in pixels
- `Degrees(f64)` - A value in degrees
- `Color(Rgba)` - A color value
- `Transform(Transform)` - A transform value

### AnimationTarget

A collection of animation values keyed by property names.

```rust
pub struct AnimationTarget {
    // Implementation details
}
```

**Methods:**
- `new() -> Self` - Create a new empty animation target
- `insert(key: String, value: AnimationValue)` - Insert a value
- `get(key: &str) -> Option<&AnimationValue>` - Get a value by key
- `remove(key: &str) -> Option<AnimationValue>` - Remove a value

### Transform

Represents a 2D transform with translation, scale, rotation, and skew.

```rust
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
    pub skew_x: f64,
    pub skew_y: f64,
}
```

**Methods:**
- `new() -> Self` - Create a new transform
- `compose(&self, other: &Transform) -> Transform` - Compose with another transform
- `to_css(&self) -> String` - Convert to CSS transform string

## Animation Values

### MotionValue<T>

A reactive value that tracks animation state.

```rust
pub struct MotionValue<T: Clone + Send + Sync + 'static> {
    // Implementation details
}
```

**Methods:**
- `new(initial: T) -> Self` - Create a new motion value
- `get(&self) -> T` - Get the current value
- `set(&self, value: T)` - Set the value
- `get_velocity(&self) -> f64` - Get the current velocity
- `set_velocity(&self, velocity: f64)` - Set the velocity
- `subscribe(&self, callback: impl Fn(&T) + Send + Sync + 'static)` - Subscribe to changes

### MotionNumber

A specialized motion value for f64 numbers.

```rust
pub type MotionNumber = MotionValue<f64>;
```

### MotionTransform

A specialized motion value for transforms.

```rust
pub type MotionTransform = MotionValue<Transform>;
```

### MotionValues

A collection of motion values.

```rust
pub struct MotionValues {
    // Implementation details
}
```

**Methods:**
- `new() -> Self` - Create a new collection
- `add(&mut self, key: &str, value: MotionValue<T>)` - Add a motion value
- `get(&self, key: &str) -> Option<&MotionValue<T>>` - Get a motion value
- `keys(&self) -> impl Iterator<Item = &String>` - Get all keys

## Transitions

### Transition

Configuration for how an animation should transition.

```rust
pub struct Transition {
    pub duration: Option<f64>,
    pub delay: Option<f64>,
    pub ease: Easing,
    pub repeat: Option<RepeatConfig>,
    pub stagger: Option<StaggerConfig>,
}
```

**Fields:**
- `duration` - Animation duration in seconds
- `delay` - Delay before animation starts in seconds
- `ease` - Easing function to use
- `repeat` - Repeat configuration
- `stagger` - Stagger configuration for multiple elements

### RepeatConfig

Configuration for repeating animations.

```rust
pub struct RepeatConfig {
    pub count: Option<usize>,
    pub reverse: bool,
    pub delay: Option<f64>,
}
```

### StaggerConfig

Configuration for staggered animations.

```rust
pub struct StaggerConfig {
    pub delay: f64,
    pub from: StaggerFrom,
    pub ease: Easing,
}
```

## Easing Functions

### Easing

An easing function for animations.

```rust
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    BackIn,
    BackOut,
    BackInOut,
    Spring(SpringConfig),
    CubicBezier(f64, f64, f64, f64),
}
```

**Methods:**
- `evaluate(&self, t: f64) -> f64` - Evaluate the easing function at time t

## Spring Configuration

### SpringConfig

Configuration for spring-based animations.

```rust
pub struct SpringConfig {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub rest_speed: f64,
    pub rest_delta: f64,
}
```

**Default values:**
- `stiffness`: 100.0
- `damping`: 10.0
- `mass`: 1.0
- `rest_speed`: 0.01
- `rest_delta`: 0.01

## Components

### MotionDiv

A div element with motion capabilities.

```rust
pub fn MotionDiv(
    #[prop(optional)] initial: Option<AnimationTarget>,
    #[prop(optional)] animate: Option<AnimationTarget>,
    #[prop(optional)] exit: Option<AnimationTarget>,
    #[prop(optional)] transition: Option<Transition>,
    #[prop(optional)] variants: Option<Variants>,
    #[prop(optional)] layout: Option<bool>,
    #[prop(optional)] drag: Option<DragConfig>,
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    #[prop(optional)] while_focus: Option<AnimationTarget>,
    #[prop(optional)] while_in_view: Option<AnimationTarget>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] id: Option<String>,
    children: Children,
) -> impl IntoView
```

### MotionSpan

A span element with motion capabilities.

```rust
pub fn MotionSpan(
    // Same props as MotionDiv
    children: Children,
) -> impl IntoView
```

### AnimatePresence

A component for animating elements as they enter and exit the DOM.

```rust
pub fn AnimatePresence(
    #[prop(optional)] mode: Option<PresenceMode>,
    children: Children,
) -> impl IntoView
```

## Hooks

### use_animation

A hook for managing animation state.

```rust
pub fn use_animation() -> (ReadSignal<bool>, WriteSignal<bool>)
```

**Returns:**
- `ReadSignal<bool>` - Read signal for animation state
- `WriteSignal<bool>` - Write signal for animation state

### use_in_view

A hook for detecting when an element is in view.

```rust
pub fn use_in_view(element: NodeRef<leptos::html::Div>) -> ReadSignal<bool>
```

**Parameters:**
- `element` - Reference to the element to monitor

**Returns:**
- `ReadSignal<bool>` - Read signal indicating if element is in view

## Utilities

### motion_target!

A macro for creating animation targets.

```rust
motion_target!(
    "opacity" => AnimationValue::Number(1.0),
    "x" => AnimationValue::Pixels(100.0),
    "scale" => AnimationValue::Number(1.5)
)
```

### Variants

A builder for creating animation variants.

```rust
pub struct Variants {
    // Implementation details
}

impl Variants {
    pub fn new() -> Self
    pub fn variant(self, name: &str, target: AnimationTarget) -> Self
}
```

### DragConfig

Configuration for drag interactions.

```rust
pub struct DragConfig {
    pub axis: DragAxis,
    pub constraints: DragConstraints,
    pub snap_to_grid: Option<f64>,
    pub snap_to_center: bool,
}
```

### DragAxis

Axis configuration for drag interactions.

```rust
pub enum DragAxis {
    X,
    Y,
    Both,
}
```

### DragConstraints

Constraints for drag interactions.

```rust
pub struct DragConstraints {
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
    pub bottom: Option<f64>,
}
```

## Error Types

### AnimationError

Errors that can occur during animation.

```rust
pub enum AnimationError {
    InvalidProperty { property: String },
    AlreadyRunning { handle: AnimationHandle },
    NotFound { handle: AnimationHandle },
    EngineError(String),
}
```

### AnimationHandle

A handle to a running animation.

```rust
pub struct AnimationHandle {
    // Implementation details
}
```

## Constants

### Default Values

```rust
// Default spring configuration
pub const DEFAULT_SPRING: SpringConfig = SpringConfig {
    stiffness: 100.0,
    damping: 10.0,
    mass: 1.0,
    rest_speed: 0.01,
    rest_delta: 0.01,
};

// Default transition
pub const DEFAULT_TRANSITION: Transition = Transition {
    duration: None,
    delay: None,
    ease: Easing::Linear,
    repeat: None,
    stagger: None,
};
```

## Examples

### Basic Animation

```rust
use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn AnimatedBox() -> impl IntoView {
    view! {
        <MotionDiv
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(100.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "Animated Content"
        </MotionDiv>
    }
}
```

### Spring Animation

```rust
#[component]
fn SpringBox() -> impl IntoView {
    view! {
        <MotionDiv
            animate=motion_target!(
                "scale" => AnimationValue::Number(2.0)
            )
            transition=Transition {
                ease: Easing::Spring(SpringConfig {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }
        >
            "Spring Animation"
        </MotionDiv>
    }
}
```

### Gesture Animation

```rust
#[component]
fn GestureBox() -> impl IntoView {
    view! {
        <MotionDiv
            while_hover=Some(motion_target!(
                "scale" => AnimationValue::Number(1.1),
                "rotate" => AnimationValue::Degrees(5.0)
            ))
            while_tap=Some(motion_target!(
                "scale" => AnimationValue::Number(0.9)
            ))
        >
            "Hover and Tap Me"
        </MotionDiv>
    }
}
```

### Variants

```rust
#[component]
fn VariantBox() -> impl IntoView {
    let variants = Variants::new()
        .variant("hidden", motion_target!(
            "opacity" => AnimationValue::Number(0.0),
            "x" => AnimationValue::Pixels(-100.0)
        ))
        .variant("visible", motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "x" => AnimationValue::Pixels(0.0)
        ));

    view! {
        <MotionDiv
            variants=Some(variants)
            initial=Some("hidden".to_string())
            animate=Some("visible".to_string())
        >
            "Variant Animation"
        </MotionDiv>
    }
}
```

This API reference provides comprehensive documentation for all public APIs in Leptos Motion. For more detailed examples and tutorials, see the [Getting Started Guide](getting_started.md) and [Examples](../examples/).
