use leptos::*;
use leptos::prelude::*;  // Brings in create_signal, NodeRef, ElementChild
use leptos_motion_dom::MotionDiv;
use leptos_motion_core::{AnimationValue, Transition, Easing};
use std::collections::HashMap;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_animated, set_is_animated) = signal(false);

    view! {
        <html>
            <head>
                <title>"Leptos Motion SSR Demo (MotionDiv)"</title>
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
                    
                    .server-info {
                        background: rgba(0, 0, 0, 0.2);
                        padding: 15px;
                        border-radius: 8px;
                        margin: 10px 0;
                    }
                    
                    .motion-box {
                        width: 100px;
                        height: 100px;
                        background: rgba(255, 255, 255, 0.2);
                        border: 2px solid white;
                        border-radius: 8px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        user-select: none;
                        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
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
                    "#
                </style>
            </head>
            <body>
                <div class="demo-container">
                    <h1>"🚀 Leptos Motion SSR Demo (MotionDiv)"</h1>
                    <p>"Server-Side Rendered Rust/WASM animations with MotionDiv"</p>
                    
                    <div class="server-info">
                        <h2>"Server-Side Rendering"</h2>
                        <p>"This content was rendered on the server"</p>
                        <p>"Count: " {count}</p>
                    </div>
                    
                    <section>
                        <h2>"MotionDiv Animation"</h2>
                        <button on:click=move |_| set_is_animated.update(|a| *a = !*a)>
                            "Toggle Animation"
                        </button>
                        
                        <div style="cursor: pointer;" on:click=move |_| set_count.update(|c| *c += 1)>
                            <MotionDiv
                                node_ref=NodeRef::new()
                                class="motion-box".to_string()
                                style="background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; font-weight: bold;".to_string()
                                initial=HashMap::from([
                                    ("x".to_string(), AnimationValue::Pixels(0.0)),
                                    ("y".to_string(), AnimationValue::Pixels(0.0)),
                                    ("opacity".to_string(), AnimationValue::Number(1.0)),
                                ])
                                animate=if is_animated.get() { 
                                    HashMap::from([
                                        ("x".to_string(), AnimationValue::Pixels(100.0)),
                                        ("y".to_string(), AnimationValue::Pixels(-50.0)),
                                        ("opacity".to_string(), AnimationValue::Number(0.8)),
                                    ])
                                } else { HashMap::new() }
                                _transition=Transition {
                                    duration: Some(0.6),
                                    ease: Easing::EaseInOut,
                                    ..Default::default()
                                }
                            >
                                "Click me! Count: " {count}
                            </MotionDiv>
                        </div>
                    </section>
                    
                    <button on:click=move |_| set_count.update(|c| *c += 1)>
                        "Increment Counter"
                    </button>
                    
                    <h2>"Features"</h2>
                    <ul>
                        <li>"✅ Server-side rendering for fast initial load"</li>
                        <li>"✅ Progressive enhancement for accessibility"</li>
                        <li>"✅ SEO-friendly content for search engines"</li>
                        <li>"✅ WASM-compatible MotionDiv animations"</li>
                        <li>"✅ No WASM time system issues"</li>
                        <li>"✅ No RefCell borrowing conflicts"</li>
                    </ul>
                </div>
            </body>
        </html>
    }
}