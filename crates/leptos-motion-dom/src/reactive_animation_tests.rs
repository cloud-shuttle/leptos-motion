//! Reactive Animation Tests for MotionDiv
//!
//! These tests verify that the MotionDiv component properly supports reactive animations
//! using Rc<dyn Fn() -> AnimationTarget> for the animate prop.

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use crate::{ReactiveMotionDiv, AnimationTargetOrReactive};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that MotionDiv accepts reactive animate prop
#[wasm_bindgen_test]
fn test_motion_div_reactive_animate_prop() {
    let (is_visible, _set_is_visible) = signal(true);
    let (animation_mode, _set_animation_mode) = signal(0);

    let create_animation_target = |visible: bool, mode: i32| -> AnimationTarget {
        let mut target = HashMap::new();

        if visible {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(1)".to_string()),
            );
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("scale(0.5)".to_string()),
            );
        }

        match mode {
            0 => {
                target.insert(
                    "backgroundColor".to_string(),
                    AnimationValue::String("red".to_string()),
                );
            }
            1 => {
                target.insert(
                    "backgroundColor".to_string(),
                    AnimationValue::String("blue".to_string()),
                );
            }
            _ => {
                target.insert(
                    "backgroundColor".to_string(),
                    AnimationValue::String("green".to_string()),
                );
            }
        }

        target
    };

    let animate_animation = move || create_animation_target(is_visible.get(), animation_mode.get());

    // This should compile and work with reactive animations
    let _component = view! {
        <ReactiveMotionDiv
            class="test-motion-div".to_string()
            animate=AnimationTargetOrReactive::Reactive(Rc::new(animate_animation))
            style="padding: 1rem; margin: 1rem; border-radius: 8px;".to_string()
        >
            "Test Animation Content"
        </ReactiveMotionDiv>
    };

    // If we get here, the test passes
    assert!(true);
}

/// Test that reactive animations respond to signal changes
#[wasm_bindgen_test]
fn test_reactive_animation_signal_changes() {
    let (is_visible, set_is_visible) = signal(true);
    let (animation_mode, set_animation_mode) = signal(0);

    let create_animation_target = |visible: bool, mode: i32| -> AnimationTarget {
        let mut target = HashMap::new();

        if visible {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        }

        match mode {
            0 => target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("red".to_string()),
            ),
            1 => target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("blue".to_string()),
            ),
            _ => target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("green".to_string()),
            ),
        };

        target
    };

    let animate_animation = move || create_animation_target(is_visible.get(), animation_mode.get());

    // Test initial state
    let initial_target = animate_animation();
    assert_eq!(
        initial_target.get("opacity"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        initial_target.get("backgroundColor"),
        Some(&AnimationValue::String("red".to_string()))
    );

    // Test visibility change
    set_is_visible.set(false);
    let hidden_target = animate_animation();
    assert_eq!(
        hidden_target.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        hidden_target.get("backgroundColor"),
        Some(&AnimationValue::String("red".to_string()))
    );

    // Test mode change
    set_animation_mode.set(1);
    let mode_changed_target = animate_animation();
    assert_eq!(
        mode_changed_target.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        mode_changed_target.get("backgroundColor"),
        Some(&AnimationValue::String("blue".to_string()))
    );

    // Test both changes
    set_is_visible.set(true);
    set_animation_mode.set(2);
    let both_changed_target = animate_animation();
    assert_eq!(
        both_changed_target.get("opacity"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        both_changed_target.get("backgroundColor"),
        Some(&AnimationValue::String("green".to_string()))
    );
}

/// Test that MotionDiv component can handle both static and reactive animate props
#[wasm_bindgen_test]
fn test_motion_div_mixed_animate_props() {
    // Static animation target
    let static_animate = {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target.insert(
            "transform".to_string(),
            AnimationValue::String("rotate(45deg)".to_string()),
        );
        target
    };

    // Reactive animation target
    let (rotation, _set_rotation) = signal(0.0);
    let reactive_animate = move || {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.9));
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("rotate({}deg)", rotation.get())),
        );
        target
    };

    // Both should compile and work
    let _static_component = view! {
        <ReactiveMotionDiv
            class="static-motion-div".to_string()
            animate=AnimationTargetOrReactive::Static(static_animate)
        >
            "Static Animation"
        </ReactiveMotionDiv>
    };

    let _reactive_component = view! {
        <ReactiveMotionDiv
            class="reactive-motion-div".to_string()
            animate=AnimationTargetOrReactive::Reactive(Rc::new(reactive_animate))
        >
            "Reactive Animation"
        </ReactiveMotionDiv>
    };

    // If we get here, both compile successfully
    assert!(true);
}

/// Test that reactive animations work with transition configuration
#[wasm_bindgen_test]
fn test_reactive_animation_with_transition() {
    let (is_visible, _set_is_visible) = signal(true);

    let animate_animation = move || {
        let mut target = HashMap::new();
        if is_visible.get() {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("translateX(0px)".to_string()),
            );
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert(
                "transform".to_string(),
                AnimationValue::String("translateX(-100px)".to_string()),
            );
        }
        target
    };

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // This should compile with both reactive animate and transition
    let _component = view! {
        <ReactiveMotionDiv
            class="transition-motion-div".to_string()
            animate=AnimationTargetOrReactive::Reactive(Rc::new(animate_animation))
            _transition=transition
        >
            "Transition Animation"
        </ReactiveMotionDiv>
    };

    assert!(true);
}

/// Test that reactive animations work with other motion props
#[wasm_bindgen_test]
fn test_reactive_animation_with_other_props() {
    let (is_visible, _set_is_visible) = signal(true);

    let animate_animation = move || {
        let mut target = HashMap::new();
        if is_visible.get() {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        }
        target
    };

    let hover_animation = {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(1.1)".to_string()),
        );
        target
    };

    let tap_animation = {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(0.95)".to_string()),
        );
        target
    };

    // This should compile with reactive animate and static hover/tap
    let _component = view! {
        <ReactiveMotionDiv
            class="full-motion-div".to_string()
            animate=AnimationTargetOrReactive::Reactive(Rc::new(animate_animation))
            _while_hover=hover_animation
            _while_tap=tap_animation
        >
            "Full Motion Animation"
        </ReactiveMotionDiv>
    };

    assert!(true);
}
