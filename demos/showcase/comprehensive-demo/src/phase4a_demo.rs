//! Phase 4A Demo: Function-based Props
//!
//! Demonstrates the new function-based animation props with Box<dyn Fn() + Send + Sync>

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

/// Helper function to create a function-based animation target
fn create_function_animation_target() -> Box<dyn Fn() -> AnimationTarget + Send + Sync> {
    Box::new(|| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(1.2)".to_string()),
        );
        target
    })
}

/// Helper function to create a dynamic animation target based on time
fn create_dynamic_animation_target() -> Box<dyn Fn() -> AnimationTarget + Send + Sync> {
    Box::new(|| {
        // Use a simple counter-based animation instead of time for WASM compatibility
        let opacity = 0.7;
        let scale = 1.1;

        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(opacity));
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", scale)),
        );
        target
    })
}

#[component]
pub fn Phase4aDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 Phase 4A Demo: Function-based Props".into());

    let (counter, set_counter) = signal(0);

    // Create function-based animation targets
    let function_target = create_function_animation_target();
    let dynamic_target = create_dynamic_animation_target();

    // Create a counter-based animation function
    let counter_animation = Box::new(move || {
        let count = counter.get();
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("rotate({}deg)", count * 10)),
        );
        target.insert(
            "background-color".to_string(),
            AnimationValue::String(format!("hsl({}, 70%, 50%)", count * 20)),
        );
        target
    });

    // Create hover animation function
    let hover_animation = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "background-color".to_string(),
            AnimationValue::String("red".to_string()),
        );
        target.insert(
            "transform".to_string(),
            AnimationValue::String("rotate(5deg)".to_string()),
        );
        target
    });

    // Create tap animation function
    let tap_animation = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(0.95)".to_string()),
        );
        target.insert(
            "box-shadow".to_string(),
            AnimationValue::String("0 4px 8px rgba(0,0,0,0.3)".to_string()),
        );
        target
    });

    // Clone functions for reuse
    let function_target_clone = create_function_animation_target();
    let hover_animation_clone = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "background-color".to_string(),
            AnimationValue::String("red".to_string()),
        );
        target.insert(
            "transform".to_string(),
            AnimationValue::String("rotate(5deg)".to_string()),
        );
        target
    });

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: #f0f0f0; min-height: 100vh;">
            <h1 style="color: #333; text-align: center;">"🎯 Phase 4A: Function-based Props Demo"</h1>
            <p style="text-align: center; font-size: 18px; margin-bottom: 30px;">
                "Demonstrating dynamic animation targets with Box<dyn Fn() + Send + Sync>"
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; max-width: 1200px; margin: 0 auto;">

                // Static Function-based Animation
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Static Function Animation"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Uses a function that returns a static animation target"</p>
                    <ReactiveMotionDiv
                        animate_fn=function_target
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Static"
                    </ReactiveMotionDiv>
                </div>

                // Dynamic Time-based Animation
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Dynamic Time Animation"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Uses a function that calculates animation based on current time"</p>
                    <ReactiveMotionDiv
                        animate_fn=dynamic_target
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #a8e6cf, #ffd3a5); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Time"
                    </ReactiveMotionDiv>
                </div>

                // Counter-based Animation
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Counter Animation"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Uses a function that responds to signal changes"</p>
                    <div style="margin-bottom: 10px;">
                        <button
                            style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; margin-right: 10px;"
                            on:click=move |_| set_counter.update(|c| *c += 1)
                        >
                            "Increment"
                        </button>
                        <span style="font-weight: bold;">"Counter: " {counter}</span>
                    </div>
                    <ReactiveMotionDiv
                        animate_fn=counter_animation
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff9a9e, #fecfef); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Count"
                    </ReactiveMotionDiv>
                </div>

                // Hover Animation
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Hover Animation"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Uses a function for hover state animation"</p>
                    <ReactiveMotionDiv
                        _while_hover_fn=hover_animation
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #667eea, #764ba2); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Hover"
                    </ReactiveMotionDiv>
                </div>

                // Tap Animation
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Tap Animation"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Uses a function for tap state animation"</p>
                    <ReactiveMotionDiv
                        _while_tap_fn=tap_animation
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #f093fb, #f5576c); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Tap"
                    </ReactiveMotionDiv>
                </div>

                // Mixed Static and Function Props
                <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0;">"Mixed Props"</h3>
                    <p style="color: #666; margin-bottom: 15px;">"Combines static initial with function-based animate"</p>
                    <ReactiveMotionDiv
                        initial=create_animation_target("opacity", 0.3)
                        animate_fn=function_target_clone
                        _while_hover_fn=hover_animation_clone
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #4facfe, #00f2fe); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    >
                        "Mixed"
                    </ReactiveMotionDiv>
                </div>
            </div>

            <div style="text-align: center; margin-top: 30px; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); max-width: 800px; margin-left: auto; margin-right: auto;">
                <h3 style="color: #333; margin-top: 0;">"✅ Phase 4A Features"</h3>
                <ul style="text-align: left; color: #666; line-height: 1.6;">
                    <li>"<strong>Function-based Props:</strong> animate_fn, while_hover_fn, while_tap_fn"</li>
                    <li>"<strong>Thread Safety:</strong> Box<dyn Fn() + Send + Sync> ensures safe concurrent access"</li>
                    <li>"<strong>Dynamic Targets:</strong> Functions can calculate animation values at runtime"</li>
                    <li>"<strong>Signal Integration:</strong> Functions can access and respond to Leptos signals"</li>
                    <li>"<strong>Mixed Usage:</strong> Can combine static and function-based props"</li>
                </ul>
            </div>
        </div>
    }
}
