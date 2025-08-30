//! Leptos Motion Core
//! 
//! Core animation engine providing hybrid RAF/WAAPI animation system,
//! spring physics, easing functions, and motion value management.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod animation;
pub mod easing;
pub mod engine;
pub mod interpolation;
pub mod math;
pub mod spring;
pub mod time;
pub mod types;
pub mod values;

// Re-export core types
pub use animation::{AnimationBuilder, AnimationConfig, Variants};
pub use easing::EasingFn;
pub use engine::{AnimationEngine, HybridEngine, WaapiEngine, RafEngine, PlaybackState};
pub use interpolation::Interpolate;
pub use math::{clamp, map_range, distance_2d, smooth_step, smoother_step};
pub use spring::{SpringSimulator, SpringState};
pub use time::Timer;
pub use types::{
    AnimationHandle, AnimationValue, AnimationTarget, Transition,
    Transform, ComplexValue, SpringConfig, RepeatConfig, StaggerConfig, StaggerFrom, Easing
};
pub use values::{MotionValue, MotionNumber, MotionTransform, MotionValues};

/// Result type for animation operations
pub type Result<T> = std::result::Result<T, AnimationError>;

/// Core animation error types
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    /// Animation engine not available
    #[error("Animation engine not available: {0}")]
    EngineUnavailable(String),
    
    /// Invalid animation property
    #[error("Invalid animation property: {property}")]
    InvalidProperty { property: String },
    
    /// Animation already running
    #[error("Animation already running with handle: {handle:?}")]
    AlreadyRunning { handle: AnimationHandle },
    
    /// Animation not found
    #[error("Animation not found: {handle:?}")]
    NotFound { handle: AnimationHandle },
    
    /// DOM operation failed
    #[error("DOM operation failed: {0}")]
    DomError(String),
    
    /// Mathematical error (division by zero, invalid range, etc.)
    #[error("Math error: {0}")]
    MathError(String),
}