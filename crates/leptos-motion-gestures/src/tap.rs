//! Tap gesture implementation

use crate::{GestureEvent, GestureResult, GestureHandler};
use std::time::{Duration, Instant};

/// Tap gesture handler
pub struct TapGesture {
    /// Whether tap is active
    pub active: bool,
    /// Start position
    start_position: Option<(f64, f64)>,
    /// End position
    end_position: Option<(f64, f64)>,
    /// Start time
    start_time: Option<Instant>,
    /// End time
    end_time: Option<Instant>,
    /// Maximum tap duration
    max_duration: Duration,
    /// Maximum tap distance
    max_distance: f64,
    /// Tap count for multi-tap
    tap_count: u32,
    /// Last tap time for multi-tap detection
    last_tap_time: Option<Instant>,
    /// Multi-tap timeout
    multi_tap_timeout: Duration,
    /// Tap callbacks
    on_single_tap: Option<Box<dyn Fn((f64, f64)) + Send + Sync>>,
    on_double_tap: Option<Box<dyn Fn((f64, f64)) + Send + Sync>>,
    on_triple_tap: Option<Box<dyn Fn((f64, f64)) + Send + Sync>>,
}

/// Tap type
#[derive(Debug, Clone, PartialEq)]
pub enum TapType {
    /// Single tap
    Single,
    /// Double tap
    Double,
    /// Triple tap
    Triple,
    /// Long press
    LongPress,
}

impl TapGesture {
    /// Create new tap gesture
    pub fn new() -> Self {
        Self {
            active: false,
            start_position: None,
            end_position: None,
            start_time: None,
            end_time: None,
            max_duration: Duration::from_millis(300),
            max_distance: 10.0,
            tap_count: 0,
            last_tap_time: None,
            multi_tap_timeout: Duration::from_millis(500),
            on_single_tap: None,
            on_double_tap: None,
            on_triple_tap: None,
        }
    }

    /// Set maximum tap duration
    pub fn max_duration(mut self, duration: Duration) -> Self {
        self.max_duration = duration;
        self
    }

    /// Set maximum tap distance
    pub fn max_distance(mut self, distance: f64) -> Self {
        self.max_distance = distance.max(0.0);
        self
    }

    /// Set multi-tap timeout
    pub fn multi_tap_timeout(mut self, timeout: Duration) -> Self {
        self.multi_tap_timeout = timeout;
        self
    }

    /// Set single tap callback
    pub fn on_single_tap<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_single_tap = Some(Box::new(callback));
        self
    }

    /// Set double tap callback
    pub fn on_double_tap<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_double_tap = Some(Box::new(callback));
        self
    }

    /// Set triple tap callback
    pub fn on_triple_tap<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + Send + Sync + 'static,
    {
        self.on_triple_tap = Some(Box::new(callback));
        self
    }

    /// Get tap count
    pub fn get_tap_count(&self) -> u32 {
        self.tap_count
    }

    /// Get tap duration
    pub fn get_duration(&self) -> Option<Duration> {
        if let (Some(start_time), Some(end_time)) = (self.start_time, self.end_time) {
            Some(end_time.duration_since(start_time))
        } else {
            None
        }
    }

    /// Get tap distance
    pub fn get_distance(&self) -> Option<f64> {
        if let (Some(start_pos), Some(end_pos)) = (self.start_position, self.end_position) {
            let dx = end_pos.0 - start_pos.0;
            let dy = end_pos.1 - start_pos.1;
            Some((dx * dx + dy * dy).sqrt())
        } else {
            None
        }
    }

    /// Check if tap is valid (within duration and distance limits)
    fn is_valid_tap(&self) -> bool {
        if let (Some(duration), Some(distance)) = (self.get_duration(), self.get_distance()) {
            duration <= self.max_duration && distance <= self.max_distance
        } else {
            false
        }
    }

    /// Check if this is a multi-tap continuation
    fn is_multi_tap_continuation(&self) -> bool {
        if let Some(last_tap) = self.last_tap_time {
            if let Some(end_time) = self.end_time {
                end_time.duration_since(last_tap) <= self.multi_tap_timeout
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Update tap count and trigger appropriate callbacks
    fn update_tap_count(&mut self) {
        if self.is_valid_tap() {
            if self.is_multi_tap_continuation() {
                self.tap_count += 1;
            } else {
                self.tap_count = 1;
            }

            self.last_tap_time = self.end_time;

            // Trigger appropriate callback
            if let Some(end_pos) = self.end_position {
                match self.tap_count {
                    1 => {
                        if let Some(ref callback) = self.on_single_tap {
                            callback(end_pos);
                        }
                    }
                    2 => {
                        if let Some(ref callback) = self.on_double_tap {
                            callback(end_pos);
                        }
                    }
                    3 => {
                        if let Some(ref callback) = self.on_triple_tap {
                            callback(end_pos);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            // Invalid tap, reset count
            self.tap_count = 0;
        }
    }

    /// Get the detected tap type
    pub fn get_tap_type(&self) -> Option<TapType> {
        if !self.is_valid_tap() {
            return None;
        }

        match self.tap_count {
            1 => Some(TapType::Single),
            2 => Some(TapType::Double),
            3 => Some(TapType::Triple),
            _ => None,
        }
    }
}

impl GestureHandler for TapGesture {
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult {
        match event {
            GestureEvent::TouchStart { touches } => {
                if let Some(touch) = touches.first() {
                    self.start_position = Some((touch.x, touch.y));
                    self.start_time = Some(Instant::now());
                    self.active = true;
                }
            }
            GestureEvent::TouchMove { touches } => {
                if self.active {
                    if let Some(touch) = touches.first() {
                        // Update end position during move
                        self.end_position = Some((touch.x, touch.y));
                    }
                }
            }
            GestureEvent::TouchEnd { touches } => {
                if self.active {
                    if let Some(touch) = touches.first() {
                        self.end_position = Some((touch.x, touch.y));
                        self.end_time = Some(Instant::now());
                        
                        // Process the tap
                        self.update_tap_count();
                        
                        self.active = false;
                    }
                }
            }
            _ => {}
        }

        GestureResult {
            recognized: self.is_valid_tap(),
            gesture_type: crate::MultiTouchGestureType::None, // Tap is not a multi-touch gesture
            data: None,
            confidence: if self.is_valid_tap() { 0.95 } else { 0.0 },
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn reset(&mut self) {
        self.active = false;
        self.start_position = None;
        self.end_position = None;
        self.start_time = None;
        self.end_time = None;
        self.tap_count = 0;
        self.last_tap_time = None;
    }
}

impl Default for TapGesture {
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
    fn test_tap_gesture_creation() {
        let tap = TapGesture::new();
        assert!(!tap.active);
        assert_eq!(tap.get_tap_count(), 0);
        assert!(tap.get_tap_type().is_none());
    }

    #[test]
    fn test_tap_gesture_configuration() {
        let tap = TapGesture::new()
            .max_duration(Duration::from_millis(500))
            .max_distance(20.0)
            .multi_tap_timeout(Duration::from_millis(1000));
        
        assert_eq!(tap.max_duration, Duration::from_millis(500));
        assert_eq!(tap.max_distance, 20.0);
        assert_eq!(tap.multi_tap_timeout, Duration::from_millis(1000));
    }

    #[test]
    fn test_tap_gesture_validation() {
        let mut tap = TapGesture::new()
            .max_duration(Duration::from_millis(100))
            .max_distance(5.0);
        
        // Test valid tap
        tap.start_position = Some((0.0, 0.0));
        tap.end_position = Some((2.0, 2.0));
        tap.start_time = Some(Instant::now());
        tap.end_time = Some(Instant::now() + Duration::from_millis(50));
        
        assert!(tap.is_valid_tap());
        
        // Test invalid tap (too far)
        tap.end_position = Some((10.0, 10.0));
        assert!(!tap.is_valid_tap());
    }
}