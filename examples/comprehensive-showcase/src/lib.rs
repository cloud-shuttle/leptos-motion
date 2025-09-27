//! Comprehensive Showcase - Professional motion library examples
//!
//! This demo showcases comprehensive examples like those you'd find 
//! in professional motion libraries (similar to Framer Motion's examples)

use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    // Skip logger initialization to prevent panics in browser
    // The demo will work without logging
}

#[wasm_bindgen]
pub fn comprehensive_showcase() {
    // Mount to the app div specifically
    let _ = leptos::mount::mount_to_body(|| view! { 
        <ShowcaseComponent />
    });
}

#[component]
fn ShowcaseComponent() -> impl IntoView {
    let (button_scale, set_button_scale) = signal(1.0);
    let (card_x, set_card_x) = signal(0.0);
    let (loading_rotation, set_loading_rotation) = signal(0.0);
    let (message, set_message) = signal("Click the buttons to see animations!".to_string());

    let handle_button_click = move |_| {
        set_button_scale.update(|scale| *scale = if *scale == 1.0 { 1.2 } else { 1.0 });
        set_message.set("Button Animation: Scale effect!".to_string());
    };

    let handle_card_click = move |_| {
        set_card_x.update(|x| *x = if *x == 0.0 { 50.0 } else { 0.0 });
        set_message.set("Card Animation: Slide effect!".to_string());
    };

    let handle_loading_click = move |_| {
        set_loading_rotation.update(|rot| *rot += 360.0);
        set_message.set("Loading Animation: Rotation effect!".to_string());
    };

    view! {
        <div id="app" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; min-height: 100vh; font-family: system-ui;">
            <h1 style="font-size: 48px; text-align: center; margin-bottom: 20px;">
                "🎨 Leptos Motion"
            </h1>
            <p style="font-size: 24px; text-align: center; margin-bottom: 40px;">
                "Professional Motion Library Showcase"
            </p>
            <div style="display: flex; justify-content: center; gap: 20px; flex-wrap: wrap;">
                <div 
                    style=move || format!("background: #4CAF50; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer; transition: all 0.3s ease; transform: scale({})", button_scale.get())
                    on:click=handle_button_click
                >
                    "Button Animation"
                </div>
                <div 
                    style=move || format!("background: #2196F3; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer; transition: all 0.3s ease; transform: translateX({}px)", card_x.get())
                    on:click=handle_card_click
                >
                    "Card Animation"
                </div>
                <div 
                    style=move || format!("background: #FF9800; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer; transition: all 0.3s ease; transform: rotate({}deg)", loading_rotation.get())
                    on:click=handle_loading_click
                >
                    "Loading Animation"
                </div>
            </div>
            <div style="margin-top: 40px; text-align: center;">
                <p style="font-size: 18px; opacity: 0.8;">
                    {message}
                </p>
            </div>
        </div>
    }
}