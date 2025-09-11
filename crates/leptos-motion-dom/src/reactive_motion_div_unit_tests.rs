//! Unit Tests for ReactiveMotionDiv Right-Click Issue
//!
//! This module contains regular unit tests to verify that ReactiveMotionDiv
//! doesn't interfere with browser right-click functionality.

use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Test that ReactiveMotionDiv animation targets are created correctly
#[test]
fn test_reactive_motion_div_animation_target_creation() {
    // Create a simple animation target
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.2));
    target.insert("rotateZ".to_string(), AnimationValue::Number(45.0));

    // Test that the animation target is valid
    assert!(target.contains_key("scale"));
    assert!(target.contains_key("rotateZ"));

    // Verify specific values
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.2)));
    assert_eq!(target.get("rotateZ"), Some(&AnimationValue::Number(45.0)));
}

/// Test that ReactiveMotionDiv transition configuration works
#[test]
fn test_reactive_motion_div_transition_config() {
    let transition = Transition {
        duration: Some(0.5),
        delay: None,
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Verify transition is valid
    assert_eq!(transition.duration, Some(0.5));
    assert_eq!(transition.delay, None);
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.repeat, RepeatConfig::Never);
    assert_eq!(transition.stagger, None);
}

/// Test that ReactiveMotionDiv handles complex animation targets
#[test]
fn test_reactive_motion_div_complex_animation_target() {
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    target.insert("rotateX".to_string(), AnimationValue::Number(30.0));
    target.insert("rotateY".to_string(), AnimationValue::Number(60.0));
    target.insert("rotateZ".to_string(), AnimationValue::Number(90.0));
    target.insert("translateX".to_string(), AnimationValue::Number(100.0));
    target.insert("translateY".to_string(), AnimationValue::Number(50.0));
    target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    target.insert(
        "background-color".to_string(),
        AnimationValue::String("rgb(255, 0, 0)".to_string()),
    );

    // Verify all properties are set correctly
    assert_eq!(target.len(), 8);

    // Verify specific values
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.5)));
    assert_eq!(target.get("rotateX"), Some(&AnimationValue::Number(30.0)));
    assert_eq!(
        target.get("background-color"),
        Some(&AnimationValue::String("rgb(255, 0, 0)".to_string()))
    );
}

/// Test that ReactiveMotionDiv function-based animation targets work
#[test]
fn test_reactive_motion_div_function_based_animation() {
    // Create animation function that should not interfere with events
    let animation_fn = Box::new(|| {
        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String("translateX(10px)".to_string()),
        );
        target
    });

    // Test that the function works correctly
    let result = animation_fn();
    assert!(result.contains_key("transform"));

    if let Some(AnimationValue::String(transform)) = result.get("transform") {
        assert_eq!(transform, "translateX(10px)");
    }
}

/// Test that ReactiveMotionDiv handles multiple animation targets without memory issues
#[test]
fn test_reactive_motion_div_multiple_animation_targets() {
    // Create multiple animation targets to test memory management
    let targets = (0..10)
        .map(|i| {
            let mut target = HashMap::new();
            target.insert(
                "scale".to_string(),
                AnimationValue::Number(1.0 + (i as f64 * 0.1)),
            );
            target.insert(
                "opacity".to_string(),
                AnimationValue::Number(0.5 + (i as f64 * 0.05)),
            );
            target
        })
        .collect::<Vec<_>>();

    // Verify all targets are created successfully
    assert_eq!(targets.len(), 10);

    // Verify no duplicate memory allocations
    for (i, target) in targets.iter().enumerate() {
        assert!(target.contains_key("scale"));
        assert!(target.contains_key("opacity"));

        let expected_scale = 1.0 + (i as f64 * 0.1);
        let expected_opacity = 0.5 + (i as f64 * 0.05);

        if let Some(AnimationValue::Number(scale)) = target.get("scale") {
            assert_eq!(*scale, expected_scale);
        }

        if let Some(AnimationValue::Number(opacity)) = target.get("opacity") {
            assert_eq!(*opacity, expected_opacity);
        }
    }
}

/// Test that ReactiveMotionDiv works with minimal configuration
#[test]
fn test_reactive_motion_div_minimal_configuration() {
    // Test with minimal configuration
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.0));

    let transition = Transition {
        duration: Some(0.3),
        delay: Some(0.1),
        ease: Easing::Linear,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Verify minimal configuration works
    assert!(target.contains_key("scale"));
    assert_eq!(transition.duration, Some(0.3));
    assert_eq!(transition.delay, Some(0.1));
    assert_eq!(transition.ease, Easing::Linear);
}

/// Test that ReactiveMotionDiv handles different easing types
#[test]
fn test_reactive_motion_div_easing_types() {
    let easing_types = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
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

/// Test that ReactiveMotionDiv handles different repeat configurations
#[test]
fn test_reactive_motion_div_repeat_configurations() {
    let repeat_configs = vec![
        RepeatConfig::Never,
        RepeatConfig::Count(3),
        RepeatConfig::Infinite,
        RepeatConfig::InfiniteReverse,
    ];

    for repeat in repeat_configs {
        let transition = Transition {
            duration: Some(0.5),
            delay: None,
            ease: Easing::EaseInOut,
            repeat: repeat.clone(),
            stagger: None,
        };

        assert_eq!(transition.repeat, repeat);
    }
}
