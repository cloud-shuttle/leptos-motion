# Stagger Animations Design Document

## Overview

Stagger animations create sequential or overlapping animation effects across multiple elements. Instead of all elements animating simultaneously, each element starts its animation with a delay relative to the previous element, creating cascading visual effects.

## Core Concepts

### Stagger Configuration
Define how animations stagger across a group of elements:

```rust
let stagger_config = StaggerConfig {
    delay: 0.1,        // 100ms delay between each element
    start_delay: 0.0,  // Initial delay before first element
    from: "first",     // "first", "last", "center", or index
    direction: "normal", // "normal", "reverse", or "center"
};
```

### Application to Element Groups
Apply stagger to collections of elements:

```rust
<MotionDiv
    stagger=StaggerConfig::default().delay(0.1)
    animate=hashmap! {
        "opacity" => AnimationValue::Number(1.0),
        "y" => AnimationValue::Pixels(0.0),
    }
>
    // Children elements will animate in sequence
    {items.into_iter().map(|item| view! {
        <div class="item">{item}</div>
    }).collect::<Vec<_>>()}
</MotionDiv>
```

## API Design

### StaggerConfig Structure
```rust
#[derive(Clone, Debug)]
pub struct StaggerConfig {
    pub delay: f64,              // Delay between each element (seconds)
    pub start_delay: f64,        // Initial delay before first element
    pub from: StaggerFrom,       // Which element to start from
    pub direction: StaggerDirection, // Animation direction
    pub ease: Option<EasingFunction>, // Optional easing override
}

#[derive(Clone, Debug)]
pub enum StaggerFrom {
    First,      // Start from first element
    Last,       // Start from last element
    Center,     // Start from center element
    Index(usize), // Start from specific index
}

#[derive(Clone, Debug)]
pub enum StaggerDirection {
    Normal,     // Forward direction (0, 1, 2, ...)
    Reverse,    // Backward direction (..., 2, 1, 0)
    Center,     // Outward from center (1, 0, 2, 3, ...)
}
```

### StaggerAnimation Component
```rust
#[component]
pub fn StaggeredMotion(
    children: Children,
    stagger: StaggerConfig,
    animate: AnimateProp,
    initial: Option<AnimateProp>,
    exit: Option<AnimateProp>,
    transition: Option<Transition>,
) -> impl IntoView
```

## Implementation Strategy

### 1. Element Tracking
- Automatically detect child elements
- Assign stagger index to each element
- Track animation state per element

### 2. Delay Calculation
For N elements with stagger delay D:

```
Element 0: delay = start_delay
Element 1: delay = start_delay + D
Element 2: delay = start_delay + 2D
...
Element N: delay = start_delay + ND
```

### 3. Direction Handling

#### Normal Direction (0 → N)
```
[0] → [1] → [2] → [3] → [4]
```

#### Reverse Direction (N → 0)
```
[4] → [3] → [2] → [1] → [0]
```

#### Center Direction (outward from center)
```
     [2]
   [1]   [3]
 [0]   [4]   [5]
```

## Advanced Features

### Dynamic Staggering
Stagger delay can be calculated dynamically:

```rust
let dynamic_stagger = StaggerConfig {
    delay: 0.05,  // 50ms
    start_delay: 0.0,
    from: StaggerFrom::Center,
    direction: StaggerDirection::Center,
};
```

### Conditional Staggering
Apply different stagger patterns based on conditions:

```rust
let conditional_stagger = if is_mobile {
    StaggerConfig::default().delay(0.05)  // Faster on mobile
} else {
    StaggerConfig::default().delay(0.1)   // Slower on desktop
};
```

### Stagger Groups
Group elements for different stagger patterns:

```rust
<StaggeredMotion stagger=group_a_config>
    <div class="group-a">Item 1</div>
    <div class="group-a">Item 2</div>
</StaggeredMotion>

<StaggeredMotion stagger=group_b_config>
    <div class="group-b">Item 3</div>
    <div class="group-b">Item 4</div>
</StaggeredMotion>
```

## Integration Points

### With MotionDiv
- Extend MotionDiv with `stagger` prop
- Automatic child element detection
- Integration with existing animation system

### With Variants
Stagger animations work with variants:

```rust
<MotionDiv
    variants=card_variants
    stagger=stagger_config
    animate="visible"
/>
```

### With Layout Animations
Stagger elements appearing in layout animations.

### With Gestures
Stagger animations triggered by user interactions.

## Performance Considerations

- Minimal overhead for stagger calculation
- Efficient element tracking and indexing
- Reuse stagger configurations across components
- Memory-efficient storage of stagger state

## Animation Timing

### Sequential Staggering
Each element waits for the previous to complete:

```
Element 0: starts at T+0, duration D
Element 1: starts at T+D, duration D
Element 2: starts at T+2D, duration D
```

### Overlapping Staggering
Elements start before previous complete:

```
Element 0: starts at T+0, duration D
Element 1: starts at T+0.5, duration D (50% overlap)
Element 2: starts at T+1.0, duration D (50% overlap)
```

## Testing Strategy

### Unit Tests
- Stagger delay calculations
- Direction handling (normal, reverse, center)
- Edge cases (single element, empty groups)

### Integration Tests
- StaggeredMotion component functionality
- MotionDiv stagger prop
- Complex stagger patterns

### E2E Tests
- Visual stagger animations
- Performance with many elements
- Browser compatibility

### Performance Tests
- Stagger calculation overhead
- Memory usage with large element groups
- Animation frame rate maintenance
