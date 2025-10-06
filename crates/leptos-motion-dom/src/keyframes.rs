//! Keyframes System - Multi-step animations with intermediate states
//!
//! This module provides support for complex multi-step animations by defining
//! intermediate animation states at specific points in time, enabling sophisticated
//! animation sequences beyond simple from/to transitions.

use crate::AnimationValue;
use std::collections::HashMap;

/// Re-export EasingFunction for use in keyframes
pub use crate::simple_animation_engine::EasingFunction;

/// Represents a single keyframe in an animation sequence
#[derive(Clone, Debug)]
pub struct Keyframe {
    /// Progress point (0.0 to 1.0) where this keyframe occurs
    pub progress: f64,
    /// Animation properties at this keyframe
    pub properties: HashMap<String, AnimationValue>,
    /// Optional easing function for the transition to this keyframe
    pub easing: Option<EasingFunction>,
}

impl Keyframe {
    /// Create a new keyframe at the specified progress point
    pub fn new(progress: f64, properties: HashMap<String, AnimationValue>) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            properties,
            easing: None,
        }
    }

    /// Set the easing function for this keyframe
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = Some(easing);
        self
    }

    /// Create a keyframe with only easing (no properties)
    pub fn easing_only(progress: f64, easing: EasingFunction) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            properties: HashMap::new(),
            easing: Some(easing),
        }
    }
}

/// Represents a complete keyframes animation sequence
#[derive(Clone, Debug)]
pub struct Keyframes {
    keyframes: Vec<Keyframe>,
}

impl Keyframes {
    /// Create a new keyframes sequence
    pub fn new(keyframes: Vec<Keyframe>) -> Self {
        let mut sorted_keyframes = keyframes;
        sorted_keyframes.sort_by(|a, b| a.progress.partial_cmp(&b.progress).unwrap());
        Self {
            keyframes: sorted_keyframes,
        }
    }

    /// Add a keyframe to the sequence
    pub fn add(mut self, keyframe: Keyframe) -> Self {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.progress.partial_cmp(&b.progress).unwrap());
        self
    }

    /// Get the keyframe at a specific progress point (0.0 to 1.0)
    pub fn get_at_progress(&self, progress: f64) -> HashMap<String, AnimationValue> {
        let progress = progress.clamp(0.0, 1.0);

        if self.keyframes.is_empty() {
            return HashMap::new();
        }

        // Find the exact keyframe or interpolate between keyframes
        if let Some(exact_frame) = self.keyframes.iter().find(|k| (k.progress - progress).abs() < f64::EPSILON) {
            return exact_frame.properties.clone();
        }

        // Find keyframes to interpolate between
        let mut before_idx = None;
        let mut after_idx = None;

        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.progress <= progress {
                before_idx = Some(i);
            }
            if keyframe.progress >= progress && after_idx.is_none() {
                after_idx = Some(i);
                break;
            }
        }

        match (before_idx, after_idx) {
            (Some(before), Some(after)) if before == after => {
                // Exact match
                self.keyframes[before].properties.clone()
            }
            (Some(before), Some(after)) => {
                // Interpolate between keyframes
                self.interpolate_keyframes(before, after, progress)
            }
            (Some(before), None) => {
                // After the last keyframe, return the last one
                self.keyframes[before].properties.clone()
            }
            (None, Some(after)) => {
                // Before the first keyframe, return the first one
                self.keyframes[after].properties.clone()
            }
            (None, None) => {
                // No keyframes
                HashMap::new()
            }
        }
    }

    /// Interpolate between two keyframes at a specific progress point
    fn interpolate_keyframes(&self, from_idx: usize, to_idx: usize, progress: f64) -> HashMap<String, AnimationValue> {
        let from_frame = &self.keyframes[from_idx];
        let to_frame = &self.keyframes[to_idx];

        let from_progress = from_frame.progress;
        let to_progress = to_frame.progress;

        // Calculate interpolation factor
        let factor = if (to_progress - from_progress).abs() < f64::EPSILON {
            0.0
        } else {
            (progress - from_progress) / (to_progress - from_progress)
        };

        let mut result = HashMap::new();

        // Get all unique property names from both keyframes
        let mut all_props = std::collections::HashSet::new();
        for key in from_frame.properties.keys() {
            all_props.insert(key.clone());
        }
        for key in to_frame.properties.keys() {
            all_props.insert(key.clone());
        }

        // Interpolate each property
        for prop_name in all_props {
            let from_value = from_frame.properties.get(&prop_name);
            let to_value = to_frame.properties.get(&prop_name);

            match (from_value, to_value) {
                (Some(from_val), Some(to_val)) => {
                    // Both keyframes have this property, interpolate
                    if let Some(interpolated) = Self::interpolate_values(from_val, to_val, factor) {
                        result.insert(prop_name, interpolated);
                    }
                }
                (Some(from_val), None) => {
                    // Only in from keyframe, use from value
                    result.insert(prop_name, from_val.clone());
                }
                (None, Some(to_val)) => {
                    // Only in to keyframe, use to value
                    result.insert(prop_name, to_val.clone());
                }
                (None, None) => {
                    // Should not happen
                }
            }
        }

        result
    }

    /// Interpolate between two animation values
    fn interpolate_values(from: &AnimationValue, to: &AnimationValue, factor: f64) -> Option<AnimationValue> {
        match (from, to) {
            (AnimationValue::Number(from_val), AnimationValue::Number(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * factor;
                Some(AnimationValue::Number(interpolated))
            }
            (AnimationValue::Pixels(from_val), AnimationValue::Pixels(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * factor;
                Some(AnimationValue::Pixels(interpolated))
            }
            (AnimationValue::Degrees(from_val), AnimationValue::Degrees(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * factor;
                Some(AnimationValue::Degrees(interpolated))
            }
            // For non-numeric values, use the 'to' value when factor >= 0.5, otherwise 'from'
            _ => {
                if factor >= 0.5 {
                    Some(to.clone())
                } else {
                    Some(from.clone())
                }
            }
        }
    }

    /// Get all keyframes
    pub fn keyframes(&self) -> &[Keyframe] {
        &self.keyframes
    }

    /// Get the number of keyframes
    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    /// Check if keyframes sequence is empty
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Validate the keyframes sequence
    pub fn validate(&self) -> Result<(), String> {
        if self.keyframes.is_empty() {
            return Err("Keyframes sequence cannot be empty".to_string());
        }

        // Check that progress values are in range
        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.progress < 0.0 || keyframe.progress > 1.0 {
                return Err(format!("Keyframe {} has invalid progress {} (must be between 0.0 and 1.0)", i, keyframe.progress));
            }
        }

        // Check that keyframes are in ascending order (should be sorted)
        for i in 1..self.keyframes.len() {
            if self.keyframes[i].progress < self.keyframes[i-1].progress {
                return Err("Keyframes must be in ascending progress order".to_string());
            }
        }

        Ok(())
    }

    /// Get the duration factor (useful for calculating actual animation duration)
    pub fn duration_factor(&self) -> f64 {
        if self.keyframes.is_empty() {
            1.0
        } else {
            self.keyframes.last().unwrap().progress
        }
    }
}

/// Builder pattern for creating keyframes
pub struct KeyframesBuilder {
    keyframes: Vec<Keyframe>,
}

impl KeyframesBuilder {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    pub fn keyframe(mut self, progress: f64, properties: HashMap<String, AnimationValue>) -> Self {
        self.keyframes.push(Keyframe::new(progress, properties));
        self
    }

    pub fn keyframe_with_easing(
        mut self,
        progress: f64,
        properties: HashMap<String, AnimationValue>,
        easing: EasingFunction,
    ) -> Self {
        self.keyframes.push(Keyframe::new(progress, properties).with_easing(easing));
        self
    }

    pub fn easing_at(mut self, progress: f64, easing: EasingFunction) -> Self {
        self.keyframes.push(Keyframe::easing_only(progress, easing));
        self
    }

    pub fn build(self) -> Result<Keyframes, String> {
        let keyframes = Keyframes::new(self.keyframes);
        keyframes.validate()?;
        Ok(keyframes)
    }
}

impl Default for KeyframesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro for creating keyframes
#[macro_export]
macro_rules! keyframes {
    ($($progress:expr => {$($key:expr => $value:expr),* $(,)?}),* $(,)?) => {{
        let mut builder = $crate::keyframes::KeyframesBuilder::new();
        $(
            let mut props = std::collections::HashMap::new();
            $(
                props.insert($key.into(), $value);
            )*
            builder = builder.keyframe($progress, props);
        )*
        builder.build().unwrap()
    }};

    ($($progress:expr => {$($key:expr => $value:expr),* $(,)?} with $easing:expr),* $(,)?) => {{
        let mut builder = $crate::keyframes::KeyframesBuilder::new();
        $(
            let mut props = std::collections::HashMap::new();
            $(
                props.insert($key.into(), $value);
            )*
            builder = builder.keyframe_with_easing($progress, props, $easing);
        )*
        builder.build().unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_creation() {
        let keyframe = Keyframe::new(0.5, hashmap! {
            "opacity" => AnimationValue::Number(0.8),
            "x" => AnimationValue::Pixels(100.0),
        });

        assert_eq!(keyframe.progress, 0.5);
        assert_eq!(keyframe.properties.len(), 2);
    }

    #[test]
    fn test_keyframes_creation() {
        let keyframes = Keyframes::new(vec![
            Keyframe::new(0.0, hashmap! { "opacity" => AnimationValue::Number(0.0) }),
            Keyframe::new(0.5, hashmap! { "opacity" => AnimationValue::Number(0.5) }),
            Keyframe::new(1.0, hashmap! { "opacity" => AnimationValue::Number(1.0) }),
        ]);

        assert_eq!(keyframes.len(), 3);
    }

    #[test]
    fn test_keyframes_interpolation() {
        let keyframes = Keyframes::new(vec![
            Keyframe::new(0.0, hashmap! { "x" => AnimationValue::Pixels(0.0) }),
            Keyframe::new(1.0, hashmap! { "x" => AnimationValue::Pixels(200.0) }),
        ]);

        let at_start = keyframes.get_at_progress(0.0);
        let at_middle = keyframes.get_at_progress(0.5);
        let at_end = keyframes.get_at_progress(1.0);

        assert_eq!(at_start.get("x"), Some(&AnimationValue::Pixels(0.0)));
        assert_eq!(at_middle.get("x"), Some(&AnimationValue::Pixels(100.0)));
        assert_eq!(at_end.get("x"), Some(&AnimationValue::Pixels(200.0)));
    }

    #[test]
    fn test_keyframes_validation() {
        // Valid keyframes
        let valid_keyframes = Keyframes::new(vec![
            Keyframe::new(0.0, hashmap! { "opacity" => AnimationValue::Number(0.0) }),
            Keyframe::new(0.5, hashmap! { "opacity" => AnimationValue::Number(0.5) }),
            Keyframe::new(1.0, hashmap! { "opacity" => AnimationValue::Number(1.0) }),
        ]);
        assert!(valid_keyframes.validate().is_ok());

        // Invalid keyframes (out of range)
        let invalid_keyframes = Keyframes::new(vec![
            Keyframe::new(-0.1, hashmap! { "opacity" => AnimationValue::Number(0.0) }),
        ]);
        assert!(invalid_keyframes.validate().is_err());
    }

    #[test]
    fn test_keyframes_macro() {
        let keyframes = keyframes! {
            0.0 => { "opacity" => AnimationValue::Number(0.0) },
            0.5 => { "opacity" => AnimationValue::Number(0.5) },
            1.0 => { "opacity" => AnimationValue::Number(1.0) }
        };

        assert_eq!(keyframes.len(), 3);
        assert!(keyframes.validate().is_ok());
    }

    #[test]
    fn test_keyframes_builder() {
        let keyframes = KeyframesBuilder::new()
            .keyframe(0.0, hashmap! { "scale" => AnimationValue::Number(0.8) })
            .keyframe(0.5, hashmap! { "scale" => AnimationValue::Number(1.0) })
            .keyframe(1.0, hashmap! { "scale" => AnimationValue::Number(1.2) })
            .build()
            .unwrap();

        assert_eq!(keyframes.len(), 3);
        let at_middle = keyframes.get_at_progress(0.5);
        assert_eq!(at_middle.get("scale"), Some(&AnimationValue::Number(1.0)));
    }

    #[test]
    fn test_interpolation_different_value_types() {
        let keyframes = Keyframes::new(vec![
            Keyframe::new(0.0, hashmap! {
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(0.0),
            }),
            Keyframe::new(1.0, hashmap! {
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(100.0),
            }),
        ]);

        let at_half = keyframes.get_at_progress(0.5);
        assert_eq!(at_half.get("opacity"), Some(&AnimationValue::Number(0.5)));
        assert_eq!(at_half.get("x"), Some(&AnimationValue::Pixels(50.0)));
    }
}
