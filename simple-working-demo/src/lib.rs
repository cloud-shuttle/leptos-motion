//! Phase 2 Reactive Animation Demo Library
//!
//! This library demonstrates the leptos-motion library with actual working animations.

use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

// Global state for animation values
static ANIMATION_STATE: std::sync::OnceLock<(
    RwSignal<f64>, // scale
    RwSignal<f64>, // rotation
    RwSignal<f64>, // opacity
    RwSignal<f64>, // x_position
    RwSignal<f64>, // y_position
    RwSignal<HashMap<String, AnimationValue>>, // animate_signal
)> = std::sync::OnceLock::new();

fn get_animation_state() -> &'static (
    RwSignal<f64>,
    RwSignal<f64>,
    RwSignal<f64>,
    RwSignal<f64>,
    RwSignal<f64>,
    RwSignal<HashMap<String, AnimationValue>>,
) {
    ANIMATION_STATE.get_or_init(|| {
        let scale = RwSignal::new(1.0);
        let rotation = RwSignal::new(0.0);
        let opacity = RwSignal::new(1.0);
        let x_position = RwSignal::new(0.0);
        let y_position = RwSignal::new(0.0);
        let animate_signal = RwSignal::new(HashMap::new());

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
            animate_signal.set(animations);
        });

        (scale, rotation, opacity, x_position, y_position, animate_signal)
    })
}

/// Initialize the demo application
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

/// Main App component
#[component]
fn App() -> impl IntoView {
    view! {
        <div style="max-width: 1200px; margin: 0 auto; padding: 20px;">
            <h1 style="text-align: center; color: white; margin-bottom: 30px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                "Simple Working Demo - Leptos Motion"
            </h1>
            <p style="text-align: center; color: white; font-size: 1.2em; margin-bottom: 30px; opacity: 0.9;">
                "WASM-powered animations with interactive controls"
            </p>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 30px;">
                <div style="background: white; border-radius: 12px; padding: 25px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-bottom: 20px; font-size: 1.3em;">"Animation Controls"</h3>
                    <Controls/>
                </div>

                <div style="background: white; border-radius: 12px; padding: 30px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); min-height: 400px; display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden;">
                    <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: radial-gradient(circle at 20% 20%, rgba(102, 126, 234, 0.1) 0%, transparent 50%), radial-gradient(circle at 80% 80%, rgba(118, 75, 162, 0.1) 0%, transparent 50%); pointer-events: none;"></div>
                    <AnimationArea/>
                </div>
            </div>

            <div style="background: white; border-radius: 12px; padding: 25px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); margin-top: 20px;">
                <h3 style="color: #333; margin-bottom: 15px; font-size: 1.3em;">"Animation Status"</h3>
                <StatusPanel/>
            </div>
        </div>
    }
}

/// Controls component
#[component]
fn Controls() -> impl IntoView {
    let (scale, rotation, opacity, x_position, y_position, _animate_signal) = get_animation_state();

    view! {
        <div>
            <div class="control-group">
                <label>"Scale: " {move || format!("{:.1}", scale.get())}</label>
                <input
                    type="range"
                    min="0.5"
                    max="2.0"
                    step="0.1"
                    value=move || scale.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                        scale.set(value);
                    }
                />
            </div>

            <div class="control-group">
                <label>"Rotation: " {move || format!("{:.0}°", rotation.get())}</label>
                <input
                    type="range"
                    min="0"
                    max="360"
                    step="10"
                    value=move || rotation.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                        rotation.set(value);
                    }
                />
            </div>

            <div class="control-group">
                <label>"Opacity: " {move || format!("{:.1}", opacity.get())}</label>
                <input
                    type="range"
                    min="0.0"
                    max="1.0"
                    step="0.1"
                    value=move || opacity.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                        opacity.set(value);
                    }
                />
            </div>

            <div class="control-group">
                <label>"X Position: " {move || format!("{:.0}px", x_position.get())}</label>
                <input
                    type="range"
                    min="-200"
                    max="200"
                    step="10"
                    value=move || x_position.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                        x_position.set(value);
                    }
                />
            </div>

            <div class="control-group">
                <label>"Y Position: " {move || format!("{:.0}px", y_position.get())}</label>
                <input
                    type="range"
                    min="-200"
                    max="200"
                    step="10"
                    value=move || y_position.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                        y_position.set(value);
                    }
                />
            </div>

            <div style="margin: 20px 0;">
                <button
                    class="reset-button"
                    on:click=move |_| {
                        scale.set(1.0);
                        rotation.set(0.0);
                        opacity.set(1.0);
                        x_position.set(0.0);
                        y_position.set(0.0);
                    }
                >
                    "Reset Animation"
                </button>
            </div>
        </div>
    }
}

/// Animation area component
#[component]
fn AnimationArea() -> impl IntoView {
    let (_scale, _rotation, _opacity, _x_position, _y_position, animate_signal) = get_animation_state();

    // Create initial values
    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("transform".to_string(), AnimationValue::String("translate(0px, 0px) scale(1) rotate(0deg)".to_string()));
        initial.insert("opacity".to_string(), AnimationValue::Number(1.0));
        initial
    };

    // Create transition configuration
    let transition = Transition {
        duration: Some(0.3),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div>
            <ReactiveMotionDivV2
                initial=initial_values
                animate=animate_signal.read_only()
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
            </ReactiveMotionDivV2>
        </div>
    }
}

/// Status panel component
#[component]
fn StatusPanel() -> impl IntoView {
    let (scale, rotation, opacity, x_position, y_position, _animate_signal) = get_animation_state();

    view! {
        <div>
            <div class="status-item">
                <span class="status-label">"Scale:"</span>
                <span class="status-value">{move || format!("{:.1}", scale.get())}</span>
            </div>
            <div class="status-item">
                <span class="status-label">"Rotation:"</span>
                <span class="status-value">{move || format!("{:.0}°", rotation.get())}</span>
            </div>
            <div class="status-item">
                <span class="status-label">"Opacity:"</span>
                <span class="status-value">{move || format!("{:.1}", opacity.get())}</span>
            </div>
            <div class="status-item">
                <span class="status-label">"Position:"</span>
                <span class="status-value">{move || format!("({:.0}px, {:.0}px)", x_position.get(), y_position.get())}</span>
            </div>
        </div>
    }
}