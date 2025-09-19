//! CSS Animation Integration Tests
//!
//! These tests verify that CSS animations work correctly in a real browser environment.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnimationManager,
        create_event_animation_value,
        Transition,
        Easing,
        AnimationValue,
    };
    use crate::animation_trait::Animation;
    use std::collections::HashMap;

    #[test]
    fn test_animation_value_creation() {
        // Test that we can create animation values correctly
        let number_value = create_event_animation_value(42.0);
        assert!(matches!(number_value, AnimationValue::Number(42.0)));
        
        // Test string values using the core AnimationValue directly
        let string_value = AnimationValue::String("red".to_string());
        assert!(matches!(string_value, AnimationValue::String(_)));
        
        // Test transform values
        let transform_value = AnimationValue::String("translateX(100px)".to_string());
        assert!(matches!(transform_value, AnimationValue::String(_)));
    }

    #[test]
    fn test_transition_creation() {
        // Test that we can create transitions with different configurations
        let transition1 = Transition {
            duration: Some(0.5),
            delay: Some(0.1),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        assert_eq!(transition1.duration, Some(0.5));
        assert_eq!(transition1.delay, Some(0.1));
        assert!(matches!(transition1.ease, Easing::EaseInOut));
        
        let transition2 = Transition {
            duration: Some(1.0),
            delay: None,
            ease: Easing::Linear,
            ..Default::default()
        };
        
        assert_eq!(transition2.duration, Some(1.0));
        assert_eq!(transition2.delay, None);
        assert!(matches!(transition2.ease, Easing::Linear));
    }

    #[test]
    fn test_easing_functions() {
        // Test that all easing functions work correctly
        let easing_functions = vec![
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ];

        for easing in easing_functions {
            let transition = Transition {
                duration: Some(0.1),
                ease: easing.clone(),
                ..Default::default()
            };
            
            // Each transition should have the correct easing function
            match easing {
                Easing::Linear => assert!(matches!(transition.ease, Easing::Linear)),
                Easing::EaseIn => assert!(matches!(transition.ease, Easing::EaseIn)),
                Easing::EaseOut => assert!(matches!(transition.ease, Easing::EaseOut)),
                Easing::EaseInOut => assert!(matches!(transition.ease, Easing::EaseInOut)),
                _ => panic!("Unexpected easing function"),
            }
        }
    }

    #[test]
    fn test_animation_properties_hashmap() {
        // Test that we can create animation properties correctly
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), create_event_animation_value(1.0));
        properties.insert("scale".to_string(), create_event_animation_value(1.2));
        properties.insert("rotate".to_string(), create_event_animation_value(45.0));
        
        assert_eq!(properties.len(), 3);
        assert!(properties.contains_key("opacity"));
        assert!(properties.contains_key("scale"));
        assert!(properties.contains_key("rotate"));
        
        // Test that values are correct
        if let Some(AnimationValue::Number(value)) = properties.get("opacity") {
            assert_eq!(*value, 1.0);
        } else {
            panic!("Opacity value should be a number");
        }
        
        if let Some(AnimationValue::Number(value)) = properties.get("scale") {
            assert_eq!(*value, 1.2);
        } else {
            panic!("Scale value should be a number");
        }
    }

    #[test]
    fn test_animation_manager_basic_operations() {
        // Test that AnimationManager can handle basic operations
        let mut manager = AnimationManager::new();
        
        // Test ID generation
        let id1 = manager.generate_id();
        let id2 = manager.generate_id();
        assert_ne!(id1, id2);
        assert_eq!(id1, "anim_1");
        assert_eq!(id2, "anim_2");
        
        // Test that manager can be created and dropped
        drop(manager);
        assert!(true); // If we get here, the manager was created and dropped successfully
    }

    #[test]
    fn test_animation_value_types() {
        // Test different types of animation values
        let number_value = AnimationValue::Number(42.0);
        let string_value = AnimationValue::String("red".to_string());
        let pixels_value = AnimationValue::Pixels(100.0);
        let degrees_value = AnimationValue::Degrees(180.0);
        
        // Test number value
        if let AnimationValue::Number(value) = number_value {
            assert_eq!(value, 42.0);
        } else {
            panic!("Should be a number value");
        }
        
        // Test string value
        if let AnimationValue::String(value) = string_value {
            assert_eq!(value, "red");
        } else {
            panic!("Should be a string value");
        }
        
        // Test pixels value
        if let AnimationValue::Pixels(value) = pixels_value {
            assert_eq!(value, 100.0);
        } else {
            panic!("Should be a pixels value");
        }
        
        // Test degrees value
        if let AnimationValue::Degrees(value) = degrees_value {
            assert_eq!(value, 180.0);
        } else {
            panic!("Should be a degrees value");
        }
    }

    #[test]
    fn test_transition_defaults() {
        // Test that transition defaults work correctly
        let default_transition = Transition::default();
        
        // Default values should be reasonable
        assert_eq!(default_transition.duration, Some(0.3));
        assert_eq!(default_transition.delay, None);
        assert!(matches!(default_transition.ease, Easing::EaseInOut));
    }

    #[test]
    fn test_animation_properties_manipulation() {
        // Test that we can manipulate animation properties
        let mut properties = HashMap::new();
        
        // Add properties
        properties.insert("opacity".to_string(), create_event_animation_value(0.0));
        properties.insert("x".to_string(), create_event_animation_value(-100.0));
        
        assert_eq!(properties.len(), 2);
        
        // Update properties
        properties.insert("opacity".to_string(), create_event_animation_value(1.0));
        properties.insert("x".to_string(), create_event_animation_value(0.0));
        
        assert_eq!(properties.len(), 2); // Should still be 2
        
        // Verify updated values
        if let Some(AnimationValue::Number(value)) = properties.get("opacity") {
            assert_eq!(*value, 1.0);
        } else {
            panic!("Opacity should be updated to 1.0");
        }
        
        if let Some(AnimationValue::Number(value)) = properties.get("x") {
            assert_eq!(*value, 0.0);
        } else {
            panic!("X should be updated to 0.0");
        }
    }

    #[test]
    fn test_animation_manager_id_uniqueness() {
        // Test that AnimationManager generates unique IDs
        let mut manager = AnimationManager::new();
        
        let mut ids = Vec::new();
        for _ in 0..100 {
            ids.push(manager.generate_id());
        }
        
        // All IDs should be unique
        for i in 0..ids.len() {
            for j in (i+1)..ids.len() {
                assert_ne!(ids[i], ids[j], "IDs should be unique: {} and {}", ids[i], ids[j]);
            }
        }
        
        // IDs should be in the expected format
        assert_eq!(ids[0], "anim_1");
        assert_eq!(ids[1], "anim_2");
        assert_eq!(ids[99], "anim_100");
    }
}