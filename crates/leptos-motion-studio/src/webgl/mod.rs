//! WebGL rendering system
//!
//! This module provides WebGL-based rendering capabilities,
//! broken down into focused, maintainable modules.

pub mod context;
pub mod shader_program;
pub mod gpu_animation;
pub mod renderer;
pub mod acceleration;
pub mod capabilities;
pub mod canvas;

// Re-export main types for convenience
pub use context::*;
pub use shader_program::*;
pub use gpu_animation::*;
pub use renderer::*;
pub use acceleration::*;
pub use capabilities::*;
pub use canvas::*;
