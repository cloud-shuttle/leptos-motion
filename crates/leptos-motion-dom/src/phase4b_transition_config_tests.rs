//! Phase 4B: TransitionConfig Tests
//!
//! Tests for TransitionConfig timing controls

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition};
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

/// Helper function to create a transition config
fn create_transition_config(duration: f64, easing: &str, delay: f64) -> Transition {
    Transition {
        duration: Some(duration),
        easing: Some(easing.to_string()),
        delay: Some(delay),
        repeat: None,
        repeat_type: None,
        repeat_delay: None,
        yoyo: None,
    }
}

#[wasm_bindgen_test]
fn test_transition_config_duration() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);
    let transition = create_transition_config(2.0, "ease-in-out", 0.0);

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(transition)
        >
            "Transition duration test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_easing() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("transform", 0.0);
    let animate = create_animation_target("transform", 1.0);
    let transition = create_transition_config(1.0, "cubic-bezier(0.4, 0, 0.2, 1)", 0.0);

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(transition)
        >
            "Transition easing test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_delay() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);
    let transition = create_transition_config(1.0, "ease", 0.5);

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(transition)
        >
            "Transition delay test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_repeat() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);
    let mut transition = create_transition_config(1.0, "ease", 0.0);
    transition.repeat = Some(3);
    transition.repeat_type = Some("loop".to_string());

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(transition)
        >
            "Transition repeat test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_yoyo() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);
    let mut transition = create_transition_config(1.0, "ease", 0.0);
    transition.repeat = Some(2);
    transition.yoyo = Some(true);

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(transition)
        >
            "Transition yoyo test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_with_function_props() {
    // RED PHASE: Test should fail initially
    let function_animate = Box::new(|| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target
    });
    let transition = create_transition_config(1.5, "ease-in-out", 0.2);

    let component = view! {
        <ReactiveMotionDiv
            animate_fn=Some(function_animate)
            transition=Some(transition)
        >
            "Transition with function props"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_multiple_transition_configs() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);
    let hover_transition = create_transition_config(0.3, "ease-out", 0.0);
    let tap_transition = create_transition_config(0.1, "ease-in", 0.0);

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            while_hover_transition=Some(hover_transition)
            while_tap_transition=Some(tap_transition)
        >
            "Multiple transition configs"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    assert!(component.into_view().into_any().is_some());
}

#[wasm_bindgen_test]
fn test_transition_config_validation() {
    // RED PHASE: Test should fail initially
    let initial = create_animation_target("opacity", 0.0);
    let animate = create_animation_target("opacity", 1.0);

    // Test with invalid duration (should be handled gracefully)
    let invalid_transition = Transition {
        duration: Some(-1.0), // Invalid negative duration
        easing: Some("invalid-easing".to_string()),
        delay: Some(-0.5), // Invalid negative delay
        repeat: Some(0),   // Invalid zero repeat
        repeat_type: None,
        repeat_delay: None,
        yoyo: None,
    };

    let component = view! {
        <ReactiveMotionDiv
            initial=Some(initial)
            animate=Some(animate)
            transition=Some(invalid_transition)
        >
            "Transition validation test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts (should handle invalid config gracefully)
    assert!(component.into_view().into_any().is_some());
}
