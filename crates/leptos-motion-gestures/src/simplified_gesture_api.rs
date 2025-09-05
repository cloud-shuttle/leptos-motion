//! Simplified Gesture API
//! 
//! This module provides a simplified, user-friendly gesture API
//! that provides a clean interface for gesture handling.

use crate::*;
use std::time::Instant;

/// Simplified gesture types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimplifiedGestureType {
    /// No gesture detected
    None,
    /// Pinch/zoom gesture
    Pinch,
    /// Rotation gesture
    Rotation,
    /// Pan/drag gesture
    Pan,
    /// Multi-touch gesture (combination of pinch, rotation, pan)
    MultiTouch,
}

/// Simplified gesture configuration
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedGestureConfig {
    /// Maximum number of touches to track
    pub max_touches: usize,
    /// Minimum distance for gesture detection
    pub min_distance: f64,
    /// Gesture timeout in milliseconds
    pub timeout: u64,
    /// Enable pinch gesture detection
    pub enable_pinch: bool,
    /// Enable rotation gesture detection
    pub enable_rotation: bool,
    /// Enable pan gesture detection
    pub enable_pan: bool,
}

/// Simplified gesture result
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedGestureResult {
    /// Gesture type
    pub gesture_type: SimplifiedGestureType,
    /// Scale factor (for pinch gestures)
    pub scale: Option<f64>,
    /// Rotation angle in degrees (for rotation gestures)
    pub rotation: Option<f64>,
    /// Translation vector (for pan gestures)
    pub translation: Option<SimplifiedVector2D>,
    /// Velocity vector
    pub velocity: Option<SimplifiedVector2D>,
    /// Gesture center point
    pub center: Option<SimplifiedVector2D>,
    /// Gesture confidence (0.0 to 1.0)
    pub confidence: f64,
    /// Gesture duration in milliseconds
    pub duration: u64,
}

/// Simplified 2D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimplifiedVector2D {
    pub x: f64,
    pub y: f64,
}

/// Simplified gesture bounds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimplifiedGestureBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

/// Simplified gesture data
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedGestureData {
    pub gesture_type: SimplifiedGestureType,
    pub touch_count: usize,
    pub is_active: bool,
    pub center: Option<SimplifiedVector2D>,
    pub bounds: Option<SimplifiedGestureBounds>,
    pub distance: Option<f64>,
    pub angle: Option<f64>,
    pub confidence: f64,
    pub duration: u64,
}

/// Simplified gesture detector that provides a clean, simple interface
/// 
/// This is the main public API for gesture detection. It provides
/// a clean, simple interface while hiding the complexity of the
/// underlying multi-touch gesture detection system.
pub struct SimplifiedGestureDetector {
    /// Internal multi-touch detector (hidden from public API)
    internal_detector: MultiTouchGestureDetector,
    /// Current gesture state
    current_gesture: SimplifiedGestureType,
    /// Gesture start time
    gesture_start: Option<Instant>,
    /// Last gesture result
    last_result: SimplifiedGestureResult,
}

impl SimplifiedGestureDetector {
    /// Create a new simplified gesture detector with default configuration
    pub fn new() -> Self {
        Self::with_config(SimplifiedGestureConfig::default())
    }
    
    /// Create a new simplified gesture detector with custom configuration
    pub fn with_config(config: SimplifiedGestureConfig) -> Self {
        let internal_config = GestureConfig {
            basic_gestures: true,
            multi_touch: true,
            pinch_to_zoom: config.enable_pinch,
            rotation: config.enable_rotation,
            sensitivity: 0.5,
            min_distance: config.min_distance,
            max_touches: config.max_touches,
            timeout_ms: config.timeout,
        };
        
        Self {
            internal_detector: MultiTouchGestureDetector::new(internal_config),
            current_gesture: SimplifiedGestureType::None,
            gesture_start: None,
            last_result: SimplifiedGestureResult::default(),
        }
    }
    
    /// Handle touch start event
    pub fn handle_touch_start(&mut self, touches: Vec<TouchPoint>) -> SimplifiedGestureResult {
        let result = self.internal_detector.handle_touch_start(touches);
        self.current_gesture = self.convert_gesture_type(result.gesture_type.clone());
        
        if self.current_gesture != SimplifiedGestureType::None {
            self.gesture_start = Some(Instant::now());
        }
        
        self.last_result = self.create_simplified_result(result);
        self.last_result.clone()
    }
    
    /// Handle touch move event
    pub fn handle_touch_move(&mut self, touches: Vec<TouchPoint>) -> SimplifiedGestureResult {
        let result = self.internal_detector.handle_touch_move(touches);
        self.current_gesture = self.convert_gesture_type(result.gesture_type.clone());
        
        self.last_result = self.create_simplified_result(result);
        self.last_result.clone()
    }
    
    /// Handle touch end event
    pub fn handle_touch_end(&mut self, touch_ids: Vec<u64>) -> SimplifiedGestureResult {
        // Create empty touch points for the touch IDs that ended
        let touches: Vec<TouchPoint> = touch_ids.into_iter().map(|id| TouchPoint {
            id,
            x: 0.0,
            y: 0.0,
            pressure: 0.0,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        }).collect();
        
        let result = self.internal_detector.handle_touch_end(touches);
        self.current_gesture = SimplifiedGestureType::None;
        self.gesture_start = None;
        
        self.last_result = self.create_simplified_result(result);
        self.last_result.clone()
    }
    
    /// Cancel current gesture
    pub fn cancel(&mut self) {
        self.internal_detector.reset();
        self.current_gesture = SimplifiedGestureType::None;
        self.gesture_start = None;
        self.last_result = SimplifiedGestureResult::default();
    }
    
    /// Reset gesture detector
    pub fn reset(&mut self) {
        self.internal_detector.reset();
        self.current_gesture = SimplifiedGestureType::None;
        self.gesture_start = None;
        self.last_result = SimplifiedGestureResult::default();
    }
    
    /// Check if a gesture is currently active
    pub fn is_active(&self) -> bool {
        self.internal_detector.is_active()
    }
    
    /// Get the number of active touches
    pub fn touch_count(&self) -> usize {
        self.internal_detector.get_state().touches.len()
    }
    
    /// Get the current gesture type
    pub fn gesture_type(&self) -> SimplifiedGestureType {
        self.current_gesture
    }
    
    /// Get gesture data
    pub fn get_gesture_data(&self) -> Option<SimplifiedGestureData> {
        if !self.is_active() {
            return None;
        }
        
        Some(SimplifiedGestureData {
            gesture_type: self.current_gesture,
            touch_count: self.touch_count(),
            is_active: self.is_active(),
            center: self.get_center(),
            bounds: self.get_bounds(),
            distance: self.get_distance(),
            angle: self.get_angle(),
            confidence: self.get_confidence(),
            duration: self.get_duration(),
        })
    }
    
    /// Get gesture confidence
    pub fn get_confidence(&self) -> f64 {
        // For now, return a default confidence since it's not available in MultiTouchState
        0.8
    }
    
    /// Get gesture center point
    pub fn get_center(&self) -> Option<SimplifiedVector2D> {
        if self.touch_count() < 2 {
            return None;
        }
        
        let state = self.internal_detector.get_state();
        let touches: Vec<&TouchPoint> = state.touches.values().collect();
        
        if touches.len() < 2 {
            return None;
        }
        
        let center_x = touches.iter().map(|t| t.x).sum::<f64>() / touches.len() as f64;
        let center_y = touches.iter().map(|t| t.y).sum::<f64>() / touches.len() as f64;
        
        Some(SimplifiedVector2D { x: center_x, y: center_y })
    }
    
    /// Get gesture bounds
    pub fn get_bounds(&self) -> Option<SimplifiedGestureBounds> {
        if self.touch_count() < 2 {
            return None;
        }
        
        let state = self.internal_detector.get_state();
        let touches: Vec<&TouchPoint> = state.touches.values().collect();
        
        if touches.is_empty() {
            return None;
        }
        
        let min_x = touches.iter().map(|t| t.x).fold(f64::INFINITY, f64::min);
        let max_x = touches.iter().map(|t| t.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = touches.iter().map(|t| t.y).fold(f64::INFINITY, f64::min);
        let max_y = touches.iter().map(|t| t.y).fold(f64::NEG_INFINITY, f64::max);
        
        Some(SimplifiedGestureBounds {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
    
    /// Get gesture distance
    pub fn get_distance(&self) -> Option<f64> {
        if self.touch_count() < 2 {
            return None;
        }
        
        let state = self.internal_detector.get_state();
        let touches: Vec<&TouchPoint> = state.touches.values().collect();
        
        if touches.len() < 2 {
            return None;
        }
        
        let dx = touches[1].x - touches[0].x;
        let dy = touches[1].y - touches[0].y;
        Some((dx * dx + dy * dy).sqrt())
    }
    
    /// Get gesture angle
    pub fn get_angle(&self) -> Option<f64> {
        if self.touch_count() < 2 {
            return None;
        }
        
        let state = self.internal_detector.get_state();
        let touches: Vec<&TouchPoint> = state.touches.values().collect();
        
        if touches.len() < 2 {
            return None;
        }
        
        let dx = touches[1].x - touches[0].x;
        let dy = touches[1].y - touches[0].y;
        Some(dy.atan2(dx).to_degrees())
    }
    
    /// Get gesture duration
    pub fn get_duration(&self) -> u64 {
        if let Some(start) = self.gesture_start {
            start.elapsed().as_millis() as u64
        } else {
            0
        }
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: SimplifiedGestureConfig) {
        let internal_config = GestureConfig {
            basic_gestures: true,
            multi_touch: true,
            pinch_to_zoom: config.enable_pinch,
            rotation: config.enable_rotation,
            sensitivity: 0.5,
            min_distance: config.min_distance,
            max_touches: config.max_touches,
            timeout_ms: config.timeout,
        };
        
        self.internal_detector.update_config(internal_config);
    }
    
    /// Get current configuration
    pub fn get_config(&self) -> SimplifiedGestureConfig {
        // For now, return default config since we can't access internal config
        // This could be improved by storing the config in the simplified detector
        SimplifiedGestureConfig::default()
    }
    
    /// Convert internal gesture type to simplified gesture type
    fn convert_gesture_type(&self, gesture_type: MultiTouchGestureType) -> SimplifiedGestureType {
        match gesture_type {
            MultiTouchGestureType::None => SimplifiedGestureType::None,
            MultiTouchGestureType::Pinch => SimplifiedGestureType::Pinch,
            MultiTouchGestureType::Rotation => SimplifiedGestureType::Rotation,
            MultiTouchGestureType::MultiSwipe => SimplifiedGestureType::Pan,
            MultiTouchGestureType::PinchAndRotate => SimplifiedGestureType::MultiTouch,
            MultiTouchGestureType::MultiTap => SimplifiedGestureType::None,
        }
    }
    
    /// Create simplified gesture result from internal result
    fn create_simplified_result(&self, result: GestureResult) -> SimplifiedGestureResult {
        SimplifiedGestureResult {
            gesture_type: self.convert_gesture_type(result.gesture_type),
            scale: None, // Not available in GestureResult
            rotation: None, // Not available in GestureResult
            translation: None, // Not available in GestureResult
            velocity: None, // Not available in GestureResult
            center: self.get_center(),
            confidence: result.confidence,
            duration: self.get_duration(),
        }
    }
}

impl Default for SimplifiedGestureDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SimplifiedGestureDetector {
    fn clone(&self) -> Self {
        // Create a new detector with the same configuration
        let config = self.get_config();
        Self {
            internal_detector: MultiTouchGestureDetector::new(GestureConfig {
                basic_gestures: true,
                multi_touch: true,
                pinch_to_zoom: config.enable_pinch,
                rotation: config.enable_rotation,
                sensitivity: 0.5,
                min_distance: config.min_distance,
                max_touches: config.max_touches,
                timeout_ms: config.timeout,
            }),
            current_gesture: self.current_gesture,
            gesture_start: self.gesture_start,
            last_result: self.last_result.clone(),
        }
    }
}

impl std::fmt::Debug for SimplifiedGestureDetector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimplifiedGestureDetector")
            .field("is_active", &self.is_active())
            .field("touch_count", &self.touch_count())
            .field("gesture_type", &self.gesture_type())
            .finish()
    }
}

impl Default for SimplifiedGestureConfig {
    fn default() -> Self {
        Self {
            max_touches: 5,
            min_distance: 10.0,
            timeout: 1000,
            enable_pinch: true,
            enable_rotation: true,
            enable_pan: true,
        }
    }
}

impl SimplifiedGestureConfig {
    /// Create new simplified gesture config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum number of touches
    pub fn max_touches(mut self, max_touches: usize) -> Self {
        self.max_touches = max_touches;
        self
    }
    
    /// Set minimum distance for gesture detection
    pub fn min_distance(mut self, min_distance: f64) -> Self {
        self.min_distance = min_distance;
        self
    }
    
    /// Set gesture timeout
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Enable pinch gesture detection
    pub fn enable_pinch(mut self, enable: bool) -> Self {
        self.enable_pinch = enable;
        self
    }
    
    /// Enable rotation gesture detection
    pub fn enable_rotation(mut self, enable: bool) -> Self {
        self.enable_rotation = enable;
        self
    }
    
    /// Enable pan gesture detection
    pub fn enable_pan(mut self, enable: bool) -> Self {
        self.enable_pan = enable;
        self
    }
}

impl Default for SimplifiedGestureResult {
    fn default() -> Self {
        Self {
            gesture_type: SimplifiedGestureType::None,
            scale: None,
            rotation: None,
            translation: None,
            velocity: None,
            center: None,
            confidence: 0.0,
            duration: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simplified_gesture_detector_creation() {
        let detector = SimplifiedGestureDetector::new();
        assert!(!detector.is_active());
        assert_eq!(detector.touch_count(), 0);
        assert_eq!(detector.gesture_type(), SimplifiedGestureType::None);
    }
    
    #[test]
    fn test_simplified_gesture_detector_with_config() {
        let config = SimplifiedGestureConfig::new()
            .max_touches(3)
            .min_distance(5.0);
        
        let detector = SimplifiedGestureDetector::with_config(config);
        assert!(!detector.is_active());
        assert_eq!(detector.touch_count(), 0);
    }
    
    #[test]
    fn test_simplified_gesture_config_fluent_api() {
        let config = SimplifiedGestureConfig::new()
            .max_touches(5)
            .min_distance(10.0)
            .timeout(1000)
            .enable_pinch(true)
            .enable_rotation(false)
            .enable_pan(true);
        
        assert_eq!(config.max_touches, 5);
        assert_eq!(config.min_distance, 10.0);
        assert_eq!(config.timeout, 1000);
        assert!(config.enable_pinch);
        assert!(!config.enable_rotation);
        assert!(config.enable_pan);
    }
    
    #[test]
    fn test_simplified_gesture_detector_clone() {
        let detector1 = SimplifiedGestureDetector::new();
        let detector2 = detector1.clone();
        
        assert_eq!(detector1.is_active(), detector2.is_active());
        assert_eq!(detector1.touch_count(), detector2.touch_count());
        assert_eq!(detector1.gesture_type(), detector2.gesture_type());
    }
    
    #[test]
    fn test_simplified_gesture_detector_debug() {
        let detector = SimplifiedGestureDetector::new();
        let debug_str = format!("{:?}", detector);
        assert!(debug_str.contains("SimplifiedGestureDetector"));
        assert!(debug_str.contains("is_active"));
        assert!(debug_str.contains("touch_count"));
        assert!(debug_str.contains("gesture_type"));
    }
}
