//! Animation type definitions for pooling

/// Types of animations that can be pooled
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationType {
    /// Transform animation (position, rotation, scale)
    Transform,
    /// Opacity animation
    Opacity,
    /// Color animation
    Color,
    /// Path animation
    Path,
    /// Custom animation type
    Custom(String),
}

/// Animation state
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    /// Animation is ready to be used
    Ready,
    /// Animation is currently playing
    Playing,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
}

impl AnimationType {
    /// Get the string representation of the animation type
    pub fn as_str(&self) -> &str {
        match self {
            AnimationType::Transform => "transform",
            AnimationType::Opacity => "opacity",
            AnimationType::Color => "color",
            AnimationType::Path => "path",
            AnimationType::Custom(name) => name,
        }
    }

    /// Create a custom animation type
    pub fn custom(name: &str) -> Self {
        AnimationType::Custom(name.to_string())
    }
}

impl AnimationState {
    /// Check if the animation is active (playing or paused)
    pub fn is_active(&self) -> bool {
        matches!(self, AnimationState::Playing | AnimationState::Paused)
    }

    /// Check if the animation is finished
    pub fn is_finished(&self) -> bool {
        matches!(self, AnimationState::Completed)
    }
}
