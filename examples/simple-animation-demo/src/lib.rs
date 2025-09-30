use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to;
use leptos_motion_dom::MotionDiv;
use leptos_motion_core::{AnimationValue, Transition, Easing};
use wasm_bindgen::prelude::*;
use web_sys::window;
use std::collections::HashMap;

// Simple animated box component using MotionDiv
#[component]
pub fn SimpleAnimatedBox() -> impl IntoView {
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    // Create base initial values
    let initial_values = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(1.0)),
        ("rotate".to_string(), AnimationValue::Number(0.0)),
    ]);

    // Create animated values based on state
    let animate_values = move || {
        let mut animations = HashMap::from([
            ("scale".to_string(), AnimationValue::Number(1.0)),
            ("rotate".to_string(), AnimationValue::Number(0.0)),
        ]);

        if is_hovered.get() {
            animations.insert("scale".to_string(), AnimationValue::Number(1.1));
            animations.insert("rotate".to_string(), AnimationValue::Number(5.0));
        }

        if is_tapped.get() {
            animations.insert("scale".to_string(), AnimationValue::Number(0.9));
        }

        leptos_motion_dom::AnimateProp::Static(animations)
    };

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.3),
        ease: Easing::EaseOut,
        ..Default::default()
    };

    view! {
        <MotionDiv
            node_ref=NodeRef::new()
            class="animated-box".to_string()
            style="width: 120px; height: 120px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer; box-shadow: 0 4px 8px rgba(0,0,0,0.1);".to_string()
            initial=initial_values
            animate=animate_values()
            _transition=transition
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
        >
            "Animated Box"
        </MotionDiv>
    }
}

// Simple animated button component using MotionDiv
#[component]
pub fn SimpleAnimatedButton(text: String) -> impl IntoView {
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    // Create initial values for the button
    let initial_values = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(1.0)),
    ]);

    // Create animated values based on state
    let animate_values = move || {
        let mut animations = HashMap::from([
            ("scale".to_string(), AnimationValue::Number(1.0)),
        ]);

        if is_hovered.get() {
            animations.insert("scale".to_string(), AnimationValue::Number(1.05));
        }

        if is_tapped.get() {
            animations.insert("scale".to_string(), AnimationValue::Number(0.95));
        }

        leptos_motion_dom::AnimateProp::Static(animations)
    };

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.2),
        ease: Easing::EaseOut,
        ..Default::default()
    };

    view! {
        <MotionDiv
            node_ref=NodeRef::new()
            class="animated-button".to_string()
            style={
                if is_hovered.get() {
                    "padding: 12px 24px; border: none; border-radius: 8px; cursor: pointer; font-weight: bold; background-color: #0056b3; color: white;".to_string()
                } else {
                    "padding: 12px 24px; border: none; border-radius: 8px; cursor: pointer; font-weight: bold; background-color: #007bff; color: white;".to_string()
                }
            }
            initial=initial_values
            animate=animate_values()
            _transition=transition
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
        >
            {text}
        </MotionDiv>
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