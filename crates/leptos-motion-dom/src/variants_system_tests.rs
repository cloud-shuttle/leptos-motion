// Variants System Tests
//
// These tests define the expected behavior of the Variants system
// for named animation states using Test-Driven Development.

use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Animation variant definition
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationVariant {
    /// Animation target values
    pub target: HashMap<String, AnimationValue>,
    /// Transition configuration
    pub transition: Option<Transition>,
}

impl AnimationVariant {
    pub fn new(target: HashMap<String, AnimationValue>) -> Self {
        Self {
            target,
            transition: None,
        }
    }

    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }
}

/// Variants collection for named animation states
#[derive(Debug, Clone, PartialEq)]
pub struct Variants {
    /// Named animation variants
    pub variants: HashMap<String, AnimationVariant>,
}

impl Variants {
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
        }
    }

    /// Add a variant with a name
    pub fn add_variant(&mut self, name: String, variant: AnimationVariant) {
        self.variants.insert(name, variant);
    }

    /// Get a variant by name
    pub fn get_variant(&self, name: &str) -> Option<&AnimationVariant> {
        self.variants.get(name)
    }

    /// Check if a variant exists
    pub fn has_variant(&self, name: &str) -> bool {
        self.variants.contains_key(name)
    }

    /// Get all variant names
    pub fn variant_names(&self) -> Vec<&String> {
        self.variants.keys().collect()
    }

    /// Get variant count
    pub fn variant_count(&self) -> usize {
        self.variants.len()
    }
}

impl Default for Variants {
    fn default() -> Self {
        Self::new()
    }
}

/// Variants manager for handling variant state
pub struct VariantsManager {
    variants: Variants,
    current_variant: Option<String>,
    previous_variant: Option<String>,
}

impl VariantsManager {
    pub fn new(variants: Variants) -> Self {
        Self {
            variants,
            current_variant: None,
            previous_variant: None,
        }
    }

    /// Set the current variant
    pub fn set_variant(&mut self, variant_name: &str) -> bool {
        if self.variants.has_variant(variant_name) {
            self.previous_variant = self.current_variant.clone();
            self.current_variant = Some(variant_name.to_string());
            true
        } else {
            false
        }
    }

    /// Get the current variant
    pub fn current_variant(&self) -> Option<&String> {
        self.current_variant.as_ref()
    }

    /// Get the previous variant
    pub fn previous_variant(&self) -> Option<&String> {
        self.previous_variant.as_ref()
    }

    /// Get the current variant's animation target
    pub fn current_animation_target(&self) -> Option<&AnimationVariant> {
        if let Some(ref current) = self.current_variant {
            self.variants.get_variant(current)
        } else {
            None
        }
    }

    /// Get the previous variant's animation target
    pub fn previous_animation_target(&self) -> Option<&AnimationVariant> {
        if let Some(ref previous) = self.previous_variant {
            self.variants.get_variant(previous)
        } else {
            None
        }
    }

    /// Reset to no variant
    pub fn reset(&mut self) {
        self.previous_variant = self.current_variant.clone();
        self.current_variant = None;
    }

    /// Check if currently in a variant
    pub fn is_in_variant(&self) -> bool {
        self.current_variant.is_some()
    }
}

/// Test basic variants functionality
#[test]
fn test_variants_basic() {
    let mut variants = Variants::new();

    // Create a simple variant
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.0));

    let variant = AnimationVariant::new(target);
    variants.add_variant("visible".to_string(), variant);

    // Should have one variant
    assert_eq!(variants.variant_count(), 1);
    assert!(variants.has_variant("visible"));
    assert!(!variants.has_variant("hidden"));

    // Should be able to get the variant
    let visible_variant = variants.get_variant("visible").unwrap();
    assert_eq!(
        visible_variant.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );
    assert_eq!(
        visible_variant.target.get("scale").unwrap(),
        &AnimationValue::Number(1.0)
    );
}

/// Test variants with multiple states
#[test]
fn test_variants_multiple_states() {
    let mut variants = Variants::new();

    // Create visible variant
    let mut visible_target = HashMap::new();
    visible_target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    visible_target.insert("scale".to_string(), AnimationValue::Number(1.0));
    let visible_variant = AnimationVariant::new(visible_target);
    variants.add_variant("visible".to_string(), visible_variant);

    // Create hidden variant
    let mut hidden_target = HashMap::new();
    hidden_target.insert("opacity".to_string(), AnimationValue::Number(0.0));
    hidden_target.insert("scale".to_string(), AnimationValue::Number(0.8));
    let hidden_variant = AnimationVariant::new(hidden_target);
    variants.add_variant("hidden".to_string(), hidden_variant);

    // Should have two variants
    assert_eq!(variants.variant_count(), 2);
    assert!(variants.has_variant("visible"));
    assert!(variants.has_variant("hidden"));

    // Should be able to get both variants
    let visible = variants.get_variant("visible").unwrap();
    let hidden = variants.get_variant("hidden").unwrap();

    assert_eq!(
        visible.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );
    assert_eq!(
        hidden.target.get("opacity").unwrap(),
        &AnimationValue::Number(0.0)
    );
}

/// Test variants with custom transitions
#[test]
fn test_variants_with_transitions() {
    let mut variants = Variants::new();

    // Create variant with custom transition
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.0));

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.1),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    let variant = AnimationVariant::new(target).with_transition(transition.clone());
    variants.add_variant("fadeOut".to_string(), variant);

    // Should have the variant with transition
    let fade_out = variants.get_variant("fadeOut").unwrap();
    assert!(fade_out.transition.is_some());
    assert_eq!(fade_out.transition.as_ref().unwrap().duration, Some(0.5));
    assert_eq!(
        fade_out.transition.as_ref().unwrap().ease,
        Easing::EaseInOut
    );
    assert_eq!(fade_out.transition.as_ref().unwrap().delay, Some(0.1));
}

/// Test variants manager functionality
#[test]
fn test_variants_manager() {
    let mut variants = Variants::new();

    // Add variants
    let mut visible_target = HashMap::new();
    visible_target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let visible_variant = AnimationVariant::new(visible_target);
    variants.add_variant("visible".to_string(), visible_variant);

    let mut hidden_target = HashMap::new();
    hidden_target.insert("opacity".to_string(), AnimationValue::Number(0.0));
    let hidden_variant = AnimationVariant::new(hidden_target);
    variants.add_variant("hidden".to_string(), hidden_variant);

    // Create manager
    let mut manager = VariantsManager::new(variants);

    // Should start with no variant
    assert!(!manager.is_in_variant());
    assert!(manager.current_variant().is_none());
    assert!(manager.previous_variant().is_none());

    // Set to visible variant
    assert!(manager.set_variant("visible"));
    assert!(manager.is_in_variant());
    assert_eq!(manager.current_variant().unwrap(), "visible");
    assert!(manager.previous_variant().is_none());

    // Should be able to get current animation target
    let current_target = manager.current_animation_target().unwrap();
    assert_eq!(
        current_target.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );

    // Set to hidden variant
    assert!(manager.set_variant("hidden"));
    assert_eq!(manager.current_variant().unwrap(), "hidden");
    assert_eq!(manager.previous_variant().unwrap(), "visible");

    // Should be able to get previous animation target
    let previous_target = manager.previous_animation_target().unwrap();
    assert_eq!(
        previous_target.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );
}

/// Test variants manager with invalid variant
#[test]
fn test_variants_manager_invalid_variant() {
    let variants = Variants::new();
    let mut manager = VariantsManager::new(variants);

    // Try to set invalid variant
    assert!(!manager.set_variant("nonexistent"));
    assert!(!manager.is_in_variant());
    assert!(manager.current_variant().is_none());
}

/// Test variants manager reset functionality
#[test]
fn test_variants_manager_reset() {
    let mut variants = Variants::new();

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let variant = AnimationVariant::new(target);
    variants.add_variant("visible".to_string(), variant);

    let mut manager = VariantsManager::new(variants);

    // Set variant
    manager.set_variant("visible");
    assert!(manager.is_in_variant());
    assert_eq!(manager.current_variant().unwrap(), "visible");

    // Reset
    manager.reset();
    assert!(!manager.is_in_variant());
    assert!(manager.current_variant().is_none());
    assert_eq!(manager.previous_variant().unwrap(), "visible");
}

/// Test variants with complex animation targets
#[test]
fn test_variants_complex_targets() {
    let mut variants = Variants::new();

    // Create complex variant with multiple properties
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.2));
    target.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
    target.insert("x".to_string(), AnimationValue::Number(100.0));
    target.insert("y".to_string(), AnimationValue::Number(-50.0));

    let variant = AnimationVariant::new(target);
    variants.add_variant("complex".to_string(), variant);

    // Should have the complex variant
    let complex = variants.get_variant("complex").unwrap();
    assert_eq!(complex.target.len(), 5);
    assert_eq!(
        complex.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );
    assert_eq!(
        complex.target.get("scale").unwrap(),
        &AnimationValue::Number(1.2)
    );
    assert_eq!(
        complex.target.get("rotateZ").unwrap(),
        &AnimationValue::Number(45.0)
    );
    assert_eq!(
        complex.target.get("x").unwrap(),
        &AnimationValue::Number(100.0)
    );
    assert_eq!(
        complex.target.get("y").unwrap(),
        &AnimationValue::Number(-50.0)
    );
}

/// Test variants with different value types
#[test]
fn test_variants_different_value_types() {
    let mut variants = Variants::new();

    // Create variant with different value types
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.5));
    target.insert(
        "color".to_string(),
        AnimationValue::String("#ff0000".to_string()),
    );
    target.insert(
        "borderRadius".to_string(),
        AnimationValue::String("50%".to_string()),
    );

    let variant = AnimationVariant::new(target);
    variants.add_variant("mixed".to_string(), variant);

    // Should have the mixed variant
    let mixed = variants.get_variant("mixed").unwrap();
    assert_eq!(
        mixed.target.get("opacity").unwrap(),
        &AnimationValue::Number(0.5)
    );
    assert_eq!(
        mixed.target.get("color").unwrap(),
        &AnimationValue::String("#ff0000".to_string())
    );
    assert_eq!(
        mixed.target.get("borderRadius").unwrap(),
        &AnimationValue::String("50%".to_string())
    );
}

/// Test variants performance with many variants
#[test]
fn test_variants_performance() {
    let mut variants = Variants::new();

    // Add many variants
    for i in 0..1000 {
        let mut target = HashMap::new();
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(i as f64 / 1000.0),
        );

        let variant = AnimationVariant::new(target);
        variants.add_variant(format!("variant{}", i), variant);
    }

    // Should have 1000 variants
    assert_eq!(variants.variant_count(), 1000);

    // Should be able to access variants quickly
    assert!(variants.has_variant("variant500"));
    let variant = variants.get_variant("variant500").unwrap();
    assert_eq!(
        variant.target.get("opacity").unwrap(),
        &AnimationValue::Number(0.5)
    );
}

/// Test variants manager with many transitions
#[test]
fn test_variants_manager_many_transitions() {
    let mut variants = Variants::new();

    // Add variants with different transitions
    for i in 0..100 {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));

        let transition = Transition {
            duration: Some(i as f64 / 100.0),
            ease: Easing::EaseOut,
            delay: None,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let variant = AnimationVariant::new(target).with_transition(transition);
        variants.add_variant(format!("transition{}", i), variant);
    }

    let mut manager = VariantsManager::new(variants);

    // Test setting different variants
    for i in 0..100 {
        let variant_name = format!("transition{}", i);
        assert!(manager.set_variant(&variant_name));
        assert_eq!(manager.current_variant().unwrap(), &variant_name);

        let current = manager.current_animation_target().unwrap();
        assert!(current.transition.is_some());
        assert_eq!(
            current.transition.as_ref().unwrap().duration,
            Some(i as f64 / 100.0)
        );
    }
}

/// Test variants edge cases
#[test]
fn test_variants_edge_cases() {
    let mut variants = Variants::new();

    // Test empty variant
    let empty_target = HashMap::new();
    let empty_variant = AnimationVariant::new(empty_target);
    variants.add_variant("empty".to_string(), empty_variant);

    let empty = variants.get_variant("empty").unwrap();
    assert_eq!(empty.target.len(), 0);

    // Test variant with empty string name
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let variant = AnimationVariant::new(target);
    variants.add_variant("".to_string(), variant);

    assert!(variants.has_variant(""));
    assert_eq!(variants.variant_count(), 2);

    // Test overwriting variant
    let mut new_target = HashMap::new();
    new_target.insert("opacity".to_string(), AnimationValue::Number(0.0));
    let new_variant = AnimationVariant::new(new_target);
    variants.add_variant("empty".to_string(), new_variant);

    assert_eq!(variants.variant_count(), 2); // Should still be 2, not 3
    let updated_empty = variants.get_variant("empty").unwrap();
    assert_eq!(
        updated_empty.target.get("opacity").unwrap(),
        &AnimationValue::Number(0.0)
    );
}
