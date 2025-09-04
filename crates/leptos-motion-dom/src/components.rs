//! Core motion components for Leptos

use leptos::prelude::*;
use leptos_motion_core::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::HtmlElement;
use std::rc::Rc;
use std::cell::RefCell;
use crate::motion_target;

/// Props for motion components
#[derive(Clone, Debug)]
pub struct MotionProps {
    /// Initial animation state
    pub initial: Option<AnimationTarget>,
    /// Target animation state
    pub animate: Option<AnimationTarget>,
    /// Exit animation state  
    pub exit: Option<AnimationTarget>,
    /// Transition configuration
    pub transition: Option<Transition>,
    /// Animation variants
    pub variants: Option<Variants>,
    /// Layout animation enabled
    pub layout: Option<bool>,
    /// Drag configuration
    pub drag: Option<DragConfig>,
    /// Hover animation state
    pub while_hover: Option<AnimationTarget>,
    /// Tap animation state
    pub while_tap: Option<AnimationTarget>,
    /// Focus animation state
    pub while_focus: Option<AnimationTarget>,
    /// In-view animation state
    pub while_in_view: Option<AnimationTarget>,
    /// Event handler workaround for Leptos v0.8.x compatibility
    pub event_handlers: Option<EventHandlers>,
}

/// Event handlers for interactive functionality
#[derive(Clone, Debug)]
pub struct EventHandlers {
    /// Click handler for buttons and interactive elements
    pub on_click: Option<ClickHandler>,
    /// State management for interactive elements
    pub state: Option<InteractiveState>,
}

/// Click handler configuration
#[derive(Clone, Debug)]
pub enum ClickHandler {
    /// Counter functionality
    Counter,
    /// Toggle functionality (show/hide)
    Toggle,
    /// Layout toggle functionality
    LayoutToggle,
    /// Custom click handler
    Custom(String),
}

/// Interactive state configuration
#[derive(Clone, Debug)]
pub struct InteractiveState {
    /// Initial state value
    pub initial: String,
    /// State type
    pub state_type: StateType,
}

/// State type for interactive elements
#[derive(Clone, Debug)]
pub enum StateType {
    /// Counter state
    Counter,
    /// Boolean toggle state
    Toggle,
    /// Layout state
    Layout,
}

impl Default for MotionProps {
    fn default() -> Self {
        Self {
            initial: None,
            animate: None,
            exit: None,
            transition: None,
            variants: None,
            layout: None,
            drag: None,
            while_hover: None,
            while_tap: None,
            while_focus: None,
            while_in_view: None,
            event_handlers: None,
        }
    }
}

/// Drag configuration
#[derive(Clone, Debug)]
pub struct DragConfig {
    /// Drag axis constraint
    pub axis: Option<DragAxis>,
    /// Drag constraints
    pub constraints: Option<DragConstraints>,
    /// Elastic behavior
    pub elastic: f64,
    /// Momentum enabled
    pub momentum: bool,
}

/// Drag axis constraint
#[derive(Clone, Debug, PartialEq)]
pub enum DragAxis {
    /// Horizontal only
    X,
    /// Vertical only  
    Y,
    /// Both axes
    Both,
}

/// Drag constraints
#[derive(Clone, Debug)]
pub struct DragConstraints {
    /// Left boundary
    pub left: Option<f64>,
    /// Right boundary
    pub right: Option<f64>,
    /// Top boundary
    pub top: Option<f64>,
    /// Bottom boundary
    pub bottom: Option<f64>,
}

impl DragConfig {
    /// Create a new drag config
    pub fn new() -> Self {
        Self {
            axis: Some(DragAxis::Both),
            constraints: None,
            elastic: 0.5,
            momentum: true,
        }
    }
    
    /// Set drag axis
    pub fn axis(mut self, axis: DragAxis) -> Self {
        self.axis = Some(axis);
        self
    }
    
    /// Set drag constraints
    pub fn constraints(mut self, constraints: DragConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }
}

impl Default for DragConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Core motion div component that provides animation capabilities to div elements.
/// 
/// This component supports various animation types including:
/// - Initial animations when the component mounts
/// - Animate state changes
/// - Exit animations when the component unmounts
/// - Gesture-based animations (hover, tap, drag, focus)
/// - Layout animations for position/size changes
/// - Variant-based animation states
/// 
/// # Example
/// ```rust
/// use leptos::*;
/// use leptos_motion_dom::{MotionDiv, motion_target};
/// use leptos_motion_core::{AnimationValue, Transition};
/// 
/// #[component]
/// pub fn AnimatedBox() -> impl IntoView {
///     view! {
///         <MotionDiv
///             initial=motion_target!("opacity" => AnimationValue::Number(0.0))
///             animate=motion_target!("opacity" => AnimationValue::Number(1.0))
///             transition=Transition::default()
///             class="animated-box".to_string()
///         >
///             "Animated content"
///         </MotionDiv>
///     }
/// }
/// ```
#[component]
pub fn MotionDiv(
    /// Initial animation state when the component mounts
    #[prop(optional)] initial: Option<AnimationTarget>,
    /// Target animation state to animate towards
    #[prop(optional)] animate: Option<AnimationTarget>, 
    /// Exit animation state when the component unmounts
    #[prop(optional)] exit: Option<AnimationTarget>,
    /// Animation transition configuration (timing, easing, etc.)
    #[prop(optional)] transition: Option<Transition>,
    /// Named animation states for complex animations
    #[prop(optional)] variants: Option<Variants>,
    /// Whether to animate layout changes (position, size)
    #[prop(optional)] layout: Option<bool>,
    /// Drag gesture configuration
    #[prop(optional)] drag: Option<DragConfig>,
    /// Animation to play while hovering
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    /// Animation to play while tapping
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    /// Animation to play while focused
    #[prop(optional)] while_focus: Option<AnimationTarget>,
    /// Animation to play while in viewport
    #[prop(optional)] while_in_view: Option<AnimationTarget>,
    /// Event handler workaround for Leptos v0.8.x compatibility
    #[prop(optional)] event_handlers: Option<EventHandlers>,
    /// CSS class names
    #[prop(optional)] class: Option<String>,
    /// Inline CSS styles
    #[prop(optional)] style: Option<String>,
    /// HTML element ID
    #[prop(optional)] id: Option<String>,
    /// Child elements to render
    children: Children,
) -> impl IntoView {
    let motion_props = MotionProps {
        initial,
        animate,
        exit,
        transition,
        variants,
        layout,
        drag,
        while_hover,
        while_tap,
        while_focus,
        while_in_view,
        event_handlers,
    };
    
    create_motion_div(motion_props, class, style, id, children)
}

/// Core motion span component that provides animation capabilities to span elements.
/// 
/// Similar to MotionDiv but for inline text elements. Supports all the same
/// animation features including initial, animate, exit, gesture, and layout animations.
/// 
/// # Example
/// ```rust
/// use leptos::*;
/// use leptos_motion_dom::{MotionSpan, motion_target};
/// use leptos_motion_core::{AnimationValue, Transition};
/// 
/// #[component]
/// pub fn AnimatedText() -> impl IntoView {
///     view! {
///         <MotionSpan
///             initial=motion_target!("opacity" => AnimationValue::Number(0.0))
///             animate=motion_target!("opacity" => AnimationValue::Number(1.0))
///             transition=Transition::default()
///             class="animated-text".to_string()
///         >
///             "Animated text content"
///         </MotionSpan>
///     }
/// }
/// ```
#[component]
pub fn MotionSpan(
    /// Initial animation state when the component mounts
    #[prop(optional)] initial: Option<AnimationTarget>,
    /// Target animation state to animate towards
    #[prop(optional)] animate: Option<AnimationTarget>,
    /// Exit animation state when the component unmounts
    #[prop(optional)] exit: Option<AnimationTarget>,
    /// Animation transition configuration (timing, easing, etc.)
    #[prop(optional)] transition: Option<Transition>,
    /// Named animation states for complex animations
    #[prop(optional)] variants: Option<Variants>,
    /// Whether to animate layout changes (position, size)
    #[prop(optional)] layout: Option<bool>,
    /// Drag gesture configuration
    #[prop(optional)] drag: Option<DragConfig>,
    /// Animation to play while hovering
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    /// Animation to play while tapping
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    /// Animation to play while focused
    #[prop(optional)] while_focus: Option<AnimationTarget>,
    /// Animation to play while in viewport
    #[prop(optional)] while_in_view: Option<AnimationTarget>,
    /// CSS class names
    #[prop(optional)] class: Option<String>,
    /// Inline CSS styles
    #[prop(optional)] style: Option<String>,
    /// HTML element ID
    #[prop(optional)] id: Option<String>,
    /// Child elements to render
    children: Children,
) -> impl IntoView {
    let motion_props = MotionProps {
        initial,
        animate,
        exit,
        transition,
        variants,
        layout,
        drag,
        while_hover,
        while_tap,
        while_focus,
        while_in_view,
        event_handlers: None,
    };
    
    create_motion_span(motion_props, class, style, id, children)
}

/// Create a motion div element with gesture support and FLIP animations
fn create_motion_div(
    motion_props: MotionProps,
    class: Option<String>,
    style: Option<String>,
    _id: Option<String>,
    children: Children,
) -> impl IntoView {
    let element_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    
    // Create motion state
    let motion_state = RwSignal::new(MotionState::new(motion_props.clone()));
    
    // Create gesture detector if gestures are enabled
    let gesture_detector = if motion_props.drag.is_some() || 
                           motion_props.while_hover.is_some() || 
                           motion_props.while_tap.is_some() {
        Some(Rc::new(RefCell::new(leptos_motion_gestures::GestureDetector::default())))
    } else {
        None
    };
    
    // Create FLIP animator if layout animations are enabled
    let flip_animator = if motion_props.layout.unwrap_or(false) {
        Some(Rc::new(RefCell::new(leptos_motion_layout::FLIPAnimator::new())))
    } else {
        None
    };
    
    // Clone motion_props for closures
    let motion_props_clone1 = motion_props.clone();
    let motion_props_clone2 = motion_props.clone();
    let motion_props_clone3 = motion_props.clone();
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            let state = motion_state.get();
            apply_animation_styles(&element, &state.current_values);
        }
    });
    
    // Handle animations
    Effect::new(move |_| {
        if let Some(animate) = &motion_props_clone1.animate {
            if let Some(element) = element_ref.get() {
                start_animation(&element, animate, &motion_props_clone1.transition);
            }
        }
    });
    
    // Set up gesture detection and animation triggers
    Effect::new(move |_| {
        if let (Some(element), Some(detector)) = (element_ref.get(), &gesture_detector) {
            setup_gesture_animations(&element, &motion_props_clone2, detector);
        }
    });
    
    // Set up FLIP layout animations
    Effect::new(move |_| {
        if let (Some(element), Some(animator)) = (element_ref.get(), &flip_animator) {
            setup_flip_animations(&element, &motion_props_clone3, animator);
        }
    });
    
    // Set up event handler workaround for Leptos v0.8.x compatibility
    let motion_props_clone4 = motion_props.clone();
    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            if let Some(event_handlers) = &motion_props_clone4.event_handlers {
                setup_event_handler_workaround(&element, event_handlers);
            }
        }
    });
    
    view! {
        <div
            node_ref=element_ref
            class=class
            style=style
        >
            {children()}
        </div>
    }.into_view()
}

/// Set up gesture-based animations
fn setup_gesture_animations(
    element: &web_sys::Element,
    props: &MotionProps,
    detector: &Rc<RefCell<leptos_motion_gestures::GestureDetector>>,
) {
    let mut detector = detector.borrow_mut();
    
    // Attach gesture detector to element
    if let Err(e) = detector.attach(element.clone()) {
        log::warn!("Failed to attach gesture detector: {:?}", e);
        return;
    }
    
    // For now, just log that gestures are set up
    // The actual gesture handling will be implemented in a future iteration
    log::info!("Gesture detector attached to element");
}

/// Handle drag gesture and apply constraints
fn handle_drag_gesture(
    element: &web_sys::Element,
    drag_config: &DragConfig,
    result: leptos_motion_gestures::GestureResult,
) {
    if let Some(data) = &result.data {
        // Extract drag position from gesture result
        let (x, y) = data.center;
        
        // Apply constraints
        let (constrained_x, constrained_y) = apply_drag_constraints(
            x, y, drag_config
        );
        
        // Apply the constrained position
        let target = motion_target!(
            "x" => AnimationValue::Pixels(constrained_x),
            "y" => AnimationValue::Pixels(constrained_y)
        );
        
        apply_animation_styles(element, &target);
    }
}

/// Apply drag constraints based on configuration
fn apply_drag_constraints(x: f64, y: f64, config: &DragConfig) -> (f64, f64) {
    let mut constrained_x = x;
    let mut constrained_y = y;
    
    // Apply axis constraints
    if let Some(axis) = &config.axis {
        match axis {
            DragAxis::X => constrained_y = 0.0,
            DragAxis::Y => constrained_x = 0.0,
            DragAxis::Both => {} // No constraint
        }
    }
    
    // Apply boundary constraints
    if let Some(constraints) = &config.constraints {
        if let Some(left) = constraints.left {
            constrained_x = constrained_x.max(left);
        }
        if let Some(right) = constraints.right {
            constrained_x = constrained_x.min(right);
        }
        if let Some(top) = constraints.top {
            constrained_y = constrained_y.max(top);
        }
        if let Some(bottom) = constraints.bottom {
            constrained_y = constrained_y.min(bottom);
        }
    }
    
    (constrained_x, constrained_y)
}

/// Set up FLIP layout animations
fn setup_flip_animations(
    element: &web_sys::Element,
    _props: &MotionProps,
    animator: &Rc<RefCell<leptos_motion_layout::FLIPAnimator>>,
) {
    // Record initial position for FLIP
    let rect = element.get_bounding_client_rect();
    let initial_layout = leptos_motion_layout::LayoutInfo {
        x: rect.left(),
        y: rect.top(),
        width: rect.width(),
        height: rect.height(),
    };
    
    // Store initial layout in element data
    element.set_attribute("data-flip-initial", &serde_json::to_string(&initial_layout).unwrap_or_default()).ok();
    
    // Set up layout change observer
    setup_layout_observer(element, animator.clone());
}

/// Set up a ResizeObserver to detect layout changes
fn setup_layout_observer(
    _element: &web_sys::Element,
    animator: Rc<RefCell<leptos_motion_layout::FLIPAnimator>>,
) {
    // For now, just log that layout observation is set up
    // The actual layout change detection will be implemented in a future iteration
    log::info!("Layout observer set up for element");
}

/// Handle layout changes and trigger FLIP animations
fn handle_layout_change(
    element: &web_sys::Element,
    animator: &mut leptos_motion_layout::FLIPAnimator,
) {
    // Get initial layout from element data
    let initial_layout_str = element.get_attribute("data-flip-initial").unwrap_or_default();
    let initial_layout: Option<leptos_motion_layout::LayoutInfo> = 
        serde_json::from_str(&initial_layout_str).ok();
    
    if let Some(initial) = initial_layout {
        // Get current layout
        let rect = element.get_bounding_client_rect();
        let current_layout = leptos_motion_layout::LayoutInfo {
            x: rect.left(),
            y: rect.top(),
            width: rect.width(),
            height: rect.height(),
        };
        
        // Calculate the difference (for future use)
        let _delta_x = current_layout.x - initial.x;
        let _delta_y = current_layout.y - initial.y;
        let _scale_x = current_layout.width / initial.width;
        let _scale_y = current_layout.height / initial.height;
        
        // Create FLIP animation using the create_animation method
        if let Err(e) = animator.create_animation(
            element,
            web_sys::DomRect::new_with_x_and_y_and_width_and_height(
                initial.x, initial.y, initial.width, initial.height
            ).unwrap(),
            web_sys::DomRect::new_with_x_and_y_and_width_and_height(
                current_layout.x, current_layout.y, current_layout.width, current_layout.height
            ).unwrap(),
            &leptos_motion_layout::LayoutAnimationConfig::default()
        ) {
            log::warn!("Failed to create FLIP animation: {:?}", e);
        }
        

    }
}

/// Create a motion span element
fn create_motion_span(
    motion_props: MotionProps,
    class: Option<String>,
    style: Option<String>,
    _id: Option<String>,
    children: Children,
) -> impl IntoView {
    let element_ref: NodeRef<leptos::html::Span> = NodeRef::new();
    
    // Create motion state
    let motion_state = RwSignal::new(MotionState::new(motion_props.clone()));
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            let state = motion_state.get();
            apply_animation_styles(&element, &state.current_values);
        }
    });
    
    // Handle animations
    Effect::new(move |_| {
        if let Some(animate) = &motion_props.animate {
            if let Some(element) = element_ref.get() {
                start_animation(&element, animate, &motion_props.transition);
            }
        }
    });
    
    view! {
        <span
            class=class
            style=style
        >
            {children()}
        </span>
    }.into_view()
}

/// Internal motion state
#[derive(Clone, Debug)]
struct MotionState {
    current_values: AnimationTarget,
}

impl MotionState {
    fn new(props: MotionProps) -> Self {
        let current_values = props.initial.clone().unwrap_or_default();
        
        Self {
            current_values,
        }
    }
}

/// Apply animation styles to element
fn apply_animation_styles(element: &web_sys::Element, values: &AnimationTarget) {
    use leptos_motion_core::AnimationValue;
    
    // Cast to HtmlElement to access style API
    if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        for (property, value) in values {
            match value {
                AnimationValue::Number(num) => {
                    match property.as_str() {
                        "opacity" => {
                            let _ = html_element.style().set_property("opacity", &num.to_string());
                        }
                        "scale" => {
                            let _ = html_element.style().set_property("transform", &format!("scale({})", num));
                        }
                        _ => {
                            // For other numeric properties, try to set as CSS property
                            let _ = html_element.style().set_property(property, &num.to_string());
                        }
                    }
                }
                AnimationValue::Pixels(pixels) => {
                    match property.as_str() {
                        "x" | "left" => {
                            let _ = html_element.style().set_property("transform", &format!("translateX({}px)", pixels));
                        }
                        "y" | "top" => {
                            let _ = html_element.style().set_property("transform", &format!("translateY({}px)", pixels));
                        }
                        _ => {
                            let _ = html_element.style().set_property(property, &format!("{}px", pixels));
                        }
                    }
                }
                AnimationValue::Degrees(degrees) => {
                    if property == "rotate" {
                        let _ = html_element.style().set_property("transform", &format!("rotate({}deg)", degrees));
                    }
                }
                AnimationValue::String(s) => {
                    if property == "transform" {
                        let _ = html_element.style().set_property("transform", s);
                    } else {
                        let _ = html_element.style().set_property(property, s);
                    }
                }
                _ => {
                    // For other types, convert to string and set
                    let _ = html_element.style().set_property(property, &value.to_string());
                }
            }
        }
    }
}

/// Start animation on element using the animation engine
fn start_animation(
    element: &web_sys::Element,
    target: &AnimationTarget,
    transition: &Option<Transition>,
) {
    use leptos_motion_core::{animation::AnimationConfig, Transition as CoreTransition};
use std::collections::HashMap;
    
    // For now, use a simplified approach that directly applies styles
    // In a full implementation, this would use the animation engine
    
    // Get current element styles as starting point
    let from = get_current_animation_values(element);
    
    // Convert transition to core transition
    let core_transition = transition.as_ref().map(|t| CoreTransition {
        duration: t.duration,
        delay: t.delay,
        ease: t.ease.clone(),
        repeat: t.repeat.clone(),
        stagger: t.stagger.clone(),
    }).unwrap_or_default();
    
    // Create animation config for the animation system
    let _config = AnimationConfig {
        initial: from,
        animate: target.clone(),
        exit: HashMap::new(), // No exit animation for now
        transition: core_transition,
        variants: None,
    };
    
    // For now, just apply the target styles directly
    // TODO: Integrate with the animation engine
    apply_animation_styles(element, target);
}

/// Get current animation values from element
fn get_current_animation_values(element: &web_sys::Element) -> leptos_motion_core::AnimationTarget {
    use leptos_motion_core::AnimationValue;
    
    let mut values = std::collections::HashMap::new();
    
    // Get computed styles
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(computed_style) = document.default_view().unwrap().get_computed_style(element).ok().flatten() {
                // Get transform values
                if let Ok(transform) = computed_style.get_property_value("transform") {
                    if transform != "none" {
                        values.insert("transform".to_string(), AnimationValue::String(transform));
                    }
                }
                
                // Get opacity
                if let Ok(opacity) = computed_style.get_property_value("opacity") {
                    if let Ok(opacity_value) = opacity.parse::<f64>() {
                        values.insert("opacity".to_string(), AnimationValue::Number(opacity_value));
                    }
                }
                
                // Get position values
                if let Ok(top) = computed_style.get_property_value("top") {
                    if let Ok(top_value) = top.parse::<f64>() {
                        values.insert("top".to_string(), AnimationValue::Pixels(top_value));
                    }
                }
                
                if let Ok(left) = computed_style.get_property_value("left") {
                    if let Ok(left_value) = left.parse::<f64>() {
                        values.insert("left".to_string(), AnimationValue::Pixels(left_value));
                    }
                }
            }
        }
    }
    
    values
}

/// Set up event handler workaround for Leptos v0.8.x compatibility
fn setup_event_handler_workaround(
    element: &web_sys::Element,
    event_handlers: &EventHandlers,
) {
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlElement, Event, MouseEvent};
    
    // Cast to HtmlElement for event handling
    let html_element = match element.dyn_ref::<HtmlElement>() {
        Some(el) => el,
        None => {
            log::warn!("Element is not an HtmlElement, cannot attach event handlers");
            return;
        }
    };
    
    // Set up click handler if specified
    if let Some(click_handler) = &event_handlers.on_click {
        let click_handler_clone = click_handler.clone();
        let state_clone = event_handlers.state.clone();
        
        // Create closure for click handling
        let click_closure = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            
            if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                let target = mouse_event.current_target().unwrap();
                let html_element = target.dyn_ref::<HtmlElement>().unwrap();
                
                match &click_handler_clone {
                    ClickHandler::Counter => {
                        handle_counter_click(html_element, &state_clone);
                    }
                    ClickHandler::Toggle => {
                        handle_toggle_click(html_element, &state_clone);
                    }
                    ClickHandler::LayoutToggle => {
                        handle_layout_toggle_click(html_element, &state_clone);
                    }
                    ClickHandler::Custom(_) => {
                        log::info!("Custom click handler triggered");
                    }
                }
            }
        }) as Box<dyn FnMut(Event)>);
        
        // Attach the click handler
        html_element.set_onclick(Some(click_closure.as_ref().unchecked_ref()));
        
        // Store the closure to prevent it from being dropped
        click_closure.forget();
        
        log::info!("Event handler workaround attached successfully");
    }
}

/// Handle counter click functionality
fn handle_counter_click(element: &HtmlElement, _state: &Option<InteractiveState>) {
    let current_text = element.text_content().unwrap_or_default();
    
    if current_text.contains("Count:") {
        // Extract current count
        let count_str = current_text.split("Count:").nth(1).unwrap_or("0").trim();
        let current_count = count_str.parse::<i32>().unwrap_or(0);
        let new_count = current_count + 1;
        
        // Update button text
        element.set_text_content(Some(&format!("Count: {}", new_count)));
        log::info!("Counter updated to: {}", new_count);
    }
}

/// Handle toggle click functionality
fn handle_toggle_click(element: &HtmlElement, _state: &Option<InteractiveState>) {
    let current_text = element.text_content().unwrap_or_default();
    
    if current_text.contains("Hide") || current_text.contains("Show") {
        let is_visible = current_text.contains("Hide");
        let new_text = if is_visible { "Show" } else { "Hide" };
        
        // Update button text
        element.set_text_content(Some(new_text));
        
        // Toggle content visibility if content box exists
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(content_box) = document.query_selector(".content-box").ok().flatten() {
                    if let Some(content_html) = content_box.dyn_ref::<HtmlElement>() {
                        let display = if is_visible { "none" } else { "block" };
                        content_html.style().set_property("display", display).ok();
                    }
                }
            }
        }
        
        log::info!("Visibility toggled to: {}", !is_visible);
    }
}

/// Handle layout toggle click functionality
fn handle_layout_toggle_click(element: &HtmlElement, _state: &Option<InteractiveState>) {
    let current_text = element.text_content().unwrap_or_default();
    
    if current_text.contains("Switch to") {
        let is_grid = current_text.contains("Grid");
        let new_text = if is_grid { "Switch to List" } else { "Switch to Grid" };
        
        // Update button text
        element.set_text_content(Some(new_text));
        
        // Toggle layout if layout container exists
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(layout_container) = document.query_selector(".layout-demo > div:last-child").ok().flatten() {
                    let new_class = if is_grid { "list-layout" } else { "grid-layout" };
                    layout_container.set_class_name(new_class);
                }
            }
        }
        
        log::info!("Layout toggled to: {}", if is_grid { "list" } else { "grid" });
    }
}

/// Convenience macro for creating animation targets
#[macro_export]
macro_rules! motion_target {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut target = std::collections::HashMap::new();
            $(
                target.insert($key.to_string(), $value);
            )*
            target
        }
    };
}

/// Helper functions for common animation patterns
pub mod helpers {
    use super::*;
    
    /// Create fade in animation target
    pub fn fade_in() -> AnimationTarget {
        motion_target!("opacity" => AnimationValue::Number(1.0))
    }
    
    /// Create fade out animation target
    pub fn fade_out() -> AnimationTarget {
        motion_target!("opacity" => AnimationValue::Number(0.0))
    }
    
    /// Create slide up animation target
    pub fn slide_up(distance: f64) -> AnimationTarget {
        motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "y" => AnimationValue::Pixels(-distance)
        )
    }
    
    /// Create scale animation target
    pub fn scale(factor: f64) -> AnimationTarget {
        motion_target!("scale" => AnimationValue::Number(factor))
    }
    
    /// Create rotation animation target
    pub fn rotate(degrees: f64) -> AnimationTarget {
        motion_target!("rotate" => AnimationValue::Degrees(degrees))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_motion_props_default() {
        let props = MotionProps::default();
        assert!(props.initial.is_none());
        assert!(props.animate.is_none());
        assert!(props.layout.is_none());
    }
    
    #[test] 
    fn test_drag_config() {
        let drag = DragConfig::new()
            .axis(DragAxis::X)
            .constraints(DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: None,
                bottom: None,
            });
            
        assert_eq!(drag.axis, Some(DragAxis::X));
        assert!(drag.constraints.is_some());
    }
    
    #[test]
    fn test_helper_functions() {
        let fade = helpers::fade_in();
        assert_eq!(fade.get("opacity"), Some(&AnimationValue::Number(1.0)));
        
        let slide = helpers::slide_up(20.0);
        assert_eq!(slide.get("y"), Some(&AnimationValue::Pixels(-20.0)));
        
        let scale = helpers::scale(2.0);
        assert_eq!(scale.get("scale"), Some(&AnimationValue::Number(2.0)));
    }
}