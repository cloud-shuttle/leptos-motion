//! Variants System - Named animation states and reusable definitions
//!
//! This module provides the core functionality for defining named animation states
//! (variants) that can be applied to components, enabling complex state-based animations
//! and reusable animation definitions.

use crate::AnimationValue;
use std::collections::HashMap;

/// Represents a collection of named animation variants
#[derive(Clone, Debug)]
pub struct Variants {
    /// Map of variant names to their animation properties
    variants: HashMap<String, HashMap<String, AnimationValue>>,
    /// Default transition configuration for all variants
    default_transition: Option<crate::Transition>,
}

impl Variants {
    /// Create a new empty Variants collection
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            default_transition: None,
        }
    }

    /// Add a named variant with its animation properties
    pub fn add<S: Into<String>>(
        mut self,
        name: S,
        properties: HashMap<String, AnimationValue>,
    ) -> Self {
        self.variants.insert(name.into(), properties);
        self
    }

    /// Set a default transition for all variants
    pub fn with_transition(mut self, transition: crate::Transition) -> Self {
        self.default_transition = Some(transition);
        self
    }

    /// Get a variant by name
    pub fn get(&self, name: &str) -> Option<&HashMap<String, AnimationValue>> {
        self.variants.get(name)
    }

    /// Get the default transition
    pub fn default_transition(&self) -> Option<&crate::Transition> {
        self.default_transition.as_ref()
    }

    /// Check if a variant exists
    pub fn has_variant(&self, name: &str) -> bool {
        self.variants.contains_key(name)
    }

    /// Get all variant names
    pub fn variant_names(&self) -> Vec<&String> {
        self.variants.keys().collect()
    }

    /// Merge variant properties with explicit properties (explicit takes precedence)
    pub fn resolve_variant(
        &self,
        variant_name: &str,
        explicit_props: Option<&HashMap<String, AnimationValue>>,
    ) -> Option<HashMap<String, AnimationValue>> {
        let variant_props = self.get(variant_name)?;

        let mut resolved = variant_props.clone();

        // Merge explicit properties (they take precedence)
        if let Some(explicit) = explicit_props {
            for (key, value) in explicit {
                resolved.insert(key.clone(), value.clone());
            }
        }

        Some(resolved)
    }

    /// Get the number of variants
    pub fn len(&self) -> usize {
        self.variants.len()
    }

    /// Check if variants collection is empty
    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }
}

impl Default for Variants {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for converting various types into Variants
pub trait IntoVariants {
    fn into_variants(self) -> Variants;
}

impl IntoVariants for Variants {
    fn into_variants(self) -> Variants {
        self
    }
}

impl IntoVariants for HashMap<String, HashMap<String, AnimationValue>> {
    fn into_variants(self) -> Variants {
        let mut variants = Variants::new();
        for (name, properties) in self {
            variants = variants.add(name, properties);
        }
        variants
    }
}

/// Builder pattern for creating variants
pub struct VariantsBuilder {
    variants: Variants,
}

impl VariantsBuilder {
    pub fn new() -> Self {
        Self {
            variants: Variants::new(),
        }
    }

    pub fn variant<S: Into<String>>(
        mut self,
        name: S,
        properties: HashMap<String, AnimationValue>,
    ) -> Self {
        self.variants = self.variants.add(name, properties);
        self
    }

    pub fn transition(mut self, transition: crate::Transition) -> Self {
        self.variants = self.variants.with_transition(transition);
        self
    }

    pub fn build(self) -> Variants {
        self.variants
    }
}

impl Default for VariantsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro for creating variants
#[macro_export]
macro_rules! variants {
    ($($name:expr => {$($key:expr => $value:expr),* $(,)?}),* $(,)?) => {{
        let mut variants = $crate::variants::Variants::new();
        $(
            let mut props = std::collections::HashMap::new();
            $(
                props.insert($key.into(), $value);
            )*
            variants = variants.add($name, props);
        )*
        variants
    }};

    ($($name:expr => {$($key:expr => $value:expr),* $(,)?}),* $(,)? ; transition = $transition:expr) => {{
        let mut variants = variants!($($name => {$($key => $value),*}),* );
        variants = variants.with_transition($transition);
        variants
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnimationValue, Transition, Easing};

    #[test]
    fn test_variants_creation() {
        let variants = Variants::new()
            .add("initial", hashmap! {
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8),
            })
            .add("animate", hashmap! {
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(1.0),
            });

        assert!(variants.has_variant("initial"));
        assert!(variants.has_variant("animate"));
        assert!(!variants.has_variant("exit"));
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn test_variant_resolution() {
        let variants = Variants::new()
            .add("initial", hashmap! {
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8),
            });

        let resolved = variants.resolve_variant("initial", None).unwrap();
        assert_eq!(resolved.get("opacity"), Some(&AnimationValue::Number(0.0)));
        assert_eq!(resolved.get("scale"), Some(&AnimationValue::Number(0.8)));
    }

    #[test]
    fn test_variant_resolution_with_explicit_override() {
        let variants = Variants::new()
            .add("initial", hashmap! {
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8),
            });

        let explicit = hashmap! {
            "opacity" => AnimationValue::Number(0.5), // Override
            "x" => AnimationValue::Pixels(100.0),     // Add new
        };

        let resolved = variants.resolve_variant("initial", Some(&explicit)).unwrap();
        assert_eq!(resolved.get("opacity"), Some(&AnimationValue::Number(0.5))); // Overridden
        assert_eq!(resolved.get("scale"), Some(&AnimationValue::Number(0.8)));   // From variant
        assert_eq!(resolved.get("x"), Some(&AnimationValue::Pixels(100.0)));     // Added
    }

    #[test]
    fn test_variants_with_transition() {
        let transition = Transition {
            duration: Some(0.5),
            ease: Some(Easing::EaseOut),
            ..Default::default()
        };

        let variants = Variants::new()
            .add("enter", hashmap! { "opacity" => AnimationValue::Number(1.0) })
            .with_transition(transition.clone());

        assert_eq!(variants.default_transition(), Some(&transition));
    }

    #[test]
    fn test_variants_macro() {
        let variants = variants! {
            "initial" => {
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8),
            },
            "animate" => {
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(1.0),
            }
        };

        assert!(variants.has_variant("initial"));
        assert!(variants.has_variant("animate"));
    }

    #[test]
    fn test_variants_builder() {
        let variants = VariantsBuilder::new()
            .variant("hover", hashmap! { "scale" => AnimationValue::Number(1.1) })
            .variant("tap", hashmap! { "scale" => AnimationValue::Number(0.95) })
            .build();

        assert!(variants.has_variant("hover"));
        assert!(variants.has_variant("tap"));
        assert_eq!(variants.len(), 2);
    }
}