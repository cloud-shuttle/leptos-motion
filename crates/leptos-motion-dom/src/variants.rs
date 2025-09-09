//! Variants System
//!
//! A system for defining and managing named animation states

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;

/// Variants manager for named animation states
#[derive(Debug, Clone)]
pub struct Variants {
    states: HashMap<String, AnimationTarget>,
    current_state: Option<String>,
}

impl Variants {
    /// Create a new variants manager
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            current_state: None,
        }
    }
    
    /// Add a variant with the given name and animation target
    pub fn add_variant(&mut self, name: String, target: AnimationTarget) {
        self.states.insert(name, target);
    }
    
    /// Set the current variant
    pub fn set_variant(&mut self, name: &str) -> bool {
        if self.states.contains_key(name) {
            self.current_state = Some(name.to_string());
            true
        } else {
            false
        }
    }
    
    /// Get the current variant name
    pub fn current_variant(&self) -> Option<&String> {
        self.current_state.as_ref()
    }
    
    /// Get the current animation target
    pub fn current_animation_target(&self) -> Option<&AnimationTarget> {
        if let Some(ref state) = self.current_state {
            self.states.get(state)
        } else {
            None
        }
    }
    
    /// Remove a variant
    pub fn remove_variant(&mut self, name: &str) -> Option<AnimationTarget> {
        if self.current_state.as_ref() == Some(&name.to_string()) {
            self.current_state = None;
        }
        self.states.remove(name)
    }
    
    /// Clear all variants
    pub fn clear_variants(&mut self) {
        self.states.clear();
        self.current_state = None;
    }
    
    /// Get variant names
    pub fn variant_names(&self) -> Vec<String> {
        self.states.keys().cloned().collect()
    }
    
    /// Check if a variant exists
    pub fn has_variant(&self, name: &str) -> bool {
        self.states.contains_key(name)
    }
    
    /// Get a variant by name
    pub fn get_variant(&self, name: &str) -> Option<&AnimationTarget> {
        self.states.get(name)
    }
    
    /// Get all variants
    pub fn get_all_variants(&self) -> &HashMap<String, AnimationTarget> {
        &self.states
    }
    
    /// Get variant count
    pub fn variant_count(&self) -> usize {
        self.states.len()
    }
    
    /// Check if any variant is currently active
    pub fn has_active_variant(&self) -> bool {
        self.current_state.is_some()
    }
}

impl Default for Variants {
    fn default() -> Self {
        Self::new()
    }
}

/// Variants with transitions
#[derive(Debug, Clone)]
pub struct VariantsWithTransitions {
    variants: Variants,
    transitions: HashMap<String, Transition>,
}

impl VariantsWithTransitions {
    /// Create a new variants manager with transitions
    pub fn new() -> Self {
        Self {
            variants: Variants::new(),
            transitions: HashMap::new(),
        }
    }
    
    /// Add a variant with transition
    pub fn add_variant_with_transition(
        &mut self,
        name: String,
        target: AnimationTarget,
        transition: Transition,
    ) {
        self.variants.add_variant(name.clone(), target);
        self.transitions.insert(name, transition);
    }
    
    /// Set variant with transition
    pub fn set_variant(&mut self, name: &str) -> bool {
        self.variants.set_variant(name)
    }
    
    /// Get current variant
    pub fn current_variant(&self) -> Option<&String> {
        self.variants.current_variant()
    }
    
    /// Get current animation target
    pub fn current_animation_target(&self) -> Option<&AnimationTarget> {
        self.variants.current_animation_target()
    }
    
    /// Get current transition
    pub fn current_transition(&self) -> Option<&Transition> {
        if let Some(ref state) = self.variants.current_state {
            self.transitions.get(state)
        } else {
            None
        }
    }
    
    /// Get transition for a variant
    pub fn get_variant_transition(&self, name: &str) -> Option<&Transition> {
        self.transitions.get(name)
    }
    
    /// Remove variant and its transition
    pub fn remove_variant(&mut self, name: &str) -> (Option<AnimationTarget>, Option<Transition>) {
        let target = self.variants.remove_variant(name);
        let transition = self.transitions.remove(name);
        (target, transition)
    }
    
    /// Clear all variants and transitions
    pub fn clear(&mut self) {
        self.variants.clear_variants();
        self.transitions.clear();
    }
    
    /// Get variant count
    pub fn variant_count(&self) -> usize {
        self.variants.variant_count()
    }
}

impl Default for VariantsWithTransitions {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook for using variants in Leptos components
pub fn use_variants() -> (
    ReadSignal<Option<String>>,
    WriteSignal<Option<String>>,
    ReadSignal<Option<AnimationTarget>>,
    WriteSignal<HashMap<String, AnimationTarget>>,
) {
    let (current_variant, set_current_variant) = signal(None::<String>);
    let (current_target, set_current_target) = signal(None::<AnimationTarget>);
    let (variants, set_variants) = signal(HashMap::<String, AnimationTarget>::new());
    
    // Update current target when variant changes
    create_effect(move |_| {
        if let Some(variant) = current_variant.get() {
            if let Some(target) = variants.get().get(&variant) {
                set_current_target.set(Some(target.clone()));
            }
        } else {
            set_current_target.set(None);
        }
    });
    
    (current_variant, set_current_variant, current_target, set_variants)
}

/// Utility functions for common variants
pub mod common_variants {
    use super::*;
    
    /// Create a fade variant
    pub fn fade(opacity: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), leptos_motion_core::AnimationValue::Number(opacity));
        target
    }
    
    /// Create a scale variant
    pub fn scale(scale: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("scale".to_string(), leptos_motion_core::AnimationValue::Number(scale));
        target
    }
    
    /// Create a rotate variant
    pub fn rotate(rotation: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("rotateZ".to_string(), leptos_motion_core::AnimationValue::Number(rotation));
        target
    }
    
    /// Create a translate variant
    pub fn translate(x: f64, y: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("x".to_string(), leptos_motion_core::AnimationValue::Number(x));
        target.insert("y".to_string(), leptos_motion_core::AnimationValue::Number(y));
        target
    }
    
    /// Create a combined transform variant
    pub fn transform(scale: f64, rotation: f64, x: f64, y: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("scale".to_string(), leptos_motion_core::AnimationValue::Number(scale));
        target.insert("rotateZ".to_string(), leptos_motion_core::AnimationValue::Number(rotation));
        target.insert("x".to_string(), leptos_motion_core::AnimationValue::Number(x));
        target.insert("y".to_string(), leptos_motion_core::AnimationValue::Number(y));
        target
    }
    
    /// Create a color variant
    pub fn color(r: f64, g: f64, b: f64) -> AnimationTarget {
        let mut target = HashMap::new();
        target.insert("backgroundColor".to_string(), leptos_motion_core::AnimationValue::String(
            format!("rgb({}, {}, {})", r, g, b)
        ));
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variants_basic() {
        let mut variants = Variants::new();
        
        assert_eq!(variants.variant_count(), 0);
        assert!(!variants.has_active_variant());
        
        let target = common_variants::fade(1.0);
        variants.add_variant("visible".to_string(), target);
        
        assert_eq!(variants.variant_count(), 1);
        assert!(variants.has_variant("visible"));
        
        assert!(variants.set_variant("visible"));
        assert!(variants.has_active_variant());
        assert_eq!(variants.current_variant().unwrap(), "visible");
    }
    
    #[test]
    fn test_variants_with_transitions() {
        let mut variants = VariantsWithTransitions::new();
        
        let target = common_variants::scale(1.2);
        let transition = Transition {
            duration: Some(0.3),
            ease: leptos_motion_core::Easing::EaseOut,
            delay: None,
            repeat: leptos_motion_core::RepeatConfig::Never,
            stagger: None,
        };
        
        variants.add_variant_with_transition("hover".to_string(), target, transition.clone());
        
        assert_eq!(variants.variant_count(), 1);
        
        assert!(variants.set_variant("hover"));
        assert_eq!(variants.current_variant().unwrap(), "hover");
        assert_eq!(variants.current_transition().unwrap(), &transition);
    }
    
    #[test]
    fn test_common_variants() {
        let fade_target = common_variants::fade(0.5);
        let scale_target = common_variants::scale(1.5);
        let rotate_target = common_variants::rotate(45.0);
        let translate_target = common_variants::translate(10.0, 20.0);
        
        assert!(fade_target.contains_key("opacity"));
        assert!(scale_target.contains_key("scale"));
        assert!(rotate_target.contains_key("rotateZ"));
        assert!(translate_target.contains_key("x"));
        assert!(translate_target.contains_key("y"));
    }
    
    #[test]
    fn test_variants_removal() {
        let mut variants = Variants::new();
        
        let target = common_variants::fade(1.0);
        variants.add_variant("visible".to_string(), target);
        variants.set_variant("visible");
        
        assert!(variants.has_active_variant());
        
        let removed = variants.remove_variant("visible");
        assert!(removed.is_some());
        assert!(!variants.has_variant("visible"));
        assert!(!variants.has_active_variant());
    }
}
