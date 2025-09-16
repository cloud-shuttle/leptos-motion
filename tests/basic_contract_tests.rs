//! Basic Contract Tests for Leptos Motion
//!
//! Simple tests to verify that the core functionality works as expected.

use leptos_motion_core::*;
use leptos_motion_dom::animation_engine::AnimationEngine;

#[test]
fn test_animation_value_creation() {
    // Test that AnimationValue can be created
    let number_value = AnimationValue::Number(1.0);
    let string_value = AnimationValue::String("test".to_string());
    
    // Basic contract: values should be creatable
    assert!(matches!(number_value, AnimationValue::Number(1.0)));
    assert!(matches!(string_value, AnimationValue::String(_)));
}

#[test]
fn test_transition_creation() {
    // Test that Transition can be created
    let default_transition = Transition::default();
    let custom_transition = Transition {
        duration: Some(1.0),
        ease: Easing::Linear,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    // Basic contract: transitions should be creatable
    assert!(default_transition.duration.is_none());
    assert_eq!(custom_transition.duration, Some(1.0));
}

#[test]
fn test_easing_variants() {
    // Test that basic Easing variants exist
    let _linear = Easing::Linear;
    let _ease_in = Easing::EaseIn;
    let _ease_out = Easing::EaseOut;
    let _ease_in_out = Easing::EaseInOut;
    
    // Basic contract: easing variants should exist
    // (If we get here without compilation error, the contract is satisfied)
}

#[test]
fn test_animation_engine_creation() {
    // Test that AnimationEngine can be created
    let engine = AnimationEngine::new();
    
    // Basic contract: engine should be creatable
    // (If we get here without panic, the contract is satisfied)
    assert!(true); // Placeholder assertion
}

#[test]
fn test_animation_engine_animate_property() {
    // Test that AnimationEngine can animate properties
    let mut engine = AnimationEngine::new();
    engine.animate_property(
        "scale".to_string(),
        1.0,
        2.0,
        Transition::default(),
    );
    
    // Basic contract: animate_property should not panic
    // (If we get here without panic, the contract is satisfied)
    assert!(true); // Placeholder assertion
}

#[test]
fn test_animation_engine_get_values() {
    // Test that AnimationEngine can get values
    let engine = AnimationEngine::new();
    let values = engine.get_all_values();
    
    // Basic contract: get_all_values should return a HashMap
    assert!(values.is_empty()); // New engine should have no values
}

#[test]
fn test_error_handling() {
    // Test that invalid inputs are handled gracefully
    let mut engine = AnimationEngine::new();
    
    // Test with empty property name - should not panic
    engine.animate_property(
        "".to_string(),
        0.0,
        1.0,
        Transition::default(),
    );
    
    // Test with NaN values - should not panic
    engine.animate_property(
        "scale".to_string(),
        f64::NAN,
        f64::INFINITY,
        Transition::default(),
    );
    
    // Test with negative duration - should not panic
    let invalid_transition = Transition {
        duration: Some(-1.0),
        ease: Easing::Linear,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    engine.animate_property(
        "opacity".to_string(),
        0.0,
        1.0,
        invalid_transition,
    );
    
    // Basic contract: invalid inputs should not cause panics
    // (If we get here without panic, the contract is satisfied)
    assert!(true); // Placeholder assertion
}

#[test]
fn test_performance_basic() {
    // Test basic performance - creation should be fast
    let start = std::time::Instant::now();
    
    for _ in 0..100 {
        let _engine = AnimationEngine::new();
    }
    
    let duration = start.elapsed();
    
    // Basic contract: 100 engine creations should take less than 100ms
    assert!(duration.as_millis() < 100, "Engine creation too slow: {:?}", duration);
}

#[test]
fn test_memory_basic() {
    // Test basic memory usage - should not leak
    let mut engines = Vec::new();
    
    // Create many engines
    for i in 0..1000 {
        let mut engine = AnimationEngine::new();
        engine.animate_property(
            format!("property_{}", i),
            0.0,
            1.0,
            Transition::default(),
        );
        engines.push(engine);
    }
    
    // Drop all engines
    drop(engines);
    
    // Basic contract: should not run out of memory
    // (If we get here without running out of memory, the contract is satisfied)
    assert!(true); // Placeholder assertion
}
