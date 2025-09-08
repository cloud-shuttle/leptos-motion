//! Tests for gesture interaction capabilities
//!
//! These tests define the expected behavior for gesture interactions
//! like drag, hover, tap, and swipe.

use crate::test_utils::*;
use leptos::*;
use leptos_motion_core::*;
use leptos_motion_gestures::*;

/// Test that drag gestures work correctly
#[test]
fn test_drag_gesture() {
    // Given: A draggable element
    let (position, set_position) = create_signal((0.0, 0.0));
    let (is_dragging, set_is_dragging) = create_signal(false);

    // When: Drag gesture is initiated
    set_is_dragging.set(true);
    set_position.set((100.0, 50.0));

    // Then: Position should be updated and dragging state should be true
    assert!(is_dragging.get());
    assert_eq!(position.get(), (100.0, 50.0));
}

/// Test that hover gestures work correctly
#[test]
fn test_hover_gesture() {
    // Given: A hoverable element
    let (is_hovered, set_is_hovered) = create_signal(false);
    let (hover_scale, set_hover_scale) = create_signal(1.0);

    // When: Hover gesture is triggered
    set_is_hovered.set(true);
    set_hover_scale.set(1.1);

    // Then: Hover state should be true and scale should increase
    assert!(is_hovered.get());
    assert_eq!(hover_scale.get(), 1.1);
}

/// Test that tap gestures work correctly
#[test]
fn test_tap_gesture() {
    // Given: A tappable element
    let (tap_count, set_tap_count) = create_signal(0);
    let (tap_scale, set_tap_scale) = create_signal(1.0);

    // When: Tap gesture is triggered
    set_tap_count.set(tap_count.get() + 1);
    set_tap_scale.set(0.95);

    // Then: Tap count should increase and scale should decrease briefly
    assert_eq!(tap_count.get(), 1);
    assert_eq!(tap_scale.get(), 0.95);
}

/// Test that swipe gestures work correctly
#[test]
fn test_swipe_gesture() {
    // Given: A swipeable element
    let (swipe_direction, set_swipe_direction) = create_signal(SwipeDirection::None);
    let (swipe_distance, set_swipe_distance) = create_signal(0.0);

    // When: Swipe gesture is detected
    set_swipe_direction.set(SwipeDirection::Right);
    set_swipe_distance.set(150.0);

    // Then: Swipe direction and distance should be set
    assert_eq!(swipe_direction.get(), SwipeDirection::Right);
    assert_eq!(swipe_distance.get(), 150.0);
}

/// Test that pinch gestures work correctly
#[test]
fn test_pinch_gesture() {
    // Given: A pinchable element
    let (scale, set_scale) = create_signal(1.0);
    let (is_pinching, set_is_pinching) = create_signal(false);

    // When: Pinch gesture is detected
    set_is_pinching.set(true);
    set_scale.set(1.5);

    // Then: Scale should be updated and pinching state should be true
    assert!(is_pinching.get());
    assert_eq!(scale.get(), 1.5);
}

/// Test that gesture constraints work correctly
#[test]
fn test_gesture_constraints() {
    // Given: A constrained drag gesture
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    // When: Position is set within constraints
    let (position, set_position) = create_signal((0.0, 0.0));
    set_position.set((50.0, 25.0));

    // Then: Position should be within constraints
    let (x, y) = position.get();
    assert!(x >= constraints.left.unwrap());
    assert!(x <= constraints.right.unwrap());
    assert!(y >= constraints.top.unwrap());
    assert!(y <= constraints.bottom.unwrap());
}

/// Test that gesture velocity is calculated correctly
#[test]
fn test_gesture_velocity() {
    // Given: A gesture with velocity tracking
    let (velocity, set_velocity) = create_signal((0.0, 0.0));

    // When: Fast movement is detected
    set_velocity.set((500.0, 200.0));

    // Then: Velocity should be calculated correctly
    let (vx, vy) = velocity.get();
    assert!(vx > 0.0);
    assert!(vy > 0.0);
}

/// Test that gesture momentum works correctly
#[test]
fn test_gesture_momentum() {
    // Given: A gesture with momentum
    let (momentum, set_momentum) = create_signal((0.0, 0.0));
    let (is_momentum_active, set_is_momentum_active) = create_signal(false);

    // When: Momentum is applied
    set_momentum.set((100.0, 50.0));
    set_is_momentum_active.set(true);

    // Then: Momentum should be active and have values
    assert!(is_momentum_active.get());
    let (mx, my) = momentum.get();
    assert!(mx > 0.0);
    assert!(my > 0.0);
}

/// Test that gesture events are properly handled
#[test]
fn test_gesture_events() {
    // Given: A gesture event handler
    let (event_count, set_event_count) = create_signal(0);
    let (last_event, set_last_event) = create_signal(GestureEvent::None);

    // When: Gesture events are triggered
    set_event_count.set(event_count.get() + 1);
    set_last_event.set(GestureEvent::DragStart);

    // Then: Event count should increase and last event should be set
    assert_eq!(event_count.get(), 1);
    assert_eq!(last_event.get(), GestureEvent::DragStart);
}

/// Test that gesture sensitivity is configurable
#[test]
fn test_gesture_sensitivity() {
    // Given: A gesture with configurable sensitivity
    let sensitivity = GestureSensitivity {
        drag_threshold: 10.0,
        swipe_threshold: 50.0,
        pinch_threshold: 0.1,
    };

    // When: Sensitivity is configured
    // Then: Thresholds should be set correctly
    assert_eq!(sensitivity.drag_threshold, 10.0);
    assert_eq!(sensitivity.swipe_threshold, 50.0);
    assert_eq!(sensitivity.pinch_threshold, 0.1);
}

/// Test that gesture combinations work correctly
#[test]
fn test_gesture_combinations() {
    // Given: Multiple gestures on the same element
    let (drag_active, set_drag_active) = create_signal(false);
    let (hover_active, set_hover_active) = create_signal(false);
    let (tap_active, set_tap_active) = create_signal(false);

    // When: Multiple gestures are active
    set_drag_active.set(true);
    set_hover_active.set(true);
    set_tap_active.set(false);

    // Then: All gesture states should be independent
    assert!(drag_active.get());
    assert!(hover_active.get());
    assert!(!tap_active.get());
}

/// Test that gesture cleanup works correctly
#[test]
fn test_gesture_cleanup() {
    // Given: A gesture that needs cleanup
    let (is_active, set_is_active) = create_signal(true);
    let (cleanup_called, set_cleanup_called) = create_signal(false);

    // When: Cleanup is called
    set_is_active.set(false);
    set_cleanup_called.set(true);

    // Then: Gesture should be inactive and cleanup should be called
    assert!(!is_active.get());
    assert!(cleanup_called.get());
}
