//! Simple unit tests for core functionality that can actually be tested

use crate::types::*;
use crate::{AnimationError, ErrorContext, ErrorHandler, DefaultErrorHandler, RecoveryStrategy};
use std::collections::HashMap;

#[cfg(test)]
mod basic_types_tests {
    use super::*;

    #[test]
    fn test_animation_handle_creation() {
        let handle = AnimationHandle(123);
        assert_eq!(handle.0, 123);
    }

    #[test]
    fn test_animation_handle_equality() {
        let handle1 = AnimationHandle(123);
        let handle2 = AnimationHandle(123);
        let handle3 = AnimationHandle(456);
        
        assert_eq!(handle1, handle2);
        assert_ne!(handle1, handle3);
    }

    #[test]
    fn test_animation_handle_hash() {
        let handle1 = AnimationHandle(123);
        let handle2 = AnimationHandle(123);
        let handle3 = AnimationHandle(456);
        
        let mut map = HashMap::new();
        map.insert(handle1, "test1");
        map.insert(handle3, "test3");
        
        assert_eq!(map.get(&handle2), Some(&"test1"));
        assert_eq!(map.get(&handle3), Some(&"test3"));
    }

    #[test]
    fn test_animation_value_number() {
        let value = AnimationValue::Number(42.5);
        assert_eq!(value.to_string_value(), "42.5");
    }

    #[test]
    fn test_animation_value_pixels() {
        let value = AnimationValue::Pixels(100.0);
        assert_eq!(value.to_string_value(), "100px");
    }

    #[test]
    fn test_animation_value_percentage() {
        let value = AnimationValue::Percentage(50.0);
        assert_eq!(value.to_string_value(), "50%");
    }

    #[test]
    fn test_animation_value_degrees() {
        let value = AnimationValue::Degrees(90.0);
        assert_eq!(value.to_string_value(), "90deg");
    }

    #[test]
    fn test_animation_value_radians() {
        let value = AnimationValue::Radians(3.14159);
        assert_eq!(value.to_string_value(), "3.14159rad");
    }

    #[test]
    fn test_animation_value_color() {
        let value = AnimationValue::Color("#ff0000".to_string());
        assert_eq!(value.to_string_value(), "#ff0000");
    }

    #[test]
    fn test_animation_value_string() {
        let value = AnimationValue::String("auto".to_string());
        assert_eq!(value.to_string_value(), "auto");
    }

    #[test]
    fn test_animation_value_equality() {
        let value1 = AnimationValue::Number(42.0);
        let value2 = AnimationValue::Number(42.0);
        let value3 = AnimationValue::Number(43.0);
        
        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
    }

    #[test]
    fn test_animation_value_clone() {
        let value1 = AnimationValue::Number(42.0);
        let value2 = value1.clone();
        assert_eq!(value1, value2);
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_transform_default() {
        let transform = Transform::default();
        assert_eq!(transform.x, None);
        assert_eq!(transform.y, None);
        assert_eq!(transform.z, None);
        assert_eq!(transform.scale_x, None);
        assert_eq!(transform.scale_y, None);
        assert_eq!(transform.scale, None);
        assert_eq!(transform.rotate_x, None);
        assert_eq!(transform.rotate_y, None);
        assert_eq!(transform.rotate_z, None);
        assert_eq!(transform.skew_x, None);
        assert_eq!(transform.skew_y, None);
    }

    #[test]
    fn test_transform_new() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            z: Some(30.0),
            scale_x: Some(1.5),
            scale_y: Some(2.0),
            scale: Some(1.0),
            rotate_x: Some(45.0),
            rotate_y: Some(90.0),
            rotate_z: Some(180.0),
            skew_x: Some(10.0),
            skew_y: Some(20.0),
        };
        
        assert_eq!(transform.x, Some(10.0));
        assert_eq!(transform.y, Some(20.0));
        assert_eq!(transform.z, Some(30.0));
        assert_eq!(transform.scale_x, Some(1.5));
        assert_eq!(transform.scale_y, Some(2.0));
        assert_eq!(transform.scale, Some(1.0));
        assert_eq!(transform.rotate_x, Some(45.0));
        assert_eq!(transform.rotate_y, Some(90.0));
        assert_eq!(transform.rotate_z, Some(180.0));
        assert_eq!(transform.skew_x, Some(10.0));
        assert_eq!(transform.skew_y, Some(20.0));
    }

    #[test]
    fn test_transform_equality() {
        let transform1 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform2 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform3 = Transform {
            x: Some(11.0),
            y: Some(20.0),
            ..Default::default()
        };
        
        assert_eq!(transform1, transform2);
        assert_ne!(transform1, transform3);
    }

    #[test]
    fn test_transform_clone() {
        let transform1 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform2 = transform1.clone();
        assert_eq!(transform1, transform2);
    }
}

#[cfg(test)]
mod transition_tests {
    use super::*;

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert_eq!(transition.duration, Some(0.3));
        assert_eq!(transition.ease, Easing::EaseInOut);
        assert_eq!(transition.delay, None);
        assert_eq!(transition.repeat, RepeatConfig::Never);
        assert_eq!(transition.stagger, None);
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Count(3),
            stagger: None,
        };
        
        assert_eq!(transition.duration, Some(0.5));
        assert_eq!(transition.ease, Easing::Linear);
        assert_eq!(transition.delay, Some(0.1));
        assert_eq!(transition.repeat, RepeatConfig::Count(3));
        assert_eq!(transition.stagger, None);
    }

    #[test]
    fn test_transition_equality() {
        let transition1 = Transition {
            duration: Some(0.5),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Count(3),
            stagger: None,
        };
        let transition2 = Transition {
            duration: Some(0.5),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Count(3),
            stagger: None,
        };
        let transition3 = Transition {
            duration: Some(0.6),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Count(3),
            stagger: None,
        };
        
        assert_eq!(transition1, transition2);
        assert_ne!(transition1, transition3);
    }

    #[test]
    fn test_transition_clone() {
        let transition1 = Transition {
            duration: Some(0.5),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Count(3),
            stagger: None,
        };
        let transition2 = transition1.clone();
        assert_eq!(transition1, transition2);
    }
}

#[cfg(test)]
mod stagger_config_tests {
    use super::*;

    #[test]
    fn test_stagger_config_new() {
        let config = StaggerConfig {
            delay: 0.1,
            from: StaggerFrom::First,
        };
        assert_eq!(config.delay, 0.1);
        assert_eq!(config.from, StaggerFrom::First);
    }

    #[test]
    fn test_stagger_config_with_last() {
        let config = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::Last,
        };
        
        assert_eq!(config.delay, 0.2);
        assert_eq!(config.from, StaggerFrom::Last);
    }

    #[test]
    fn test_stagger_config_equality() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::Last,
        };
        let config2 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::Last,
        };
        let config3 = StaggerConfig {
            delay: 0.3,
            from: StaggerFrom::Last,
        };
        
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_stagger_config_clone() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::Last,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }
}

#[cfg(test)]
mod stagger_from_tests {
    use super::*;

    #[test]
    fn test_stagger_from_first() {
        let from = StaggerFrom::First;
        assert_eq!(format!("{:?}", from), "First");
    }

    #[test]
    fn test_stagger_from_last() {
        let from = StaggerFrom::Last;
        assert_eq!(format!("{:?}", from), "Last");
    }

    #[test]
    fn test_stagger_from_center() {
        let from = StaggerFrom::Center;
        assert_eq!(format!("{:?}", from), "Center");
    }

    #[test]
    fn test_stagger_from_index() {
        let from = StaggerFrom::Index(5);
        assert_eq!(format!("{:?}", from), "Index(5)");
    }

    #[test]
    fn test_stagger_from_equality() {
        let from1 = StaggerFrom::First;
        let from2 = StaggerFrom::First;
        let from3 = StaggerFrom::Last;
        
        assert_eq!(from1, from2);
        assert_ne!(from1, from3);
    }

    #[test]
    fn test_stagger_from_clone() {
        let from1 = StaggerFrom::First;
        let from2 = from1.clone();
        assert_eq!(from1, from2);
    }
}

#[cfg(test)]
mod animation_target_tests {
    use super::*;

    #[test]
    fn test_animation_target_new() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        
        assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(0.5)));
        assert_eq!(target.get("x"), Some(&AnimationValue::Pixels(100.0)));
        assert_eq!(target.len(), 2);
    }

    #[test]
    fn test_animation_target_is_empty() {
        let target = AnimationTarget::new();
        assert!(target.is_empty());
        
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        assert!(!target.is_empty());
    }

    #[test]
    fn test_animation_target_contains_key() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        assert!(target.contains_key("opacity"));
        assert!(!target.contains_key("x"));
    }

    #[test]
    fn test_animation_target_remove() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        assert_eq!(target.remove("opacity"), Some(AnimationValue::Number(0.5)));
        assert_eq!(target.remove("opacity"), None);
        assert!(target.is_empty());
    }

    #[test]
    fn test_animation_target_iter() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        
        let mut keys: Vec<_> = target.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["opacity", "x"]);
        
        let values: Vec<_> = target.values().collect();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_animation_target_clone() {
        let mut target1 = AnimationTarget::new();
        target1.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let target2 = target1.clone();
        assert_eq!(target1, target2);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_animation_error_engine_unavailable() {
        let error = AnimationError::EngineUnavailable("Test engine".to_string());
        assert_eq!(format!("{}", error), "Animation engine not available: Test engine");
    }

    #[test]
    fn test_animation_error_invalid_property() {
        let error = AnimationError::InvalidProperty {
            property: "invalid_prop".to_string(),
        };
        assert_eq!(format!("{}", error), "Invalid animation property: invalid_prop");
    }

    #[test]
    fn test_animation_error_already_running() {
        let handle = AnimationHandle(123);
        let error = AnimationError::AlreadyRunning { handle };
        assert_eq!(format!("{}", error), "Animation already running with handle: AnimationHandle(123)");
    }

    #[test]
    fn test_animation_error_not_found() {
        let handle = AnimationHandle(456);
        let error = AnimationError::NotFound { handle };
        assert_eq!(format!("{}", error), "Animation not found: AnimationHandle(456)");
    }

    #[test]
    fn test_error_context_new() {
        let context = ErrorContext::new("test_operation");
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, None);
        assert!(context.additional_info.is_empty());
    }

    #[test]
    fn test_error_context_with_component() {
        let context = ErrorContext::new("test_operation")
            .with_component("TestComponent");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, Some("TestComponent".to_string()));
    }

    #[test]
    fn test_error_context_with_info() {
        let context = ErrorContext::new("test_operation")
            .with_info("key1", "value1")
            .with_info("key2", "value2");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.additional_info.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.additional_info.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_default_error_handler_default() {
        let handler = DefaultErrorHandler::default();
        assert!(handler.log_errors);
        assert!(!handler.show_user_messages);
    }

    #[test]
    fn test_default_error_handler_handle_error_engine_unavailable() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::EngineUnavailable("Test".to_string());
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Fallback);
    }

    #[test]
    fn test_default_error_handler_handle_error_invalid_property() {
        let handler = DefaultErrorHandler::default();
        let error = AnimationError::InvalidProperty {
            property: "invalid".to_string(),
        };
        let context = ErrorContext::new("test_operation");
        
        let strategy = handler.handle_error(&error, &context);
        assert_eq!(strategy, RecoveryStrategy::Skip);
    }

    #[test]
    fn test_recovery_strategy_equality() {
        let strategy1 = RecoveryStrategy::Retry;
        let strategy2 = RecoveryStrategy::Retry;
        let strategy3 = RecoveryStrategy::Fallback;
        
        assert_eq!(strategy1, strategy2);
        assert_ne!(strategy1, strategy3);
    }
}
