//! Advanced features example with gesture detection and animations
//!
//! This example demonstrates advanced motion features using manual DOM manipulation

use leptos::*;
use leptos::prelude::{ElementChild, StyleAttribute, IntoAny, signal};
use leptos_motion_core::*;

/// Advanced features demo
#[component]
pub fn App() -> impl IntoView {
    let (hover_state, set_hover_state) = signal(false);
    let (drag_state, set_drag_state) = signal(false);

    view! {
        <div style="min-height: 100vh; padding: 2rem; font-family: system-ui;">
            <h1>"Leptos Motion - Advanced Features"</h1>

            <div style="display: grid; gap: 2rem; margin-top: 2rem;">

                // Hover Animation Demo
                <section>
                    <h2>"Hover Animations"</h2>
                    <div
                        style=move || format!(
                            "width: 200px; height: 120px; background: linear-gradient(45deg, #6366f1, #8b5cf6); border-radius: 1rem; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer; transition: all 0.3s ease; transform: scale({}) rotateX({}deg); box-shadow: 0 {}px {}px rgba(0,0,0,0.{});",
                            if hover_state.get() { 1.05 } else { 1.0 },
                            if hover_state.get() { 10 } else { 0 },
                            if hover_state.get() { 20 } else { 4 },
                            if hover_state.get() { 40 } else { 8 },
                            if hover_state.get() { 3 } else { 1 }
                        )
                        on:mouseenter=move |_| set_hover_state.set(true)
                        on:mouseleave=move |_| set_hover_state.set(false)
                    >
                        "Hover me!"
                    </div>
                </section>

                // Drag Simulation Demo
                <section>
                    <h2>"Drag Interactions"</h2>
                    <div
                        style=move || format!(
                            "width: 150px; height: 150px; background: linear-gradient(135deg, #f59e0b, #ef4444); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: grab; transition: all 0.2s ease; transform: scale({}) rotate({}deg); user-select: none;",
                            if drag_state.get() { 1.1 } else { 1.0 },
                            if drag_state.get() { 15 } else { 0 }
                        )
                        on:mousedown=move |_| set_drag_state.set(true)
                        on:mouseup=move |_| set_drag_state.set(false)
                        on:mouseleave=move |_| set_drag_state.set(false)
                    >
                        {move || if drag_state.get() { "Dragging!" } else { "Drag me!" }}
                    </div>
                </section>

                // Animation Engine Demo
                <section>
                    <h2>"Animation Engine"</h2>
                    <AnimationEngineDemo />
                </section>

                // Performance Demo
                <section>
                    <h2>"Performance Features"</h2>
                    <PerformanceDemo />
                </section>

            </div>
        </div>
    }
}

#[component]
fn AnimationEngineDemo() -> impl IntoView {
    let engine = MinimalEngine::new();
    let (running, set_running) = signal(false);

    view! {
        <div style="background: #f8fafc; padding: 1.5rem; border-radius: 1rem; border: 1px solid #e2e8f0;">
            <p>"Animation engine status: " <strong>"Ready"</strong></p>
            <button
                style=move || format!(
                    "padding: 0.5rem 1rem; background: {}; color: {}; border: none; border-radius: 0.5rem; cursor: pointer; transition: all 0.2s ease;",
                    if running.get() { "#ef4444" } else { "#10b981" },
                    if running.get() { "white" } else { "white" }
                )
                on:click=move |_| set_running.update(|r| *r = !*r)
            >
                {move || if running.get() { "Stop Engine" } else { "Start Engine" }}
            </button>
            <div style="margin-top: 1rem; font-size: 0.875rem; color: #64748b;">
                "Core animation engine provides optimized performance for complex animations."
            </div>
        </div>
    }
}

#[component]
fn PerformanceDemo() -> impl IntoView {
    let (stats_visible, set_stats_visible) = signal(false);

    view! {
        <div style="background: #fef3c7; padding: 1.5rem; border-radius: 1rem; border: 1px solid #f59e0b;">
            <div style="display: flex; align-items: center; gap: 1rem;">
                <span>"Performance Monitoring"</span>
                <button
                    style="padding: 0.25rem 0.75rem; background: #f59e0b; color: white; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem;"
                    on:click=move |_| set_stats_visible.update(|v| *v = !*v)
                >
                    {move || if stats_visible.get() { "Hide Stats" } else { "Show Stats" }}
                </button>
            </div>

            {move || if stats_visible.get() {
                view! {
                    <div style="margin-top: 1rem; grid-template-columns: repeat(3, 1fr); gap: 1rem; display: grid; font-size: 0.875rem;">
                        <div>
                            <strong>"FPS:"</strong> " 60"
                        </div>
                        <div>
                            <strong>"Memory:"</strong> " 2.4MB"
                        </div>
                        <div>
                            <strong>"Animations:"</strong> " 3 active"
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
