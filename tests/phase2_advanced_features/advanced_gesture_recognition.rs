//! Phase 2 TDD Implementation: Advanced Gesture Recognition
//!
//! This module implements complex multi-touch gestures with physics-based
//! animations, velocity calculations, and gesture predictions.
//!
//! Red Phase: Comprehensive gesture recognition tests
//! Green Phase: Minimal gesture system implementation
//! Refactor Phase: Performance and accuracy optimization

use leptos_motion_core::{
    AnimationConfig, AnimationEngine, AnimationError, AnimationHandle, AnimationValue,
};
use rstest::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Complex gesture combinations (pinch + rotate + drag)
/// This will FAIL initially - need multi-touch gesture system
#[rstest]
#[case::pinch_and_rotate(vec!["pinch", "rotate"])]
#[case::drag_and_rotate(vec!["drag", "rotate"])]
#[case::triple_combination(vec!["pinch", "rotate", "drag"])]
#[wasm_bindgen_test]
fn test_complex_gesture_combinations(#[case] gesture_types: Vec<&str>) {
    // Arrange: Create gesture recognizer
    let mut recognizer = GestureRecognizer::new();
    let engine = AnimationEngine::new();

    // Set up simultaneous gesture detection
    for gesture_type in &gesture_types {
        recognizer.enable_gesture(*gesture_type);
    }

    // Simulate multi-touch input sequence
    let touch_sequence = create_multi_touch_sequence(&gesture_types);

    // Act: Process touch events
    let mut detected_gestures = Vec::new();
    for touch_event in touch_sequence {
        if let Ok(gesture) = recognizer.process_touch_event(touch_event) {
            detected_gestures.push(gesture);
        }
    }

    // Assert: Should detect all expected gesture types
    for expected_gesture in &gesture_types {
        assert!(
            detected_gestures
                .iter()
                .any(|g| g.gesture_type() == *expected_gesture),
            "Should detect {} gesture in combination {:?}",
            expected_gesture,
            gesture_types
        );
    }

    // Assert: Gestures should be detected simultaneously
    if gesture_types.len() > 1 {
        let simultaneous_count = detected_gestures
            .iter()
            .filter(|g| g.is_simultaneous())
            .count();
        assert!(
            simultaneous_count > 0,
            "Should detect at least some simultaneous gestures"
        );
    }
}

/// Test: Velocity-based physics for gestures
/// This will FAIL initially - need velocity calculation system
#[rstest]
#[case::slow_drag(50.0, 0.1)] // 50px/s over 0.1s
#[case::medium_drag(200.0, 0.2)] // 200px/s over 0.2s
#[case::fast_drag(800.0, 0.05)] // 800px/s over 0.05s
#[wasm_bindgen_test]
fn test_gesture_velocity_calculations(#[case] distance_px: f64, #[case] duration_s: f64) {
    // Arrange: Create gesture recognizer with velocity tracking
    let mut recognizer = GestureRecognizer::new();
    recognizer.enable_velocity_tracking(true);

    let start_pos = TouchPosition { x: 0.0, y: 0.0 };
    let end_pos = TouchPosition {
        x: distance_px,
        y: 0.0,
    };
    let expected_velocity = distance_px / duration_s;

    // Act: Simulate drag gesture with known velocity
    let drag_result =
        recognizer.simulate_drag_gesture(start_pos, end_pos, Duration::from_secs_f64(duration_s));

    // Assert: Should calculate velocity correctly
    assert!(drag_result.is_ok(), "Drag gesture should be recognized");

    let gesture = drag_result.unwrap();
    let calculated_velocity = gesture.velocity().magnitude();

    // Allow 5% tolerance for velocity calculation
    let velocity_error = (calculated_velocity - expected_velocity).abs() / expected_velocity;
    assert!(
        velocity_error < 0.05,
        "Velocity calculation error too high: expected {}px/s, got {}px/s ({}% error)",
        expected_velocity,
        calculated_velocity,
        velocity_error * 100.0
    );

    // Assert: Velocity direction should be correct
    let velocity_angle = gesture.velocity().angle_degrees();
    assert!(
        velocity_angle.abs() < 5.0, // Within 5 degrees of horizontal
        "Velocity angle should be horizontal, got {} degrees",
        velocity_angle
    );
}

/// Test: Gesture prediction for smooth animations
/// This will FAIL initially - need predictive gesture system
#[rstest]
#[case::linear_prediction(vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)])]
#[case::curved_prediction(vec![(0.0, 0.0), (10.0, 5.0), (20.0, 15.0)])]
#[case::accelerating_prediction(vec![(0.0, 0.0), (5.0, 0.0), (15.0, 0.0)])]
#[wasm_bindgen_test]
fn test_gesture_prediction(#[case] touch_points: Vec<(f64, f64)>) {
    // Arrange: Create gesture recognizer with prediction enabled
    let mut recognizer = GestureRecognizer::new();
    recognizer.enable_prediction(true);
    recognizer.set_prediction_window_ms(100); // Predict 100ms ahead

    // Convert touch points to positions
    let positions: Vec<TouchPosition> = touch_points
        .iter()
        .map(|(x, y)| TouchPosition { x: *x, y: *y })
        .collect();

    // Act: Process touch sequence and get prediction
    let mut gesture_data = GestureData::new();
    for (i, position) in positions.iter().enumerate() {
        let timestamp = i as f64 * 50.0; // 50ms between points
        gesture_data.add_touch_point(*position, timestamp);
    }

    let prediction = recognizer.predict_gesture_continuation(&gesture_data);

    // Assert: Should provide valid prediction
    assert!(prediction.is_ok(), "Gesture prediction should succeed");

    let predicted_position = prediction.unwrap();

    // Assert: Prediction should be reasonable based on trajectory
    let last_position = positions.last().unwrap();
    let predicted_distance = predicted_position.distance_to(*last_position);

    // Prediction should not be too far from last known position
    assert!(
        predicted_distance < 100.0,
        "Prediction {} is too far from last position {}",
        predicted_position,
        last_position
    );

    // Prediction should not be at the exact same location (unless stationary)
    if positions.len() > 1 {
        let movement = positions[positions.len() - 1].distance_to(positions[positions.len() - 2]);
        if movement > 1.0 {
            // If there was significant movement
            assert!(
                predicted_distance > 1.0,
                "Prediction should show continued movement for non-stationary gestures"
            );
        }
    }
}

/// Test: Custom gesture definitions and recognition
/// This will FAIL initially - need custom gesture system
#[wasm_bindgen_test]
fn test_custom_gesture_definitions() {
    // Arrange: Create gesture recognizer with custom gestures
    let mut recognizer = GestureRecognizer::new();

    // Define custom "heart" gesture (simplified as two curves)
    let heart_gesture = CustomGesture::new("heart")
        .with_stroke_pattern(vec![
            StrokeSegment::curve(
                TouchPosition::new(0.0, 0.0),
                TouchPosition::new(25.0, -25.0),
                TouchPosition::new(50.0, 0.0),
            ),
            StrokeSegment::curve(
                TouchPosition::new(50.0, 0.0),
                TouchPosition::new(75.0, -25.0),
                TouchPosition::new(100.0, 0.0),
            ),
            StrokeSegment::line(
                TouchPosition::new(100.0, 0.0),
                TouchPosition::new(50.0, 50.0),
            ),
        ])
        .with_tolerance(15.0) // 15px tolerance
        .with_min_confidence(0.7);

    recognizer.register_custom_gesture(heart_gesture);

    // Define custom "lightning" gesture (zigzag pattern)
    let lightning_gesture = CustomGesture::new("lightning")
        .with_stroke_pattern(vec![
            StrokeSegment::line(TouchPosition::new(0.0, 0.0), TouchPosition::new(30.0, 40.0)),
            StrokeSegment::line(
                TouchPosition::new(30.0, 40.0),
                TouchPosition::new(10.0, 80.0),
            ),
            StrokeSegment::line(
                TouchPosition::new(10.0, 80.0),
                TouchPosition::new(40.0, 120.0),
            ),
        ])
        .with_tolerance(20.0)
        .with_min_confidence(0.8);

    recognizer.register_custom_gesture(lightning_gesture);

    // Act: Test heart gesture recognition
    let heart_sequence = create_heart_touch_sequence();
    let heart_result = recognizer.recognize_gesture_from_sequence(heart_sequence);

    // Act: Test lightning gesture recognition
    let lightning_sequence = create_lightning_touch_sequence();
    let lightning_result = recognizer.recognize_gesture_from_sequence(lightning_sequence);

    // Assert: Should recognize custom gestures
    assert!(heart_result.is_ok(), "Should recognize heart gesture");
    assert_eq!(heart_result.unwrap().name(), "heart");

    assert!(
        lightning_result.is_ok(),
        "Should recognize lightning gesture"
    );
    assert_eq!(lightning_result.unwrap().name(), "lightning");

    // Assert: Should have appropriate confidence levels
    let heart_confidence = recognizer.get_last_recognition_confidence();
    assert!(
        heart_confidence >= 0.7,
        "Heart gesture confidence should meet minimum threshold"
    );

    let lightning_confidence = recognizer.get_last_recognition_confidence();
    assert!(
        lightning_confidence >= 0.8,
        "Lightning gesture confidence should meet minimum threshold"
    );
}

/// Test: Performance with many simultaneous touch points
/// This will FAIL initially - need optimized multi-touch handling
#[rstest]
#[case::five_fingers(5)]
#[case::ten_fingers(10)]
#[case::twenty_points(20)] // Stress test
#[wasm_bindgen_test]
fn test_multitouch_performance(#[case] touch_count: usize) {
    // Arrange: Create gesture recognizer with many touch points
    let mut recognizer = GestureRecognizer::new();
    let start_time = Instant::now();

    // Create many simultaneous touch points
    let mut touch_points = Vec::new();
    for i in 0..touch_count {
        let angle = (i as f64 / touch_count as f64) * 2.0 * std::f64::consts::PI;
        let x = 100.0 + 50.0 * angle.cos();
        let y = 100.0 + 50.0 * angle.sin();
        touch_points.push(TouchPosition::new(x, y));
    }

    // Act: Process all touch points simultaneously
    let mut processed_count = 0;
    for touch_point in touch_points {
        let touch_event = TouchEvent::new(processed_count, touch_point, TouchEventType::Move);
        if recognizer.process_touch_event(touch_event).is_ok() {
            processed_count += 1;
        }
    }

    let processing_duration = start_time.elapsed();

    // Assert: Should process all touches quickly
    assert_eq!(
        processed_count, touch_count,
        "Should process all {} touch points",
        touch_count
    );

    // Assert: Performance should be acceptable
    let max_duration = Duration::from_millis(50); // 50ms max for multi-touch
    assert!(
        processing_duration < max_duration,
        "Multi-touch processing took too long: {}ms (max {}ms) for {} touches",
        processing_duration.as_millis(),
        max_duration.as_millis(),
        touch_count
    );

    // Assert: Memory usage should be reasonable
    let memory_stats = recognizer.get_memory_stats();
    let max_memory_per_touch = 1024; // 1KB per touch point
    assert!(
        memory_stats.estimated_bytes < touch_count * max_memory_per_touch,
        "Memory usage too high: {} bytes for {} touches (max {} bytes per touch)",
        memory_stats.estimated_bytes,
        touch_count,
        max_memory_per_touch
    );
}

/// Test: Gesture animation integration with engine
/// This will FAIL initially - need gesture-animation integration
#[wasm_bindgen_test]
fn test_gesture_animation_integration() {
    // Arrange: Create integrated gesture-animation system
    let mut recognizer = GestureRecognizer::new();
    let engine = AnimationEngine::new();

    // Set up gesture-triggered animations
    recognizer.bind_gesture_to_animation(
        "swipe_right",
        AnimationConfig {
            id: Some("swipe_animation".to_string()),
            target: create_slide_animation_target(100.0, 0.0),
            duration: Some(0.3),
            ease: crate::Easing::EaseOut,
            delay: None,
            repeat: crate::RepeatConfig::None,
        },
    );

    recognizer.bind_gesture_to_animation(
        "pinch_out",
        AnimationConfig {
            id: Some("scale_animation".to_string()),
            target: create_scale_animation_target(1.5),
            duration: Some(0.4),
            ease: crate::Easing::Spring(SpringConfig::default()),
            delay: None,
            repeat: crate::RepeatConfig::None,
        },
    );

    // Act: Simulate gesture and trigger animation
    let swipe_gesture = recognizer
        .simulate_swipe_gesture(
            TouchPosition::new(0.0, 50.0),
            TouchPosition::new(100.0, 50.0),
            Duration::from_millis(200),
        )
        .expect("Swipe gesture should be recognized");

    let animation_handle = engine
        .trigger_gesture_animation(&swipe_gesture)
        .expect("Should trigger animation from gesture");

    // Assert: Animation should be started by gesture
    assert!(
        animation_handle.is_active(),
        "Gesture-triggered animation should be active"
    );

    // Assert: Animation should have correct properties
    let animation_id = engine.get_animation_id(animation_handle).unwrap();
    assert_eq!(animation_id, "swipe_animation");

    // Act: Simulate pinch gesture
    let pinch_gesture = recognizer
        .simulate_pinch_gesture(
            vec![
                TouchPosition::new(50.0, 50.0),
                TouchPosition::new(52.0, 48.0),
            ], // Start close
            vec![
                TouchPosition::new(25.0, 25.0),
                TouchPosition::new(75.0, 75.0),
            ], // End far
            Duration::from_millis(300),
        )
        .expect("Pinch gesture should be recognized");

    let scale_handle = engine
        .trigger_gesture_animation(&pinch_gesture)
        .expect("Should trigger scale animation from pinch");

    // Assert: Both animations can run simultaneously
    assert!(animation_handle.is_active());
    assert!(scale_handle.is_active());
    assert_ne!(animation_handle, scale_handle);
}

// ============================================================================
// Helper Types and Functions for Gesture Tests
// ============================================================================

/// Gesture recognition system for complex multi-touch input
#[derive(Debug)]
pub struct GestureRecognizer {
    enabled_gestures: Vec<String>,
    velocity_tracking: bool,
    prediction_enabled: bool,
    prediction_window_ms: u64,
    custom_gestures: HashMap<String, CustomGesture>,
    last_confidence: f64,
    gesture_animations: HashMap<String, AnimationConfig>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self {
            enabled_gestures: Vec::new(),
            velocity_tracking: false,
            prediction_enabled: false,
            prediction_window_ms: 50,
            custom_gestures: HashMap::new(),
            last_confidence: 0.0,
            gesture_animations: HashMap::new(),
        }
    }

    pub fn enable_gesture(&mut self, gesture_type: &str) {
        if !self.enabled_gestures.contains(&gesture_type.to_string()) {
            self.enabled_gestures.push(gesture_type.to_string());
        }
    }

    pub fn enable_velocity_tracking(&mut self, enabled: bool) {
        self.velocity_tracking = enabled;
    }

    pub fn enable_prediction(&mut self, enabled: bool) {
        self.prediction_enabled = enabled;
    }

    pub fn set_prediction_window_ms(&mut self, window_ms: u64) {
        self.prediction_window_ms = window_ms;
    }

    pub fn register_custom_gesture(&mut self, gesture: CustomGesture) {
        self.custom_gestures.insert(gesture.name.clone(), gesture);
    }

    pub fn bind_gesture_to_animation(&mut self, gesture_name: &str, animation: AnimationConfig) {
        self.gesture_animations
            .insert(gesture_name.to_string(), animation);
    }

    pub fn process_touch_event(&mut self, _event: TouchEvent) -> Result<Gesture, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Gesture recognition not yet implemented".to_string(),
        ))
    }

    pub fn simulate_drag_gesture(
        &mut self,
        _start: TouchPosition,
        _end: TouchPosition,
        _duration: Duration,
    ) -> Result<Gesture, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Drag simulation not yet implemented".to_string(),
        ))
    }

    pub fn simulate_swipe_gesture(
        &mut self,
        _start: TouchPosition,
        _end: TouchPosition,
        _duration: Duration,
    ) -> Result<Gesture, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Swipe simulation not yet implemented".to_string(),
        ))
    }

    pub fn simulate_pinch_gesture(
        &mut self,
        _start_points: Vec<TouchPosition>,
        _end_points: Vec<TouchPosition>,
        _duration: Duration,
    ) -> Result<Gesture, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Pinch simulation not yet implemented".to_string(),
        ))
    }

    pub fn predict_gesture_continuation(
        &self,
        _gesture_data: &GestureData,
    ) -> Result<TouchPosition, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Gesture prediction not yet implemented".to_string(),
        ))
    }

    pub fn recognize_gesture_from_sequence(
        &mut self,
        _sequence: Vec<TouchPosition>,
    ) -> Result<CustomGesture, GestureError> {
        // Red Phase: Mock implementation
        Err(GestureError::NotImplemented(
            "Custom gesture recognition not yet implemented".to_string(),
        ))
    }

    pub fn get_last_recognition_confidence(&self) -> f64 {
        self.last_confidence
    }

    pub fn get_memory_stats(&self) -> GestureMemoryStats {
        GestureMemoryStats {
            estimated_bytes: self.enabled_gestures.len() * 256 + self.custom_gestures.len() * 512,
        }
    }
}

/// Touch position with x/y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPosition {
    pub x: f64,
    pub y: f64,
}

impl TouchPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: TouchPosition) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::fmt::Display for TouchPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

/// Gesture velocity with magnitude and direction
#[derive(Debug, Clone)]
pub struct GestureVelocity {
    pub dx_per_sec: f64,
    pub dy_per_sec: f64,
}

impl GestureVelocity {
    pub fn new(dx_per_sec: f64, dy_per_sec: f64) -> Self {
        Self {
            dx_per_sec,
            dy_per_sec,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.dx_per_sec.powi(2) + self.dy_per_sec.powi(2)).sqrt()
    }

    pub fn angle_degrees(&self) -> f64 {
        self.dy_per_sec.atan2(self.dx_per_sec).to_degrees()
    }
}

/// Recognized gesture with type and properties
#[derive(Debug, Clone)]
pub struct Gesture {
    gesture_type: String,
    velocity: GestureVelocity,
    is_simultaneous: bool,
}

impl Gesture {
    pub fn gesture_type(&self) -> &str {
        &self.gesture_type
    }

    pub fn velocity(&self) -> &GestureVelocity {
        &self.velocity
    }

    pub fn is_simultaneous(&self) -> bool {
        self.is_simultaneous
    }
}

/// Custom gesture definition for recognition
#[derive(Debug, Clone)]
pub struct CustomGesture {
    pub name: String,
    stroke_pattern: Vec<StrokeSegment>,
    tolerance: f64,
    min_confidence: f64,
}

impl CustomGesture {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stroke_pattern: Vec::new(),
            tolerance: 10.0,
            min_confidence: 0.6,
        }
    }

    pub fn with_stroke_pattern(mut self, pattern: Vec<StrokeSegment>) -> Self {
        self.stroke_pattern = pattern;
        self
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn with_min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = confidence;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Stroke segment for custom gesture patterns
#[derive(Debug, Clone)]
pub enum StrokeSegment {
    Line {
        start: TouchPosition,
        end: TouchPosition,
    },
    Curve {
        start: TouchPosition,
        control: TouchPosition,
        end: TouchPosition,
    },
}

impl StrokeSegment {
    pub fn line(start: TouchPosition, end: TouchPosition) -> Self {
        Self::Line { start, end }
    }

    pub fn curve(start: TouchPosition, control: TouchPosition, end: TouchPosition) -> Self {
        Self::Curve {
            start,
            control,
            end,
        }
    }
}

/// Touch event data
#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub id: usize,
    pub position: TouchPosition,
    pub event_type: TouchEventType,
}

impl TouchEvent {
    pub fn new(id: usize, position: TouchPosition, event_type: TouchEventType) -> Self {
        Self {
            id,
            position,
            event_type,
        }
    }
}

/// Touch event types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchEventType {
    Start,
    Move,
    End,
}

/// Gesture data for tracking and prediction
#[derive(Debug)]
pub struct GestureData {
    touch_points: Vec<(TouchPosition, f64)>, // (position, timestamp)
}

impl GestureData {
    pub fn new() -> Self {
        Self {
            touch_points: Vec::new(),
        }
    }

    pub fn add_touch_point(&mut self, position: TouchPosition, timestamp: f64) {
        self.touch_points.push((position, timestamp));
    }
}

/// Memory statistics for gesture recognition
#[derive(Debug)]
pub struct GestureMemoryStats {
    pub estimated_bytes: usize,
}

/// Gesture system errors
#[derive(Debug)]
pub enum GestureError {
    NotImplemented(String),
    InvalidInput(String),
    RecognitionFailed(String),
}

// Helper functions for tests
fn create_multi_touch_sequence(gesture_types: &[&str]) -> Vec<TouchEvent> {
    // Mock multi-touch sequence for tests
    vec![
        TouchEvent::new(0, TouchPosition::new(50.0, 50.0), TouchEventType::Start),
        TouchEvent::new(1, TouchPosition::new(60.0, 40.0), TouchEventType::Start),
        TouchEvent::new(0, TouchPosition::new(55.0, 55.0), TouchEventType::Move),
        TouchEvent::new(1, TouchPosition::new(65.0, 45.0), TouchEventType::Move),
    ]
}

fn create_heart_touch_sequence() -> Vec<TouchPosition> {
    // Mock heart-shaped gesture sequence
    vec![
        TouchPosition::new(0.0, 0.0),
        TouchPosition::new(25.0, -25.0),
        TouchPosition::new(50.0, 0.0),
        TouchPosition::new(75.0, -25.0),
        TouchPosition::new(100.0, 0.0),
        TouchPosition::new(50.0, 50.0),
    ]
}

fn create_lightning_touch_sequence() -> Vec<TouchPosition> {
    // Mock lightning-shaped gesture sequence
    vec![
        TouchPosition::new(0.0, 0.0),
        TouchPosition::new(30.0, 40.0),
        TouchPosition::new(10.0, 80.0),
        TouchPosition::new(40.0, 120.0),
    ]
}

fn create_slide_animation_target(x: f64, y: f64) -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target
        .values
        .insert("x".to_string(), AnimationValue::Pixels(x));
    target
        .values
        .insert("y".to_string(), AnimationValue::Pixels(y));
    target
}

fn create_scale_animation_target(scale: f64) -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target
        .values
        .insert("scale".to_string(), AnimationValue::Number(scale));
    target
}

// Extension methods for AnimationEngine to support gesture integration
impl AnimationEngine {
    pub fn trigger_gesture_animation(
        &self,
        _gesture: &Gesture,
    ) -> Result<AnimationHandle, AnimationError> {
        Err(AnimationError::NotImplemented(
            "Gesture-triggered animations not yet implemented".to_string(),
        ))
    }

    pub fn get_animation_id(&self, _handle: AnimationHandle) -> Result<String, AnimationError> {
        Err(AnimationError::NotImplemented(
            "Animation ID lookup not yet implemented".to_string(),
        ))
    }
}

// Mock SpringConfig for compilation
struct SpringConfig;
impl SpringConfig {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gesture_recognizer_creation() {
        let recognizer = GestureRecognizer::new();
        assert!(recognizer.enabled_gestures.is_empty());
        assert!(!recognizer.velocity_tracking);
        assert!(!recognizer.prediction_enabled);
    }

    #[test]
    fn test_touch_position_distance() {
        let pos1 = TouchPosition::new(0.0, 0.0);
        let pos2 = TouchPosition::new(3.0, 4.0);
        assert_eq!(pos1.distance_to(pos2), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_gesture_velocity() {
        let velocity = GestureVelocity::new(3.0, 4.0);
        assert_eq!(velocity.magnitude(), 5.0);
        assert!((velocity.angle_degrees() - 53.13).abs() < 0.1); // atan2(4,3) in degrees
    }

    #[test]
    fn test_custom_gesture_builder() {
        let gesture = CustomGesture::new("test")
            .with_tolerance(15.0)
            .with_min_confidence(0.8);

        assert_eq!(gesture.name(), "test");
        assert_eq!(gesture.tolerance, 15.0);
        assert_eq!(gesture.min_confidence, 0.8);
    }
}
