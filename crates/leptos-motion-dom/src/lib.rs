//! Leptos Motion DOM Integration
//! 
//! Leptos components and DOM utilities for motion animations

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod components;
pub mod elements;
pub mod hooks;
pub mod presence;
pub mod utils;

// Re-export commonly used items
pub use components::*;
pub use elements::*;
pub use hooks::*;
pub use presence::*;
pub use utils::*;

// Re-export core types for convenience
pub use leptos_motion_core::*;