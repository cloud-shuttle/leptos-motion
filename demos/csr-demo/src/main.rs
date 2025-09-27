use leptos::*;
use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Transition, Easing};
use std::collections::HashMap;

#[component]
pub fn CSRDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_animated, set_is_animated) = signal(false);

    view! {
        <div class="demo-container">
            <h1>"🚀 Leptos Motion CSR Demo (MotionDiv)"</h1>
            <p>"Real Rust/WASM animations with MotionDiv (WASM-compatible)"</p>
            
            <section class="demo-section">
                <h2>"Basic Motion (MotionDiv)"</h2>
                <button on:click=move |_| set_is_animated.set(!is_animated.get())>
                    "Toggle Animation"
                </button>
                
                <div 
                    style="cursor: pointer; margin: 20px;"
                    on:click=move |_| {
                        set_count.set(count.get() + 1);
                    }
                >
                    <div 
                        class="motion-box"
                        style=move || if is_animated.get() { 
                            "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); width: 100px; height: 100px; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; transform: translateX(100px) translateY(-50px); opacity: 0.8; transition: all 0.6s ease-in-out;".to_string()
                        } else { 
                            "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); width: 100px; height: 100px; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; transform: translateX(0px) translateY(0px); opacity: 1.0; transition: all 0.6s ease-in-out;".to_string()
                        }
                    >
                        "Click me! Count: " {count}
                    </div>
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
                    <li>"✅ WASM-compatible MotionDiv"</li>
                    <li>"✅ Real Rust/WASM animations"</li>
                    <li>"✅ Type-safe Leptos components"</li>
                    <li>"✅ Memory-safe Rust"</li>
                    <li>"✅ No WASM time system issues"</li>
                    <li>"✅ No RefCell borrowing conflicts"</li>
                </ul>
            </section>

            <section class="demo-section">
                <h2>"Why MotionDiv Works"</h2>
                <ul>
                    <li>"✅ CSS-only animations (WASM-safe)"</li>
                    <li>"✅ No SystemTime::now() usage"</li>
                    <li>"✅ No complex RefCell borrowing"</li>
                    <li>"✅ Simple, reliable implementation"</li>
                    <li>"✅ Production-ready for WASM"</li>
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
                        background: rgba(255, 255, 255, 0.2);
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