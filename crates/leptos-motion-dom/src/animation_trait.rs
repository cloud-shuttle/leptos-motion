//! Animation Trait
//!
//! This module defines the base Animation trait that all animation types must implement.
//! It provides a clean interface for starting, stopping, and updating animations.

use leptos_motion_core::*;
use std::result::Result;

/// Base trait for all animation types
pub trait Animation {
    /// Start the animation
    fn start(&mut self) -> Result<(), AnimationError>;
    
    /// Stop the animation
    fn stop(&mut self) -> Result<(), AnimationError>;
    
    /// Check if animation is complete
    fn is_complete(&self) -> bool;
    
    /// Get animation progress (0.0 to 1.0)
    fn progress(&self) -> f64;
    
    /// Update animation state (called by animation manager)
    fn update(&mut self, delta_time: f64) -> Result<(), AnimationError>;
    
    /// Get animation ID
    fn id(&self) -> &str;
    
    /// Get animation duration in seconds
    fn duration(&self) -> f64;
    
    /// Check if animation is currently running
    fn is_running(&self) -> bool;
}

/// Animation state for tracking progress
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    /// Animation is ready to start
    Ready,
    /// Animation is currently running
    Running,
    /// Animation has completed
    Complete,
    /// Animation was stopped
    Stopped,
    /// Animation encountered an error
    Error(String),
}

/// Animation configuration
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// Animation ID
    pub id: String,
    /// Animation duration in seconds
    pub duration: f64,
    /// Animation delay in seconds
    pub delay: f64,
    /// Animation easing function
    pub easing: Easing,
    /// Whether animation should repeat
    pub repeat: bool,
    /// Number of times to repeat (None = infinite)
    pub repeat_count: Option<usize>,
    /// Whether animation should reverse on repeat
    pub reverse: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            duration: 0.3,
            delay: 0.0,
            easing: Easing::EaseInOut,
            repeat: false,
            repeat_count: None,
            reverse: false,
        }
    }
}

/// Animation result type
pub type AnimationResult<T> = Result<T, AnimationError>;

/// Animation error types
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationError {
    /// Animation not found
    NotFound(String),
    /// Animation already running
    AlreadyRunning(String),
    /// Animation not running
    NotRunning(String),
    /// Invalid animation configuration
    InvalidConfig(String),
    /// DOM manipulation error
    DomError(String),
    /// Animation engine unavailable
    EngineUnavailable(String),
    /// Animation timeout
    Timeout(String),
    /// Generic error
    Generic(String),
}

impl std::fmt::Display for AnimationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationError::NotFound(id) => write!(f, "Animation not found: {}", id),
            AnimationError::AlreadyRunning(id) => write!(f, "Animation already running: {}", id),
            AnimationError::NotRunning(id) => write!(f, "Animation not running: {}", id),
            AnimationError::InvalidConfig(msg) => write!(f, "Invalid animation config: {}", msg),
            AnimationError::DomError(msg) => write!(f, "DOM error: {}", msg),
            AnimationError::EngineUnavailable(msg) => write!(f, "Animation engine unavailable: {}", msg),
            AnimationError::Timeout(msg) => write!(f, "Animation timeout: {}", msg),
            AnimationError::Generic(msg) => write!(f, "Animation error: {}", msg),
        }
    }
}

impl std::error::Error for AnimationError {}

/// Helper functions for animation calculations
pub mod animation_utils {
    use super::*;
    
    /// Apply easing function to progress value
    pub fn apply_easing(easing: &Easing, t: f64) -> f64 {
        match easing {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            _ => t, // Default to linear for unsupported easing functions
        }
    }
    
    /// Interpolate between two values
    pub fn interpolate(from: f64, to: f64, progress: f64) -> f64 {
        from + (to - from) * progress
    }
    
    /// Clamp value between min and max
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        value.min(max).max(min)
    }
    
    /// Convert easing to CSS easing function
    pub fn easing_to_css(easing: &Easing) -> &'static str {
        match easing {
            Easing::Linear => "linear",
            Easing::EaseIn => "ease-in",
            Easing::EaseOut => "ease-out",
            Easing::EaseInOut => "ease-in-out",
            _ => "ease-in-out",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_config_default() {
        let config = AnimationConfig::default();
        assert_eq!(config.duration, 0.3);
        assert_eq!(config.delay, 0.0);
        assert_eq!(config.easing, Easing::EaseInOut);
        assert!(!config.repeat);
        assert_eq!(config.repeat_count, None);
        assert!(!config.reverse);
    }
    
    #[test]
    fn test_animation_utils() {
        // Test easing functions
        assert_eq!(animation_utils::apply_easing(&Easing::Linear, 0.5), 0.5);
        assert_eq!(animation_utils::apply_easing(&Easing::EaseIn, 0.5), 0.25);
        assert_eq!(animation_utils::apply_easing(&Easing::EaseOut, 0.5), 0.75);
        
        // Test interpolation
        assert_eq!(animation_utils::interpolate(0.0, 1.0, 0.5), 0.5);
        assert_eq!(animation_utils::interpolate(10.0, 20.0, 0.5), 15.0);
        
        // Test clamping
        assert_eq!(animation_utils::clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(animation_utils::clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(animation_utils::clamp(15.0, 0.0, 10.0), 10.0);
        
        // Test CSS easing
        assert_eq!(animation_utils::easing_to_css(&Easing::Linear), "linear");
        assert_eq!(animation_utils::easing_to_css(&Easing::EaseIn), "ease-in");
    }
    
    #[test]
    fn test_animation_error_display() {
        let error = AnimationError::NotFound("test".to_string());
        assert_eq!(format!("{}", error), "Animation not found: test");
        
        let error = AnimationError::InvalidConfig("bad config".to_string());
        assert_eq!(format!("{}", error), "Invalid animation config: bad config");
    }
}
