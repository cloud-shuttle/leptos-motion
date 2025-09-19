//! Test to verify the animation engine is working

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation_engine::AnimationEngine;
    use leptos_motion_core::{Transition, Easing};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_animation_engine_creation() {
        let engine = AnimationEngine::new();
        assert_eq!(engine.get_all_values().len(), 0);
    }

    #[test]
    fn test_animation_engine_basic_animation() {
        let mut engine = AnimationEngine::new();
        
        // Set up a callback to track updates
        let updates = Rc::new(RefCell::new(Vec::new()));
        let updates_clone = updates.clone();
        
        engine.on_update(move |values| {
            updates_clone.borrow_mut().push(values.clone());
        });
        
        // Start a simple animation
        let transition = Transition {
            duration: Some(0.1), // Very short for testing
            ease: Easing::Linear,
            ..Default::default()
        };
        
        let result = engine.animate_property("x".to_string(), 0.0, 100.0, transition);
        assert!(result.is_ok());
        
        // Check that we have the animation
        assert_eq!(engine.get_all_values().len(), 1);
        assert_eq!(engine.get_property_value("x"), Some(0.0));
    }

    #[test]
    fn test_animation_engine_multiple_properties() {
        let mut engine = AnimationEngine::new();
        
        let transition = Transition {
            duration: Some(0.1),
            ease: Easing::Linear,
            ..Default::default()
        };
        
        // Animate multiple properties
        let result1 = engine.animate_property("x".to_string(), 0.0, 100.0, transition.clone());
        let result2 = engine.animate_property("y".to_string(), 0.0, 200.0, transition);
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        // Check that we have both animations
        assert_eq!(engine.get_all_values().len(), 2);
        assert_eq!(engine.get_property_value("x"), Some(0.0));
        assert_eq!(engine.get_property_value("y"), Some(0.0));
    }

    #[test]
    fn test_animation_engine_stop_property() {
        let mut engine = AnimationEngine::new();
        
        let transition = Transition {
            duration: Some(0.1),
            ease: Easing::Linear,
            ..Default::default()
        };
        
        // Start animation
        let result = engine.animate_property("x".to_string(), 0.0, 100.0, transition);
        assert!(result.is_ok());
        assert_eq!(engine.get_all_values().len(), 1);
        
        // Stop animation
        engine.stop_property("x");
        assert_eq!(engine.get_all_values().len(), 0);
    }

    #[test]
    fn test_animation_engine_stop_all() {
        let mut engine = AnimationEngine::new();
        
        let transition = Transition {
            duration: Some(0.1),
            ease: Easing::Linear,
            ..Default::default()
        };
        
        // Start multiple animations
        let _ = engine.animate_property("x".to_string(), 0.0, 100.0, transition.clone());
        let _ = engine.animate_property("y".to_string(), 0.0, 200.0, transition);
        assert_eq!(engine.get_all_values().len(), 2);
        
        // Stop all animations
        engine.stop_all();
        assert_eq!(engine.get_all_values().len(), 0);
    }
}
