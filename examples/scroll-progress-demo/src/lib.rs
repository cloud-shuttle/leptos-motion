//! Scroll Progress Demo - Advanced Scroll-triggered Animations
//!
//! This demo showcases complex scroll-based animations similar to the React Motion example:
//! - Scroll-triggered progress indicators
//! - SVG circle progress with pathLength
//! - Sticky positioning with scroll transforms
//! - Viewport tracking with offset calculations
//! - Multiple items with individual scroll tracking

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
}

#[wasm_bindgen]
pub fn scroll_progress_demo() {
    leptos::mount::mount_to_body(|| view! {
        <ScrollProgressDemo />
    });
}

#[component]
fn ScrollProgressDemo() -> impl IntoView {
    view! {
        <div style="
            font-family: system-ui, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        ">
            // Header
            <header style="
                text-align: center;
                padding: 60px 20px;
                color: white;
            ">
                <h1 style="
                    font-size: 3rem;
                    margin-bottom: 20px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4, #feca57);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    font-weight: 800;
                ">
                    "📊 Scroll Progress Demo"
                </h1>
                <p style="font-size: 1.4rem; margin-bottom: 30px; opacity: 0.9;">
                    "Scroll to see progress indicators animate"
                </p>
            </header>

            // Scroll Items
            <main>
                {move || (1..=12).map(|i| {
                    view! {
                        <ScrollItem key=i />
                    }
                }).collect::<Vec<_>>()}
            </main>

            // Footer
            <footer style="
                text-align: center;
                padding: 60px 20px;
                color: white;
                opacity: 0.8;
            ">
                <div style="
                    max-width: 600px;
                    margin: 0 auto;
                    background: rgba(255,255,255,0.1);
                    padding: 30px;
                    border-radius: 15px;
                    backdrop-filter: blur(10px);
                ">
                    <h3 style="margin-bottom: 20px; font-size: 1.5rem;">
                        "✨ Scroll-triggered Features"
                    </h3>
                    <ul style="
                        text-align: left;
                        line-height: 1.6;
                        margin: 0;
                        padding-left: 20px;
                    ">
                        <li>"<strong>Progress Indicators:</strong> SVG circle animations"</li>
                        <li>"<strong>Viewport Tracking:</strong> Intersection Observer API"</li>
                        <li>"<strong>Sticky Positioning:</strong> Fixed progress icons"</li>
                        <li>"<strong>Scroll-based Transforms:</strong> Dynamic animations"</li>
                        <li>"<strong>Multiple Items:</strong> Individual scroll tracking"</li>
                    </ul>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn ScrollItem(key: i32) -> impl IntoView {
    let (scroll_progress, set_scroll_progress) = signal(0.0);
    let (is_visible, set_is_visible) = signal(false);
    let item_ref = NodeRef::<leptos::html::Div>::new();

    // Set up scroll tracking
    create_effect(move |_| {
        if let Some(element) = item_ref.get() {
            let window = window().unwrap();
            let window_clone = window.clone();
            
            // Set up scroll listener
            let scroll_callback = Closure::wrap(Box::new(move || {
                // Calculate progress based on element position
                let element_rect = element.get_bounding_client_rect();
                let element_top = element_rect.top();
                let element_height = element_rect.height();
                let window_height = window_clone.inner_height().unwrap().as_f64().unwrap_or(800.0);
                
                // Progress calculation: 0 when element starts entering viewport, 1 when fully scrolled past
                let progress = if element_top <= window_height && element_top + element_height >= 0.0 {
                    // Element is in or near viewport
                    let viewport_progress = (window_height - element_top) / (window_height + element_height);
                    viewport_progress.clamp(0.0, 1.0)
                } else if element_top < 0.0 {
                    // Element is above viewport
                    1.0
                } else {
                    // Element is below viewport
                    0.0
                };
                
                set_scroll_progress.set(progress);
            }) as Box<dyn FnMut()>);
            
            window.add_event_listener_with_callback(
                "scroll",
                scroll_callback.as_ref().unchecked_ref(),
            ).unwrap();
            
            scroll_callback.forget();
        }
    });

    view! {
        <section style="
            height: 100vh;
            max-height: 400px;
            display: flex;
            justify-content: center;
            align-items: center;
            position: relative;
        ">
            <div 
                node_ref=item_ref
                style="
                    width: 200px;
                    height: 250px;
                    border: 2px dotted #ff0088;
                    position: relative;
                    background: rgba(255,255,255,0.05);
                    border-radius: 10px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    backdrop-filter: blur(10px);
                "
            >
                // Progress Icon Container
                <figure style="
                    position: sticky;
                    top: 0;
                    width: 80px;
                    height: 80px;
                    margin: 0;
                    padding: 0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">
                    <svg
                        style="
                            stroke-dashoffset: 0;
                            stroke-width: 5;
                            fill: none;
                            transform: translateX(-100px) rotate(-90deg);
                            stroke: #ff0088;
                        "
                        width="75"
                        height="75"
                        viewBox="0 0 100 100"
                    >
                        // Background circle
                        <circle
                            style="
                                opacity: 0.2;
                                stroke-dashoffset: 0;
                                stroke-width: 5;
                                fill: none;
                            "
                            cx="50"
                            cy="50"
                            r="30"
                            pathLength="1"
                        />
                        // Progress circle
                        <circle
                            cx="50"
                            cy="50"
                            r="30"
                            pathLength="1"
                            style=move || format!("
                                stroke-dashoffset: {};
                                stroke-width: 5;
                                fill: none;
                                stroke: #ff0088;
                                transition: stroke-dashoffset 0.1s ease;
                            ", 
                            // Convert progress (0-1) to stroke-dashoffset (1-0)
                            // When progress is 0, dashoffset is 1 (no stroke visible)
                            // When progress is 1, dashoffset is 0 (full stroke visible)
                            1.0 - scroll_progress.get()
                            )
                        />
                    </svg>
                </figure>
                
                // Item number
                <div style="
                    position: absolute;
                    top: 10px;
                    left: 10px;
                    color: #ff0088;
                    font-weight: bold;
                    font-size: 1.2rem;
                ">
                    {key}
                </div>
                
                // Progress percentage
                <div style="
                    position: absolute;
                    bottom: 10px;
                    right: 10px;
                    color: #ff0088;
                    font-weight: bold;
                    font-size: 0.9rem;
                ">
                    {move || format!("{:.0}%", scroll_progress.get() * 100.0)}
                </div>
            </div>
        </section>
    }
}
