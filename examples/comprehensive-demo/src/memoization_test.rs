//! Memoization Test
//!
//! TDD for Phase 5A: Implement memoization with create_memo

use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, Transition};
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

#[component]
pub fn MemoizationTest() -> impl IntoView {
    // Create a signal that changes to test memoization
    let (counter, set_counter) = signal(0);
    let (expensive_value, set_expensive_value) = signal(0.0);

    // Memoized animation target that only recalculates when counter changes
    let memoized_animate = Memo::new(move |_| {
        // Simulate expensive calculation
        let base_value = counter.get() as f64 * 0.1;
        let mut animate = HashMap::new();
        animate.insert(
            "opacity".to_string(),
            AnimationValue::Number(0.5 + base_value),
        );
        animate.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", 0.8 + base_value)),
        );
        animate
    });

    // Memoized transition that only recalculates when expensive_value changes
    let memoized_transition = Memo::new(move |_| {
        let duration = 0.5 + (expensive_value.get() * 0.1);
        Transition {
            duration: Some(duration),
            delay: Some(0.0),
            ease: Easing::EaseInOut,
            repeat: leptos_motion_core::RepeatConfig::Never,
            stagger: None,
        }
    });

    // Memoized style calculation
    let memoized_style = Memo::new(move |_| {
        let hue = (counter.get() * 30) % 360;
        format!(
            "width: 100px; height: 100px; background: linear-gradient(45deg, hsl({}, 70%, 60%), hsl({}, 70%, 40%)); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;",
            hue,
            hue + 60
        )
    });

    view! {
        <div>
            <h1>"Memoization Test"</h1>
            <p>"Testing memoization with create_memo for performance optimization"</p>

            <div style="margin: 20px 0;">
                <button
                    on:click=move |_| set_counter.update(|c| *c += 1)
                    style="margin-right: 10px; padding: 8px 16px; background: #007acc; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Increment Counter: " {counter}
                </button>

                <button
                    on:click=move |_| set_expensive_value.update(|v| *v += 0.1)
                    style="padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Update Expensive Value: " {move || format!("{:.1}", expensive_value.get())}
                </button>
            </div>

            <div style="display: flex; gap: 20px; margin: 20px 0;">
                <div>
                    <h3>"Memoized Animation"</h3>
                    <p>"Counter: " {counter} " | Expensive: " {move || format!("{:.1}", expensive_value.get())}</p>
                    <ReactiveMotionDiv
                        animate=memoized_animate.get()
                        transition=memoized_transition.get()
                        style=memoized_style.get()
                    >
                        "Memoized"
                    </ReactiveMotionDiv>
                </div>

                <div>
                    <h3>"Non-Memoized (for comparison)"</h3>
                    <p>"This recalculates on every render"</p>
                    <ReactiveMotionDiv
                        animate=(move || {
                            let base_value = counter.get() as f64 * 0.1;
                            let mut animate = HashMap::new();
                            animate.insert("opacity".to_string(), AnimationValue::Number(0.5 + base_value));
                            animate.insert("transform".to_string(), AnimationValue::String(format!("scale({})", 0.8 + base_value)));
                            animate
                        })()
                        style=(move || {
                            let hue = (counter.get() * 30) % 360;
                            format!(
                                "width: 100px; height: 100px; background: linear-gradient(45deg, hsl({}, 70%, 60%), hsl({}, 70%, 40%)); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;",
                                hue, hue + 60
                            )
                        })()
                    >
                        "Non-Memoized"
                    </ReactiveMotionDiv>
                </div>
            </div>
        </div>
    }
}
