//! Value interpolation utilities

use crate::{AnimationValue, Transform};

/// Trait for interpolatable values
pub trait Interpolate {
    /// Interpolate between two values at given progress (0.0 to 1.0)
    fn interpolate(&self, to: &Self, progress: f64) -> Self;
}

/// Linear interpolation between two f64 values
pub fn lerp(from: f64, to: f64, progress: f64) -> f64 {
    from + (to - from) * progress
}

impl Interpolate for f64 {
    fn interpolate(&self, to: &Self, progress: f64) -> Self {
        lerp(*self, *to, progress)
    }
}

impl Interpolate for AnimationValue {
    fn interpolate(&self, to: &Self, progress: f64) -> Self {
        match (self, to) {
            (AnimationValue::Number(from), AnimationValue::Number(to)) => {
                AnimationValue::Number(lerp(*from, *to, progress))
            }
            (AnimationValue::Pixels(from), AnimationValue::Pixels(to)) => {
                AnimationValue::Pixels(lerp(*from, *to, progress))
            }
            (AnimationValue::Percentage(from), AnimationValue::Percentage(to)) => {
                AnimationValue::Percentage(lerp(*from, *to, progress))
            }
            (AnimationValue::Degrees(from), AnimationValue::Degrees(to)) => {
                AnimationValue::Degrees(lerp(*from, *to, progress))
            }
            (AnimationValue::Radians(from), AnimationValue::Radians(to)) => {
                AnimationValue::Radians(lerp(*from, *to, progress))
            }
            (AnimationValue::Transform(from), AnimationValue::Transform(to)) => {
                AnimationValue::Transform(from.interpolate(to, progress))
            }
            (AnimationValue::Color(from), AnimationValue::Color(to)) => {
                AnimationValue::Color(interpolate_color(from, to, progress))
            }
            _ => {
                // For incompatible types, snap at 0.5 progress
                if progress < 0.5 {
                    self.clone()
                } else {
                    to.clone()
                }
            }
        }
    }
}

impl Interpolate for Transform {
    fn interpolate(&self, to: &Self, progress: f64) -> Self {
        Transform {
            x: interpolate_option(self.x, to.x, progress),
            y: interpolate_option(self.y, to.y, progress),
            z: interpolate_option(self.z, to.z, progress),
            rotate_x: interpolate_option(self.rotate_x, to.rotate_x, progress),
            rotate_y: interpolate_option(self.rotate_y, to.rotate_y, progress),
            rotate_z: interpolate_option(self.rotate_z, to.rotate_z, progress),
            scale: interpolate_option(self.scale, to.scale, progress),
            scale_x: interpolate_option(self.scale_x, to.scale_x, progress),
            scale_y: interpolate_option(self.scale_y, to.scale_y, progress),
            skew_x: interpolate_option(self.skew_x, to.skew_x, progress),
            skew_y: interpolate_option(self.skew_y, to.skew_y, progress),
        }
    }
}

/// Interpolate between two optional f64 values
fn interpolate_option(from: Option<f64>, to: Option<f64>, progress: f64) -> Option<f64> {
    match (from, to) {
        (Some(from), Some(to)) => Some(lerp(from, to, progress)),
        (Some(from), None) => Some(lerp(from, 0.0, progress)),
        (None, Some(to)) => Some(lerp(0.0, to, progress)),
        (None, None) => None,
    }
}

/// Interpolate between two color strings (basic implementation)
fn interpolate_color(from: &str, to: &str, progress: f64) -> String {
    // This is a simplified implementation
    // A full implementation would parse CSS colors and interpolate in color space
    if progress < 0.5 {
        from.to_string()
    } else {
        to.to_string()
    }
}

/// Color parsing and interpolation utilities
pub mod color {
    use super::lerp;

    /// RGBA color representation
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Rgba {
        /// Red component (0.0 - 1.0)
        pub r: f64,
        /// Green component (0.0 - 1.0)
        pub g: f64,
        /// Blue component (0.0 - 1.0)
        pub b: f64,
        /// Alpha component (0.0 - 1.0)
        pub a: f64,
    }

    impl Rgba {
        /// Create a new RGBA color
        pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
            Self {
                r: r.clamp(0.0, 255.0),
                g: g.clamp(0.0, 255.0),
                b: b.clamp(0.0, 255.0),
                a: a.clamp(0.0, 1.0),
            }
        }

        /// Interpolate between two colors
        pub fn interpolate(&self, to: &Self, progress: f64) -> Self {
            Self {
                r: lerp(self.r, to.r, progress),
                g: lerp(self.g, to.g, progress),
                b: lerp(self.b, to.b, progress),
                a: lerp(self.a, to.a, progress),
            }
        }

        /// Convert to CSS rgba() string
        pub fn to_css(&self) -> String {
            if self.a < 1.0 {
                format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
            } else {
                format!("rgb({}, {}, {})", self.r, self.g, self.b)
            }
        }

        /// Parse color from hex string
        pub fn from_hex(hex: &str) -> Option<Self> {
            let hex = hex.trim_start_matches('#');
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f64;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f64;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f64;
                Some(Self::new(r, g, b, 1.0))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "approx")]
    use approx::assert_relative_eq;

    #[cfg(feature = "approx")]
    #[test]
    fn test_lerp() {
        assert_relative_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_relative_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_relative_eq!(lerp(0.0, 100.0, 1.0), 100.0);
        assert_relative_eq!(lerp(50.0, 150.0, 0.25), 75.0);
    }

    #[test]
    fn test_animation_value_interpolation() {
        let from = AnimationValue::Number(0.0);
        let to = AnimationValue::Number(100.0);

        let mid = from.interpolate(&to, 0.5);
        assert_eq!(mid, AnimationValue::Number(50.0));

        let pixels_from = AnimationValue::Pixels(10.0);
        let pixels_to = AnimationValue::Pixels(20.0);
        let pixels_mid = pixels_from.interpolate(&pixels_to, 0.3);
        assert_eq!(pixels_mid, AnimationValue::Pixels(13.0));
    }

    #[test]
    fn test_transform_interpolation() {
        let from = Transform {
            x: Some(0.0),
            y: Some(0.0),
            scale: Some(1.0),
            ..Default::default()
        };

        let to = Transform {
            x: Some(100.0),
            y: Some(50.0),
            scale: Some(2.0),
            ..Default::default()
        };

        let mid = from.interpolate(&to, 0.5);
        assert_eq!(mid.x, Some(50.0));
        assert_eq!(mid.y, Some(25.0));
        assert_eq!(mid.scale, Some(1.5));
    }

    #[test]
    fn test_option_interpolation() {
        assert_eq!(interpolate_option(Some(0.0), Some(100.0), 0.5), Some(50.0));
        assert_eq!(interpolate_option(Some(10.0), None, 0.5), Some(5.0));
        assert_eq!(interpolate_option(None, Some(20.0), 0.5), Some(10.0));
        assert_eq!(interpolate_option(None, None, 0.5), None);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_rgba_color() {
        use super::color::Rgba;

        let red = Rgba::new(255.0, 0.0, 0.0, 1.0);
        let blue = Rgba::new(0.0, 0.0, 255.0, 1.0);

        let purple = red.interpolate(&blue, 0.5);
        assert_relative_eq!(purple.r, 127.5, epsilon = 0.1);
        assert_relative_eq!(purple.g, 0.0);
        assert_relative_eq!(purple.b, 127.5, epsilon = 0.1);

        assert_eq!(red.to_css(), "rgb(255, 0, 0)");

        let transparent_red = Rgba::new(255.0, 0.0, 0.0, 0.5);
        assert_eq!(transparent_red.to_css(), "rgba(255, 0, 0, 0.5)");
    }

    #[test]
    fn test_hex_color_parsing() {
        use super::color::Rgba;

        let red = Rgba::from_hex("#ff0000").unwrap();
        assert_eq!(red.r, 255.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);
        assert_eq!(red.a, 1.0);

        let green = Rgba::from_hex("00ff00").unwrap();
        assert_eq!(green.g, 255.0);

        assert!(Rgba::from_hex("invalid").is_none());
    }
}
