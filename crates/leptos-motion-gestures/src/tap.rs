//! Tap gesture implementation

/// Tap gesture handler (placeholder)
pub struct TapGesture {
    /// Whether tap is active
    pub active: bool,
}

impl TapGesture {
    /// Create new tap gesture
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for TapGesture {
    fn default() -> Self {
        Self::new()
    }
}