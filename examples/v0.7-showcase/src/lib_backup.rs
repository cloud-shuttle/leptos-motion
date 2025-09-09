use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_dom::{ReactiveMotionDiv, reactive_animate};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="showcase-grid">
            <SpringPhysicsDemo />
            <VariantsDemo />
            <TimelineDemo />
            <PerformanceDemo />
        </div>
    }
}

#[component]
fn SpringPhysicsDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    view! {
        <div class="showcase-card">
            <div class="feature-badge">"🌊 Spring Physics"</div>
            <h3>"Spring Physics Engine"</h3>
            <p>"Natural, physics-based animations with configurable tension, friction, and mass parameters."</p>
            
            <div class="demo-area">
                <ReactiveMotionDiv
                    class="w-20 h-20 bg-green-500 rounded-full cursor-pointer".to_string()
                    animate=reactive_animate(move || {
                        if is_active.get() {
                            HashMap::from([
                                ("x".to_string(), AnimationValue::Pixels(200.0)),
                                ("scale".to_string(), AnimationValue::Number(1.2)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(34, 197, 94)".to_string()))
                            ])
                        } else {
                            HashMap::from([
                                ("x".to_string(), AnimationValue::Pixels(0.0)),
                                ("scale".to_string(), AnimationValue::Number(1.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(34, 197, 94)".to_string()))
                            ])
                        }
                    })
                    _transition=Transition {
                        duration: Some(1.0),
                        ease: Easing::Spring(SpringConfig::default()),
                        ..Default::default()
                    }
                    on:click=move |_| set_is_active.update(|x| *x = !*x)
                >
                    "Click me!"
                </ReactiveMotionDiv>
            </div>
            
            <div class="controls">
                <button
                    class="btn btn-primary"
                    on:click=move |_| set_is_active.update(|x| *x = !*x)
                >
                    {move || if is_active.get() { "Reset" } else { "Animate" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn VariantsDemo() -> impl IntoView {
    let (current_variant, set_current_variant) = signal("idle".to_string());
    
    view! {
        <div class="showcase-card">
            <div class="feature-badge">"🎭 Variants System"</div>
            <h3>"Named Animation States"</h3>
            <p>"Define reusable animation states with descriptive names for smooth transitions between different states."</p>
            
            <div class="demo-area">
                <ReactiveMotionDiv
                    class="w-24 h-24 rounded-lg cursor-pointer flex items-center justify-center text-white font-bold text-lg".to_string()
                    animate=reactive_animate(move || {
                        match current_variant.get().as_str() {
                            "idle" => HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(1.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("#a78bfa".to_string())),
                                ("rotate".to_string(), AnimationValue::Number(0.0))
                            ]),
                            "hover" => HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(1.1)),
                                ("backgroundColor".to_string(), AnimationValue::String("#8b5cf6".to_string())),
                                ("rotate".to_string(), AnimationValue::Number(5.0))
                            ]),
                            "active" => HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(0.95)),
                                ("backgroundColor".to_string(), AnimationValue::String("#7c3aed".to_string())),
                                ("rotate".to_string(), AnimationValue::Number(-5.0))
                            ]),
                            _ => HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(1.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("#a78bfa".to_string())),
                                ("rotate".to_string(), AnimationValue::Number(0.0))
                            ])
                        }
                    })
                    on:mouseenter=move |_| set_current_variant.set("hover".to_string())
                    on:mouseleave=move |_| set_current_variant.set("idle".to_string())
                    on:click=move |_| set_current_variant.set("active".to_string())
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    {move || match current_variant.get().as_str() {
                        "idle" => "Idle",
                        "hover" => "Hover",
                        "active" => "Active",
                        _ => "Unknown"
                    }}
                </ReactiveMotionDiv>
            </div>
            
            <div class="controls">
                <button
                    class="btn btn-primary"
                    on:click=move |_| set_current_variant.set("idle".to_string())
                >
                    "Idle"
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| set_current_variant.set("hover".to_string())
                >
                    "Hover"
                </button>
                <button
                    class="btn btn-accent"
                    on:click=move |_| set_current_variant.set("active".to_string())
                >
                    "Active"
                </button>
            </div>
        </div>
    }
}

#[component]
fn TimelineDemo() -> impl IntoView {
    let (is_playing, set_is_playing) = signal(false);
    let (current_step, set_current_step) = signal(0);
    
    view! {
        <div class="showcase-card">
            <div class="feature-badge">"⏰ Timeline Sequences"</div>
            <h3>"Advanced Orchestration"</h3>
            <p>"Create complex, multi-step animation sequences with precise timing and orchestration control."</p>
            
            <div class="demo-area">
                <ReactiveMotionDiv
                    class="w-20 h-20 bg-orange-500 rounded-lg".to_string()
                    animate=reactive_animate(move || {
                        let step = current_step.get();
                        match step {
                            0 => HashMap::from([
                                ("opacity".to_string(), AnimationValue::Number(1.0)),
                                ("scale".to_string(), AnimationValue::Number(1.0)),
                                ("x".to_string(), AnimationValue::Pixels(0.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(249, 115, 22)".to_string()))
                            ]),
                            1 => HashMap::from([
                                ("opacity".to_string(), AnimationValue::Number(0.8)),
                                ("scale".to_string(), AnimationValue::Number(1.2)),
                                ("x".to_string(), AnimationValue::Pixels(50.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(234, 88, 12)".to_string()))
                            ]),
                            2 => HashMap::from([
                                ("opacity".to_string(), AnimationValue::Number(0.6)),
                                ("scale".to_string(), AnimationValue::Number(0.8)),
                                ("x".to_string(), AnimationValue::Pixels(100.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(194, 65, 12)".to_string()))
                            ]),
                            _ => HashMap::from([
                                ("opacity".to_string(), AnimationValue::Number(1.0)),
                                ("scale".to_string(), AnimationValue::Number(1.0)),
                                ("x".to_string(), AnimationValue::Pixels(0.0)),
                                ("backgroundColor".to_string(), AnimationValue::String("rgb(249, 115, 22)".to_string()))
                            ])
                        }
                    })
                    _transition=Transition {
                        duration: Some(0.5),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    {move || format!("Step {}", current_step.get())}
                </ReactiveMotionDiv>
            </div>
            
            <div class="controls">
                <button
                    class="btn btn-primary"
                    on:click=move |_| {
                        if is_playing.get_untracked() {
                            set_is_playing.set(false);
                        } else {
                            set_is_playing.set(true);
                            set_current_step.set(0);
                        }
                    }
                >
                    {move || if is_playing.get() { "Stop" } else { "Play" }}
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| set_current_step.set(0)
                >
                    "Reset"
                </button>
                <button
                    class="btn btn-accent"
                    on:click=move |_| {
                        let next = (current_step.get_untracked() + 1) % 5;
                        set_current_step.set(next);
                    }
                >
                    "Next Step"
                </button>
            </div>
        </div>
    }
}

#[component]
fn PerformanceDemo() -> impl IntoView {
    let (element_count, set_element_count) = signal(5);
    let (is_animating, set_is_animating) = signal(false);
    
    view! {
        <div class="showcase-card">
            <div class="feature-badge">"⚡ Performance"</div>
            <h3>"Performance Optimizations"</h3>
            <p>"Memory pools, caching, and edge case handling for optimal performance in production applications."</p>
            
            <div class="demo-area">
                <div class="grid grid-cols-5 gap-2">
                    {move || (0..element_count.get()).map(|_i| {
                        view! {
                            <ReactiveMotionDiv
                                class="w-8 h-8 bg-red-500 rounded".to_string()
                                animate=reactive_animate(move || {
                                    if is_animating.get() {
                                        HashMap::from([
                                            ("scale".to_string(), AnimationValue::Number(1.5)),
                                            ("rotate".to_string(), AnimationValue::Number(360.0))
                                        ])
                                    } else {
                                        HashMap::from([
                                            ("scale".to_string(), AnimationValue::Number(1.0)),
                                            ("rotate".to_string(), AnimationValue::Number(0.0))
                                        ])
                                    }
                                })
                                _transition=Transition {
                                    duration: Some(0.5),
                                    ease: Easing::EaseInOut,
                                    ..Default::default()
                                }
                            >
                                ""
                            </ReactiveMotionDiv>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            
            <div class="controls">
                <button
                    class="btn btn-primary"
                    on:click=move |_| set_is_animating.update(|x| *x = !*x)
                >
                    {move || if is_animating.get() { "Stop" } else { "Animate" }}
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| set_element_count.set(5)
                >
                    "5 Elements"
                </button>
                <button
                    class="btn btn-accent"
                    on:click=move |_| set_element_count.set(25)
                >
                    "25 Elements"
                </button>
            </div>
        </div>
    }
}