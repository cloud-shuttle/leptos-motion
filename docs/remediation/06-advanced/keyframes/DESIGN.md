# Keyframes System Design Document

## Overview

The Keyframes System enables complex multi-step animations by defining intermediate animation states. Unlike simple from/to animations, keyframes allow specifying multiple intermediate points that an animation passes through.

## Core Concepts

### Keyframe Definition
A keyframe animation defines multiple animation states at specific points in time:

```rust
let keyframes = Keyframes::new(vec![
    Keyframe::new(0.0, hashmap! {  // 0% through animation
        "opacity" => AnimationValue::Number(0.0),
        "x" => AnimationValue::Pixels(0.0),
    }),
    Keyframe::new(0.5, hashmap! {  // 50% through animation
        "opacity" => AnimationValue::Number(0.8),
        "x" => AnimationValue::Pixels(100.0),
    }),
    Keyframe::new(1.0, hashmap! {  // 100% through animation
        "opacity" => AnimationValue::Number(1.0),
        "x" => AnimationValue::Pixels(200.0),
    }),
]);
```

### Animation Integration
Keyframes integrate with the existing animation system:

```rust
<MotionDiv
    animate=AnimateProp::Keyframes(keyframes)
    transition=Transition {
        duration: Some(2.0),
        ease: Some(Easing::Linear),
        ..Default::default()
    }
/>
```

## API Design

### Keyframe Structure
```rust
#[derive(Clone)]
pub struct Keyframe {
    pub progress: f64,  // 0.0 to 1.0
    pub properties: HashMap<String, AnimationValue>,
    pub easing: Option<EasingFunction>,  // Optional per-keyframe easing
}

impl Keyframe {
    pub fn new(progress: f64, properties: HashMap<String, AnimationValue>) -> Self;
    pub fn with_easing(mut self, easing: EasingFunction) -> Self;
}
```

### Keyframes Container
```rust
#[derive(Clone)]
pub struct Keyframes {
    keyframes: Vec<Keyframe>,
}

impl Keyframes {
    pub fn new(keyframes: Vec<Keyframe>) -> Self;
    pub fn add(mut self, keyframe: Keyframe) -> Self;
    pub fn get_at_progress(&self, progress: f64) -> HashMap<String, AnimationValue>;
    pub fn validate(&self) -> Result<(), String>;
}
```

### AnimateProp Extension
```rust
pub enum AnimateProp {
    // ... existing variants ...
    Keyframes(Keyframes),
}
```

## Implementation Strategy

### 1. Keyframe Interpolation
- Linear interpolation between keyframes for each property
- Support different interpolation types (linear, ease-in, custom curves)
- Handle mismatched properties between keyframes

### 2. Timing Integration
- Keyframes work with existing Transition system
- Support for custom timing per keyframe
- Integration with animation easing functions

### 3. Property Handling
- Automatic interpolation for numeric properties
- Support for color interpolation (future enhancement)
- Transform property interpolation

## Advanced Features

### Per-Keyframe Easing
```rust
Keyframe::new(0.5, properties)
    .with_easing(EasingFunction::EaseOut)
```

### Keyframe Composition
Combine multiple keyframe sequences for complex animations.

### Dynamic Keyframes
Generate keyframes programmatically based on data.

## Integration Points

### With Variants System
Keyframes can be used within variants:

```rust
let variants = Variants::new()
    .add("bounce", Keyframes::new(vec![
        Keyframe::new(0.0, hashmap!("y" => AnimationValue::Pixels(0.0))),
        Keyframe::new(0.5, hashmap!("y" => AnimationValue::Pixels(-50.0))),
        Keyframe::new(1.0, hashmap!("y" => AnimationValue::Pixels(0.0))),
    ]));
```

### With Motion Paths
Keyframe-based path animations for complex SVG animations.

### With Scroll Animations
Keyframe animations triggered by scroll position.

## Performance Considerations

- Pre-compute interpolation functions when possible
- Cache keyframe resolution for repeated animations
- Optimize for common keyframe patterns (bounce, pulse, etc.)
- Minimize allocations during animation playback

## Interpolation Algorithms

### Linear Interpolation
```rust
fn lerp(start: f64, end: f64, progress: f64) -> f64 {
    start + (end - start) * progress
}
```

### Eased Interpolation
Apply easing functions to keyframe transitions.

### Property-Specific Interpolation
- Numbers: Linear interpolation
- Colors: HSL interpolation (future)
- Transforms: Matrix interpolation
- Paths: Path morphing

## Testing Strategy

### Unit Tests
- Keyframe creation and validation
- Interpolation accuracy
- Edge cases (single keyframe, empty keyframes)

### Integration Tests
- MotionDiv with keyframe animations
- Complex multi-property keyframes
- Performance benchmarks

### E2E Tests
- Visual keyframe animations
- Timing accuracy
- Browser compatibility
