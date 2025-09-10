use leptos::prelude::*;
use leptos_motion_core::types::AnimationValue;
use leptos_motion_dom::signal_based_animation_controller::*;
use std::collections::HashMap;
use std::rc::Rc;

#[component]
pub fn MotionShowcaseDemo() -> impl IntoView {
    // ✅ Signal-based state management for all demo features
    let (show_exit_demo, set_show_exit_demo) = signal(true);
    let (layout_mode, set_layout_mode) = signal("grid".to_string());
    let (scroll_progress, set_scroll_progress) = signal(0.0);
    let (gesture_state, set_gesture_state) = signal("idle".to_string());

    // ✅ Create animation controllers for different features
    let independent_transform_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    }));

    let spring_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    }));

    let scroll_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    }));

    let gesture_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map
    }));

    let layout_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    }));

    // ✅ Independent Transform Animations - Using CSS classes for better control
    let move_right = move |_| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("independent-transform-box"))
        {
            element.set_attribute("style", "width: 80px; height: 80px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateX(100px) translateY(0px) rotateZ(0deg) scale(1); transition: transform 0.3s ease;").unwrap();
        }
    };

    let move_down = move |_| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("independent-transform-box"))
        {
            element.set_attribute("style", "width: 80px; height: 80px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateX(0px) translateY(50px) rotateZ(0deg) scale(1); transition: transform 0.3s ease;").unwrap();
        }
    };

    let rotate = move |_| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("independent-transform-box"))
        {
            element.set_attribute("style", "width: 80px; height: 80px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateX(0px) translateY(0px) rotateZ(180deg) scale(1); transition: transform 0.3s ease;").unwrap();
        }
    };

    let scale_up = move |_| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("independent-transform-box"))
        {
            element.set_attribute("style", "width: 80px; height: 80px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateX(0px) translateY(0px) rotateZ(0deg) scale(1.5); transition: transform 0.3s ease;").unwrap();
        }
    };

    // ✅ Spring Physics Animations
    let spring_bounce = move |_| {
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("spring-ball"))
        {
            // Reset to initial state first
            element.set_attribute("style", "width: 100px; height: 100px; background: linear-gradient(45deg, #ff9ff3, #54a0ff); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateY(0px) scale(1); transition: transform 0.1s ease;").unwrap();

            // Then immediately bounce up with spring physics
            element.set_attribute("style", "width: 100px; height: 100px; background: linear-gradient(45deg, #ff9ff3, #54a0ff); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateY(-50px) scale(1.2); transition: transform 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);").unwrap();
        }
    };

    // ✅ Gesture Animations
    let handle_hover = move |_| {
        set_gesture_state.set("hover".to_string());
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("gesture-card"))
        {
            element.set_attribute("style", "width: 150px; height: 150px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; cursor: pointer; user-select: none; transform: scale(1.1) rotateZ(5deg); transition: transform 0.2s ease;").unwrap();
        }
    };

    let handle_hover_end = move |_| {
        set_gesture_state.set("idle".to_string());
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("gesture-card"))
        {
            element.set_attribute("style", "width: 150px; height: 150px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; cursor: pointer; user-select: none; transform: scale(1) rotateZ(0deg); transition: transform 0.2s ease;").unwrap();
        }
    };

    let handle_press = move |_| {
        set_gesture_state.set("pressed".to_string());
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("gesture-card"))
        {
            element.set_attribute("style", "width: 150px; height: 150px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; cursor: pointer; user-select: none; transform: scale(0.95) rotateZ(5deg); transition: transform 0.1s ease;").unwrap();
        }
    };

    let handle_press_end = move |_| {
        set_gesture_state.set("hover".to_string());
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("gesture-card"))
        {
            element.set_attribute("style", "width: 150px; height: 150px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; cursor: pointer; user-select: none; transform: scale(1.1) rotateZ(5deg); transition: transform 0.2s ease;").unwrap();
        }
    };

    // ✅ Layout Animation
    let toggle_layout = move |_| {
        let new_mode = if layout_mode.get() == "grid" {
            "list".to_string()
        } else {
            "grid".to_string()
        };
        set_layout_mode.set(new_mode.clone());

        // Animate layout transition
        let mut target = HashMap::new();
        if new_mode == "list" {
            target.insert("x".to_string(), AnimationValue::Pixels(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        } else {
            target.insert("x".to_string(), AnimationValue::Pixels(20.0));
            target.insert("y".to_string(), AnimationValue::Pixels(20.0));
        }
        layout_controller.animate_to(target);
    };

    // ✅ Exit Animation
    let toggle_exit_demo = move |_| {
        set_show_exit_demo.update(|show| *show = !*show);
    };

    view! {
        <div class="motion-showcase" style="padding: 2rem; max-width: 1400px; margin: 0 auto; font-family: system-ui, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh;">
            <header style="text-align: center; margin-bottom: 4rem; color: white;">
                <h1 style="font-size: 4rem; margin-bottom: 1rem; font-weight: 800; text-shadow: 0 4px 8px rgba(0,0,0,0.3);">
                    "Animations that move"
                </h1>
                <p style="font-size: 1.5rem; margin-bottom: 2rem; opacity: 0.9; max-width: 800px; margin-left: auto; margin-right: auto;">
                    "Create high-performance web animations with Motion's easy-to-use API — from simple transforms to advanced interactive gestures."
                </p>
            </header>

            <main style="display: grid; gap: 4rem;">
                // ✅ Independent Transforms Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 3rem; align-items: center;">
                        <div>
                            <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                                "Independent transforms"
                            </h2>
                            <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6;">
                                "Animate x, y, rotateZ etc independently, without wrapper elements."
                            </p>
                            <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                                <button on:click=move_right style="padding: 1rem 2rem; background: #4ecdc4; color: white; border: none; border-radius: 12px; cursor: pointer; font-weight: 600; font-size: 1rem;">
                                    "Move Right"
                                </button>
                                <button on:click=move_down style="padding: 1rem 2rem; background: #ff6b6b; color: white; border: none; border-radius: 12px; cursor: pointer; font-weight: 600; font-size: 1rem;">
                                    "Move Down"
                                </button>
                                <button on:click=rotate style="padding: 1rem 2rem; background: #45b7d1; color: white; border: none; border-radius: 12px; cursor: pointer; font-weight: 600; font-size: 1rem;">
                                    "Rotate"
                                </button>
                                <button on:click=scale_up style="padding: 1rem 2rem; background: #96ceb4; color: white; border: none; border-radius: 12px; cursor: pointer; font-weight: 600; font-size: 1rem;">
                                    "Scale Up"
                                </button>
                            </div>
                        </div>
                        <div style="display: flex; justify-content: center; align-items: center; height: 300px; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px;">
                            <div
                                style="width: 80px; height: 80px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateX(0px) translateY(0px) rotateZ(0deg) scale(1); transition: transform 0.3s ease;"
                                id="independent-transform-box"
                            >
                                "Box"
                            </div>
                        </div>
                    </div>
                </section>

                // ✅ Spring Physics Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 3rem; align-items: center;">
                        <div>
                            <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                                "Spring physics"
                            </h2>
                            <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6;">
                                "Real spring physics for great-feeling animations."
                            </p>
                            <button on:click=spring_bounce style="padding: 1.5rem 3rem; background: linear-gradient(45deg, #ff9ff3, #54a0ff); color: white; border: none; border-radius: 16px; cursor: pointer; font-weight: 700; font-size: 1.2rem; box-shadow: 0 8px 16px rgba(0,0,0,0.2);">
                                "Bounce!"
                            </button>
                        </div>
                        <div style="display: flex; justify-content: center; align-items: center; height: 300px; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px;">
                            <div
                                style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff9ff3, #54a0ff); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: translateY(0px) scale(1); transition: transform 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);"
                                id="spring-ball"
                            >
                                "Ball"
                            </div>
                        </div>
                    </div>
                </section>

                // ✅ Gestures Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 3rem; align-items: center;">
                        <div>
                            <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                                "Gestures"
                            </h2>
                            <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6;">
                                "Hover, press and drag gestures that feel native, not \"webby\"."
                            </p>
                            <div style="padding: 1rem; background: #f8f9fa; border-radius: 12px; margin-bottom: 1rem;">
                                <p style="margin: 0; font-weight: 600; color: #333;">
                                    "Gesture State: " {gesture_state}
                                </p>
                            </div>
                            <p style="font-size: 1rem; color: #888; font-style: italic;">
                                "Hover and press the card to see gesture animations"
                            </p>
                        </div>
                        <div style="display: flex; justify-content: center; align-items: center; height: 300px; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px;">
                            <div
                                style="width: 150px; height: 150px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; cursor: pointer; user-select: none; transform: scale(1) rotateZ(0deg); transition: transform 0.2s ease;"
                                id="gesture-card"
                                on:mouseenter=handle_hover
                                on:mouseleave=handle_hover_end
                                on:mousedown=handle_press
                                on:mouseup=handle_press_end
                            >
                                "Gesture Card"
                            </div>
                        </div>
                    </div>
                </section>

                // ✅ Layout Animation Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div>
                        <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700; text-align: center;">
                            "Layout animation"
                        </h2>
                        <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6; text-align: center; max-width: 600px; margin-left: auto; margin-right: auto;">
                            "Animate between different layouts with Motion's industry-leading layout animation engine."
                        </p>
                        <div style="text-align: center; margin-bottom: 3rem;">
                            <button on:click=toggle_layout style="padding: 1.5rem 3rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 16px; cursor: pointer; font-weight: 700; font-size: 1.2rem; box-shadow: 0 8px 16px rgba(0,0,0,0.2);">
                                {move || if layout_mode.get() == "grid" { "Switch to List" } else { "Switch to Grid" }}
                            </button>
                        </div>
                        <div
                            style=move || {
                                if layout_mode.get() == "grid" {
                                    "display: grid; grid-template-columns: repeat(4, 1fr); gap: 2rem; padding: 2rem; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px;"
                                } else {
                                    "display: flex; flex-direction: column; gap: 1rem; padding: 2rem; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px;"
                                }
                            }
                        >
                            {move || (0..8).map(|i| {
                                view! {
                                    <div
                                        style="padding: 2rem; background: linear-gradient(45deg, #96ceb4, #feca57); border-radius: 12px; color: white; font-weight: 600; text-align: center; font-size: 1.1rem;"
                                    >
                                        {format!("Item {}", i + 1)}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </section>

                // ✅ Exit Animation Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="text-align: center;">
                        <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                            "Exit animation"
                        </h2>
                        <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6; max-width: 600px; margin-left: auto; margin-right: auto;">
                            "AnimatePresence makes it easy to animate elements as they exit."
                        </p>
                        <button on:click=toggle_exit_demo style="padding: 1.5rem 3rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); color: white; border: none; border-radius: 16px; cursor: pointer; font-weight: 700; font-size: 1.2rem; box-shadow: 0 8px 16px rgba(0,0,0,0.2); margin-bottom: 3rem;">
                            {move || if show_exit_demo.get() { "Hide Element" } else { "Show Element" }}
                        </button>
                        <div style="display: flex; justify-content: center; align-items: center; min-height: 200px;">
                            <div
                                style=move || {
                                    if show_exit_demo.get() {
                                        "width: 200px; height: 200px; background: linear-gradient(45deg, #ff9ff3, #54a0ff); border-radius: 20px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.5rem; box-shadow: 0 12px 24px rgba(0,0,0,0.2);"
                                    } else {
                                        "width: 200px; height: 200px; background: transparent; display: flex; align-items: center; justify-content: center;"
                                    }
                                }
                            >
                                {move || if show_exit_demo.get() { "Animated Element" } else { "" }}
                            </div>
                        </div>
                    </div>
                </section>

                // ✅ Scroll Animation Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="text-align: center;">
                        <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                            "Scroll animation"
                        </h2>
                        <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6; max-width: 600px; margin-left: auto; margin-right: auto;">
                            "Smooth, hardware-accelerated scroll animations."
                        </p>
                        <div style="height: 400px; overflow-y: auto; background: linear-gradient(45deg, #f8f9fa, #e9ecef); border-radius: 16px; padding: 2rem;">
                            {move || (0..20).map(|i| {
                                view! {
                                    <div
                                        style="height: 100px; background: linear-gradient(45deg, #96ceb4, #feca57); border-radius: 12px; margin-bottom: 1rem; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem;"
                                    >
                                        {format!("Scroll Item {}", i + 1)}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </section>

                // ✅ Timeline Sequences Section
                <section class="feature-section" style="background: rgba(255,255,255,0.95); border-radius: 20px; padding: 3rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <div style="text-align: center;">
                        <h2 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333; font-weight: 700;">
                            "Timeline sequences"
                        </h2>
                        <p style="font-size: 1.2rem; color: #666; margin-bottom: 2rem; line-height: 1.6; max-width: 600px; margin-left: auto; margin-right: auto;">
                            "Variants, stagger and timelines make it easy to precisely orchestrate animations."
                        </p>
                        <div style="display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap;">
                            {move || (0..6).map(|i| {
                                view! {
                                    <div
                                        style="width: 80px; height: 80px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: scale(1); transition: transform 0.3s ease;"
                                        id=format!("timeline-item-{}", i)
                                    >
                                        {i + 1}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                        <div style="text-align: center; margin-top: 2rem;">
                            <button
                                on:click=move |_| {
                                    // Reset all items to initial state first
                                    for i in 0..6 {
                                        let element_id = format!("timeline-item-{}", i);
                                        if let Some(element) = web_sys::window()
                                            .and_then(|w| w.document())
                                            .and_then(|d| d.get_element_by_id(&element_id)) {
                                            element.set_attribute("style", "width: 80px; height: 80px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: scale(1); transition: transform 0.1s ease;").unwrap();
                                        }
                                    }

                                    // Then animate with stagger effect using CSS transition delays
                                    for i in 0..6 {
                                        let element_id = format!("timeline-item-{}", i);
                                        if let Some(element) = web_sys::window()
                                            .and_then(|w| w.document())
                                            .and_then(|d| d.get_element_by_id(&element_id)) {
                                            // Use CSS transition delay for stagger effect
                                            element.set_attribute("style", &format!("width: 80px; height: 80px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 1.2rem; transform: scale(1.2) rotateZ(360deg); transition: transform 0.5s ease {}ms;", i * 100)).unwrap();
                                        }
                                    }
                                }
                                style="padding: 1rem 2rem; background: linear-gradient(45deg, #5f27cd, #00d2d3); color: white; border: none; border-radius: 12px; cursor: pointer; font-weight: 600; font-size: 1rem;"
                            >
                                "Animate Sequence"
                            </button>
                        </div>
                    </div>
                </section>
            </main>

            <footer style="text-align: center; margin-top: 4rem; padding: 2rem; color: white; opacity: 0.8;">
                <p style="font-size: 1.2rem; margin-bottom: 1rem;">
                    "Built with Leptos Motion's SignalBasedAnimationController"
                </p>
                <p style="font-size: 1rem;">
                    "High-performance animations with proper signal tracking and WASM memory management"
                </p>
            </footer>
        </div>
    }
}
