//! Leptos Motion DOM Integration
//! 
//! Leptos components and DOM utilities for motion animations

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod components;
pub mod elements;
pub mod hooks;
pub mod performance;
pub mod presence;
pub mod utils;
pub mod simplified_event_handling;

// Re-export commonly used items
pub use components::*;
pub use elements::*;
pub use hooks::*;
pub use presence::*;
pub use utils::*;

// Re-export simplified event handling (new public API)
pub use simplified_event_handling::{SimplifiedMotionProps, SimplifiedDragConfig};

// Re-export core types for convenience
pub use leptos_motion_core::*;

// Include simplified event handling tests
#[cfg(test)]
mod simplified_event_handling_tests {
    include!("simplified_event_handling_tests.rs");
}