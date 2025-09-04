//! Advanced gesture recognition for Leptos Motion

#![warn(missing_docs)]

pub mod drag;
pub mod hover;
pub mod tap;
pub mod multi_touch;
pub mod gesture_detector;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Advanced gesture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureConfig {
    /// Basic gestures enabled
    pub basic_gestures: bool,
    /// Multi-touch gestures enabled
    pub multi_touch: bool,
    /// Pinch-to-zoom enabled
    pub pinch_to_zoom: bool,
    /// Rotation gestures enabled
    pub rotation: bool,
    /// Gesture sensitivity (0.0 to 1.0)
    pub sensitivity: f64,
    /// Minimum distance for gesture recognition
    pub min_distance: f64,
    /// Maximum number of simultaneous touches
    pub max_touches: usize,
    /// Gesture timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            basic_gestures: true,
            multi_touch: true,
            pinch_to_zoom: true,
            rotation: true,
            sensitivity: 0.5,
            min_distance: 10.0,
            max_touches: 5,
            timeout_ms: 300,
        }
    }
}

impl GestureConfig {
    /// Enable basic gestures only
    pub fn basic_only(mut self) -> Self {
        self.multi_touch = false;
        self.pinch_to_zoom = false;
        self.rotation = false;
        self
    }

    /// Enable multi-touch gestures
    pub fn enable_multi_touch(mut self) -> Self {
        self.multi_touch = true;
        self
    }

    /// Enable pinch-to-zoom
    pub fn enable_pinch_to_zoom(mut self) -> Self {
        self.pinch_to_zoom = true;
        self.multi_touch = true;
        self
    }

    /// Enable rotation gestures
    pub fn enable_rotation(mut self) -> Self {
        self.rotation = true;
        self.multi_touch = true;
        self
    }

    /// Set gesture sensitivity
    pub fn sensitivity(mut self, sensitivity: f64) -> Self {
        self.sensitivity = sensitivity.clamp(0.0, 1.0);
        self
    }

    /// Set minimum distance threshold
    pub fn min_distance(mut self, distance: f64) -> Self {
        self.min_distance = distance.max(0.0);
        self
    }

    /// Set maximum number of touches
    pub fn max_touches(mut self, max: usize) -> Self {
        self.max_touches = max.max(1);
        self
    }

    /// Set gesture timeout
    pub fn timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

/// Touch point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchPoint {
    /// Unique identifier for the touch
    pub id: u64,
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Pressure (0.0 to 1.0)
    pub pressure: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// Multi-touch gesture state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTouchState {
    /// Active touch points
    pub touches: HashMap<u64, TouchPoint>,
    /// Center point of all touches
    pub center: (f64, f64),
    /// Average distance from center
    pub average_distance: f64,
    /// Scale factor (1.0 = no change)
    pub scale: f64,
    /// Rotation angle in radians
    pub rotation: f64,
    /// Gesture is active
    pub active: bool,
    /// Gesture type
    pub gesture_type: MultiTouchGestureType,
}

/// Types of multi-touch gestures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MultiTouchGestureType {
    /// No gesture detected
    None,
    /// Pinch-to-zoom gesture
    Pinch,
    /// Rotation gesture
    Rotation,
    /// Combined pinch and rotation
    PinchAndRotate,
    /// Multi-finger tap
    MultiTap,
    /// Multi-finger swipe
    MultiSwipe,
}

impl Default for MultiTouchState {
    fn default() -> Self {
        Self {
            touches: HashMap::new(),
            center: (0.0, 0.0),
            average_distance: 0.0,
            scale: 1.0,
            rotation: 0.0,
            active: false,
            gesture_type: MultiTouchGestureType::None,
        }
    }
}

/// Gesture event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GestureEvent {
    /// Touch start event
    TouchStart { 
        /// The touch points that started
        touches: Vec<TouchPoint> 
    },
    /// Touch move event
    TouchMove { 
        /// The touch points that moved
        touches: Vec<TouchPoint> 
    },
    /// Touch end event
    TouchEnd { 
        /// The touch points that ended
        touches: Vec<TouchPoint> 
    },
    /// Gesture recognized event
    GestureRecognized { 
        /// The recognized gesture state
        gesture: MultiTouchState 
    },
    /// Gesture updated event
    GestureUpdated { 
        /// The updated gesture state
        gesture: MultiTouchState 
    },
    /// Gesture completed event
    GestureCompleted { 
        /// The completed gesture state
        gesture: MultiTouchState 
    },
}

/// Gesture recognition result
#[derive(Debug, Clone)]
pub struct GestureResult {
    /// Whether a gesture was recognized
    pub recognized: bool,
    /// The recognized gesture type
    pub gesture_type: MultiTouchGestureType,
    /// Gesture data
    pub data: Option<MultiTouchState>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

impl Default for GestureResult {
    fn default() -> Self {
        Self {
            recognized: false,
            gesture_type: MultiTouchGestureType::None,
            data: None,
            confidence: 0.0,
        }
    }
}

/// Gesture handler trait for custom gesture implementations
pub trait GestureHandler {
    /// Handle gesture events
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult;
    
    /// Check if gesture is active
    fn is_active(&self) -> bool;
    
    /// Reset gesture state
    fn reset(&mut self);
}

/// Re-export commonly used types
pub use multi_touch::MultiTouchGestureDetector;
pub use gesture_detector::GestureDetector;