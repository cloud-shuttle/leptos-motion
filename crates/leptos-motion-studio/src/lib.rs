//! Leptos Motion Studio
//!
//! Visual animation editor and motion studio for creating complex animations
//! with timeline-based keyframes, 3D transforms, and GPU acceleration.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod export;
pub mod morphing;
pub mod pooling;
pub mod preview;
pub mod project;
pub mod studio;
pub mod timeline;
pub mod transforms;
pub mod webgl;

// Re-export main studio components
pub use export::{AnimationExporter, CodeGenerator, ExportFormat};
pub use morphing::{MorphTransition, PathMorpher, SvgMorphing};
pub use pooling::{AnimationPool, MemoryManager, PooledAnimation};
pub use preview::{LivePreview, PreviewRenderer};
pub use project::{ProjectManager, ProjectSettings, StudioProject};
pub use studio::MotionStudio;
pub use timeline::{AnimationTimeline, KeyframeEditor, Timeline3D};
pub use transforms::{Perspective, Transform3D, TransformMatrix3D};
pub use webgl::{GPUAnimation, WebGLAcceleration, WebGLRenderer};

/// Result type for studio operations
pub type Result<T> = std::result::Result<T, StudioError>;

/// Studio-specific error types
#[derive(Debug, thiserror::Error)]
pub enum StudioError {
    /// WebGL context creation failed
    #[error("WebGL context creation failed: {0}")]
    WebGLError(String),

    /// 3D transform calculation error
    #[error("3D transform error: {0}")]
    Transform3DError(String),

    /// SVG path morphing error
    #[error("SVG morphing error: {0}")]
    MorphingError(String),

    /// Timeline validation error
    #[error("Timeline error: {0}")]
    TimelineError(String),

    /// Project file error
    #[error("Project file error: {0}")]
    ProjectError(String),

    /// Export format error
    #[error("Export error: {0}")]
    ExportError(String),

    /// Memory pool exhaustion
    #[error("Animation pool exhausted: {0}")]
    PoolExhausted(String),

    /// GPU memory error
    #[error("GPU memory error: {0}")]
    GPUMemoryError(String),
}

#[cfg(test)]
mod tests;
