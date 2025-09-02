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
pub mod performance;
pub mod spring;
pub mod time;
pub mod types;
pub mod values;

// Re-export animation presets
pub use animation::presets::AnimationPresets;
pub use animation::presets::SlideDirection;
pub use animation::presets::springs;
pub use animation::presets::easings;

// Re-export core types
pub use animation::{AnimationBuilder, AnimationConfig, Variants};
pub use easing::EasingFn;
pub use engine::{AnimationEngine, OptimizedHybridEngine, WaapiEngine, RafEngine, PlaybackState};
pub use interpolation::Interpolate;
pub use math::{clamp, map_range, distance_2d, smooth_step, smoother_step};
pub use spring::{SpringSimulator, SpringState};
pub use time::Timer;
pub use types::{
    AnimationHandle, AnimationValue, AnimationTarget, Transition,
    Transform, ComplexValue, SpringConfig, RepeatConfig, StaggerConfig, StaggerFrom, Easing
};
pub use values::{MotionValue, MotionNumber, MotionTransform, MotionValues};

// Note: Error handling types are defined in this file, not re-exported

/// Result type for animation operations
pub type Result<T> = std::result::Result<T, AnimationError>;

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// The operation that was performed
    pub operation: String,
    /// The component where the operation occurred
    pub component: Option<String>,
    /// When the operation occurred
    pub timestamp: std::time::Instant,
    /// Additional context information
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            component: None,
            timestamp: std::time::Instant::now(),
            additional_info: std::collections::HashMap::new(),
        }
    }
    
    /// Add component information
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        self.component = Some(component.into());
        self
    }
    
    /// Add additional information
    pub fn with_info(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_info.insert(key.into(), value.into());
        self
    }
}

/// Error recovery strategies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecoveryStrategy {
    /// Retry the operation
    Retry,
    /// Use fallback configuration
    Fallback,
    /// Skip the operation
    Skip,
    /// Abort the animation
    Abort,
}

/// Error handler for animation operations
pub trait ErrorHandler {
    /// Handle an animation error
    fn handle_error(&self, error: &AnimationError, context: &ErrorContext) -> RecoveryStrategy;
    
    /// Log error for debugging
    fn log_error(&self, error: &AnimationError, context: &ErrorContext);
    
    /// Get user-friendly error message
    fn user_message(&self, error: &AnimationError) -> String;
}

/// Default error handler implementation
#[derive(Debug, Clone)]
pub struct DefaultErrorHandler {
    /// Whether to log errors to console
    pub log_errors: bool,
    /// Whether to show user-friendly error messages
    pub show_user_messages: bool,
}

impl Default for DefaultErrorHandler {
    fn default() -> Self {
        Self {
            log_errors: true,
            show_user_messages: false,
        }
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &AnimationError, context: &ErrorContext) -> RecoveryStrategy {
        // Log the error if enabled
        if self.log_errors {
            self.log_error(error, context);
        }
        
        // Determine recovery strategy based on error type
        match error {
            AnimationError::EngineUnavailable(_) => RecoveryStrategy::Fallback,
            AnimationError::InvalidProperty { property: _ } => RecoveryStrategy::Skip,
            AnimationError::AlreadyRunning { handle: _ } => RecoveryStrategy::Skip,
            AnimationError::NotFound { handle: _ } => RecoveryStrategy::Abort,
            AnimationError::DomError(_) => RecoveryStrategy::Retry,
            AnimationError::MathError(_) => RecoveryStrategy::Skip,
            AnimationError::PerformanceBudgetExceeded(_) => RecoveryStrategy::Fallback,
            AnimationError::InvalidConfig(_) => RecoveryStrategy::Fallback,
            AnimationError::MemoryError(_) => RecoveryStrategy::Abort,
            AnimationError::TimingError(_) => RecoveryStrategy::Retry,
        }
    }
    
    fn log_error(&self, error: &AnimationError, context: &ErrorContext) {
        eprintln!("Animation Error: {:?}", error);
        eprintln!("Context: {:?}", context);
    }
    
    fn user_message(&self, error: &AnimationError) -> String {
        if !self.show_user_messages {
            return String::new();
        }
        
        match error {
            AnimationError::EngineUnavailable(_) => "Animation system temporarily unavailable".to_string(),
            AnimationError::InvalidProperty { property: _ } => "Invalid animation property".to_string(),
            AnimationError::AlreadyRunning { handle: _ } => "Animation already in progress".to_string(),
            AnimationError::NotFound { handle: _ } => "Animation not found".to_string(),
            AnimationError::DomError(_) => "Animation display error".to_string(),
            AnimationError::MathError(_) => "Animation calculation error".to_string(),
            AnimationError::PerformanceBudgetExceeded(_) => "Animation performance limit reached".to_string(),
            AnimationError::InvalidConfig(_) => "Invalid animation configuration".to_string(),
            AnimationError::MemoryError(_) => "Animation memory error".to_string(),
            AnimationError::TimingError(_) => "Animation timing error".to_string(),
        }
    }
}

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
    
    /// Performance budget exceeded
    #[error("Performance budget exceeded: {0}")]
    PerformanceBudgetExceeded(String),
    
    /// Invalid animation configuration
    #[error("Invalid animation configuration: {0}")]
    InvalidConfig(String),
    
    /// Memory allocation failed
    #[error("Memory allocation failed: {0}")]
    MemoryError(String),
    
    /// Animation timing error
    #[error("Animation timing error: {0}")]
    TimingError(String),
}