//! TDD Tests for Signal-Based Animation Controller
//!
//! These tests verify the proven signal-based patterns work correctly.

use leptos::prelude::*;
use leptos_motion_dom::signal_based_animation_controller::*;
use leptos_motion_core::types::AnimationValue;
use std::collections::HashMap;

#[test]
fn test_signal_based_animation_controller_creation() {
    // ✅ Test: Controller can be created with initial values
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map
    };

    let controller = SignalBasedAnimationController::new(initial_values.clone());
    
    // Verify initial state
    let current_values = controller.get_current_values();
    assert_eq!(current_values.len(), 2);
    assert_eq!(current_values.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(current_values.get("x"), Some(&AnimationValue::Pixels(0.0)));
    
    // Verify initial animation state
    assert!(!controller.is_animation_playing());
    assert_eq!(controller.get_progress(), 0.0);
}

#[test]
fn test_animate_to_triggers_animation() {
    // ✅ Test: animate_to properly triggers animation
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    };

    let controller = SignalBasedAnimationController::new(initial_values);
    
    // Initially not playing
    assert!(!controller.is_animation_playing());
    
    // Animate to new values
    let target_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("x".to_string(), AnimationValue::Pixels(100.0));
        map
    };
    
    controller.animate_to(target_values);
    
    // Should now be playing
    assert!(controller.is_animation_playing());
}

#[test]
fn test_animation_state_signal_tracking() {
    // ✅ Test: Animation state properly tracks signal changes
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    };

    let controller = SignalBasedAnimationController::new(initial_values);
    
    // Get the animation state signal
    let animation_state = controller.animation_state;
    
    // Verify initial state
    let state = animation_state.get();
    assert!(!state.is_playing);
    assert_eq!(state.progress, 0.0);
    assert_eq!(state.current_values.len(), 1);
    
    // Animate to trigger state change
    let target_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map
    };
    
    controller.animate_to(target_values);
    
    // State should be updated
    let updated_state = animation_state.get();
    assert!(updated_state.is_playing);
    assert_eq!(updated_state.progress, 1.0); // Our mock sets this to 1.0
}

#[test]
fn test_wasm_signal_handler_creation() {
    // ✅ Test: WASM signal handler can be created and managed
    let mut handler = WasmSignalHandler::new();
    
    // Create a signal
    let signal_id = handler.create_animation_signal("opacity", 1.0);
    assert_eq!(signal_id, 0);
    
    // Update the signal
    let result = handler.update_animation_value("opacity", 0.5);
    assert!(result.is_ok());
    
    // Try to update non-existent signal
    let result = handler.update_animation_value("nonexistent", 0.0);
    assert!(result.is_err());
}

#[test]
fn test_wasm_motion_component_lifecycle() {
    // ✅ Test: WASM motion component lifecycle management
    let mut component = WasmMotionComponent::new("test-component");
    
    // Initialize with values
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    };
    
    let js_initial = serde_wasm_bindgen::to_value(&initial_values).unwrap();
    component.initialize(initial_values);
    
    // Animate to new values
    let target_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map
    };
    
    let js_target = serde_wasm_bindgen::to_value(&target_values).unwrap();
    component.animate_to(target_values);
    
    // Cleanup
    component.cleanup();
}

#[test]
fn test_animation_value_to_css_conversion() {
    // ✅ Test: Animation values are properly converted to CSS
    assert_eq!(animation_value_to_css(&AnimationValue::Number(1.0)), "1");
    assert_eq!(animation_value_to_css(&AnimationValue::Pixels(100.0)), "100px");
    assert_eq!(animation_value_to_css(&AnimationValue::Degrees(45.0)), "45deg");
    assert_eq!(animation_value_to_css(&AnimationValue::Color("red".to_string())), "red");
    assert_eq!(animation_value_to_css(&AnimationValue::String("auto".to_string())), "auto");
}

#[test]
fn test_initial_to_css_conversion() {
    // ✅ Test: Initial values are properly converted to CSS string
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("x".to_string(), AnimationValue::Pixels(100.0));
        map.insert("rotate".to_string(), AnimationValue::Degrees(45.0));
        map
    };
    
    let css = initial_to_css(&initial_values);
    
    // Should contain all properties
    assert!(css.contains("opacity: 1"));
    assert!(css.contains("x: 100px"));
    assert!(css.contains("rotate: 45deg"));
}

#[test]
fn test_signal_tracking_with_multiple_updates() {
    // ✅ Test: Signal tracking works with multiple updates
    let initial_values = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    };

    let controller = SignalBasedAnimationController::new(initial_values);
    
    // First animation
    let target1 = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.5));
        map
    };
    
    controller.animate_to(target1);
    assert!(controller.is_animation_playing());
    
    // Second animation (should trigger again)
    let target2 = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("x".to_string(), AnimationValue::Pixels(50.0));
        map
    };
    
    controller.animate_to(target2);
    assert!(controller.is_animation_playing());
}

#[test]
fn test_animation_state_default() {
    // ✅ Test: AnimationState has proper default values
    let state = AnimationState::default();
    
    assert!(!state.is_playing);
    assert!(state.current_values.is_empty());
    assert!(state.target_values.is_empty());
    assert_eq!(state.progress, 0.0);
}

#[test]
fn test_animation_state_clone_and_partial_eq() {
    // ✅ Test: AnimationState can be cloned and compared
    let state1 = AnimationState {
        is_playing: true,
        current_values: {
            let mut map = HashMap::new();
            map.insert("opacity".to_string(), AnimationValue::Number(1.0));
            map
        },
        target_values: HashMap::new(),
        progress: 0.5,
    };
    
    let state2 = state1.clone();
    assert_eq!(state1, state2);
    
    let state3 = AnimationState {
        is_playing: false,
        current_values: {
            let mut map = HashMap::new();
            map.insert("opacity".to_string(), AnimationValue::Number(1.0));
            map
        },
        target_values: HashMap::new(),
        progress: 0.5,
    };
    
    assert_ne!(state1, state3);
}
