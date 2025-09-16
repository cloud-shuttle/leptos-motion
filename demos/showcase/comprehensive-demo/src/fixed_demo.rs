//! Fixed Demo using proven WASM + signals patterns
//!
//! This demo uses the FixedMotionDiv component to test if the hanging issue is resolved

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::fixed_motion_div::*;
use std::collections::HashMap;

#[component]
pub fn FixedDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 FixedDemo: Component created".into());

    // ✅ Create reactive signals for animation
    let (x_pos, set_x_pos) = signal(0.0);
    let (y_pos, set_y_pos) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    let (is_animated, set_animated) = signal(false);

    // ✅ Create reactive animation target using signals
    let (animate_target, set_animate_target) = signal({
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(0.0));
        target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target
    });

    // ✅ Update animation target when signals change
    Effect::new(move |_| {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(x_pos.get()));
        target.insert("y".to_string(), AnimationValue::Pixels(y_pos.get()));
        target.insert(
            "rotateZ".to_string(),
            AnimationValue::Degrees(rotation.get()),
        );
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(if is_animated.get() { 1.0 } else { 0.5 }),
        );
        set_animate_target.set(target);
    });

    // ✅ Create initial animation target
    let initial_target = {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(0.0));
        target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target
    };

    // ✅ Animation controls
    let animate = move |_| {
        set_x_pos.set(100.0);
        set_y_pos.set(50.0);
        set_rotation.set(45.0);
        set_animated.set(true);
    };

    let reset = move |_| {
        set_x_pos.set(0.0);
        set_y_pos.set(0.0);
        set_rotation.set(0.0);
        set_animated.set(false);
    };

    view! {
        <div style="padding: 2rem; max-width: 800px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 2rem;">
                <h1 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333;">
                    "🎯 Fixed Demo - WASM + Signals"
                </h1>
                <p style="color: #666; margin-bottom: 1rem;">
                    "Testing the fixed MotionDiv component with proper signal tracking"
                </p>
            </header>

            <main>
                <section style="margin-bottom: 3rem; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
                    <h2 style="margin-bottom: 1rem; color: #333;">"Fixed MotionDiv Test"</h2>

                    <div style="display: flex; gap: 1rem; margin-bottom: 2rem;">
                        <button
                            on:click=animate
                            style="padding: 0.5rem 1rem; background: #4ecdc4; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Animate"
                        </button>
                        <button
                            on:click=reset
                            style="padding: 0.5rem 1rem; background: #ff6b6b; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Reset"
                        </button>
                    </div>

                    <div style="display: flex; justify-content: center; align-items: center; min-height: 200px; position: relative;">
                        <FixedMotionDiv
                            initial=initial_target.clone()
                            animate=animate_target.into()
                            style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                        >
                            "Fixed"
                        </FixedMotionDiv>
                    </div>

                    <div style="margin-top: 1rem; font-size: 0.9rem; color: #666;">
                        <p>"Current values:"</p>
                        <p>{move || format!("x: {:.1}px", x_pos.get())}</p>
                        <p>{move || format!("y: {:.1}px", y_pos.get())}</p>
                        <p>{move || format!("rotation: {:.1}°", rotation.get())}</p>
                        <p>{move || format!("animated: {}", is_animated.get())}</p>
                    </div>
                </section>

                <section style="margin-bottom: 3rem; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
                    <h2 style="margin-bottom: 1rem; color: #333;">"Signal Tracking Test"</h2>
                    <p style="color: #666; margin-bottom: 1rem;">
                        "This tests if signals are properly tracked and DOM updates work without hanging."
                    </p>

                    <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
                        <button
                            on:click=move |_| set_x_pos.set(x_pos.get() + 10.0)
                            style="padding: 0.5rem 1rem; background: #45b7d1; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "X +10"
                        </button>
                        <button
                            on:click=move |_| set_y_pos.set(y_pos.get() + 10.0)
                            style="padding: 0.5rem 1rem; background: #96ceb4; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Y +10"
                        </button>
                        <button
                            on:click=move |_| set_rotation.set(rotation.get() + 15.0)
                            style="padding: 0.5rem 1rem; background: #feca57; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Rotate +15°"
                        </button>
                    </div>

                    <div style="display: flex; justify-content: center; align-items: center; min-height: 150px; position: relative;">
                        <FixedMotionDiv
                            initial=initial_target.clone()
                            animate=animate_target.into()
                            style="width: 80px; height: 80px; background: linear-gradient(45deg, #45b7d1, #96ceb4); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                        >
                            "Signal"
                        </FixedMotionDiv>
                    </div>
                </section>
            </main>
        </div>
    }
}
