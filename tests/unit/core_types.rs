use leptos_motion_core::{
    AnimationHandle, AnimationValue, AnimationTarget, Transition,
    Easing, SpringConfig, RepeatConfig
};
use std::collections::HashMap;

#[test]
fn test_animation_handle_creation() {
    let handle = AnimationHandle(42);
    assert_eq!(handle.0, 42);
}

#[test]
fn test_animation_handle_equality() {
    let handle1 = AnimationHandle(42);
    let handle2 = AnimationHandle(42);
    let handle3 = AnimationHandle(43);
    
    assert_eq!(handle1, handle2);
    assert_ne!(handle1, handle3);
}

#[test]
fn test_animation_handle_clone() {
    let handle = AnimationHandle(42);
    let cloned = handle.clone();
    
    assert_eq!(handle, cloned);
}

#[test]
fn test_animation_handle_copy() {
    let handle = AnimationHandle(42);
    let copied = handle;
    
    assert_eq!(handle, copied);
}

#[test]
fn test_animation_handle_debug() {
    let handle = AnimationHandle(42);
    let debug_str = format!("{:?}", handle);
    
    assert!(debug_str.contains("42"));
}

#[test]
fn test_animation_value_number() {
    let value = AnimationValue::Number(42.5);
    
    match value {
        AnimationValue::Number(n) => assert_eq!(n, 42.5),
        _ => panic!("Expected Number variant"),
    }
}

#[test]
fn test_animation_value_string() {
    let value = AnimationValue::String("test".to_string());
    
    match value {
        AnimationValue::String(s) => assert_eq!(s, "test"),
        _ => panic!("Expected String variant"),
    }
}

#[test]
fn test_animation_value_pixels() {
    let value = AnimationValue::Pixels(100.0);
    
    match value {
        AnimationValue::Pixels(p) => assert_eq!(p, 100.0),
        _ => panic!("Expected Pixels variant"),
    }
}

#[test]
fn test_animation_value_clone() {
    let value = AnimationValue::Number(42.5);
    let cloned = value.clone();
    
    assert_eq!(value, cloned);
}

#[test]
fn test_animation_value_equality() {
    let value1 = AnimationValue::Number(42.5);
    let value2 = AnimationValue::Number(42.5);
    let value3 = AnimationValue::Number(43.0);
    
    assert_eq!(value1, value2);
    assert_ne!(value1, value3);
}

#[test]
fn test_animation_value_debug() {
    let values = vec![
        AnimationValue::Number(42.5),
        AnimationValue::String("test".to_string()),
        AnimationValue::Pixels(100.0),
    ];
    
    for value in values {
        let debug_str = format!("{:?}", value);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_animation_target_creation() {
    let target = AnimationTarget::new();
    
    assert_eq!(target.len(), 0);
}

#[test]
fn test_animation_target_with_value() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    assert_eq!(target.len(), 1);
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));
}

#[test]
fn test_animation_target_multiple_values() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    target.insert("rotation".to_string(), AnimationValue::String("45deg".to_string()));
    
    assert_eq!(target.len(), 3);
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.5)));
    assert_eq!(target.get("rotation"), Some(&AnimationValue::String("45deg".to_string())));
}

#[test]
fn test_animation_target_get_nonexistent() {
    let target = AnimationTarget::new();
    
    assert_eq!(target.get("nonexistent"), None);
}

#[test]
fn test_animation_target_remove() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    target.remove("opacity");
    
    assert_eq!(target.len(), 1);
    assert_eq!(target.get("opacity"), None);
    assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.5)));
}

#[test]
fn test_animation_target_clear() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    target.clear();
    
    assert_eq!(target.len(), 0);
}

#[test]
fn test_animation_target_clone() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    let cloned = target.clone();
    
    assert_eq!(cloned.len(), 2);
    assert_eq!(cloned.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(cloned.get("scale"), Some(&AnimationValue::Number(1.5)));
}

#[test]
fn test_animation_target_debug() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    let debug_str = format!("{:?}", target);
    
    assert!(debug_str.contains("opacity"));
    assert!(debug_str.contains("1.0"));
}

#[test]
fn test_transition_creation() {
    let transition = Transition::new();
    
    assert_eq!(transition.duration, None);
    assert_eq!(transition.ease, Easing::Linear);
    assert_eq!(transition.delay, None);
    assert_eq!(transition.repeat, RepeatConfig::None);
}

#[test]
fn test_transition_with_duration() {
    let mut transition = Transition::new();
    transition.duration = Some(0.5);
    
    assert_eq!(transition.duration, Some(0.5));
    assert_eq!(transition.ease, Easing::Linear);
    assert_eq!(transition.delay, None);
}

#[test]
fn test_transition_with_ease() {
    let mut transition = Transition::new();
    transition.ease = Easing::EaseInOut;
    
    assert_eq!(transition.duration, None);
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.delay, None);
}

#[test]
fn test_transition_with_delay() {
    let mut transition = Transition::new();
    transition.delay = Some(0.1);
    
    assert_eq!(transition.duration, None);
    assert_eq!(transition.ease, Easing::Linear);
    assert_eq!(transition.delay, Some(0.1));
}

#[test]
fn test_transition_with_repeat() {
    let repeat_config = RepeatConfig::Count(3);
    let mut transition = Transition::new();
    transition.repeat = repeat_config.clone();
    
    assert_eq!(transition.duration, None);
    assert_eq!(transition.ease, Easing::Linear);
    assert_eq!(transition.delay, None);
    assert_eq!(transition.repeat, repeat_config);
}

#[test]
fn test_transition_clone() {
    let mut transition = Transition::new();
    transition.duration = Some(0.5);
    transition.ease = Easing::EaseInOut;
    
    let cloned = transition.clone();
    
    assert_eq!(cloned.duration, transition.duration);
    assert_eq!(cloned.ease, transition.ease);
    assert_eq!(cloned.delay, transition.delay);
    assert_eq!(cloned.repeat, transition.repeat);
}

#[test]
fn test_transition_debug() {
    let mut transition = Transition::new();
    transition.duration = Some(0.5);
    transition.ease = Easing::EaseInOut;
    
    let debug_str = format!("{:?}", transition);
    
    assert!(debug_str.contains("Transition"));
    assert!(debug_str.contains("0.5"));
    assert!(debug_str.contains("EaseInOut"));
}

#[test]
fn test_easing_variants() {
    let easings = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
        Easing::CircIn,
        Easing::CircOut,
    ];
    
    for easing in easings {
        let debug_str = format!("{:?}", easing);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_easing_clone() {
    let easing = Easing::EaseInOut;
    let cloned = easing.clone();
    
    assert_eq!(easing, cloned);
}

#[test]
fn test_easing_equality() {
    let easing1 = Easing::EaseInOut;
    let easing2 = Easing::EaseInOut;
    let easing3 = Easing::EaseIn;
    
    assert_eq!(easing1, easing2);
    assert_ne!(easing1, easing3);
}

#[test]
fn test_spring_config_creation() {
    let config = SpringConfig::new();
    
    assert_eq!(config.stiffness, 100.0);
    assert_eq!(config.damping, 10.0);
    assert_eq!(config.mass, 1.0);
    assert_eq!(config.rest_delta, 0.01);
}

#[test]
fn test_spring_config_with_stiffness() {
    let mut config = SpringConfig::new();
    config.stiffness = 200.0;
    
    assert_eq!(config.stiffness, 200.0);
    assert_eq!(config.damping, 10.0);
    assert_eq!(config.mass, 1.0);
}

#[test]
fn test_spring_config_with_damping() {
    let mut config = SpringConfig::new();
    config.damping = 20.0;
    
    assert_eq!(config.stiffness, 100.0);
    assert_eq!(config.damping, 20.0);
    assert_eq!(config.mass, 1.0);
}

#[test]
fn test_spring_config_with_mass() {
    let mut config = SpringConfig::new();
    config.mass = 2.0;
    
    assert_eq!(config.stiffness, 100.0);
    assert_eq!(config.damping, 10.0);
    assert_eq!(config.mass, 2.0);
}

#[test]
fn test_spring_config_with_rest_delta() {
    let mut config = SpringConfig::new();
    config.rest_delta = 0.005;
    
    assert_eq!(config.stiffness, 100.0);
    assert_eq!(config.damping, 10.0);
    assert_eq!(config.mass, 1.0);
    assert_eq!(config.rest_delta, 0.005);
}

#[test]
fn test_spring_config_clone() {
    let mut config = SpringConfig::new();
    config.stiffness = 200.0;
    config.damping = 20.0;
    
    let cloned = config.clone();
    
    assert_eq!(cloned.stiffness, config.stiffness);
    assert_eq!(cloned.damping, config.damping);
    assert_eq!(cloned.mass, config.mass);
    assert_eq!(cloned.rest_delta, config.rest_delta);
}

#[test]
fn test_spring_config_debug() {
    let mut config = SpringConfig::new();
    config.stiffness = 200.0;
    config.damping = 20.0;
    
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("SpringConfig"));
    assert!(debug_str.contains("200.0"));
    assert!(debug_str.contains("20.0"));
}

#[test]
fn test_repeat_config_variants() {
    let repeat_configs = vec![
        RepeatConfig::None,
        RepeatConfig::Count(3),
        RepeatConfig::Infinite,
        RepeatConfig::Mirror,
    ];
    
    for config in repeat_configs {
        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_repeat_config_clone() {
    let config = RepeatConfig::Count(3);
    let cloned = config.clone();
    
    assert_eq!(config, cloned);
}

#[test]
fn test_repeat_config_equality() {
    let config1 = RepeatConfig::Count(3);
    let config2 = RepeatConfig::Count(3);
    let config3 = RepeatConfig::Count(4);
    
    assert_eq!(config1, config2);
    assert_ne!(config1, config3);
}

#[test]
fn test_animation_target_iter() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    let mut values: Vec<_> = target.iter().collect();
    values.sort_by(|a, b| a.0.cmp(b.0));
    
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], ("opacity", &AnimationValue::Number(1.0)));
    assert_eq!(values[1], ("scale", &AnimationValue::Number(1.5)));
}

#[test]
fn test_animation_target_keys() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    let mut keys: Vec<_> = target.keys().collect();
    keys.sort();
    
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0], "opacity");
    assert_eq!(keys[1], "scale");
}

#[test]
fn test_animation_target_values() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    
    let mut values: Vec<_> = target.values().collect();
    values.sort_by(|a, b| match (a, b) {
        (AnimationValue::Number(n1), AnimationValue::Number(n2)) => n1.partial_cmp(n2).unwrap(),
        _ => std::cmp::Ordering::Equal,
    });
    
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], &AnimationValue::Number(1.0));
    assert_eq!(values[1], &AnimationValue::Number(1.5));
}

#[test]
fn test_animation_target_len() {
    let mut target = AnimationTarget::new();
    assert_eq!(target.len(), 0);
    
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    assert_eq!(target.len(), 1);
    
    target.insert("scale".to_string(), AnimationValue::Number(1.5));
    assert_eq!(target.len(), 2);
    
    target.remove("opacity");
    assert_eq!(target.len(), 1);
}

#[test]
fn test_animation_target_is_empty() {
    let mut target = AnimationTarget::new();
    assert!(target.is_empty());
    
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    assert!(!target.is_empty());
    
    target.clear();
    assert!(target.is_empty());
}

#[test]
fn test_animation_target_contains_key() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    assert!(target.contains_key("opacity"));
    assert!(!target.contains_key("scale"));
}

#[test]
fn test_animation_target_get_or_default() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    let opacity = target.get("opacity").unwrap_or(&AnimationValue::Number(0.0));
    let scale = target.get("scale").unwrap_or(&AnimationValue::Number(1.0));
    
    assert_eq!(opacity, &AnimationValue::Number(1.0));
    assert_eq!(scale, &AnimationValue::Number(1.0));
}

#[test]
fn test_animation_target_merge() {
    let mut target1 = AnimationTarget::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    let mut target2 = AnimationTarget::new();
    target2.insert("scale".to_string(), AnimationValue::Number(1.5));
    target2.insert("opacity".to_string(), AnimationValue::Number(0.5)); // Override
    
    target1.extend(target2);
    
    assert_eq!(target1.len(), 2);
    assert_eq!(target1.get("opacity"), Some(&AnimationValue::Number(0.5)));
    assert_eq!(target1.get("scale"), Some(&AnimationValue::Number(1.5)));
}
