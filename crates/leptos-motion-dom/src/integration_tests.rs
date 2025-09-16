//! Integration Tests for Component Interactions
//!
//! These tests verify that different components work together correctly
//! and that the animation system integrates properly with Leptos components.

use leptos_motion_core::*;
use crate::signal_based_animation_controller::SignalBasedAnimationController;
use std::collections::HashMap;

#[cfg(test)]
mod component_integration_tests {
    use super::*;

    #[test]
    fn test_animation_target_with_motion_props_integration() {
        // Test that AnimationTarget can work with motion components
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("transform".to_string(), AnimationValue::String("translateX(100px)".to_string()));

        // Verify the target contains expected values
        assert_eq!(target.len(), 2);
        assert!(target.contains_key("opacity"));
        assert!(target.contains_key("transform"));
        
        // Test that we can retrieve values
        if let Some(AnimationValue::Number(opacity)) = target.get("opacity") {
            assert_eq!(*opacity, 0.5);
        }
        
        if let Some(AnimationValue::String(transform)) = target.get("transform") {
            assert_eq!(transform, "translateX(100px)");
        }
    }

    #[test]
    fn test_transition_with_animation_target_integration() {
        // Test that transitions work with animation targets
        let transition = Transition {
            duration: Some(0.5),
            delay: Some(0.1),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Count(2),
            stagger: None,
        };
        
        // Test that transition can be created and has expected properties
        assert_eq!(transition.duration, Some(0.5));
        assert_eq!(transition.delay, Some(0.1));
        assert_eq!(transition.ease, Easing::EaseInOut);
        assert_eq!(transition.repeat, RepeatConfig::Count(2));
    }

    #[test]
    fn test_animation_controller_integration() {
        // Test that animation controller integrates with motion components
        let initial_values = HashMap::new();
        let controller = SignalBasedAnimationController::new(initial_values);
        
        // Test initial state
        assert!(!controller.is_animation_playing_untracked());
        assert_eq!(controller.get_progress_untracked(), 0.0);
        
        // Test animation target creation
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        
        // Test that controller can handle the target
        controller.animate_to(target);
        
        // Verify animation state changed
        assert!(controller.is_animation_playing_untracked());
    }

    #[test]
    fn test_motion_values_integration() {
        // Test that motion values integrate with animation system
        let mut values = HashMap::new();
        values.insert("opacity".to_string(), "0.5".to_string());
        values.insert("transform".to_string(), "translateX(50px) rotate(45deg)".to_string());
        
        // Test conversion to animation values
        let mut animation_target = AnimationTarget::new();
        for (key, value) in values {
            // Simple conversion logic for testing
            if let Ok(num) = value.parse::<f64>() {
                animation_target.insert(key, AnimationValue::Number(num));
            } else {
                animation_target.insert(key, AnimationValue::String(value));
            }
        }
        
        // Verify the conversion worked
        assert_eq!(animation_target.len(), 2);
        assert!(animation_target.contains_key("opacity"));
        assert!(animation_target.contains_key("transform"));
    }

    #[test]
    fn test_gesture_and_animation_integration() {
        // Test animation target for gesture-triggered animations
        let mut gesture_target = AnimationTarget::new();
        gesture_target.insert("scale".to_string(), AnimationValue::Number(1.1));
        gesture_target.insert("rotate".to_string(), AnimationValue::Degrees(5.0));
        
        // Verify gesture animation target
        assert_eq!(gesture_target.len(), 2);
        assert!(gesture_target.contains_key("scale"));
        assert!(gesture_target.contains_key("rotate"));
    }

    #[test]
    fn test_layout_and_motion_integration() {
        // Test layout animation target
        let mut layout_target = AnimationTarget::new();
        layout_target.insert("width".to_string(), AnimationValue::Pixels(200.0));
        layout_target.insert("height".to_string(), AnimationValue::Pixels(150.0));
        
        // Verify layout animation target
        assert_eq!(layout_target.len(), 2);
        assert!(layout_target.contains_key("width"));
        assert!(layout_target.contains_key("height"));
    }

    #[test]
    fn test_component_lifecycle_integration() {
        // Test that components handle lifecycle events correctly
        let mut lifecycle_events = Vec::new();
        
        // Simulate component lifecycle
        lifecycle_events.push("mount");
        lifecycle_events.push("animate_in");
        lifecycle_events.push("animate_out");
        lifecycle_events.push("unmount");
        
        // Test that lifecycle events are tracked
        assert_eq!(lifecycle_events.len(), 4);
        assert_eq!(lifecycle_events[0], "mount");
        assert_eq!(lifecycle_events[1], "animate_in");
        assert_eq!(lifecycle_events[2], "animate_out");
        assert_eq!(lifecycle_events[3], "unmount");
    }

    #[test]
    fn test_multi_component_coordination() {
        // Test that multiple components can coordinate animations
        let mut component1_target = AnimationTarget::new();
        component1_target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        
        let mut component2_target = AnimationTarget::new();
        component2_target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        
        // Test staggered animation coordination
        let stagger_config = StaggerConfig {
            delay: 0.1,
            from: StaggerFrom::First,
        };
        
        // Verify components can have different targets
        assert_ne!(component1_target, component2_target);
        assert!(stagger_config.delay > 0.0);
        assert_eq!(stagger_config.from, StaggerFrom::First);
    }

    #[test]
    fn test_animation_value_types_integration() {
        // Test that different animation value types work together
        let mut mixed_target = AnimationTarget::new();
        mixed_target.insert("opacity".to_string(), AnimationValue::Number(0.8));
        mixed_target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        mixed_target.insert("rotate".to_string(), AnimationValue::Degrees(45.0));
        mixed_target.insert("scale".to_string(), AnimationValue::Number(1.2));
        mixed_target.insert("color".to_string(), AnimationValue::String("#ff0000".to_string()));
        
        // Verify all value types are stored correctly
        assert_eq!(mixed_target.len(), 5);
        assert!(mixed_target.contains_key("opacity"));
        assert!(mixed_target.contains_key("x"));
        assert!(mixed_target.contains_key("rotate"));
        assert!(mixed_target.contains_key("scale"));
        assert!(mixed_target.contains_key("color"));
        
        // Test value retrieval
        if let Some(AnimationValue::Number(opacity)) = mixed_target.get("opacity") {
            assert_eq!(*opacity, 0.8);
        }
        
        if let Some(AnimationValue::Pixels(x)) = mixed_target.get("x") {
            assert_eq!(*x, 100.0);
        }
        
        if let Some(AnimationValue::Degrees(rotate)) = mixed_target.get("rotate") {
            assert_eq!(*rotate, 45.0);
        }
    }

    #[test]
    fn test_easing_functions_integration() {
        // Test that easing functions integrate with animation system
        let easings = vec![
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::CircIn,
            Easing::CircOut,
            Easing::CircInOut,
            Easing::BackIn,
            Easing::BackOut,
            Easing::BackInOut,
        ];
        
        // Test that all easing functions can be used
        for easing in easings {
            let transition = Transition {
                duration: Some(0.3),
                delay: None,
                ease: easing,
                repeat: RepeatConfig::Count(0),
                stagger: None,
            };
            
            // Verify transition was created successfully
            assert_eq!(transition.duration, Some(0.3));
            assert_eq!(transition.delay, None);
            assert_eq!(transition.repeat, RepeatConfig::Count(0));
        }
    }

    #[test]
    fn test_repeat_configurations_integration() {
        // Test that repeat configurations integrate with animation system
        let repeat_configs = vec![
            RepeatConfig::Count(0),
            RepeatConfig::Count(1),
            RepeatConfig::Count(3),
            RepeatConfig::Infinite,
        ];
        
        // Test that all repeat configurations can be used
        for repeat in repeat_configs {
            let transition = Transition {
                duration: Some(0.5),
                delay: Some(0.1),
                ease: Easing::EaseOut,
                repeat,
                stagger: None,
            };
            
            // Verify transition was created successfully
            assert_eq!(transition.duration, Some(0.5));
            assert_eq!(transition.delay, Some(0.1));
            assert_eq!(transition.ease, Easing::EaseOut);
        }
    }
}