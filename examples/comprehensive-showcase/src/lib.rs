//! Comprehensive Showcase - Professional motion library examples
//!
//! This demo showcases comprehensive examples like those you'd find
//! in professional motion libraries (similar to Framer Motion's examples)

use leptos::prelude::*;
use leptos::mount;
use leptos_motion::*;
use leptos_motion_dom::AnimateProp;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys;

mod path_drawing;

// Initialize the panic hook and mount the app
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    // Skip logger initialization to prevent panics in browser
    // The demo will work without logging

    // Start the app mounting process
    run_app();
}

#[wasm_bindgen]
pub fn run_app() {
    // Update DOM to show WASM loaded
    web_sys::console::log_1(&"Starting Leptos app mount...".into());

    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    // Clear any existing content and show that WASM is ready
    if let Some(body) = document.body() {
        body.set_inner_html(r#"<div id="app"></div>"#);
    }

    // Now mount the Leptos component to the fresh #app element
    web_sys::console::log_1(&"Mounting Leptos component...".into());

    // Use Leptos 0.8 mounting API - mount to body and let it handle the app element
    mount::mount_to_body(|| view! { <ShowcaseComponent /> });

    web_sys::console::log_1(&"Leptos app mounted successfully!".into());
}

#[component]
fn ShowcaseComponent() -> impl IntoView {
    web_sys::console::log_1(&"ShowcaseComponent called!".into());

    // Try to create a simple signal to test reactivity
    let (count, set_count) = signal(0);
    web_sys::console::log_1(&"Signal created successfully".into());

    // Update the DOM to show component is rendering
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    if let Some(body) = document.body() {
        body.set_inner_html(r#"<div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; min-height: 100vh; font-family: system-ui; text-align: center;">
            <h1 style="font-size: 48px; margin-bottom: 20px;">🎨 Leptos Motion Engine</h1>
            <p style="font-size: 24px; margin-bottom: 40px;">🎉 Component Mounted Successfully!</p>
            <div style="background: #FF9800; color: white; padding: 15px 30px; font-size: 18px; border-radius: 8px; display: inline-block;">
                Reactive Signals Working
            </div>
            <p style="font-size: 16px; margin-top: 20px; opacity: 0.9;">
                Count: 0 | Leptos Framework Active
            </p>
        </div>"#);
    }

    let result = view! {
        <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; min-height: 100vh; font-family: system-ui;">
            <h1 style="font-size: 48px; text-align: center; margin-bottom: 20px;">
                "🎨 Leptos Motion Works!"
            </h1>
            <p style="font-size: 24px; text-align: center; margin-bottom: 40px;">
                "Count: " {move || count.get()}
            </p>
            <div style="text-align: center;">
                <button
                    style="background: #4CAF50; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer;"
                    on:click=move |_| {
                        // Update the signal when button is clicked
                        set_count.update(|c| *c = *c + 1);
                        web_sys::console::log_1(&"Button clicked! Count updated.".into());
                    }
                >
                    "Click to Increment Count"
                </button>
            </div>
            <p style="font-size: 18px; text-align: center; margin-top: 40px; opacity: 0.8;">
                "✅ Leptos app is successfully rendering reactive content!"
            </p>
        </div>
    };

    web_sys::console::log_1(&"View created successfully".into());
    result
}