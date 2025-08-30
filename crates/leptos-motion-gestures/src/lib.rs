//! Gesture recognition for Leptos Motion

#![warn(missing_docs)]

pub mod drag;
pub mod hover;
pub mod tap;

/// Gesture configuration placeholder
#[derive(Debug, Clone)]
pub struct GestureConfig {
    /// Gesture enabled
    pub enabled: bool,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}