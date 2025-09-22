# RAF Engine Component Design

## Overview
The RAF (RequestAnimationFrame) Engine provides browser-native animation scheduling using `requestAnimationFrame` for optimal performance and smooth 60fps animations.

## Current Issues
- **CRITICAL**: Stub implementation with placeholder RAF ID
- Transform composition overwrites previous transforms
- Incorrect from/to value derivation
- No actual `requestAnimationFrame` scheduling

## Design Goals
- Real RAF loop integration with browser APIs
- Composite transform handling (x, y, scale, rotate simultaneously)
- Proper from/to value interpolation
- Memory-efficient animation state management
- 60fps performance target

## API Design

### Core Types
```rust
pub struct RafEngine {
    animations: HashMap<AnimationHandle, AnimationState>,
    raf_handle: Option<i32>,
    scheduler: RafScheduler,
    transform_compositor: TransformCompositor,
}

pub struct AnimationState {
    target_element: HtmlElement,
    properties: HashMap<String, PropertyAnimation>,
    start_time: f64,
    duration: f64,
    easing: EasingFunction,
    callbacks: AnimationCallbacks,
}

pub struct PropertyAnimation {
    from_value: AnimationValue,
    to_value: AnimationValue,
    current_value: AnimationValue,
}
```

### Public Interface
```rust
impl RafEngine {
    pub fn new() -> Self
    pub fn start_animation(&mut self, config: AnimationConfig) -> Result<AnimationHandle>
    pub fn stop_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn pause_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn resume_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn get_animation_state(&self, handle: AnimationHandle) -> Option<AnimationProgress>
}
```

## Implementation Plan

### Phase 1: Core RAF Loop (Week 1, Day 1-2)
**File**: `crates/leptos-motion-core/src/engine/raf.rs`
**Target Lines**: <200

```rust
fn start_raf_loop(&mut self) {
    let closure = Closure::wrap(Box::new(move |timestamp: f64| {
        self.tick(timestamp);
        if self.has_active_animations() {
            request_animation_frame(&closure);
        }
    }) as Box<dyn FnMut(f64)>);
    
    self.raf_handle = Some(request_animation_frame(&closure));
    closure.forget(); // Store reference properly
}
```

### Phase 2: Transform Compositor (Week 1, Day 3)
**File**: `crates/leptos-motion-core/src/engine/transform_compositor.rs`
**Target Lines**: <150

```rust
pub struct TransformCompositor {
    transforms: HashMap<String, f64>, // x, y, scale, rotate, etc.
}

impl TransformCompositor {
    pub fn set_property(&mut self, property: &str, value: f64)
    pub fn build_transform_string(&self) -> String
    pub fn clear(&mut self)
}
```

### Phase 3: Property Interpolation (Week 1, Day 4-5)
**File**: `crates/leptos-motion-core/src/engine/property_interpolator.rs`
**Target Lines**: <100

```rust
pub fn interpolate_property(
    from: &AnimationValue,
    to: &AnimationValue,
    progress: f64,
    easing: &EasingFunction,
) -> AnimationValue
```

## File Structure
```
crates/leptos-motion-core/src/engine/
├── raf.rs                    # Main RAF engine (<200 lines)
├── transform_compositor.rs   # Transform handling (<150 lines)
├── property_interpolator.rs  # Value interpolation (<100 lines)
└── raf_scheduler.rs          # RAF scheduling logic (<100 lines)
```

## Testing Strategy
- Unit tests for each component (<50 lines each)
- Integration tests with DOM elements
- Performance benchmarks (60fps target)
- Memory leak detection
- Contract tests for public API

## Performance Requirements
- Animation setup: <0.5ms
- Frame update: <16.67ms (60fps)
- Memory per animation: <100KB
- Transform composition: <0.1ms

## Dependencies
```rust
use web_sys::{window, HtmlElement, Performance};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
```

## Success Criteria
- [ ] Real RAF loop running at 60fps
- [ ] Multiple transforms on same element work correctly
- [ ] Smooth interpolation between values
- [ ] No memory leaks
- [ ] Contract tests pass
- [ ] All files under 200 lines
