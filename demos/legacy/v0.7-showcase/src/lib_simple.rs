use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_dom::{ReactiveMotionDiv, signal_animate};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Initialize logger only if not already initialized
    let _ = console_log::init_with_level(log::Level::Debug);

    web_sys::console::log_1(&"About to mount to app div".into());

    let _handle = mount_to(
        || {
            web_sys::console::log_1(&"App component rendering".into());
            view! {
                <App />
            }
        },
        "app",
    );

    web_sys::console::log_1(&"Mount completed".into());
}

#[component]
fn App() -> impl IntoView {
    web_sys::console::log_1(&"App component starting to render".into());

    view! {
        <div class="showcase-grid">
            <div class="showcase-card">
                <div class="feature-badge">🎯 Spring Physics</div>
                <h3>Reactive Spring Animations</h3>
                <p>Experience smooth spring physics with reactive state management.</p>
                <div class="demo-area">
                    <SpringPhysicsDemo />
                </div>
            </div>
        </div>
    }
}

#[component]
fn SpringPhysicsDemo() -> impl IntoView {
    web_sys::console::log_1(&"SpringPhysicsDemo starting to render".into());

    let (is_active, set_is_active) = create_signal(false);

    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(
            &format!("Animation triggered, is_active: {}", is_active.get()).into(),
        );
    };

    let animation_target = signal_animate(move || {
        let is_active_val = is_active.get();
        web_sys::console::log_1(
            &format!(
                "Returning {} animation",
                if is_active_val { "active" } else { "idle" }
            )
            .into(),
        );

        if is_active_val {
            HashMap::from([
                (
                    "transform".to_string(),
                    "translateX(100px) scale(1.2) rotate(10deg)".to_string(),
                ),
                ("background-color".to_string(), "#3b82f6".to_string()),
            ])
        } else {
            HashMap::from([
                (
                    "transform".to_string(),
                    "translateX(0px) scale(1.0) rotate(0deg)".to_string(),
                ),
                ("background-color".to_string(), "#ef4444".to_string()),
            ])
        }
    });

    web_sys::console::log_1(&"SpringPhysicsDemo finished rendering".into());

    view! {
        <div class="controls">
            <button class="btn btn-primary" on:click=toggle>
                "Toggle Spring Physics"
            </button>
        </div>
        <ReactiveMotionDiv
            animate=animation_target
            class="w-20 h-20 bg-red-500 rounded-lg flex items-center justify-center text-white font-bold text-lg"
        >
            <div>"🎯"</div>
        </ReactiveMotionDiv>
    }
}
