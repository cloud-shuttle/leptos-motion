//! SVG morphing and path animation system
//!
//! This module provides SVG path morphing capabilities broken down into
//! focused, maintainable modules.

pub mod svg_path;
pub mod path_commands;
pub mod geometry;
pub mod path_morpher;
pub mod morph_config;
pub mod svg_morphing;
pub mod morph_transition;
pub mod editor;

// Re-export main types for convenience
pub use svg_path::*;
pub use path_commands::*;
pub use geometry::*;
pub use path_morpher::*;
pub use morph_config::*;
pub use svg_morphing::*;
pub use morph_transition::*;
pub use editor::*;
