use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to;
use wasm_bindgen::prelude::*;
use web_sys::window;

// Simple animated box component - no complex props, just basic functionality
#[component]
pub fn SimpleAnimatedBox() -> impl IntoView {
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    let box_style = move || {
        let mut style = "width: 120px; height: 120px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer; box-shadow: 0 4px 8px rgba(0,0,0,0.1); transition: all 0.3s ease;".to_string();
        
        if is_hovered.get() {
            style.push_str(" transform: scale(1.1) rotate(5deg);");
        }
        
        if is_tapped.get() {
            style.push_str(" transform: scale(0.9);");
        }
        
        style
    };

    view! {
        <div
            style=box_style
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
        >
            "Animated Box"
        </div>
    }
}

// Simple animated button component - no complex props, just basic functionality
#[component]
pub fn SimpleAnimatedButton(text: String) -> impl IntoView {
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    let button_style = move || {
        let mut style = "padding: 12px 24px; border: none; border-radius: 8px; cursor: pointer; font-weight: bold; transition: all 0.2s ease; background-color: #007bff; color: white;".to_string();
        
        if is_hovered.get() {
            style.push_str(" transform: scale(1.05); background-color: #0056b3;");
        }
        
        if is_tapped.get() {
            style.push_str(" transform: scale(0.95);");
        }
        
        style
    };

    view! {
        <div
            style=button_style
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
        >
            {text}
        </div>
    }
}

// Main demo component
#[component]
pub fn App() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh;">
            <div style="max-width: 1200px; margin: 0 auto;">
                <h1 style="color: white; text-align: center; margin-bottom: 30px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                    "🎬 Leptos Motion - Simple Components Demo"
                </h1>
                
                <div style="background: white; border-radius: 15px; padding: 20px; margin-bottom: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h2 style="color: #333; margin-bottom: 20px;">"Animation Controls"</h2>
                    
                    <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                        <SimpleAnimatedButton text=if is_animated.get() { "Reset Animation".to_string() } else { "Start Animation".to_string() } />
                        <SimpleAnimatedButton text="Success Button".to_string() />
                        <SimpleAnimatedButton text="Danger Button".to_string() />
                    </div>
                </div>

                <div style="background: white; border-radius: 15px; padding: 20px; margin-bottom: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h3 style="color: #333; margin-bottom: 15px;">"🚀 Simple Animated Components"</h3>
                    <p style="color: #666; margin-bottom: 20px;">"These use simple, focused components with CSS transitions."</p>
                    
                    <div style="display: flex; gap: 20px; flex-wrap: wrap; justify-content: center;">
                        <SimpleAnimatedBox />
                        <SimpleAnimatedBox />
                        <SimpleAnimatedBox />
                    </div>
                </div>

                <div style="background: white; border-radius: 15px; padding: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h3 style="color: #333; margin-bottom: 15px;">"🎯 What's Working:"</h3>
                    <ul style="color: #666; line-height: 1.6;">
                        <li>"✅ Simple, focused components (SimpleAnimatedBox, SimpleAnimatedButton)"</li>
                        <li>"✅ CSS transition-based animations"</li>
                        <li>"✅ Hover and tap interaction animations"</li>
                        <li>"✅ Easy to test and understand"</li>
                        <li>"✅ No complex prop type issues"</li>
                        <li>"✅ Composable and reusable"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

// Initialize the Leptos app
#[wasm_bindgen]
pub fn init_leptos_app() {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"🎬 Initializing Simple Leptos Animation Demo...".into());
    
    // Mount to the #app div
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(app_element) = document.get_element_by_id("app") {
                web_sys::console::log_1(&"✅ Found #app element, mounting Leptos app...".into());
                
                // Clear the existing content
                app_element.set_inner_html("");
                
                // Cast to HtmlElement for mount_to
                if let Ok(html_element) = app_element.dyn_into::<web_sys::HtmlElement>() {
                    // Mount the Leptos app
                    let _ = mount_to(html_element, || view! { <App /> });
                    
                    web_sys::console::log_1(&"✅ Leptos app mounted successfully!".into());
                    return;
                }
            }
        }
    }
    
    web_sys::console::log_1(&"❌ Failed to mount Leptos app!".into());
}

// Keep the simple test function for debugging
#[wasm_bindgen]
pub fn test_rust() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🚀 RUST FUNCTION CALLED!".into());
}

#[wasm_bindgen]
pub fn hello_world() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}