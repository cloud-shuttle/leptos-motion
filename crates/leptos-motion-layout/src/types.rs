//! Core types for layout animations

use serde::{Deserialize, Serialize};

/// Layout animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutAnimationConfig {
    /// Animation duration in milliseconds
    pub duration: f64,
    /// Easing function for the animation
    pub easing: String,
    /// Animation delay in milliseconds
    pub delay: f64,
    /// Stagger delay between elements in milliseconds
    pub stagger: f64,
    /// Whether to use FLIP algorithm
    pub use_flip: bool,
    /// Shared element transition settings
    pub shared_elements: SharedElementConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
    /// Layout preset to use
    pub preset: Option<String>,
}

/// Shared element configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedElementConfig {
    /// Whether to enable shared element transitions
    pub enabled: bool,
    /// Transition duration for shared elements
    pub duration: f64,
    /// Easing for shared element transitions
    pub easing: String,
    /// Whether to maintain aspect ratio
    pub maintain_aspect_ratio: bool,
    /// Z-index management strategy
    pub z_index_strategy: ZIndexStrategy,
}

/// Performance configuration for layout animations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Whether to use hardware acceleration
    pub hardware_acceleration: bool,
    /// Frame budget for layout calculations
    pub frame_budget_ms: f64,
    /// Whether to batch layout updates
    pub batch_updates: bool,
    /// Memory pool size for layout objects
    pub memory_pool_size: usize,
}

/// Z-index management strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZIndexStrategy {
    /// Maintain original z-index
    Maintain,
    /// Elevate during transition
    Elevate,
    /// Use custom z-index
    Custom(i32),
}

impl Default for LayoutAnimationConfig {
    fn default() -> Self {
        Self {
            duration: 300.0,
            easing: "ease-in-out".to_string(),
            delay: 0.0,
            stagger: 0.0,
            use_flip: true,
            shared_elements: SharedElementConfig::default(),
            performance: PerformanceConfig::default(),
            preset: None,
        }
    }
}

impl Default for SharedElementConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 400.0,
            easing: "cubic-bezier(0.4, 0.0, 0.2, 1)".to_string(),
            maintain_aspect_ratio: true,
            z_index_strategy: ZIndexStrategy::Elevate,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            frame_budget_ms: 16.67, // 60 FPS
            batch_updates: true,
            memory_pool_size: 100,
        }
    }
}

impl Default for ZIndexStrategy {
    fn default() -> Self {
        Self::Maintain
    }
}
