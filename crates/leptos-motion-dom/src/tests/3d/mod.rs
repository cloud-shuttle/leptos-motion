//! # 3D Animation Tests
//!
//! This module contains comprehensive tests for 3D animation features
//! broken down into focused, maintainable test files.

pub mod morphing_animation_tests;
pub mod particle_system_tests;
pub mod complex_transform_tests;
pub mod perspective_effect_tests;
pub mod path_animation_tests;

// Re-export all test modules for easy access
pub use morphing_animation_tests::*;
pub use particle_system_tests::*;
pub use complex_transform_tests::*;
pub use perspective_effect_tests::*;
pub use path_animation_tests::*;
