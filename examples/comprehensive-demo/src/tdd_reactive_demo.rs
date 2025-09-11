//! TDD Reactive Animation Demo
//!
//! This demo showcases our TDD-implemented reactive animation system
//! with proper signal tracking and effect dependencies.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::{
    signal_based_animation_controller::SignalBasedAnimationController, ReactiveMotionDiv,
};
use std::collections::HashMap;

#[component]
pub fn TddReactiveDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 TDD Reactive Demo: Component created".into());

    // ✅ Create reactive signals for animation state
    let (is_animated, set_animated) = signal(false);
    let (x_pos, set_x_pos) = signal(0.0);
    let (y_pos, set_y_pos) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    let (scale, set_scale) = signal(1.0);
    let (opacity, set_opacity) = signal(1.0);
    let (color_hue, set_color_hue) = signal(0.0);

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
            target.insert(
                "filter".to_string(),
                AnimationValue::String(format!("hue-rotate({}deg)", color_hue.get())),
            );
        } else {
            target.insert("x".to_string(), AnimationValue::Pixels(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "filter".to_string(),
                AnimationValue::String("hue-rotate(0deg)".to_string()),
            );
        }
        target
    };

    // ✅ Create initial animation target
    let initial_target = {
        let mut initial = HashMap::new();
        initial.insert("x".to_string(), AnimationValue::Pixels(0.0));
        initial.insert("y".to_string(), AnimationValue::Pixels(0.0));
        initial.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
        initial.insert("opacity".to_string(), AnimationValue::Number(1.0));
        initial.insert(
            "filter".to_string(),
            AnimationValue::String("hue-rotate(0deg)".to_string()),
        );
        initial
    };

    // ✅ Create signal-based animation controller
    let controller = SignalBasedAnimationController::new(initial_target.clone());

    // ✅ Button handlers that update signals
    let handle_animate = move |_| {
        web_sys::console::log_1(&"🎬 TDD Demo: Starting animation".into());
        set_animated.set(true);
        set_x_pos.set(200.0);
        set_y_pos.set(100.0);
        set_rotation.set(180.0);
        set_scale.set(1.5);
        set_opacity.set(0.7);
        set_color_hue.set(180.0);
    };

    let handle_reset = move |_| {
        web_sys::console::log_1(&"🔄 TDD Demo: Resetting animation".into());
        set_animated.set(false);
        set_x_pos.set(0.0);
        set_y_pos.set(0.0);
        set_rotation.set(0.0);
        set_scale.set(1.0);
        set_opacity.set(1.0);
        set_color_hue.set(0.0);
    };

    let handle_rainbow = move |_| {
        web_sys::console::log_1(&"🌈 TDD Demo: Rainbow animation".into());
        set_animated.set(true);
        set_x_pos.set(100.0);
        set_y_pos.set(50.0);
        set_rotation.set(360.0);
        set_scale.set(1.2);
        set_opacity.set(0.8);
        set_color_hue.set(360.0);
    };

    let handle_bounce = move |_| {
        web_sys::console::log_1(&"⚡ TDD Demo: Bounce animation".into());
        set_animated.set(true);
        set_x_pos.set(150.0);
        set_y_pos.set(-50.0);
        set_rotation.set(0.0);
        set_scale.set(1.3);
        set_opacity.set(1.0);
        set_color_hue.set(120.0);
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto;">
            <h1>"🎯 TDD Reactive Animation Demo"</h1>
            <p>"This demo showcases our TDD-implemented reactive animation system with proper signal tracking and effect dependencies."</p>

            <div style="margin: 20px 0; display: flex; gap: 10px; flex-wrap: wrap;">
                <button
                    on:click=handle_animate
                    style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Animate"
                </button>
                <button
                    on:click=handle_reset
                    style="padding: 10px 20px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Reset"
                </button>
                <button
                    on:click=handle_rainbow
                    style="padding: 10px 20px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4, #feca57); color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Rainbow"
                </button>
                <button
                    on:click=handle_bounce
                    style="padding: 10px 20px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Bounce"
                </button>
            </div>

            <div style="margin: 20px 0; padding: 15px; background: #f8f9fa; border-radius: 8px;">
                <h3>"Animation State:"</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px;">
                    <p>{move || format!("Animated: {}", is_animated.get())}</p>
                    <p>{move || format!("X: {:.1}px", x_pos.get())}</p>
                    <p>{move || format!("Y: {:.1}px", y_pos.get())}</p>
                    <p>{move || format!("Rotation: {:.1}°", rotation.get())}</p>
                    <p>{move || format!("Scale: {:.1}", scale.get())}</p>
                    <p>{move || format!("Opacity: {:.1}", opacity.get())}</p>
                    <p>{move || format!("Hue: {:.1}°", color_hue.get())}</p>
                </div>
            </div>

            <div style="margin: 20px 0; border: 2px solid #007bff; padding: 20px; border-radius: 8px; min-height: 200px; position: relative;">
                <h3>"ReactiveMotionDiv Element:"</h3>
                <ReactiveMotionDiv
                    initial=initial_target.clone()
                    animate=animate_target()
                    style="position: absolute; top: 50px; left: 50px;".to_string()
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
                        transition: all 0.3s ease;
                    ">
                        "TDD"
                    </div>
                </ReactiveMotionDiv>
            </div>

            <div style="margin: 20px 0; border: 2px solid #28a745; padding: 20px; border-radius: 8px; min-height: 200px; position: relative;">
                <h3>"Multiple Reactive Elements:"</h3>
                <ReactiveMotionDiv
                    initial=initial_target.clone()
                    animate=animate_target()
                    style="position: absolute; top: 50px; left: 50px;".to_string()
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
                        transition: all 0.3s ease;
                    ">
                        "1"
                    </div>
                </ReactiveMotionDiv>
                <ReactiveMotionDiv
                    initial=initial_target.clone()
                    animate=animate_target()
                    style="position: absolute; top: 50px; left: 200px;".to_string()
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
                        transition: all 0.3s ease;
                    ">
                        "2"
                    </div>
                </ReactiveMotionDiv>
                <ReactiveMotionDiv
                    initial=initial_target.clone()
                    animate=animate_target()
                    style="position: absolute; top: 50px; left: 350px;".to_string()
                >
                    <div style="
                        width: 70px;
                        height: 70px;
                        background: linear-gradient(45deg, #20c997, #fd7e14);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-weight: bold;
                        box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                        transition: all 0.3s ease;
                    ">
                        "3"
                    </div>
                </ReactiveMotionDiv>
            </div>

            <div style="margin: 20px 0; padding: 15px; background: #e9ecef; border-radius: 8px;">
                <h3>"TDD Implementation Features:"</h3>
                <ul style="margin: 10px 0; padding-left: 20px;">
                    <li>"✅ Proper signal tracking with Effect::new()"</li>
                    <li>"✅ Reactive animation targets that update automatically"</li>
                    <li>"✅ Signal-based animation controller"</li>
                    <li>"✅ WASM memory management with cleanup"</li>
                    <li>"✅ Multiple independent reactive elements"</li>
                    <li>"✅ Comprehensive test coverage"</li>
                </ul>
            </div>
        </div>
    }
}
