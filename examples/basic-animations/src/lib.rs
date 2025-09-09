//! Basic animations example using manual DOM manipulation
//!
//! This example demonstrates how to create smooth animations without
//! requiring the MotionDiv component

use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

/// Simple working animation example
#[component]
pub fn App() -> impl IntoView {
    let (animated, set_animated) = signal(false);

    view! {
        <div style="min-height: 100vh; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 2rem;">
            <h1>"Leptos Motion - Basic Animations"</h1>

            <button
                class="animation-button"
                style=move || format!(
                    "padding: 1rem 2rem; font-size: 1.2rem; background-color: #3b82f6; color: white; border: none; border-radius: 0.5rem; cursor: pointer; transition: all 0.3s ease; transform: scale({}); opacity: {};",
                    if animated.get() { 1.1 } else { 1.0 },
                    if animated.get() { 0.8 } else { 1.0 }
                )
                on:click=move |_| set_animated.update(|a| *a = !*a)
            >
                {move || if animated.get() { "Reset Animation" } else { "Start Animation" }}
            </button>

            <div
                class="animated-box"
                style=move || format!(
                    "width: 100px; height: 100px; background-color: #ef4444; border-radius: 1rem; transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1); transform: translateX({}) rotate({}deg) scale({});",
                    if animated.get() { "200px" } else { "0px" },
                    if animated.get() { 180 } else { 0 },
                    if animated.get() { 1.5 } else { 1.0 }
                )
            >
            </div>

            <p class="description" style="max-width: 600px; text-align: center; color: #666;">
                "This example shows a working animation using CSS transitions. "
                "The leptos-motion-core library provides the foundation for more advanced animations."
            </p>
        </div>
    }
}

#[component]
pub fn SimpleAnimation() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    
    view! {
        <div>
            <h2>"Leptos Motion Core Engine Demo"</h2>
            <p>"Animation engine initialized successfully!"</p>
            
            <MotionDiv
                class="w-20 h-20 bg-blue-500 rounded-lg cursor-pointer".to_string()
                animate=HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(if is_visible.get() { 1.0 } else { 0.5 })),
                    ("scale".to_string(), AnimationValue::Number(if is_visible.get() { 1.2 } else { 1.0 }))
                ])
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
                on:click=move |_| set_is_visible.update(|x| *x = !*x)
            >
                "Click me!"
            </MotionDiv>
        </div>
    }
}
