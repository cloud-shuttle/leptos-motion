//! TransitionConfig Test
//!
//! TDD for Phase 4B: Add TransitionConfig for timing controls

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, Transition};
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

#[component]
pub fn TransitionConfigTest() -> impl IntoView {
    // Create animation targets with different timing configurations
    let mut fast_animate = HashMap::new();
    fast_animate.insert("opacity".to_string(), AnimationValue::Number(0.5));
    fast_animate.insert(
        "transform".to_string(),
        AnimationValue::String("scale(0.8)".to_string()),
    );

    let mut slow_animate = HashMap::new();
    slow_animate.insert("opacity".to_string(), AnimationValue::Number(0.9));
    slow_animate.insert(
        "transform".to_string(),
        AnimationValue::String("scale(1.1)".to_string()),
    );

    // Create transition configurations
    let fast_transition = Transition {
        duration: Some(0.1), // 100ms
        delay: Some(0.0),
        ease: Easing::EaseOut,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    };

    let slow_transition = Transition {
        duration: Some(2.0), // 2000ms
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div>
            <h1>"TransitionConfig Test"</h1>
            <p>"Testing timing controls for animations"</p>

            <div style="display: flex; gap: 20px; margin: 20px 0;">
                <div>
                    <h3>"Fast Animation (100ms)"</h3>
                    <ReactiveMotionDiv
                        animate=fast_animate
                        transition=fast_transition
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ffa500); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "Fast"
                    </ReactiveMotionDiv>
                </div>

                <div>
                    <h3>"Slow Animation (2000ms)"</h3>
                    <ReactiveMotionDiv
                        animate=slow_animate
                        transition=slow_transition
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #4ecdc4, #44a08d); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "Slow"
                    </ReactiveMotionDiv>
                </div>
            </div>
        </div>
    }
}
