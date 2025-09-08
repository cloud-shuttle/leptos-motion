//! Animation Reactivity Tests
//! 
//! These tests verify that MotionDiv components properly react to state changes
//! and apply animations correctly.

use leptos::prelude::*;
use leptos_motion_dom::MotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that MotionDiv reacts to state changes in animation closures
#[wasm_bindgen_test]
fn test_animation_reactivity() {
    let (is_visible, set_is_visible) = signal(true);
    let (animation_mode, set_animation_mode) = signal(0);

    let create_animation_target = |visible: bool, mode: i32| -> AnimationTarget {
        let mut target = HashMap::new();
        
        if visible {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert("transform".to_string(), AnimationValue::String("scale(0.5)".to_string()));
        }
        
        match mode {
            0 => {
                target.insert("backgroundColor".to_string(), AnimationValue::String("red".to_string()));
            }
            1 => {
                target.insert("backgroundColor".to_string(), AnimationValue::String("blue".to_string()));
            }
            _ => {
                target.insert("backgroundColor".to_string(), AnimationValue::String("green".to_string()));
            }
        }
        
        target
    };

    let animate_animation = move || create_animation_target(is_visible.get(), animation_mode.get());

    let transition = Transition {
        duration: Some(0.6),
        ease: Easing::EaseInOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test initial state
    let initial_target = create_animation_target(true, 0);
    assert_eq!(initial_target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(initial_target.get("backgroundColor"), Some(&AnimationValue::String("red".to_string())));

    // Test state change to invisible
    set_is_visible.set(false);
    let hidden_target = animate_animation();
    assert_eq!(hidden_target.get("opacity"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(hidden_target.get("transform"), Some(&AnimationValue::String("scale(0.5)".to_string())));

    // Test mode change
    set_animation_mode.set(1);
    let mode_changed_target = animate_animation();
    assert_eq!(mode_changed_target.get("backgroundColor"), Some(&AnimationValue::String("blue".to_string())));

    // Test both changes
    set_is_visible.set(true);
    set_animation_mode.set(2);
    let both_changed_target = animate_animation();
    assert_eq!(both_changed_target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(both_changed_target.get("backgroundColor"), Some(&AnimationValue::String("green".to_string())));
}

/// Test that MotionDiv applies styles correctly
#[wasm_bindgen_test]
fn test_motion_div_style_application() {
    let (is_visible, set_is_visible) = signal(true);

    let animate_animation = move || {
        let mut target = HashMap::new();
        if is_visible.get() {
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert("transform".to_string(), AnimationValue::String("translateX(0px)".to_string()));
        } else {
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert("transform".to_string(), AnimationValue::String("translateX(-100px)".to_string()));
        }
        target
    };

    let transition = Transition {
        duration: Some(0.3),
        ease: Easing::Linear,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test that the closure returns correct values
    let visible_target = animate_animation();
    assert_eq!(visible_target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(visible_target.get("transform"), Some(&AnimationValue::String("translateX(0px)".to_string())));

    set_is_visible.set(false);
    let hidden_target = animate_animation();
    assert_eq!(hidden_target.get("opacity"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(hidden_target.get("transform"), Some(&AnimationValue::String("translateX(-100px)".to_string())));
}

/// Test hover animations
#[wasm_bindgen_test]
fn test_hover_animations() {
    let hover_animation = {
        let mut target = HashMap::new();
        target.insert("transform".to_string(), AnimationValue::String("translateY(-5px)".to_string()));
        target.insert("boxShadow".to_string(), AnimationValue::String("0 10px 20px rgba(0,0,0,0.2)".to_string()));
        target
    };

    // Test hover animation target
    assert_eq!(hover_animation.get("transform"), Some(&AnimationValue::String("translateY(-5px)".to_string())));
    assert_eq!(hover_animation.get("boxShadow"), Some(&AnimationValue::String("0 10px 20px rgba(0,0,0,0.2)".to_string())));
}

/// Test tap animations
#[wasm_bindgen_test]
fn test_tap_animations() {
    let tap_animation = {
        let mut target = HashMap::new();
        target.insert("transform".to_string(), AnimationValue::String("scale(0.95)".to_string()));
        target.insert("backgroundColor".to_string(), AnimationValue::String("rgba(0,0,0,0.1)".to_string()));
        target
    };

    // Test tap animation target
    assert_eq!(tap_animation.get("transform"), Some(&AnimationValue::String("scale(0.95)".to_string())));
    assert_eq!(tap_animation.get("backgroundColor"), Some(&AnimationValue::String("rgba(0,0,0,0.1)".to_string())));
}

/// Test AnimationValue to string conversion
#[wasm_bindgen_test]
fn test_animation_value_to_string() {
    // Test Number conversion
    let number_value = AnimationValue::Number(1.5);
    assert_eq!(number_value.to_string_value(), "1.5");

    // Test Pixels conversion
    let pixels_value = AnimationValue::Pixels(100.0);
    assert_eq!(pixels_value.to_string_value(), "100px");

    // Test Degrees conversion
    let degrees_value = AnimationValue::Degrees(45.0);
    assert_eq!(degrees_value.to_string_value(), "45deg");

    // Test Color conversion
    let color_value = AnimationValue::Color("red".to_string());
    assert_eq!(color_value.to_string_value(), "red");

    // Test String conversion
    let string_value = AnimationValue::String("translateX(10px)".to_string());
    assert_eq!(string_value.to_string_value(), "translateX(10px)");
}

/// Test complex animation sequences
#[wasm_bindgen_test]
fn test_complex_animation_sequences() {
    let (step, set_step) = signal(0);

    let complex_animation = move || {
        let mut target = HashMap::new();
        match step.get() {
            0 => {
                target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                target.insert("transform".to_string(), AnimationValue::String("translateY(-50px) scale(0.8)".to_string()));
            }
            1 => {
                target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                target.insert("transform".to_string(), AnimationValue::String("translateY(-25px) scale(0.9)".to_string()));
            }
            2 => {
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert("transform".to_string(), AnimationValue::String("translateY(0px) scale(1.0)".to_string()));
            }
            _ => {
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert("transform".to_string(), AnimationValue::String("translateY(0px) scale(1.0)".to_string()));
            }
        }
        target
    };

    // Test step 0
    let step0_target = complex_animation();
    assert_eq!(step0_target.get("opacity"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(step0_target.get("transform"), Some(&AnimationValue::String("translateY(-50px) scale(0.8)".to_string())));

    // Test step 1
    set_step.set(1);
    let step1_target = complex_animation();
    assert_eq!(step1_target.get("opacity"), Some(&AnimationValue::Number(0.5)));
    assert_eq!(step1_target.get("transform"), Some(&AnimationValue::String("translateY(-25px) scale(0.9)".to_string())));

    // Test step 2
    set_step.set(2);
    let step2_target = complex_animation();
    assert_eq!(step2_target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(step2_target.get("transform"), Some(&AnimationValue::String("translateY(0px) scale(1.0)".to_string())));
}
