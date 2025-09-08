//! Core types for animation values, targets, and configuration

#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for animation instances
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct AnimationHandle(pub u64);

/// Animation value types that can be animated
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum AnimationValue {
    /// Numeric value (unitless)
    Number(f64),
    /// Pixel value
    Pixels(f64),
    /// Percentage value
    Percentage(f64),
    /// Degree value for rotations
    Degrees(f64),
    /// Radians value for rotations
    Radians(f64),
    /// Color value (CSS format)
    Color(String),
    /// Transform matrix
    Transform(Transform),
    /// String value for non-numeric properties
    String(String),
    /// Complex value with custom interpolation
    Complex(ComplexValue),
}

impl AnimationValue {
    /// Convert animation value to string representation
    pub fn to_string(&self) -> String {
        match self {
            AnimationValue::Number(n) => n.to_string(),
            AnimationValue::Pixels(p) => format!("{}px", p),
            AnimationValue::Percentage(p) => format!("{}%", p),
            AnimationValue::Degrees(d) => format!("{}deg", d),
            AnimationValue::Radians(r) => format!("{}rad", r),
            AnimationValue::Color(c) => c.clone(),
            AnimationValue::Transform(t) => {
                let mut transforms = Vec::new();

                if let Some(x) = t.x {
                    transforms.push(format!("translateX({}px)", x));
                }
                if let Some(y) = t.y {
                    transforms.push(format!("translateY({}px)", y));
                }
                if let Some(z) = t.z {
                    transforms.push(format!("translateZ({}px)", z));
                }
                if let Some(rx) = t.rotate_x {
                    transforms.push(format!("rotateX({}deg)", rx));
                }
                if let Some(ry) = t.rotate_y {
                    transforms.push(format!("rotateY({}deg)", ry));
                }
                if let Some(rz) = t.rotate_z {
                    transforms.push(format!("rotateZ({}deg)", rz));
                }
                if let Some(s) = t.scale {
                    transforms.push(format!("scale({})", s));
                }
                if let Some(sx) = t.scale_x {
                    transforms.push(format!("scaleX({})", sx));
                }
                if let Some(sy) = t.scale_y {
                    transforms.push(format!("scaleY({})", sy));
                }
                if let Some(skew_x) = t.skew_x {
                    transforms.push(format!("skewX({}deg)", skew_x));
                }
                if let Some(skew_y) = t.skew_y {
                    transforms.push(format!("skewY({}deg)", skew_y));
                }

                transforms.join(" ")
            }
            AnimationValue::String(s) => s.clone(),
            AnimationValue::Complex(_) => "complex".to_string(),
        }
    }

    /// Interpolate between two animation values
    pub fn interpolate(&self, other: &AnimationValue, progress: f64) -> AnimationValue {
        match (self, other) {
            (AnimationValue::Number(a), AnimationValue::Number(b)) => {
                AnimationValue::Number(a + (b - a) * progress)
            }
            (AnimationValue::Pixels(a), AnimationValue::Pixels(b)) => {
                AnimationValue::Pixels(a + (b - a) * progress)
            }
            (AnimationValue::Percentage(a), AnimationValue::Percentage(b)) => {
                AnimationValue::Percentage(a + (b - a) * progress)
            }
            (AnimationValue::Degrees(a), AnimationValue::Degrees(b)) => {
                AnimationValue::Degrees(a + (b - a) * progress)
            }
            (AnimationValue::Radians(a), AnimationValue::Radians(b)) => {
                AnimationValue::Radians(a + (b - a) * progress)
            }
            (AnimationValue::Transform(a), AnimationValue::Transform(b)) => {
                AnimationValue::Transform(Transform {
                    x: match (a.x, b.x) {
                        (Some(x1), Some(x2)) => Some(x1 + (x2 - x1) * progress),
                        (Some(x), None) => Some(x),
                        (None, Some(x)) => Some(x),
                        (None, None) => None,
                    },
                    y: match (a.y, b.y) {
                        (Some(y1), Some(y2)) => Some(y1 + (y2 - y1) * progress),
                        (Some(y), None) => Some(y),
                        (None, Some(y)) => Some(y),
                        (None, None) => None,
                    },
                    z: match (a.z, b.z) {
                        (Some(z1), Some(z2)) => Some(z1 + (z2 - z1) * progress),
                        (Some(z), None) => Some(z),
                        (None, Some(z)) => Some(z),
                        (None, None) => None,
                    },
                    rotate_x: match (a.rotate_x, b.rotate_x) {
                        (Some(r1), Some(r2)) => Some(r1 + (r2 - r1) * progress),
                        (Some(r), None) => Some(r),
                        (None, Some(r)) => Some(r),
                        (None, None) => None,
                    },
                    rotate_y: match (a.rotate_y, b.rotate_y) {
                        (Some(r1), Some(r2)) => Some(r1 + (r2 - r1) * progress),
                        (Some(r), None) => Some(r),
                        (None, Some(r)) => Some(r),
                        (None, None) => None,
                    },
                    rotate_z: match (a.rotate_z, b.rotate_z) {
                        (Some(r1), Some(r2)) => Some(r1 + (r2 - r1) * progress),
                        (Some(r), None) => Some(r),
                        (None, Some(r)) => Some(r),
                        (None, None) => None,
                    },
                    scale_x: match (a.scale_x, b.scale_x) {
                        (Some(s1), Some(s2)) => Some(s1 + (s2 - s1) * progress),
                        (Some(s), None) => Some(s),
                        (None, Some(s)) => Some(s),
                        (None, None) => None,
                    },
                    scale_y: match (a.scale_y, b.scale_y) {
                        (Some(s1), Some(s2)) => Some(s1 + (s2 - s1) * progress),
                        (Some(s), None) => Some(s),
                        (None, Some(s)) => Some(s),
                        (None, None) => None,
                    },
                    skew_x: match (a.skew_x, b.skew_x) {
                        (Some(s1), Some(s2)) => Some(s1 + (s2 - s1) * progress),
                        (Some(s), None) => Some(s),
                        (None, Some(s)) => Some(s),
                        (None, None) => None,
                    },
                    skew_y: match (a.skew_y, b.skew_y) {
                        (Some(s1), Some(s2)) => Some(s1 + (s2 - s1) * progress),
                        (Some(s), None) => Some(s),
                        (None, Some(s)) => Some(s),
                        (None, None) => None,
                    },
                    scale: match (a.scale, b.scale) {
                        (Some(s1), Some(s2)) => Some(s1 + (s2 - s1) * progress),
                        (Some(s), None) => Some(s),
                        (None, Some(s)) => Some(s),
                        (None, None) => None,
                    },
                })
            }
            // For other types, return the "from" value if progress < 0.5, otherwise return "to" value
            (from, _) if progress < 0.5 => from.clone(),
            (_, to) => to.clone(),
        }
    }
}

/// 3D transform representation
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Transform {
    /// X translation
    pub x: Option<f64>,
    /// Y translation
    pub y: Option<f64>,
    /// Z translation
    pub z: Option<f64>,
    /// Rotation around X axis (degrees)
    pub rotate_x: Option<f64>,
    /// Rotation around Y axis (degrees)
    pub rotate_y: Option<f64>,
    /// Rotation around Z axis (degrees)
    pub rotate_z: Option<f64>,
    /// Uniform scale
    pub scale: Option<f64>,
    /// X scale
    pub scale_x: Option<f64>,
    /// Y scale
    pub scale_y: Option<f64>,
    /// X skew (degrees)
    pub skew_x: Option<f64>,
    /// Y skew (degrees)
    pub skew_y: Option<f64>,
}

/// Complex animation value with custom interpolation
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct ComplexValue {
    /// Raw value data
    #[cfg(feature = "serde-support")]
    pub data: serde_json::Value,
    #[cfg(not(feature = "serde-support"))]
    pub data: String,
    /// Type identifier for custom interpolation
    pub value_type: String,
}

/// Animation target containing property-value pairs
pub type AnimationTarget = HashMap<String, AnimationValue>;

/// Transition configuration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Transition {
    /// Animation duration in seconds
    pub duration: Option<f64>,
    /// Animation delay in seconds
    pub delay: Option<f64>,
    /// Easing function
    pub ease: Easing,
    /// Repeat configuration
    pub repeat: RepeatConfig,
    /// Stagger configuration for multiple elements
    pub stagger: Option<StaggerConfig>,
}

/// Easing function types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum Easing {
    /// Linear interpolation
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Circular easing in
    CircIn,
    /// Circular easing out
    CircOut,
    /// Circular easing in and out
    CircInOut,
    /// Back easing in (overshoot)
    BackIn,
    /// Back easing out (overshoot)
    BackOut,
    /// Back easing in and out (overshoot)
    BackInOut,
    /// Spring physics
    #[cfg(feature = "approx")]
    Spring(SpringConfig),
    /// Cubic bezier curve
    Bezier(f64, f64, f64, f64),
}

impl Easing {
    /// Evaluate the easing function at the given progress (0.0 to 1.0)
    /// This is a basic implementation for when the easing module is not available
    pub fn basic_evaluate(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            Easing::CircIn => 1.0 - (1.0 - t * t).sqrt(),
            Easing::CircOut => ((2.0 - t) * t).sqrt(),
            Easing::CircInOut => {
                if t < 0.5 {
                    (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - 4.0 * (1.0 - t) * (1.0 - t)).sqrt()) / 2.0
                }
            }
            Easing::BackIn => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                C3 * t * t * t - C1 * t * t
            }
            Easing::BackOut => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
            }
            Easing::BackInOut => {
                const C1: f64 = 1.70158;
                const C2: f64 = C1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
                }
            }
            #[cfg(feature = "approx")]
            Easing::Spring(_) => {
                // For now, return linear interpolation for spring
                // This should be replaced with proper spring physics when available
                t
            }
            Easing::Bezier(_x1, y1, _x2, y2) => {
                // Simple cubic bezier implementation
                // This is a simplified version - for production use, consider a more robust implementation
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;

                3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
            }
        }
    }
}

/// Spring animation configuration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[cfg(feature = "approx")]
pub struct SpringConfig {
    /// Spring stiffness (higher = faster)
    pub stiffness: f64,
    /// Spring damping (higher = less oscillation)
    pub damping: f64,
    /// Object mass (higher = more inertia)
    pub mass: f64,
    /// Initial velocity
    pub velocity: f64,
    /// Rest threshold for position
    pub rest_delta: f64,
    /// Rest threshold for velocity
    pub rest_speed: f64,
}

/// Repeat configuration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum RepeatConfig {
    /// No repetition
    Never,
    /// Repeat N times
    Count(u32),
    /// Infinite repetition
    Infinite,
    /// Infinite with alternating direction
    InfiniteReverse,
}

/// Stagger configuration for multiple element animations
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct StaggerConfig {
    /// Delay between each element
    pub delay: f64,
    /// Starting position for stagger
    pub from: StaggerFrom,
}

/// Stagger starting position
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum StaggerFrom {
    /// Start from first element
    First,
    /// Start from last element
    Last,
    /// Start from center element
    Center,
    /// Start from specific index
    Index(usize),
}

// Default implementations

impl Default for Transition {
    fn default() -> Self {
        Self {
            duration: Some(0.3),
            delay: None,
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Never,
            stagger: None,
        }
    }
}

#[cfg(feature = "approx")]
impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::EaseInOut
    }
}

// Utility implementations

impl AnimationValue {
    /// Extract numeric value if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
            AnimationValue::Number(n) => Some(*n),
            AnimationValue::Pixels(n) => Some(*n),
            AnimationValue::Percentage(n) => Some(*n),
            AnimationValue::Degrees(n) => Some(*n),
            AnimationValue::Radians(n) => Some(*n),
            _ => None,
        }
    }

    /// Check if value is numeric (can be interpolated)
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            AnimationValue::Number(_)
                | AnimationValue::Pixels(_)
                | AnimationValue::Percentage(_)
                | AnimationValue::Degrees(_)
                | AnimationValue::Radians(_)
        )
    }

    /// Get the unit suffix for CSS
    pub fn unit(&self) -> &'static str {
        match self {
            AnimationValue::Number(_) => "",
            AnimationValue::Pixels(_) => "px",
            AnimationValue::Percentage(_) => "%",
            AnimationValue::Degrees(_) => "deg",
            AnimationValue::Radians(_) => "rad",
            _ => "",
        }
    }
}

impl Transform {
    /// Create a new transform with translation
    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        }
    }

    /// Create a new transform with rotation
    pub fn rotate(degrees: f64) -> Self {
        Self {
            rotate_z: Some(degrees),
            ..Default::default()
        }
    }

    /// Create a new transform with scale
    pub fn scale(scale: f64) -> Self {
        Self {
            scale: Some(scale),
            ..Default::default()
        }
    }

    /// Check if transform is identity (no changes)
    pub fn is_identity(&self) -> bool {
        let zero_or_none = |v: Option<f64>| v.map_or(true, |v| v == 0.0);
        let one_or_none = |v: Option<f64>| v.map_or(true, |v| v == 1.0);

        zero_or_none(self.x)
            && zero_or_none(self.y)
            && zero_or_none(self.z)
            && zero_or_none(self.rotate_x)
            && zero_or_none(self.rotate_y)
            && zero_or_none(self.rotate_z)
            && one_or_none(self.scale)
            && one_or_none(self.scale_x)
            && one_or_none(self.scale_y)
            && zero_or_none(self.skew_x)
            && zero_or_none(self.skew_y)
    }

    /// Convert to CSS transform string
    pub fn to_css(&self) -> String {
        let mut transforms = Vec::new();

        if let (Some(x), Some(y)) = (self.x, self.y) {
            transforms.push(format!("translate({}px, {}px)", x, y));
        } else {
            if let Some(x) = self.x {
                transforms.push(format!("translateX({}px)", x));
            }
            if let Some(y) = self.y {
                transforms.push(format!("translateY({}px)", y));
            }
        }

        if let Some(z) = self.z {
            transforms.push(format!("translateZ({}px)", z));
        }

        if let Some(rotate) = self.rotate_z {
            transforms.push(format!("rotate({}deg)", rotate));
        }
        if let Some(rotate_x) = self.rotate_x {
            transforms.push(format!("rotateX({}deg)", rotate_x));
        }
        if let Some(rotate_y) = self.rotate_y {
            transforms.push(format!("rotateY({}deg)", rotate_y));
        }

        if let Some(scale) = self.scale {
            transforms.push(format!("scale({})", scale));
        } else {
            if let Some(scale_x) = self.scale_x {
                transforms.push(format!("scaleX({})", scale_x));
            }
            if let Some(scale_y) = self.scale_y {
                transforms.push(format!("scaleY({})", scale_y));
            }
        }

        if let Some(skew_x) = self.skew_x {
            transforms.push(format!("skewX({}deg)", skew_x));
        }
        if let Some(skew_y) = self.skew_y {
            transforms.push(format!("skewY({}deg)", skew_y));
        }

        transforms.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_value_numeric() {
        let value = AnimationValue::Number(42.0);
        assert_eq!(value.as_number(), Some(42.0));
        assert!(value.is_numeric());
        assert_eq!(value.unit(), "");

        let pixels = AnimationValue::Pixels(100.0);
        assert_eq!(pixels.unit(), "px");
    }

    #[test]
    fn test_transform_identity() {
        let identity = Transform::default();
        assert!(identity.is_identity());

        let translated = Transform::translate(10.0, 20.0);
        assert!(!translated.is_identity());
    }

    #[test]
    fn test_transform_css() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            rotate_z: Some(45.0),
            scale: Some(1.5),
            ..Default::default()
        };

        let css = transform.to_css();
        assert!(css.contains("translate(10px, 20px)"));
        assert!(css.contains("rotate(45deg)"));
        assert!(css.contains("scale(1.5)"));
    }
}

// Leptos v0.8 compatibility helper functions
#[cfg(feature = "leptos-integration")]
pub mod leptos_helpers {
    use super::*;

    /// Convert AnimationTarget to a CSS class string
    pub fn animation_target_to_class_string(target: &AnimationTarget) -> String {
        target
            .iter()
            .map(|(key, value)| format!("{}-{}", key, value.to_string().replace(" ", "-")))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Convert AnimationValue to a string for attributes
    pub fn animation_value_to_attribute_string(value: &AnimationValue) -> String {
        value.to_string()
    }

    /// Convert Transition to CSS properties string
    pub fn transition_to_css_properties(transition: &Transition) -> String {
        let mut properties = Vec::new();

        if let Some(duration) = transition.duration {
            properties.push(format!("transition-duration: {}s", duration));
        }

        if let Some(delay) = transition.delay {
            properties.push(format!("transition-delay: {}s", delay));
        }

        let ease_string = match transition.ease {
            Easing::Linear => "linear",
            Easing::EaseIn => "ease-in",
            Easing::EaseOut => "ease-out",
            Easing::EaseInOut => "ease-in-out",
            Easing::CircIn => "cubic-bezier(0.55, 0.055, 0.675, 0.19)",
            Easing::CircOut => "cubic-bezier(0.215, 0.61, 0.355, 1)",
            Easing::CircInOut => "cubic-bezier(0.645, 0.045, 0.355, 1)",
            Easing::BackIn => "cubic-bezier(0.6, -0.28, 0.735, 0.045)",
            Easing::BackOut => "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
            Easing::BackInOut => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
            Easing::Spring(_) => "cubic-bezier(0.68, -0.55, 0.265, 1.55)", // Simplified
            Easing::Bezier(a, b, c, d) => {
                return format!("cubic-bezier({}, {}, {}, {})", a, b, c, d);
            }
        };

        properties.push(format!("transition-timing-function: {}", ease_string));
        properties.join("; ")
    }
}
