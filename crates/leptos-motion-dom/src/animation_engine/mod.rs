//! Animation Engine for Leptos Motion
//!
//! This module provides a robust animation engine broken down into
//! focused, maintainable components for different aspects of animation.

pub mod state_management;
pub mod timing_interpolation;
pub mod easing_functions;
pub mod spring_physics;
pub mod memory_safety;
pub mod dom_animation_engine;

// Re-export main types for convenience
pub use state_management::{AnimationState, PropertyAnimation, AnimationStateManager};
pub use timing_interpolation::{TimingUtils, InterpolationUtils, AnimationTimingController};
pub use easing_functions::EasingFunctions;
pub use spring_physics::{SpringConfig, SpringPhysics, SpringAnimationManager};
pub use memory_safety::MemorySafety;
pub use dom_animation_engine::DomAnimationEngine;

// Re-export the core AnimationEngine trait
pub use leptos_motion_core::AnimationEngine;
