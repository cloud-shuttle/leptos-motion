//! Core animation engine traits and types

use std::collections::HashMap;
use crate::{AnimationHandle, AnimationValue, Result, Transition};
use web_sys;

/// Core animation engine trait
pub trait AnimationEngine {
    /// Check if this engine is available in current environment
    fn is_available(&self) -> bool;

    /// Start an animation and return a handle
    fn animate(&mut self, animation: &AnimationConfig) -> Result<AnimationHandle>;

    /// Stop an animation by handle
    fn stop(&mut self, handle: AnimationHandle) -> Result<()>;

    /// Pause an animation
    fn pause(&mut self, handle: AnimationHandle) -> Result<()>;

    /// Resume a paused animation
    fn resume(&mut self, handle: AnimationHandle) -> Result<()>;

    /// Update all animations (for RAF-based engines)
    fn tick(&mut self, timestamp: f64) -> Result<()>;

    /// Get current playback state
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState>;

    /// Check if an animation is running
    fn is_running(&self, handle: AnimationHandle) -> bool;

    /// Get performance metrics
    #[cfg(feature = "performance-metrics")]
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport>;

    /// Get performance metrics (no-op when feature disabled)
    #[cfg(not(feature = "performance-metrics"))]
    fn get_performance_metrics(&self) -> Option<()>;
}

/// Configuration for an animation
pub struct AnimationConfig {
    /// Target element to animate
    pub element: web_sys::Element,
    /// Animation values to animate to
    pub values: HashMap<String, AnimationValue>,
    /// Animation transition settings
    pub transition: Transition,
    /// Whether to use hardware acceleration
    pub hardware_accelerated: bool,
    /// Animation priority (for performance optimization)
    pub priority: AnimationPriority,
    /// Callback for animation completion
    pub on_complete: Option<Box<dyn Fn() + Send + Sync>>,
    /// Callback for animation progress
    pub on_progress: Option<Box<dyn Fn(f64) + Send + Sync>>,
}

impl Clone for AnimationConfig {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            values: self.values.clone(),
            transition: self.transition.clone(),
            hardware_accelerated: self.hardware_accelerated,
            priority: self.priority,
            on_complete: None, // Can't clone function pointers
            on_progress: None, // Can't clone function pointers
        }
    }
}

/// Animation priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnimationPriority {
    /// Low priority - can be throttled or delayed
    Low,
    /// Normal priority - standard processing
    Normal,
    /// High priority - should be processed immediately
    High,
    /// Critical priority - must be processed immediately
    Critical,
}

/// Playback state of an animation
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    /// Animation is not started
    Idle,
    /// Animation is pending start
    Pending,
    /// Animation is currently running
    Running,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
    /// Animation has finished (alias for Completed)
    Finished,
    /// Animation was cancelled
    Cancelled,
    /// Animation encountered an error
    Error(String),
}
