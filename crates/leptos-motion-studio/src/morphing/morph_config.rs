//! Morphing configuration and settings

use super::path_morpher::MorphQuality;

/// Configuration for path morphing
#[derive(Debug, Clone)]
pub struct MorphConfig {
    /// Morphing duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: EasingFunction,
    /// Morphing quality
    pub quality: MorphQuality,
    /// Whether to preserve path structure
    pub preserve_structure: bool,
    /// Maximum number of points to generate
    pub max_points: Option<usize>,
    /// Whether to auto-close paths
    pub auto_close: bool,
}

/// Easing functions for morphing
#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    /// Linear interpolation
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
    /// Custom bezier curve
    Bezier(f64, f64, f64, f64),
}

impl Default for MorphConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            easing: EasingFunction::EaseInOut,
            quality: MorphQuality::Medium,
            preserve_structure: true,
            max_points: None,
            auto_close: false,
        }
    }
}

impl MorphConfig {
    /// Create a new morphing configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set morphing duration
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Set morphing quality
    pub fn with_quality(mut self, quality: MorphQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Set whether to preserve path structure
    pub fn with_preserve_structure(mut self, preserve: bool) -> Self {
        self.preserve_structure = preserve;
        self
    }

    /// Set maximum number of points
    pub fn with_max_points(mut self, max_points: Option<usize>) -> Self {
        self.max_points = max_points;
        self
    }

    /// Set auto-close behavior
    pub fn with_auto_close(mut self, auto_close: bool) -> Self {
        self.auto_close = auto_close;
        self
    }
}

impl EasingFunction {
    /// Apply easing to a progress value (0.0 to 1.0)
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            EasingFunction::Bezier(a, b, c, d) => {
                // Simplified bezier implementation
                // In reality, this would be a proper cubic bezier calculation
                t // Placeholder
            }
        }
    }
}
