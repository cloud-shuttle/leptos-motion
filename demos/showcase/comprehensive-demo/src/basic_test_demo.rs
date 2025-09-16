//! Basic Test Demo
//!
//! A minimal demo that just renders basic HTML to test if Leptos is working

use leptos::prelude::*;

#[component]
pub fn BasicTestDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 Basic Test Demo: Component created".into());

    let (count, set_count) = signal(0);

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: #f0f0f0; min-height: 100vh;">
            <h1 style="color: #333; text-align: center;">"🎯 Basic Test Demo"</h1>
            <p style="text-align: center; font-size: 18px;">"This is a basic test to see if Leptos is rendering correctly."</p>

            <div style="text-align: center; margin: 20px;">
                <button
                    on:click=move |_| set_count.update(|c| *c += 1)
                    style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px;"
                >
                    "Click me!"
                </button>
                <p style="margin: 10px 0; font-size: 20px;">"Count: " {count}</p>
            </div>

            <div style="background: white; padding: 20px; margin: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                <h2 style="color: #333;">"✅ If you can see this, Leptos is working!"</h2>
                <p>"This demo tests basic Leptos functionality without any custom animation components."</p>
                <ul>
                    <li>"✅ HTML rendering"</li>
                    <li>"✅ CSS styling"</li>
                    <li>"✅ Event handling"</li>
                    <li>"✅ Reactive signals"</li>
                </ul>
            </div>
        </div>
    }
}
