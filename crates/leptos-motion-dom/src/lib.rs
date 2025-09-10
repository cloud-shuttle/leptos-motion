//! Leptos Motion DOM Integration
//!
//! Leptos components and DOM utilities for motion animations

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod components;
pub mod elements;
pub mod fixed_motion_div;
pub mod hooks;
/// Improved motion div implementation with enhanced features
pub mod improved_motion_div;
pub mod performance;
pub mod presence;
pub mod reactive_motion_div;
// pub mod reactive_motion_div_fixed; // Disabled due to threading issues
pub mod simplified_event_handling;
pub mod utils;

// Phase 2: Animation Engine Integration
pub mod animation_engine;
pub mod easing_functions;
pub mod enhanced_motion_div;
pub mod repeat_config;
pub mod transform_animations;

// Signal-based animation controller with proven patterns
// pub mod signal_based_controller; // Temporarily disabled due to compilation errors
// pub mod signal_based_motion_div; // Temporarily disabled due to compilation errors
pub mod signal_based_animation_controller;
pub mod simple_signal_based_motion_div;

// New v0.7 features
pub mod animate_presence;
pub mod performance_optimizations;
pub mod spring_physics;
pub mod timeline;
pub mod variants;

#[cfg(feature = "css-animations")]
pub mod css_animations;

#[cfg(test)]
mod accessibility_tests;

#[cfg(test)]
mod components_tests;
#[cfg(test)]
mod motion_div_tdd_tests;

#[cfg(test)]
mod motion_div_api_fix_tests;

#[cfg(test)]
mod api_contract_tests;

#[cfg(test)]
mod regression_prevention_tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod animation_engine_tests;

#[cfg(test)]
mod enhanced_motion_div_tests;

// Include the DOM integration TDD tests
// #[cfg(test)]
// mod dom_integration_tdd_tests;

// Include the advanced features TDD tests
// #[cfg(test)]
// mod advanced_features_tdd_tests;

#[cfg(test)]
mod reactive_animation_tests;

#[cfg(test)]
mod signal_based_controller_tests;

#[cfg(test)]
mod drag_animation_tests {
    include!("drag_animation_tests.rs");
}

#[cfg(test)]
mod drag_integration_tests {
    include!("drag_integration_tests.rs");
}

#[cfg(test)]
mod drag_constraint_tests {
    include!("drag_constraint_tests.rs");
}

#[cfg(test)]
mod performance_benchmark_tests {
    include!("performance_benchmark_tests.rs");
}

#[cfg(test)]
mod drag_constraint_integration_tests {
    include!("drag_constraint_integration_tests.rs");
}

#[cfg(test)]
mod flip_animation_tests {
    include!("flip_animation_tests.rs");
}

#[cfg(test)]
mod keyframe_animation_tests {
    include!("keyframe_animation_tests.rs");
}

#[cfg(test)]
mod stagger_animation_tests {
    include!("stagger_animation_tests.rs");
}

#[cfg(test)]
mod momentum_animation_tests {
    include!("momentum_animation_tests.rs");
}

#[cfg(test)]
mod momentum_integration_tests {
    include!("momentum_integration_tests.rs");
}

#[cfg(test)]
mod advanced_performance_tests {
    include!("advanced_performance_tests.rs");
}

#[cfg(test)]
mod cross_browser_tests {
    include!("cross_browser_tests.rs");
}

#[cfg(test)]
mod spring_physics_tests {
    include!("spring_physics_tests.rs");
}

#[cfg(test)]
mod animate_presence_tests {
    include!("animate_presence_tests.rs");
}

#[cfg(test)]
mod variants_system_tests {
    include!("variants_system_tests.rs");
}

#[cfg(test)]
mod timeline_sequences_tests {
    include!("timeline_sequences_tests.rs");
}

#[cfg(test)]
mod polish_optimize_tests {
    include!("polish_optimize_tests.rs");
}

// #[cfg(test)]
// mod phase1_engine_integration_tests;

// #[cfg(test)]
// mod phase2_leptos_compatibility_tests;

// #[cfg(test)]
// mod phase3_feature_completion_tests;

// #[cfg(test)]
// mod phase4_performance_polish_tests;

// Re-export commonly used items
// pub use components::*; // Temporarily disabled due to unused imports
pub use elements::*;
pub use hooks::*;
// pub use presence::*; // Disabled due to conflict with animate_presence
pub use utils::*;

// Re-export new v0.7 features
pub use animate_presence::*;
pub use performance_optimizations::*;
pub use spring_physics::*;
pub use timeline::*;
pub use variants::*;

#[cfg(feature = "css-animations")]
pub use css_animations::*;

// Re-export components
pub use components::{MotionDiv, MotionSpan};
pub use reactive_motion_div::{
    AnimationTargetOrReactive, ReactiveMotionDiv, reactive_animate, signal_animate, static_animate,
};
// pub use reactive_motion_div_fixed::ReactiveMotionDivFixed; // Disabled due to threading issues
// Improved motion div module is not yet implemented
// pub use improved_motion_div::{
//     ImprovedMotionDiv, use_animation_state, use_drag_state, use_in_view, use_layout_animation,
// };

// Re-export simplified event handling (new public API)
pub use simplified_event_handling::{
    DragAxis, DragConfig, DragConstraints, EventHandlers, MotionProps, SimplifiedDragConfig,
    SimplifiedMotionProps,
};

// Re-export core types for convenience
pub use leptos_motion_core::{
    AnimationConfig, AnimationEngine, AnimationHandle, AnimationTarget, AnimationValue,
    ComplexValue, Easing, MotionNumber, MotionTransform, MotionValue, MotionValues, RepeatConfig,
    SpringConfig, StaggerConfig, StaggerFrom, Transform, Transition, Variants,
};

// Include simplified event handling tests
#[cfg(test)]
mod simplified_event_handling_tests {
    include!("simplified_event_handling_tests.rs");
}
