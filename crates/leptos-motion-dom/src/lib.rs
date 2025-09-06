//! Leptos Motion DOM Integration
//!
//! Leptos components and DOM utilities for motion animations

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod components;
pub mod elements;
pub mod hooks;
pub mod improved_motion_div;
pub mod performance;
pub mod presence;
pub mod simplified_event_handling;
pub mod utils;

#[cfg(feature = "css-animations")]
pub mod css_animations;

#[cfg(test)]
mod accessibility_tests;

#[cfg(test)]
mod components_tests;
#[cfg(test)]
mod motion_div_tdd_tests;

// Re-export commonly used items
// pub use components::*; // Temporarily disabled due to unused imports
pub use elements::*;
pub use hooks::*;
pub use presence::*;
pub use utils::*;

#[cfg(feature = "css-animations")]
pub use css_animations::*;

// Re-export components
pub use components::{MotionDiv, MotionSpan};
pub use improved_motion_div::{ImprovedMotionDiv, use_animation_state, use_in_view, use_drag_state, use_layout_animation};

// Re-export simplified event handling (new public API)
pub use simplified_event_handling::{
    DragAxis, DragConfig, DragConstraints, EventHandlers, MotionProps, SimplifiedDragConfig,
    SimplifiedMotionProps,
};

// Re-export core types for convenience
pub use leptos_motion_core::*;

// Include simplified event handling tests
#[cfg(test)]
mod simplified_event_handling_tests {
    include!("simplified_event_handling_tests.rs");
}
