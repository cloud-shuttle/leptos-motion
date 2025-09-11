use leptos::prelude::*;
use leptos_motion_core::AnimationValue;
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

/// Simple Demo Component
///
/// This is a minimal demo to test if the basic functionality works
#[component]
pub fn SimpleDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎨 SimpleDemo: Component created".into());
    let (is_animated, set_is_animated) = signal(false);

    // Simple animation
    let animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if is_animated.get() {
            target.insert("rotateZ".to_string(), AnimationValue::Number(360.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.5));
        } else {
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
        }
        target
    });

    web_sys::console::log_1(&"🎨 SimpleDemo: About to render view".into());
    view! {
        <div style="text-align: center; padding: 2rem;">
            <h1 style="color: white; margin-bottom: 2rem;">"Simple Demo Test"</h1>

            <button
                style="background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 1rem; cursor: pointer; margin-bottom: 2rem;"
                on:click=move |_| set_is_animated.update(|a| *a = !*a)
            >
                "Toggle Animation"
            </button>

            <ReactiveMotionDiv
                animate_fn=Box::new(move || animation.get())
                style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ee5a24); margin: 0 auto; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
            >
                "Hello!"
            </ReactiveMotionDiv>

            <p style="color: white; margin-top: 2rem;">
                "If you can see this and click the button, the demo is working!"
            </p>
        </div>
    }
}
