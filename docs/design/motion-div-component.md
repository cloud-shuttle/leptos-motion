# MotionDiv Component Design

## Overview
**Purpose**: Declarative animation component for Leptos  
**Status**: Core component, currently broken  
**Complexity**: High (event handling, state management)  
**Lines**: Target <300 lines total across modules

## Architecture

### Core Structure
```rust
#[component]
pub fn MotionDiv(
    /// Animation configuration
    #[prop(into, optional)]
    animate: Option<AnimateProp>,

    /// Initial animation state
    #[prop(into, optional)]
    initial: Option<AnimateProp>,

    /// Exit animation state
    #[prop(into, optional)]
    exit: Option<AnimateProp>,

    /// Hover animation state
    #[prop(into, optional)]
    while_hover: Option<AnimateProp>,

    /// Tap animation state
    #[prop(into, optional)]
    while_tap: Option<AnimateProp>,

    /// Drag animation state
    #[prop(into, optional)]
    while_drag: Option<AnimateProp>,

    /// Transition configuration
    #[prop(into, optional)]
    transition: Option<Transition>,

    /// Drag configuration
    #[prop(into, optional)]
    drag: Option<DragConfig>,

    /// Layout animation control
    #[prop(into, optional)]
    layout: Option<bool>,

    /// CSS classes
    #[prop(into, optional)]
    class: Option<String>,

    /// Inline styles
    #[prop(into, optional)]
    style: Option<String>,

    /// Child content
    children: Children,

    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<html::Div>>,
) -> impl IntoView
```

### Module Structure
```
motion_div/
├── lib.rs          (<100 lines) - Main component
├── props.rs        (<150 lines) - Property definitions
├── state.rs        (<150 lines) - Internal state management
├── handlers.rs     (<200 lines) - Event handlers
├── renderer.rs     (<150 lines) - DOM rendering logic
└── animations.rs   (<200 lines) - Animation orchestration
```

## State Management

### Internal State
```rust
#[derive(Clone)]
pub struct MotionDivState {
    /// Current animation values
    current_values: HashMap<String, AnimationValue>,

    /// Target animation values
    target_values: HashMap<String, AnimationValue>,

    /// Animation handles
    active_animations: HashMap<String, AnimationHandle>,

    /// Event state
    is_hovered: bool,
    is_tapped: bool,
    is_dragging: bool,

    /// Drag state
    drag_offset: (f64, f64),
    drag_constraints: Option<DragConstraints>,
}
```

### State Updates
```rust
impl MotionDivState {
    pub fn update_from_props(&mut self, props: &MotionDivProps) {
        // Update target values based on current state
        match (&self.is_hovered, &self.is_tapped, &self.is_dragging) {
            (true, _, _) => self.apply_hover_state(props),
            (_, true, _) => self.apply_tap_state(props),
            (_, _, true) => self.apply_drag_state(props),
            _ => self.apply_base_state(props),
        }
    }

    fn apply_hover_state(&mut self, props: &MotionDivProps) {
        if let Some(while_hover) = &props.while_hover {
            self.target_values = while_hover.get_values();
        }
    }
}
```

## Animation Orchestration

### Animation Controller
```rust
pub struct AnimationController {
    engine: Box<dyn AnimationEngine>,
    element: HtmlElement,
    state: MotionDivState,
}

impl AnimationController {
    pub fn new(element: HtmlElement) -> Self {
        Self {
            engine: Box::new(HybridAnimationEngine::new()),
            element,
            state: MotionDivState::default(),
        }
    }

    pub fn animate_to(&mut self, values: HashMap<String, AnimationValue>) {
        for (property, value) in values {
            let handle = self.engine.animate_property(
                &self.element,
                &property,
                value,
                Transition::default(),
            );
            self.state.active_animations.insert(property, handle);
        }
    }
}
```

### Property Animation
```rust
impl AnimationController {
    fn animate_transform(&mut self, transform: &Transform) {
        let css_value = format!(
            "translate({}px, {}px) rotate({}deg) scale({}, {})",
            transform.x, transform.y, transform.rotation,
            transform.scale_x, transform.scale_y
        );

        self.engine.animate_property(
            &self.element,
            "transform",
            AnimationValue::String(css_value),
            self.transition.clone(),
        );
    }

    fn animate_opacity(&mut self, opacity: f64) {
        self.engine.animate_property(
            &self.element,
            "opacity",
            AnimationValue::Number(opacity),
            self.transition.clone(),
        );
    }
}
```

## Event Handling

### Event Handler Types
```rust
pub struct EventHandlers {
    pub mouse_enter: Option<Box<dyn Fn()>>,
    pub mouse_leave: Option<Box<dyn Fn()>>,
    pub mouse_down: Option<Box<dyn Fn()>>,
    pub mouse_up: Option<Box<dyn Fn()>>,
    pub drag_start: Option<Box<dyn Fn(DragEvent)>>,
    pub drag_move: Option<Box<dyn Fn(DragEvent)>>,
    pub drag_end: Option<Box<dyn Fn(DragEvent)>>,
}
```

### Drag Implementation
```rust
impl EventHandlers {
    pub fn setup_drag_handlers(&self, element: &HtmlElement, state: &mut MotionDivState) {
        let element_clone = element.clone();
        let mut state_clone = state.clone();

        // Mouse down handler
        element.set_onmousedown(Some(Box::new(move |event| {
            state_clone.is_dragging = true;
            // Start drag tracking
        })));

        // Mouse move handler
        element.set_onmousemove(Some(Box::new(move |event| {
            if state_clone.is_dragging {
                // Update drag position
                // Apply constraints
                // Trigger drag animations
            }
        })));
    }
}
```

## Rendering Logic

### DOM Rendering
```rust
impl MotionDiv {
    fn render(&self) -> HtmlElement {
        let div = document().create_element("div").unwrap();

        // Apply classes
        if let Some(class) = &self.class {
            div.set_class_name(class);
        }

        // Apply initial styles
        self.apply_initial_styles(&div);

        // Setup event handlers
        self.setup_event_handlers(&div);

        // Render children
        for child in self.children() {
            div.append_child(&child).unwrap();
        }

        div
    }

    fn apply_initial_styles(&self, element: &HtmlElement) {
        if let Some(initial) = &self.initial {
            let styles = self.compute_styles(initial.get_values());
            element.set_attribute("style", &styles).unwrap();
        }
    }
}
```

## Performance Considerations

### Optimization Strategies
1. **Debounced Updates**: Batch style changes
2. **Virtual DOM**: Minimize actual DOM manipulations
3. **Animation Pool**: Reuse animation instances
4. **Memory Management**: Clean up event listeners

### Memory Safety
```rust
impl Drop for MotionDiv {
    fn drop(&mut self) {
        // Clean up event listeners
        self.cleanup_event_listeners();

        // Cancel active animations
        for (_, handle) in self.state.active_animations.drain() {
            self.engine.cancel_animation(handle);
        }

        // Release DOM references
        self.element = None;
    }
}
```

## Error Handling

### Animation Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum MotionDivError {
    #[error("Animation engine error: {0}")]
    AnimationError(String),

    #[error("DOM manipulation error: {0}")]
    DomError(String),

    #[error("Invalid animation value: {0}")]
    InvalidValue(String),
}
```

### Recovery Strategies
```rust
impl MotionDiv {
    fn handle_animation_error(&mut self, error: MotionDivError) {
        match error {
            MotionDivError::AnimationError(_) => {
                // Fallback to CSS transitions
                self.use_css_fallback();
            }
            MotionDivError::DomError(_) => {
                // Retry operation
                self.retry_dom_operation();
            }
            MotionDivError::InvalidValue(_) => {
                // Skip invalid animation
                warn!("Skipping invalid animation value");
            }
        }
    }
}
```

## Testing Strategy

### Unit Tests
- Property animation logic
- State transition handling
- Event handler setup

### Integration Tests
- Full component rendering
- Animation lifecycle
- Event interaction flows

### Performance Tests
- Animation frame rates
- Memory usage patterns
- DOM manipulation efficiency

## API Contract

### Component Contract
```rust
#[contract_trait]
pub trait MotionDivContract {
    fn animate(&self, values: AnimateProp) -> Result<(), MotionDivError>;
    fn set_transition(&mut self, transition: Transition);
    fn add_event_handler(&mut self, event: EventType, handler: EventHandler);
    fn render(&self) -> HtmlElement;
}
```

### Animation Contract
- Must support 60fps animations
- Must handle concurrent animations
- Must provide cancellation API
- Must support spring and tween easing

This design ensures MotionDiv is performant, maintainable, and provides a clean API for complex animations while staying under 300 lines per module.
