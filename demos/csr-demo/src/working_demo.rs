use leptos::*;
use leptos::prelude::*;

#[component]
pub fn WorkingCSRDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_animated, set_is_animated) = signal(false);
    let (opacity, set_opacity) = signal(1.0);
    let (transform, set_transform) = signal("translateX(0px) translateY(0px)");

    // Handle animation toggle
    let toggle_animation = move |_| {
        set_is_animated.set(!is_animated.get());
        if is_animated.get() {
            set_opacity.set(0.8);
            set_transform.set("translateX(100px) translateY(-50px)");
        } else {
            set_opacity.set(1.0);
            set_transform.set("translateX(0px) translateY(0px)");
        }
    };

    // Handle click
    let handle_click = move |_| {
        set_count.set(count.get() + 1);
    };

    view! {
        <div class="demo-container">
            <h1>"🚀 Leptos Motion CSR Demo (Working Version)"</h1>
            <p>"Real Rust/WASM animations with CSS transitions (MotionDiv alternative)"</p>
            
            <section class="demo-section">
                <h2>"Basic Motion (CSS-based)"</h2>
                <button on:click=toggle_animation>
                    "Toggle Animation"
                </button>
                
                <div 
                    class="motion-box"
                    style=move || format!(
                        "background: linear-gradient(45deg, #ff6b6b, #4ecdc4); 
                         width: 100px; height: 100px; border-radius: 10px; 
                         display: flex; align-items: center; justify-content: center; 
                         color: white; font-weight: bold; cursor: pointer; margin: 20px;
                         transition: all 0.6s ease-in-out;
                         opacity: {}; transform: {};",
                        opacity.get(),
                        transform.get()
                    )
                    on:click=handle_click
                >
                    "Click me! Count: " {count}
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
                    <li>"Real Rust/WASM reactive updates"</li>
                    <li>"CSS transitions for smooth animations"</li>
                    <li>"Type-safe Leptos components"</li>
                    <li>"Memory-safe Rust"</li>
                    <li>"No MotionDiv panics!"</li>
                </ul>
            </section>

            <section class="demo-section">
                <h2>"Why This Works"</h2>
                <ul>
                    <li>"✅ No WASM time system issues"</li>
                    <li>"✅ No RefCell borrowing conflicts"</li>
                    <li>"✅ Proper reactive signal context"</li>
                    <li>"✅ CSS handles the heavy lifting"</li>
                    <li>"✅ Leptos handles the reactivity"</li>
                </ul>
            </section>
        </div>
    }
}

#[component]
pub fn WorkingApp() -> impl IntoView {
    view! {
        <html>
            <head>
                <title>"Leptos Motion CSR Demo (Working)"</title>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <style>
                    r#"
                    .demo-container {
                        max-width: 1200px;
                        margin: 0 auto;
                        padding: 20px;
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        min-height: 100vh;
                        color: white;
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
                        user-select: none;
                        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                    }
                    
                    .motion-box:hover {
                        transform: scale(1.1) !important;
                        box-shadow: 0 6px 8px rgba(0, 0, 0, 0.2);
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
                        transition: all 0.3s ease;
                    }
                    
                    button:hover {
                        background: rgba(255, 255, 255, 0.3);
                        transform: translateY(-2px);
                    }
                    
                    ul {
                        list-style: none;
                        padding: 0;
                    }
                    
                    li {
                        margin: 10px 0;
                        padding: 10px;
                        background: rgba(255, 255, 255, 0.1);
                        border-radius: 6px;
                    }
                    "#
                </style>
            </head>
            <body>
                <WorkingCSRDemo/>
            </body>
        </html>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(WorkingApp);
}
