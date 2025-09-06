//! Drag gesture implementation

use crate::{GestureEvent, GestureHandler, GestureResult};
use std::time::{Duration, Instant};

/// Drag gesture handler
pub struct DragGesture {
    /// Whether drag is active
    pub active: bool,
    /// Start position
    start_position: Option<(f64, f64)>,
    /// Current position
    current_position: Option<(f64, f64)>,
    /// Start time
    start_time: Option<Instant>,
    /// Last update time
    last_update: Option<Instant>,
    /// Drag threshold
    threshold: f64,
    /// Drag velocity
    velocity: (f64, f64),
    /// Drag direction
    direction: DragDirection,
}

/// Drag direction
#[derive(Debug, Clone, PartialEq)]
pub enum DragDirection {
    /// No direction
    None,
    /// Horizontal drag
    Horizontal,
    /// Vertical drag
    Vertical,
    /// Diagonal drag
    Diagonal,
}

impl DragGesture {
    /// Create new drag gesture
    pub fn new() -> Self {
        Self {
            active: false,
            start_position: None,
            current_position: None,
            start_time: None,
            last_update: None,
            threshold: 10.0,
            velocity: (0.0, 0.0),
            direction: DragDirection::None,
        }
    }

    /// Set drag threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.max(0.0);
        self
    }

    /// Get drag delta (current - start)
    pub fn get_delta(&self) -> (f64, f64) {
        match (self.start_position, self.current_position) {
            (Some((start_x, start_y)), Some((current_x, current_y))) => {
                (current_x - start_x, current_y - start_y)
            }
            _ => (0.0, 0.0),
        }
    }

    /// Get drag velocity
    pub fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }

    /// Get drag direction
    pub fn get_direction(&self) -> &DragDirection {
        &self.direction
    }

    /// Check if drag exceeds threshold
    pub fn exceeds_threshold(&self) -> bool {
        let (delta_x, delta_y) = self.get_delta();
        (delta_x * delta_x + delta_y * delta_y).sqrt() > self.threshold
    }

    /// Update drag direction based on current movement
    fn update_direction(&mut self) {
        let (delta_x, delta_y) = self.get_delta();
        let abs_x = delta_x.abs();
        let abs_y = delta_y.abs();

        self.direction = if abs_x > abs_y * 2.0 {
            DragDirection::Horizontal
        } else if abs_y > abs_x * 2.0 {
            DragDirection::Vertical
        } else {
            DragDirection::Diagonal
        };
    }

    /// Calculate drag velocity
    fn calculate_velocity(&mut self) {
        if let (Some(start_time), Some(last_update), Some(start_pos), Some(current_pos)) = (
            self.start_time,
            self.last_update,
            self.start_position,
            self.current_position,
        ) {
            let duration = last_update.duration_since(start_time);
            if duration > Duration::from_millis(0) {
                let delta_x = current_pos.0 - start_pos.0;
                let delta_y = current_pos.1 - start_pos.1;
                let duration_secs = duration.as_secs_f64();

                self.velocity = (delta_x / duration_secs, delta_y / duration_secs);
            }
        }
    }
}

impl GestureHandler for DragGesture {
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult {
        match event {
            GestureEvent::TouchStart { touches } => {
                if let Some(touch) = touches.first() {
                    self.start_position = Some((touch.x, touch.y));
                    self.current_position = Some((touch.x, touch.y));
                    self.start_time = Some(Instant::now());
                    self.last_update = Some(Instant::now());
                    self.active = true;
                    self.velocity = (0.0, 0.0);
                    self.direction = DragDirection::None;
                }
            }
            GestureEvent::TouchMove { touches } => {
                if self.active {
                    if let Some(touch) = touches.first() {
                        self.current_position = Some((touch.x, touch.y));
                        self.last_update = Some(Instant::now());

                        if self.exceeds_threshold() {
                            self.update_direction();
                            self.calculate_velocity();
                        }
                    }
                }
            }
            GestureEvent::TouchEnd { touches: _ } => {
                if self.active {
                    // Calculate final velocity
                    self.calculate_velocity();
                    self.active = false;
                }
            }
            _ => {}
        }

        GestureResult {
            recognized: self.active && self.exceeds_threshold(),
            gesture_type: crate::MultiTouchGestureType::None, // Drag is not a multi-touch gesture
            data: None,
            confidence: if self.exceeds_threshold() { 0.8 } else { 0.0 },
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn reset(&mut self) {
        self.active = false;
        self.start_position = None;
        self.current_position = None;
        self.start_time = None;
        self.last_update = None;
        self.velocity = (0.0, 0.0);
        self.direction = DragDirection::None;
    }
}

impl Default for DragGesture {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TouchPoint;

    fn create_touch_point(x: f64, y: f64) -> TouchPoint {
        TouchPoint {
            id: 1,
            x,
            y,
            pressure: 1.0,
            timestamp: 0,
        }
    }

    #[test]
    fn test_drag_gesture_creation() {
        let drag = DragGesture::new();
        assert!(!drag.active);
        assert_eq!(drag.get_delta(), (0.0, 0.0));
        assert_eq!(drag.get_direction(), &DragDirection::None);
    }

    #[test]
    fn test_drag_gesture_threshold() {
        let drag = DragGesture::new().threshold(20.0);
        assert_eq!(drag.threshold, 20.0);
    }

    #[test]
    fn test_drag_gesture_handling() {
        let mut drag = DragGesture::new().threshold(5.0);

        // Start drag
        let start_event = GestureEvent::TouchStart {
            touches: vec![create_touch_point(100.0, 100.0)],
        };
        let result = drag.handle_gesture(start_event);
        assert!(drag.active);
        assert!(!result.recognized); // Not yet above threshold

        // Move drag
        let move_event = GestureEvent::TouchMove {
            touches: vec![create_touch_point(110.0, 110.0)],
        };
        let result = drag.handle_gesture(move_event);
        assert!(drag.active);
        assert!(result.recognized); // Now above threshold

        // End drag
        let end_event = GestureEvent::TouchEnd {
            touches: vec![create_touch_point(110.0, 110.0)],
        };
        let _result = drag.handle_gesture(end_event);
        assert!(!drag.active);
    }
}
