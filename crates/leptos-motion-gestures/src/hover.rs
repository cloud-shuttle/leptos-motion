//! Hover gesture implementation

use crate::{GestureEvent, GestureHandler, GestureResult};
use std::time::{Duration, Instant};

/// Type alias for hover callback functions
type HoverCallback = Option<Box<dyn Fn((f64, f64)) + Send + Sync>>;

/// Hover gesture handler
pub struct HoverGesture {
    /// Whether hover is active
    pub active: bool,
    /// Current position
    current_position: Option<(f64, f64)>,
    /// Hover start time
    start_time: Option<Instant>,
    /// Hover duration threshold
    duration_threshold: Duration,
    /// Hover area bounds
    bounds: Option<((f64, f64), (f64, f64))>,
    /// Hover callbacks
    on_enter: HoverCallback,
    on_leave: HoverCallback,
    on_move: HoverCallback,
}

impl HoverGesture {
    /// Create new hover gesture
    pub fn new() -> Self {
        Self {
            active: false,
            current_position: None,
            start_time: None,
            duration_threshold: Duration::from_millis(100),
            bounds: None,
            on_enter: None,
            on_leave: None,
            on_move: None,
        }
    }

    /// Set hover duration threshold
    pub fn duration_threshold(mut self, duration: Duration) -> Self {
        self.duration_threshold = duration;
        self
    }

    /// Set hover area bounds
    pub fn bounds(mut self, min: (f64, f64), max: (f64, f64)) -> Self {
        self.bounds = Some((min, max));
        self
    }

    /// Set hover enter callback
    pub fn on_enter<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_enter = Some(Box::new(callback));
        self
    }

    /// Set hover leave callback
    pub fn on_leave<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_leave = Some(Box::new(callback));
        self
    }

    /// Set hover move callback
    pub fn on_move<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_move = Some(Box::new(callback));
        self
    }

    /// Get current hover position
    pub fn get_position(&self) -> Option<(f64, f64)> {
        self.current_position
    }

    /// Get hover duration
    pub fn get_duration(&self) -> Duration {
        if let (Some(start_time), Some(_)) = (self.start_time, self.current_position) {
            start_time.elapsed()
        } else {
            Duration::from_millis(0)
        }
    }

    /// Check if position is within bounds
    fn is_within_bounds(&self, position: (f64, f64)) -> bool {
        if let Some(((min_x, min_y), (max_x, max_y))) = self.bounds {
            position.0 >= min_x && position.0 <= max_x && position.1 >= min_y && position.1 <= max_y
        } else {
            true // No bounds set, always within
        }
    }

    /// Check if hover has exceeded duration threshold
    fn has_exceeded_threshold(&self) -> bool {
        self.get_duration() >= self.duration_threshold
    }
}

impl GestureHandler for HoverGesture {
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult {
        match event {
            GestureEvent::TouchStart { touches } => {
                if let Some(touch) = touches.first() {
                    let position = (touch.x, touch.y);

                    if self.is_within_bounds(position) {
                        self.current_position = Some(position);
                        self.start_time = Some(Instant::now());

                        // Trigger enter callback
                        if let Some(ref callback) = self.on_enter {
                            callback(position);
                        }
                    }
                }
            }
            GestureEvent::TouchMove { touches } => {
                if let Some(touch) = touches.first() {
                    let position = (touch.x, touch.y);

                    if self.is_within_bounds(position) {
                        let was_active = self.active;
                        self.current_position = Some(position);

                        if !was_active && self.has_exceeded_threshold() {
                            self.active = true;
                        }

                        // Trigger move callback
                        if let Some(ref callback) = self.on_move {
                            callback(position);
                        }
                    } else {
                        // Position is outside bounds
                        if self.active {
                            self.active = false;

                            // Trigger leave callback
                            if let Some(ref callback) = self.on_leave
                                && let Some(current_pos) = self.current_position {
                                callback(current_pos);
                            }
                        }
                        self.current_position = None;
                        self.start_time = None;
                    }
                }
            }
            GestureEvent::TouchEnd { touches: _ } => {
                if self.active {
                    self.active = false;

                    // Trigger leave callback
                    if let Some(ref callback) = self.on_leave
                        && let Some(current_pos) = self.current_position {
                        callback(current_pos);
                    }
                }
                self.current_position = None;
                self.start_time = None;
            }
            _ => {}
        }

        GestureResult {
            recognized: self.active,
            gesture_type: crate::MultiTouchGestureType::None, // Hover is not a multi-touch gesture
            data: None,
            confidence: if self.active { 0.9 } else { 0.0 },
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn reset(&mut self) {
        self.active = false;
        self.current_position = None;
        self.start_time = None;
    }
}

impl Default for HoverGesture {
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
    fn test_hover_gesture_creation() {
        let hover = HoverGesture::new();
        assert!(!hover.active);
        assert!(hover.get_position().is_none());
        assert_eq!(hover.get_duration(), Duration::from_millis(0));
    }

    #[test]
    fn test_hover_gesture_bounds() {
        let hover = HoverGesture::new().bounds((0.0, 0.0), (100.0, 100.0));
        assert!(hover.is_within_bounds((50.0, 50.0)));
        assert!(!hover.is_within_bounds((150.0, 150.0)));
    }

    #[test]
    fn test_hover_gesture_duration_threshold() {
        let hover = HoverGesture::new().duration_threshold(Duration::from_millis(200));
        assert_eq!(hover.duration_threshold, Duration::from_millis(200));
    }
}
