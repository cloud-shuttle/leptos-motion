//! TDD Tests for MotionDiv Engine Integration (Phase 1)
//!
//! This module contains comprehensive failing tests for connecting the sophisticated
//! animation engine to MotionDiv components, addressing the integration gap.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;

// Import the actual MotionDiv component that we want to test
use crate::components::MotionDiv;

/// Test that MotionDiv actually uses the animation engine instead of style setting
#[test]
fn test_motion_div_uses_animation_engine() {
    // RED PHASE: This test will fail initially because MotionDiv doesn't use the engine

    // This test will fail because the current MotionDiv doesn't support:
    // 1. node_ref parameter
    // 2. Animation engine integration
    // 3. Proper transition handling

    let test_element = create_test_element();
    let (is_visible, set_visible) = signal(false);

    // This will fail to compile because MotionDiv doesn't have a node_ref prop
    let component = view! {
        <MotionDiv
            node_ref=test_element.clone()  // ❌ This prop doesn't exist
            initial=create_animation_target("opacity", 0.0)
            animate=create_animation_target("opacity", 1.0)
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "Test Content"
        </MotionDiv>
    };

    // Mount the component
    mount_to_body(move || component);

    // Verify the animation engine was called
    let engine_calls = get_animation_engine_calls();
    assert_eq!(
        engine_calls.len(),
        1,
        "Animation engine should be called once"
    );

    let call = &engine_calls[0];
    // Note: NodeRef doesn't implement PartialEq, so we can't compare directly
    // In a real implementation, we'd verify the element reference differently
    assert_eq!(call.from.get("opacity"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(call.to.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(call.transition.duration, Some(0.5));
}

/// Test that MotionDiv requires animation engine context
#[test]
fn test_motion_div_requires_animation_engine_context() {
    // RED PHASE: This test will fail because MotionDiv doesn't require engine context

    // This test will fail because the current MotionDiv doesn't:
    // 1. Require an AnimationEngine context
    // 2. Use the engine for animations

    let component = view! {
        <MotionDiv
            initial=create_animation_target("opacity", 0.0)
            animate=create_animation_target("opacity", 1.0)
        >
            "Test Content"
        </MotionDiv>
    };

    // This should fail because no AnimationEngine context is provided
    mount_to_body(move || component);

    // If we get here, the test fails because MotionDiv should require engine context
    panic!("MotionDiv should require AnimationEngine context but doesn't");
}

/// Test that MotionDiv properly handles NodeRef integration
#[test]
fn test_motion_div_node_ref_integration() {
    // RED PHASE: NodeRef integration is missing

    let node_ref = NodeRef::new();
    let (animation_handle, set_handle) = signal(None::<AnimationHandle>);

    let component = view! {
        <MotionDiv
            node_ref=node_ref.clone()
            initial=create_animation_target("scale", 0.5)
            animate=create_animation_target("scale", 1.0)
            // TODO: Add on_animation_start callback when implemented
        >
            "NodeRef Test"
        </MotionDiv>
    };

    mount_to_body(move || component);

    // Verify NodeRef is properly connected
    assert!(
        node_ref.get().is_some(),
        "NodeRef should be connected to DOM element"
    );

    // Verify animation handle is returned
    assert!(
        animation_handle.get().is_some(),
        "Animation handle should be provided"
    );

    let handle = animation_handle.get().unwrap();
    // Note: AnimationHandle doesn't have is_valid method in current implementation
    // This test will fail until we implement proper animation handle validation
}

/// Test that MotionDiv supports reactive animations
#[test]
fn test_motion_div_reactive_animations() {
    // RED PHASE: Reactive animations don't work

    let (is_visible, set_visible) = signal(false);
    let animation_target = move || {
        if is_visible.get() {
            create_animation_target("opacity", 1.0)
        } else {
            create_animation_target("opacity", 0.0)
        }
    };

    let component = view! {
        <MotionDiv
            animate=animation_target()
            transition=Transition {
                duration: Some(0.3),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Reactive Animation"
        </MotionDiv>
    };

    mount_to_body(move || component);

    // Initial state should be invisible
    let initial_calls = get_animation_engine_calls();
    assert_eq!(initial_calls.len(), 1);
    assert_eq!(
        initial_calls[0].to.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );

    // Change state
    set_visible.set(true);

    // Should trigger new animation
    let updated_calls = get_animation_engine_calls();
    assert_eq!(updated_calls.len(), 2);
    assert_eq!(
        updated_calls[1].to.get("opacity"),
        Some(&AnimationValue::Number(1.0))
    );
}

/// Test that MotionDiv properly handles gesture integration
#[test]
fn test_motion_div_gesture_integration() {
    // RED PHASE: Gesture integration is missing

    let node_ref = NodeRef::new();
    let (drag_position, set_drag_position) = signal((0.0, 0.0));

    let component = view! {
        <MotionDiv
            node_ref=node_ref.clone()
            // TODO: Add drag prop when implemented
            // TODO: Add while_drag and on_drag props when implemented
        >
            "Draggable Element"
        </MotionDiv>
    };

    mount_to_body(move || component);

    // Verify gesture detector is attached
    let element = node_ref.get().unwrap();
    let gesture_detector = get_gesture_detector(&element);
    assert!(
        gesture_detector.is_some(),
        "Gesture detector should be attached"
    );

    // Simulate drag event
    simulate_drag_event(&element, 10.0, 20.0);

    // Verify drag position is updated
    assert_eq!(drag_position.get(), (10.0, 20.0));

    // Verify while_drag animation is triggered
    let drag_animations = get_gesture_animations(&element);
    assert!(drag_animations.contains_key("while_drag"));
}

/// Test that MotionDiv handles animation lifecycle properly
#[test]
fn test_motion_div_animation_lifecycle() {
    // RED PHASE: Animation lifecycle is not properly managed

    let (animation_state, set_animation_state) = signal(AnimationState::Idle);
    let (completion_count, set_completion_count) = signal(0);

    let component = view! {
        <MotionDiv
            initial=create_animation_target("opacity", 0.0)
            animate=create_animation_target("opacity", 1.0)
            transition=Transition {
                duration: Some(0.1), // Short for testing
                ease: Easing::Linear,
                ..Default::default()
            }
            // TODO: Add animation callbacks when implemented
        >
            "Lifecycle Test"
        </MotionDiv>
    };

    mount_to_body(move || component);

    // Verify initial state
    assert_eq!(animation_state.get(), AnimationState::Running);

    // Wait for animation to complete
    wait_for_animation_completion(150); // 100ms + buffer

    // Verify completion
    assert_eq!(animation_state.get(), AnimationState::Completed);
    assert_eq!(completion_count.get(), 1);
}

/// Test that MotionDiv properly cleans up animations on unmount
#[test]
fn test_motion_div_cleanup_on_unmount() {
    // RED PHASE: Cleanup is not implemented

    let (is_mounted, set_mounted) = signal(true);
    let node_ref = NodeRef::new();

    let component = move || {
        if is_mounted.get() {
            Some(view! {
                <MotionDiv
                    node_ref=node_ref.clone()
                    animate=create_animation_target("opacity", 1.0)
                >
                    "Cleanup Test"
                </MotionDiv>
            })
        } else {
            None
        }
    };

    mount_to_body(move || component);

    // Start animation
    let initial_animations = get_active_animations();
    assert_eq!(initial_animations.len(), 1);

    // Unmount component
    set_mounted.set(false);

    // Wait for cleanup
    wait_for_cleanup(50);

    // Verify animation is cleaned up
    let final_animations = get_active_animations();
    assert_eq!(
        final_animations.len(),
        0,
        "Animations should be cleaned up on unmount"
    );
}

// Helper functions for tests

fn create_test_element() -> NodeRef<leptos::html::Div> {
    let node_ref = NodeRef::new();
    // In a real test, this would be connected to an actual DOM element
    node_ref
}

fn create_animation_target(property: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), AnimationValue::Number(value));
    target
}

fn get_animation_engine_calls() -> Vec<AnimationEngineCall> {
    // This would track calls to the animation engine
    // In a real implementation, this would be a global test state
    vec![]
}

fn get_gesture_detector(_element: &web_sys::Element) -> Option<()> {
    // This would retrieve the gesture detector attached to the element
    None
}

fn simulate_drag_event(_element: &web_sys::Element, _x: f64, _y: f64) {
    // This would simulate a drag event on the element
}

fn get_gesture_animations(_element: &web_sys::Element) -> HashMap<String, AnimationTarget> {
    // This would retrieve gesture-triggered animations
    HashMap::new()
}

fn get_active_animations() -> Vec<AnimationHandle> {
    // This would retrieve all active animations
    vec![]
}

fn wait_for_animation_completion(_ms: u64) {
    // This would wait for animation completion
    std::thread::sleep(std::time::Duration::from_millis(10));
}

fn wait_for_cleanup(_ms: u64) {
    // This would wait for cleanup to complete
    std::thread::sleep(std::time::Duration::from_millis(10));
}

#[derive(Debug, Clone, PartialEq)]
enum AnimationState {
    Idle,
    Running,
    Completed,
    Error,
}

#[derive(Debug, Clone)]
struct AnimationEngineCall {
    element: NodeRef<leptos::html::Div>,
    from: AnimationTarget,
    to: AnimationTarget,
    transition: Transition,
}
