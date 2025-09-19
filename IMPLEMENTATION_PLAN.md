# Animation Engine Implementation Plan

## Overview
This plan implements the event-driven, callback-based animation architecture described in `ANIMATION_ENGINE_DESIGN.md`.

## Phase 1: Core Infrastructure (Week 1)

### Day 1-2: AnimationManager and Base Traits
**Files to create:**
- `crates/leptos-motion-dom/src/animation_manager.rs`
- `crates/leptos-motion-dom/src/animation_trait.rs`
- `crates/leptos-motion-dom/src/animation_handle.rs`

**Implementation:**
```rust
// animation_manager.rs
pub struct AnimationManager {
    animations: Rc<RefCell<HashMap<String, Box<dyn Animation>>>>,
}

impl AnimationManager {
    pub fn new() -> Self { /* ... */ }
    pub fn register(&self, id: String, animation: Box<dyn Animation>) -> AnimationHandle { /* ... */ }
    pub fn unregister(&self, handle: AnimationHandle) { /* ... */ }
    pub fn get_animation(&self, id: &str) -> Option<Weak<RefCell<Box<dyn Animation>>>> { /* ... */ }
}

// animation_trait.rs
pub trait Animation: Send + Sync {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn is_complete(&self) -> bool;
    fn progress(&self) -> f64;
    fn update(&mut self, delta_time: f64) -> Result<()>;
    fn id(&self) -> &str;
}

// animation_handle.rs
pub struct AnimationHandle {
    id: String,
    manager: Weak<RefCell<AnimationManager>>,
}
```

### Day 3-4: CSS Transition Animation
**Files to create:**
- `crates/leptos-motion-dom/src/css_transition_animation.rs`

**Implementation:**
```rust
pub struct CssTransitionAnimation {
    id: String,
    element: Element,
    properties: HashMap<String, AnimationValue>,
    transition: Transition,
    start_time: Option<f64>,
    duration: f64,
    is_complete: bool,
}

impl Animation for CssTransitionAnimation {
    fn start(&mut self) -> Result<()> {
        // Apply CSS transition and target values
        // No RAF loop needed - CSS handles the animation
    }
    
    fn update(&mut self, _delta_time: f64) -> Result<()> {
        // CSS transitions don't need updates
        Ok(())
    }
}
```

### Day 5: Integration and Testing
**Files to update:**
- `crates/leptos-motion-dom/src/lib.rs`
- `examples/phase3-clean-demo/src/lib.rs`

**Tasks:**
- Export new components
- Create basic demo
- Write unit tests
- Fix compilation issues

## Phase 2: Advanced Animations (Week 2)

### Day 1-2: JavaScript Animation
**Files to create:**
- `crates/leptos-motion-dom/src/javascript_animation.rs`

**Implementation:**
```rust
pub struct JavaScriptAnimation {
    id: String,
    element: Element,
    properties: HashMap<String, AnimationValue>,
    transition: Transition,
    start_time: Option<f64>,
    duration: f64,
    is_complete: bool,
    on_complete: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl Animation for JavaScriptAnimation {
    fn start(&mut self) -> Result<()> {
        // Use CSS transitions for simple properties
        // Use JavaScript for complex interpolations
    }
    
    fn update(&mut self, delta_time: f64) -> Result<()> {
        // Update complex animations that CSS can't handle
    }
}
```

### Day 3-4: Keyframe Animation
**Files to create:**
- `crates/leptos-motion-dom/src/keyframe_animation.rs`

**Implementation:**
```rust
pub struct KeyframeAnimation {
    id: String,
    element: Element,
    keyframes: Vec<Keyframe>,
    current_keyframe: usize,
    start_time: Option<f64>,
    duration: f64,
    is_complete: bool,
}

pub struct Keyframe {
    time: f64, // 0.0 to 1.0
    properties: HashMap<String, AnimationValue>,
    easing: Easing,
}
```

### Day 5: Stagger Animation
**Files to create:**
- `crates/leptos-motion-dom/src/stagger_animation.rs`

**Implementation:**
```rust
pub struct StaggerAnimation {
    id: String,
    elements: Vec<Element>,
    properties: HashMap<String, AnimationValue>,
    transition: Transition,
    stagger_delay: f64,
    start_time: Option<f64>,
    is_complete: bool,
}
```

## Phase 3: Integration (Week 3)

### Day 1-2: MotionDiv Integration
**Files to update:**
- `crates/leptos-motion-dom/src/motion_div.rs`

**Implementation:**
```rust
#[component]
pub fn MotionDiv(
    // ... props
) -> impl IntoView {
    let animation_manager = use_context::<AnimationManager>();
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);
    
    // Create animation based on current state
    let current_animation = move || {
        if is_tapped.get() {
            while_tap.clone().unwrap_or_default()
        } else if is_hovered.get() {
            while_hover.clone().unwrap_or_default()
        } else {
            animate.clone().unwrap_or_default()
        }
    };
    
    // Apply animation when state changes
    Effect::new(move |_| {
        let target = current_animation();
        if !target.is_empty() {
            if let Some(element) = node_ref.get() {
                let animation = CssTransitionAnimation::new(
                    element,
                    target,
                    transition.clone().unwrap_or_default(),
                );
                let handle = animation_manager.register(animation);
                // Handle will be dropped when component unmounts
            }
        }
    });
    
    view! { /* ... */ }
}
```

### Day 3-4: Event Handling
**Files to create:**
- `crates/leptos-motion-dom/src/event_handlers.rs`

**Implementation:**
```rust
pub struct EventHandlers {
    on_hover: Option<Box<dyn Fn() + Send + Sync>>,
    on_tap: Option<Box<dyn Fn() + Send + Sync>>,
    on_drag: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
}

impl EventHandlers {
    pub fn handle_hover(&self) { /* ... */ }
    pub fn handle_tap(&self) { /* ... */ }
    pub fn handle_drag(&self, x: f64, y: f64) { /* ... */ }
}
```

### Day 5: Performance Optimization
**Files to create:**
- `crates/leptos-motion-dom/src/performance.rs`

**Implementation:**
```rust
pub struct PerformanceMonitor {
    fps_counter: FpsCounter,
    memory_usage: MemoryUsage,
    animation_count: usize,
}

impl PerformanceMonitor {
    pub fn track_animation_start(&mut self) { /* ... */ }
    pub fn track_animation_end(&mut self) { /* ... */ }
    pub fn get_stats(&self) -> PerformanceStats { /* ... */ }
}
```

## Phase 4: Advanced Features (Week 4)

### Day 1-2: Layout Animations
**Files to create:**
- `crates/leptos-motion-dom/src/layout_animation.rs`

**Implementation:**
```rust
pub struct LayoutAnimation {
    id: String,
    element: Element,
    from_layout: LayoutInfo,
    to_layout: LayoutInfo,
    transition: Transition,
    start_time: Option<f64>,
    is_complete: bool,
}

pub struct LayoutInfo {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
```

### Day 3-4: Gesture Recognition
**Files to create:**
- `crates/leptos-motion-dom/src/gesture_recognition.rs`

**Implementation:**
```rust
pub struct GestureRecognizer {
    element: Element,
    on_drag: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
    on_pinch: Option<Box<dyn Fn(f64) + Send + Sync>>,
    on_rotate: Option<Box<dyn Fn(f64) + Send + Sync>>,
}

impl GestureRecognizer {
    pub fn handle_touch_start(&mut self, event: TouchEvent) { /* ... */ }
    pub fn handle_touch_move(&mut self, event: TouchEvent) { /* ... */ }
    pub fn handle_touch_end(&mut self, event: TouchEvent) { /* ... */ }
}
```

### Day 5: Animation Sequences
**Files to create:**
- `crates/leptos-motion-dom/src/animation_sequence.rs`

**Implementation:**
```rust
pub struct AnimationSequence {
    id: String,
    animations: Vec<Box<dyn Animation>>,
    current_animation: usize,
    is_complete: bool,
}

impl Animation for AnimationSequence {
    fn start(&mut self) -> Result<()> {
        // Start first animation
    }
    
    fn update(&mut self, delta_time: f64) -> Result<()> {
        // Update current animation, start next when complete
    }
}
```

## Testing Strategy

### Unit Tests
- **AnimationManager**: Test registration, unregistration, cleanup
- **Animation Trait**: Test all animation types
- **Event Handlers**: Test event handling logic
- **Performance**: Test performance monitoring

### Integration Tests
- **MotionDiv**: Test component integration
- **Animation Sequences**: Test complex animation chains
- **Gesture Recognition**: Test touch/gesture handling
- **Layout Animations**: Test layout change animations

### Performance Tests
- **Memory Usage**: Test for memory leaks
- **FPS**: Test animation smoothness
- **CPU Usage**: Test performance impact
- **Battery Life**: Test on mobile devices

## Migration Strategy

### Step 1: Create New Architecture
1. Implement `AnimationManager` and `Animation` trait
2. Create `CssTransitionAnimation` for simple cases
3. Add `AnimationHandle` for control
4. Write unit tests

### Step 2: Update MotionDiv
1. Replace complex animation engines with `AnimationManager`
2. Use CSS transitions for simple properties
3. Use JavaScript animations for complex cases
4. Update demos to use new API

### Step 3: Remove Old Code
1. Delete broken animation engines
2. Remove RAF-based implementations
3. Clean up unused imports and dependencies
4. Update documentation

### Step 4: Add Advanced Features
1. Implement keyframe animations
2. Add stagger animations
3. Create layout animations
4. Add gesture recognition

## Success Criteria

### Technical Criteria
- ✅ Zero compilation errors
- ✅ Zero memory leaks
- ✅ < 5ms animation setup time
- ✅ 60fps smooth animations
- ✅ < 1MB memory usage

### Code Quality Criteria
- ✅ 100% test coverage for core components
- ✅ Zero clippy warnings
- ✅ Clear documentation
- ✅ Consistent API design

### User Experience Criteria
- ✅ Smooth animations on all devices
- ✅ Responsive to user interactions
- ✅ Consistent behavior across browsers
- ✅ Easy to use API

## Risk Mitigation

### Technical Risks
- **Risk**: CSS transitions not supported on all browsers
- **Mitigation**: Fallback to JavaScript animations

- **Risk**: Performance issues with complex animations
- **Mitigation**: Performance monitoring and optimization

- **Risk**: Memory leaks from event listeners
- **Mitigation**: Proper cleanup and weak references

### Timeline Risks
- **Risk**: Implementation takes longer than expected
- **Mitigation**: Prioritize core features, defer advanced features

- **Risk**: Integration issues with existing code
- **Mitigation**: Gradual migration, maintain backward compatibility

## Conclusion

This implementation plan provides a clear roadmap for building a robust, performant animation system that:

1. **Solves borrowing issues** with event-driven architecture
2. **Improves performance** with CSS transitions
3. **Prevents memory leaks** with proper cleanup
4. **Simplifies testing** with clear interfaces
5. **Follows Rust best practices** with proper ownership

The key is to implement incrementally, test thoroughly, and maintain backward compatibility during the migration.
