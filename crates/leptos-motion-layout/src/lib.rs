//! Layout animations for Leptos Motion

#![warn(missing_docs)]

/// FLIP layout animator (placeholder)
#[derive(Default)]
pub struct LayoutAnimator {
    /// Active animations
    pub active: bool,
}

impl LayoutAnimator {
    /// Create new layout animator
    pub fn new() -> Self {
        Self { active: false }
    }
}