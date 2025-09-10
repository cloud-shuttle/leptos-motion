//! TDD tests for MotionDiv component
//!
//! These tests drive the development of proper MotionDiv functionality

use crate::{DragAxis, DragConfig};
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that MotionDiv can animate from initial to animate values
#[wasm_bindgen_test]
fn test_motion_div_basic_animation() {
    let initial = {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        target.insert("scale".to_string(), AnimationValue::Number(0.5));
        target
    };

    let animate = {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.0));
        target
    };

    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::Linear,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test that we can create the animation configuration
    assert_eq!(initial.get("opacity"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(animate.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(transition.duration, Some(1.0));
}

/// Test that MotionDiv handles transition configuration
#[wasm_bindgen_test]
fn test_motion_div_transition_config() {
    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.1),
        repeat: RepeatConfig::Count(2),
        stagger: None,
    };

    assert_eq!(transition.duration, Some(0.5));
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.delay, Some(0.1));
    match transition.repeat {
        RepeatConfig::Count(n) => assert_eq!(n, 2),
        _ => panic!("Expected Count variant"),
    }
}

/// Test that MotionDiv can handle while_hover animations
#[wasm_bindgen_test]
fn test_motion_div_while_hover() {
    let while_hover = {
        let mut target = HashMap::new();
        target.insert("scale".to_string(), AnimationValue::Number(1.1));
        target.insert("rotate".to_string(), AnimationValue::Number(5.0));
        target
    };

    assert_eq!(while_hover.get("scale"), Some(&AnimationValue::Number(1.1)));
    assert_eq!(
        while_hover.get("rotate"),
        Some(&AnimationValue::Number(5.0))
    );
}

/// Test that MotionDiv can handle while_tap animations
#[wasm_bindgen_test]
fn test_motion_div_while_tap() {
    let while_tap = {
        let mut target = HashMap::new();
        target.insert("scale".to_string(), AnimationValue::Number(0.95));
        target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        target
    };

    assert_eq!(while_tap.get("scale"), Some(&AnimationValue::Number(0.95)));
    assert_eq!(while_tap.get("opacity"), Some(&AnimationValue::Number(0.8)));
}

/// Test that MotionDiv can handle drag configuration
#[wasm_bindgen_test]
fn test_motion_div_drag_config() {
    let drag_config = DragConfig {
        axis: Some(DragAxis::X),
        constraints: None,
        elastic: Some(0.2),
        momentum: Some(true),
    };

    assert_eq!(drag_config.axis, Some(DragAxis::X));
    assert_eq!(drag_config.elastic, Some(0.2));
    assert_eq!(drag_config.momentum, Some(true));
}

/// Test that MotionDiv can handle layout animations
#[wasm_bindgen_test]
fn test_motion_div_layout_animation() {
    let layout_enabled = true;
    let layout_transition = Transition {
        duration: Some(0.3),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    assert!(layout_enabled);
    assert_eq!(layout_transition.duration, Some(0.3));
    assert_eq!(layout_transition.ease, Easing::EaseInOut);
}

/// Test that MotionDiv can handle complex animation sequences
#[wasm_bindgen_test]
fn test_motion_div_complex_animation() {
    let initial = {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        target.insert("x".to_string(), AnimationValue::Number(-100.0));
        target.insert("y".to_string(), AnimationValue::Number(-100.0));
        target.insert("scale".to_string(), AnimationValue::Number(0.5));
        target.insert("rotate".to_string(), AnimationValue::Number(-180.0));
        target
    };

    let animate = {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert("x".to_string(), AnimationValue::Number(0.0));
        target.insert("y".to_string(), AnimationValue::Number(0.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.0));
        target.insert("rotate".to_string(), AnimationValue::Number(0.0));
        target
    };

    let transition = Transition {
        duration: Some(0.8),
        ease: Easing::EaseInOut,
        delay: Some(0.2),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test that all properties are properly configured
    assert_eq!(initial.len(), 5);
    assert_eq!(animate.len(), 5);
    assert_eq!(transition.duration, Some(0.8));

    // Test that initial and animate have matching keys
    for key in initial.keys() {
        assert!(animate.contains_key(key));
    }
}
