//! Simple Signal-Based MotionDiv Component
//!
//! This component implements the proven patterns from the user's guide for
//! proper signal tracking, WASM memory management, and effect dependencies.
//! Simplified version without complex WASM bindings.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Simple signal-based MotionDiv component that properly tracks all signal changes
#[component]
pub fn SimpleSignalBasedMotionDiv(
    /// Initial animation values
    initial: HashMap<String, AnimationValue>,
    /// Animate function that returns animation values
    animate: impl Fn() -> HashMap<String, AnimationValue> + 'static,
    /// CSS style string
    #[prop(optional)]
    style: Option<String>,
    /// Children to render
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Effect that properly tracks animate function changes
    Effect::new(move |_| {
        let animate_values = animate(); // Call the animate function

        if let Some(div) = node_ref.get() {
            // Apply animation to DOM element
            for (property, value) in animate_values {
                let css_value = animation_value_to_css(&value);
                if let Err(e) = div
                    .unchecked_ref::<web_sys::HtmlElement>()
                    .style()
                    .set_property(&property, &css_value)
                {
                    web_sys::console::error_1(
                        &format!("Failed to set CSS property {}: {:?}", property, e).into(),
                    );
                }
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            style=move || {
                let mut css = animation_values_to_css(&initial);
                if let Some(provided_style) = &style {
                    css.push_str("; ");
                    css.push_str(provided_style);
                }
                css
            }
        >
            {children()}
        </div>
    }
}

/// Helper function to convert animation values to CSS
pub fn animation_value_to_css(value: &AnimationValue) -> String {
    match value {
        AnimationValue::Pixels(val) => format!("{}px", val),
        AnimationValue::Number(val) => val.to_string(),
        AnimationValue::Degrees(val) => format!("{}deg", val),
        AnimationValue::Percentage(val) => format!("{}%", val),
        AnimationValue::Radians(val) => format!("{}rad", val),
        AnimationValue::Color(val) => val.clone(),
        AnimationValue::String(val) => val.clone(),
        AnimationValue::Transform(_) => "matrix(1,0,0,1,0,0)".to_string(),
        AnimationValue::Complex(_) => "0".to_string(),
    }
}

/// Helper function to convert animation values to CSS for a HashMap
pub fn animation_values_to_css(values: &HashMap<String, AnimationValue>) -> String {
    values
        .iter()
        .map(|(key, value)| format!("{}: {}", key, animation_value_to_css(value)))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Reactive MotionDiv that uses proper signal tracking patterns
#[component]
pub fn ReactiveSimpleMotionDiv(
    /// Animate signal that triggers reactive updates
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    /// Whether the component is visible
    is_visible: ReadSignal<bool>,
    /// Children to render
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Effect with explicit dependencies
    Effect::new(move |_| {
        // This effect will re-run when ANY of these signals change:
        let animate_values = animate.get(); // Dependency 1
        let visible = is_visible.get(); // Dependency 2

        if visible {
            if let Some(div) = node_ref.get() {
                // Apply animation to DOM element
                for (property, value) in animate_values {
                    let css_value = animation_value_to_css(&value);
                    if let Err(e) = div
                        .unchecked_ref::<web_sys::HtmlElement>()
                        .style()
                        .set_property(&property, &css_value)
                    {
                        web_sys::console::error_1(
                            &format!("Failed to set CSS property {}: {:?}", property, e).into(),
                        );
                    }
                }
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || if is_visible.get() { "visible" } else { "hidden" }
        >
            {children()}
        </div>
    }
}
