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
};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Event-driven MotionDiv component
#[component]
pub fn EventDrivenMotionDiv(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    
    /// Target animation values
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    
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
    
    /// Layout animation
    #[prop(optional, default = false)]
    _layout: bool,
    
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
    
    // State management
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);
    let (is_dragging, set_dragging) = signal(false);
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            if let Some(initial_values) = &initial {
                apply_initial_styles(&element, initial_values);
            }
        }
    });
    
    // Handle layout animation
    if _layout {
        Effect::new(move |_| {
            if let Some(element) = node_ref.get() {
                if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
                    let style = html_element.style();
                    // Enable layout animations
                    let _ = style.set_property("will-change", "transform, opacity");
                    let _ = style.set_property("transform-origin", "center center");
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
            
            if let Some(element) = node_ref.get() {
                if let Some(hover_values) = &while_hover {
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
        
        move |_| {
            set_hovered.set(false);
            
            if let Some(element) = node_ref.get() {
                if let Some(animate_values) = &animate {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        animate_values,
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
            
            if let Some(element) = node_ref.get() {
                if let Some(tap_values) = &while_tap {
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
            
            if let Some(element) = node_ref.get() {
                if let Some(animate_values) = &animate {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        animate_values,
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
    
    // Handle animate prop changes
    let transition_for_effect = _transition.clone();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            if let Some(animate_values) = &animate {
                if !is_hovered.get() && !is_tapped.get() && !is_dragging.get() {
                    trigger_animation(
                        &animation_manager,
                        &element,
                        animate_values,
                        &transition_for_effect,
                        animation_type.clone(),
                        &keyframes,
                        &stagger_config,
                        &spring_config,
                        "animate",
                    );
                }
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
                ("width", AnimationValue::Number(n)) => {
                    let _ = style.set_property("width", &format!("{}px", n));
                }
                ("height", AnimationValue::Number(n)) => {
                    let _ = style.set_property("height", &format!("{}px", n));
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
    let id = format!("{}_{}", animation_name, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
    
    let mut manager = animation_manager.borrow_mut();
    
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
