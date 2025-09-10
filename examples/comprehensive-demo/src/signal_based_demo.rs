//! Signal-Based Demo using proven WASM + signals patterns
//!
//! This demo follows the exact patterns from the user's guide to ensure
//! no browser hanging issues occur.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::simple_signal_based_motion_div::*;
use std::collections::HashMap;

#[component]
pub fn SignalBasedDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 SignalBasedDemo: Component created".into());

    // ✅ Create reactive signals for animation state
    let (is_animated, set_animated) = signal(false);
    let (x_pos, set_x_pos) = signal(0.0);
    let (y_pos, set_y_pos) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    let (scale, set_scale) = signal(1.0);
    let (opacity, set_opacity) = signal(1.0);

    // ✅ Create reactive animation target using signals
    let animate_target = move || {
        let mut target = HashMap::new();
        if is_animated.get() {
            target.insert("x".to_string(), AnimationValue::Pixels(x_pos.get()));
            target.insert("y".to_string(), AnimationValue::Pixels(y_pos.get()));
            target.insert(
                "rotateZ".to_string(),
                AnimationValue::Degrees(rotation.get()),
            );
            target.insert("scale".to_string(), AnimationValue::Number(scale.get()));
            target.insert("opacity".to_string(), AnimationValue::Number(opacity.get()));
        } else {
            target.insert("x".to_string(), AnimationValue::Pixels(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }
        target
    };

    // ✅ Create transition signal
    let transition_signal = signal(Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    });

    // ✅ Create visibility signal
    let (is_visible, set_is_visible) = signal(true);

    // ✅ Create animate signal that properly tracks changes
    let (animate_signal, set_animate_signal) = signal(animate_target());

    // ✅ Effect to update animate signal when dependencies change
    Effect::new(move |_| {
        let new_target = animate_target();
        set_animate_signal.set(new_target);
    });

    // ✅ Button handlers that update signals
    let handle_animate = move |_| {
        web_sys::console::log_1(&"🎬 SignalBasedDemo: Starting animation".into());
        set_animated.set(true);
        set_x_pos.set(200.0);
        set_y_pos.set(100.0);
        set_rotation.set(180.0);
        set_scale.set(1.5);
        set_opacity.set(0.7);
    };

    let handle_reset = move |_| {
        web_sys::console::log_1(&"🔄 SignalBasedDemo: Resetting animation".into());
        set_animated.set(false);
        set_x_pos.set(0.0);
        set_y_pos.set(0.0);
        set_rotation.set(0.0);
        set_scale.set(1.0);
        set_opacity.set(1.0);
    };

    let handle_toggle_visibility = move |_| {
        set_is_visible.update(|visible| *visible = !*visible);
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"🎯 Signal-Based Motion Demo"</h1>
            <p>"This demo uses the proven signal-based patterns from the user's guide."</p>

            <div style="margin: 20px 0;">
                <button
                    on:click=handle_animate
                    style="margin-right: 10px; padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Animate"
                </button>
                <button
                    on:click=handle_reset
                    style="margin-right: 10px; padding: 10px 20px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Reset"
                </button>
                <button
                    on:click=handle_toggle_visibility
                    style="padding: 10px 20px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Toggle Visibility"
                </button>
            </div>

            <div style="margin: 20px 0;">
                <h3>"Animation State:"</h3>
                <p>{move || format!("Animated: {}", is_animated.get())}</p>
                <p>{move || format!("X: {:.1}px", x_pos.get())}</p>
                <p>{move || format!("Y: {:.1}px", y_pos.get())}</p>
                <p>{move || format!("Rotation: {:.1}°", rotation.get())}</p>
                <p>{move || format!("Scale: {:.1}", scale.get())}</p>
                <p>{move || format!("Opacity: {:.1}", opacity.get())}</p>
                <p>{move || format!("Visible: {}", is_visible.get())}</p>
            </div>

            <div style="margin: 20px 0; border: 2px solid #007bff; padding: 20px; border-radius: 8px;">
                <h3>"Animated Element:"</h3>
                <SimpleSignalBasedMotionDiv
                    initial={
                        let mut initial = HashMap::new();
                        initial.insert("x".to_string(), AnimationValue::Pixels(0.0));
                        initial.insert("y".to_string(), AnimationValue::Pixels(0.0));
                        initial.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
                        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
                        initial.insert("opacity".to_string(), AnimationValue::Number(1.0));
                        initial
                    }
                    animate=animate_signal
                >
                    <div style="
                        width: 100px;
                        height: 100px;
                        background: linear-gradient(45deg, #007bff, #28a745);
                        border-radius: 8px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                    ">
                        "Motion"
                    </div>
                </SimpleSignalBasedMotionDiv>
            </div>

            <div style="margin: 20px 0; border: 2px solid #28a745; padding: 20px; border-radius: 8px;">
                <h3>"Reactive Element:"</h3>
                <ReactiveSimpleMotionDiv
                    animate=animate_signal
                    is_visible=is_visible
                >
                    <div style="
                        width: 80px;
                        height: 80px;
                        background: linear-gradient(45deg, #dc3545, #ffc107);
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                    ">
                        "Reactive"
                    </div>
                </ReactiveSimpleMotionDiv>
            </div>

            <div style="margin: 20px 0; border: 2px solid #ffc107; padding: 20px; border-radius: 8px;">
                <h3>"Simple Element:"</h3>
                <SimpleSignalBasedMotionDiv
                    initial={
                        let mut initial = HashMap::new();
                        initial.insert("x".to_string(), AnimationValue::Pixels(0.0));
                        initial.insert("y".to_string(), AnimationValue::Pixels(0.0));
                        initial.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
                        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
                        initial.insert("opacity".to_string(), AnimationValue::Number(1.0));
                        initial
                    }
                    animate=animate_signal
                >
                    <div style="
                        width: 60px;
                        height: 60px;
                        background: linear-gradient(45deg, #6f42c1, #e83e8c);
                        border-radius: 4px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                    ">
                        "Simple"
                    </div>
                </SimpleSignalBasedMotionDiv>
            </div>
        </div>
    }
}
