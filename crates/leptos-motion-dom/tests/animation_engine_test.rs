//! Test animation engine functionality

use leptos_motion_dom::animation_engine::DomAnimationEngine;
use leptos_motion_core::{Transition, Easing, RepeatConfig, AnimationValue};
use std::collections::HashMap;

#[test]
fn test_animation_engine_creation() {
    let engine = DomAnimationEngine::new();
    assert!(engine.get_all_values().is_empty());
}

#[test]
fn test_animate_property() {
    let mut engine = DomAnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    let _ = engine.animate_property("scale".to_string(), 1.0, 2.0, transition);
    
    // Check if animation was created
    assert!(engine.get_property_value("scale").is_some());
    assert_eq!(engine.get_property_value("scale").unwrap(), &AnimationValue::Number(2.0));
    
    // Check all values
    let all_values = engine.get_all_values();
    assert_eq!(all_values.len(), 1);
    assert_eq!(all_values.get("scale").unwrap(), &AnimationValue::Number(2.0));
}

#[test]
fn test_animate_multiple_properties() {
    let mut engine = DomAnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), AnimationValue::Number(1.0));
    properties.insert("x".to_string(), AnimationValue::Number(100.0));
    
    let _ = engine.animate_properties(properties);
    
    let all_values = engine.get_all_values();
    assert_eq!(all_values.len(), 2);
    assert_eq!(all_values.get("opacity").unwrap(), &AnimationValue::Number(1.0));
    assert_eq!(all_values.get("x").unwrap(), &AnimationValue::Number(100.0));
}

#[test]
fn test_stop_property() {
    let mut engine = DomAnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    let _ = engine.animate_property("scale".to_string(), 1.0, 2.0, transition.clone());
    let _ = engine.animate_property("opacity".to_string(), 1.0, 0.5, transition);
    
    // Stop one property
    engine.stop_property("scale");
    
    let remaining_values = engine.get_all_values();
    assert_eq!(remaining_values.len(), 1);
    assert!(remaining_values.get("scale").is_none());
    assert!(remaining_values.get("opacity").is_some());
}

#[test]
fn test_stop_all() {
    let mut engine = DomAnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    let _ = engine.animate_property("scale".to_string(), 1.0, 2.0, transition.clone());
    let _ = engine.animate_property("opacity".to_string(), 1.0, 0.5, transition);
    
    // Stop all animations
    engine.stop_all();
    
    let final_values = engine.get_all_values();
    assert!(final_values.is_empty());
}
