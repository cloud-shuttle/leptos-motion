//! Phase 2 Reactive Animation Demo Library
//!
//! This library demonstrates the leptos-motion library with actual working animations.

use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::*;
use std::collections::HashMap;

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
    
    // Mount controls to the controls container
    if let Some(controls_container) = leptos::web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("controls-container")) {
        mount_to(controls_container, || view! { <Controls/> });
    }
    
    // Mount animation to the animation container
    if let Some(animation_container) = leptos::web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("animation-container")) {
        mount_to(animation_container, || view! { <AnimationArea/> });
    }
    
    // Mount status to the status container
    if let Some(status_container) = leptos::web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("status-container")) {
        mount_to(status_container, || view! { <StatusPanel/> });
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