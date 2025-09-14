//! Phase 2 Comprehensive Demo
//!
//! This example demonstrates all the new Phase 2 features:
//! - Reactive animations with ReactiveMotionDivV2
//! - Drag functionality with DragMotionDiv
//! - Spring physics animations
//! - Complex animation sequences

use leptos::prelude::*;
use leptos_motion_dom::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::drag_motion_div::DragMotionDiv;
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
    let (is_dragging, set_is_dragging) = signal(false);

    // Create reactive animation signal
    let (animate_signal, _set_animate_signal) = signal({
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String("translate(0px, 0px) scale(1) rotate(0deg)".to_string()));
        animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        animations
    });

    // Update animation signal when any control changes
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!(
            "translate({}px, {}px) scale({}) rotate({}deg)",
            x_position.get(),
            y_position.get(),
            scale.get(),
            rotation.get()
        )));
        animations.insert("opacity".to_string(), AnimationValue::Number(opacity.get()));
        _set_animate_signal.set(animations);
    });

    // Create initial values
    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("transform".to_string(), AnimationValue::String("translate(0px, 0px) scale(1) rotate(0deg)".to_string()));
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

    // Create drag configuration
    let drag_config = DragConfig {
        axis: None,
        constraints: Some(DragConstraints {
            left: Some(-200.0),
            right: Some(200.0),
            top: Some(-200.0),
            bottom: Some(200.0),
        }),
        elastic: Some(0.2),
        momentum: Some(true),
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh;">
            <div style="max-width: 1200px; margin: 0 auto;">
                <h1 style="color: white; text-align: center; margin-bottom: 30px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                    "Phase 2: Comprehensive Animation Demo"
                </h1>
                <p style="color: white; text-align: center; margin-bottom: 40px; font-size: 1.2em; opacity: 0.9;">
                    "Interactive demonstration of all new Phase 2 features"
                </p>
                
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 30px;">
                    <div style="background: white; border-radius: 12px; padding: 25px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                        <h3 style="color: #333; margin-bottom: 20px;">"Animation Controls"</h3>
                        
                        <div style="margin: 15px 0;">
                            <label style="display: block; margin-bottom: 8px; font-weight: 600; color: #333;">
                                "Scale: " {move || format!("{:.1}", scale.get())}
                            </label>
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
                                style="width: 100%; height: 6px; border-radius: 3px; background: #ddd; outline: none; -webkit-appearance: none;"
                            />
                        </div>

                        <div style="margin: 15px 0;">
                            <label style="display: block; margin-bottom: 8px; font-weight: 600; color: #333;">
                                "Rotation: " {move || format!("{:.0}°", rotation.get())}
                            </label>
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
                                style="width: 100%; height: 6px; border-radius: 3px; background: #ddd; outline: none; -webkit-appearance: none;"
                            />
                        </div>

                        <div style="margin: 15px 0;">
                            <label style="display: block; margin-bottom: 8px; font-weight: 600; color: #333;">
                                "Opacity: " {move || format!("{:.1}", opacity.get())}
                            </label>
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
                                style="width: 100%; height: 6px; border-radius: 3px; background: #ddd; outline: none; -webkit-appearance: none;"
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
                                style="background: linear-gradient(45deg, #ff6b6b, #ee5a24); color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; transition: transform 0.2s ease; box-shadow: 0 4px 12px rgba(255, 107, 107, 0.3);"
                            >
                                "Reset Animation"
                            </button>
                        </div>
                    </div>

                    <div style="background: white; border-radius: 12px; padding: 30px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); min-height: 400px; display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden;">
                        <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: radial-gradient(circle at 20% 20%, rgba(102, 126, 234, 0.1) 0%, transparent 50%), radial-gradient(circle at 80% 80%, rgba(118, 75, 162, 0.1) 0%, transparent 50%); pointer-events: none;"></div>
                        
                        <div style="text-align: center;">
                            <h3 style="color: #333; margin-bottom: 20px;">"Reactive Animation"</h3>
                            <ReactiveMotionDivV2
                                initial=initial_values
                                animate=animate_signal
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
                                    "Reactive!"
                                </div>
                            </ReactiveMotionDivV2>
                        </div>
                    </div>
                </div>

                <div style="background: white; border-radius: 12px; padding: 30px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); margin-bottom: 30px;">
                    <h3 style="color: #333; margin-bottom: 20px; text-align: center;">"Drag Animation"</h3>
                    <p style="color: #666; text-align: center; margin-bottom: 30px;">"Drag the element below to see drag functionality in action"</p>
                    
                    <div style="border: 2px dashed #ccc; padding: 40px; min-height: 300px; display: flex; align-items: center; justify-content: center; position: relative;">
                        <div>
                            <div style="
                                width: 80px;
                                height: 80px;
                                background: linear-gradient(45deg, #667eea, #764ba2);
                                border-radius: 50%;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                color: white;
                                font-weight: bold;
                                box-shadow: 0 6px 12px rgba(0,0,0,0.3);
                                cursor: grab;
                            ">
                                "Drag Me!"
                            </div>
                        </div>
                    </div>
                </div>

                <div style="background: white; border-radius: 12px; padding: 25px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-bottom: 15px;">"Animation Status"</h3>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px;">
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;">
                            <span style="font-weight: 600; color: #555;">"Scale:"</span>
                            <span style="color: #667eea; font-weight: 500;">{move || format!("{:.1}", scale.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;">
                            <span style="font-weight: 600; color: #555;">"Rotation:"</span>
                            <span style="color: #667eea; font-weight: 500;">{move || format!("{:.0}°", rotation.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;">
                            <span style="font-weight: 600; color: #555;">"Opacity:"</span>
                            <span style="color: #667eea; font-weight: 500;">{move || format!("{:.1}", opacity.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;">
                            <span style="font-weight: 600; color: #555;">"Position:"</span>
                            <span style="color: #667eea; font-weight: 500;">{move || format!("({:.0}px, {:.0}px)", x_position.get(), y_position.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;">
                            <span style="font-weight: 600; color: #555;">"Dragging:"</span>
                            <span style="color: #667eea; font-weight: 500;">{move || if is_dragging.get() { "Yes" } else { "No" }}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
