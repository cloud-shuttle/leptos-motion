//! Drag gesture implementation

/// Drag gesture handler (placeholder)
pub struct DragGesture {
    /// Whether drag is active
    pub active: bool,
}

impl DragGesture {
    /// Create new drag gesture
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for DragGesture {
    fn default() -> Self {
        Self::new()
    }
}