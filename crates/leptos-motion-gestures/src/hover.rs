//! Hover gesture implementation

/// Hover gesture handler (placeholder)
pub struct HoverGesture {
    /// Whether hover is active
    pub active: bool,
}

impl HoverGesture {
    /// Create new hover gesture
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for HoverGesture {
    fn default() -> Self {
        Self::new()
    }
}