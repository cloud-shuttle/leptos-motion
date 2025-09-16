//! Performance Demo for Leptos Motion
//!
//! This demo benchmarks the performance of the leptos-motion library
//! with various animation scenarios and stress tests.

use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::*;
use std::collections::HashMap;

/// Initialize the performance demo
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <PerformanceDemo/> })
}

/// Main performance demo component
#[component]
fn PerformanceDemo() -> impl IntoView {
    let (is_running, set_running) = create_signal(false);
    let (fps, set_fps) = create_signal(0.0);
    let (frame_time, set_frame_time) = create_signal(0.0);
    let (animation_count, set_animation_count) = create_signal(0);

    view! {
        <div style="max-width: 1400px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;">
            <div style="text-align: center; margin-bottom: 40px;">
                <h1 style="color: #333; font-size: 2.5em; margin-bottom: 10px;">
                    "🚀 Leptos Motion Performance Benchmark"
                </h1>
                <p style="color: #666; font-size: 1.2em;">
                    "WASM-powered performance testing for animation library"
                </p>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 30px; margin: 30px 0;">
                <div style="background: #f9f9f9; border-radius: 12px; padding: 25px; box-shadow: 0 4px 16px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0; margin-bottom: 20px; font-size: 1.4em;">
                        "🎬 Animation Performance Test"
                    </h3>
                    
                    <div style="margin-bottom: 20px;">
                        <button
                            style="background: linear-gradient(45deg, #667eea, #764ba2); color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; margin-right: 10px;"
                            on:click=move |_| {
                                set_running.set(true);
                                start_animation_test();
                            }
                            disabled=move || is_running.get()
                        >
                            {move || if is_running.get() { "Running..." } else { "Start Performance Test" }}
                        </button>
                        
                        <button
                            style="background: #f44336; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer;"
                            on:click=move |_| {
                                set_running.set(false);
                                stop_performance_test();
                            }
                            disabled=move || !is_running.get()
                        >
                            "Stop Test"
                        </button>
                    </div>

                    <div style="background: white; border: 2px dashed #ddd; border-radius: 8px; padding: 20px; min-height: 300px; position: relative; overflow: hidden;">
                        <PerformanceAnimationArea is_running=is_running/>
                    </div>

                    <div style="background: #f0f8ff; border-radius: 8px; padding: 20px; margin-top: 20px;">
                        <h4 style="color: #1976d2; margin-top: 0; margin-bottom: 15px;">"Performance Metrics"</h4>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #e0e0e0;">
                            <span style="font-weight: 600; color: #555;">"FPS:"</span>
                            <span style="color: #1976d2; font-weight: 500;">{move || format!("{:.1}", fps.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #e0e0e0;">
                            <span style="font-weight: 600; color: #555;">"Frame Time:"</span>
                            <span style="color: #1976d2; font-weight: 500;">{move || format!("{:.2}ms", frame_time.get())}</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0;">
                            <span style="font-weight: 600; color: #555;">"Animations:"</span>
                            <span style="color: #1976d2; font-weight: 500;">{move || animation_count.get()}</span>
                        </div>
                    </div>
                </div>

                <div style="background: #f9f9f9; border-radius: 12px; padding: 25px; box-shadow: 0 4px 16px rgba(0,0,0,0.1);">
                    <h3 style="color: #333; margin-top: 0; margin-bottom: 20px; font-size: 1.4em;">
                        "📊 Performance Results"
                    </h3>
                    
                    <div style="background: white; border: 2px dashed #ddd; border-radius: 8px; padding: 20px; min-height: 300px; position: relative; overflow: hidden;">
                        <div style="text-align: center; padding: 50px; color: #666;">
                            "Performance chart will appear here during testing"
                        </div>
                    </div>

                    <div style="background: #f0f8ff; border-radius: 8px; padding: 20px; margin-top: 20px;">
                        <h4 style="color: #1976d2; margin-top: 0; margin-bottom: 15px;">"Benchmark Results"</h4>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #e0e0e0;">
                            <span style="font-weight: 600; color: #555;">"Average FPS:"</span>
                            <span style="color: #1976d2; font-weight: 500;">"--"</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #e0e0e0;">
                            <span style="font-weight: 600; color: #555;">"Min FPS:"</span>
                            <span style="color: #1976d2; font-weight: 500;">"--"</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #e0e0e0;">
                            <span style="font-weight: 600; color: #555;">"Max FPS:"</span>
                            <span style="color: #1976d2; font-weight: 500;">"--"</span>
                        </div>
                        <div style="display: flex; justify-content: space-between; padding: 8px 0;">
                            <span style="font-weight: 600; color: #555;">"Test Duration:"</span>
                            <span style="color: #1976d2; font-weight: 500;">"--"</span>
                        </div>
                    </div>
                </div>
            </div>

            <div style="background: #e8f5e8; border: 1px solid #4caf50; border-radius: 8px; padding: 20px; margin: 30px 0; text-align: center;">
                <h3 style="color: #2e7d32; margin: 0 0 10px 0;">"🎯 Performance Benchmark Results"</h3>
                <p style="color: #2e7d32; margin: 0;">
                    "This demo tests the performance of leptos-motion with various animation scenarios."
                </p>
            </div>
        </div>
    }
}

/// Performance animation area component
#[component]
fn PerformanceAnimationArea(is_running: ReadSignal<bool>) -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (rotation, set_rotation) = create_signal(0.0);
    let (opacity, set_opacity) = create_signal(1.0);
    let (x_position, set_x_position) = create_signal(0.0);
    let (y_position, set_y_position) = create_signal(0.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

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
        set_animate_signal.set(animations);
    });

    // Performance test animation loop
    Effect::new(move |_| {
        if is_running.get() {
            let _interval = set_interval_with_handle(
                move || {
                    // Random animation values for stress testing
                    set_scale.set(0.5 + (js_sys::Math::random() * 1.5));
                    set_rotation.set(js_sys::Math::random() * 360.0);
                    set_opacity.set(0.3 + (js_sys::Math::random() * 0.7));
                    set_x_position.set((js_sys::Math::random() - 0.5) * 200.0);
                    set_y_position.set((js_sys::Math::random() - 0.5) * 200.0);
                },
                std::time::Duration::from_millis(50)
            );
        }
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
        duration: Some(0.3),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div style="position: relative; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
            <ReactiveMotionDivV2
                initial=initial_values
                animate=animate_signal
                transition=transition
            >
                <div style="
                    width: 80px;
                    height: 80px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    box-shadow: 0 4px 8px rgba(0,0,0,0.2);
                ">
                    "WASM"
                </div>
            </ReactiveMotionDivV2>
        </div>
    }
}

// Performance monitoring functions
fn start_animation_test() {
    web_sys::console::log_1(&"Starting animation performance test".into());
}

fn stop_performance_test() {
    web_sys::console::log_1(&"Stopping animation performance test".into());
}