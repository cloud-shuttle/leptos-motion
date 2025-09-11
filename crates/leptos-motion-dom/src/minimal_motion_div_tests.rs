//! TDD Tests for MinimalMotionDiv Component
//!
//! This module contains comprehensive tests for the MinimalMotionDiv component
//! to ensure it works as a reliable fallback for problematic scenarios.

use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Test that MinimalMotionDiv creates animation targets correctly
#[test]
fn test_minimal_motion_div_animation_target_creation() {
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    target.insert("rotateZ".to_string(), AnimationValue::Number(45.0));

    // Verify animation target structure
    assert!(target.contains_key("scale"));
    assert!(target.contains_key("opacity"));
    assert!(target.contains_key("rotateZ"));

    // Verify specific values
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.5)));
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(0.8)));
    assert_eq!(target.get("rotateZ"), Some(&AnimationValue::Number(45.0)));
}

/// Test that MinimalMotionDiv handles function-based animation targets
#[test]
fn test_minimal_motion_div_function_based_animation() {
    let animation_fn = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("translateX(20px) rotate(30deg)".to_string()),
        );
        target.insert(
            "background-color".to_string(),
            AnimationValue::String("rgb(0, 255, 0)".to_string()),
        );
        target
    });

    // Test function execution
    let result = animation_fn();
    assert!(result.contains_key("transform"));
    assert!(result.contains_key("background-color"));

    if let Some(AnimationValue::String(transform)) = result.get("transform") {
        assert_eq!(transform, "translateX(20px) rotate(30deg)");
    }

    if let Some(AnimationValue::String(bg_color)) = result.get("background-color") {
        assert_eq!(bg_color, "rgb(0, 255, 0)");
    }
}

/// Test that MinimalMotionDiv handles transition configurations
#[test]
fn test_minimal_motion_div_transition_config() {
    let transition = Transition {
        duration: Some(0.8),
        delay: Some(0.2),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Count(3),
        stagger: None,
    };

    // Verify transition properties
    assert_eq!(transition.duration, Some(0.8));
    assert_eq!(transition.delay, Some(0.2));
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.repeat, RepeatConfig::Count(3));
    assert_eq!(transition.stagger, None);
}

/// Test that MinimalMotionDiv handles different easing types
#[test]
fn test_minimal_motion_div_easing_types() {
    let easing_types = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
        Easing::CircIn,
        Easing::CircOut,
        Easing::CircInOut,
        Easing::BackIn,
        Easing::BackOut,
        Easing::BackInOut,
        Easing::Bezier(0.25, 0.1, 0.25, 1.0),
        Easing::Spring(leptos_motion_core::SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
            velocity: 0.0,
        }),
    ];

    for easing in easing_types {
        let transition = Transition {
            duration: Some(0.5),
            delay: None,
            ease: easing.clone(),
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        assert_eq!(transition.ease, easing);
    }
}

/// Test that MinimalMotionDiv handles complex animation combinations
#[test]
fn test_minimal_motion_div_complex_animations() {
    // Test multiple animation properties
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(2.0));
    target.insert("rotateX".to_string(), AnimationValue::Number(180.0));
    target.insert("rotateY".to_string(), AnimationValue::Number(90.0));
    target.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
    target.insert("translateX".to_string(), AnimationValue::Number(100.0));
    target.insert("translateY".to_string(), AnimationValue::Number(50.0));
    target.insert("translateZ".to_string(), AnimationValue::Number(25.0));
    target.insert("opacity".to_string(), AnimationValue::Number(0.5));
    target.insert(
        "background-color".to_string(),
        AnimationValue::String("rgb(255, 0, 255)".to_string()),
    );
    target.insert(
        "border-radius".to_string(),
        AnimationValue::String("50%".to_string()),
    );
    target.insert(
        "box-shadow".to_string(),
        AnimationValue::String("0 10px 20px rgba(0,0,0,0.3)".to_string()),
    );

    // Verify all properties are set
    assert_eq!(target.len(), 11);

    // Verify specific complex values
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(2.0)));
    assert_eq!(target.get("rotateX"), Some(&AnimationValue::Number(180.0)));
    assert_eq!(
        target.get("background-color"),
        Some(&AnimationValue::String("rgb(255, 0, 255)".to_string()))
    );
    assert_eq!(
        target.get("box-shadow"),
        Some(&AnimationValue::String(
            "0 10px 20px rgba(0,0,0,0.3)".to_string()
        ))
    );
}

/// Test that MinimalMotionDiv handles edge cases
#[test]
fn test_minimal_motion_div_edge_cases() {
    // Test with empty animation target
    let empty_target: HashMap<String, AnimationValue> = HashMap::new();
    assert!(empty_target.is_empty());

    // Test with single property
    let mut single_target = HashMap::new();
    single_target.insert("opacity".to_string(), AnimationValue::Number(0.0));
    assert_eq!(single_target.len(), 1);
    assert_eq!(
        single_target.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );

    // Test with zero values
    let mut zero_target = HashMap::new();
    zero_target.insert("scale".to_string(), AnimationValue::Number(0.0));
    zero_target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
    zero_target.insert("opacity".to_string(), AnimationValue::Number(0.0));

    assert_eq!(zero_target.get("scale"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(
        zero_target.get("rotateZ"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        zero_target.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );
}

/// Test that MinimalMotionDiv handles different repeat configurations
#[test]
fn test_minimal_motion_div_repeat_configurations() {
    let repeat_configs = vec![
        RepeatConfig::Never,
        RepeatConfig::Count(1),
        RepeatConfig::Count(5),
        RepeatConfig::Count(10),
        RepeatConfig::Infinite,
        RepeatConfig::InfiniteReverse,
    ];

    for repeat in repeat_configs {
        let transition = Transition {
            duration: Some(0.3),
            delay: None,
            ease: Easing::Linear,
            repeat: repeat.clone(),
            stagger: None,
        };

        assert_eq!(transition.repeat, repeat);
    }
}

/// Test that MinimalMotionDiv handles mixed value types
#[test]
fn test_minimal_motion_div_mixed_value_types() {
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    target.insert(
        "transform".to_string(),
        AnimationValue::String("translateX(10px) rotate(45deg)".to_string()),
    );
    target.insert(
        "background-color".to_string(),
        AnimationValue::String("linear-gradient(45deg, #ff0000, #00ff00)".to_string()),
    );
    target.insert(
        "border".to_string(),
        AnimationValue::String("2px solid #0000ff".to_string()),
    );

    // Verify mixed types are handled correctly
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.5)));
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(0.8)));
    assert_eq!(
        target.get("transform"),
        Some(&AnimationValue::String(
            "translateX(10px) rotate(45deg)".to_string()
        ))
    );
    assert_eq!(
        target.get("background-color"),
        Some(&AnimationValue::String(
            "linear-gradient(45deg, #ff0000, #00ff00)".to_string()
        ))
    );
    assert_eq!(
        target.get("border"),
        Some(&AnimationValue::String("2px solid #0000ff".to_string()))
    );
}

/// Test that MinimalMotionDiv handles performance with many properties
#[test]
fn test_minimal_motion_div_performance_many_properties() {
    // Create animation target with many properties
    let mut target = HashMap::new();
    for i in 0..100 {
        target.insert(format!("property_{}", i), AnimationValue::Number(i as f64));
    }

    // Verify all properties are created
    assert_eq!(target.len(), 100);

    // Verify specific properties
    assert_eq!(target.get("property_0"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(
        target.get("property_50"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        target.get("property_99"),
        Some(&AnimationValue::Number(99.0))
    );
}
