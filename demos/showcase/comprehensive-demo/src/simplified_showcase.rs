use leptos_motion_dom::AnimateProp;use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

/// Simplified Motion Showcase Demo
///
/// This is a minimal version of MotionShowcaseDemo to identify what's causing the right-click issue
#[component]
pub fn SimplifiedShowcase() -> impl IntoView {
    web_sys::console::log_1(&"🎨 SimplifiedShowcase: Component created".into());

    let (is_animated, set_is_animated) = signal(false);

    // Simple animation
    let initial = move || {
        let mut initial = HashMap::new();
        initial.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
        initial
    };

    let animate = move || {
        let mut animate = HashMap::new();
        if is_animated.get() {
            animate.insert("rotate".to_string(), AnimationValue::Degrees(360.0));
            animate.insert("scale".to_string(), AnimationValue::Number(1.2));
        } else {
            animate.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
            animate.insert("scale".to_string(), AnimationValue::Number(1.0));
        }
        animate
    };

    let transition_config = Transition {
        duration: Some(0.5),
        delay: None,
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    web_sys::console::log_1(&"🎨 SimplifiedShowcase: About to render view".into());
    view! {
        <div style="text-align: center; padding: 2rem; background: #282c34; min-height: 100vh; color: white;">
            <h1 style="color: #61dafb; margin-bottom: 2rem;">"Simplified Motion Showcase"</h1>

            <p style="margin-bottom: 2rem;">"Testing basic animation functionality"</p>

            <button
                style="background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 1rem; cursor: pointer; margin-bottom: 2rem;"
                on:click=move |_| set_is_animated.update(|a| *a = !*a)
            >
                "Toggle Animation"
            </button>

            <MotionDiv
                node_ref=NodeRef::new()
                initial=(move || initial())()
                animate=AnimateProp::Static((move || animate())())
                _transition=transition_config
                style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ee5a24); margin: 0 auto; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
            >
                "Animate!"
            </MotionDiv>

            <p style="color: #90EE90; margin-top: 2rem;">
                "If you can see this and right-click works, basic animations are working!"
            </p>
        </div>
    }
}
