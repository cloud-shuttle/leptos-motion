//! Leptos Motion DOM Integration
//!
//! Leptos components and DOM utilities for motion animations

#![warn(missing_docs)]
#![forbid(unsafe_code)]

// Core animation system (Phase 3 - Event-driven architecture)
pub mod animation_trait;
pub mod animation_handle;
pub mod css_transition_animation;
pub mod keyframe_animation;
pub mod stagger_animation;
pub mod spring_animation;
pub mod event_driven_motion_div;
pub mod event_handlers;
pub mod performance_monitor;

// Reactive animation system
pub mod animate_prop;

// Simple animation system (WASM-compatible)
pub mod simple_animation_engine;

// Small, focused components - removed for now
// pub mod simple_components;
// SimpleMotionDiv removed - using single MotionDiv API
pub mod elements;
pub mod motion_path;
pub mod hooks;
pub mod utils;

// Animation engine (refactored into focused modules)
pub mod animation_engine;
pub mod easing_functions;
pub mod repeat_config;
pub mod transform_animations;

// Legacy modules removed - consolidated to single MotionDiv API

// New v0.7 features
pub mod animate_presence;
pub mod performance_optimizations;
pub mod optimized_animation_manager;
pub mod memory_management;
pub mod integrated_memory_manager;
pub mod timeline;
pub mod variants;

// Phase 4: Layout Animations
pub mod layout_animations;

// Phase 5: Shared Layout Transitions
pub mod shared_layout_transitions;

#[cfg(feature = "css-animations")]
pub mod css_animations;

// Test modules
#[cfg(test)]
mod memory_safety_test;
#[cfg(test)]
mod event_driven_tests;
#[cfg(test)]
mod css_animation_integration_test;
#[cfg(test)]
mod animation_engine_test;

// Refactored test modules
#[cfg(test)]
mod tests;

// Re-export core types from leptos-motion-core
pub use leptos_motion_core::{
    AnimationConfig, AnimationEngine, AnimationHandle, AnimationTarget, AnimationValue,
    ComplexValue, Easing, MotionNumber, MotionTransform, MotionValue, MotionValues, RepeatConfig,
    SpringConfig, StaggerConfig, StaggerFrom, Transform, Transition, Variants,
};

// Ensure type consistency across crates
pub type DomTransition = Transition;
pub type DomAnimationValue = AnimationValue;
pub type DomEasing = Easing;

// Re-export event-driven components (Phase 3 - Primary API)
pub use animation_trait::{Animation, AnimationError, AnimationResult, AnimationState, animation_utils};
pub use animation_handle::{DomAnimationHandle as EventAnimationHandle, AnimationManager};
pub use css_transition_animation::CssTransitionAnimation;
// SimpleMotionDiv removed - using single MotionDiv API
pub use keyframe_animation::{KeyframeAnimation, Keyframe};
pub use stagger_animation::{StaggerAnimation, StaggerConfig as EventStaggerConfig, create_stagger_animation};
pub use spring_animation::{SpringAnimation, SpringConfig as EventSpringConfig};
pub use event_driven_motion_div::{
    EventDrivenMotionDiv, 
    AnimationType, 
    DragAxis as EventDragAxis, 
    DragConstraints as EventDragConstraints, 
    create_animation_value as create_event_animation_value, 
    create_animation_target as create_event_animation_target, 
    create_drag_constraints as create_event_drag_constraints
};

// Re-export reactive animation types
pub use animate_prop::{AnimateProp, IntoAnimateProp, resolve_animate_prop};

// Primary MotionDiv API - Single component for all use cases
pub use event_driven_motion_div::EventDrivenMotionDiv as MotionDiv;
pub use event_driven_motion_div::EventDrivenMotionDiv as ReactiveMotionDiv;
pub use event_driven_motion_div::EventDrivenMotionDiv as DragMotionDiv;

// Layout animation types
pub use layout_animations::{
    LayoutConfig,
    LayoutType,
    LayoutAnimationManager,
    LayoutPerformanceMetrics,
};

// Shared layout transition types
pub use shared_layout_transitions::{
    SharedLayoutConfig,
    SharedTransitionType,
    LayoutAnimationConfig,
    SharedElement,
    ElementState,
    SharedElementManager,
    SharedLayoutMetrics,
};

// Variants system
pub use variants::{Variants as AnimationVariants, IntoVariants, VariantsBuilder};

pub use motion_path::MotionPath;

// Legacy compatibility - use MotionDiv instead
// SimpleMotionDiv and CleanMotionDiv are now aliases for MotionDiv
pub use event_handlers::{
    EventHandlerManager, 
    DragEventHandler, 
    HoverEventHandler, 
    TapEventHandler, 
    GestureEventHandler, 
    SwipeDirection
};
pub use performance_monitor::{
    PerformanceMonitor, 
    PerformanceStats, 
    FpsCounter, 
    MemoryTracker, 
    AnimationStats, 
    get_performance_stats, 
    get_performance_report, 
    record_frame, 
    track_animation_start, 
    track_animation_end
};

// Re-export simple components - removed for now
// pub use simple_components::*;
pub use hooks::*;
pub use utils::*;

// Re-export new v0.7 features
pub use animate_presence::*;
pub use performance_optimizations::*;
pub use optimized_animation_manager::{OptimizedAnimationManager, OptimizedAnimationStats};
pub use memory_management::{
    AnimationMemoryManager, AutoMemoryManager, MemoryStats, MemoryPressure, GCStrategy
};
pub use integrated_memory_manager::{
    IntegratedMemoryManager, MemoryOptimizationSettings, ComprehensiveStats
};
pub use timeline::*;
pub use variants::*;

#[cfg(feature = "css-animations")]
pub use css_animations::*;

// Legacy re-exports removed - using single MotionDiv API

/// Helper function to create reactive animation targets from closures
/// 
/// This function converts a closure that returns `HashMap<String, AnimationValue>`
/// into the expected `Box<dyn Fn() -> AnimationTarget>` type for the ReactiveMotionDiv component.
pub fn reactive_animate<F>(closure: F) -> Box<dyn Fn() -> AnimationTarget>
where
    F: Fn() -> std::collections::HashMap<String, AnimationValue> + 'static,
{
    Box::new(move || AnimationTarget::from(closure()))
}