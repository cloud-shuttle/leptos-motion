# Variants System Design Document

## Overview

The Variants System provides a way to define named animation states (variants) that can be applied to components. This is a fundamental feature that enables complex state-based animations and reusable animation definitions.

## Core Concepts

### Variant Definition
A variant is a named set of animation properties that can be applied to a component:

```rust
let variants = Variants::new()
    .add("initial", hashmap! {
        "opacity" => AnimationValue::Number(0.0),
        "scale" => AnimationValue::Number(0.8),
    })
    .add("enter", hashmap! {
        "opacity" => AnimationValue::Number(1.0),
        "scale" => AnimationValue::Number(1.0),
    })
    .add("exit", hashmap! {
        "opacity" => AnimationValue::Number(0.0),
        "scale" => AnimationValue::Number(0.9),
    });
```

### Component Integration
Components can specify which variant to use via props:

```rust
<MotionDiv
    variants=variants
    initial="initial"
    animate="enter"
    exit="exit"
    // ... other props
>
    "Content"
</MotionDiv>
```

## API Design

### Variants Structure
```rust
#[derive(Clone)]
pub struct Variants {
    variants: HashMap<String, HashMap<String, AnimationValue>>,
    default_transition: Option<Transition>,
}

impl Variants {
    pub fn new() -> Self;
    pub fn add<S: Into<String>>(mut self, name: S, properties: HashMap<String, AnimationValue>) -> Self;
    pub fn with_transition(mut self, transition: Transition) -> Self;
    pub fn get(&self, name: &str) -> Option<&HashMap<String, AnimationValue>>;
}
```

### MotionDiv Props Extension
```rust
#[component]
fn MotionDiv(
    // ... existing props ...
    variants: Option<Variants>,
    initial: Option<String>,  // variant name
    animate: Option<String>,  // variant name
    exit: Option<String>,     // variant name
    // ... rest of props ...
) -> impl IntoView
```

## Implementation Strategy

### 1. Variant Resolution
When a variant name is specified, resolve it to animation properties:
- Look up variant in Variants map
- Merge with explicit animate/initial props (explicit props take precedence)
- Apply default transition if specified

### 2. State Management
Track current variant state for transitions:
- `initial` → `animate` on mount
- `animate` → `exit` on unmount
- Dynamic variant changes via reactive updates

### 3. Inheritance
Support variant inheritance from parent components for complex layouts.

## Integration Points

### With MotionDiv
- Extend MotionDiv to accept variants, initial, animate, exit props
- Integrate with existing animation system
- Maintain backward compatibility

### With Layout Animations
- Variants can specify layout properties
- Automatic layout animations between variants

### With Shared Layout
- Variants work with layout_id for shared element transitions

## Performance Considerations

- Variant lookups should be O(1)
- Minimize allocations during variant resolution
- Cache resolved variant properties when possible
- Support lazy evaluation for complex variants

## Testing Strategy

### Unit Tests
- Variant creation and resolution
- Property merging precedence
- Transition application

### Integration Tests
- MotionDiv with variants
- Variant transitions
- Inheritance scenarios

### E2E Tests
- Complex variant-based animations
- State transitions
- Performance benchmarks
