//! Phase 2 Reactive Animation Demo
//!
//! This example demonstrates the new ReactiveMotionDiv component with:
//! - Reactive animations that respond to signal changes
//! - Proper animation engine integration
//! - Interactive controls for testing

use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div::ReactiveMotionDiv;
use leptos_motion_dom::*;
use std::collections::HashMap;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    // Animation state signals
    let (scale, set_scale) = signal(1.0);
    let (rotation, set_rotation) = signal(0.0);
    let (opacity, set_opacity) = signal(1.0);
    let (x_position, set_x_position) = signal(0.0);
    let (y_position, set_y_position) = signal(0.0);

    // Create reactive animation signal using individual properties
    let (animate_signal, _set_animate_signal) = signal({
        let mut animations = HashMap::new();
        animations.insert("x".to_string(), AnimationValue::Pixels(0.0));
        animations.insert("y".to_string(), AnimationValue::Pixels(0.0));
        animations.insert("scale".to_string(), AnimationValue::Number(1.0));
        animations.insert("rotation".to_string(), AnimationValue::Degrees(0.0));
        animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        animations
    });

    // Update animation signal when any control changes
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("x".to_string(), AnimationValue::Pixels(x_position.get()));
        animations.insert("y".to_string(), AnimationValue::Pixels(y_position.get()));
        animations.insert("scale".to_string(), AnimationValue::Number(scale.get()));
        animations.insert("rotation".to_string(), AnimationValue::Degrees(rotation.get()));
        animations.insert("opacity".to_string(), AnimationValue::Number(opacity.get()));
        _set_animate_signal.set(animations);
    });

    // Create initial values using individual properties
    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("x".to_string(), AnimationValue::Pixels(0.0));
        initial.insert("y".to_string(), AnimationValue::Pixels(0.0));
        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
        initial.insert("rotation".to_string(), AnimationValue::Degrees(0.0));
        initial.insert("opacity".to_string(), AnimationValue::Number(1.0));
        initial
    };

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Phase 2: Reactive Animation Demo"</h1>
            <p>"This demo shows the new ReactiveMotionDiv component with proper animation engine integration."</p>
            
            <div style="display: flex; gap: 20px; margin: 20px 0;">
                <div style="flex: 1;">
                    <h3>"Animation Controls"</h3>
                    
                    <div style="margin: 10px 0;">
                        <label>"Scale: " {move || format!("{:.1}", scale.get())}</label>
                        <input
                            type="range"
                            min="0.5"
                            max="2.0"
                            step="0.1"
                            value=move || scale.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                set_scale.set(value);
                            }
                        />
                    </div>

                    <div style="margin: 10px 0;">
                        <label>"Rotation: " {move || format!("{:.0}°", rotation.get())}</label>
                        <input
                            type="range"
                            min="0"
                            max="360"
                            step="10"
                            value=move || rotation.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_rotation.set(value);
                            }
                        />
                    </div>

                    <div style="margin: 10px 0;">
                        <label>"Opacity: " {move || format!("{:.1}", opacity.get())}</label>
                        <input
                            type="range"
                            min="0.0"
                            max="1.0"
                            step="0.1"
                            value=move || opacity.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                set_opacity.set(value);
                            }
                        />
                    </div>

                    <div style="margin: 10px 0;">
                        <label>"X Position: " {move || format!("{:.0}px", x_position.get())}</label>
                        <input
                            type="range"
                            min="-200"
                            max="200"
                            step="10"
                            value=move || x_position.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_x_position.set(value);
                            }
                        />
                    </div>

                    <div style="margin: 10px 0;">
                        <label>"Y Position: " {move || format!("{:.0}px", y_position.get())}</label>
                        <input
                            type="range"
                            min="-200"
                            max="200"
                            step="10"
                            value=move || y_position.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_y_position.set(value);
                            }
                        />
                    </div>

                    <div style="margin: 20px 0;">
                        <button
                            on:click=move |_| {
                                set_scale.set(1.0);
                                set_rotation.set(0.0);
                                set_opacity.set(1.0);
                                set_x_position.set(0.0);
                                set_y_position.set(0.0);
                            }
                        >
                            "Reset Animation"
                        </button>
                    </div>
                </div>

                <div style="flex: 1;">
                    <h3>"Animated Element"</h3>
                    <div style="border: 2px dashed #ccc; padding: 20px; min-height: 300px; display: flex; align-items: center; justify-content: center;">
                        <ReactiveMotionDiv
                            initial=initial_values
                            animate=Box::new(move || animate_signal.get())
                            transition=transition
                        >
                            <div style="
                                width: 100px;
                                height: 100px;
                                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                                border-radius: 10px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: white;
                                font-weight: bold;
                                box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                            ">
                                "Animated!"
                            </div>
                        </ReactiveMotionDiv>
                    </div>
                </div>
            </div>

            <div style="margin-top: 30px; padding: 20px; background: #f5f5f5; border-radius: 8px;">
                <h3>"Animation Status"</h3>
                <p>"Scale: " {move || format!("{:.1}", scale.get())}</p>
                <p>"Rotation: " {move || format!("{:.0}°", rotation.get())}</p>
                <p>"Opacity: " {move || format!("{:.1}", opacity.get())}</p>
                <p>"Position: " {move || format!("({:.0}px, {:.0}px)", x_position.get(), y_position.get())}</p>
            </div>
        </div>
    }
}
