//! Multi-touch gesture detection and recognition

use crate::{
    GestureConfig, GestureEvent, GestureHandler, GestureResult, MultiTouchGestureType,
    MultiTouchState, TouchPoint,
};

use std::time::{Duration, Instant};

/// Multi-touch gesture detector
pub struct MultiTouchGestureDetector {
    /// Configuration for gesture detection
    config: GestureConfig,
    /// Current gesture state
    state: MultiTouchState,
    /// Previous gesture state for delta calculations
    previous_state: Option<MultiTouchState>,
    /// Gesture start time
    start_time: Option<Instant>,
    /// Last update time
    last_update: Option<Instant>,
    /// Gesture confidence tracking
    confidence: f64,
    /// Minimum confidence threshold
    min_confidence: f64,
}

impl MultiTouchGestureDetector {
    /// Create a new multi-touch gesture detector
    pub fn new(config: GestureConfig) -> Self {
        Self {
            config,
            state: MultiTouchState::default(),
            previous_state: None,
            start_time: None,
            last_update: None,
            confidence: 0.0,
            min_confidence: 0.3,
        }
    }

    /// Create with default configuration
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(GestureConfig::default())
    }

    /// Update the detector configuration
    pub fn update_config(&mut self, config: GestureConfig) {
        self.config = config;
    }

    /// Process touch start event
    pub fn handle_touch_start(&mut self, touches: Vec<TouchPoint>) -> GestureResult {
        if touches.len() > self.config.max_touches {
            return GestureResult::default();
        }

        // Add new touches
        for touch in touches {
            self.state.touches.insert(touch.id, touch);
        }

        // Initialize gesture state
        if self.state.touches.len() >= 2 {
            self.start_time = Some(Instant::now());
            self.last_update = Some(Instant::now());
            self.state.active = true;
            self.update_gesture_state();
            self.detect_gesture_type();
            // Update confidence after detecting gesture type
            self.update_confidence();
        }

        self.create_gesture_result()
    }

    /// Process touch move event
    pub fn handle_touch_move(&mut self, touches: Vec<TouchPoint>) -> GestureResult {
        if !self.state.active {
            return GestureResult::default();
        }

        // Save current state as previous state BEFORE updating touches
        self.last_update = Some(Instant::now());
        self.previous_state = Some(self.state.clone());

        // Update existing touches
        for touch in &touches {
            if let Some(existing) = self.state.touches.get_mut(&touch.id) {
                *existing = touch.clone();
            }
        }

        // Remove touches that are no longer active
        let active_ids: Vec<u64> = touches.iter().map(|t| t.id).collect();
        self.state.touches.retain(|id, _| active_ids.contains(id));

        if self.state.touches.len() < 2 {
            // Not enough touches for multi-touch gesture
            self.reset();
            return GestureResult::default();
        }

        self.update_gesture_state();
        self.update_confidence();

        self.create_gesture_result()
    }

    /// Process touch end event
    pub fn handle_touch_end(&mut self, touches: Vec<TouchPoint>) -> GestureResult {
        if !self.state.active {
            return GestureResult::default();
        }

        // Remove ended touches
        for touch in touches {
            self.state.touches.remove(&touch.id);
        }

        if self.state.touches.len() < 2 {
            // Complete the gesture
            let result = self.create_gesture_result();
            self.reset();
            result
        } else {
            // Update remaining touches
            self.update_gesture_state();
            self.create_gesture_result()
        }
    }

    /// Update the current gesture state
    fn update_gesture_state(&mut self) {
        if self.state.touches.len() < 2 {
            return;
        }

        // Calculate center point
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for touch in self.state.touches.values() {
            sum_x += touch.x;
            sum_y += touch.y;
        }
        let center_x = sum_x / self.state.touches.len() as f64;
        let center_y = sum_y / self.state.touches.len() as f64;
        self.state.center = (center_x, center_y);

        // Calculate average distance from center
        let mut total_distance = 0.0;
        for touch in self.state.touches.values() {
            let dx = touch.x - center_x;
            let dy = touch.y - center_y;
            total_distance += (dx * dx + dy * dy).sqrt();
        }
        self.state.average_distance = total_distance / self.state.touches.len() as f64;

        // Calculate scale and rotation if we have previous state
        if let Some(prev_state) = &self.previous_state
            && prev_state.touches.len() >= 2 {
            // Calculate scale change
            let scale_change = self.state.average_distance / prev_state.average_distance;
            self.state.scale = scale_change;

            // Calculate rotation change
            if self.state.touches.len() >= 2 {
                let current_angle = self.calculate_gesture_angle();
                let previous_angle = self.calculate_previous_gesture_angle();
                let rotation_change = current_angle - previous_angle;
                self.state.rotation = rotation_change;
            }
        }
    }

    /// Detect the type of gesture being performed
    fn detect_gesture_type(&mut self) {
        if self.state.touches.len() < 2 {
            self.state.gesture_type = MultiTouchGestureType::None;
            return;
        }

        // For initial touch start, we can't detect scale/rotation changes yet
        // but we can set up the gesture type based on configuration
        if self.previous_state.is_none() {
            if self.config.pinch_to_zoom && self.config.rotation {
                self.state.gesture_type = MultiTouchGestureType::PinchAndRotate;
            } else if self.config.pinch_to_zoom {
                self.state.gesture_type = MultiTouchGestureType::Pinch;
            } else if self.config.rotation {
                self.state.gesture_type = MultiTouchGestureType::Rotation;
            } else {
                self.state.gesture_type = MultiTouchGestureType::MultiTap;
            }
            return;
        }

        // Check for pinch gesture
        let has_pinch = self.config.pinch_to_zoom
            && (self.state.scale - 1.0).abs() > self.config.min_distance / 1000.0;

        // Check for rotation gesture
        let has_rotation = self.config.rotation && self.state.rotation.abs() > 0.1; // ~5.7 degrees

        // Determine gesture type
        self.state.gesture_type = match (has_pinch, has_rotation) {
            (true, true) => MultiTouchGestureType::PinchAndRotate,
            (true, false) => MultiTouchGestureType::Pinch,
            (false, true) => MultiTouchGestureType::Rotation,
            (false, false) => {
                // Check for other gesture types
                if self.is_multi_tap() {
                    MultiTouchGestureType::MultiTap
                } else if self.is_multi_swipe() {
                    MultiTouchGestureType::MultiSwipe
                } else {
                    MultiTouchGestureType::None
                }
            }
        };
    }

    /// Calculate the current gesture angle
    fn calculate_gesture_angle(&self) -> f64 {
        if self.state.touches.len() < 2 {
            return 0.0;
        }

        let touches: Vec<&TouchPoint> = self.state.touches.values().collect();
        let (x1, y1) = (touches[0].x, touches[0].y);
        let (x2, y2) = (touches[1].x, touches[1].y);

        (y2 - y1).atan2(x2 - x1)
    }

    /// Calculate the previous gesture angle
    fn calculate_previous_gesture_angle(&self) -> f64 {
        if let Some(prev_state) = &self.previous_state
            && prev_state.touches.len() >= 2 {
            let touches: Vec<&TouchPoint> = prev_state.touches.values().collect();
            let (x1, y1) = (touches[0].x, touches[0].y);
            let (x2, y2) = (touches[1].x, touches[1].y);
            return (y2 - y1).atan2(x2 - x1);
        }
        0.0
    }

    /// Check if this is a multi-tap gesture
    fn is_multi_tap(&self) -> bool {
        if let Some(start_time) = self.start_time {
            if let Some(last_update) = self.last_update {
                let duration = last_update.duration_since(start_time);
                duration < Duration::from_millis(self.config.timeout_ms)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if this is a multi-swipe gesture
    fn is_multi_swipe(&self) -> bool {
        if let Some(start_time) = self.start_time {
            if let Some(last_update) = self.last_update {
                let duration = last_update.duration_since(start_time);
                duration > Duration::from_millis(self.config.timeout_ms)
                    && self.state.average_distance > self.config.min_distance
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Update gesture confidence based on various factors
    fn update_confidence(&mut self) {
        let mut confidence = 0.0;

        // Base confidence on number of touches
        confidence += (self.state.touches.len() as f64 / 5.0) * 0.3;

        // Confidence based on gesture type detection
        match self.state.gesture_type {
            MultiTouchGestureType::Pinch | MultiTouchGestureType::Rotation => {
                confidence += 0.4;
            }
            MultiTouchGestureType::PinchAndRotate => {
                confidence += 0.5;
            }
            MultiTouchGestureType::MultiTap | MultiTouchGestureType::MultiSwipe => {
                confidence += 0.3;
            }
            MultiTouchGestureType::None => {
                confidence += 0.1;
            }
        }

        // Confidence based on movement consistency
        if let Some(prev_state) = &self.previous_state {
            let distance_change = (self.state.average_distance - prev_state.average_distance).abs();
            if distance_change < self.config.min_distance {
                confidence += 0.2;
            }
        }

        self.confidence = confidence.clamp(0.0, 1.0);
    }

    /// Create a gesture result from current state
    fn create_gesture_result(&self) -> GestureResult {
        let recognized = self.state.active && self.confidence >= self.min_confidence;

        // Debug output
        if self.state.active {
            println!(
                "DEBUG: active={}, confidence={:.3}, min_confidence={:.3}, gesture_type={:?}",
                self.state.active, self.confidence, self.min_confidence, self.state.gesture_type
            );
        }

        GestureResult {
            recognized,
            gesture_type: self.state.gesture_type.clone(),
            data: if self.state.active {
                Some(self.state.clone())
            } else {
                None
            },
            confidence: self.confidence,
        }
    }

    /// Get the current gesture state
    pub fn get_state(&self) -> &MultiTouchState {
        &self.state
    }

    /// Get the current scale factor
    pub fn get_scale(&self) -> f64 {
        self.state.scale
    }

    /// Get the current rotation angle
    pub fn get_rotation(&self) -> f64 {
        self.state.rotation
    }

    /// Get the gesture center point
    pub fn get_center(&self) -> (f64, f64) {
        self.state.center
    }

    /// Check if a specific gesture type is active
    pub fn is_gesture_type(&self, gesture_type: MultiTouchGestureType) -> bool {
        self.state.active && self.state.gesture_type == gesture_type
    }
}

impl GestureHandler for MultiTouchGestureDetector {
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult {
        match event {
            GestureEvent::TouchStart { touches } => self.handle_touch_start(touches),
            GestureEvent::TouchMove { touches } => self.handle_touch_move(touches),
            GestureEvent::TouchEnd { touches } => self.handle_touch_end(touches),
            _ => GestureResult::default(),
        }
    }

    fn is_active(&self) -> bool {
        self.state.active
    }

    fn reset(&mut self) {
        self.state = MultiTouchState::default();
        self.previous_state = None;
        self.start_time = None;
        self.last_update = None;
        self.confidence = 0.0;
    }
}

impl Default for MultiTouchGestureDetector {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TouchPoint;

    fn create_touch_point(id: u64, x: f64, y: f64) -> TouchPoint {
        TouchPoint {
            id,
            x,
            y,
            pressure: 1.0,
            timestamp: 0,
        }
    }

    #[test]
    fn test_gesture_detector_creation() {
        let detector = MultiTouchGestureDetector::default();
        assert!(!detector.is_active());
        assert_eq!(detector.get_scale(), 1.0);
        assert_eq!(detector.get_rotation(), 0.0);
    }

    #[test]
    fn test_pinch_gesture_detection() {
        let mut detector = MultiTouchGestureDetector::new(
            GestureConfig::default().basic_only().enable_pinch_to_zoom(),
        );

        // Start with two touches
        let touches = vec![
            create_touch_point(1, 100.0, 100.0),
            create_touch_point(2, 200.0, 200.0),
        ];

        let result = detector.handle_touch_start(touches);
        assert!(result.recognized);
        assert_eq!(result.gesture_type, MultiTouchGestureType::Pinch);

        // Move touches apart (pinch out)
        let touches = vec![
            create_touch_point(1, 50.0, 50.0),
            create_touch_point(2, 250.0, 250.0),
        ];

        let result = detector.handle_touch_move(touches);
        assert!(result.recognized);
        assert!(detector.get_scale() > 1.0);
    }

    #[test]
    fn test_rotation_gesture_detection() {
        let mut detector =
            MultiTouchGestureDetector::new(GestureConfig::default().basic_only().enable_rotation());

        // Start with two touches in a horizontal line
        let touches = vec![
            create_touch_point(1, 100.0, 100.0), // Left point
            create_touch_point(2, 200.0, 100.0), // Right point
        ];

        let result = detector.handle_touch_start(touches);
        assert!(result.recognized);

        // First move - establish previous state (still horizontal)
        let touches = vec![
            create_touch_point(1, 100.0, 150.0), // Left point moved down
            create_touch_point(2, 200.0, 150.0), // Right point moved down
        ];

        let result = detector.handle_touch_move(touches);
        assert!(result.recognized);

        // Second move - create actual rotation by moving left point diagonally
        let touches = vec![
            create_touch_point(1, 150.0, 200.0), // Left point moved diagonally (creates rotation)
            create_touch_point(2, 200.0, 150.0), // Right point stays at same height
        ];

        let result = detector.handle_touch_move(touches);
        assert!(result.recognized);
        assert!(detector.get_rotation().abs() > 0.0);
    }

    #[test]
    fn test_rotation_calculation_debug() {
        let mut detector =
            MultiTouchGestureDetector::new(GestureConfig::default().basic_only().enable_rotation());

        // Start with two touches in a clear horizontal line
        let touches = vec![
            create_touch_point(1, 0.0, 0.0),   // Origin
            create_touch_point(2, 100.0, 0.0), // Horizontal right
        ];

        let result = detector.handle_touch_start(touches);
        assert!(result.recognized);

        // Move to create a 45-degree rotation
        let touches = vec![
            create_touch_point(1, 0.0, 0.0),     // Keep origin fixed
            create_touch_point(2, 100.0, 100.0), // Move to 45 degrees
        ];

        let result = detector.handle_touch_move(touches);
        assert!(result.recognized);

        // The angle from (0,0) to (100,0) is 0 radians
        // The angle from (0,0) to (100,100) is atan2(100,100) = π/4 ≈ 0.785398 radians
        // So rotation should be approximately π/4 ≈ 0.785398
        let expected_rotation = std::f64::consts::PI / 4.0;
        let actual_rotation = detector.get_rotation();

        assert!(
            actual_rotation.abs() > 0.1,
            "Rotation should be significant, got: {}",
            actual_rotation
        );
    }
}

// Include modern TDD tests
#[cfg(test)]
mod tdd_tests {
    include!("multi_touch_tdd_tests.rs");
}
