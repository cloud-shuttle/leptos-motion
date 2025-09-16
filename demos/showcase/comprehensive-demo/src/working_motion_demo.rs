//! Working Motion Demo - Using the actual leptos-motion API correctly
//!
//! This demo showcases the real leptos-motion features:
//! - Simple API with MotionDiv components
//! - Independent transforms (x, y, rotateZ, etc.)
//! - Spring physics
//! - Gestures (hover, tap)
//! - Layout animations
//! - Timeline sequences

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
pub fn WorkingMotionDemo() -> impl IntoView {
    view! {
        <div style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 3rem;">
                <h1 style="font-size: 3rem; margin-bottom: 1rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                    "🚀 Leptos Motion"
                </h1>
                <p style="font-size: 1.2rem; color: #666; margin-bottom: 0.5rem;">
                    "Create high-performance web animations with Motion's easy-to-use API"
                </p>
                <p style="color: #888;">
                    "From simple transforms to advanced interactive gestures"
                </p>
            </header>

            <main>
                // Simple API Demo
                <SimpleApiDemo/>

                // Independent Transforms Demo
                <IndependentTransformsDemo/>

                // Spring Physics Demo
                <SpringPhysicsDemo/>

                // Gestures Demo
                <GesturesDemo/>

                // Layout Animation Demo
                <LayoutAnimationDemo/>

                // Timeline Sequences Demo
                <TimelineSequencesDemo/>
            </main>
        </div>
    }
}

#[component]
fn SimpleApiDemo() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Simple API"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Motion's pick-up-and-play API is easy to start and fun to master."
            </p>

            <div style="display: flex; gap: 2rem; align-items: center; flex-wrap: wrap;">
                <button
                    on:click=move |_| set_is_animated.set(!is_animated.get())
                    style="
                        padding: 1rem 2rem;
                        background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                        color: white;
                        border: none;
                        border-radius: 8px;
                        font-size: 1rem;
                        font-weight: 600;
                        cursor: pointer;
                        transition: transform 0.2s ease;
                    "
                >
                    {move || if is_animated.get() { "Reset" } else { "Animate" }}
                </button>

                <MotionDiv
                    initial=create_simple_animation_target(false)
                    animate=create_simple_animation_target(is_animated.get())
                    _transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::EaseInOut,
                        repeat: RepeatConfig::Never,
                        delay: Some(0.0),
                        stagger: None,
                    }
                    style="
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
                    ".to_string()
                >
                    "Motion"
                </MotionDiv>
            </div>
        </section>
    }
}

#[component]
fn IndependentTransformsDemo() -> impl IntoView {
    let (x_pos, set_x_pos) = signal(0.0);
    let (y_pos, set_y_pos) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    let (scale, set_scale) = signal(1.0);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Independent Transforms"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Animate x, y, rotateZ etc independently, without wrapper elements."
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
                <div>
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"X Position"</label>
                    <input
                        type="range"
                        min="-200"
                        max="200"
                        value=move || x_pos.get().to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_x_pos.set(value);
                        }
                        style="width: 100%;"
                    />
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Y Position"</label>
                    <input
                        type="range"
                        min="-200"
                        max="200"
                        value=move || y_pos.get().to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_y_pos.set(value);
                        }
                        style="width: 100%;"
                    />
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Rotation"</label>
                    <input
                        type="range"
                        min="0"
                        max="360"
                        value=move || rotation.get().to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_rotation.set(value);
                        }
                        style="width: 100%;"
                    />
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Scale"</label>
                    <input
                        type="range"
                        min="0.5"
                        max="2.0"
                        step="0.1"
                        value=move || scale.get().to_string()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                            set_scale.set(value);
                        }
                        style="width: 100%;"
                    />
                </div>
            </div>

            <div style="position: relative; height: 300px; background: #f8f9fa; border-radius: 8px; overflow: hidden;">
                <MotionDiv
                    initial=create_transform_animation_target(0.0, 0.0, 0.0, 1.0)
                    animate=create_transform_animation_target(x_pos.get(), y_pos.get(), rotation.get(), scale.get())
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        repeat: RepeatConfig::Never,
                        delay: Some(0.0),
                        stagger: None,
                    }
                    style="
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        width: 80px;
                        height: 80px;
                        background: linear-gradient(45deg, #ff9a9e, #fecfef);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: #333;
                        font-weight: bold;
                        box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                    ".to_string()
                >
                    "Transform"
                </MotionDiv>
            </div>
        </section>
    }
}

#[component]
fn SpringPhysicsDemo() -> impl IntoView {
    let (is_springing, set_is_springing) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Spring Physics"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Real spring physics for great-feeling animations."
            </p>

            <div style="display: flex; gap: 2rem; align-items: center; flex-wrap: wrap;">
                <button
                    on:click=move |_| set_is_springing.set(!is_springing.get())
                    style="
                        padding: 1rem 2rem;
                        background: linear-gradient(45deg, #a8edea, #fed6e3);
                        color: #333;
                        border: none;
                        border-radius: 8px;
                        font-size: 1rem;
                        font-weight: 600;
                        cursor: pointer;
                        transition: transform 0.2s ease;
                    "
                >
                    {move || if is_springing.get() { "Reset Spring" } else { "Spring!" }}
                </button>

                <MotionDiv
                    initial=create_spring_animation_target(false)
                    animate=create_spring_animation_target(is_springing.get())
                    _transition=Transition {
                        duration: Some(1.2),
                        ease: Easing::Spring(SpringConfig {
                            stiffness: 100.0,
                            damping: 10.0,
                            mass: 1.0,
                            velocity: 0.0,
                            rest_delta: 0.01,
                            rest_speed: 0.01,
                        }),
                        repeat: RepeatConfig::Never,
                        delay: Some(0.0),
                        stagger: None,
                    }
                    style="
                        width: 100px;
                        height: 100px;
                        background: linear-gradient(45deg, #ffecd2, #fcb69f);
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: #333;
                        font-weight: bold;
                        box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                    ".to_string()
                >
                    "Spring"
                </MotionDiv>
            </div>
        </section>
    }
}

#[component]
fn GesturesDemo() -> impl IntoView {
    let (hover_count, set_hover_count) = signal(0);
    let (press_count, set_press_count) = signal(0);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Gestures"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Hover, press and drag gestures that feel native, not \"webby\"."
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem;">
                <div style="text-align: center;">
                    <h3 style="margin-bottom: 1rem; color: #333;">"Hover Gesture"</h3>
                    <MotionDiv
                        while_hover=create_hover_animation()
                        on:mouseenter=move |_| set_hover_count.set(hover_count.get() + 1)
                        style="
                            width: 120px;
                            height: 120px;
                            background: linear-gradient(45deg, #667eea, #764ba2);
                            border-radius: 12px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: white;
                            font-weight: bold;
                            margin: 0 auto;
                            cursor: pointer;
                            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                        ".to_string()
                    >
                        "Hover Me"
                    </MotionDiv>
                    <p style="margin-top: 1rem; color: #666;">
                        "Hovered: " {move || hover_count.get()}
                    </p>
                </div>

                <div style="text-align: center;">
                    <h3 style="margin-bottom: 1rem; color: #333;">"Tap Gesture"</h3>
                    <MotionDiv
                        while_tap=create_tap_animation()
                        on:click=move |_| set_press_count.set(press_count.get() + 1)
                        style="
                            width: 120px;
                            height: 120px;
                            background: linear-gradient(45deg, #ff9a9e, #fecfef);
                            border-radius: 12px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: #333;
                            font-weight: bold;
                            margin: 0 auto;
                            cursor: pointer;
                            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                        ".to_string()
                    >
                        "Tap Me"
                    </MotionDiv>
                    <p style="margin-top: 1rem; color: #666;">
                        "Pressed: " {move || press_count.get()}
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn LayoutAnimationDemo() -> impl IntoView {
    let (layout_mode, set_layout_mode) = signal("grid");

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Layout Animation"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Animate between different layouts with Motion's industry-leading layout animation engine."
            </p>

            <div style="margin-bottom: 2rem;">
                <button
                    on:click=move |_| set_layout_mode.set(if layout_mode.get() == "grid" { "flex" } else { "grid" })
                    style="
                        padding: 1rem 2rem;
                        background: linear-gradient(45deg, #a8edea, #fed6e3);
                        color: #333;
                        border: none;
                        border-radius: 8px;
                        font-size: 1rem;
                        font-weight: 600;
                        cursor: pointer;
                        margin-right: 1rem;
                    "
                >
                    {move || if layout_mode.get() == "grid" { "Switch to Flex" } else { "Switch to Grid" }}
                </button>
                <span style="color: #666;">
                    "Current: " {move || layout_mode.get().to_uppercase()}
                </span>
            </div>

            <div
                style=move || {
                    let base_style = "
                        display: flex;
                        gap: 1rem;
                        padding: 2rem;
                        background: #f8f9fa;
                        border-radius: 8px;
                        min-height: 200px;
                        align-items: center;
                        justify-content: center;
                    ";
                    if layout_mode.get() == "grid" {
                        format!("{} grid-template-columns: repeat(3, 1fr);", base_style)
                    } else {
                        base_style.to_string()
                    }
                }
            >
                {move || (0..6).map(|i| {
                    view! {
                        <MotionDiv
                            _layout=true
                            _transition=Transition {
                                duration: Some(0.5),
                                ease: Easing::EaseInOut,
                                repeat: RepeatConfig::Never,
                                delay: Some(0.0),
                                stagger: None,
                            }
                            style="
                                width: 80px;
                                height: 80px;
                                background: linear-gradient(45deg, #ffecd2, #fcb69f);
                                border-radius: 8px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: #333;
                                font-weight: bold;
                                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                            ".to_string()
                        >
                            {i + 1}
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn TimelineSequencesDemo() -> impl IntoView {
    let (is_playing, set_is_playing) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; border: 1px solid #e0e0e0; border-radius: 12px;">
            <h2 style="font-size: 2rem; margin-bottom: 1.5rem; color: #333;">"Timeline Sequences"</h2>
            <p style="margin-bottom: 2rem; color: #666;">
                "Variants, stagger and timelines make it easy to precisely orchestrate animations."
            </p>

            <div style="margin-bottom: 2rem;">
                <button
                    on:click=move |_| set_is_playing.set(!is_playing.get())
                    style="
                        padding: 1rem 2rem;
                        background: linear-gradient(45deg, #667eea, #764ba2);
                        color: white;
                        border: none;
                        border-radius: 8px;
                        font-size: 1rem;
                        font-weight: 600;
                        cursor: pointer;
                    "
                >
                    {move || if is_playing.get() { "Reset Sequence" } else { "Play Sequence" }}
                </button>
            </div>

            <div style="display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;">
                {move || (0..5).map(|i| {
                    view! {
                        <MotionDiv
                   initial=create_sequence_animation_target(false, i)
                   animate=create_sequence_animation_target(is_playing.get(), i)
                    _transition=Transition {
                       duration: Some(0.6),
                       ease: Easing::EaseOut,
                       repeat: RepeatConfig::Never,
                       delay: Some((i as f64) * 0.1),
                       stagger: None,
                   }
                            style="
                                width: 60px;
                                height: 60px;
                                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                                border-radius: 50%;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: white;
                                font-weight: bold;
                                box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                            ".to_string()
                        >
                            {i + 1}
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// Helper functions to create animation targets
fn create_simple_animation_target(animated: bool) -> AnimationTarget {
    let mut target = HashMap::new();
    if animated {
        target.insert("scale".to_string(), AnimationValue::Number(1.2));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(360.0));
    } else {
        target.insert("scale".to_string(), AnimationValue::Number(1.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
    }
    target
}

fn create_transform_animation_target(x: f64, y: f64, rotation: f64, scale: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("x".to_string(), AnimationValue::Pixels(x));
    target.insert("y".to_string(), AnimationValue::Pixels(y));
    target.insert("rotateZ".to_string(), AnimationValue::Degrees(rotation));
    target.insert("scale".to_string(), AnimationValue::Number(scale));
    target
}

fn create_spring_animation_target(springing: bool) -> AnimationTarget {
    let mut target = HashMap::new();
    if springing {
        target.insert("scale".to_string(), AnimationValue::Number(1.5));
        target.insert("y".to_string(), AnimationValue::Pixels(-50.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(180.0));
    } else {
        target.insert("scale".to_string(), AnimationValue::Number(1.0));
        target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
    }
    target
}

fn create_hover_animation() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.1));
    target.insert("y".to_string(), AnimationValue::Pixels(-5.0));
    target
}

fn create_tap_animation() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(0.95));
    target
}

fn create_sequence_animation_target(playing: bool, _index: usize) -> AnimationTarget {
    let mut target = HashMap::new();
    if playing {
        target.insert("y".to_string(), AnimationValue::Pixels(-30.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.2));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(360.0));
    } else {
        target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
    }
    target
}
