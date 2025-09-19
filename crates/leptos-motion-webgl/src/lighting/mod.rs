//! Lighting system for WebGL rendering
//!
//! This module provides a comprehensive lighting system broken down into
//! focused, maintainable components for different light types and calculations.

pub mod light_types;
pub mod ambient_lighting;
pub mod directional_lighting;
pub mod point_lighting;
pub mod spot_lighting;
pub mod lighting_calculations;

// Re-export main types for convenience
pub use light_types::{Light, LightType, Color};
pub use ambient_lighting::AmbientLight;
pub use directional_lighting::DirectionalLight;
pub use point_lighting::PointLight;
pub use spot_lighting::SpotLight;
pub use lighting_calculations::LightingManager;
