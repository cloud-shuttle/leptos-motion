# API Design Specifications

## Overview

This document outlines the unified API design for the Leptos Motion library, consolidating the current multiple competing implementations into a single, coherent interface.

## Design Principles

### 1. Consistency
- Unified API across all animation types
- Consistent naming conventions
- Standardized error handling
- Predictable behavior patterns

### 2. Ergonomics
- Intuitive component APIs
- Minimal boilerplate
- Type-safe configurations
- Clear documentation

### 3. Performance
- Zero-cost abstractions where possible
- Efficient memory usage
- Optimized rendering paths
- Minimal JavaScript interop

## Core API Structure

### 1. Motion Components

```rust
// Primary motion component
pub struct MotionDiv {
    // Component props
    pub initial: Option<MotionValues>,
    pub animate: Option<MotionValues>,
    pub exit: Option<MotionValues>,
    pub transition: Option<Transition>,
    pub variants: Option<Variants>,
    pub drag: Option<DragConfig>,
    pub layout: Option<bool>,
    pub layout_id: Option<String>,
    pub on_animation_start: Option<Box<dyn Fn()>>,
    pub on_animation_complete: Option<Box<dyn Fn()>>,
    pub on_drag_start: Option<Box<dyn Fn()>>,
    pub on_drag: Option<Box<dyn Fn(f64, f64)>>,
    pub on_drag_end: Option<Box<dyn Fn()>>,
}

// Animate presence component
pub struct AnimatePresence {
    pub mode: PresenceMode,
    pub initial: Option<bool>,
    pub on_exit_complete: Option<Box<dyn Fn()>>,
}

// Motion value component
pub struct MotionValue<T> {
    pub value: T,
    pub transition: Option<Transition>,
}
```

### 2. Animation Values

```rust
pub type MotionValues = HashMap<String, MotionValue<AnimationValue>>;

#[derive(Debug, Clone)]
pub enum AnimationValue {
    Number(f64),
    String(String),
    Color(String),
    Pixels(f64),
    Percent(f64),
    Degrees(f64),
    Radians(f64),
    Transform(Transform),
    Complex(ComplexValue),
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub translate_x: Option<f64>,
    pub translate_y: Option<f64>,
    pub translate_z: Option<f64>,
    pub scale: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
    pub rotate: Option<f64>,
    pub rotate_x: Option<f64>,
    pub rotate_y: Option<f64>,
    pub rotate_z: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
}
```

### 3. Transition Configuration

```rust
#[derive(Debug, Clone)]
pub struct Transition {
    pub duration: Option<f64>,
    pub delay: Option<f64>,
    pub ease: Option<Easing>,
    pub repeat: Option<RepeatConfig>,
    pub repeat_type: Option<RepeatType>,
    pub repeat_delay: Option<f64>,
    pub stagger: Option<StaggerConfig>,
    pub stagger_direction: Option<StaggerDirection>,
    pub stagger_children: Option<f64>,
    pub when: Option<String>,
    pub type_: Option<AnimationType>,
    pub damping: Option<f64>,
    pub stiffness: Option<f64>,
    pub mass: Option<f64>,
    pub velocity: Option<f64>,
    pub rest_delta: Option<f64>,
    pub rest_speed: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    Spring(SpringConfig),
    CubicBezier(f64, f64, f64, f64),
}
```

### 4. Variants System

```rust
#[derive(Debug, Clone)]
pub struct Variants {
    pub variants: HashMap<String, MotionValues>,
    pub transition: Option<Transition>,
}

impl Variants {
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            transition: None,
        }
    }
    
    pub fn add_variant(&mut self, name: String, values: MotionValues) {
        self.variants.insert(name, values);
    }
    
    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }
}
```

## Drag and Drop API

### 1. Drag Configuration

```rust
#[derive(Debug, Clone)]
pub struct DragConfig {
    pub enabled: bool,
    pub axis: Option<DragAxis>,
    pub constraints: Option<DragConstraints>,
    pub momentum: Option<DragMomentum>,
    pub drag_propagation: Option<bool>,
    pub drag_elastics: Option<DragElastics>,
    pub drag_momentum: Option<bool>,
    pub drag_transition: Option<Transition>,
    pub drag_snap: Option<DragSnap>,
}

#[derive(Debug, Clone)]
pub enum DragAxis {
    X,
    Y,
    XY,
}

#[derive(Debug, Clone)]
pub struct DragConstraints {
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
    pub min_z: Option<f64>,
    pub max_z: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct DragMomentum {
    pub enabled: bool,
    pub damping: f64,
    pub stiffness: f64,
    pub mass: f64,
    pub velocity: f64,
    pub rest_delta: f64,
    pub rest_speed: f64,
}
```

### 2. Drag Events

```rust
pub struct DragEvent {
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub point: (f64, f64),
}

pub trait DragEventHandler {
    fn on_drag_start(&self, event: &DragEvent);
    fn on_drag(&self, event: &DragEvent);
    fn on_drag_end(&self, event: &DragEvent);
}
```

## Layout Animation API

### 1. Layout Configuration

```rust
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub enabled: bool,
    pub transition: Option<Transition>,
    pub layout_id: Option<String>,
    pub layout_root: Option<bool>,
    pub layout_dependency: Option<String>,
}

impl LayoutConfig {
    pub fn new() -> Self {
        Self {
            enabled: true,
            transition: None,
            layout_id: None,
            layout_root: None,
            layout_dependency: None,
        }
    }
    
    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }
    
    pub fn with_layout_id(mut self, layout_id: String) -> Self {
        self.layout_id = Some(layout_id);
        self
    }
}
```

### 2. Layout Events

```rust
pub struct LayoutEvent {
    pub layout_id: String,
    pub bounds: BoundingBox,
    pub delta: Vector2,
    pub velocity: Vector2,
}

pub trait LayoutEventHandler {
    fn on_layout_start(&self, event: &LayoutEvent);
    fn on_layout_complete(&self, event: &LayoutEvent);
}
```

## Gesture API

### 1. Gesture Configuration

```rust
#[derive(Debug, Clone)]
pub struct GestureConfig {
    pub tap: Option<TapConfig>,
    pub pan: Option<PanConfig>,
    pub pinch: Option<PinchConfig>,
    pub rotate: Option<RotateConfig>,
    pub hover: Option<HoverConfig>,
    pub focus: Option<FocusConfig>,
}

#[derive(Debug, Clone)]
pub struct TapConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub timeout: f64,
    pub on_tap: Option<Box<dyn Fn()>>,
    pub on_tap_start: Option<Box<dyn Fn()>>,
    pub on_tap_cancel: Option<Box<dyn Fn()>>,
}

#[derive(Debug, Clone)]
pub struct PanConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub on_pan: Option<Box<dyn Fn(f64, f64)>>,
    pub on_pan_start: Option<Box<dyn Fn()>>,
    pub on_pan_end: Option<Box<dyn Fn()>>,
}
```

### 2. Gesture Events

```rust
pub struct GestureEvent {
    pub gesture_type: GestureType,
    pub position: (f64, f64),
    pub delta: (f64, f64),
    pub velocity: (f64, f64),
    pub scale: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone)]
pub enum GestureType {
    Tap,
    Pan,
    Pinch,
    Rotate,
    Hover,
    Focus,
}
```

## Animation Hooks

### 1. Animation Hooks

```rust
pub struct AnimationHooks {
    pub on_animation_start: Option<Box<dyn Fn()>>,
    pub on_animation_complete: Option<Box<dyn Fn()>>,
    pub on_animation_cancel: Option<Box<dyn Fn()>>,
    pub on_update: Option<Box<dyn Fn(f64)>>,
}

impl AnimationHooks {
    pub fn new() -> Self {
        Self {
            on_animation_start: None,
            on_animation_complete: None,
            on_animation_cancel: None,
            on_update: None,
        }
    }
    
    pub fn on_start(mut self, callback: Box<dyn Fn()>) -> Self {
        self.on_animation_start = Some(callback);
        self
    }
    
    pub fn on_complete(mut self, callback: Box<dyn Fn()>) -> Self {
        self.on_animation_complete = Some(callback);
        self
    }
}
```

### 2. Animation Controls

```rust
pub struct AnimationControls {
    pub start: Box<dyn Fn()>,
    pub stop: Box<dyn Fn()>,
    pub pause: Box<dyn Fn()>,
    pub resume: Box<dyn Fn()>,
    pub reverse: Box<dyn Fn()>,
    pub seek: Box<dyn Fn(f64)>,
    pub set: Box<dyn Fn(MotionValues)>,
}

impl AnimationControls {
    pub fn new() -> Self {
        Self {
            start: Box::new(|| {}),
            stop: Box::new(|| {}),
            pause: Box::new(|| {}),
            resume: Box::new(|| {}),
            reverse: Box::new(|| {}),
            seek: Box::new(|_| {}),
            set: Box::new(|_| {}),
        }
    }
}
```

## Error Handling

### 1. Animation Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("Invalid animation value: {0}")]
    InvalidValue(String),
    
    #[error("Animation not found: {0}")]
    AnimationNotFound(String),
    
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),
    
    #[error("Drag constraint violation: {0}")]
    DragConstraintViolation(String),
    
    #[error("Layout animation failed: {0}")]
    LayoutAnimationFailed(String),
    
    #[error("Gesture recognition failed: {0}")]
    GestureRecognitionFailed(String),
}
```

### 2. Error Recovery

```rust
impl MotionDiv {
    fn handle_animation_error(&self, error: AnimationError) -> Result<()> {
        match error {
            AnimationError::InvalidValue(_) => {
                // Log error and continue with default values
                log::warn!("Invalid animation value, using default");
                Ok(())
            }
            AnimationError::AnimationNotFound(_) => {
                // Create new animation
                self.create_animation()?;
                Ok(())
            }
            _ => Err(error),
        }
    }
}
```

## Performance Considerations

### 1. Optimization Strategies

```rust
pub struct PerformanceConfig {
    pub enable_hardware_acceleration: bool,
    pub enable_batching: bool,
    pub enable_culling: bool,
    pub max_animations: usize,
    pub frame_rate_target: f64,
}

impl PerformanceConfig {
    pub fn default() -> Self {
        Self {
            enable_hardware_acceleration: true,
            enable_batching: true,
            enable_culling: true,
            max_animations: 100,
            frame_rate_target: 60.0,
        }
    }
}
```

### 2. Memory Management

```rust
pub struct MemoryConfig {
    pub enable_object_pooling: bool,
    pub max_pool_size: usize,
    pub cleanup_interval: f64,
    pub enable_garbage_collection: bool,
}

impl MemoryConfig {
    pub fn default() -> Self {
        Self {
            enable_object_pooling: true,
            max_pool_size: 1000,
            cleanup_interval: 1000.0, // 1 second
            enable_garbage_collection: true,
        }
    }
}
```

## Testing Strategy

### 1. API Testing

```rust
#[cfg(test)]
mod api_tests {
    use super::*;
    
    #[test]
    fn test_motion_div_creation() {
        let motion_div = MotionDiv::new()
            .with_initial(MotionValues::new())
            .with_animate(MotionValues::new())
            .with_transition(Transition::default());
        
        assert!(motion_div.is_valid());
    }
    
    #[test]
    fn test_drag_configuration() {
        let drag_config = DragConfig {
            enabled: true,
            axis: Some(DragAxis::XY),
            constraints: Some(DragConstraints::new()),
            momentum: Some(DragMomentum::default()),
            ..Default::default()
        };
        
        assert!(drag_config.is_valid());
    }
}
```

### 2. Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_animation_lifecycle() {
        let mut motion_div = MotionDiv::new();
        let initial_values = MotionValues::new();
        let animate_values = MotionValues::new();
        
        // Test animation start
        motion_div.set_initial(initial_values);
        motion_div.set_animate(animate_values);
        motion_div.start_animation();
        
        assert!(motion_div.is_animating());
        
        // Test animation completion
        motion_div.wait_for_completion();
        assert!(!motion_div.is_animating());
    }
}
```

## Migration Guide

### 1. From Current Implementation

```rust
// Old API
let motion_div = ReactiveMotionDiv::new()
    .with_initial(initial_values)
    .with_animate(animate_values);

// New API
let motion_div = MotionDiv::new()
    .with_initial(initial_values)
    .with_animate(animate_values);
```

### 2. Breaking Changes

- `ReactiveMotionDiv` → `MotionDiv`
- `DragMotionDiv` → `MotionDiv` with drag config
- `LayoutMotionDiv` → `MotionDiv` with layout config
- Unified error handling
- Consistent naming conventions

## Conclusion

This API design provides a unified, consistent interface for the Leptos Motion library that addresses the current fragmentation while maintaining performance and usability requirements.
