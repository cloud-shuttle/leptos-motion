//! Minimal serialization support without serde
//!
//! This module provides basic serialization capabilities for core types
//! without the overhead of the full serde ecosystem.

use std::collections::HashMap;
use crate::types::*;

/// Minimal serialization trait for core types
pub trait MinimalSerialize {
    fn to_json(&self) -> String;
}

/// Minimal deserialization trait for core types
pub trait MinimalDeserialize: Sized {
    fn from_json(json: &str) -> Result<Self, String>;
}

impl MinimalSerialize for AnimationValue {
    fn to_json(&self) -> String {
        match self {
            AnimationValue::Number(n) => format!("{{\"type\":\"number\",\"value\":{}}}", n),
            AnimationValue::Pixels(p) => format!("{{\"type\":\"pixels\",\"value\":{}}}", p),
            AnimationValue::Percentage(p) => format!("{{\"type\":\"percentage\",\"value\":{}}}", p),
            AnimationValue::Degrees(d) => format!("{{\"type\":\"degrees\",\"value\":{}}}", d),
            AnimationValue::Radians(r) => format!("{{\"type\":\"radians\",\"value\":{}}}", r),
            AnimationValue::Color(c) => format!("{{\"type\":\"color\",\"value\":\"{}\"}}", c),
            AnimationValue::Transform(t) => format!("{{\"type\":\"transform\",\"value\":{}}}", t.to_json()),
            AnimationValue::String(s) => format!("{{\"type\":\"string\",\"value\":\"{}\"}}", s),
            AnimationValue::Complex(c) => format!("{{\"type\":\"complex\",\"value\":{}}}", c.to_json()),
        }
    }
}

impl MinimalSerialize for Transform {
    fn to_json(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(x) = self.x {
            parts.push(format!("\"x\":{}", x));
        }
        if let Some(y) = self.y {
            parts.push(format!("\"y\":{}", y));
        }
        if let Some(z) = self.z {
            parts.push(format!("\"z\":{}", z));
        }
        if let Some(rx) = self.rotate_x {
            parts.push(format!("\"rotate_x\":{}", rx));
        }
        if let Some(ry) = self.rotate_y {
            parts.push(format!("\"rotate_y\":{}", ry));
        }
        if let Some(rz) = self.rotate_z {
            parts.push(format!("\"rotate_z\":{}", rz));
        }
        if let Some(s) = self.scale {
            parts.push(format!("\"scale\":{}", s));
        }
        if let Some(sx) = self.scale_x {
            parts.push(format!("\"scale_x\":{}", sx));
        }
        if let Some(sy) = self.scale_y {
            parts.push(format!("\"scale_y\":{}", sy));
        }
        if let Some(skew_x) = self.skew_x {
            parts.push(format!("\"skew_x\":{}", skew_x));
        }
        if let Some(skew_y) = self.skew_y {
            parts.push(format!("\"skew_y\":{}", skew_y));
        }
        
        format!("{{{}}}", parts.join(","))
    }
}

impl MinimalSerialize for ComplexValue {
    fn to_json(&self) -> String {
        // For complex values, we'll use a simple string representation
        // In a real implementation, you might want to handle this differently
        format!("{{\"type\":\"{}\",\"data\":\"{}\"}}", self.value_type, "complex_data")
    }
}

impl MinimalSerialize for AnimationTarget {
    fn to_json(&self) -> String {
        let mut parts = Vec::new();
        
        for (key, value) in self {
            parts.push(format!("\"{}\":{}", key, value.to_json()));
        }
        
        format!("{{{}}}", parts.join(","))
    }
}

impl MinimalSerialize for Transition {
    fn to_json(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(duration) = self.duration {
            parts.push(format!("\"duration\":{}", duration));
        }
        if let Some(delay) = self.delay {
            parts.push(format!("\"delay\":{}", delay));
        }
        
        parts.push(format!("\"ease\":\"{}\"", self.ease.to_string()));
        parts.push(format!("\"repeat\":\"{}\"", self.repeat.to_string()));
        
        if let Some(stagger) = &self.stagger {
            parts.push(format!("\"stagger\":{}", stagger.to_json()));
        }
        
        format!("{{{}}}", parts.join(","))
    }
}

impl MinimalSerialize for StaggerConfig {
    fn to_json(&self) -> String {
        format!("{{\"delay\":{},\"from\":\"{}\"}}", self.delay, self.from.to_string())
    }
}

impl MinimalSerialize for SpringConfig {
    fn to_json(&self) -> String {
        format!(
            "{{\"stiffness\":{},\"damping\":{},\"mass\":{},\"velocity\":{},\"rest_delta\":{},\"rest_speed\":{}}}",
            self.stiffness, self.damping, self.mass, self.velocity, self.rest_delta, self.rest_speed
        )
    }
}

// Extension traits for string conversion
pub trait ToStringExt {
    fn to_string(&self) -> String;
}

impl ToStringExt for Easing {
    fn to_string(&self) -> String {
        match self {
            Easing::Linear => "linear".to_string(),
            Easing::EaseIn => "ease-in".to_string(),
            Easing::EaseOut => "ease-out".to_string(),
            Easing::EaseInOut => "ease-in-out".to_string(),
            Easing::CircIn => "circ-in".to_string(),
            Easing::CircOut => "circ-out".to_string(),
            Easing::CircInOut => "circ-in-out".to_string(),
            Easing::BackIn => "back-in".to_string(),
            Easing::BackOut => "back-out".to_string(),
            Easing::BackInOut => "back-in-out".to_string(),
            Easing::Spring(_) => "spring".to_string(),
            Easing::Bezier(_, _, _, _) => "bezier".to_string(),
        }
    }
}

impl ToStringExt for RepeatConfig {
    fn to_string(&self) -> String {
        match self {
            RepeatConfig::Never => "never".to_string(),
            RepeatConfig::Count(n) => format!("count:{}", n),
            RepeatConfig::Infinite => "infinite".to_string(),
            RepeatConfig::InfiniteReverse => "infinite-reverse".to_string(),
        }
    }
}

impl ToStringExt for StaggerFrom {
    fn to_string(&self) -> String {
        match self {
            StaggerFrom::First => "first".to_string(),
            StaggerFrom::Last => "last".to_string(),
            StaggerFrom::Center => "center".to_string(),
            StaggerFrom::Index(n) => format!("index:{}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_animation_value_serialization() {
        let value = AnimationValue::Number(42.0);
        let json = value.to_json();
        assert!(json.contains("\"type\":\"number\""));
        assert!(json.contains("\"value\":42"));
    }

    #[test]
    fn test_transform_serialization() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            scale: Some(1.5),
            ..Default::default()
        };
        let json = transform.to_json();
        assert!(json.contains("\"x\":10"));
        assert!(json.contains("\"y\":20"));
        assert!(json.contains("\"scale\":1.5"));
    }

    #[test]
    fn test_animation_target_serialization() {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        let animation_target: AnimationTarget = target;
        
        let json = animation_target.to_json();
        assert!(json.contains("\"opacity\""));
        assert!(json.contains("\"x\""));
    }

    #[test]
    fn test_transition_serialization() {
        let transition = Transition {
            duration: Some(1.0),
            ease: Easing::EaseOut,
            ..Default::default()
        };
        let json = transition.to_json();
        assert!(json.contains("\"duration\":1"));
        assert!(json.contains("\"ease\":\"ease-out\""));
    }
}
