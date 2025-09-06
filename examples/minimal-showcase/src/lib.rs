use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_motion_core::*;

use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, _set_count) = signal(0);

    view! {
        <div class="app">
            <h1>"Leptos Motion - Minimal Core Only! 🚀"</h1>

            <div class="demo-section">
                <h2>"✅ Core Animation Engine Only:"</h2>
                <ul>
                    <li>"RAF Animation Loop"</li>
                    <li>"WAAPI Integration"</li>
                    <li>"Motion Values"</li>
                    <li>"Basic Transitions"</li>
                </ul>
            </div>

            <div class="demo-section">
                <h2>"🎬 Core Animation Demo:"</h2>

                <div class="animated-box">
                    "Core Animation Engine Test"
                </div>

                <button on:click=move |_| {
                    // Test minimal animation engine
                    let engine = MinimalEngine::new();
                    // This uses the minimal engine with only essential features
                }>
                    "Test Core Engine"
                </button>
            </div>

            <div class="demo-section">
                <h2>"📊 Bundle Size Test:"</h2>
                <p>"This example only includes the core animation engine to test minimal bundle size."</p>
                <p>"Target: < 50KB WASM"</p>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    mount_to_body(App);
}
