//! TDD Test for reactive style application
//!
//! This test demonstrates the expected behavior:
//! 1. A signal controls the animation state
//! 2. When the signal changes, the DOM element's style should update
//! 3. The visual properties (opacity, background-color, transform) should change

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue};
use std::collections::HashMap;

#[component]
pub fn ReactiveStyleTest() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);

    // Create animation target based on signal state
    let animation_target = move || {
        let mut target = HashMap::new();
        if is_visible.get() {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "background-color".to_string(),
                AnimationValue::String("red".to_string()),
            );
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(1)".to_string()),
            );
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.5));
            target.insert(
                "background-color".to_string(),
                AnimationValue::String("blue".to_string()),
            );
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(0.8)".to_string()),
            );
        }
        target
    };

    view! {
        <div style="padding: 2rem;">
            <h1>"Reactive Style Test (TDD)"</h1>
            <button on:click=move |_| {
                let new_state = !is_visible.get();
                set_is_visible.set(new_state);
                web_sys::console::log_1(&format!("Button clicked! State: {}", new_state).into());
            }>
                {move || if is_visible.get() { "Hide" } else { "Show" }}
            </button>

            // This is the test element - it should update its styles when the signal changes
            <div
                id="test-element"
                style=move || {
                    let target = animation_target();
                    let mut style_parts = vec![
                        "padding: 2rem".to_string(),
                        "margin: 2rem auto".to_string(),
                        "border-radius: 10px".to_string(),
                        "color: white".to_string(),
                        "font-size: 1.5rem".to_string(),
                        "font-weight: bold".to_string(),
                        "text-align: center".to_string(),
                        "min-width: 200px".to_string(),
                        "min-height: 100px".to_string(),
                        "display: flex".to_string(),
                        "align-items: center".to_string(),
                        "justify-content: center".to_string(),
                        "transition: all 0.6s ease-in-out".to_string(),
                    ];

                    // Add animation styles from the target
                    for (key, value) in target {
                        style_parts.push(format!("{}: {}", key, value.to_string_value()));
                    }

                    let style_string = style_parts.join("; ");
                    web_sys::console::log_1(&format!("Generated style: {}", style_string).into());
                    style_string
                }
            >
                "Reactive Animation Test"
            </div>
        </div>
    }
}
