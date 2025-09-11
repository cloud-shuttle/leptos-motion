//! Batched Updates Test
//!
//! TDD for Phase 5B: Add batched DOM updates with requestAnimationFrame

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, Transition};
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

#[component]
pub fn BatchedUpdatesTest() -> impl IntoView {
    // Create multiple signals that change rapidly to test batching
    let (counter1, set_counter1) = signal(0);
    let (counter2, set_counter2) = signal(0);
    let (counter3, set_counter3) = signal(0);
    let (is_animating, set_is_animating) = signal(false);

    // Create animation targets that will be batched
    let animate1 = Memo::new(move |_| {
        let mut animate = HashMap::new();
        animate.insert(
            "opacity".to_string(),
            AnimationValue::Number(0.3 + (counter1.get() as f64 * 0.1)),
        );
        animate.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", 0.5 + (counter1.get() as f64 * 0.05))),
        );
        animate
    });

    let animate2 = Memo::new(move |_| {
        let mut animate = HashMap::new();
        animate.insert(
            "opacity".to_string(),
            AnimationValue::Number(0.4 + (counter2.get() as f64 * 0.1)),
        );
        animate.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", 0.6 + (counter2.get() as f64 * 0.05))),
        );
        animate
    });

    let animate3 = Memo::new(move |_| {
        let mut animate = HashMap::new();
        animate.insert(
            "opacity".to_string(),
            AnimationValue::Number(0.5 + (counter3.get() as f64 * 0.1)),
        );
        animate.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", 0.7 + (counter3.get() as f64 * 0.05))),
        );
        animate
    });

    // Create a shared transition for all elements
    let shared_transition = Transition {
        duration: Some(0.3),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    };

    // Function to trigger rapid updates (simulating batched updates)
    let trigger_rapid_updates = move |_| {
        set_is_animating.set(true);

        // Simulate rapid updates that should be batched
        for i in 0..10 {
            let delay = i * 50; // 50ms between updates
            let i = i;
            set_timeout(
                move || {
                    set_counter1.update(|c| *c += 1);
                    set_counter2.update(|c| *c += 1);
                    set_counter3.update(|c| *c += 1);

                    if i == 9 {
                        set_is_animating.set(false);
                    }
                },
                std::time::Duration::from_millis(delay),
            );
        }
    };

    // Function to reset all counters
    let reset_counters = move |_| {
        set_counter1.set(0);
        set_counter2.set(0);
        set_counter3.set(0);
    };

    view! {
        <div>
            <h1>"Batched Updates Test"</h1>
            <p>"Testing batched DOM updates with requestAnimationFrame for performance optimization"</p>

            <div style="margin: 20px 0;">
                <button
                    on:click=trigger_rapid_updates
                    disabled=is_animating
                    style="margin-right: 10px; padding: 8px 16px; background: #007acc; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {move || if is_animating.get() { "Animating..." } else { "Start Rapid Updates" }}
                </button>

                <button
                    on:click=reset_counters
                    style="padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Reset Counters"
                </button>
            </div>

            <div style="margin: 20px 0;">
                <p>"Counter 1: " {counter1} " | Counter 2: " {counter2} " | Counter 3: " {counter3}</p>
                <p>"Status: " {move || if is_animating.get() { "Animating (batched updates)" } else { "Ready" }}</p>
            </div>

            <div style="display: flex; gap: 20px; margin: 20px 0;">
                <div>
                    <h3>"Element 1"</h3>
                    <ReactiveMotionDiv
                        animate=animate1.get()
                        transition=shared_transition.clone()
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ffa500); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "1"
                    </ReactiveMotionDiv>
                </div>

                <div>
                    <h3>"Element 2"</h3>
                    <ReactiveMotionDiv
                        animate=animate2.get()
                        transition=shared_transition.clone()
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #4ecdc4, #44a08d); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "2"
                    </ReactiveMotionDiv>
                </div>

                <div>
                    <h3>"Element 3"</h3>
                    <ReactiveMotionDiv
                        animate=animate3.get()
                        transition=shared_transition.clone()
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #a8e6cf, #ffd3a5); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                    >
                        "3"
                    </ReactiveMotionDiv>
                </div>
            </div>

            <div style="margin: 20px 0; padding: 15px; background: #f8f9fa; border-radius: 8px; border-left: 4px solid #007acc;">
                <h4>"Performance Note"</h4>
                <p>"This test demonstrates batched DOM updates. When you click 'Start Rapid Updates', multiple animation properties change rapidly. The requestAnimationFrame batching ensures these updates are synchronized with the browser's refresh rate for optimal performance."</p>
            </div>
        </div>
    }
}
