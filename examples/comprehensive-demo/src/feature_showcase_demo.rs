//! Feature Showcase Demo - Comprehensive demonstration of all leptos-motion features
//!
//! This demo showcases:
//! - Simple API with easy-to-use components
//! - Independent transforms (x, y, rotateZ, etc.)
//! - Scroll animations
//! - Spring physics
//! - Exit animations (AnimatePresence)
//! - Gestures (hover, press, drag)
//! - Layout animations
//! - Timeline sequences and variants

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
pub fn FeatureShowcaseDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎭 FeatureShowcaseDemo: Component created".into());
    view! {
        <div style="padding: 2rem; max-width: 1400px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 4rem;">
                <h1 style="font-size: 3.5rem; margin-bottom: 1rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4, #feca57); -webkit-background-clip: text; -webkit-text-fill-color: transparent; font-weight: 800;">
                    "🚀 Leptos Motion"
                </h1>
                <p style="font-size: 1.4rem; color: #666; margin-bottom: 0.5rem; font-weight: 300;">
                    "Create high-performance web animations with Motion's easy-to-use API"
                </p>
                <p style="color: #888; font-size: 1.1rem;">
                    "From simple transforms to advanced interactive gestures"
                </p>
            </header>

            <main>
                // Simple API Demo
                <SimpleApiDemo/>

                // Independent Transforms Demo
                <IndependentTransformsDemo/>

                // Scroll Animation Demo
                <ScrollAnimationDemo/>

                // Spring Physics Demo
                <SpringPhysicsDemo/>

                // Exit Animation Demo
                <ExitAnimationDemo/>

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
    let (is_animated, set_animated) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Simple API"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Motion's pick-up-and-play API is easy to start and fun to master."
            </p>

            <div style="display: flex; justify-content: center; gap: 2rem; margin-bottom: 2rem;">
                <button
                    style="padding: 1rem 2rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.3s ease;"
                    on:click=move |_| set_animated.set(!is_animated.get())
                >
                    {move || if is_animated.get() { "Reset" } else { "Animate" }}
                </button>
            </div>

            <div style="display: flex; justify-content: center; align-items: center; min-height: 200px;">
                <MotionDiv
                    initial={
                        let mut target = HashMap::new();
                        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                        target.insert("scale".to_string(), AnimationValue::Number(0.8));
                        target
                    }
                    animate={
                        let mut target = HashMap::new();
                        if is_animated.get() {
                            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                            target.insert("scale".to_string(), AnimationValue::Number(1.2));
                            target.insert("rotateZ".to_string(), AnimationValue::Degrees(360.0));
                        } else {
                            target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                            target.insert("scale".to_string(), AnimationValue::Number(0.8));
                            target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
                        }
                        target
                    }
                    _transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::EaseInOut,
                        repeat: RepeatConfig::Never,
                        delay: Some(0.0),
                        stagger: None,
                    }
                    style="
                        width: 120px;
                        height: 120px;
                        background: linear-gradient(45deg, #667eea, #764ba2);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: 600;
                        font-size: 1.1rem;
                        box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
                    ".to_string()
                >
                    "Simple API"
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
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Independent Transforms"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Animate x, y, rotateZ etc independently, without wrapper elements."
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
                <div style="text-align: center;">
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"X Position"</label>
                    <input
                        type="range"
                        min="-100"
                        max="100"
                        value=move || x_pos.get() as i32
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_x_pos.set(value);
                        }
                        style="width: 100%;"
                    />
                    <span style="font-size: 0.9rem; color: #666;">{move || format!("{:.0}px", x_pos.get())}</span>
                </div>

                <div style="text-align: center;">
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Y Position"</label>
                    <input
                        type="range"
                        min="-100"
                        max="100"
                        value=move || y_pos.get() as i32
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_y_pos.set(value);
                        }
                        style="width: 100%;"
                    />
                    <span style="font-size: 0.9rem; color: #666;">{move || format!("{:.0}px", y_pos.get())}</span>
                </div>

                <div style="text-align: center;">
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Rotation"</label>
                    <input
                        type="range"
                        min="0"
                        max="360"
                        value=move || rotation.get() as i32
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            set_rotation.set(value);
                        }
                        style="width: 100%;"
                    />
                    <span style="font-size: 0.9rem; color: #666;">{move || format!("{:.0}°", rotation.get())}</span>
                </div>

                <div style="text-align: center;">
                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 600;">"Scale"</label>
                    <input
                        type="range"
                        min="50"
                        max="200"
                        value=move || (scale.get() * 100.0) as i32
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(100.0) / 100.0;
                            set_scale.set(value);
                        }
                        style="width: 100%;"
                    />
                    <span style="font-size: 0.9rem; color: #666;">{move || format!("{:.1}x", scale.get())}</span>
                </div>
            </div>

            <div style="display: flex; justify-content: center; align-items: center; min-height: 300px; position: relative;">
                <MotionDiv
                    animate={
                        let mut target = HashMap::new();
                        target.insert("x".to_string(), AnimationValue::Pixels(x_pos.get()));
                        target.insert("y".to_string(), AnimationValue::Pixels(y_pos.get()));
                        target.insert("rotateZ".to_string(), AnimationValue::Degrees(rotation.get()));
                        target.insert("scale".to_string(), AnimationValue::Number(scale.get()));
                        target
                    }
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        repeat: RepeatConfig::Never,
                        delay: Some(0.0),
                        stagger: None,
                    }
                    style="
                        width: 100px;
                        height: 100px;
                        background: linear-gradient(45deg, #ff9a9e, #fecfef);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: #333;
                        font-weight: 600;
                        font-size: 0.9rem;
                        box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
                        position: absolute;
                    ".to_string()
                >
                    "Transform"
                </MotionDiv>
            </div>
        </section>
    }
}

#[component]
fn ScrollAnimationDemo() -> impl IntoView {
    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Scroll Animation"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Smooth, hardware-accelerated scroll animations."
            </p>

            <div style="height: 400px; overflow-y: auto; background: white; border-radius: 8px; padding: 2rem;">
                <div style="height: 800px; display: flex; flex-direction: column; gap: 2rem;">
                    {move || (0..10).map(|i| {
                        view! {
                            <MotionDiv
                                initial={
                                    let mut target = HashMap::new();
                                    target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                    target.insert("y".to_string(), AnimationValue::Pixels(50.0));
                                    target
                                }
                                animate={
                                    let mut target = HashMap::new();
                                    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                    target.insert("y".to_string(), AnimationValue::Pixels(0.0));
                                    target
                                }
                                _transition=Transition {
                                    duration: Some(0.6),
                                    ease: Easing::EaseOut,
                                    repeat: RepeatConfig::Never,
                                    delay: Some(i as f64 * 0.1),
                                    stagger: None,
                                }
                                style="
                                    padding: 2rem;
                                    background: linear-gradient(45deg, #a8edea, #fed6e3);
                                    border-radius: 12px;
                                    color: #333;
                                    font-weight: 600;
                                    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
                                ".to_string()
                            >
                                {format!("Scroll Item {}", i + 1)}
                            </MotionDiv>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn SpringPhysicsDemo() -> impl IntoView {
    let (is_springing, set_springing) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Spring Physics"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Real spring physics for great-feeling animations."
            </p>

            <div style="display: flex; justify-content: center; gap: 2rem; margin-bottom: 2rem;">
                <button
                    style="padding: 1rem 2rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.3s ease;"
                    on:click=move |_| set_springing.set(!is_springing.get())
                >
                    {move || if is_springing.get() { "Reset" } else { "Spring!" }}
                </button>
            </div>

            <div style="display: flex; justify-content: center; align-items: center; min-height: 200px;">
                <MotionDiv
                    initial={
                        let mut target = HashMap::new();
                        target.insert("scale".to_string(), AnimationValue::Number(1.0));
                        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
                        target
                    }
                    animate={
                        let mut target = HashMap::new();
                        if is_springing.get() {
                            target.insert("scale".to_string(), AnimationValue::Number(1.5));
                            target.insert("rotateZ".to_string(), AnimationValue::Degrees(180.0));
                        } else {
                            target.insert("scale".to_string(), AnimationValue::Number(1.0));
                            target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
                        }
                        target
                    }
                    _transition=Transition {
                        duration: Some(1.0),
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
                        width: 120px;
                        height: 120px;
                        background: linear-gradient(45deg, #667eea, #764ba2);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: 600;
                        font-size: 1.1rem;
                        box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
                    ".to_string()
                >
                    "Spring"
                </MotionDiv>
            </div>
        </section>
    }
}

#[component]
fn ExitAnimationDemo() -> impl IntoView {
    let (items, set_items) = signal(vec![1, 2, 3, 4, 5]);

    let add_item = move |_| {
        let current_items = items.get();
        let new_id = current_items.iter().max().unwrap_or(&0) + 1;
        set_items.set([current_items, vec![new_id]].concat());
    };

    let remove_item = move |id: i32| {
        let current_items = items.get();
        set_items.set(current_items.into_iter().filter(|&x| x != id).collect());
    };

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Exit Animation"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "AnimatePresence makes it easy to animate elements as they exit."
            </p>

            <div style="display: flex; justify-content: center; gap: 2rem; margin-bottom: 2rem;">
                <button
                    style="padding: 1rem 2rem; background: linear-gradient(45deg, #4ecdc4, #44a08d); color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.3s ease;"
                    on:click=add_item
                >
                    "Add Item"
                </button>
            </div>

            <div style="display: flex; flex-wrap: wrap; gap: 1rem; justify-content: center; min-height: 200px;">
                <AnimatePresence>
                    {move || items.get().into_iter().map(|item| {
                        view! {
                            <MotionDiv
                                initial={
                                    let mut target = HashMap::new();
                                    target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                    target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                    target
                                }
                                animate={
                                    let mut target = HashMap::new();
                                    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                    target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                    target
                                }
                                _transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseInOut,
                                    repeat: RepeatConfig::Never,
                                    delay: Some(0.0),
                                    stagger: None,
                                }
                                style="
                                    width: 80px;
                                    height: 80px;
                                    background: linear-gradient(45deg, #ff9a9e, #fecfef);
                                    border-radius: 8px;
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    color: #333;
                                    font-weight: 600;
                                    cursor: pointer;
                                    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
                                ".to_string()
                                on:click=move |_| remove_item(item)
                            >
                                {format!("{}", item)}
                            </MotionDiv>
                        }
                    }).collect::<Vec<_>>()}
                </AnimatePresence>
            </div>
        </section>
    }
}

#[component]
fn GesturesDemo() -> impl IntoView {
    let (hover_count, set_hover_count) = signal(0);
    let (tap_count, set_tap_count) = signal(0);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Gestures"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Hover, press and drag gestures that feel native, not 'webby'."
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; margin-bottom: 2rem;">
                <div style="text-align: center;">
                    <h3 style="margin-bottom: 1rem; color: #333;">"Hover Gesture"</h3>
                    <MotionDiv
                        while_hover={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(1.1));
                            target.insert("rotateZ".to_string(), AnimationValue::Degrees(5.0));
                            target
                        }
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
                            font-weight: 600;
                            cursor: pointer;
                            box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
                            margin: 0 auto;
                        ".to_string()
                    >
                        "Hover Me"
                    </MotionDiv>
                    <p style="margin-top: 1rem; color: #666;">{move || format!("Hovered {} times", hover_count.get())}</p>
                </div>

                <div style="text-align: center;">
                    <h3 style="margin-bottom: 1rem; color: #333;">"Tap Gesture"</h3>
                    <MotionDiv
                        while_tap={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(0.95));
                            target
                        }
                        on:click=move |_| set_tap_count.set(tap_count.get() + 1)
                        style="
                            width: 120px;
                            height: 120px;
                            background: linear-gradient(45deg, #ff9a9e, #fecfef);
                            border-radius: 12px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: #333;
                            font-weight: 600;
                            cursor: pointer;
                            box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
                            margin: 0 auto;
                        ".to_string()
                    >
                        "Tap Me"
                    </MotionDiv>
                    <p style="margin-top: 1rem; color: #666;">{move || format!("Tapped {} times", tap_count.get())}</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn LayoutAnimationDemo() -> impl IntoView {
    let (layout_mode, set_layout_mode) = signal("grid".to_string());

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Layout Animation"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Animate between different layouts with Motion's industry-leading layout animation engine."
            </p>

            <div style="display: flex; justify-content: center; gap: 2rem; margin-bottom: 2rem;">
                <button
                    style="padding: 1rem 2rem; background: linear-gradient(45deg, #667eea, #764ba2); color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.3s ease;"
                    on:click=move |_| set_layout_mode.set(if layout_mode.get() == "grid" { "flex".to_string() } else { "grid".to_string() })
                >
                    {move || if layout_mode.get() == "grid" { "Switch to Flex" } else { "Switch to Grid" }}
                </button>
            </div>

            <div style="
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
                gap: 1.5rem;
                width: 100%;
                transition: all 0.5s ease;
            " class:flex=move || layout_mode.get() == "flex" class:grid=move || layout_mode.get() == "grid">
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
                                background: linear-gradient(45deg, #a8edea, #fed6e3);
                                padding: 2rem;
                                border-radius: 12px;
                                text-align: center;
                                font-weight: 700;
                                color: #333;
                                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
                                font-size: 1.2rem;
                            ".to_string()
                        >
                            {format!("Item {}", i + 1)}
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn TimelineSequencesDemo() -> impl IntoView {
    let (is_playing, set_playing) = signal(false);

    view! {
        <section style="margin-bottom: 4rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
            <h2 style="font-size: 2.5rem; margin-bottom: 2rem; color: #333; text-align: center;">
                "Timeline Sequences"
            </h2>
            <p style="text-align: center; margin-bottom: 2rem; color: #666; font-size: 1.1rem;">
                "Variants, stagger and timelines make it easy to precisely orchestrate animations."
            </p>

            <div style="display: flex; justify-content: center; gap: 2rem; margin-bottom: 2rem;">
                <button
                    style="padding: 1rem 2rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; transition: all 0.3s ease;"
                    on:click=move |_| set_playing.set(!is_playing.get())
                >
                    {move || if is_playing.get() { "Reset" } else { "Play Sequence" }}
                </button>
            </div>

            <div style="display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap;">
                {move || (0..5).map(|i| {
                    view! {
                        <MotionDiv
                            initial={
                                let mut target = HashMap::new();
                                target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                target.insert("y".to_string(), AnimationValue::Pixels(50.0));
                                target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                target
                            }
                            animate={
                                let mut target = HashMap::new();
                                if is_playing.get() {
                                    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                    target.insert("y".to_string(), AnimationValue::Pixels(0.0));
                                    target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                } else {
                                    target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                    target.insert("y".to_string(), AnimationValue::Pixels(50.0));
                                    target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                }
                                target
                            }
                            _transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseOut,
                                repeat: RepeatConfig::Never,
                                delay: Some(i as f64 * 0.2),
                                stagger: None,
                            }
                            style="
                                width: 80px;
                                height: 80px;
                                background: linear-gradient(45deg, #667eea, #764ba2);
                                border-radius: 12px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: white;
                                font-weight: 600;
                                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
                            ".to_string()
                        >
                            {format!("{}", i + 1)}
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
