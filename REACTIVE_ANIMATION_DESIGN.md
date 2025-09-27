# Reactive Animation System Design Document

## Executive Summary

This document outlines the design and implementation of a reactive animation system for leptos-motion that enables MotionDiv components to respond to Leptos signal changes in real-time. The system will provide seamless integration with Leptos's reactive framework while maintaining performance and developer experience.

## Current State Analysis

### Problems with Existing MotionDiv
```rust
// Current API - Static only
pub fn EventDrivenMotionDiv(
    animate: Option<HashMap<String, AnimationValue>>,  // ❌ Static values only
    // ... other props
) -> impl IntoView
```

### What We Need
```rust
// Target API - Reactive support
pub fn ReactiveMotionDiv(
    animate: impl Fn() -> HashMap<String, AnimationValue>,  // ✅ Reactive closures
    // ... other props
) -> impl IntoView
```

## Design Goals

### 1. Reactive Integration
- MotionDiv components must respond to Leptos signal changes
- Animations should trigger automatically when signals update
- No manual intervention required from developers

### 2. Performance
- Minimal overhead for signal tracking
- Efficient animation updates
- No unnecessary re-renders

### 3. Developer Experience
- Simple, intuitive API
- Type-safe animation properties
- Clear error messages

### 4. Backward Compatibility
- Existing static animations continue to work
- Gradual migration path
- No breaking changes

## Architecture Design

### Core Components

#### 1. ReactiveMotionDiv Component
```rust
#[component]
pub fn ReactiveMotionDiv(
    /// Reactive animation values
    animate: impl Fn() -> HashMap<String, AnimationValue> + 'static,
    
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    
    /// Animation while hovering
    #[prop(optional)]
    while_hover: Option<impl Fn() -> HashMap<String, AnimationValue> + 'static>,
    
    /// Animation while tapping
    #[prop(optional)]
    while_tap: Option<impl Fn() -> HashMap<String, AnimationValue> + 'static>,
    
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    
    /// CSS classes
    #[prop(optional, default = "".to_string())]
    class: String,
    
    /// CSS styles
    #[prop(optional, default = "".to_string())]
    style: String,
    
    /// Children
    children: Children,
    
    /// Node reference
    node_ref: NodeRef<leptos::html::Div>,
) -> impl IntoView
```

#### 2. Signal Tracking System
```rust
pub struct ReactiveAnimationTracker {
    animate_fn: Box<dyn Fn() -> HashMap<String, AnimationValue>>,
    current_values: HashMap<String, AnimationValue>,
    animation_engine: Rc<RefCell<AnimationEngine>>,
    node_ref: NodeRef<leptos::html::Div>,
}

impl ReactiveAnimationTracker {
    pub fn new(
        animate_fn: impl Fn() -> HashMap<String, AnimationValue> + 'static,
        node_ref: NodeRef<leptos::html::Div>,
    ) -> Self {
        // Initialize with current signal values
        let current_values = animate_fn();
        
        Self {
            animate_fn: Box::new(animate_fn),
            current_values,
            animation_engine: get_global_animation_engine(),
            node_ref,
        }
    }
    
    pub fn update_animation(&mut self) {
        let new_values = (self.animate_fn)();
        
        // Compare with current values to detect changes
        if new_values != self.current_values {
            self.current_values = new_values;
            self.trigger_animation();
        }
    }
    
    fn trigger_animation(&self) {
        // Apply new animation values to DOM element
        if let Some(element) = self.node_ref.get() {
            self.apply_animation_to_element(&element, &self.current_values);
        }
    }
}
```

#### 3. Effect Integration
```rust
impl ReactiveMotionDiv {
    fn setup_reactive_animation(
        animate_fn: impl Fn() -> HashMap<String, AnimationValue> + 'static,
        node_ref: NodeRef<leptos::html::Div>,
        transition: Option<Transition>,
    ) {
        let mut tracker = ReactiveAnimationTracker::new(animate_fn, node_ref);
        
        // Set up Leptos effect to track signal changes
        Effect::new(move |_| {
            // This effect will re-run when any signal accessed in animate_fn changes
            tracker.update_animation();
        });
    }
}
```

## Implementation Plan

### Phase 1: Core Reactive System

#### 1.1 Create ReactiveAnimationTracker
```rust
// File: crates/leptos-motion-dom/src/reactive_animation_tracker.rs
pub struct ReactiveAnimationTracker {
    animate_fn: Box<dyn Fn() -> HashMap<String, AnimationValue>>,
    current_values: HashMap<String, AnimationValue>,
    animation_engine: Rc<RefCell<AnimationEngine>>,
    node_ref: NodeRef<leptos::html::Div>,
    transition: Option<Transition>,
}

impl ReactiveAnimationTracker {
    pub fn new(
        animate_fn: impl Fn() -> HashMap<String, AnimationValue> + 'static,
        node_ref: NodeRef<leptos::html::Div>,
        transition: Option<Transition>,
    ) -> Self {
        let current_values = animate_fn();
        
        Self {
            animate_fn: Box::new(animate_fn),
            current_values,
            animation_engine: get_global_animation_engine(),
            node_ref,
            transition,
        }
    }
    
    pub fn update_animation(&mut self) {
        let new_values = (self.animate_fn)();
        
        if new_values != self.current_values {
            self.current_values = new_values;
            self.trigger_animation();
        }
    }
    
    fn trigger_animation(&self) {
        if let Some(element) = self.node_ref.get() {
            self.apply_animation_to_element(&element, &self.current_values);
        }
    }
    
    fn apply_animation_to_element(
        &self,
        element: &web_sys::Element,
        values: &HashMap<String, AnimationValue>,
    ) {
        // Apply CSS transitions or use animation engine
        for (property, value) in values {
            self.set_css_property(element, property, value);
        }
    }
    
    fn set_css_property(
        &self,
        element: &web_sys::Element,
        property: &str,
        value: &AnimationValue,
    ) {
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            let css_value = animation_value_to_css(value);
            let _ = html_element.style().set_property(property, &css_value);
        }
    }
}
```

#### 1.2 Create ReactiveMotionDiv Component
```rust
// File: crates/leptos-motion-dom/src/reactive_motion_div.rs
#[component]
pub fn ReactiveMotionDiv(
    animate: impl Fn() -> HashMap<String, AnimationValue> + 'static,
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    #[prop(optional)]
    while_hover: Option<impl Fn() -> HashMap<String, AnimationValue> + 'static>,
    #[prop(optional)]
    while_tap: Option<impl Fn() -> HashMap<String, AnimationValue> + 'static>,
    #[prop(optional)]
    transition: Option<Transition>,
    #[prop(optional, default = "".to_string())]
    class: String,
    #[prop(optional, default = "".to_string())]
    style: String,
    children: Children,
    node_ref: NodeRef<leptos::html::Div>,
) -> impl IntoView {
    // Set up reactive animation tracking
    let tracker = Rc::new(RefCell::new(ReactiveAnimationTracker::new(
        animate,
        node_ref.clone(),
        transition,
    )));
    
    // Set up Leptos effect for signal tracking
    Effect::new(move |_| {
        if let Ok(mut tracker) = tracker.try_borrow_mut() {
            tracker.update_animation();
        }
    });
    
    // Apply initial styles
    if let Some(initial_values) = initial {
        Effect::new(move |_| {
            if let Some(element) = node_ref.get() {
                apply_initial_styles(&element, &initial_values);
            }
        });
    }
    
    // Set up hover and tap handlers
    let hover_handler = while_hover.map(|hover_fn| {
        let tracker = tracker.clone();
        move |_| {
            if let Ok(mut tracker) = tracker.try_borrow_mut() {
                let hover_values = hover_fn();
                // Apply hover animation
            }
        }
    });
    
    let tap_handler = while_tap.map(|tap_fn| {
        let tracker = tracker.clone();
        move |_| {
            if let Ok(mut tracker) = tracker.try_borrow_mut() {
                let tap_values = tap_fn();
                // Apply tap animation
            }
        }
    });
    
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            on:mouseenter=hover_handler
            on:click=tap_handler
        >
            {children()}
        </div>
    }
}
```

### Phase 2: Enhanced Features

#### 2.1 Gesture Support
```rust
pub fn ReactiveMotionDiv(
    // ... existing props
    #[prop(optional)]
    while_drag: Option<impl Fn() -> HashMap<String, AnimationValue> + 'static>,
    #[prop(optional)]
    drag: Option<DragConfig>,
) -> impl IntoView {
    // Implement drag gesture handling
}
```

#### 2.2 Layout Animations
```rust
pub fn ReactiveMotionDiv(
    // ... existing props
    #[prop(optional, default = false)]
    layout: bool,
) -> impl IntoView {
    // Implement layout animation support
}
```

#### 2.3 Spring Physics
```rust
pub fn ReactiveMotionDiv(
    // ... existing props
    #[prop(optional)]
    spring_config: Option<SpringConfig>,
) -> impl IntoView {
    // Implement spring physics animations
}
```

### Phase 3: Performance Optimizations

#### 3.1 Signal Change Detection
```rust
impl ReactiveAnimationTracker {
    fn has_values_changed(&self, new_values: &HashMap<String, AnimationValue>) -> bool {
        // Efficient comparison to avoid unnecessary updates
        if self.current_values.len() != new_values.len() {
            return true;
        }
        
        for (key, value) in new_values {
            if let Some(current_value) = self.current_values.get(key) {
                if !values_equal(current_value, value) {
                    return true;
                }
            } else {
                return true;
            }
        }
        
        false
    }
}
```

#### 3.2 Animation Batching
```rust
pub struct AnimationBatcher {
    pending_updates: Vec<AnimationUpdate>,
    batch_timer: Option<u32>,
}

impl AnimationBatcher {
    pub fn schedule_update(&mut self, update: AnimationUpdate) {
        self.pending_updates.push(update);
        
        if self.batch_timer.is_none() {
            self.batch_timer = Some(request_animation_frame(|| {
                self.process_batch();
            }));
        }
    }
    
    fn process_batch(&mut self) {
        // Process all pending updates in a single frame
        for update in self.pending_updates.drain(..) {
            update.apply();
        }
        self.batch_timer = None;
    }
}
```

## API Design

### Basic Usage
```rust
let (is_animated, set_animated) = signal(false);

view! {
    <ReactiveMotionDiv
        animate=move || {
            let mut target = HashMap::new();
            if is_animated.get() {
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert("scale".to_string(), AnimationValue::Number(1.2));
            } else {
                target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                target.insert("scale".to_string(), AnimationValue::Number(0.8));
            }
            target
        }
        transition=Transition {
            duration: Some(0.3),
            ease: Easing::EaseOut,
            repeat: RepeatConfig::Never,
            delay: Some(0.0),
        }
    >
        "Animated Content"
    </ReactiveMotionDiv>
}
```

### Advanced Usage with Gestures
```rust
let (is_dragging, set_dragging) = signal(false);
let (drag_position, set_drag_position) = signal((0.0, 0.0));

view! {
    <ReactiveMotionDiv
        animate=move || {
            let mut target = HashMap::new();
            let (x, y) = drag_position.get();
            target.insert("x".to_string(), AnimationValue::Pixels(x));
            target.insert("y".to_string(), AnimationValue::Pixels(y));
            target
        }
        while_drag=move || {
            let mut target = HashMap::new();
            if is_dragging.get() {
                target.insert("scale".to_string(), AnimationValue::Number(1.1));
                target.insert("rotateZ".to_string(), AnimationValue::Degrees(5.0));
            }
            target
        }
        drag=DragConfig {
            axis: Some(DragAxis::Both),
            momentum: Some(true),
            elastic: Some(0.2),
            constraints: None,
        }
    >
        "Draggable Content"
    </ReactiveMotionDiv>
}
```

## Migration Strategy

### Step 1: Add ReactiveMotionDiv
- Create new component alongside existing MotionDiv
- No breaking changes to existing code
- Gradual adoption

### Step 2: Update Demos
- Convert existing demos to use ReactiveMotionDiv
- Demonstrate reactive capabilities
- Provide migration examples

### Step 3: Deprecate Static API
- Mark static MotionDiv as deprecated
- Provide migration guide
- Remove in future version

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reactive_animation_updates() {
        let (signal, set_signal) = signal(false);
        
        let animate_fn = move || {
            let mut target = HashMap::new();
            target.insert("opacity".to_string(), 
                AnimationValue::Number(if signal.get() { 1.0 } else { 0.0 }));
            target
        };
        
        let tracker = ReactiveAnimationTracker::new(animate_fn, NodeRef::new(), None);
        
        // Initial state
        assert_eq!(tracker.current_values.get("opacity"), 
            Some(&AnimationValue::Number(0.0)));
        
        // Update signal
        set_signal.set(true);
        tracker.update_animation();
        
        // Should update animation values
        assert_eq!(tracker.current_values.get("opacity"), 
            Some(&AnimationValue::Number(1.0)));
    }
}
```

### Integration Tests
```rust
#[wasm_bindgen_test]
fn test_reactive_motion_div_integration() {
    let (is_visible, set_visible) = signal(false);
    
    let component = view! {
        <ReactiveMotionDiv
            animate=move || {
                let mut target = HashMap::new();
                target.insert("opacity".to_string(), 
                    AnimationValue::Number(if is_visible.get() { 1.0 } else { 0.0 }));
                target
            }
        >
            "Test Content"
        </ReactiveMotionDiv>
    };
    
    mount_to_body(component);
    
    // Initial state
    let initial_opacity = get_computed_style("opacity");
    assert_eq!(initial_opacity, "0");
    
    // Update signal
    set_visible.set(true);
    
    // Should trigger animation
    let updated_opacity = get_computed_style("opacity");
    assert_eq!(updated_opacity, "1");
}
```

## Performance Considerations

### Signal Tracking Overhead
- Minimal overhead for signal tracking
- Only update when values actually change
- Efficient comparison algorithms

### Animation Performance
- Use CSS transitions when possible
- Batch multiple property updates
- Avoid unnecessary DOM manipulations

### Memory Management
- Proper cleanup of animation resources
- Weak references to prevent memory leaks
- Efficient data structures

## Conclusion

This design provides a comprehensive solution for adding reactive animation support to leptos-motion. The implementation will:

1. **Enable reactive animations** that respond to Leptos signal changes
2. **Maintain performance** with efficient signal tracking and animation updates
3. **Provide excellent developer experience** with a simple, intuitive API
4. **Ensure backward compatibility** with existing static animations
5. **Support advanced features** like gestures, layout animations, and spring physics

The phased implementation approach ensures a smooth transition while delivering immediate value to developers using leptos-motion in reactive applications.
