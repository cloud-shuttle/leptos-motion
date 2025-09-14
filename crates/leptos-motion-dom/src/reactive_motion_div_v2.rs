//! Reactive MotionDiv v2 - Properly integrated with animation engine
//!
//! This module provides a MotionDiv component that properly integrates with the
//! animation engine and handles reactive updates correctly.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys;

use crate::animation_engine::AnimationEngine;

/// Reactive MotionDiv that properly integrates with the animation engine
#[component]
pub fn ReactiveMotionDivV2(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    /// Animate signal that triggers reactive updates
    #[prop(optional)]
    animate: Option<ReadSignal<HashMap<String, AnimationValue>>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    
    // Create animation engine
    let mut animation_engine = AnimationEngine::new();
    
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
                    numeric_value, // Start and end at same value for initial
                    transition,
                );
            }
        }
    }

    // Start the animation engine
    animation_engine.start_animation_loop();

    // Handle reactive animate signal
    if let Some(animate_signal) = animate {
        Effect::new(move |_| {
            let animate_values = animate_signal.get();
            
            // Apply values directly to DOM for string properties like transform
            if let Some(div) = node_ref.get() {
                if let Some(html_element) = div.dyn_ref::<web_sys::HtmlElement>() {
                    for (property, value) in animate_values {
                        let css_value = match value {
                            AnimationValue::String(s) => s,
                            AnimationValue::Number(n) => n.to_string(),
                            AnimationValue::Pixels(p) => format!("{}px", p),
                            AnimationValue::Degrees(d) => format!("{}deg", d),
                            AnimationValue::Percentage(p) => format!("{}%", p),
                            AnimationValue::Radians(r) => format!("{}rad", r),
                            AnimationValue::Color(c) => c,
                            AnimationValue::Transform(t) => format!("{:?}", t),
                            AnimationValue::Complex(c) => format!("{:?}", c),
                        };
                        let _ = html_element.style().set_property(&property, &css_value);
                    }
                }
            }
        });
    }

    view! {
        <div node_ref=node_ref>
            {children()}
        </div>
    }
}

/// Helper function to convert AnimationValue to numeric value
fn animation_value_to_numeric(value: &AnimationValue) -> Option<f64> {
    match value {
        AnimationValue::Number(n) => Some(*n),
        AnimationValue::Pixels(p) => Some(*p),
        AnimationValue::Degrees(d) => Some(*d),
        AnimationValue::Percentage(p) => Some(*p),
        AnimationValue::Radians(r) => Some(*r),
        _ => None,
    }
}

/// Extension trait for AnimationValue to add numeric conversion
trait AnimationValueNumeric {
    fn to_numeric_value(&self) -> Option<f64>;
}

impl AnimationValueNumeric for AnimationValue {
    fn to_numeric_value(&self) -> Option<f64> {
        animation_value_to_numeric(self)
    }
}
