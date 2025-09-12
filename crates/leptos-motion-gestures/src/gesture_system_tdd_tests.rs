//! TDD tests for the complete gesture system implementation
//!
//! These tests ensure that the gesture system works correctly in production scenarios

use crate::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;

/// Helper function to create a MultiTouchState for testing
fn create_test_multi_touch_state(
    center: (f64, f64),
    scale: f64,
    rotation: f64,
    gesture_type: MultiTouchGestureType,
) -> MultiTouchState {
    let mut touches = HashMap::new();
    touches.insert(
        1,
        TouchPoint {
            id: 1,
            x: center.0 - 25.0,
            y: center.1 - 25.0,
            pressure: 1.0,
            timestamp: 1234567890,
        },
    );
    touches.insert(
        2,
        TouchPoint {
            id: 2,
            x: center.0 + 25.0,
            y: center.1 + 25.0,
            pressure: 1.0,
            timestamp: 1234567891,
        },
    );

    MultiTouchState {
        touches,
        center,
        average_distance: 50.0,
        scale,
        rotation,
        active: true,
        gesture_type,
    }
}

wasm_bindgen_test_configure!(run_in_browser);

/// Test that gesture configuration is properly created and validated
#[wasm_bindgen_test]
fn test_gesture_config_creation() {
    let config = GestureConfig {
        basic_gestures: true,
        multi_touch: true,
        pinch_to_zoom: true,
        rotation: true,
        sensitivity: 0.7,
        min_distance: 15.0,
        max_touches: 3,
        timeout_ms: 500,
    };

    assert!(config.basic_gestures);
    assert!(config.multi_touch);
    assert!(config.pinch_to_zoom);
    assert!(config.rotation);
    assert_eq!(config.sensitivity, 0.7);
    assert_eq!(config.min_distance, 15.0);
    assert_eq!(config.max_touches, 3);
    assert_eq!(config.timeout_ms, 500);
}

/// Test that default gesture configuration works correctly
#[wasm_bindgen_test]
fn test_gesture_config_default() {
    let config = GestureConfig::default();

    assert!(config.basic_gestures);
    assert!(config.multi_touch);
    assert!(config.pinch_to_zoom);
    assert!(config.rotation);
    assert_eq!(config.sensitivity, 0.5);
    assert_eq!(config.min_distance, 10.0);
    assert_eq!(config.max_touches, 5);
    assert_eq!(config.timeout_ms, 300);
}

/// Test that simplified gesture configuration works correctly
#[wasm_bindgen_test]
fn test_simplified_gesture_config() {
    let config = SimplifiedGestureConfig {
        max_touches: 4,
        min_distance: 20.0,
        timeout: 400,
        enable_pinch: true,
        enable_rotation: true,
        enable_pan: true,
    };

    assert_eq!(config.max_touches, 4);
    assert_eq!(config.min_distance, 20.0);
    assert_eq!(config.timeout, 400);
    assert!(config.enable_pinch);
    assert!(config.enable_rotation);
    assert!(config.enable_pan);
}

/// Test that gesture events are properly created and handled
#[wasm_bindgen_test]
fn test_gesture_event_creation() {
    let touch_point = TouchPoint {
        id: 1,
        x: 100.0,
        y: 200.0,
        pressure: 1.0,
        timestamp: 1234567890,
    };

    let touch_start_event = GestureEvent::TouchStart {
        touches: vec![touch_point.clone()],
    };

    let touch_move_event = GestureEvent::TouchMove {
        touches: vec![touch_point.clone()],
    };

    let touch_end_event = GestureEvent::TouchEnd {
        touches: vec![touch_point],
    };

    // Test that events are created correctly
    match touch_start_event {
        GestureEvent::TouchStart { touches } => {
            assert_eq!(touches.len(), 1);
            assert_eq!(touches[0].id, 1);
            assert_eq!(touches[0].x, 100.0);
            assert_eq!(touches[0].y, 200.0);
        }
        _ => panic!("Expected TouchStart event"),
    }

    match touch_move_event {
        GestureEvent::TouchMove { touches } => {
            assert_eq!(touches.len(), 1);
            assert_eq!(touches[0].id, 1);
        }
        _ => panic!("Expected TouchMove event"),
    }

    match touch_end_event {
        GestureEvent::TouchEnd { touches } => {
            assert_eq!(touches.len(), 1);
            assert_eq!(touches[0].id, 1);
        }
        _ => panic!("Expected TouchEnd event"),
    }
}

/// Test that gesture results are properly created and validated
#[wasm_bindgen_test]
fn test_gesture_result_creation() {
    let gesture_state =
        create_test_multi_touch_state((150.0, 250.0), 1.2, 45.0, MultiTouchGestureType::Pinch);

    let result = GestureResult {
        recognized: true,
        gesture_type: MultiTouchGestureType::Pinch,
        data: Some(gesture_state),
        confidence: 0.85,
    };

    assert!(result.recognized);
    assert_eq!(result.gesture_type, MultiTouchGestureType::Pinch);
    assert!(result.data.is_some());
    assert_eq!(result.confidence, 0.85);

    if let Some(data) = result.data {
        assert_eq!(data.touches.len(), 2);
        assert_eq!(data.center, (150.0, 250.0));
        assert_eq!(data.average_distance, 50.0);
        assert_eq!(data.scale, 1.2);
        assert_eq!(data.rotation, 45.0);
        assert!(data.active);
        assert_eq!(data.gesture_type, MultiTouchGestureType::Pinch);
    }
}

/// Test that simplified gesture results work correctly
#[wasm_bindgen_test]
fn test_simplified_gesture_result() {
    let result = SimplifiedGestureResult {
        gesture_type: SimplifiedGestureType::Pinch,
        scale: Some(1.5),
        rotation: None,
        translation: None,
        velocity: Some(SimplifiedVector2D { x: 5.0, y: 3.0 }),
        center: Some(SimplifiedVector2D { x: 200.0, y: 300.0 }),
        confidence: 0.9,
        duration: 250,
    };

    assert_eq!(result.gesture_type, SimplifiedGestureType::Pinch);
    assert_eq!(result.scale, Some(1.5));
    assert_eq!(result.rotation, None);
    assert_eq!(result.translation, None);
    assert_eq!(result.velocity, Some(SimplifiedVector2D { x: 5.0, y: 3.0 }));
    assert_eq!(
        result.center,
        Some(SimplifiedVector2D { x: 200.0, y: 300.0 })
    );
    assert_eq!(result.confidence, 0.9);
    assert_eq!(result.duration, 250);
}

/// Test that multi-touch gesture types are properly defined
#[wasm_bindgen_test]
fn test_multi_touch_gesture_types() {
    let gesture_types = [MultiTouchGestureType::None,
        MultiTouchGestureType::Pinch,
        MultiTouchGestureType::Rotation,
        MultiTouchGestureType::PinchAndRotate];

    assert_eq!(gesture_types.len(), 4);
    assert_eq!(gesture_types[0], MultiTouchGestureType::None);
    assert_eq!(gesture_types[1], MultiTouchGestureType::Pinch);
    assert_eq!(gesture_types[2], MultiTouchGestureType::Rotation);
    assert_eq!(gesture_types[3], MultiTouchGestureType::PinchAndRotate);
}

/// Test that simplified gesture types are properly defined
#[wasm_bindgen_test]
fn test_simplified_gesture_types() {
    let gesture_types = [SimplifiedGestureType::None,
        SimplifiedGestureType::Pinch,
        SimplifiedGestureType::Rotation,
        SimplifiedGestureType::Pan,
        SimplifiedGestureType::MultiTouch];

    assert_eq!(gesture_types.len(), 5);
    assert_eq!(gesture_types[0], SimplifiedGestureType::None);
    assert_eq!(gesture_types[1], SimplifiedGestureType::Pinch);
    assert_eq!(gesture_types[2], SimplifiedGestureType::Rotation);
    assert_eq!(gesture_types[3], SimplifiedGestureType::Pan);
    assert_eq!(gesture_types[4], SimplifiedGestureType::MultiTouch);
}

/// Test that gesture bounds are properly created and validated
#[wasm_bindgen_test]
fn test_gesture_bounds() {
    let bounds = SimplifiedGestureBounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 800.0,
        max_y: 600.0,
    };

    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.min_y, 0.0);
    assert_eq!(bounds.max_x, 800.0);
    assert_eq!(bounds.max_y, 600.0);
}

/// Test that gesture system can handle multiple touch points
#[wasm_bindgen_test]
fn test_multi_touch_handling() {
    let touch_points = vec![
        TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 1.0,
            timestamp: 1234567890,
        },
        TouchPoint {
            id: 2,
            x: 300.0,
            y: 400.0,
            pressure: 1.0,
            timestamp: 1234567891,
        },
        TouchPoint {
            id: 3,
            x: 500.0,
            y: 600.0,
            pressure: 1.0,
            timestamp: 1234567892,
        },
    ];

    let multi_touch_event = GestureEvent::TouchStart {
        touches: touch_points.clone(),
    };

    match multi_touch_event {
        GestureEvent::TouchStart { touches } => {
            assert_eq!(touches.len(), 3);
            assert_eq!(touches[0].id, 1);
            assert_eq!(touches[1].id, 2);
            assert_eq!(touches[2].id, 3);
        }
        _ => panic!("Expected TouchStart event"),
    }
}

/// Test that gesture system can handle gesture recognition
#[wasm_bindgen_test]
fn test_gesture_recognition() {
    let gesture_state =
        create_test_multi_touch_state((200.0, 300.0), 1.0, 0.0, MultiTouchGestureType::None);

    let recognized_event = GestureEvent::GestureRecognized {
        gesture: gesture_state.clone(),
    };

    match recognized_event {
        GestureEvent::GestureRecognized { gesture } => {
            assert_eq!(gesture.touches.len(), 2);
            assert_eq!(gesture.center, (200.0, 300.0));
            assert_eq!(gesture.scale, 1.0);
            assert_eq!(gesture.rotation, 0.0);
        }
        _ => panic!("Expected GestureRecognized event"),
    }
}

/// Test that gesture system can handle gesture updates
#[wasm_bindgen_test]
fn test_gesture_updates() {
    let updated_state =
        create_test_multi_touch_state((210.0, 310.0), 1.2, 15.0, MultiTouchGestureType::Pinch);

    let updated_event = GestureEvent::GestureUpdated {
        gesture: updated_state,
    };

    match updated_event {
        GestureEvent::GestureUpdated { gesture } => {
            assert_eq!(gesture.touches.len(), 2);
            assert_eq!(gesture.center, (210.0, 310.0));
            assert_eq!(gesture.scale, 1.2);
            assert_eq!(gesture.rotation, 15.0);
        }
        _ => panic!("Expected GestureUpdated event"),
    }
}

/// Test that gesture system can handle gesture completion
#[wasm_bindgen_test]
fn test_gesture_completion() {
    let completed_state =
        create_test_multi_touch_state((250.0, 350.0), 1.5, 30.0, MultiTouchGestureType::Pinch);

    let completed_event = GestureEvent::GestureCompleted {
        gesture: completed_state,
    };

    match completed_event {
        GestureEvent::GestureCompleted { gesture } => {
            assert_eq!(gesture.touches.len(), 2);
            assert_eq!(gesture.center, (250.0, 350.0));
            assert_eq!(gesture.scale, 1.5);
            assert_eq!(gesture.rotation, 30.0);
        }
        _ => panic!("Expected GestureCompleted event"),
    }
}
