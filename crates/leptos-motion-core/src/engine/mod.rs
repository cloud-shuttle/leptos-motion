//! Animation engine implementations
//!
//! This module contains the core animation engine implementations,
//! broken down into smaller, focused modules for better maintainability.

pub mod traits;
pub mod hybrid;
pub mod waapi;
pub mod raf;
pub mod feature_detector;
pub mod utils;

// Re-export the main types for convenience
pub use traits::*;
pub use hybrid::OptimizedHybridEngine;
pub use waapi::WaapiEngine;
pub use raf::RafEngine;
pub use feature_detector::FeatureDetector;
