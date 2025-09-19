//! Memory Safety Tests for Animation Engine
//!
//! This module contains tests to verify that our memory safety fixes work correctly.

#[cfg(test)]
mod tests {
    use super::super::animation_engine::DomAnimationEngine;
    use leptos_motion_core::{Transition, Easing};

    #[test]
    fn test_animation_engine_creation() {
        let engine = DomDomAnimationEngine::new();
        // Test that we can get values (should be empty initially)
        let values = engine.get_all_values();
        assert!(values.is_empty());
    }

    #[test]
    fn test_animate_property_validation() {
        let mut engine = DomAnimationEngine::new();
        
        // Test empty property name
        let result = engine.animate_property("".to_string(), 0.0, 100.0, Transition::default());
        assert!(result.is_err());
        
        // Test valid property
        let result = engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_animate_property_finite_values() {
        let mut engine = DomAnimationEngine::new();
        
        // Test infinite values
        let result = engine.animate_property("opacity".to_string(), f64::INFINITY, 1.0, Transition::default());
        assert!(result.is_err());
        
        // Test NaN values
        let result = engine.animate_property("opacity".to_string(), f64::NAN, 1.0, Transition::default());
        assert!(result.is_err());
        
        // Test valid finite values
        let result = engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_animate_property_long_name() {
        let mut engine = DomAnimationEngine::new();
        
        // Test very long property name
        let long_name = "a".repeat(2000);
        let result = engine.animate_property(long_name, 0.0, 1.0, Transition::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_animate_properties_validation() {
        let mut engine = DomAnimationEngine::new();
        
        let mut properties = std::collections::HashMap::new();
        properties.insert("opacity".to_string(), (0.0, 1.0, Transition::default()));
        properties.insert("".to_string(), (0.0, 1.0, Transition::default())); // Invalid
        
        let result = engine.animate_properties(properties);
        assert!(result.is_err()); // Should fail due to empty property name
    }

    #[test]
    fn test_animation_engine_memory_safety() {
        let mut engine = DomAnimationEngine::new();
        
        // Test multiple animations
        for i in 0..100 {
            let property = format!("property_{}", i);
            let result = engine.animate_property(property, 0.0, 1.0, Transition::default());
            assert!(result.is_ok());
        }
        
        // Engine should have all the animations
        let values = engine.get_all_values();
        assert_eq!(values.len(), 100);
    }

    #[test]
    fn test_animation_engine_stop_and_start() {
        let mut engine = DomAnimationEngine::new();
        
        // Start an animation
        let result = engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_ok());
        
        // Check that animation was added
        let values = engine.get_all_values();
        assert!(values.contains_key("opacity"));
        
        // Stop all animations
        engine.stop_all();
        
        // Check that animations were cleared
        let values = engine.get_all_values();
        assert!(values.is_empty());
    }

    #[test]
    fn test_animation_engine_get_current_values() {
        let mut engine = DomAnimationEngine::new();
        
        // Start an animation
        let result = engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_ok());
        
        // Get current values
        let values = engine.get_all_values();
        assert!(values.contains_key("opacity"));
        assert_eq!(values["opacity"], 0.0); // Should be initial value
    }
}
