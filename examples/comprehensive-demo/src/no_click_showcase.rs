use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

/// No Click Showcase Demo
///
/// This version removes all click handlers to test if they're causing the right-click issue
#[component]
pub fn NoClickShowcase() -> impl IntoView {
    web_sys::console::log_1(&"🎨 NoClickShowcase: Component created".into());

    let (is_animated, set_is_animated) = signal(false);

    // Simple animation
    let animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if is_animated.get() {
            target.insert("rotateZ".to_string(), AnimationValue::Number(360.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.2));
        } else {
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
        }
        target
    });

    let transition_config = Transition {
        duration: Some(0.5),
        delay: None,
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    web_sys::console::log_1(&"🎨 NoClickShowcase: About to render view".into());
    view! {
        <div style="text-align: center; padding: 2rem; background: #282c34; min-height: 100vh; color: white;">
            <h1 style="color: #61dafb; margin-bottom: 2rem;">"No Click Showcase"</h1>

            <p style="margin-bottom: 2rem;">"Testing ReactiveMotionDiv without click handlers"</p>

            <button
                style="background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 1rem; cursor: pointer; margin-bottom: 2rem;"
                on:click=move |_| set_is_animated.update(|a| *a = !*a)
            >
                "Toggle Animation"
            </button>

            <ReactiveMotionDiv
                animate_fn=Box::new(move || animation.get())
                transition=transition_config
                style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ee5a24); margin: 0 auto; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
            >
                "Animate!"
            </ReactiveMotionDiv>

            <p style="color: #90EE90; margin-top: 2rem;">
                "If you can see this and right-click works, the issue is with click handlers!"
            </p>
        </div>
    }
}
