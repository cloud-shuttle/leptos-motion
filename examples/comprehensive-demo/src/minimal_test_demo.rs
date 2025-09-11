//! Minimal Test Demo
//!
//! A minimal demo to test if basic Leptos rendering works

use leptos::prelude::*;

#[component]
pub fn MinimalTestDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 Minimal Test Demo: Component created".into());

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: #f0f0f0; min-height: 100vh;">
            <h1 style="color: #333; text-align: center;">"🎯 Minimal Test Demo"</h1>
            <p style="text-align: center; font-size: 18px;">"This is a minimal test to see if basic Leptos rendering works."</p>

            <div style="text-align: center; margin: 20px;">
                <button
                    style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px;"
                >
                    "Test Button"
                </button>
            </div>

            <div style="background: white; padding: 20px; margin: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                <h2 style="color: #333;">"✅ If you can see this, Leptos is working!"</h2>
                <p>"This demo tests basic Leptos functionality without any custom animation components."</p>
            </div>
        </div>
    }
}
