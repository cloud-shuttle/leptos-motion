use leptos::prelude::*;

/// Ultra Simple Demo Component
///
/// This is the most basic possible demo to test if Leptos itself works
#[component]
pub fn UltraSimpleDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎨 UltraSimpleDemo: Component created".into());

    let (count, set_count) = signal(0);

    web_sys::console::log_1(&"🎨 UltraSimpleDemo: About to render view".into());
    view! {
        <div style="text-align: center; padding: 2rem; background: #282c34; min-height: 100vh; color: white;">
            <h1 style="color: #61dafb; margin-bottom: 2rem;">"Ultra Simple Demo"</h1>

            <p style="font-size: 1.2rem; margin-bottom: 2rem;">
                "Count: " {count}
            </p>

            <button
                style="background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 1rem; cursor: pointer; margin-bottom: 2rem;"
                on:click=move |_| set_count.update(|c| *c += 1)
            >
                "Increment"
            </button>

            <p style="color: #90EE90; margin-top: 2rem;">
                "If you can see this and click the button, Leptos is working!"
            </p>
        </div>
    }
}
