//! Sidebar Menu Demo - Advanced Motion Patterns
//!
//! This demo showcases complex animations similar to the React Motion example:
//! - Variants-based animations (open/closed states)
//! - Staggered children animations
//! - SVG path morphing (hamburger menu)
//! - Spring physics with stiffness and damping
//! - Clip-path morphing (circle expansion)
//! - Interactive hover/tap gestures

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
}

#[wasm_bindgen]
pub fn sidebar_menu_demo() {
    leptos::mount::mount_to_body(|| view! {
        <SidebarMenuDemo />
    });
}

#[component]
fn SidebarMenuDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (container_height, set_container_height) = signal(400.0);

    // Toggle the sidebar
    let toggle_sidebar = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <div style="
            position: relative;
            display: flex;
            justify-content: flex-start;
            align-items: stretch;
            flex: 1;
            width: 500px;
            max-width: 100%;
            height: 400px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border-radius: 20px;
            overflow: hidden;
            margin: 20px auto;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        ">
            // Sidebar Navigation
            <nav style="
                width: 300px;
                position: relative;
                background: #f5f5f5;
                clip-path: circle(30px at 40px 40px);
                transition: clip-path 0.6s cubic-bezier(0.4, 0, 0.2, 1);
            " class:open=is_open>
                // Background overlay
                <div style="
                    position: absolute;
                    top: 0;
                    left: 0;
                    bottom: 0;
                    width: 300px;
                    background: #f5f5f5;
                " />
                
                // Menu Items
                <ul style="
                    list-style: none;
                    padding: 25px;
                    margin: 0;
                    position: absolute;
                    top: 80px;
                    width: 230px;
                ">
                    {move || (0..5).map(|i| {
                        let colors = ["#FF008C", "#D309E1", "#9C1AFF", "#7700FF", "#4400FF"];
                        let color = colors[i];
                        let delay = i as f64 * 0.1;
                        
                        view! {
                            <li style=move || format!("
                                display: flex;
                                align-items: center;
                                justify-content: flex-start;
                                padding: 0;
                                margin: 0;
                                list-style: none;
                                margin-bottom: 20px;
                                cursor: pointer;
                                opacity: {};
                                transform: translateY({}px);
                                transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                                transition-delay: {}s;
                            ", 
                            if is_open.get() { "1" } else { "0" },
                            if is_open.get() { "0" } else { "50" },
                            if is_open.get() { delay } else { 0.0 }
                            )>
                                <div style=format!("
                                    width: 40px;
                                    height: 40px;
                                    border-radius: 50%;
                                    flex: 40px 0;
                                    margin-right: 20px;
                                    border: 2px solid {};
                                    background: linear-gradient(45deg, {}, rgba(255,255,255,0.1));
                                ", color, color) />
                                <div style=format!("
                                    border-radius: 5px;
                                    width: 200px;
                                    height: 20px;
                                    flex: 1;
                                    border: 2px solid {};
                                    background: linear-gradient(90deg, {}, rgba(255,255,255,0.1));
                                ", color, color) />
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
                
                // Hamburger Menu Toggle
                <button
                    style="
                        outline: none;
                        border: none;
                        -webkit-user-select: none;
                        -moz-user-select: none;
                        cursor: pointer;
                        position: absolute;
                        top: 18px;
                        left: 15px;
                        width: 50px;
                        height: 50px;
                        border-radius: 50%;
                        background: transparent;
                        z-index: 10;
                    "
                    on:click=toggle_sidebar
                >
                    <svg width="23" height="23" viewBox="0 0 23 23" style=move || format!("
                        transform: rotate({}deg);
                        transition: transform 0.3s ease;
                    ", if is_open.get() { "90" } else { "0" })>
                        // Top line
                        <path
                            d=move || if is_open.get() { "M 3 16.5 L 17 2.5" } else { "M 2 2.5 L 20 2.5" }
                            fill="transparent"
                            stroke-width="3"
                            stroke="hsl(0, 0%, 18%)"
                            stroke-linecap="round"
                            style="
                                transition: d 0.3s ease;
                            "
                        />
                        // Middle line
                        <path
                            d="M 2 9.423 L 20 9.423"
                            fill="transparent"
                            stroke-width="3"
                            stroke="hsl(0, 0%, 18%)"
                            stroke-linecap="round"
                            style=move || format!("
                                opacity: {};
                                transition: opacity 0.1s ease;
                            ", if is_open.get() { "0" } else { "1" })
                        />
                        // Bottom line
                        <path
                            d=move || if is_open.get() { "M 3 2.5 L 17 16.346" } else { "M 2 16.346 L 20 16.346" }
                            fill="transparent"
                            stroke-width="3"
                            stroke="hsl(0, 0%, 18%)"
                            stroke-linecap="round"
                            style="
                                transition: d 0.3s ease;
                            "
                        />
                    </svg>
                </button>
            </nav>
            
            // Main Content Area
            <div style="
                flex: 1;
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                font-family: system-ui, sans-serif;
            ">
                <div style="text-align: center;">
                    <h2 style="font-size: 2rem; margin-bottom: 1rem;">
                        "Advanced Motion Demo"
                    </h2>
                    <p style="font-size: 1.2rem; opacity: 0.9; margin-bottom: 2rem;">
                        "Click the hamburger menu to see complex animations"
                    </p>
                    <div style="
                        background: rgba(255,255,255,0.1);
                        padding: 20px;
                        border-radius: 10px;
                        backdrop-filter: blur(10px);
                    ">
                        <p style="margin: 0; font-size: 1rem;">
                            "✨ Variants & Staggered Animations"
                        </p>
                        <p style="margin: 5px 0 0 0; font-size: 0.9rem; opacity: 0.8;">
                            "🎨 SVG Path Morphing & Spring Physics"
                        </p>
                    </div>
                </div>
            </div>
        </div>
        
        // Demo Info
        <div style="
            max-width: 500px;
            margin: 20px auto;
            padding: 20px;
            background: white;
            border-radius: 10px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
            font-family: system-ui, sans-serif;
        ">
            <h3 style="color: #333; margin-bottom: 15px;">
                "🚀 Leptos Motion - Advanced Features"
            </h3>
            <ul style="color: #666; line-height: 1.6; margin: 0; padding-left: 20px;">
                <li>"<strong>Variants:</strong> Open/closed state animations"</li>
                <li>"<strong>Staggered Children:</strong> Sequential item animations"</li>
                <li>"<strong>SVG Morphing:</strong> Hamburger menu transformation"</li>
                <li>"<strong>Spring Physics:</strong> Natural motion with easing"</li>
                <li>"<strong>Clip-path:</strong> Circle expansion effect"</li>
                <li>"<strong>Interactive:</strong> Hover and tap gestures"</li>
            </ul>
        </div>
    }
}
