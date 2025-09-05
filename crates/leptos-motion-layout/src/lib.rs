//! Leptos Motion Layout
//! 
//! Layout animation system providing FLIP animations, shared element transitions,
//! and layout change detection for smooth UI transitions.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod flip;
pub mod shared_elements;
pub mod layout_tracker;
pub mod simplified_layout_api;

// Re-export main types
pub use flip::{FLIPAnimator, FLIPAnimation, FLIPState, TransformValues, EasingFunction};
pub use shared_elements::{SharedElementManager, SharedElementConfig, ZIndexStrategy};
pub use layout_tracker::{LayoutTracker, LayoutChange, LayoutChangeType, PerformanceImpact};

// Re-export simplified layout API (new public API)
pub use simplified_layout_api::{
    SimplifiedLayoutManager, SimplifiedLayoutConfig, SimplifiedEasing,
    SimplifiedAnimationStatus, SimplifiedPerformanceMetrics
};

/// Layout information for FLIP animations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LayoutInfo {
    /// X position
    pub x: f64,
    /// Y position
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

impl Default for LayoutInfo {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl LayoutInfo {
    /// Create new layout info
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    /// Create layout info from dimensions
    pub fn from_dimensions(width: f64, height: f64) -> Self {
        Self::new(0.0, 0.0, width, height)
    }
    
    /// Create layout info from position
    pub fn from_position(x: f64, y: f64) -> Self {
        Self::new(x, y, 0.0, 0.0)
    }
    
    /// Get area of layout
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    /// Get center point
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
    
    /// Check if point is inside layout
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= (self.x + self.width) && y >= self.y && y <= (self.y + self.height)
    }
}

/// Layout animation configuration
#[derive(Debug, Clone)]
pub struct LayoutAnimationConfig {
    /// Whether layout animations are enabled
    pub enabled: bool,
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: EasingFunction,
    /// Whether to use hardware acceleration
    pub hardware_accelerated: bool,
}

impl Default for LayoutAnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 0.3,
            easing: EasingFunction::default(),
            hardware_accelerated: true,
        }
    }
}

impl LayoutAnimationConfig {
    /// Create new config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set duration
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }
    
    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
    
    /// Enable hardware acceleration
    pub fn hardware_accelerated(mut self, enabled: bool) -> Self {
        self.hardware_accelerated = enabled;
        self
    }
    
    /// Enable/disable animations
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_info_new() {
        let info = LayoutInfo::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(info.x, 10.0);
        assert_eq!(info.y, 20.0);
        assert_eq!(info.width, 100.0);
        assert_eq!(info.height, 200.0);
    }

    #[test]
    fn test_layout_info_default() {
        let info = LayoutInfo::default();
        assert_eq!(info.x, 0.0);
        assert_eq!(info.y, 0.0);
        assert_eq!(info.width, 0.0);
        assert_eq!(info.height, 0.0);
    }

    #[test]
    fn test_layout_info_from_dimensions() {
        let info = LayoutInfo::from_dimensions(100.0, 200.0);
        assert_eq!(info.x, 0.0);
        assert_eq!(info.y, 0.0);
        assert_eq!(info.width, 100.0);
        assert_eq!(info.height, 200.0);
    }

    #[test]
    fn test_layout_info_from_position() {
        let info = LayoutInfo::from_position(10.0, 20.0);
        assert_eq!(info.x, 10.0);
        assert_eq!(info.y, 20.0);
        assert_eq!(info.width, 0.0);
        assert_eq!(info.height, 0.0);
    }

    #[test]
    fn test_layout_info_area() {
        let info = LayoutInfo::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(info.area(), 200.0);
    }

    #[test]
    fn test_layout_info_center() {
        let info = LayoutInfo::new(10.0, 20.0, 100.0, 200.0);
        let (cx, cy) = info.center();
        assert_eq!(cx, 60.0); // 10 + 100/2
        assert_eq!(cy, 120.0); // 20 + 200/2
    }

    #[test]
    fn test_layout_info_contains_point() {
        let info = LayoutInfo::new(10.0, 20.0, 100.0, 200.0);
        
        // Inside
        assert!(info.contains_point(50.0, 100.0));
        assert!(info.contains_point(10.0, 20.0)); // Top-left corner
        assert!(info.contains_point(110.0, 220.0)); // Bottom-right corner
        
        // Outside
        assert!(!info.contains_point(5.0, 100.0)); // Left of bounds
        assert!(!info.contains_point(115.0, 100.0)); // Right of bounds
        assert!(!info.contains_point(50.0, 15.0)); // Above bounds
        assert!(!info.contains_point(50.0, 225.0)); // Below bounds
    }

    #[test]
    fn test_layout_animation_config_default() {
        let config = LayoutAnimationConfig::default();
        assert!(config.enabled);
        assert_eq!(config.duration, 0.3);
        assert!(config.hardware_accelerated);
    }

    #[test]
    fn test_layout_animation_config_new() {
        let config = LayoutAnimationConfig::new();
        assert!(config.enabled);
        assert_eq!(config.duration, 0.3);
    }

    #[test]
    fn test_layout_animation_config_builder() {
        let config = LayoutAnimationConfig::new()
            .with_duration(0.5)
            .hardware_accelerated(false)
            .enabled(false);
            
        assert!(!config.enabled);
        assert_eq!(config.duration, 0.5);
        assert!(!config.hardware_accelerated);
    }

    #[test]
    fn test_layout_animation_config_with_easing() {
        let config = LayoutAnimationConfig::new()
            .with_easing(EasingFunction::Linear);
            
        match config.easing {
            EasingFunction::Linear => {},
            _ => panic!("Expected Linear easing function"),
        }
    }
}

// Include simplified layout tests
#[cfg(test)]
mod simplified_layout_tests {
    include!("simplified_layout_tests.rs");
}