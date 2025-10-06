//! Event-Driven MotionDiv
//!
//! This module implements a MotionDiv component that uses the new event-driven
//! animation architecture, providing a clean API similar to motion.dev.

use leptos::prelude::*;
use leptos_motion_core::*;
use crate::{
    OptimizedAnimationManager,
    CssTransitionAnimation,
    KeyframeAnimation,
    Keyframe,
    StaggerAnimation,
    EventStaggerConfig,
    SpringAnimation,
    EventSpringConfig,
    AnimationValue,
    Transition,
    AnimateProp,
    resolve_animate_prop,
    LayoutAnimationManager,
    LayoutConfig,
    SharedElementManager,
    SharedLayoutConfig,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Get current time in nanoseconds (WASM-compatible)
fn get_current_time_nanos() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        // Use js_sys::Date::now() for WASM
        (js_sys::Date::now() * 1_000_000.0) as u128
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use SystemTime for native targets
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}

/// Event-driven MotionDiv component
#[component]
pub fn EventDrivenMotionDiv(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    
    /// Target animation values (reactive support)
    #[prop(optional)]
    animate: Option<AnimateProp>,
    
    /// Animation while hovering
    #[prop(optional)]
    while_hover: Option<HashMap<String, AnimationValue>>,
    
    /// Animation while tapping
    #[prop(optional)]
    while_tap: Option<HashMap<String, AnimationValue>>,
    
    /// Animation while dragging
    #[prop(optional)]
    while_drag: Option<HashMap<String, AnimationValue>>,
    
    /// Transition configuration
    #[prop(optional)]
    _transition: Option<Transition>,
    
    /// Animation type (css, keyframe, stagger, spring)
    #[prop(optional, default = AnimationType::Css)]
    animation_type: AnimationType,
    
    /// Keyframes for keyframe animations
    #[prop(optional)]
    keyframes: Option<Vec<Keyframe>>,
    
    /// Stagger configuration
    #[prop(optional)]
    stagger_config: Option<EventStaggerConfig>,
    
    /// Spring configuration
    #[prop(optional)]
    spring_config: Option<EventSpringConfig>,
    
    /// Drag constraints
    #[prop(optional)]
    _drag_constraints: Option<DragConstraints>,
    
    /// Whether element is draggable
    #[prop(optional)]
    drag: Option<DragConfig>,
    
    /// Enable layout animations
    #[prop(optional, default = false)]
    layout: bool,

    /// Layout animation configuration
    #[prop(optional)]
    layout_config: Option<crate::LayoutConfig>,

    /// Layout ID for shared element transitions
    #[prop(optional)]
    layout_id: Option<String>,

    /// Shared layout configuration
    #[prop(optional)]
    shared_layout: Option<crate::SharedLayoutConfig>,

    /// Animation variants
    #[prop(optional)]
    variants: Option<crate::AnimationVariants>,

    /// Initial variant name
    #[prop(optional)]
    initial_variant: Option<String>,

    /// Animate variant name
    #[prop(optional)]
    animate_variant: Option<String>,

    /// Exit variant name
    #[prop(optional)]
    exit_variant: Option<String>,

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
) -> impl IntoView {
    // Get animation manager from context or create new one
    let animation_manager = use_context::<Rc<RefCell<OptimizedAnimationManager>>>()
        .unwrap_or_else(|| Rc::new(RefCell::new(OptimizedAnimationManager::new())));

    // Create layout animation manager if layout animations are enabled
    let layout_manager = if layout {
        Some(Rc::new(RefCell::new(crate::LayoutAnimationManager::new())))
    } else {
        None
    };

    // Create shared element manager for shared layout transitions
    let shared_manager = if layout_id.is_some() || shared_layout.is_some() {
        layout_manager.as_ref().map(|lm| Rc::new(RefCell::new(crate::SharedElementManager::new(lm.clone()))))
    } else {
        None
    };

    // State management
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);
    let (is_dragging, set_dragging) = signal(false);
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    
    // Apply initial styles (from initial prop or initial_variant)
    let variants_clone = variants.clone();
    let initial_variant_clone = initial_variant.clone();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let initial_values = if let Some(variants) = &variants_clone {
                // Use initial variant if specified
                if let Some(variant_name) = &initial_variant_clone {
                    variants.resolve_variant(variant_name, None).unwrap_or_default()
                } else {
                    HashMap::new()
                }
            } else {
                // Use initial prop directly
                initial.clone().unwrap_or_default()
            };

            if !initial_values.is_empty() {
                apply_initial_styles(&element, &initial_values);
            }
        }
    });
    
    // Handle layout animation and shared layout transitions
    if layout || layout_id.is_some() || shared_layout.is_some() {
        let layout_config = layout_config.unwrap_or_default();
        let shared_config = shared_layout.unwrap_or_default();
        let layout_manager_clone = layout_manager.clone();
        let shared_manager_clone = shared_manager.clone();
        Effect::new(move |_| {
            if let Some(element) = node_ref.get()
                && let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
                    let style = html_element.style();
                    // Enable layout animations
                    let _ = style.set_property("will-change", "transform, opacity");
                    let _ = style.set_property("transform-origin", "center center");

                    // Register element for layout animations
                    if let Some(layout_manager) = &layout_manager_clone {
                        let element_id = format!("motion-div-{:p}", &*layout_manager.as_ref());
                        let mut manager = layout_manager.borrow_mut();
                        let _ = manager.register_element(
                            element_id,
                            &html_element,
                            layout_config.clone(),
                        );
                    }

                    // Register element for shared layout transitions
                    if let (Some(shared_manager), Some(layout_id)) = (&shared_manager_clone, &layout_id) {
                        let mut manager = shared_manager.borrow_mut();
                        let _ = manager.register_element(
                            layout_id.clone(),
                            html_element.clone().dyn_into().unwrap(),
                            shared_config.clone(),
                        );
                    }
                }
        });
    }
    
    // Handle hover events
    let handle_mouse_enter = {
        let animation_manager = animation_manager.clone();
        let while_hover = while_hover.clone();
        let transition = _transition.clone();
        let animation_type = animation_type.clone();
        let keyframes = keyframes.clone();
        let stagger_config = stagger_config.clone();
        let spring_config = spring_config.clone();
        
        move |_| {
            set_hovered.set(true);
            
            if let Some(element) = node_ref.get()
                && let Some(hover_values) = &while_hover {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        hover_values,
                        &transition,
                        animation_type.clone(),
                        &keyframes,
                        &stagger_config,
                        &spring_config,
                        "hover",
                    );
                }
        }
    };
    
    let handle_mouse_leave = {
        let animation_manager = animation_manager.clone();
        let animate = animate.clone();
        let transition = _transition.clone();
        let animation_type = animation_type.clone();
        let keyframes = keyframes.clone();
        let stagger_config = stagger_config.clone();
        let spring_config = spring_config.clone();
        let variants_clone = variants.clone();
        let animate_variant_clone = animate_variant.clone();

        move |_| {
            set_hovered.set(false);

            if let Some(element) = node_ref.get() {
                // Resolve animation values (either from animate prop or variants)
                let animate_values = if let Some(variants) = &variants_clone {
                    // Use variants if specified
                    if let Some(variant_name) = &animate_variant_clone {
                        variants.resolve_variant(variant_name, None).unwrap_or_default()
                    } else {
                        HashMap::new()
                    }
                } else if let Some(animate_prop) = &animate {
                    // Use animate prop directly
                    resolve_animate_prop(&Some(animate_prop.clone()))
                } else {
                    HashMap::new()
                };

                if !animate_values.is_empty() {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        &animate_values,
                        &transition,
                        animation_type.clone(),
                        &keyframes,
                        &stagger_config,
                        &spring_config,
                        "animate",
                    );
                }
            }
        }
    };
    
    // Handle tap events
    let handle_click = {
        let animation_manager = animation_manager.clone();
        let while_tap = while_tap.clone();
        let transition = _transition.clone();
        let animation_type = animation_type.clone();
        let keyframes = keyframes.clone();
        let stagger_config = stagger_config.clone();
        let spring_config = spring_config.clone();
        
        move |_| {
            set_tapped.set(true);
            
            if let Some(element) = node_ref.get()
                && let Some(tap_values) = &while_tap {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        tap_values,
                        &transition,
                        animation_type.clone(),
                        &keyframes,
                        &stagger_config,
                        &spring_config,
                        "tap",
                    );
                }
            
            // Reset tap state after animation
            set_timeout(move || {
                set_tapped.set(false);
            }, std::time::Duration::from_millis(200));
        }
    };
    
    // Handle drag events
    let handle_drag_start = {
        let animation_manager = animation_manager.clone();
        let while_drag = while_drag.clone();
        let transition = _transition.clone();
        let animation_type = animation_type.clone();
        let keyframes = keyframes.clone();
        let stagger_config = stagger_config.clone();
        let spring_config = spring_config.clone();
        let drag = drag.clone();
        let drag_constraints = _drag_constraints.clone();
        
        move |_| {
            if drag.is_some() {
                set_dragging.set(true);
                
                if let Some(element) = node_ref.get() {
                    // Apply drag constraints if specified
                    if let Some(constraints) = &drag_constraints {
                        apply_drag_constraints(&element, constraints);
                    }
                    
                    if let Some(drag_values) = &while_drag {
                        trigger_animation(
                            &animation_manager,
                            &element,
                            drag_values,
                            &transition,
                            animation_type.clone(),
                            &keyframes,
                            &stagger_config,
                            &spring_config,
                            "drag",
                        );
                    }
                }
            }
        }
    };
    
    let handle_drag_end = {
        let animation_manager = animation_manager.clone();
        let animate = animate.clone();
        let transition = _transition.clone();
        let animation_type = animation_type.clone();
        let keyframes = keyframes.clone();
        let stagger_config = stagger_config.clone();
        let spring_config = spring_config.clone();
        
        move |_| {
            set_dragging.set(false);
            
            if let Some(element) = node_ref.get()
                && let Some(animate_prop) = &animate {
                    // Resolve reactive values
                    let animate_values = resolve_animate_prop(&Some(animate_prop.clone()));
                    if !animate_values.is_empty() {
                        trigger_animation(
                            &animation_manager,
                            &element,
                            &animate_values,
                            &transition,
                            animation_type.clone(),
                            &keyframes,
                            &stagger_config,
                            &spring_config,
                            "animate",
                        );
                    }
                }
        }
    };
    
    // Handle animate prop changes (reactive support)
    let transition_for_effect = _transition.clone();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get()
            && let Some(animate_prop) = &animate
                && !is_hovered.get() && !is_tapped.get() && !is_dragging.get() {
                    // Resolve reactive values
                    let animate_values = resolve_animate_prop(&Some(animate_prop.clone()));
                    if !animate_values.is_empty() {
                        // Apply CSS properties directly for reactive animations
                        apply_animation_styles(&element, &animate_values);
                    }
                }
    });
    
    // Build CSS classes
    let css_classes = {
        let drag = drag.clone();
        let mut classes = Vec::new();
        if !class.is_empty() {
            classes.push(class.clone());
        }
        if drag.is_some() {
            classes.push("draggable".to_string());
        }
        classes.join(" ")
    };
    
    view! {
        <div
            node_ref=node_ref
            class=css_classes
            style=style
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            on:click=handle_click
            on:dragstart=handle_drag_start
            on:dragend=handle_drag_end
            draggable=drag.is_some()
        >
            {children()}
        </div>
    }
}

/// Animation type enum
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    Css,
    Keyframe,
    Stagger,
    Spring,
}

impl Default for AnimationType {
    fn default() -> Self {
        Self::Css
    }
}

/// Drag axis enum
#[derive(Debug, Clone, PartialEq)]
pub enum DragAxis {
    X,
    Y,
    Both,
}

impl Default for DragAxis {
    fn default() -> Self {
        Self::Both
    }
}

/// Drag configuration
#[derive(Debug, Clone, Default)]
pub struct DragConfig {
    pub axis: Option<DragAxis>,
    pub momentum: Option<bool>,
    pub elastic: Option<f64>,
    pub constraints: Option<DragConstraints>,
}

/// Drag constraints
#[derive(Debug, Clone, Default)]
pub struct DragConstraints {
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
}

/// Apply initial styles to element
fn apply_initial_styles(element: &Element, styles: &HashMap<String, AnimationValue>) {
    if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        let style = html_element.style();

        for (property, value) in styles {
            match (property.as_str(), value) {
                ("opacity", AnimationValue::Number(n)) => {
                    let _ = style.set_property("opacity", &n.to_string());
                }
                ("scale", AnimationValue::Number(n)) => {
                    let _ = style.set_property("transform", &format!("scale({})", n));
                }
                ("x", AnimationValue::Number(n)) => {
                    let _ = style.set_property("transform", &format!("translateX({}px)", n));
                }
                ("y", AnimationValue::Number(n)) => {
                    let _ = style.set_property("transform", &format!("translateY({}px)", n));
                }
                ("rotate", AnimationValue::Number(n)) => {
                    let _ = style.set_property("transform", &format!("rotate({}deg)", n));
                }
                ("rotateZ", AnimationValue::Degrees(d)) => {
                    let _ = style.set_property("transform", &format!("rotateZ({}deg)", d));
                }
                ("rotateZ", AnimationValue::Number(n)) => {
                    let _ = style.set_property("transform", &format!("rotateZ({}deg)", n));
                }
                ("width", AnimationValue::Number(n)) => {
                    let _ = style.set_property("width", &format!("{}px", n));
                }
                ("height", AnimationValue::Number(n)) => {
                    let _ = style.set_property("height", &format!("{}px", n));
                }
                ("stroke-dashoffset", AnimationValue::Number(n)) => {
                    let _ = style.set_property("stroke-dashoffset", &n.to_string());
                }
                ("stroke-dashoffset", AnimationValue::Pixels(p)) => {
                    let _ = style.set_property("stroke-dashoffset", &format!("{}px", p));
                }
                (_, AnimationValue::String(s)) => {
                    let _ = style.set_property(property, s);
                }
                (_, AnimationValue::Color(c)) => {
                    let _ = style.set_property(property, c);
                }
                _ => {
                    // For other combinations, try to convert to string
                    if let Some(numeric) = extract_numeric_value(value) {
                        let _ = style.set_property(property, &numeric.to_string());
                    }
                }
            }
        }
    }
}

/// Extract numeric value from AnimationValue
fn extract_numeric_value(value: &AnimationValue) -> Option<f64> {
    match value {
        AnimationValue::Number(n) => Some(*n),
        AnimationValue::Pixels(p) => Some(*p),
        AnimationValue::Percentage(p) => Some(*p),
        AnimationValue::Degrees(d) => Some(*d),
        AnimationValue::Radians(r) => Some(*r),
        _ => None,
    }
}

/// Apply animation styles directly to DOM element
fn apply_animation_styles(element: &Element, styles: &HashMap<String, AnimationValue>) {
    if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        let style = html_element.style();
        
        // Debug: log when we're applying styles
        web_sys::console::log_1(&format!("Applying styles to DOM: {:?}", styles).into());
        
        // Build combined transform string for all transform properties
        let mut transform_parts = Vec::new();
        let mut has_transform = false;
        
        for (property, value) in styles {
            match (property.as_str(), value) {
                ("opacity", AnimationValue::Number(n)) => {
                    let _ = style.set_property("opacity", &n.to_string());
                }
                ("scale", AnimationValue::Number(n)) => {
                    transform_parts.push(format!("scale({})", n));
                    has_transform = true;
                }
                ("x", AnimationValue::Number(n)) => {
                    transform_parts.push(format!("translateX({}px)", n));
                    has_transform = true;
                }
                ("y", AnimationValue::Number(n)) => {
                    transform_parts.push(format!("translateY({}px)", n));
                    has_transform = true;
                }
                ("rotate", AnimationValue::Number(n)) => {
                    transform_parts.push(format!("rotate({}deg)", n));
                    has_transform = true;
                }
                ("rotateZ", AnimationValue::Degrees(d)) => {
                    transform_parts.push(format!("rotateZ({}deg)", d));
                    has_transform = true;
                }
                ("rotateZ", AnimationValue::Number(n)) => {
                    transform_parts.push(format!("rotateZ({}deg)", n));
                    has_transform = true;
                }
                ("width", AnimationValue::Number(n)) => {
                    let _ = style.set_property("width", &format!("{}px", n));
                }
                ("height", AnimationValue::Number(n)) => {
                    let _ = style.set_property("height", &format!("{}px", n));
                }
                ("stroke-dashoffset", AnimationValue::Number(n)) => {
                    let _ = style.set_property("stroke-dashoffset", &n.to_string());
                }
                ("stroke-dashoffset", AnimationValue::Pixels(p)) => {
                    let _ = style.set_property("stroke-dashoffset", &format!("{}px", p));
                }
                (_, AnimationValue::String(s)) => {
                    let _ = style.set_property(property, s);
                }
                (_, AnimationValue::Color(c)) => {
                    let _ = style.set_property(property, c);
                }
                _ => {
                    // For other combinations, try to convert to string
                    if let Some(numeric) = extract_numeric_value(value) {
                        let _ = style.set_property(property, &numeric.to_string());
                    }
                }
            }
        }
        
        // Apply combined transform if we have any transform properties
        if has_transform {
            let transform_value = transform_parts.join(" ");
            let _ = style.set_property("transform", &transform_value);
            web_sys::console::log_1(&format!("Applied transform: {}", transform_value).into());
        }
    } else {
        web_sys::console::log_1(&"Element is not an HtmlElement".into());
    }
}

/// Trigger animation based on type
fn trigger_animation(
    animation_manager: &Rc<RefCell<OptimizedAnimationManager>>,
    element: &Element,
    properties: &HashMap<String, AnimationValue>,
    _transition: &Option<Transition>,
    animation_type: AnimationType,
    keyframes: &Option<Vec<Keyframe>>,
    stagger_config: &Option<EventStaggerConfig>,
    spring_config: &Option<EventSpringConfig>,
    animation_name: &str,
) {
    let transition = _transition.clone().unwrap_or_default();
    let id = format!("{}_{}", animation_name, get_current_time_nanos());
    
    let mut manager = match animation_manager.try_borrow_mut() {
        Ok(manager) => manager,
        Err(_) => {
            // If already borrowed, skip this animation to prevent panic
            return;
        }
    };
    
    match animation_type {
        AnimationType::Css => {
            let animation = CssTransitionAnimation::new(
                id,
                element.clone(),
                properties.clone(),
                transition,
            );
            
            if let Err(e) = manager.register_optimized(Box::new(animation)) {
                eprintln!("Animation registration error: {:?}", e);
            }
        }
        AnimationType::Keyframe => {
            if let Some(keyframes) = keyframes {
                let animation = KeyframeAnimation::new(
                    id,
                    element.clone(),
                    keyframes.clone(),
                    transition,
                );
                
                if let Err(e) = manager.register_optimized(Box::new(animation)) {
                    eprintln!("Animation registration error: {:?}", e);
                }
            }
        }
        AnimationType::Stagger => {
            if let Some(stagger_config) = stagger_config {
                let elements = vec![element.clone()];
                let targets = vec![properties.clone()];
                
                let animation = StaggerAnimation::from_elements(
                    id,
                    elements,
                    targets,
                    stagger_config.clone(),
                );
                
                if let Err(e) = manager.register_optimized(Box::new(animation)) {
                    eprintln!("Animation registration error: {:?}", e);
                }
            }
        }
        AnimationType::Spring => {
            if let Some(spring_config) = spring_config {
                let animation = SpringAnimation::new(
                    id,
                    element.clone(),
                    properties.clone(),
                    spring_config.clone(),
                );
                
                if let Err(e) = manager.register_optimized(Box::new(animation)) {
                    eprintln!("Animation registration error: {:?}", e);
                }
            }
        }
    }
}

/// Helper function to create animation values
pub fn create_animation_value(value: f64) -> AnimationValue {
    AnimationValue::Number(value)
}

/// Helper function to create animation targets
pub fn create_animation_target(properties: HashMap<String, AnimationValue>) -> HashMap<String, AnimationValue> {
    properties
}

/// Helper function to create drag constraints
pub fn create_drag_constraints(
    min_x: Option<f64>,
    max_x: Option<f64>,
    min_y: Option<f64>,
    max_y: Option<f64>,
) -> DragConstraints {
    DragConstraints {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

/// Apply drag constraints to an element
fn apply_drag_constraints(element: &Element, constraints: &DragConstraints) {
    if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
        let style = html_element.style();
        
        // Apply X constraints
        if let Some(min_x) = constraints.min_x {
            let _ = style.set_property("--drag-min-x", &format!("{}px", min_x));
        }
        if let Some(max_x) = constraints.max_x {
            let _ = style.set_property("--drag-max-x", &format!("{}px", max_x));
        }
        
        // Apply Y constraints
        if let Some(min_y) = constraints.min_y {
            let _ = style.set_property("--drag-min-y", &format!("{}px", min_y));
        }
        if let Some(max_y) = constraints.max_y {
            let _ = style.set_property("--drag-max-y", &format!("{}px", max_y));
        }
    }
}
