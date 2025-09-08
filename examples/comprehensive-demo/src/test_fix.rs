//! Test file to verify the animation reactivity fix

use leptos::prelude::*;
use leptos_motion_dom::MotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
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
            target.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
            target.insert("backgroundColor".to_string(), AnimationValue::String("red".to_string()));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.5));
            target.insert("transform".to_string(), AnimationValue::String("scale(0.8)".to_string()));
            target.insert("backgroundColor".to_string(), AnimationValue::String("blue".to_string()));
        }
        target
    };

    let animate_animation = move || {
        let target = create_animation_target(is_visible.get());
        console::log_1(&format!("Animation target: {:?}", target).into());
        target
    };

    let transition = Transition {
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
            
            <MotionDiv
                initial=create_animation_target(true)
                animate=animate_animation()
                transition=transition
                style="
                    padding: 2rem;
                    margin: 2rem auto;
                    border-radius: 10px;
                    color: white;
                    font-size: 1.5rem;
                    font-weight: bold;
                    text-align: center;
                    min-width: 200px;
                    min-height: 100px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ".to_string()
            >
                "Test Animation Box"
            </MotionDiv>
        </div>
    }
}

// Main function is in lib.rs
