//! Simple animation test to verify that animations work

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

#[component]
pub fn SimpleAnimationTest() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);

    // Create a simple animation target
    let animation_target = move || {
        let mut target = HashMap::new();
        if is_visible.get() {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(1)".to_string()),
            );
            target.insert(
                "background-color".to_string(),
                AnimationValue::String("red".to_string()),
            );
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.5));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(0.8)".to_string()),
            );
            target.insert(
                "background-color".to_string(),
                AnimationValue::String("blue".to_string()),
            );
        }
        target
    };

    // Convert animation target to CSS styles
    let css_styles = move || {
        let target = animation_target();
        let mut styles = vec!["transition: all 0.6s ease-in-out".to_string()];

        for (key, value) in target {
            let css_value = match value {
                AnimationValue::Number(n) => n.to_string(),
                AnimationValue::String(s) => s,
                AnimationValue::Color(c) => c,
                AnimationValue::Pixels(p) => format!("{}px", p),
                AnimationValue::Percentage(p) => format!("{}%", p),
                AnimationValue::Degrees(d) => format!("{}deg", d),
                AnimationValue::Radians(r) => format!("{}rad", r),
                AnimationValue::Transform(t) => format!("{:?}", t),
                AnimationValue::Complex(_) => "complex".to_string(),
            };
            styles.push(format!("{}: {}", key, css_value));
        }

        styles.join("; ")
    };

    view! {
        <div style="padding: 2rem; text-align: center;">
            <h1>"Simple Animation Test"</h1>
            <button
                on:click=move |_| {
                    set_is_visible.set(!is_visible.get());
                }
                style="padding: 1rem 2rem; font-size: 1.2rem; margin: 1rem; background: #007bff; color: white; border: none; border-radius: 5px; cursor: pointer;"
            >
                {move || {
                    let visible = is_visible.get();
                    if visible { "Hide Animation" } else { "Show Animation" }
                }}
            </button>

            <div
                style=css_styles()
                style:padding="2rem"
                style:margin="2rem auto"
                style:border-radius="10px"
                style:color="white"
                style:font-size="1.5rem"
                style:font-weight="bold"
                style:text-align="center"
                style:min-width="200px"
                style:min-height="100px"
                style:display="flex"
                style:align-items="center"
                style:justify-content="center"
            >
                "Simple Animation Box"
            </div>
        </div>
    }
}
