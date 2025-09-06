//! TDD Tests for Example API Compatibility
//!
//! These tests verify that the examples can compile and work with the current API.
//! Following TDD: RED (write failing tests) -> GREEN (make them pass) -> REFACTOR

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::components::MotionDivFlexible;

#[test]
fn test_advanced_features_api_compatibility() {
    // RED: This test should fail initially, then we'll make it pass

    // Test 1: Basic MotionDivFlexible with &str class and style
    let _component = view! {
        <MotionDivFlexible
            class="test-class"
            style="background: red; width: 100px; height: 100px;"
        >
            "Test Content"
        </MotionDivFlexible>
    };

    // Test 2: MotionDivFlexible with bool drag
    let _component2 = view! {
        <MotionDivFlexible
            class="drag-test"
            drag=true
        >
            "Drag Test"
        </MotionDivFlexible>
    };

    // Test 3: MotionDivFlexible with direct AnimationTarget for while_hover
    let hover_target = AnimationTarget::new();
    let _component3 = view! {
        <MotionDivFlexible
            class="hover-test"
            while_hover=hover_target
        >
            "Hover Test"
        </MotionDivFlexible>
    };

    // Test 4: MotionDivFlexible with bool layout
    let _component4 = view! {
        <MotionDivFlexible
            class="layout-test"
            layout=true
        >
            "Layout Test"
        </MotionDivFlexible>
    };

    // Test 5: MotionDivFlexible with String key
    let _component5 = view! {
        <MotionDivFlexible
            class="key-test"
            key="test-key".to_string()
        >
            "Key Test"
        </MotionDivFlexible>
    };

    // If we get here, the API compatibility is working
    assert!(true);
}

#[test]
fn test_import_compatibility() {
    // Test that all required imports are available
    use leptos_motion_core::AnimationTarget;
    use leptos_motion_core::Keyframes;
    use leptos_motion_core::RepeatConfig;

    // Test that we can create these types
    let _keyframes = Keyframes::new();
    let _repeat_config = RepeatConfig::Infinite;
    let _animation_target = AnimationTarget::new();

    assert!(true);
}

#[test]
fn test_gesture_imports() {
    // Test that gesture-related imports work
    use leptos_motion_gestures::*;

    // This should compile if the imports are correct
    assert!(true);
}
