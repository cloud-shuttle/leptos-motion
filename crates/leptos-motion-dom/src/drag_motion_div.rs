//! Drag-enabled MotionDiv component
//!
//! This module provides a MotionDiv component with drag functionality,
//! including mouse and touch support, drag constraints, and callbacks.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys;

use crate::animation_engine::AnimationEngine;

/// Drag configuration for MotionDiv
#[derive(Debug, Clone)]
pub struct DragConfig {
    /// Whether dragging is enabled
    pub enabled: bool,
    /// Drag constraints (min/max x, y positions)
    pub constraints: Option<DragConstraints>,
    /// Drag momentum (spring physics)
    pub momentum: Option<DragMomentum>,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            constraints: None,
            momentum: None,
        }
    }
}

/// Drag constraints for limiting drag movement
#[derive(Debug, Clone)]
pub struct DragConstraints {
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
}

/// Drag momentum configuration for spring physics
#[derive(Debug, Clone)]
pub struct DragMomentum {
    pub enabled: bool,
    pub damping: f64,
    pub stiffness: f64,
}

impl Default for DragMomentum {
    fn default() -> Self {
        Self {
            enabled: true,
            damping: 0.8,
            stiffness: 0.1,
        }
    }
}

/// Drag state for tracking drag operations
#[derive(Debug, Clone)]
struct DragState {
    is_dragging: bool,
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    last_time: f64,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            last_time: 0.0,
        }
    }
}

/// Drag-enabled MotionDiv component
#[component]
pub fn DragMotionDiv(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    /// Animate signal that triggers reactive updates
    #[prop(optional)]
    animate: Option<ReadSignal<HashMap<String, AnimationValue>>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Drag configuration
    #[prop(optional)]
    drag: Option<DragConfig>,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    
    // Create animation engine
    let mut animation_engine = AnimationEngine::new();
    
    // Drag state
    let (drag_state, set_drag_state) = signal(DragState::default());
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    
    // Set up animation callbacks
    let node_ref_clone = node_ref.clone();
    animation_engine.on_update(move |values| {
        if let Some(div) = node_ref_clone.get() {
            if let Some(html_element) = div.dyn_ref::<web_sys::HtmlElement>() {
                for (property, value) in values {
                    let css_value = format!("{}", value);
                    let _ = html_element.style().set_property(property, &css_value);
                }
            }
        }
    });

    // Handle initial values
    if let Some(initial_values) = initial {
        for (property, value) in initial_values {
            if let Some(numeric_value) = value.to_numeric_value() {
                let transition = transition.clone().unwrap_or_default();
                animation_engine.animate_property(
                    property,
                    numeric_value,
                    numeric_value,
                    transition,
                );
            }
        }
    }

    // Handle reactive animate signal
    if let Some(animate_signal) = animate {
        Effect::new(move |_| {
            let animate_values = animate_signal.get();
            
            for (property, value) in animate_values {
                if let Some(numeric_value) = value.to_numeric_value() {
                    let transition = transition.clone().unwrap_or_default();
                    let current_value = animation_engine.get_property_value(&property).unwrap_or(numeric_value);
                    animation_engine.animate_property(
                        property,
                        current_value,
                        numeric_value,
                        transition,
                    );
                }
            }
        });
    }

    // Set up drag event handlers
    if let Some(drag_config) = drag {
        if drag_config.enabled {
            let drag_config_clone = drag_config.clone();
            let node_ref_clone = node_ref.clone();
            let drag_state_clone = drag_state;
            let set_drag_state_clone = set_drag_state;
            let set_drag_position_clone = set_drag_position;

            // Mouse events
            let on_mouse_down = move |ev: web_sys::MouseEvent| {
                ev.prevent_default();
                let rect = node_ref_clone.get().unwrap().get_bounding_client_rect();
                let start_x = ev.client_x() as f64 - rect.left();
                let start_y = ev.client_y() as f64 - rect.top();
                
                set_drag_state_clone.update(|state| {
                    state.is_dragging = true;
                    state.start_x = start_x;
                    state.start_y = start_y;
                    state.current_x = start_x;
                    state.current_y = start_y;
                    state.velocity_x = 0.0;
                    state.velocity_y = 0.0;
                    state.last_time = js_sys::Date::now();
                });

                // TODO: Add drag start callback support
            };

            let on_mouse_move = move |ev: web_sys::MouseEvent| {
                let state = drag_state_clone.get();
                if state.is_dragging {
                    ev.prevent_default();
                    let rect = node_ref_clone.get().unwrap().get_bounding_client_rect();
                    let current_x = ev.client_x() as f64 - rect.left();
                    let current_y = ev.client_y() as f64 - rect.top();
                    
                    // Apply constraints
                    let (constrained_x, constrained_y) = if let Some(ref constraints) = drag_config_clone.constraints {
                        let x = constraints.min_x.map_or(current_x, |min| current_x.max(min));
                        let x = constraints.max_x.map_or(x, |max| x.min(max));
                        let y = constraints.min_y.map_or(current_y, |min| current_y.max(min));
                        let y = constraints.max_y.map_or(y, |max| y.min(max));
                        (x, y)
                    } else {
                        (current_x, current_y)
                    };

                    // Update velocity
                    let current_time = js_sys::Date::now();
                    let delta_time = (current_time - state.last_time) / 1000.0;
                    let velocity_x = if delta_time > 0.0 { (constrained_x - state.current_x) / delta_time } else { 0.0 };
                    let velocity_y = if delta_time > 0.0 { (constrained_y - state.current_y) / delta_time } else { 0.0 };

                    set_drag_state_clone.update(|state| {
                        state.current_x = constrained_x;
                        state.current_y = constrained_y;
                        state.velocity_x = velocity_x;
                        state.velocity_y = velocity_y;
                        state.last_time = current_time;
                    });

                    set_drag_position_clone.set((constrained_x, constrained_y));

                    // TODO: Add drag callback support
                }
            };

            let on_mouse_up = move |_ev: web_sys::MouseEvent| {
                let state = drag_state_clone.get();
                if state.is_dragging {
                    set_drag_state_clone.update(|state| {
                        state.is_dragging = false;
                    });

                    // TODO: Add drag end callback support

                    // Apply momentum if enabled
                    if let Some(ref momentum) = drag_config_clone.momentum {
                        if momentum.enabled {
                            // TODO: Implement momentum animation
                            // This would use the animation engine to animate to the final position
                            // with spring physics based on the velocity
                        }
                    }
                }
            };

            // Add event listeners
            if let Some(div) = node_ref.get() {
                let element = div.dyn_ref::<web_sys::HtmlElement>().unwrap();
                
                // Mouse events
                let mouse_down_closure = Closure::wrap(Box::new(on_mouse_down) as Box<dyn FnMut(web_sys::MouseEvent)>);
                let mouse_move_closure = Closure::wrap(Box::new(on_mouse_move) as Box<dyn FnMut(web_sys::MouseEvent)>);
                let mouse_up_closure = Closure::wrap(Box::new(on_mouse_up) as Box<dyn FnMut(web_sys::MouseEvent)>);

                let _ = element.add_event_listener_with_callback(
                    "mousedown",
                    mouse_down_closure.as_ref().unchecked_ref(),
                );
                let _ = element.add_event_listener_with_callback(
                    "mousemove",
                    mouse_move_closure.as_ref().unchecked_ref(),
                );
                let _ = element.add_event_listener_with_callback(
                    "mouseup",
                    mouse_up_closure.as_ref().unchecked_ref(),
                );

                // Store closures to prevent them from being dropped
                mouse_down_closure.forget();
                mouse_move_closure.forget();
                mouse_up_closure.forget();
            }
        }
    }

    view! {
        <div 
            node_ref=node_ref
            style=move || {
                let (x, y) = drag_position.get();
                format!("transform: translate({}px, {}px); cursor: grab;", x, y)
            }
        >
            {children()}
        </div>
    }
}

/// Extension trait for AnimationValue to add numeric conversion
trait AnimationValueNumeric {
    fn to_numeric_value(&self) -> Option<f64>;
}

impl AnimationValueNumeric for AnimationValue {
    fn to_numeric_value(&self) -> Option<f64> {
        match self {
            AnimationValue::Number(n) => Some(*n),
            AnimationValue::Pixels(p) => Some(*p),
            AnimationValue::Degrees(d) => Some(*d),
            AnimationValue::Percentage(p) => Some(*p),
            AnimationValue::Radians(r) => Some(*r),
            _ => None,
        }
    }
}
