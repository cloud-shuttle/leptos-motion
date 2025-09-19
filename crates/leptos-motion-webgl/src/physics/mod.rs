//! Physics simulation system
//!
//! This module provides physics simulation capabilities,
//! broken down into focused, maintainable modules.

pub mod config;
pub mod rigid_body;
pub mod collision;
pub mod world;
pub mod shapes;
pub mod bounding_box;
pub mod contact;

// Re-export main types for convenience
pub use config::*;
pub use rigid_body::*;
pub use collision::*;
pub use world::*;
pub use shapes::*;
pub use bounding_box::*;
pub use contact::*;
