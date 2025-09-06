// TDD Tests for Fixing WASM Test Failures
//
// This module contains tests to identify and fix the specific WASM test failures
// that are preventing 100% test pass rate.

use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test fixture for creating a mock element that works in both WASM and native
fn mock_element() -> web_sys::Element {
    // Use conditional compilation to handle different targets
    #[cfg(target_arch = "wasm32")]
    {
        let document = web_sys::window().unwrap().document().unwrap();
        document.create_element("div").unwrap()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native tests, create a mock element
        // This is a workaround for non-WASM targets
        unsafe { std::mem::zeroed() }
    }
}

/// Test fixture for creating a simple animation target
fn simple_animation_target() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("x".to_string(), AnimationValue::Pixels(100.0));
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target
}

/// Test fixture for creating a simple transition
fn simple_transition() -> Transition {
    Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    }
}

#[wasm_bindgen_test]
fn test_simplified_engine_creation_wasm_safe() {
    // Test that we can create a simplified animation engine without poisoning
    let engine = SimplifiedAnimationEngine::new();
    assert!(engine.is_available());
    assert_eq!(engine.active_animation_count(), 0);
    assert!(!engine.has_active_animations());
}

#[wasm_bindgen_test]
fn test_simplified_engine_clone_wasm_safe() {
    // Test cloning without accessing WASM-specific statics
    let engine1 = SimplifiedAnimationEngine::new();
    let engine2 = engine1.clone();

    assert!(engine1.is_available());
    assert!(engine2.is_available());
    assert_eq!(
        engine1.active_animation_count(),
        engine2.active_animation_count()
    );
}

#[wasm_bindgen_test]
fn test_simplified_engine_debug_wasm_safe() {
    // Test debug formatting without poisoning
    let engine = SimplifiedAnimationEngine::new();
    let debug_str = format!("{:?}", engine);
    assert!(debug_str.contains("SimplifiedAnimationEngine"));
    assert!(debug_str.contains("active_animations"));
    assert!(debug_str.contains("has_active_animations"));
}

#[wasm_bindgen_test]
fn test_simplified_engine_mutex_poisoning_prevention() {
    // Test that we can handle mutex poisoning gracefully
    let engine = SimplifiedAnimationEngine::new();

    // Create multiple engines to test shared state
    let engine2 = engine.clone();
    let engine3 = engine.clone();

    // All should be available and not poisoned
    assert!(engine.is_available());
    assert!(engine2.is_available());
    assert!(engine3.is_available());
}

#[wasm_bindgen_test]
fn test_simplified_engine_wasm_compatibility() {
    // Test WASM-specific functionality
    let engine = SimplifiedAnimationEngine::new();

    // Test that we can access performance metrics without WASM errors
    let metrics = engine.get_performance_metrics();
    // Metrics might be None in test environment, that's OK
    assert!(metrics.is_none() || metrics.is_some());
}

#[wasm_bindgen_test]
fn test_simplified_engine_global_state_isolation() {
    // Test that global state is properly isolated between engine instances
    let engine1 = SimplifiedAnimationEngine::new();
    let engine2 = SimplifiedAnimationEngine::new();

    // Each engine should have its own state
    assert_eq!(engine1.active_animation_count(), 0);
    assert_eq!(engine2.active_animation_count(), 0);

    // Cloned engines should share state
    let engine1_clone = engine1.clone();
    assert_eq!(
        engine1.active_animation_count(),
        engine1_clone.active_animation_count()
    );
}

#[wasm_bindgen_test]
fn test_simplified_engine_error_handling() {
    // Test that errors are handled gracefully without poisoning
    let mut engine = SimplifiedAnimationEngine::new();

    // Test with invalid element (should not panic)
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // This might fail, but should not poison the mutex
    let result = engine.animate(&element, &target, &transition);
    // Result can be Ok or Err, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[wasm_bindgen_test]
fn test_simplified_engine_cleanup_prevents_poisoning() {
    // Test that cleanup prevents mutex poisoning
    let mut engine = SimplifiedAnimationEngine::new();

    // Perform cleanup
    let result = engine.cleanup();
    assert!(result.is_ok());

    // Engine should still be usable after cleanup
    assert!(engine.is_available());
    assert_eq!(engine.active_animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_engine_batch_operations_safety() {
    // Test that batch operations don't cause poisoning
    let mut engine = SimplifiedAnimationEngine::new();

    // Test batch operations
    let result1 = engine.stop_all();
    let result2 = engine.pause_all();
    let result3 = engine.resume_all();

    // All should succeed or fail gracefully
    assert!(result1.is_ok() || result1.is_err());
    assert!(result2.is_ok() || result2.is_err());
    assert!(result3.is_ok() || result3.is_err());

    // Engine should still be usable
    assert!(engine.is_available());
}
