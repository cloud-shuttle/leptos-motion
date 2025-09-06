//! Simple WASM test setup verification
//! 
//! This module contains basic tests to verify that the WASM testing environment
//! is working correctly.

// WASM-specific test configuration - conditional compilation
#[cfg(feature = "web-sys")]
use wasm_bindgen_test::*;

#[cfg(feature = "web-sys")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test that basic WASM environment is working
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_wasm_environment_basic() {
    // Test that we can create basic types
    let handle = crate::AnimationHandle(123);
    assert_eq!(handle.0, 123);
    
    // Test that we can create animation values
    let value = crate::AnimationValue::Number(42.0);
    match value {
        crate::AnimationValue::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected Number variant"),
    }
}

/// Test that we can create a minimal engine
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_minimal_engine_creation() {
    let engine = crate::MinimalEngine::new();
    assert_eq!(engine.active_count(), 0);
}

/// Test that we can create animation targets
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_animation_target_creation() {
    use std::collections::HashMap;
    
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), crate::AnimationValue::Number(1.0));
    target.insert("scale".to_string(), crate::AnimationValue::Number(1.2));
    
    assert_eq!(target.len(), 2);
    assert!(target.contains_key("opacity"));
    assert!(target.contains_key("scale"));
}

/// Test that we can create transitions
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_transition_creation() {
    let transition = crate::Transition {
        duration: Some(1.0),
        ease: crate::Easing::Linear,
        delay: Some(0.1),
        ..Default::default()
    };
    
    assert_eq!(transition.duration, Some(1.0));
    assert_eq!(transition.delay, Some(0.1));
}
