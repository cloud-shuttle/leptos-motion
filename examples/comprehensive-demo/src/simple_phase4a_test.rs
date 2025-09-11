//! Simple Phase 4A Test
//!
//! A minimal test to isolate the rendering issue

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue};
use std::collections::HashMap;

use leptos_motion_dom::ReactiveMotionDiv;

/// Helper function to create a simple animation target
fn create_animation_target(key: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(key.to_string(), AnimationValue::Number(value));
    target
}

#[component]
pub fn SimplePhase4aTest() -> impl IntoView {
    web_sys::console::log_1(&"🎯 Simple Phase 4A Test: Component created".into());

    // Create a simple function-based animation
    let function_target = Box::new(|| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(1.2)".to_string()),
        );
        target
    });

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1 style="color: #333;">"🎯 Simple Phase 4A Test"</h1>
            <p style="color: #666; margin-bottom: 20px;">"Testing function-based props with ReactiveMotionDiv"</p>

            <div style="display: flex; gap: 20px; align-items: center;">
                <div>
                    <h3>"Static Animation"</h3>
                    <ReactiveMotionDiv
                        initial=create_animation_target("opacity", 0.5)
                        animate=create_animation_target("opacity", 1.0)
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "Static"
                    </ReactiveMotionDiv>
                </div>

                <div>
                    <h3>"Function Animation"</h3>
                    <ReactiveMotionDiv
                        animate_fn=function_target
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #a8e6cf, #ffd3a5); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "Function"
                    </ReactiveMotionDiv>
                </div>
            </div>

            <div style="margin-top: 20px; padding: 15px; background: #f0f0f0; border-radius: 8px;">
                <h3>"Test Results"</h3>
                <p>"If you can see this text and the colored boxes above, the component is rendering correctly."</p>
                <p>"The 'Static' box should have opacity 1.0, and the 'Function' box should have opacity 0.8 and scale 1.2."</p>
            </div>
        </div>
    }
}
