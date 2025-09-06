use leptos::mount::mount_to_body;
use leptos::prelude::*;

use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (_count, _set_count) = signal(0);

    view! {
        <div class="app">
            <h1>"Leptos Motion - Ultra Minimal! 🚀"</h1>

            <div class="demo-section">
                <h2>"✅ Ultra Minimal Bundle Test:"</h2>
                <ul>
                    <li>"Only Leptos core"</li>
                    <li>"No animation engine"</li>
                    <li>"No motion components"</li>
                    <li>"Just basic Leptos"</li>
                </ul>
            </div>

            <div class="demo-section">
                <h2>"🎬 Basic Leptos Demo:"</h2>

                <div class="animated-box">
                    "Basic Leptos Component"
                </div>

                <button on:click=move |_| {
                    // Just basic Leptos functionality
                    log::info!("Button clicked!");
                }>
                    "Test Basic Leptos"
                </button>
            </div>

            <div class="demo-section">
                <h2>"📊 Bundle Size Test:"</h2>
                <p>"This example only includes basic Leptos to establish a baseline."</p>
                <p>"Target: < 30KB WASM"</p>
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
