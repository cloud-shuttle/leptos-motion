use leptos::prelude::*;
use leptos::mount;
use leptos_motion::*;
use wasm_bindgen::prelude::*;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run_app() {
    mount::mount_to_body(|| view! { <SharedLayoutDemo /> });
}

#[component]
fn SharedLayoutDemo() -> impl IntoView {
    // Layout state - controls which grid layout we're using
    let (layout_mode, set_layout_mode) = signal("grid-2x2");

    // Shared element configuration
    let shared_config = SharedLayoutConfig {
        transition_type: SharedTransitionType::Morph,
        animation: leptos_motion_dom::LayoutAnimationConfig {
            duration: Some(0.6),
        },
    };

    let cards = vec![
        ("🎨", "Design", "Creative visual design and user experience", "hero-card"),
        ("⚡", "Performance", "Lightning-fast animations and interactions", "hero-card"),
        ("🎭", "Magic", "Seamless transitions between layouts", "hero-card"),
        ("🚀", "Future", "Next-generation web animations", "hero-card"),
    ];

    view! {
        <div class="container">
            <div class="header">
                <h1>"🎭 Shared Layout Transitions"</h1>
                <p>"Elements with the same layout_id smoothly animate between positions"</p>
            </div>

            <div class="controls">
                <button
                    class=move || if layout_mode.get() == "grid-2x2" { "active" } else { "" }
                    on:click=move |_| set_layout_mode.set("grid-2x2")
                >
                    "2×2 Grid"
                </button>
                <button
                    class=move || if layout_mode.get() == "grid-1x4" { "active" } else { "" }
                    on:click=move |_| set_layout_mode.set("grid-1x4")
                >
                    "1×4 Column"
                </button>
                <button
                    class=move || if layout_mode.get() == "grid-4x1" { "active" } else { "" }
                    on:click=move |_| set_layout_mode.set("grid-4x1")
                >
                    "4×1 Row"
                </button>
            </div>

            <div class=move || format!("layout-grid {}", layout_mode.get())>
                {cards.into_iter().enumerate().map(|(index, (icon, title, description, layout_id))| {
                    view! {
                        <MotionDiv
                            layout_id=layout_id.to_string()
                            shared_layout=shared_config.clone()
                            class="card".to_string()
                            style={
                                // Different styles for different layouts to showcase transitions
                                let mode = layout_mode.get();
                                match mode.as_str() {
                                    "grid-2x2" => "min-height: 200px;".to_string(),
                                    "grid-1x4" => "min-height: 150px;".to_string(),
                                    "grid-4x1" => "min-height: 180px;".to_string(),
                                    _ => "min-height: 200px;".to_string(),
                                }
                            }
                            node_ref=leptos::NodeRef::new()
                            children=Box::new(move || view! {
                                <div class="card-icon">{icon}</div>
                                <div class="card-title">{title}</div>
                                <div class="card-description">{description}</div>
                            }.into_any())
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="info-panel">
                <h3>"🎭 Shared Layout Transitions Demo"</h3>
                <p>
                    "This demo showcases Leptos Motion's shared layout transitions. "
                    "All cards have the same layout_id (\"hero-card\"), so when you change "
                    "the layout, they smoothly animate between their old and new positions "
                    "instead of jumping instantly."
                </p>
                <p>
                    <strong>"Try it:"</strong>" Click the different layout buttons above to see "
                    "the cards smoothly morph between grid positions. Notice how each card "
                    "maintains visual continuity as it moves to its new location."
                </p>
            </div>
        </div>
    }
}
