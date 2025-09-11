//! Phase 5A: Memoization Tests
//!
//! Tests for memoization with create_memo to optimize performance

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue};
use std::collections::HashMap;
use wasm_bindgen_test::*;

use crate::{ReactiveMotionDiv, signal_based_animation_controller::SignalBasedAnimationController};

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to create a simple animation target
fn create_animation_target(key: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(key.to_string(), AnimationValue::Number(value));
    target
}

/// Helper function to create a complex animation target
fn create_complex_animation_target() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    target.insert(
        "transform".to_string(),
        AnimationValue::String("scale(1.2) rotate(45deg)".to_string()),
    );
    target.insert(
        "background-color".to_string(),
        AnimationValue::String("rgba(255, 0, 0, 0.5)".to_string()),
    );
    target.insert(
        "border-radius".to_string(),
        AnimationValue::String("10px".to_string()),
    );
    target.insert(
        "box-shadow".to_string(),
        AnimationValue::String("0 4px 8px rgba(0,0,0,0.3)".to_string()),
    );
    target
}

#[wasm_bindgen_test]
fn test_memoized_animation_target() {
    // RED PHASE: Test should fail initially
    let (counter, set_counter) = signal(0);
    let complex_target = create_complex_animation_target();

    // Create a memoized animation target that depends on counter
    let memoized_target = create_memo(move |_| {
        let count = counter.get();
        let mut target = complex_target.clone();
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", 1.0 + count as f64 * 0.1)),
        );
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            animate_memo=Some(memoized_target)
        >
            "Memoized animation target"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_style_calculation() {
    // RED PHASE: Test should fail initially
    let (width, set_width) = signal(100.0);
    let (height, set_height) = signal(100.0);

    // Create a memoized style calculation
    let memoized_style = create_memo(move |_| {
        let w = width.get();
        let h = height.get();
        format!(
            "width: {}px; height: {}px; background: linear-gradient(45deg, rgba(255,0,0,{}), rgba(0,0,255,{}))",
            w,
            h,
            w / 200.0,
            h / 200.0
        )
    });

    let component = view! {
        <ReactiveMotionDiv
            style_memo=Some(memoized_style)
        >
            "Memoized style calculation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_transition_config() {
    // RED PHASE: Test should fail initially
    let (speed, set_speed) = signal(1.0);
    let (easing_type, set_easing_type) = signal("ease".to_string());

    // Create a memoized transition config
    let memoized_transition = create_memo(move |_| {
        let s = speed.get();
        let e = easing_type.get();
        leptos_motion_core::Transition {
            duration: Some(2.0 / s),
            easing: Some(e),
            delay: Some(0.1 * s),
            repeat: None,
            repeat_type: None,
            repeat_delay: None,
            yoyo: None,
        }
    });

    let component = view! {
        <ReactiveMotionDiv
            transition_memo=Some(memoized_transition)
        >
            "Memoized transition config"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_complex_calculation() {
    // RED PHASE: Test should fail initially
    let (time, set_time) = signal(0.0);
    let (amplitude, set_amplitude) = signal(1.0);
    let (frequency, set_frequency) = signal(1.0);

    // Create a memoized complex calculation for wave animation
    let memoized_wave = create_memo(move |_| {
        let t = time.get();
        let a = amplitude.get();
        let f = frequency.get();

        let x = (t * f).sin() * a;
        let y = (t * f * 1.5).cos() * a;
        let rotation = t * 180.0 / std::f64::consts::PI;

        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!(
                "translate({}px, {}px) rotate({}deg)",
                x, y, rotation
            )),
        );
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(0.5 + 0.5 * (t * 2.0).sin()),
        );
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            animate_memo=Some(memoized_wave)
        >
            "Memoized wave animation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_conditional_animation() {
    // RED PHASE: Test should fail initially
    let (is_active, set_active) = signal(false);
    let (intensity, set_intensity) = signal(1.0);

    // Create a memoized conditional animation
    let memoized_conditional = create_memo(move |_| {
        let active = is_active.get();
        let intensity = intensity.get();

        if active {
            let mut target = HashMap::new();
            target.insert(
                "transform".to_string(),
                AnimationValue::String(format!("scale({})", 1.0 + intensity * 0.5)),
            );
            target.insert(
                "background-color".to_string(),
                AnimationValue::String(format!("rgba(255, 0, 0, {})", intensity)),
            );
            target
        } else {
            let mut target = HashMap::new();
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(1.0)".to_string()),
            );
            target.insert(
                "background-color".to_string(),
                AnimationValue::String("rgba(0, 0, 0, 0.1)".to_string()),
            );
            target
        }
    });

    let component = view! {
        <ReactiveMotionDiv
            animate_memo=Some(memoized_conditional)
        >
            "Memoized conditional animation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_performance_optimization() {
    // RED PHASE: Test should fail initially
    let (counter, set_counter) = signal(0);
    let (multiplier, set_multiplier) = signal(1.0);

    // Create a memoized expensive calculation
    let memoized_expensive = create_memo(move |_| {
        let count = counter.get();
        let mult = multiplier.get();

        // Simulate expensive calculation
        let mut result = 0.0;
        for i in 0..1000 {
            result += (count as f64 + i as f64) * mult;
        }

        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("translateX({}px)", result % 100.0)),
        );
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number((result % 1.0).abs()),
        );
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            animate_memo=Some(memoized_expensive)
        >
            "Memoized expensive calculation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_memoized_with_signal_based_controller() {
    // RED PHASE: Test should fail initially
    let controller = SignalBasedAnimationController::new(HashMap::new());
    let (animation_state, set_animation_state) = signal("idle".to_string());

    // Create a memoized animation based on controller state
    let memoized_controller_animation = create_memo(move |_| {
        let state = animation_state.get();

        match state.as_str() {
            "playing" => {
                let mut target = HashMap::new();
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(1.1)".to_string()),
                );
                target
            }
            "paused" => {
                let mut target = HashMap::new();
                target.insert("opacity".to_string(), AnimationValue::Number(0.7));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(0.9)".to_string()),
                );
                target
            }
            _ => {
                let mut target = HashMap::new();
                target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(1.0)".to_string()),
                );
                target
            }
        }
    });

    let component = view! {
        <ReactiveMotionDiv
            animate_memo=Some(memoized_controller_animation)
        >
            "Memoized controller animation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}
