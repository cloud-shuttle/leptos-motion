//! Leptos Motion Layout
//! 
//! Layout animation system providing FLIP animations, shared element transitions,
//! and layout change detection for smooth UI transitions.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod flip;
pub mod shared_elements;
pub mod layout_tracker;

// Re-export main types
pub use flip::{FLIPAnimator, FLIPAnimation, FLIPState, TransformValues, EasingFunction};
pub use shared_elements::{SharedElementManager, SharedElementConfig, ZIndexStrategy};
pub use layout_tracker::{LayoutTracker, LayoutChange, LayoutChangeType, PerformanceImpact};

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

/// Layout animation configuration
#[derive(Debug, Clone, Default)]
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