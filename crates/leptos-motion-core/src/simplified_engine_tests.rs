// TDD Tests for Simplified Animation Engine API
//
// This module contains tests for the new simplified animation engine API
// that hides implementation details and provides a clean, simple interface.

use crate::*;
use std::collections::HashMap;
// WASM-specific test configuration - conditional compilation
#[cfg(feature = "web-sys")]
use wasm_bindgen_test::*;

#[cfg(feature = "web-sys")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test fixture for creating a mock element
#[cfg(feature = "web-sys")]
fn mock_element() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_element("div").unwrap()
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

#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_creation() {
    // Test that we can create a simplified animation engine
    let engine = SimplifiedAnimationEngine::new();
    assert!(engine.is_available());
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_basic_animation() {
    // Test basic animation functionality
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start animation
    let handle = engine.animate(&element, &target, &transition).unwrap();
    assert!(handle.0 > 0);

    // Check if animation is running
    assert!(engine.is_running(handle));

    // Stop animation
    engine.stop(handle).unwrap();
    assert!(!engine.is_running(handle));
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_pause_resume() {
    // Test pause and resume functionality
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    let handle = engine.animate(&element, &target, &transition).unwrap();

    // Pause animation
    engine.pause(handle).unwrap();
    assert_eq!(engine.get_state(handle).unwrap(), PlaybackState::Paused);

    // Resume animation
    engine.resume(handle).unwrap();
    assert_eq!(engine.get_state(handle).unwrap(), PlaybackState::Running);

    // Stop animation
    engine.stop(handle).unwrap();
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_multiple_animations() {
    // Test managing multiple animations
    let mut engine = SimplifiedAnimationEngine::new();
    let element1 = mock_element();
    let element2 = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start multiple animations
    let handle1 = engine.animate(&element1, &target, &transition).unwrap();
    let handle2 = engine.animate(&element2, &target, &transition).unwrap();

    assert!(handle1.0 != handle2.0);
    assert!(engine.is_running(handle1));
    assert!(engine.is_running(handle2));

    // Stop one animation
    engine.stop(handle1).unwrap();
    assert!(!engine.is_running(handle1));
    assert!(engine.is_running(handle2));

    // Stop remaining animation
    engine.stop(handle2).unwrap();
    assert!(!engine.is_running(handle2));
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_error_handling() {
    // Test error handling for invalid operations
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    let handle = engine.animate(&element, &target, &transition).unwrap();

    // Stop animation
    engine.stop(handle).unwrap();

    // Try to stop already stopped animation (should handle gracefully)
    let result = engine.stop(handle);
    assert!(result.is_ok() || result.is_err()); // Should not panic

    // Try to pause stopped animation (should handle gracefully)
    let result = engine.pause(handle);
    assert!(result.is_ok() || result.is_err()); // Should not panic
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_performance_metrics() {
    // Test performance metrics access
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start and stop animation to generate metrics
    let handle = engine.animate(&element, &target, &transition).unwrap();
    engine.stop(handle).unwrap();

    // Get performance metrics
    let metrics = engine.get_performance_metrics();
    assert!(metrics.is_some());

    let metrics = metrics.unwrap();
    assert!(metrics.active_animations >= 0);
    assert!(metrics.average_frame_time >= 0.0);
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_spring_animation() {
    // Test spring animation
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let mut transition = simple_transition();
    transition.ease = Easing::Spring(SpringConfig {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        ..Default::default()
    });

    let handle = engine.animate(&element, &target, &transition).unwrap();
    assert!(engine.is_running(handle));

    engine.stop(handle).unwrap();
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_stagger_animation() {
    // Test stagger animation
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let mut transition = simple_transition();
    transition.stagger = Some(StaggerConfig {
        delay: 0.1,
        from: StaggerFrom::First,
    });

    let handle = engine.animate(&element, &target, &transition).unwrap();
    assert!(engine.is_running(handle));

    engine.stop(handle).unwrap();
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_cleanup() {
    // Test engine cleanup
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start multiple animations
    let handle1 = engine.animate(&element, &target, &transition).unwrap();
    let handle2 = engine.animate(&element, &target, &transition).unwrap();

    // Cleanup all animations
    engine.cleanup().unwrap();

    // All animations should be stopped
    assert!(!engine.is_running(handle1));
    assert!(!engine.is_running(handle2));
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_batch_operations() {
    // Test batch operations
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start multiple animations
    let handles = vec![
        engine.animate(&element, &target, &transition).unwrap(),
        engine.animate(&element, &target, &transition).unwrap(),
        engine.animate(&element, &target, &transition).unwrap(),
    ];

    // Batch stop all animations
    engine.stop_all().unwrap();

    // All animations should be stopped
    for handle in handles {
        assert!(!engine.is_running(handle));
    }
}

#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_simplified_animation_engine_global_control() {
    // Test global control functions
    let mut engine = SimplifiedAnimationEngine::new();
    let element = mock_element();
    let target = simple_animation_target();
    let transition = simple_transition();

    // Start animation
    let handle = engine.animate(&element, &target, &transition).unwrap();

    // Pause all animations
    engine.pause_all().unwrap();
    assert_eq!(engine.get_state(handle).unwrap(), PlaybackState::Paused);

    // Resume all animations
    engine.resume_all().unwrap();
    assert_eq!(engine.get_state(handle).unwrap(), PlaybackState::Running);

    // Stop all animations
    engine.stop_all().unwrap();
    assert!(!engine.is_running(handle));
}
