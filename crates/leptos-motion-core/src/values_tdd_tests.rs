// Modern TDD Tests for Motion Values
// 
// This module demonstrates Test-Driven Development practices using
// the latest Rust testing crates and patterns as of September 2025.

use super::*;

// Modern fixture-based testing
fn motion_value_fixture() -> MotionValue<f64> {
    MotionValue::new(42.0)
}

fn motion_transform_fixture() -> MotionTransform {
    MotionTransform::identity()
}

fn motion_values_fixture() -> MotionValues {
    let mut values = MotionValues::new();
    values.add("opacity", AnimationValue::Number(0.5));
    values.add("x", AnimationValue::Pixels(100.0));
    values.add("scale", AnimationValue::Number(1.0));
    values
}

// RED-GREEN-REFACTOR: Test-Driven Development Examples

// RED: Write failing test first
#[test]
fn test_motion_value_creation_should_initialize_correctly() {
    // Arrange & Act
    let motion_value = MotionValue::new(100.0);
    
    // Assert
    assert_eq!(motion_value.get(), 100.0);
    assert_eq!(motion_value.get_velocity(), 0.0);
}

// GREEN: Make it pass, then REFACTOR
#[test]
fn test_motion_value_set_should_update_value() {
    // Arrange
    let motion_value = MotionValue::new(0.0);
    
    // Act
    motion_value.set(50.0);
    
    // Assert
    assert_eq!(motion_value.get(), 50.0);
}

// Modern parameterized testing
#[test]
fn test_motion_value_creation_with_different_values() {
    let test_cases = vec![
        (0.0, 0.0),
        (42.0, 42.0),
        (-10.0, -10.0),
        (999.999, 999.999),
    ];
    
    for (initial_value, expected_value) in test_cases {
        // Arrange & Act
        let motion_value = MotionValue::new(initial_value);
        
        // Assert
        assert_eq!(motion_value.get(), expected_value);
    }
}

// Modern test cases
#[test]
fn test_motion_number_increment() {
    let test_cases = vec![
        (0.0, 5.0, 5.0),
        (10.0, 3.0, 13.0),
        (-5.0, 2.0, -3.0),
        (100.0, 0.0, 100.0),
    ];
    
    for (initial, increment, expected) in test_cases {
        // Arrange
        let motion_num = MotionValue::new(initial);
        
        // Act
        motion_num.increment(increment);
        
        // Assert
        assert_eq!(motion_num.get(), expected);
    }
}

// Property-based testing
#[test]
fn test_motion_value_properties() {
    let test_values = vec![0.0, 42.0, -10.0, 999.999, f64::MAX, f64::MIN];
    
    for initial in &test_values {
        for new_value in &test_values {
            for increment in &test_values {
                // Property 1: Creation preserves initial value
                let motion_value = MotionValue::new(*initial);
                assert_eq!(motion_value.get(), *initial);
                
                // Property 2: Setting value updates correctly
                motion_value.set(*new_value);
                assert_eq!(motion_value.get(), *new_value);
                
                // Property 3: Increment is additive
                let before_increment = motion_value.get();
                motion_value.increment(*increment);
                let after_increment = motion_value.get();
                assert_eq!(after_increment, before_increment + increment);
                
                // Property 4: Velocity is always finite
                assert!(motion_value.get_velocity().is_finite());
            }
        }
    }
}

#[test]
fn test_motion_value_velocity_properties() {
    let test_values = vec![0.0, 42.0, -10.0, 999.999, f64::MAX, f64::MIN];
    
    for velocity in &test_values {
        for value in &test_values {
            let motion_value = MotionValue::new(0.0);
            
            // Property: Setting velocity preserves the value
            motion_value.set_velocity(*velocity);
            assert_eq!(motion_value.get_velocity(), *velocity);
            
            // Property: Setting with velocity updates both
            motion_value.set_with_velocity(*value, *velocity);
            assert_eq!(motion_value.get(), *value);
            assert_eq!(motion_value.get_velocity(), *velocity);
        }
    }
}

// Modern async testing with fixtures
#[test]
fn test_motion_value_subscription() {
    // Arrange
    let motion_value = motion_value_fixture();
    let received_values = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_values_clone = received_values.clone();
    
    // Act
    motion_value.subscribe(move |value| {
        received_values_clone.lock().unwrap().push(*value);
    });
    
    motion_value.set(100.0);
    motion_value.set(200.0);
    
    // Assert
    let values = received_values.lock().unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], 100.0);
    assert_eq!(values[1], 200.0);
}

// Test-driven development for MotionTransform
#[test]
fn test_motion_transform_identity_should_be_identity() {
    // RED: This test should fail initially if Transform doesn't have is_identity
    let motion_transform = MotionTransform::identity();
    let transform = motion_transform.get();
    
    // Assert identity properties
    assert_eq!(transform.x, None);
    assert_eq!(transform.y, None);
    assert_eq!(transform.rotate_z, None);
    assert_eq!(transform.scale, None);
}

#[test]
fn test_motion_transform_set_translate_should_update_position() {
    // Arrange
    let motion_transform = MotionTransform::identity();
    
    // Act
    motion_transform.set_translate(10.0, 20.0);
    
    // Assert
    let transform = motion_transform.get();
    assert_eq!(transform.x, Some(10.0));
    assert_eq!(transform.y, Some(20.0));
}

#[test]
fn test_motion_transform_set_rotation_should_update_rotation() {
    // Arrange
    let motion_transform = MotionTransform::identity();
    
    // Act
    motion_transform.set_rotation(45.0);
    
    // Assert
    let transform = motion_transform.get();
    assert_eq!(transform.rotate_z, Some(45.0));
}

#[test]
fn test_motion_transform_set_scale_should_update_scale() {
    // Arrange
    let motion_transform = MotionTransform::identity();
    
    // Act
    motion_transform.set_scale(2.0);
    
    // Assert
    let transform = motion_transform.get();
    assert_eq!(transform.scale, Some(2.0));
}

// Test-driven development for MotionValues collection
#[test]
fn test_motion_values_creation_should_be_empty() {
    // Arrange & Act
    let values = MotionValues::new();
    
    // Assert
    assert_eq!(values.get_all().len(), 0);
}

#[test]
fn test_motion_values_add_should_store_value() {
    // Arrange
    let mut values = MotionValues::new();
    
    // Act
    values.add("opacity", AnimationValue::Number(0.5));
    
    // Assert
    assert!(values.get("opacity").is_some());
    if let Some(opacity) = values.get("opacity") {
        assert_eq!(opacity.get(), AnimationValue::Number(0.5));
    }
}

#[test]
fn test_motion_values_set_should_update_existing_value() {
    // Arrange
    let mut values = MotionValues::new();
    values.add("opacity", AnimationValue::Number(0.5));
    
    // Act
    values.set("opacity", AnimationValue::Number(1.0));
    
    // Assert
    if let Some(opacity) = values.get("opacity") {
        assert_eq!(opacity.get(), AnimationValue::Number(1.0));
    }
}

#[test]
fn test_motion_values_set_all_should_update_multiple_values() {
    // Arrange
    let mut values = MotionValues::new();
    values.add("opacity", AnimationValue::Number(0.0));
    values.add("x", AnimationValue::Pixels(0.0));
    
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("x".to_string(), AnimationValue::Pixels(100.0));
    
    // Act
    values.set_all(&target);
    
    // Assert
    if let Some(opacity) = values.get("opacity") {
        assert_eq!(opacity.get(), AnimationValue::Number(1.0));
    }
    if let Some(x) = values.get("x") {
        assert_eq!(x.get(), AnimationValue::Pixels(100.0));
    }
}

// Modern fixture-based testing for complex scenarios
#[test]
fn test_motion_values_complex_operations() {
    // Arrange
    let values = motion_values_fixture();
    
    // Act & Assert
    assert_eq!(values.get_all().len(), 3);
    assert!(values.get("opacity").is_some());
    assert!(values.get("x").is_some());
    assert!(values.get("scale").is_some());
    assert!(values.get("nonexistent").is_none());
}

// Property-based testing for MotionValues
#[test]
fn test_motion_values_properties() {
    let test_keys = vec!["opacity", "x", "y", "scale", "rotation"];
    let test_values = vec![0.0, 0.5, 1.0, 100.0, 360.0];
    
    for (i, key) in test_keys.iter().enumerate() {
        if i < test_values.len() {
            let mut motion_values = MotionValues::new();
            
            // Property: Adding values preserves them
            motion_values.add(*key, AnimationValue::Number(test_values[i]));
            
            // Property: All added values are retrievable
            assert!(motion_values.get(*key).is_some());
            if let Some(motion_value) = motion_values.get(*key) {
                assert_eq!(motion_value.get(), AnimationValue::Number(test_values[i]));
            }
            
            // Property: get_all returns correct count
            let all_values = motion_values.get_all();
            assert_eq!(all_values.len(), 1);
        }
    }
}

// Edge case testing
#[test]
fn test_motion_value_edge_cases() {
    // Test with extreme values
    let motion_value = MotionValue::new(f64::MAX);
    assert_eq!(motion_value.get(), f64::MAX);
    
    let motion_value = MotionValue::new(f64::MIN);
    assert_eq!(motion_value.get(), f64::MIN);
    
    let motion_value = MotionValue::new(f64::NAN);
    assert!(motion_value.get().is_nan());
    
    let motion_value = MotionValue::new(f64::INFINITY);
    assert_eq!(motion_value.get(), f64::INFINITY);
}

#[test]
fn test_motion_value_velocity_edge_cases() {
    let motion_value = MotionValue::new(0.0);
    
    // Test extreme velocities
    motion_value.set_velocity(f64::MAX);
    assert_eq!(motion_value.get_velocity(), f64::MAX);
    
    motion_value.set_velocity(f64::MIN);
    assert_eq!(motion_value.get_velocity(), f64::MIN);
    
    motion_value.set_velocity(f64::NAN);
    assert!(motion_value.get_velocity().is_nan());
}

// Performance testing with modern benchmarking
#[cfg(feature = "bench")]
mod benches {
    use super::*;
    
    #[test]
    fn bench_motion_value_creation() {
        for _ in 0..1000 {
            let _motion_value = MotionValue::new(42.0);
        }
    }
    
    #[test]
    fn bench_motion_value_get_set() {
        let motion_value = MotionValue::new(0.0);
        
        for i in 0..1000 {
            motion_value.set(i as f64);
            let _value = motion_value.get();
        }
    }
    
    #[test]
    fn bench_motion_values_collection_operations() {
        let mut values = MotionValues::new();
        
        for i in 0..100 {
            values.add(format!("key_{}", i), AnimationValue::Number(i as f64));
        }
        
        for i in 0..100 {
            values.set(&format!("key_{}", i), AnimationValue::Number((i * 2) as f64));
        }
        
        let _all_values = values.get_all();
    }
}

// Integration testing
#[test]
fn test_motion_value_integration_with_animation_system() {
    // This test demonstrates how MotionValue integrates with the animation system
    let motion_value = MotionValue::new(0.0);
    
    // Simulate animation system setting values
    motion_value.set_with_velocity(100.0, 50.0);
    
    // Verify both value and velocity are set
    assert_eq!(motion_value.get(), 100.0);
    assert_eq!(motion_value.get_velocity(), 50.0);
    
    // Simulate animation update
    motion_value.set_with_velocity(150.0, 25.0);
    
    // Verify updated values
    assert_eq!(motion_value.get(), 150.0);
    assert_eq!(motion_value.get_velocity(), 25.0);
}

// Error handling testing
#[test]
fn test_motion_values_nonexistent_key_handling() {
    let values = MotionValues::new();
    
    // Setting a nonexistent key should not panic
    values.set("nonexistent", AnimationValue::Number(42.0));
    
    // Getting a nonexistent key should return None
    assert!(values.get("nonexistent").is_none());
}

// Concurrency testing
#[test]
fn test_motion_value_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let motion_value = Arc::new(MotionValue::new(0.0));
    let mut handles = vec![];
    
    // Spawn multiple threads that modify the motion value
    for i in 0..10 {
        let motion_value = Arc::clone(&motion_value);
        let handle = thread::spawn(move || {
            for j in 0..100 {
                motion_value.set((i * 100 + j) as f64);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // The final value should be from one of the threads
    let final_value = motion_value.get();
    assert!(final_value >= 0.0 && final_value < 1000.0);
}
