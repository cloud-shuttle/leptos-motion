//! Leptos Motion Core
//!
//! Core animation engine providing hybrid RAF/WAAPI animation system,
//! spring physics, easing functions, and motion value management.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "advanced-examples")]
pub mod advanced_examples;
#[cfg(feature = "approx")]
pub mod animation;
#[cfg(feature = "developer-tools")]
pub mod developer_tools;
#[cfg(feature = "approx")]
pub mod easing;
#[cfg(feature = "ecosystem-integration")]
pub mod ecosystem_integration;
pub mod engine;
#[cfg(feature = "approx")]
pub mod interpolation;
#[cfg(feature = "approx")]
pub mod math;
#[cfg(feature = "developer-tools")]
pub mod tdd_engine;
pub mod time;
#[cfg(feature = "timeline-animations")]
pub mod timeline;
pub mod types;
pub mod values;

#[macro_use]
pub mod macros;

#[cfg(feature = "futures")]
pub mod lazy_loading;
#[cfg(feature = "memory-optimization")]
pub mod memory_optimization;
pub mod minimal_engine;
#[cfg(feature = "performance-metrics")]
pub mod performance;
pub mod simplified_engine;

// Phase 4: Dependency optimization - minimal serialization
pub mod minimal_serialization;
#[cfg(feature = "approx")]
pub mod spring;

#[cfg(test)]
mod memory_optimization_tests;

#[cfg(test)]
mod basic_functionality_tests;
#[cfg(test)]
mod bundle_size_tests;
#[cfg(test)]
mod dependency_investigation_tests;
#[cfg(test)]
mod dependency_optimization_tests;
#[cfg(test)]
mod error_handling_tdd_tests;
#[cfg(test)]
mod feature_flags_tests;
#[cfg(test)]
mod performance_optimization_tdd_tests;
#[cfg(test)]
mod performance_tests;
#[cfg(test)]
mod serde_replacement_tests;
#[cfg(test)]
mod tree_shaking_tests;
#[cfg(test)]
mod wasm_browser_tdd_tests;
#[cfg(test)]
mod wasm_optimization_tests;

#[cfg(test)]
mod bundle_size_optimization_tests;
#[cfg(test)]
mod dead_code_elimination_tests;
#[cfg(test)]
mod dependency_optimization_phase4_tests;
#[cfg(test)]
mod feature_flags_optimization_tests;
#[cfg(test)]
mod feature_flags_phase3_tests;
#[cfg(test)]
mod tree_shaking_optimization_tests;
#[cfg(test)]
mod wasm_test_setup;
#[cfg(test)]
mod web_sys_optimization_tests;

// Re-export animation presets
#[cfg(feature = "approx")]
pub use animation::presets::AnimationPresets;
#[cfg(feature = "approx")]
pub use animation::presets::SlideDirection;
#[cfg(feature = "approx")]
pub use animation::presets::easings;
#[cfg(feature = "approx")]
pub use animation::presets::springs;

// Re-export core types
#[cfg(feature = "approx")]
pub use animation::{AnimationBuilder, AnimationConfig, Variants};
#[cfg(feature = "approx")]
pub use easing::EasingFn;
#[cfg(feature = "web-sys")]
pub use engine::WaapiEngine;
pub use engine::{AnimationEngine, OptimizedHybridEngine, PlaybackState, RafEngine};
#[cfg(feature = "approx")]
pub use interpolation::Interpolate;
#[cfg(feature = "approx")]
pub use math::{clamp, distance_2d, map_range, smooth_step, smoother_step};
#[cfg(feature = "web-sys")]
pub use time::Timer;
#[cfg(feature = "approx")]
pub use types::SpringConfig;
pub use types::{
    AnimationHandle, AnimationTarget, AnimationValue, ComplexValue, Easing, RepeatConfig,
    StaggerConfig, StaggerFrom, Transform, Transition,
};

// Re-export Leptos v0.8 compatibility helpers
#[cfg(feature = "leptos-integration")]
pub use types::leptos_helpers::*;
#[cfg(feature = "leptos-integration")]
pub use values::{MotionNumber, MotionTransform, MotionValue, MotionValues};

#[cfg(feature = "futures")]
pub use lazy_loading::{
    AnimationLazyLoader, FeatureModuleLoader, LazyLoadingConfig, LazyModule, get_lazy_loader,
};
pub use minimal_engine::MinimalEngine;
pub use simplified_engine::SimplifiedAnimationEngine;
#[cfg(feature = "approx")]
pub use spring::{SpringSimulator, SpringState};

// Feature-specific re-exports
#[cfg(feature = "performance-metrics")]
pub use performance::*;

#[cfg(feature = "memory-optimization")]
pub use memory_optimization::*;

#[cfg(feature = "lazy-loading")]
pub use lazy_loading::*;

#[cfg(feature = "simplified-engine")]
pub use simplified_engine::*;

#[cfg(feature = "minimal-serialization")]
pub use minimal_serialization::*;

// TDD Engine exports for v1.0 development (conditional for production builds)
#[cfg(feature = "developer-tools")]
pub use tdd_engine::{
    AnimationConfig as TDDAnimationConfig, AnimationEngine as TDDAnimationEngine, MemoryStats,
    TDDAnimationHandle,
};

// Timeline Animation exports for Phase 2 (conditional for production builds)
#[cfg(feature = "timeline-animations")]
pub use timeline::{Timeline, TimelineKeyframe, TimelinePerformanceMetrics};

// Developer Tools exports for Phase 3
#[cfg(feature = "developer-tools")]
pub use developer_tools::*;

// Advanced Examples & Templates exports for Phase 3
#[cfg(feature = "advanced-examples")]
pub use advanced_examples::*;

// Ecosystem Integration exports for Phase 4
#[cfg(feature = "ecosystem-integration")]
pub use ecosystem_integration::*;

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
            AnimationError::NotImplemented(_) => RecoveryStrategy::Abort,
            AnimationError::InvalidValue(_) => RecoveryStrategy::Skip,
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
            AnimationError::EngineUnavailable(_) => {
                "Animation system temporarily unavailable".to_string()
            }
            AnimationError::InvalidProperty { property: _ } => {
                "Invalid animation property".to_string()
            }
            AnimationError::AlreadyRunning { handle: _ } => {
                "Animation already in progress".to_string()
            }
            AnimationError::NotFound { handle: _ } => "Animation not found".to_string(),
            AnimationError::DomError(_) => "Animation display error".to_string(),
            AnimationError::MathError(_) => "Animation calculation error".to_string(),
            AnimationError::PerformanceBudgetExceeded(_) => {
                "Animation performance limit reached".to_string()
            }
            AnimationError::InvalidConfig(_) => "Invalid animation configuration".to_string(),
            AnimationError::MemoryError(_) => "Animation memory error".to_string(),
            AnimationError::TimingError(_) => "Animation timing error".to_string(),
            AnimationError::NotImplemented(_) => "Feature not yet available".to_string(),
            AnimationError::InvalidValue(_) => "Invalid animation value".to_string(),
        }
    }
}

/// Core animation error types
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    /// Animation engine not available
    #[error("Animation engine not available: {0}")]
    EngineUnavailable(String),

    /// Invalid property error
    #[error("Invalid animation property: {property}")]
    InvalidProperty {
        /// The name of the invalid property
        property: String,
    },
    /// Animation already running error
    #[error("Animation already running with handle: {handle:?}")]
    AlreadyRunning {
        /// The handle of the already running animation
        handle: AnimationHandle,
    },
    /// Animation not found error
    #[error("Animation not found: {handle:?}")]
    NotFound {
        /// The handle of the animation that was not found
        handle: AnimationHandle,
    },

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

    /// Feature not yet implemented
    #[error("Feature not yet implemented: {0}")]
    NotImplemented(String),

    /// Invalid animation value (NaN, Infinity, etc.)
    #[error("Invalid animation value: {0}")]
    InvalidValue(String),
}

// Include simplified engine tests
#[cfg(test)]
mod simplified_engine_tests {
    include!("simplified_engine_tests.rs");
}
