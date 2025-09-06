//! Core types for animation values, targets, and configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for animation instances
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnimationHandle(pub u64);

/// Animation value types that can be animated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

/// 3D transform representation
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexValue {
    /// Raw value data
    pub data: serde_json::Value,
    /// Type identifier for custom interpolation
    pub value_type: String,
}

/// Animation target containing property-value pairs
pub type AnimationTarget = HashMap<String, AnimationValue>;

/// Transition configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Spring(SpringConfig),
    /// Cubic bezier curve
    Bezier(f64, f64, f64, f64),
}

/// Spring animation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaggerConfig {
    /// Delay between each element
    pub delay: f64,
    /// Starting position for stagger
    pub from: StaggerFrom,
}

/// Stagger starting position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
