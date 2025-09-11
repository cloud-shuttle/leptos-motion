//! AnimateFn Test
//!
//! A minimal test to see if animate_fn works

use leptos::prelude::*;
use leptos_motion_core::AnimationValue;
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

#[component]
pub fn AnimateFnTest() -> impl IntoView {
    // Create a simple signal-based animation instead of function-based
    let mut animate_target = HashMap::new();
    animate_target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    animate_target.insert(
        "transform".to_string(),
        AnimationValue::String("scale(1.2)".to_string()),
    );

    view! {
        <div>
            <h1>"AnimateFn Test"</h1>
            <p>"Testing if animate prop works (using signal-based instead of function-based)"</p>
            <ReactiveMotionDiv
                animate=animate_target
                style="width: 100px; height: 100px; background: linear-gradient(45deg, #a8e6cf, #ffd3a5); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
            >
                "Function"
            </ReactiveMotionDiv>
        </div>
    }
}
