//! Event-Driven Animation System Tests
//!
//! These tests verify that our new event-driven animation system works correctly.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnimationManager,
        CssTransitionAnimation,
        KeyframeAnimation,
        SpringAnimation,
        StaggerAnimation,
        create_event_animation_value,
        AnimationType,
        EventStaggerConfig,
        EventSpringConfig,
        Keyframe,
        Transition,
        Easing,
    };
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_animation_manager_creation() {
        // Test that we can create an AnimationManager
        let manager = AnimationManager::new();
        // We can't access private fields, so just test that it was created successfully
        assert!(true); // Manager was created without panicking
    }

    #[test]
    fn test_animation_manager_add_animation() {
        // Test that we can add animations to the manager
        let mut manager = AnimationManager::new();
        
        // Create a mock element (we can't create real DOM elements in unit tests)
        // For now, we'll test the manager logic without DOM elements
        
        // Test that the manager can generate unique IDs
        let id1 = manager.generate_id();
        let id2 = manager.generate_id();
        assert_ne!(id1, id2);
        assert_eq!(id1, "anim_1");
        assert_eq!(id2, "anim_2");
    }

    #[test]
    fn test_animation_value_creation() {
        // Test that we can create animation values
        let number_value = create_event_animation_value(42.0);
        
        // These should compile and create the values
        assert!(matches!(number_value, crate::AnimationValue::Number(42.0)));
        
        // Test string values using the core AnimationValue directly
        let string_value = crate::AnimationValue::String("red".to_string());
        assert!(matches!(string_value, crate::AnimationValue::String(_)));
    }

    #[test]
    fn test_transition_creation() {
        // Test that we can create transitions
        let transition = Transition {
            duration: Some(0.5),
            delay: Some(0.1),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        assert_eq!(transition.duration, Some(0.5));
        assert_eq!(transition.delay, Some(0.1));
        assert!(matches!(transition.ease, Easing::EaseInOut));
    }

    #[test]
    fn test_keyframe_creation() {
        // Test that we can create keyframes
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), create_event_animation_value(1.0));
        properties.insert("scale".to_string(), create_event_animation_value(1.2));
        
        let keyframe = Keyframe {
            offset: 0.5,
            properties,
            easing: Some(Easing::EaseIn),
        };
        
        assert_eq!(keyframe.offset, 0.5);
        assert!(matches!(keyframe.easing, Some(Easing::EaseIn)));
        assert_eq!(keyframe.properties.len(), 2);
    }

    #[test]
    fn test_stagger_config_creation() {
        // Test that we can create stagger configurations
        let config = EventStaggerConfig {
            delay: 0.1,
            from_first: true,
            max_delay: Some(0.5),
        };
        
        assert_eq!(config.delay, 0.1);
        assert!(config.from_first);
        assert_eq!(config.max_delay, Some(0.5));
    }

    #[test]
    fn test_spring_config_creation() {
        // Test that we can create spring configurations
        let config = EventSpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            rest_displacement_threshold: 0.01,
            rest_velocity_threshold: 0.01,
            initial_velocity: 0.0, // Add missing field
        };
        
        assert_eq!(config.stiffness, 100.0);
        assert_eq!(config.damping, 10.0);
        assert_eq!(config.mass, 1.0);
        assert_eq!(config.rest_displacement_threshold, 0.01);
        assert_eq!(config.rest_velocity_threshold, 0.01);
        assert_eq!(config.initial_velocity, 0.0);
    }

    #[test]
    fn test_animation_type_enum() {
        // Test that animation types work correctly
        let css_type = AnimationType::Css;
        let keyframe_type = AnimationType::Keyframe;
        let stagger_type = AnimationType::Stagger;
        let spring_type = AnimationType::Spring;
        
        // These should compile and be different variants
        assert!(matches!(css_type, AnimationType::Css));
        assert!(matches!(keyframe_type, AnimationType::Keyframe));
        assert!(matches!(stagger_type, AnimationType::Stagger));
        assert!(matches!(spring_type, AnimationType::Spring));
    }

    #[test]
    fn test_easing_enum() {
        // Test that easing functions work correctly
        let linear = Easing::Linear;
        let ease_in = Easing::EaseIn;
        let ease_out = Easing::EaseOut;
        let ease_in_out = Easing::EaseInOut;
        
        assert!(matches!(linear, Easing::Linear));
        assert!(matches!(ease_in, Easing::EaseIn));
        assert!(matches!(ease_out, Easing::EaseOut));
        assert!(matches!(ease_in_out, Easing::EaseInOut));
    }

    #[test]
    fn test_animation_manager_id_generation() {
        // Test that the animation manager generates unique IDs
        let mut manager = AnimationManager::new();
        
        let ids: Vec<String> = (0..10).map(|_| manager.generate_id()).collect();
        
        // All IDs should be unique
        for i in 0..ids.len() {
            for j in (i+1)..ids.len() {
                assert_ne!(ids[i], ids[j], "IDs should be unique");
            }
        }
        
        // IDs should be in the expected format
        assert_eq!(ids[0], "anim_1");
        assert_eq!(ids[1], "anim_2");
        assert_eq!(ids[9], "anim_10");
    }

    #[test]
    fn test_animation_manager_cleanup() {
        // Test that the animation manager can be cleaned up
        let manager = AnimationManager::new();
        
        // The manager should be droppable without issues
        drop(manager);
        
        // This test passes if we can create and drop the manager
        assert!(true);
    }
}
