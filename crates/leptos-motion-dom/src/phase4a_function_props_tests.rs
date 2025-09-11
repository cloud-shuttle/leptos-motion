//! Phase 4A: Function-based Props Tests
//!
//! Tests for function-based animation props using Box<dyn Fn() + Send + Sync>

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

/// Helper function to create a function-based animation target
fn create_function_animation_target() -> Box<dyn Fn() -> AnimationTarget + Send + Sync> {
    Box::new(|| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(1.2)".to_string()),
        );
        target
    })
}

/// Helper function to create a dynamic animation target based on time
fn create_dynamic_animation_target() -> Box<dyn Fn() -> AnimationTarget + Send + Sync> {
    Box::new(|| {
        let time = js_sys::Date::now() / 1000.0; // Current time in seconds
        let opacity = 0.5 + 0.3 * (time.sin());
        let scale = 1.0 + 0.2 * (time.cos());

        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(opacity));
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("scale({})", scale)),
        );
        target
    })
}

#[wasm_bindgen_test]
fn test_function_based_animate_prop() {
    // RED PHASE: Test should fail initially
    let function_target = create_function_animation_target();

    let component = view! {
        <ReactiveMotionDiv
            animate_fn=function_target
        >
            "Function-based animate test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_function_based_while_hover_prop() {
    // RED PHASE: Test should fail initially
    let hover_function = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "background-color".to_string(),
            AnimationValue::String("red".to_string()),
        );
        target.insert(
            "transform".to_string(),
            AnimationValue::String("rotate(5deg)".to_string()),
        );
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            while_hover_fn=hover_function
        >
            "Function-based hover test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_function_based_while_tap_prop() {
    // RED PHASE: Test should fail initially
    let tap_function = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("scale(0.95)".to_string()),
        );
        target.insert(
            "box-shadow".to_string(),
            AnimationValue::String("0 4px 8px rgba(0,0,0,0.3)".to_string()),
        );
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            while_tap_fn=tap_function
        >
            "Function-based tap test"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_dynamic_function_based_animation() {
    // RED PHASE: Test should fail initially
    let dynamic_function = create_dynamic_animation_target();

    let component = view! {
        <ReactiveMotionDiv
            animate_fn=dynamic_function
        >
            "Dynamic function-based animation"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_mixed_static_and_function_props() {
    // RED PHASE: Test should fail initially
    let static_initial = create_animation_target("opacity", 0.0);
    let function_animate = create_function_animation_target();
    let function_hover = Box::new(|| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target
    });

    let component = view! {
        <ReactiveMotionDiv
            initial=static_initial
            animate_fn=function_animate
            while_hover_fn=function_hover
        >
            "Mixed static and function props"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_function_props_with_signal_based_controller() {
    // RED PHASE: Test should fail initially
    let controller = SignalBasedAnimationController::new(HashMap::new());
    let function_animate = create_function_animation_target();

    let component = view! {
        <ReactiveMotionDiv
            animate_fn=function_animate
        >
            "Function props with controller"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}

#[wasm_bindgen_test]
fn test_function_props_thread_safety() {
    // RED PHASE: Test should fail initially
    // This test ensures the function props are Send + Sync
    let function_target = create_function_animation_target();

    // Simulate moving the function to another thread (conceptually)
    let moved_function = function_target;

    let component = view! {
        <ReactiveMotionDiv
            animate_fn=moved_function
        >
            "Thread-safe function props"
        </ReactiveMotionDiv>
    };

    // Verify the component compiles and mounts
    let _view = component.into_view();
}
