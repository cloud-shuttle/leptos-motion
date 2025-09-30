use leptos::*;
use leptos::prelude::*;
use leptos_motion_dom::MotionDiv;
use leptos_motion_core::{AnimationValue, Transition, Easing};
use std::collections::HashMap;

#[component]
pub fn CSRDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_animated, set_is_animated) = signal(false);

    // Create initial values for MotionDiv
    let initial_values = HashMap::from([
        ("x".to_string(), AnimationValue::Pixels(0.0)),
        ("y".to_string(), AnimationValue::Pixels(0.0)),
        ("opacity".to_string(), AnimationValue::Number(1.0)),
        ("scale".to_string(), AnimationValue::Number(1.0)),
    ]);

    // Create animated values based on state
    let animate_values = move || {
        if is_animated.get() {
            leptos_motion_dom::AnimateProp::Static(HashMap::from([
                ("x".to_string(), AnimationValue::Pixels(100.0)),
                ("y".to_string(), AnimationValue::Pixels(-50.0)),
                ("opacity".to_string(), AnimationValue::Number(0.8)),
                ("scale".to_string(), AnimationValue::Number(1.2)),
            ]))
        } else {
            leptos_motion_dom::AnimateProp::Static(HashMap::from([
                ("x".to_string(), AnimationValue::Pixels(0.0)),
                ("y".to_string(), AnimationValue::Pixels(0.0)),
                ("opacity".to_string(), AnimationValue::Number(1.0)),
                ("scale".to_string(), AnimationValue::Number(1.0)),
            ]))
        }
    };

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.6),
        ease: Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div class="demo-container">
            <h1>"🚀 Leptos Motion CSR Demo (MotionDiv)"</h1>
            <p>"Real Rust/WASM animations with MotionDiv (WASM-compatible)"</p>

            <section class="demo-section">
                <h2>"MotionDiv Animation Engine"</h2>
                <button on:click=move |_| set_is_animated.set(!is_animated.get())>
                    "Toggle Animation"
                </button>

                <div
                    style="cursor: pointer; margin: 20px;"
                    on:click=move |_| {
                        set_count.set(count.get() + 1);
                    }
                >
                    <MotionDiv
                        node_ref=NodeRef::new()
                        class="motion-box".to_string()
                        style="background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; box-shadow: 0 4px 8px rgba(0,0,0,0.2);".to_string()
                        initial=initial_values
                        animate=animate_values()
                        _transition=transition
                    >
                        "Click me! Count: " {count}
                    </MotionDiv>
                </div>
            </section>

            <section class="demo-section">
                <h2>"Interactive Counter"</h2>
                <button on:click=move |_| set_count.set(count.get() + 1)>
                    "Increment Counter: " {count}
                </button>
            </section>

            <section class="demo-section">
                <h2>"Features"</h2>
                <ul>
                    <li>"✅ Real MotionDiv with Rust animation engine"</li>
                    <li>"✅ WASM-compatible reactive animations"</li>
                    <li>"✅ Type-safe AnimationValue system"</li>
                    <li>"✅ Memory-safe Rust implementation"</li>
                    <li>"✅ Hardware-accelerated CSS transforms"</li>
                    <li>"✅ Reactive signal integration"</li>
                </ul>
            </section>

            <section class="demo-section">
                <h2>"MotionDiv Architecture"</h2>
                <ul>
                    <li>"✅ Hybrid WAAPI/CSS animation engine"</li>
                    <li>"✅ Automatic fallback system"</li>
                    <li>"✅ Performance-optimized updates"</li>
                    <li>"✅ Memory-managed animation lifecycle"</li>
                    <li>"✅ Production-ready for enterprise apps"</li>
                </ul>
            </section>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <html>
            <head>
                <title>"Leptos Motion CSR Demo"</title>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <style>
                    r#"
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        min-height: 100vh;
                        color: white;
                    }
                    
                    .demo-container {
                        max-width: 1200px;
                        margin: 0 auto;
                    }
                    
                    .demo-section {
                        margin: 40px 0;
                        padding: 20px;
                        background: rgba(255, 255, 255, 0.1);
                        border-radius: 12px;
                        backdrop-filter: blur(10px);
                    }
                    
                    .motion-box {
                        width: 100px;
                        height: 100px;
                        border: 2px solid white;
                        border-radius: 8px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        cursor: pointer;
                        user-select: none;
                        margin: 20px;
                    }
                    
                    button {
                        background: rgba(255, 255, 255, 0.2);
                        border: 2px solid white;
                        color: white;
                        padding: 10px 20px;
                        border-radius: 8px;
                        cursor: pointer;
                        font-size: 16px;
                        margin: 10px;
                    }
                    
                    button:hover {
                        background: rgba(255, 255, 255, 0.3);
                    }
                    
                    ul {
                        list-style: none;
                        padding: 0;
                    }
                    
                    li {
                        margin: 10px 0;
                        padding: 10px;
                        background: rgba(255, 255, 255, 0.1);
                        border-radius: 8px;
                    }
                    "#
                </style>
            </head>
            <body>
                <CSRDemo/>
            </body>
        </html>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}