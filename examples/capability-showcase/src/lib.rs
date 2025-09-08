//! Leptos Motion Capability Showcase
//!
//! A comprehensive demonstration of all the animation capabilities
//! available in the Leptos Motion library.

use js_sys;
use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;
use web_sys::console;

/// Main showcase component
#[component]
pub fn CapabilityShowcase() -> impl IntoView {
    view! {
        <div class="showcase-grid">
            <BasicAnimationsDemo/>
            <GestureInteractionsDemo/>
            <LayoutAnimationsDemo/>
            <KeyframeAnimationsDemo/>
            <StaggerAnimationsDemo/>
            <DragConstraintsDemo/>
            <PerformanceDemo/>
            <AdvancedFeaturesDemo/>
        </div>
    }
}

/// Basic animations showcase
#[component]
fn BasicAnimationsDemo() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);
    let (scale_mode, set_scale_mode) = signal(0);

    let create_animation_target = |visible: bool, mode: i32| -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();

        match mode {
            0 => {
                // Scale & Fade
                target.insert(
                    "opacity".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.0 }),
                );
                target.insert(
                    "scale".to_string(),
                    AnimationValue::Number(if visible { 1.0 } else { 0.3 }),
                );
            }
            1 => {
                // Rotate & Slide
                target.insert(
                    "rotate".to_string(),
                    AnimationValue::Number(if visible { 0.0 } else { 180.0 }),
                );
                target.insert(
                    "x".to_string(),
                    AnimationValue::Pixels(if visible { 0.0 } else { 100.0 }),
                );
            }
            _ => {
                // Color & Border
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(if visible {
                        "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string()
                    } else {
                        "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string()
                    }),
                );
                target.insert(
                    "border-radius".to_string(),
                    AnimationValue::Pixels(if visible { 10.0 } else { 50.0 }),
                );
            }
        }

        target
    };

    view! {
        <div class="showcase-card">
            <h3>"🎨 Basic Animations"</h3>
            <p>"Scale, rotate, fade, and color transitions"</p>

            <div class="demo-area">
                <MotionDiv
                    class="w-20 h-20 mx-auto flex items-center justify-center text-white font-bold text-lg rounded-xl shadow-lg bg-gradient-to-br from-blue-500 to-purple-600".to_string()
                    initial=create_animation_target(true, scale_mode.get())
                    animate=create_animation_target(is_visible.get(), scale_mode.get())
                    transition=Transition {
                        duration: Some(0.6),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    "✨"
                </MotionDiv>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_is_visible.update(|v| *v = !*v)
                >
                    {move || if is_visible.get() { "Hide" } else { "Show" }}
                </button>
                <button
                    class="btn secondary"
                    on:click=move |_| set_scale_mode.update(|m| *m = (*m + 1) % 3)
                >
                    "Mode " {scale_mode.get() + 1}
                </button>
            </div>
        </div>
    }
}

/// Gesture interactions showcase
#[component]
fn GestureInteractionsDemo() -> impl IntoView {
    let (tap_count, set_tap_count) = signal(0);
    let (hover_state, set_hover_state) = signal(false);

    let hover_target = move || {
        let mut target = HashMap::new();
        if hover_state.get() {
            target.insert("scale".to_string(), AnimationValue::Number(1.2));
            target.insert("rotate".to_string(), AnimationValue::Number(5.0));
            target.insert(
                "box-shadow".to_string(),
                AnimationValue::String("0 20px 40px rgba(0,0,0,0.3)".to_string()),
            );
        } else {
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
            target.insert("rotate".to_string(), AnimationValue::Number(0.0));
            target.insert(
                "box-shadow".to_string(),
                AnimationValue::String("0 5px 15px rgba(0,0,0,0.1)".to_string()),
            );
        }
        target
    };

    let tap_target = move || {
        let mut target = HashMap::new();
        target.insert("scale".to_string(), AnimationValue::Number(0.9));
        target
    };

    view! {
        <div class="showcase-card">
            <h3>"🖱️ Gesture Interactions"</h3>
            <p>"Hover, tap, and drag interactions"</p>

            <div class="demo-area">
                <MotionDiv
                    class="w-24 h-24 mx-auto cursor-pointer flex items-center justify-center text-white font-bold text-sm rounded-2xl shadow-xl from-pink-400 to-red-500".to_string()
                    while_hover=hover_target()
                    while_tap=tap_target()
                    on:mouseenter=move |_| set_hover_state.set(true)
                    on:mouseleave=move |_| set_hover_state.set(false)
                    on:click=move |_| set_tap_count.update(|c| *c += 1)
                    transition=Transition {
                        duration: Some(0.2),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    "👆 " {tap_count.get()}
                </MotionDiv>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_tap_count.set(0)
                >
                    "Reset Counter"
                </button>
            </div>
        </div>
    }
}

/// Layout animations showcase
#[component]
fn LayoutAnimationsDemo() -> impl IntoView {
    let (items, set_items) = signal(vec![1, 2, 3, 4, 5]);
    let (layout_enabled, _set_layout_enabled) = signal(true);

    let add_item = move |_| {
        set_items.update(|items| {
            let new_id = items.iter().max().unwrap_or(&0) + 1;
            items.push(new_id);
        });
    };

    let remove_item = move |_| {
        set_items.update(|items| {
            if !items.is_empty() {
                items.pop();
            }
        });
    };

    let shuffle_items = move |_| {
        set_items.update(|items| {
            use std::collections::HashSet;
            let mut new_items = Vec::new();
            let mut used = HashSet::new();

            while new_items.len() < items.len() {
                let random = (js_sys::Math::random() * items.len() as f64) as usize;
                if !used.contains(&random) {
                    new_items.push(items[random]);
                    used.insert(random);
                }
            }
            *items = new_items;
        });
    };

    view! {
        <div class="showcase-card">
            <h3>"📱 Layout Animations"</h3>
            <p>"FLIP-based smooth layout transitions"</p>

            <div class="demo-area">
                <div style="display: flex; flex-wrap: wrap; gap: 10px; min-height: 120px;">
                    <For
                        each=move || items.get()
                        key=|item| *item
                        children=move |item| {
                            view! {
                                <MotionDiv
                                    class=format!("w-12 h-12 flex items-center justify-center text-white font-bold rounded-lg shadow-md color-{}", item)
                                    layout=layout_enabled.get()
                                    transition=Transition {
                                        duration: Some(0.3),
                                        ease: Easing::EaseInOut,
                                        ..Default::default()
                                    }
                                >
                                    {item}
                                </MotionDiv>
                            }
                        }
                    />
                </div>
            </div>

            <div class="controls">
                <button class="btn" on:click=add_item>"Add Item"</button>
                <button class="btn secondary" on:click=remove_item>"Remove Item"</button>
                <button class="btn secondary" on:click=shuffle_items>"Shuffle"</button>
            </div>
        </div>
    }
}

/// Keyframe animations showcase
#[component]
fn KeyframeAnimationsDemo() -> impl IntoView {
    let (is_playing, set_is_playing) = signal(false);

    let keyframe_target = move || {
        let mut target = HashMap::new();
        if is_playing.get() {
            // Complex keyframe-like animation
            target.insert("x".to_string(), AnimationValue::Pixels(100.0));
            target.insert("y".to_string(), AnimationValue::Pixels(-50.0));
            target.insert("rotate".to_string(), AnimationValue::Number(360.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.5));
        } else {
            target.insert("x".to_string(), AnimationValue::Pixels(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
            target.insert("rotate".to_string(), AnimationValue::Number(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
        }
        target
    };

    view! {
        <div class="showcase-card">
            <h3>"🎬 Keyframe Animations"</h3>
            <p>"Multi-step animations with precise control"</p>

            <div class="demo-area">
                <MotionDiv
                    class="w-16 h-16 mx-auto flex items-center justify-center text-white font-bold text-sm rounded-xl shadow-lg from-orange-200 to-orange-400".to_string()
                    initial=keyframe_target()
                    animate=keyframe_target()
                    transition=Transition {
                        duration: Some(2.0),
                        ease: Easing::EaseInOut,
                        repeat: RepeatConfig::Infinite,
                        ..Default::default()
                    }
                >
                    "🎬"
                </MotionDiv>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_is_playing.update(|p| *p = !*p)
                >
                    {move || if is_playing.get() { "Stop" } else { "Play" }}
                </button>
            </div>
        </div>
    }
}

/// Stagger animations showcase
#[component]
fn StaggerAnimationsDemo() -> impl IntoView {
    let (is_staggered, set_is_staggered) = signal(false);

    let stagger_items = vec![1, 2, 3, 4, 5];

    let create_stagger_target = |index: usize, active: bool| -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();
        if active {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(50.0));
            target.insert("scale".to_string(), AnimationValue::Number(0.8));
        }
        target
    };

    view! {
        <div class="showcase-card">
            <h3>"⚡ Stagger Animations"</h3>
            <p>"Sequential animations with configurable delays"</p>

            <div class="demo-area">
                <div style="display: flex; gap: 10px; justify-content: center; flex-wrap: wrap;">
                    {stagger_items.into_iter().enumerate().map(|(index, item)| {
                        view! {
                            <MotionDiv
                                class=format!("w-12 h-12 flex items-center justify-center text-white font-bold rounded-lg shadow-md stagger-{}", index + 1)
                                initial=create_stagger_target(index, false)
                                animate=create_stagger_target(index, is_staggered.get())
                                transition=Transition {
                                    duration: Some(0.5),
                                    delay: Some(index as f64 * 0.1),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                }
                            >
                                {item}
                            </MotionDiv>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_is_staggered.update(|s| *s = !*s)
                >
                    {move || if is_staggered.get() { "Hide Staggered" } else { "Show Staggered" }}
                </button>
            </div>
        </div>
    }
}

/// Drag constraints showcase
#[component]
fn DragConstraintsDemo() -> impl IntoView {
    let (constraint_mode, set_constraint_mode) = signal(0);

    let get_drag_config = |mode: i32| -> DragConfig {
        match mode {
            0 => DragConfig {
                axis: Some(DragAxis::Both),
                constraints: None,
                elastic: Some(0.2),
                momentum: Some(true),
            },
            1 => DragConfig {
                axis: Some(DragAxis::X),
                constraints: Some(DragConstraints {
                    left: Some(-50.0),
                    right: Some(50.0),
                    top: None,
                    bottom: None,
                }),
                elastic: Some(0.3),
                momentum: Some(true),
            },
            _ => DragConfig {
                axis: Some(DragAxis::Both),
                constraints: Some(DragConstraints {
                    left: Some(-75.0),
                    right: Some(75.0),
                    top: Some(-75.0),
                    bottom: Some(75.0),
                }),
                elastic: Some(0.1),
                momentum: Some(true),
            },
        }
    };

    view! {
        <div class="showcase-card">
            <h3>"🎯 Drag Constraints"</h3>
            <p>"Axis and boundary constraints with elastic behavior"</p>

            <div class="demo-area" style="position: relative; height: 150px; border: 2px solid #ddd;">
                <MotionDiv
                    class="w-16 h-16 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 cursor-grab flex items-center justify-center text-white font-bold text-sm rounded-xl shadow-xl bg-gradient-to-br".to_string()
                    drag=get_drag_config(constraint_mode.get())
                    transition=Transition {
                        duration: Some(0.2),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    "🎯"
                </MotionDiv>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_constraint_mode.update(|m| *m = (*m + 1) % 3)
                >
                    {move || match constraint_mode.get() {
                        0 => "Free Drag",
                        1 => "X Only",
                        _ => "Constrained"
                    }}
                </button>
            </div>
        </div>
    }
}

/// Performance showcase
#[component]
fn PerformanceDemo() -> impl IntoView {
    let (animation_count, set_animation_count) = signal(10);
    let (is_running, set_is_running) = signal(false);

    let create_performance_animation =
        |index: usize, running: bool| -> HashMap<String, AnimationValue> {
            let mut target = HashMap::new();
            if running {
                let angle = (index as f64 * 36.0) + (js_sys::Math::random() * 360.0);
                let radius = 30.0 + (js_sys::Math::random() * 20.0);
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                target.insert("x".to_string(), AnimationValue::Pixels(x));
                target.insert("y".to_string(), AnimationValue::Pixels(y));
                target.insert("rotate".to_string(), AnimationValue::Number(angle));
            } else {
                target.insert("x".to_string(), AnimationValue::Pixels(0.0));
                target.insert("y".to_string(), AnimationValue::Pixels(0.0));
                target.insert("rotate".to_string(), AnimationValue::Number(0.0));
            }
            target
        };

    view! {
        <div class="showcase-card">
            <h3>"🚀 Performance Demo"</h3>
            <p>"Multiple concurrent animations at 60fps"</p>

            <div class="demo-area" style="position: relative; height: 120px;">
                {(0..animation_count.get()).map(|i| {
                    view! {
                        <MotionDiv
                            class=format!("w-4 h-4 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-full shadow-lg perf-{}", (i % 5) + 1)
                            initial=create_performance_animation(i, false)
                            animate=create_performance_animation(i, is_running.get())
                            transition=Transition {
                                duration: Some(2.0 + js_sys::Math::random()),
                                ease: Easing::EaseInOut,
                                repeat: RepeatConfig::Infinite,
                                ..Default::default()
                            }
                        >
                            ""
                        </MotionDiv>
                    }
                }).collect_view()}
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_is_running.update(|r| *r = !*r)
                >
                    {move || if is_running.get() { "Stop" } else { "Start" }}
                </button>
                <button
                    class="btn secondary"
                    on:click=move |_| set_animation_count.update(|c| *c = (*c % 50) + 10)
                >
                    "Count: " {animation_count.get()}
                </button>
            </div>
        </div>
    }
}

/// Advanced features showcase
#[component]
fn AdvancedFeaturesDemo() -> impl IntoView {
    let (feature_mode, set_feature_mode) = signal(0);

    let create_advanced_animation = |mode: i32| -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();

        match mode {
            0 => {
                // Spring physics
                target.insert("scale".to_string(), AnimationValue::Number(1.2));
                target.insert("rotate".to_string(), AnimationValue::Number(180.0));
            }
            1 => {
                // Color transitions
                target.insert(
                    "background".to_string(),
                    AnimationValue::Color(
                        "linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%)".to_string(),
                    ),
                );
                target.insert("border-radius".to_string(), AnimationValue::Pixels(25.0));
            }
            _ => {
                // Complex transform
                target.insert("x".to_string(), AnimationValue::Pixels(50.0));
                target.insert("y".to_string(), AnimationValue::Pixels(-30.0));
                target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
                target.insert("rotateY".to_string(), AnimationValue::Number(45.0));
            }
        }

        target
    };

    let get_transition = |mode: i32| -> Transition {
        match mode {
            0 => Transition {
                duration: Some(1.0),
                ease: Easing::EaseInOut,
                repeat: RepeatConfig::Infinite,
                ..Default::default()
            },
            1 => Transition {
                duration: Some(0.8),
                ease: Easing::EaseOut,
                ..Default::default()
            },
            _ => Transition {
                duration: Some(1.5),
                ease: Easing::EaseInOut,
                ..Default::default()
            },
        }
    };

    view! {
        <div class="showcase-card">
            <h3>"🔧 Advanced Features"</h3>
            <p>"Spring physics, 3D transforms, and complex easing"</p>

            <div class="demo-area">
                <MotionDiv
                    class="w-20 h-20 mx-auto flex items-center justify-center text-white font-bold text-lg rounded-xl shadow-lg bg-gradient-to-br".to_string()
                    initial=create_advanced_animation(feature_mode.get())
                    animate=create_advanced_animation(feature_mode.get())
                    transition=get_transition(feature_mode.get())
                >
                    "🔧"
                </MotionDiv>
            </div>

            <div class="controls">
                <button
                    class="btn"
                    on:click=move |_| set_feature_mode.update(|m| *m = (*m + 1) % 3)
                >
                    {move || match feature_mode.get() {
                        0 => "Spring Physics",
                        1 => "Color Transitions",
                        _ => "3D Transforms"
                    }}
                </button>
            </div>
        </div>
    }
}

/// Main entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing log");

    console::log_1(&"🚀 Leptos Motion Capability Showcase starting...".into());

    mount_to_body(|| {
        view! {
            <CapabilityShowcase/>
        }
    });
}
