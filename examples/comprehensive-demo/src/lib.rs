//! Simple Demo - One Leptos Motion Capability
//!
//! Demonstrating basic animation with MotionDiv

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use leptos_motion_dom::MotionDiv;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod test_fix;
use test_fix::TestFix;

/// Main demo app component
#[component]
pub fn DemoApp() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);
    let (animation_mode, set_animation_mode) = signal(0);
    let (hover_state, set_hover_state) = signal(false);

    // Create animation targets for different modes
    let create_animation_target = |visible: bool, mode: i32| -> AnimationTarget {
        let mut target = HashMap::new();

        match mode {
            0 => {
                // Scale & Fade
                target.insert(
                    "opacity".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.0 }),
                );
                target.insert(
                    "scale".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.3 }),
                );
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(if visible {
                        "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string()
                    } else {
                        "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string()
                    }),
                );
            }
            1 => {
                // Slide & Rotate
                target.insert(
                    "opacity".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.8 }),
                );
                target.insert(
                    "x".to_string(),
                    AnimationValue::Pixels(if visible { 0.0 } else { -200.0 }),
                );
                target.insert(
                    "rotate".to_string(),
                    AnimationValue::Degrees(if visible { 0.0 } else { -180.0 }),
                );
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(if visible {
                        "linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)".to_string()
                    } else {
                        "linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)".to_string()
                    }),
                );
            }
            2 => {
                // Bounce & Color
                target.insert(
                    "opacity".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.6 }),
                );
                target.insert(
                    "scale".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.8 }),
                );
                target.insert(
                    "y".to_string(),
                    AnimationValue::Pixels(if visible { 0.0 } else { -50.0 }),
                );
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(if visible {
                        "linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string()
                    } else {
                        "linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)".to_string()
                    }),
                );
            }
            _ => {
                target.insert(
                    "opacity".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.3 }),
                );
                target.insert(
                    "scale".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.3 }),
                );
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(if visible {
                        "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string()
                    } else {
                        "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string()
                    }),
                );
            }
        }

        target
    };

    // Create reactive animation targets
    let initial_animation = move || create_animation_target(true, animation_mode.get());
    let animate_animation = move || create_animation_target(is_visible.get(), animation_mode.get());

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.6),
        ease: Easing::EaseInOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Create hover animation for buttons
    let button_hover_animation = move || {
        let mut target = HashMap::new();
        target.insert("y".to_string(), AnimationValue::Pixels(-2.0));
        target.insert(
            "boxShadow".to_string(),
            AnimationValue::String("0 8px 25px rgba(102, 126, 234, 0.4)".to_string()),
        );
        target
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 2rem; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
            <div style="max-width: 800px; margin: 0 auto; text-align: center;">
                <div style="margin-bottom: 3rem;">
                    <h1 style="color: white; font-size: 3rem; font-weight: 700; margin-bottom: 1rem; text-shadow: 0 2px 4px rgba(0,0,0,0.3);">
                        "✨ Leptos Motion"
                    </h1>
                    <p style="color: rgba(255,255,255,0.9); font-size: 1.2rem; margin-bottom: 2rem;">
                        "Beautiful reactive animations with Rust & WebAssembly"
                    </p>
                </div>

                <div style="background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); border-radius: 20px; padding: 2rem; margin-bottom: 2rem; border: 1px solid rgba(255,255,255,0.2);">
                    <h2 style="color: white; margin-bottom: 1.5rem; font-size: 1.5rem;">"Animation Modes"</h2>
                    <div style="display: flex; gap: 1rem; justify-content: center; margin-bottom: 2rem; flex-wrap: wrap;">
                                    <MotionDiv
                                        class="mode-button".to_string()
                                        on:click=move |_| set_animation_mode.set(0)
                                        _while_hover=button_hover_animation()
                        style=(move || {
                            let active = animation_mode.get() == 0;
                            if active {
                                "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer; transform: scale(1.05);".to_string()
                            } else {
                                "background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.3); padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer;".to_string()
                            }
                        })()
                                    >
                                        "Scale & Fade"
                                    </MotionDiv>
                        <button
                            class="mode-button"
                            on:click=move |_| set_animation_mode.set(1)
                            style=move || {
                                let active = animation_mode.get() == 1;
                                if active {
                                    "background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); color: white; border: none; padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer; transform: scale(1.05);"
                                } else {
                                    "background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.3); padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer;"
                                }
                            }
                        >
                            "Slide & Rotate"
                        </button>
                        <button
                            class="mode-button"
                            on:click=move |_| set_animation_mode.set(2)
                            style=move || {
                                let active = animation_mode.get() == 2;
                                if active {
                                    "background: linear-gradient(135deg, #fa709a 0%, #fee140 100%); color: white; border: none; padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer; transform: scale(1.05);"
                                } else {
                                    "background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.3); padding: 0.8rem 1.5rem; border-radius: 25px; font-weight: 600; cursor: pointer;"
                                }
                            }
                        >
                            "Bounce & Color"
                        </button>
                    </div>
                </div>

                <div style="margin-bottom: 2rem;">
                    <button
                        class="main-button"
                        on:click=move |_| {
                            let current = is_visible.get();
                            console::log_1(&format!("Button clicked! Current state: {}, setting to: {}", current, !current).into());
                            set_is_visible.set(!current);
                        }
                        style="padding: 1.2rem 2.5rem; font-size: 1.3rem; font-weight: 700; border: none; border-radius: 50px; color: white; cursor: pointer; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);"
                    >
                        {move || {
                            let visible = is_visible.get();
                            if visible { "🎭 Hide Animation" } else { "✨ Show Animation" }
                        }}
                    </button>
                </div>

                <div style="display: flex; justify-content: center; align-items: center; min-height: 200px;">
                                <MotionDiv
                                    class="animated-box".to_string()
                                    initial=initial_animation()
                                    animate=animate_animation()
                                    transition=transition
                                    style="
                                        padding: 2rem;
                                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                        color: white;
                                        border-radius: 20px;
                                        font-size: 1.5rem;
                                        font-weight: 700;
                                        text-align: center;
                                        box-shadow: 0 8px 32px rgba(0,0,0,0.3);
                                        backdrop-filter: blur(10px);
                                        border: 1px solid rgba(255,255,255,0.2);
                                        min-width: 300px;
                                        min-height: 150px;
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                    ".to_string()
                >
                        {move || {
                            let mode = animation_mode.get();
                            match mode {
                                0 => "🎨 Scale & Fade Animation",
                                1 => "🌀 Slide & Rotate Animation",
                                2 => "🎪 Bounce & Color Animation",
                                _ => "✨ Default Animation"
                            }
                        }}
                    </MotionDiv>
                </div>

                <div style="margin-top: 3rem; color: rgba(255,255,255,0.8); font-size: 0.9rem;">
                    <p>"Built with Rust, Leptos, and WebAssembly"</p>
                    <p>"Reactive animations that respond to state changes"</p>
                </div>
            </div>
        </div>
    }
}

/// Main function to run the demo
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("Failed to initialize console log");

    mount_to_body(|| view! { <TestFix/> })
}
