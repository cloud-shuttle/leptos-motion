//! Scroll animations for Leptos Motion

#![warn(missing_docs)]

/// Scroll animator (placeholder)
#[derive(Default)]
pub struct ScrollAnimator {
    /// Active animations
    pub active: bool,
}

impl ScrollAnimator {
    /// Create new scroll animator
    pub fn new() -> Self {
        Self { active: false }
    }
}