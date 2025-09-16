//! Test animation engine functionality

use leptos_motion_dom::animation_engine::AnimationEngine;
use leptos_motion_core::{Transition, Easing, RepeatConfig};
use std::collections::HashMap;

#[test]
fn test_animation_engine_creation() {
    let engine = AnimationEngine::new();
    assert!(engine.get_all_values().is_empty());
}

#[test]
fn test_animate_property() {
    let mut engine = AnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    engine.animate_property("scale".to_string(), 1.0, 2.0, transition);
    
    // Check if animation was created
    assert!(engine.get_property_value("scale").is_some());
    assert_eq!(engine.get_property_value("scale").unwrap(), 1.0);
    
    // Check all values
    let all_values = engine.get_all_values();
    assert_eq!(all_values.len(), 1);
    assert_eq!(all_values.get("scale").unwrap(), &1.0);
}

#[test]
fn test_animate_multiple_properties() {
    let mut engine = AnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), (1.0, 0.5, transition.clone()));
    properties.insert("x".to_string(), (0.0, 100.0, transition.clone()));
    
    engine.animate_properties(properties);
    
    let all_values = engine.get_all_values();
    assert_eq!(all_values.len(), 2);
    assert_eq!(all_values.get("opacity").unwrap(), &1.0);
    assert_eq!(all_values.get("x").unwrap(), &0.0);
}

#[test]
fn test_stop_property() {
    let mut engine = AnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    engine.animate_property("scale".to_string(), 1.0, 2.0, transition.clone());
    engine.animate_property("opacity".to_string(), 1.0, 0.5, transition);
    
    // Stop one property
    engine.stop_property("scale");
    
    let remaining_values = engine.get_all_values();
    assert_eq!(remaining_values.len(), 1);
    assert!(remaining_values.get("scale").is_none());
    assert!(remaining_values.get("opacity").is_some());
}

#[test]
fn test_stop_all() {
    let mut engine = AnimationEngine::new();
    
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    engine.animate_property("scale".to_string(), 1.0, 2.0, transition.clone());
    engine.animate_property("opacity".to_string(), 1.0, 0.5, transition);
    
    // Stop all animations
    engine.stop_all();
    
    let final_values = engine.get_all_values();
    assert!(final_values.is_empty());
}
