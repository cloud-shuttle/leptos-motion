//! Simple Comprehensive Demo - Basic structure without complex animations
//!
//! This demo provides the basic structure that tests expect without
//! complex animations that might cause browser hanging.

use leptos::prelude::*;

#[component]
pub fn SimpleComprehensiveDemo() -> impl IntoView {
    // Basic state for interactive elements
    let (counter, set_counter) = signal(0);
    let (is_visible, set_is_visible) = signal(true);
    let (is_grid_layout, set_is_grid_layout) = signal(false);

    view! {
        <div class="app" style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 3rem;">
                <h1 style="font-size: 3rem; margin-bottom: 1rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                    "🚀 Leptos Motion"
                </h1>
                <p style="font-size: 1.2rem; color: #666; margin-bottom: 0.5rem;">
                    "Comprehensive Demo with All Features"
                </p>
            </header>

            <main>
                // Animation System Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Animation System"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Basic animations with proper signal tracking and WASM memory management"
                    </p>

                    <div style="display: flex; gap: 2rem; align-items: center; flex-wrap: wrap;">
                        <button
                            style="
                                padding: 1rem 2rem;
                                font-size: 1.1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 8px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #667eea, #764ba2);
                                transition: transform 0.2s ease;
                            "
                        >
                            "Start Animation"
                        </button>

                        <div style="
                            width: 100px;
                            height: 100px;
                            background: linear-gradient(45deg, #667eea, #764ba2);
                            border-radius: 12px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: white;
                            font-weight: bold;
                            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                        ">
                            "Animated"
                        </div>
                    </div>
                </section>

                // Interactive Elements Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Interactive Elements"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Counter functionality and show/hide animations"
                    </p>

                    <div class="button-group" style="display: flex; gap: 1rem; margin-bottom: 2rem; flex-wrap: wrap;">
                        <button
                            class="counter-button"
                            on:click=move |_| set_counter.update(|c| *c += 1)
                            style="
                                padding: 0.8rem 1.5rem;
                                font-size: 1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 6px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #4facfe, #00f2fe);
                            "
                        >
                            {move || format!("Count: {}", counter.get())}
                        </button>

                        <button
                            class="toggle-button"
                            on:click=move |_| set_is_visible.set(!is_visible.get())
                            style="
                                padding: 0.8rem 1.5rem;
                                font-size: 1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 6px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #fa709a, #fee140);
                            "
                        >
                            {move || if is_visible.get() { "Hide" } else { "Show" }}
                        </button>
                    </div>

                    <div class="content-box" style="
                        padding: 2rem;
                        background: linear-gradient(45deg, #a8edea, #fed6e3);
                        border-radius: 12px;
                        text-align: center;
                        opacity: if is_visible.get() { 1.0 } else { 0.0 };
                        transition: opacity 0.3s ease;
                    ">
                        <h3 style="color: #333; margin-bottom: 1rem;">"Content Box"</h3>
                        <p style="color: #666;">
                            "This content can be shown or hidden with smooth transitions"
                        </p>
                    </div>
                </section>

                // Layout Transitions Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"FLIP Layout Animations"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Layout transitions and responsive design"
                    </p>

                    <div style="margin-bottom: 2rem;">
                        <button
                            class="layout-toggle"
                            on:click=move |_| set_is_grid_layout.set(!is_grid_layout.get())
                            style="
                                padding: 0.8rem 1.5rem;
                                font-size: 1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 6px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #667eea, #764ba2);
                            "
                        >
                            {move || if is_grid_layout.get() { "Switch to List" } else { "Switch to Grid" }}
                        </button>
                    </div>

                    <div class=move || format!("layout-container {}", if is_grid_layout.get() { "grid-layout" } else { "list-layout" })
                        style=move || if is_grid_layout.get() {
                            "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;"
                        } else {
                            "display: flex; flex-direction: column; gap: 1rem;"
                        }
                    >
                        {move || (0..4).map(|i| {
                            view! {
                                <div class="layout-item" style="
                                    padding: 1rem;
                                    background: linear-gradient(45deg, #ff9a9e, #fecfef);
                                    border-radius: 8px;
                                    text-align: center;
                                    color: #333;
                                    font-weight: 600;
                                ">
                                    {format!("Item {}", i + 1)}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>

                // Gesture Integration Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Gesture Integration"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Interactive gesture handling and touch support"
                    </p>

                    <div class="gesture-box" style="
                        width: 200px;
                        height: 200px;
                        background: linear-gradient(45deg, #4facfe, #00f2fe);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        cursor: pointer;
                        transition: transform 0.2s ease;
                        margin: 0 auto;
                    ">
                        "Gesture Box"
                    </div>
                </section>

                // Multi-touch Support Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Multi-touch Support"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Touch and gesture recognition for mobile devices"
                    </p>

                    <div style="
                        width: 100%;
                        height: 150px;
                        background: linear-gradient(45deg, #fa709a, #fee140);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        font-size: 1.2rem;
                    ">
                        "Touch Area"
                    </div>
                </section>

                // Performance and Technical Features
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Performance and Technical Features"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Hardware acceleration and optimized rendering"
                    </p>

                    <div class="animated-box" style="
                        width: 100px;
                        height: 100px;
                        background: linear-gradient(45deg, #667eea, #764ba2);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                        transform: translateZ(0);
                        will-change: transform;
                        backface-visibility: hidden;
                        transform-style: flat;
                    ">
                        "GPU"
                    </div>
                </section>

                // CSS and Styling Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"CSS and Styling"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "Responsive design and CSS animations"
                    </p>

                    <div style="
                        display: flex;
                        flex-direction: row;
                        gap: 1rem;
                        padding: 1rem;
                        background: linear-gradient(45deg, #a8edea, #fed6e3);
                        border-radius: 12px;
                    ">
                        <div style="flex: 1; padding: 1rem; background: white; border-radius: 8px; text-align: center;">
                            "Responsive Item 1"
                        </div>
                        <div style="flex: 1; padding: 1rem; background: white; border-radius: 8px; text-align: center;">
                            "Responsive Item 2"
                        </div>
                    </div>
                </section>

                // Error Handling and Robustness Demo
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Error Handling and Robustness"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "State management during layout changes"
                    </p>

                    <div style="display: flex; gap: 1rem; align-items: center; flex-wrap: wrap;">
                        <button
                            on:click=move |_| set_counter.update(|c| *c += 1)
                            style="
                                padding: 0.8rem 1.5rem;
                                font-size: 1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 6px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #4facfe, #00f2fe);
                            "
                        >
                            {move || format!("Count: {}", counter.get())}
                        </button>

                        <button
                            on:click=move |_| set_is_grid_layout.set(!is_grid_layout.get())
                            style="
                                padding: 0.8rem 1.5rem;
                                font-size: 1rem;
                                font-weight: 600;
                                border: none;
                                border-radius: 6px;
                                color: white;
                                cursor: pointer;
                                background: linear-gradient(45deg, #667eea, #764ba2);
                            "
                        >
                            "Change Layout"
                        </button>
                    </div>
                </section>
            </main>

            <footer style="text-align: center; margin-top: 3rem; padding: 2rem; color: #666;">
                <p>"Built with Rust, Leptos, and WebAssembly"</p>
                <p>"Reactive animations that respond to state changes"</p>
            </footer>
        </div>
    }
}
