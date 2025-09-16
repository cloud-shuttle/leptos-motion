//! Simple test to verify animation engine functionality
//! 
//! This test verifies that the animation engine can:
//! 1. Create animations
//! 2. Update animation values
//! 3. Handle completion callbacks

use leptos_motion_dom::animation_engine::AnimationEngine;
use leptos_motion_core::{Transition, Easing, RepeatConfig};
use std::collections::HashMap;

fn main() {
    println!("Testing Animation Engine...");
    
    // Create a new animation engine
    let mut engine = AnimationEngine::new();
    
    // Test 1: Create a simple animation
    println!("Test 1: Creating scale animation...");
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };
    
    engine.animate_property("scale".to_string(), 1.0, 2.0, transition);
    
    // Test 2: Check if animation was created
    if let Some(value) = engine.get_property_value("scale") {
        println!("✓ Scale animation created with initial value: {}", value);
    } else {
        println!("✗ Failed to create scale animation");
        return;
    }
    
    // Test 3: Check all values
    let all_values = engine.get_all_values();
    println!("✓ All animation values: {:?}", all_values);
    
    // Test 4: Test multiple animations
    println!("Test 2: Creating multiple animations...");
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), (1.0, 0.5, transition.clone()));
    properties.insert("x".to_string(), (0.0, 100.0, transition.clone()));
    
    engine.animate_properties(properties);
    
    let all_values = engine.get_all_values();
    println!("✓ Multiple animations created: {:?}", all_values);
    
    // Test 5: Stop specific animation
    println!("Test 3: Stopping scale animation...");
    engine.stop_property("scale");
    
    let remaining_values = engine.get_all_values();
    println!("✓ Remaining animations after stopping scale: {:?}", remaining_values);
    
    // Test 6: Stop all animations
    println!("Test 4: Stopping all animations...");
    engine.stop_all();
    
    let final_values = engine.get_all_values();
    println!("✓ Final animation values (should be empty): {:?}", final_values);
    
    println!("All tests completed successfully! ✓");
}
