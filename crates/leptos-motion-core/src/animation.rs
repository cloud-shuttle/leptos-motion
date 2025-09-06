//! Core animation types and builders

use crate::{AnimationTarget, AnimationValue, Easing, Transition};
use std::collections::HashMap;

/// Animation builder for creating animations fluently
#[derive(Debug, Clone)]
pub struct AnimationBuilder {
    initial: Option<AnimationTarget>,
    animate: Option<AnimationTarget>,
    exit: Option<AnimationTarget>,
    transition: Option<Transition>,
    variants: Option<Variants>,
}

impl AnimationBuilder {
    /// Create a new animation builder
    pub fn new() -> Self {
        Self {
            initial: None,
            animate: None,
            exit: None,
            transition: None,
            variants: None,
        }
    }

    /// Set initial animation values
    pub fn initial(mut self, initial: AnimationTarget) -> Self {
        self.initial = Some(initial);
        self
    }

    /// Set target animation values
    pub fn animate(mut self, animate: AnimationTarget) -> Self {
        self.animate = Some(animate);
        self
    }

    /// Set exit animation values
    pub fn exit(mut self, exit: AnimationTarget) -> Self {
        self.exit = Some(exit);
        self
    }

    /// Set transition configuration
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }

    /// Set variants
    pub fn variants(mut self, variants: Variants) -> Self {
        self.variants = Some(variants);
        self
    }

    /// Build the final animation configuration
    pub fn build(self) -> AnimationConfig {
        AnimationConfig {
            initial: self.initial.unwrap_or_default(),
            animate: self.animate.unwrap_or_default(),
            exit: self.exit.unwrap_or_default(),
            transition: self.transition.unwrap_or_default(),
            variants: self.variants,
        }
    }
}

/// Complete animation configuration
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// Initial animation state
    pub initial: AnimationTarget,
    /// Target animation state
    pub animate: AnimationTarget,
    /// Exit animation state
    pub exit: AnimationTarget,
    /// Transition configuration
    pub transition: Transition,
    /// Animation variants
    pub variants: Option<Variants>,
}

/// Animation variants for orchestrated animations
#[derive(Debug, Clone)]
pub struct Variants {
    /// All variant definitions
    pub variants: HashMap<String, AnimationTarget>,
    /// Initial variant
    pub initial: Option<String>,
    /// Animate variant
    pub animate: Option<String>,
    /// Exit variant
    pub exit: Option<String>,
    /// Hover variant
    pub hover: Option<String>,
    /// Tap variant
    pub tap: Option<String>,
    /// Focus variant
    pub focus: Option<String>,
    /// Drag variant
    pub drag: Option<String>,
}

impl Variants {
    /// Create a new variants configuration
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            initial: None,
            animate: None,
            exit: None,
            hover: None,
            tap: None,
            focus: None,
            drag: None,
        }
    }

    /// Add a variant
    pub fn add_variant(mut self, name: impl Into<String>, target: AnimationTarget) -> Self {
        self.variants.insert(name.into(), target);
        self
    }

    /// Set initial variant
    pub fn initial(mut self, name: impl Into<String>) -> Self {
        self.initial = Some(name.into());
        self
    }

    /// Set animate variant
    pub fn animate(mut self, name: impl Into<String>) -> Self {
        self.animate = Some(name.into());
        self
    }

    /// Set exit variant
    pub fn exit(mut self, name: impl Into<String>) -> Self {
        self.exit = Some(name.into());
        self
    }

    /// Set hover variant
    pub fn hover(mut self, name: impl Into<String>) -> Self {
        self.hover = Some(name.into());
        self
    }

    /// Set tap variant
    pub fn tap(mut self, name: impl Into<String>) -> Self {
        self.tap = Some(name.into());
        self
    }

    /// Get a variant by name
    pub fn get_variant(&self, name: &str) -> Option<&AnimationTarget> {
        self.variants.get(name)
    }
}

/// Animation keyframe sequence
#[derive(Debug, Clone)]
pub struct Keyframes {
    /// Keyframe positions (0.0 to 1.0)
    pub times: Vec<f64>,
    /// Values at each keyframe
    pub values: Vec<AnimationTarget>,
    /// Easing between keyframes
    pub easings: Vec<Easing>,
}

impl Keyframes {
    /// Create a new keyframe sequence
    pub fn new() -> Self {
        Self {
            times: Vec::new(),
            values: Vec::new(),
            easings: Vec::new(),
        }
    }

    /// Add a keyframe
    pub fn add(mut self, time: f64, value: AnimationTarget, easing: Option<Easing>) -> Self {
        self.times.push(time.clamp(0.0, 1.0));
        self.values.push(value);
        self.easings.push(easing.unwrap_or(Easing::Linear));
        self
    }

    /// Get interpolated value at given progress
    pub fn interpolate_at(&self, progress: f64) -> Option<AnimationTarget> {
        if self.values.is_empty() {
            return None;
        }

        if self.values.len() == 1 {
            return Some(self.values[0].clone());
        }

        let progress = progress.clamp(0.0, 1.0);

        // Find the keyframe segment
        for i in 0..self.times.len() - 1 {
            if progress >= self.times[i] && progress <= self.times[i + 1] {
                let segment_progress =
                    (progress - self.times[i]) / (self.times[i + 1] - self.times[i]);
                let eased_progress = self.easings[i].evaluate(segment_progress);

                return Some(self.interpolate_targets(
                    &self.values[i],
                    &self.values[i + 1],
                    eased_progress,
                ));
            }
        }

        // Return last value if beyond range
        Some(self.values.last().unwrap().clone())
    }

    fn interpolate_targets(
        &self,
        from: &AnimationTarget,
        to: &AnimationTarget,
        progress: f64,
    ) -> AnimationTarget {
        let mut result = HashMap::new();

        // Collect all property names
        let mut properties: std::collections::HashSet<_> = from.keys().collect();
        properties.extend(to.keys());

        for property in properties {
            let from_value = from
                .get(property)
                .cloned()
                .unwrap_or(AnimationValue::Number(0.0));
            let to_value = to
                .get(property)
                .cloned()
                .unwrap_or(AnimationValue::Number(0.0));

            result.insert(
                property.clone(),
                from_value.interpolate(&to_value, progress),
            );
        }

        result
    }
}

/// Convenience macros for creating animation targets
#[macro_export]
macro_rules! animate {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut target = std::collections::HashMap::new();
            $(
                target.insert($key.to_string(), $value.into());
            )*
            target
        }
    };
}

/// Convenience functions for common animations
// Animation presets are now in a separate file
pub mod presets;

impl Default for AnimationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            initial: HashMap::new(),
            animate: HashMap::new(),
            exit: HashMap::new(),
            transition: Transition::default(),
            variants: None,
        }
    }
}

impl Default for Variants {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Keyframes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnimationPresets;

    #[test]
    fn test_animation_builder() {
        let animation = AnimationBuilder::new()
            .initial(animate!("opacity" => AnimationValue::Number(0.0)))
            .animate(animate!("opacity" => AnimationValue::Number(1.0)))
            .transition(Transition {
                duration: Some(1.0),
                ..Default::default()
            })
            .build();

        assert!(animation.initial.contains_key("opacity"));
        assert!(animation.animate.contains_key("opacity"));
        assert_eq!(animation.transition.duration, Some(1.0));
    }

    #[test]
    fn test_variants() {
        let variants = Variants::new()
            .add_variant("hidden", animate!("opacity" => AnimationValue::Number(0.0)))
            .add_variant(
                "visible",
                animate!("opacity" => AnimationValue::Number(1.0)),
            )
            .initial("hidden")
            .animate("visible");

        assert!(variants.get_variant("hidden").is_some());
        assert!(variants.get_variant("visible").is_some());
        assert_eq!(variants.initial, Some("hidden".to_string()));
        assert_eq!(variants.animate, Some("visible".to_string()));
    }

    #[test]
    fn test_keyframes() {
        let keyframes = Keyframes::new()
            .add(0.0, animate!("scale" => AnimationValue::Number(1.0)), None)
            .add(0.5, animate!("scale" => AnimationValue::Number(1.2)), None)
            .add(1.0, animate!("scale" => AnimationValue::Number(1.0)), None);

        let mid_value = keyframes.interpolate_at(0.25).unwrap();
        assert!(mid_value.contains_key("scale"));

        // Should be between 1.0 and 1.2
        if let AnimationValue::Number(scale) = &mid_value["scale"] {
            assert!(*scale > 1.0 && *scale < 1.2);
        }
    }

    #[test]
    fn test_presets() {
        let fade_in = AnimationPresets::fade_in();
        assert!(fade_in.initial.contains_key("opacity"));
        assert!(fade_in.animate.contains_key("opacity"));

        let slide_up = AnimationPresets::slide_up(50.0);
        assert!(slide_up.initial.contains_key("y"));

        let bounce = AnimationPresets::bounce();
        assert_eq!(bounce.times.len(), 3);
        assert_eq!(bounce.values.len(), 3);
    }

    #[test]
    fn test_animate_macro() {
        let target: HashMap<String, AnimationValue> = animate!(
            "opacity" => AnimationValue::Number(0.5),
            "x" => AnimationValue::Pixels(100.0)
        );

        assert_eq!(target.len(), 2);
        assert!(target.contains_key("opacity"));
        assert!(target.contains_key("x"));
    }
}
