//! Export functionality for Motion Studio animations

pub mod types;
pub mod exporter;
pub mod results;
pub mod generator;

// Re-export main types and structs
pub use types::*;
pub use results::*;

// Legacy exports for backward compatibility
pub use exporter::AnimationExporter;
pub use generator::{CodeGenerator, CodeTarget, CodeGenSettings};
