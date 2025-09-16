//! Test file to verify the animation reactivity fix

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use leptos_motion_dom::{AnimationTargetOrReactive, MotionDiv};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[component]
pub fn TestFix() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);

    let create_animation_target = |visible: bool| -> AnimationTarget {
        let mut target = HashMap::new();
        if visible {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(1)".to_string()),
            );
            target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("red".to_string()),
            );
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.5));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(0.8)".to_string()),
            );
            target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("blue".to_string()),
            );
        }
        target
    };

    // Create a memo that will react to signal changes
    let animate_animation = move || {
        let target = create_animation_target(is_visible.get());
        console::log_1(&format!("Animation target: {:?}", target).into());
        target
    };

    let _transition = Transition {
        duration: Some(0.6),
        ease: Easing::EaseInOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div style="padding: 2rem; text-align: center;">
            <h1>"Animation Reactivity Test"</h1>
            <button
                on:click=move |_| {
                    let current = is_visible.get();
                    console::log_1(&format!("Button clicked! Current state: {}, setting to: {}", current, !current).into());
                    set_is_visible.set(!current);
                }
                style="padding: 1rem 2rem; font-size: 1.2rem; margin: 1rem; background: #007bff; color: white; border: none; border-radius: 5px; cursor: pointer;"
            >
                {move || {
                    let visible = is_visible.get();
                    if visible { "Hide Animation" } else { "Show Animation" }
                }}
            </button>

            <div
                style=move || {
                    let animate_target = animate_animation();
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

                    // Add animation styles
                    for (key, value) in animate_target {
                        style_parts.push(format!("{}: {}", key, value.to_string_value()));
                    }

                    style_parts.join("; ")
                }
            >
                "Test Animation Box"
            </div>
        </div>
    }
}

// Main function is in lib.rs
