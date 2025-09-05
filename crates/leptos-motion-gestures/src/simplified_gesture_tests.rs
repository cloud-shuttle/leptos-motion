// TDD Tests for Simplified Gesture API
// 
// This module contains tests for the new simplified gesture API
// that provides a clean, simple interface for gesture handling.

use crate::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test fixture for creating a mock touch point
fn mock_touch_point(id: u64, x: f64, y: f64) -> TouchPoint {
    TouchPoint {
        id,
        x,
        y,
        pressure: 1.0,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
    }
}

/// Test fixture for creating a simple gesture config
fn simple_gesture_config() -> SimplifiedGestureConfig {
    SimplifiedGestureConfig::new()
        .max_touches(5)
        .min_distance(10.0)
        .timeout(1000)
        .enable_pinch(true)
        .enable_rotation(true)
        .enable_pan(true)
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_creation() {
    // Test that we can create a simplified gesture detector
    let detector = SimplifiedGestureDetector::new();
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
    assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_with_config() {
    // Test gesture detector with custom configuration
    let config = simple_gesture_config();
    let detector = SimplifiedGestureDetector::with_config(config.clone());
    
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
    assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_single_touch() {
    // Test single touch handling
    let mut detector = SimplifiedGestureDetector::new();
    let touch = mock_touch_point(1, 100.0, 100.0);
    
    let result = detector.handle_touch_start(vec![touch.clone()]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::None);
    assert_eq!(detector.touch_count(), 1);
    assert!(!detector.is_active());
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_pinch_gesture() {
    // Test pinch gesture detection
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start pinch gesture
    let result = detector.handle_touch_start(vec![touch1, touch2]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Pinch);
    assert_eq!(detector.touch_count(), 2);
    assert!(detector.is_active());
    
    // Move touches to create pinch
    let touch1_moved = mock_touch_point(1, 90.0, 100.0);
    let touch2_moved = mock_touch_point(2, 210.0, 100.0);
    
    let result = detector.handle_touch_move(vec![touch1_moved, touch2_moved]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Pinch);
    assert!(result.scale.is_some());
    assert!(result.scale.unwrap() != 1.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_rotation_gesture() {
    // Test rotation gesture detection
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start rotation gesture
    let result = detector.handle_touch_start(vec![touch1, touch2]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Pinch); // Initially detected as pinch
    
    // Move touches to create rotation
    let touch1_rotated = mock_touch_point(1, 110.0, 90.0);
    let touch2_rotated = mock_touch_point(2, 190.0, 110.0);
    
    let result = detector.handle_touch_move(vec![touch1_rotated, touch2_rotated]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Rotation);
    assert!(result.rotation.is_some());
    assert!(result.rotation.unwrap().abs() > 0.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_pan_gesture() {
    // Test pan gesture detection
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start pan gesture
    let result = detector.handle_touch_start(vec![touch1, touch2]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Pinch); // Initially detected as pinch
    
    // Move touches to create pan
    let touch1_panned = mock_touch_point(1, 150.0, 150.0);
    let touch2_panned = mock_touch_point(2, 250.0, 150.0);
    
    let result = detector.handle_touch_move(vec![touch1_panned, touch2_panned]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::Pan);
    assert!(result.translation.is_some());
    let translation = result.translation.unwrap();
    assert!(translation.x != 0.0 || translation.y != 0.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_end() {
    // Test gesture end handling
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    assert!(detector.is_active());
    
    // End gesture
    let result = detector.handle_touch_end(vec![1, 2]);
    assert_eq!(result.gesture_type, SimplifiedGestureType::None);
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_cancel() {
    // Test gesture cancellation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    assert!(detector.is_active());
    
    // Cancel gesture
    detector.cancel();
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
    assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_reset() {
    // Test gesture reset
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    assert!(detector.is_active());
    
    // Reset gesture
    detector.reset();
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
    assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_data() {
    // Test gesture data access
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get gesture data
    let data = detector.get_gesture_data();
    assert!(data.is_some());
    let data = data.unwrap();
    assert_eq!(data.gesture_type, SimplifiedGestureType::Pinch);
    assert_eq!(data.touch_count, 2);
    assert!(data.is_active);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_configuration() {
    // Test configuration management
    let mut detector = SimplifiedGestureDetector::new();
    let config = simple_gesture_config();
    
    // Update configuration
    detector.update_config(config.clone());
    
    // Verify configuration was updated
    let current_config = detector.get_config();
    assert_eq!(current_config.max_touches, config.max_touches);
    assert_eq!(current_config.min_distance, config.min_distance);
    assert_eq!(current_config.timeout, config.timeout);
    assert_eq!(current_config.enable_pinch, config.enable_pinch);
    assert_eq!(current_config.enable_rotation, config.enable_rotation);
    assert_eq!(current_config.enable_pan, config.enable_pan);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_confidence() {
    // Test gesture confidence
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get confidence
    let confidence = detector.get_confidence();
    assert!(confidence >= 0.0);
    assert!(confidence <= 1.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_velocity() {
    // Test gesture velocity calculation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Move touches quickly
    let touch1_moved = mock_touch_point(1, 150.0, 150.0);
    let touch2_moved = mock_touch_point(2, 250.0, 150.0);
    
    let result = detector.handle_touch_move(vec![touch1_moved, touch2_moved]);
    assert!(result.velocity.is_some());
    let velocity = result.velocity.unwrap();
    assert!(velocity.x != 0.0 || velocity.y != 0.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_center() {
    // Test gesture center calculation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get center
    let center = detector.get_center();
    assert!(center.is_some());
    let center = center.unwrap();
    assert_eq!(center.x, 150.0); // (100 + 200) / 2
    assert_eq!(center.y, 100.0); // (100 + 100) / 2
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_duration() {
    // Test gesture duration tracking
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get duration
    let duration = detector.get_duration();
    assert!(duration >= 0);
    
    // Move touches
    let touch1_moved = mock_touch_point(1, 110.0, 110.0);
    let touch2_moved = mock_touch_point(2, 210.0, 110.0);
    
    detector.handle_touch_move(vec![touch1_moved, touch2_moved]);
    
    // Duration should have increased
    let new_duration = detector.get_duration();
    assert!(new_duration >= duration);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_bounds() {
    // Test gesture bounds calculation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 200.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get bounds
    let bounds = detector.get_bounds();
    assert!(bounds.is_some());
    let bounds = bounds.unwrap();
    assert_eq!(bounds.min_x, 100.0);
    assert_eq!(bounds.max_x, 200.0);
    assert_eq!(bounds.min_y, 100.0);
    assert_eq!(bounds.max_y, 200.0);
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_distance() {
    // Test gesture distance calculation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get distance
    let distance = detector.get_distance();
    assert!(distance.is_some());
    let distance = distance.unwrap();
    assert_eq!(distance, 100.0); // Distance between (100,100) and (200,100)
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_angle() {
    // Test gesture angle calculation
    let mut detector = SimplifiedGestureDetector::new();
    let touch1 = mock_touch_point(1, 100.0, 100.0);
    let touch2 = mock_touch_point(2, 200.0, 100.0);
    
    // Start gesture
    detector.handle_touch_start(vec![touch1, touch2]);
    
    // Get angle
    let angle = detector.get_angle();
    assert!(angle.is_some());
    let angle = angle.unwrap();
    assert_eq!(angle, 0.0); // Horizontal line
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_clone() {
    // Test that simplified gesture detector can be cloned
    let detector1 = SimplifiedGestureDetector::new();
    let detector2 = detector1.clone();
    
    assert_eq!(detector1.is_active(), detector2.is_active());
    assert_eq!(detector1.touch_count(), detector2.touch_count());
    assert_eq!(detector1.gesture_type(), detector2.gesture_type());
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_debug() {
    // Test debug formatting
    let detector = SimplifiedGestureDetector::new();
    let debug_str = format!("{:?}", detector);
    assert!(debug_str.contains("SimplifiedGestureDetector"));
    assert!(debug_str.contains("is_active"));
    assert!(debug_str.contains("touch_count"));
    assert!(debug_str.contains("gesture_type"));
}

#[wasm_bindgen_test]
fn test_simplified_gesture_detector_gesture_default() {
    // Test default implementation
    let detector = SimplifiedGestureDetector::default();
    assert!(!detector.is_active());
    assert_eq!(detector.touch_count(), 0);
    assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
}
