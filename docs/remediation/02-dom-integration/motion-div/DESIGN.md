# MotionDiv Component Design Document

## Overview

MotionDiv is the primary user-facing component in Leptos Motion, providing a declarative API for animating HTML div elements. It combines the Animation Engine with reactive Leptos signals to create smooth, performant animations that integrate seamlessly with the Leptos ecosystem.

## Component Architecture

### Core Structure

```rust
#[component]
pub fn MotionDiv(
    // Animation Properties
    animate: Option<AnimateProp>,
    initial: Option<HashMap<String, AnimationValue>>,
    exit: Option<HashMap<String, AnimationValue>>,
    transition: Option<Transition>,

    // Gesture Properties
    while_hover: Option<HashMap<String, AnimationValue>>,
    while_tap: Option<HashMap<String, AnimationValue>>,
    while_drag: Option<HashMap<String, AnimationValue>>,

    // Layout Properties
    layout: Option<bool>,
    layout_id: Option<String>,

    // DOM Properties
    node_ref: Option<NodeRef>,
    class: Option<String>,
    style: Option<String>,
    children: Children,

    // Event Handlers
    on_hover_start: Option<Box<dyn Fn()>>,
    on_hover_end: Option<Box<dyn Fn()>>,
    on_animation_start: Option<Box<dyn Fn(String)>>,
    on_animation_complete: Option<Box<dyn Fn(String)>>,
) -> impl IntoView
```

### Internal Architecture

```
MotionDiv
├── PropsParser           # Parse and validate component props
├── StateManager          # Manage component reactive state
├── AnimationCoordinator  # Coordinate multiple animations
├── GestureHandler        # Handle user gesture events
├── LayoutObserver        # Observe layout changes (future)
├── DOMUpdater            # Update DOM with animation values
└── CleanupHandler        # Handle component cleanup
```

## Props System

### Animation Props

#### `animate: AnimateProp`
Defines the target animation state for the component.

```rust
pub enum AnimateProp {
    Static(HashMap<String, AnimationValue>),      // Static animation values
    Reactive(ReadSignal<HashMap<String, AnimationValue>>), // Reactive animation values
    Derived(Memo<HashMap<String, AnimationValue>>),        // Derived from other signals
    Fn(Rc<dyn Fn() -> HashMap<String, AnimationValue>>),   // Function-based animation
}
```

#### `initial: HashMap<String, AnimationValue>`
Defines the initial animation state before any animations run.

```rust
let initial = HashMap::from([
    ("opacity".to_string(), AnimationValue::Number(0.0)),
    ("scale".to_string(), AnimationValue::Number(0.8)),
]);
```

#### `transition: Transition`
Configures the timing and easing for animations.

```rust
#[derive(Clone, Debug)]
pub struct Transition {
    pub duration: Option<f64>,           // Animation duration in seconds
    pub delay: Option<f64>,              // Animation delay in seconds
    pub easing: Option<Easing>,          // Easing function
    pub repeat: Option<Repeat>,          // Repeat configuration
    pub repeat_type: Option<RepeatType>, // Repeat behavior
    pub repeat_delay: Option<f64>,       // Delay between repeats
}
```

### Gesture Props

#### `while_hover: HashMap<String, AnimationValue>`
Animation values applied while the element is hovered.

```rust
let while_hover = HashMap::from([
    ("scale".to_string(), AnimationValue::Number(1.05)),
    ("box-shadow".to_string(), AnimationValue::String("0 10px 25px rgba(0,0,0,0.2)".to_string())),
]);
```

#### `while_tap: HashMap<String, AnimationValue>`
Animation values applied while the element is being tapped/pressed.

#### `while_drag: HashMap<String, AnimationValue>`
Animation values applied while the element is being dragged.

### Layout Props

#### `layout: bool`
Enables layout animations when the element's size or position changes.

#### `layout_id: String`
Unique identifier for shared layout animations across components.

## State Management

### Reactive State

```rust
#[derive(Clone)]
struct MotionDivState {
    // Animation state
    current_values: ReadSignal<HashMap<String, AnimationValue>>,
    target_values: ReadSignal<HashMap<String, AnimationValue>>,

    // Gesture state
    is_hovered: ReadSignal<bool>,
    is_tapped: ReadSignal<bool>,
    is_dragging: ReadSignal<bool>,

    // Animation handles
    active_animations: HashMap<String, AnimationHandle>,

    // Layout state
    layout_values: ReadSignal<LayoutValues>,
}
```

### State Updates

```rust
// Update animation targets reactively
create_effect(move |_| {
    let animate_values = animate.get();
    state.update_target_values(animate_values);
});

// Handle gesture state changes
create_effect(move |_| {
    let is_hovered = is_hovered.get();
    if is_hovered {
        state.apply_gesture_values(while_hover.clone());
    } else {
        state.restore_base_values();
    }
});
```

## Animation Coordination

### Animation Lifecycle

1. **Initialization**: Parse props and create initial state
2. **Mounting**: Set up DOM references and event listeners
3. **Animation Start**: Create animation targets and start execution
4. **Animation Update**: Reactively update animations based on prop changes
5. **Animation Complete**: Handle completion callbacks and cleanup
6. **Unmounting**: Clean up animations and event listeners

### Multiple Animation Management

```rust
struct AnimationCoordinator {
    active_animations: HashMap<String, AnimationHandle>,
    pending_animations: Vec<AnimationRequest>,
    animation_queue: VecDeque<AnimationRequest>,
}

impl AnimationCoordinator {
    // Start new animation
    fn start_animation(&mut self, property: String, target: AnimationTarget) {
        // Cancel conflicting animations
        self.cancel_property_animations(&property);

        // Start new animation
        let handle = self.animation_engine.animate(target);
        self.active_animations.insert(property, handle);
    }

    // Cancel animations for property
    fn cancel_property_animations(&mut self, property: &str) {
        if let Some(handle) = self.active_animations.remove(property) {
            handle.cancel();
        }
    }
}
```

## Gesture Handling

### Gesture Detection

```rust
struct GestureHandler {
    hover_detector: HoverDetector,
    tap_detector: TapDetector,
    drag_detector: DragDetector,
}

impl GestureHandler {
    fn setup_gesture_listeners(&self, element: &web_sys::Element) {
        // Mouse events
        self.add_mouse_listeners(element);

        // Touch events
        self.add_touch_listeners(element);

        // Pointer events (future)
        self.add_pointer_listeners(element);
    }
}
```

### Gesture State Management

```rust
// Gesture state signals
let (is_hovered, set_is_hovered) = signal(false);
let (is_tapped, set_is_tapped) = signal(false);
let (is_dragging, set_is_dragging) = signal(false);

// Update gesture animations reactively
create_effect(move |_| {
    let gestures = HashMap::new();

    if is_hovered.get() {
        gestures.extend(while_hover.clone());
    }

    if is_tapped.get() {
        gestures.extend(while_tap.clone());
    }

    if is_dragging.get() {
        gestures.extend(while_drag.clone());
    }

    state.apply_gesture_overrides(gestures);
});
```

## DOM Integration

### DOM Updates

```rust
struct DOMUpdater {
    element: web_sys::Element,
    current_styles: HashMap<String, String>,
}

impl DOMUpdater {
    fn update_property(&mut self, property: &str, value: &AnimationValue) {
        match property {
            // CSS properties
            "opacity" | "transform" | "color" => {
                self.update_css_property(property, value);
            }

            // Style properties
            prop if prop.starts_with("--") => {
                self.update_css_variable(prop, value);
            }

            // Attribute properties
            _ => {
                self.update_dom_attribute(property, value);
            }
        }
    }

    fn update_css_property(&mut self, property: &str, value: &AnimationValue) {
        let css_value = value.to_css_string();
        self.element.style().set_property(property, &css_value).unwrap();
    }
}
```

### CSS Value Conversion

```rust
impl AnimationValue {
    fn to_css_string(&self) -> String {
        match self {
            Number(n) => n.to_string(),
            Pixels(px) => format!("{}px", px),
            Degrees(deg) => format!("{}deg", deg),
            Color(c) => c.to_css_string(),
            Transform(t) => t.to_css_string(),
            String(s) => s.clone(),
        }
    }
}
```

## Performance Optimizations

### 1. Animation Batching
- Batch DOM updates to minimize reflows
- Use `requestAnimationFrame` for coordinated updates

### 2. Memory Management
- Reuse animation objects and handles
- Automatic cleanup of completed animations
- Weak references to prevent memory leaks

### 3. Reactivity Optimization
- Debounce rapid prop changes
- Memoize expensive calculations
- Selective re-renders based on changed props

### 4. DOM Optimization
- Minimize style recalculations
- Use CSS transforms for hardware acceleration
- Avoid layout-triggering properties when possible

## Error Handling

### Prop Validation

```rust
fn validate_props(props: &MotionDivProps) -> Result<(), MotionDivError> {
    // Validate animation values
    if let Some(initial) = &props.initial {
        for (prop, value) in initial {
            validate_property_value(prop, value)?;
        }
    }

    // Validate transition configuration
    if let Some(transition) = &props.transition {
        validate_transition_config(transition)?;
    }

    Ok(())
}
```

### Runtime Error Handling

```rust
// Graceful degradation on animation failures
match animation_engine.animate(target).await {
    Ok(handle) => {
        self.active_animations.insert(property, handle);
    }
    Err(error) => {
        log::warn!("Animation failed for property {}: {:?}", property, error);
        // Continue with static value application
        self.apply_static_value(property, target_value);
    }
}
```

## Testing Strategy

### Component Tests
- Prop parsing and validation
- Animation state management
- Gesture event handling
- DOM updates and cleanup

### Integration Tests
- End-to-end animation execution
- Reactive prop updates
- Multiple animation coordination
- Memory leak prevention

### Performance Tests
- Animation frame rate consistency
- Memory usage during animations
- DOM update batching efficiency

## Browser Compatibility

### Supported Features
- **Modern Browsers**: Full feature support
- **Legacy Browsers**: Graceful degradation to static styles
- **Mobile Browsers**: Touch gesture support
- **Accessibility**: Keyboard navigation and screen reader support

## Future Extensions

### Planned Features
- **Layout Animations**: Size and position change animations
- **Shared Layout**: Cross-component layout transitions
- **Projection**: Advanced 3D layout animations
- **Scroll Animations**: Scroll-triggered animations

### Advanced Gestures
- **Drag Constraints**: Boundary and snapping constraints
- **Momentum**: Physics-based momentum continuation
- **Multi-touch**: Multi-finger gesture support

---

*This design document provides the architectural foundation for MotionDiv. Implementation details may evolve based on performance requirements and user feedback.*
