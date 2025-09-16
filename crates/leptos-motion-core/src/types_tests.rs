//! Comprehensive unit tests for core types

use crate::types::*;
use std::collections::HashMap;

#[cfg(test)]
mod animation_handle_tests {
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
    fn test_animation_handle_debug() {
        let handle = AnimationHandle(123);
        let debug_str = format!("{:?}", handle);
        assert!(debug_str.contains("123"));
    }

    #[test]
    fn test_animation_handle_clone() {
        let handle1 = AnimationHandle(123);
        let handle2 = handle1.clone();
        assert_eq!(handle1, handle2);
    }
}

#[cfg(test)]
mod animation_value_tests {
    use super::*;

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
    fn test_animation_value_transform() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            z: Some(30.0),
            scale_x: Some(1.5),
            scale_y: Some(2.0),
            scale_z: Some(1.0),
            rotate_x: Some(45.0),
            rotate_y: Some(90.0),
            rotate_z: Some(180.0),
            skew_x: Some(10.0),
            skew_y: Some(20.0),
        };
        let value = AnimationValue::Transform(transform);
        let result = value.to_string_value();
        
        assert!(result.contains("translateX(10px)"));
        assert!(result.contains("translateY(20px)"));
        assert!(result.contains("translateZ(30px)"));
        assert!(result.contains("scaleX(1.5)"));
        assert!(result.contains("scaleY(2)"));
        assert!(result.contains("scaleZ(1)"));
        assert!(result.contains("rotateX(45deg)"));
        assert!(result.contains("rotateY(90deg)"));
        assert!(result.contains("rotateZ(180deg)"));
        assert!(result.contains("skewX(10deg)"));
        assert!(result.contains("skewY(20deg)"));
    }

    #[test]
    fn test_animation_value_complex() {
        let complex = ComplexValue {
            values: vec![
                AnimationValue::Number(1.0),
                AnimationValue::Number(2.0),
                AnimationValue::Number(3.0),
            ],
            interpolation: None,
        };
        let value = AnimationValue::Complex(complex);
        let result = value.to_string_value();
        assert_eq!(result, "1,2,3");
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

    #[test]
    fn test_animation_value_debug() {
        let value = AnimationValue::Number(42.0);
        let debug_str = format!("{:?}", value);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
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
        assert_eq!(transform.scale_z, None);
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
            scale_z: Some(1.0),
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
        assert_eq!(transform.scale_z, Some(1.0));
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

    #[test]
    fn test_transform_debug() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let debug_str = format!("{:?}", transform);
        assert!(debug_str.contains("Transform"));
    }
}

#[cfg(test)]
mod complex_value_tests {
    use super::*;

    #[test]
    fn test_complex_value_new() {
        let values = vec![
            AnimationValue::Number(1.0),
            AnimationValue::Number(2.0),
            AnimationValue::Number(3.0),
        ];
        let complex = ComplexValue {
            values: values.clone(),
            interpolation: None,
        };
        
        assert_eq!(complex.values, values);
        assert_eq!(complex.interpolation, None);
    }

    #[test]
    fn test_complex_value_with_interpolation() {
        let values = vec![AnimationValue::Number(1.0)];
        let complex = ComplexValue {
            values,
            interpolation: Some("linear".to_string()),
        };
        
        assert_eq!(complex.interpolation, Some("linear".to_string()));
    }

    #[test]
    fn test_complex_value_equality() {
        let complex1 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex2 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex3 = ComplexValue {
            values: vec![AnimationValue::Number(2.0)],
            interpolation: None,
        };
        
        assert_eq!(complex1, complex2);
        assert_ne!(complex1, complex3);
    }

    #[test]
    fn test_complex_value_clone() {
        let complex1 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex2 = complex1.clone();
        assert_eq!(complex1, complex2);
    }

    #[test]
    fn test_complex_value_debug() {
        let complex = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let debug_str = format!("{:?}", complex);
        assert!(debug_str.contains("ComplexValue"));
    }
}

#[cfg(test)]
mod easing_tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = Easing::Linear;
        assert_eq!(format!("{:?}", easing), "Linear");
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = Easing::EaseIn;
        assert_eq!(format!("{:?}", easing), "EaseIn");
    }

    #[test]
    fn test_easing_ease_out() {
        let easing = Easing::EaseOut;
        assert_eq!(format!("{:?}", easing), "EaseOut");
    }

    #[test]
    fn test_easing_ease_in_out() {
        let easing = Easing::EaseInOut;
        assert_eq!(format!("{:?}", easing), "EaseInOut");
    }

    #[test]
    fn test_easing_spring() {
        let spring_config = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let easing = Easing::Spring(spring_config);
        match easing {
            Easing::Spring(config) => {
                assert_eq!(config.stiffness, 100.0);
                assert_eq!(config.damping, 10.0);
                assert_eq!(config.mass, 1.0);
            }
            _ => panic!("Expected Spring variant"),
        }
    }

    #[test]
    fn test_easing_cubic_bezier() {
        let easing = Easing::CubicBezier(0.25, 0.1, 0.25, 1.0);
        match easing {
            Easing::CubicBezier(x1, y1, x2, y2) => {
                assert_eq!(x1, 0.25);
                assert_eq!(y1, 0.1);
                assert_eq!(x2, 0.25);
                assert_eq!(y2, 1.0);
            }
            _ => panic!("Expected CubicBezier variant"),
        }
    }

    #[test]
    fn test_easing_equality() {
        let easing1 = Easing::Linear;
        let easing2 = Easing::Linear;
        let easing3 = Easing::EaseIn;
        
        assert_eq!(easing1, easing2);
        assert_ne!(easing1, easing3);
    }

    #[test]
    fn test_easing_clone() {
        let easing1 = Easing::Linear;
        let easing2 = easing1.clone();
        assert_eq!(easing1, easing2);
    }
}

#[cfg(test)]
mod spring_config_tests {
    use super::*;

    #[test]
    fn test_spring_config_default() {
        let config = SpringConfig::default();
        assert_eq!(config.stiffness, 100.0);
        assert_eq!(config.damping, 10.0);
        assert_eq!(config.mass, 1.0);
    }

    #[test]
    fn test_spring_config_new() {
        let config = SpringConfig {
            stiffness: 200.0,
            damping: 20.0,
            mass: 2.0,
        };
        
        assert_eq!(config.stiffness, 200.0);
        assert_eq!(config.damping, 20.0);
        assert_eq!(config.mass, 2.0);
    }

    #[test]
    fn test_spring_config_equality() {
        let config1 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config2 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config3 = SpringConfig {
            stiffness: 200.0,
            damping: 10.0,
            mass: 1.0,
        };
        
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_spring_config_clone() {
        let config1 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_spring_config_debug() {
        let config = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("SpringConfig"));
    }
}

#[cfg(test)]
mod transition_tests {
    use super::*;

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert_eq!(transition.duration, 0.3);
        assert_eq!(transition.easing, Easing::EaseOut);
        assert_eq!(transition.delay, 0.0);
        assert_eq!(transition.repeat, None);
        assert_eq!(transition.yoyo, false);
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        assert_eq!(transition.duration, 0.5);
        assert_eq!(transition.easing, Easing::Linear);
        assert_eq!(transition.delay, 0.1);
        assert_eq!(transition.repeat, Some(3));
        assert_eq!(transition.yoyo, true);
    }

    #[test]
    fn test_transition_equality() {
        let transition1 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition2 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition3 = Transition {
            duration: 0.6,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        assert_eq!(transition1, transition2);
        assert_ne!(transition1, transition3);
    }

    #[test]
    fn test_transition_clone() {
        let transition1 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition2 = transition1.clone();
        assert_eq!(transition1, transition2);
    }

    #[test]
    fn test_transition_debug() {
        let transition = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let debug_str = format!("{:?}", transition);
        assert!(debug_str.contains("Transition"));
    }
}

#[cfg(test)]
mod repeat_config_tests {
    use super::*;

    #[test]
    fn test_repeat_config_default() {
        let config = RepeatConfig::default();
        assert_eq!(config.count, 1);
        assert_eq!(config.direction, RepeatDirection::Normal);
    }

    #[test]
    fn test_repeat_config_new() {
        let config = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        
        assert_eq!(config.count, 5);
        assert_eq!(config.direction, RepeatDirection::Alternate);
    }

    #[test]
    fn test_repeat_config_equality() {
        let config1 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config2 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config3 = RepeatConfig {
            count: 6,
            direction: RepeatDirection::Alternate,
        };
        
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_repeat_config_clone() {
        let config1 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_repeat_config_debug() {
        let config = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RepeatConfig"));
    }
}

#[cfg(test)]
mod stagger_config_tests {
    use super::*;

    #[test]
    fn test_stagger_config_default() {
        let config = StaggerConfig::default();
        assert_eq!(config.delay, 0.1);
        assert_eq!(config.from, StaggerFrom::Start);
        assert_eq!(config.ease, Easing::Linear);
    }

    #[test]
    fn test_stagger_config_new() {
        let config = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        
        assert_eq!(config.delay, 0.2);
        assert_eq!(config.from, StaggerFrom::End);
        assert_eq!(config.ease, Easing::EaseIn);
    }

    #[test]
    fn test_stagger_config_equality() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config2 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config3 = StaggerConfig {
            delay: 0.3,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_stagger_config_clone() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_stagger_config_debug() {
        let config = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("StaggerConfig"));
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
        
        let mut values: Vec<_> = target.values().collect();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_animation_target_clone() {
        let mut target1 = AnimationTarget::new();
        target1.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let target2 = target1.clone();
        assert_eq!(target1, target2);
    }

    #[test]
    fn test_animation_target_debug() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let debug_str = format!("{:?}", target);
        assert!(debug_str.contains("AnimationTarget"));
    }
}
