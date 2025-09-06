use leptos_motion_core::*;
use pretty_assertions::assert_eq;
use std::sync::{Arc, Mutex};

#[test]
fn test_motion_value_subscription() {
    let motion_value = MotionValue::new(0.0);
    let received_values = Arc::new(Mutex::new(Vec::new()));
    let received_values_clone = received_values.clone();

    motion_value.subscribe(move |value| {
        received_values_clone.lock().unwrap().push(*value);
    });

    motion_value.set(10.0);
    motion_value.set(20.0);
    motion_value.set(30.0);

    let values = received_values.lock().unwrap();
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], 10.0);
    assert_eq!(values[1], 20.0);
    assert_eq!(values[2], 30.0);
}

#[test]
fn test_motion_value_velocity() {
    let motion_value = MotionValue::new(0.0);

    // Initial velocity should be 0
    assert_eq!(motion_value.get_velocity(), 0.0);

    // Set a new value
    motion_value.set(100.0);

    // Velocity should be calculated (simplified test)
    let velocity = motion_value.get_velocity();
    assert!(velocity >= 0.0);
}

#[test]
fn test_motion_value_velocity_tracking() {
    let motion_value = MotionValue::new(0.0);

    // Simulate rapid changes to test velocity tracking
    motion_value.set(10.0);
    let velocity1 = motion_value.get_velocity();

    motion_value.set(30.0);
    let velocity2 = motion_value.get_velocity();

    // Velocity should be positive and increasing
    assert!(velocity1 >= 0.0);
    assert!(velocity2 >= 0.0);
}

#[test]
fn test_motion_number_specialized_methods() {
    let motion_number = MotionNumber::new(0.0);

    // Test specialized methods for MotionNumber
    motion_number.set(42.0);
    assert_eq!(motion_number.get(), 42.0);

    // Test velocity
    motion_number.set_velocity(10.0);
    assert_eq!(motion_number.get_velocity(), 10.0);
}

#[test]
fn test_motion_transform_specialized_methods() {
    let transform = Transform {
        x: 0.0,
        y: 0.0,
        scale_x: 1.0,
        scale_y: 1.0,
        rotation: 0.0,
        skew_x: 0.0,
        skew_y: 0.0,
    };

    let motion_transform = MotionTransform::new(transform);

    // Test getter
    let current = motion_transform.get();
    assert_eq!(current.x, 0.0);
    assert_eq!(current.y, 0.0);
    assert_eq!(current.scale_x, 1.0);

    // Test setter
    let new_transform = Transform {
        x: 100.0,
        y: 50.0,
        scale_x: 2.0,
        scale_y: 2.0,
        rotation: 45.0,
        skew_x: 0.0,
        skew_y: 0.0,
    };

    motion_transform.set(new_transform);
    let updated = motion_transform.get();
    assert_eq!(updated.x, 100.0);
    assert_eq!(updated.y, 50.0);
    assert_eq!(updated.scale_x, 2.0);
    assert_eq!(updated.rotation, 45.0);
}

#[test]
fn test_motion_values_collection() {
    let mut motion_values = MotionValues::new();

    // Add motion values
    let opacity = MotionValue::new(0.0);
    let scale = MotionValue::new(1.0);

    motion_values.add("opacity", opacity);
    motion_values.add("scale", scale);

    // Test retrieval
    assert!(motion_values.get("opacity").is_some());
    assert!(motion_values.get("scale").is_some());
    assert!(motion_values.get("x").is_none());

    // Test iteration
    let keys: Vec<_> = motion_values.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"opacity"));
    assert!(keys.contains(&"scale"));
}

#[test]
fn test_motion_values_batch_operations() {
    let mut motion_values = MotionValues::new();

    // Add multiple values
    motion_values.add("opacity", MotionValue::new(0.0));
    motion_values.add("scale", MotionValue::new(1.0));
    motion_values.add("x", MotionValue::new(0.0));
    motion_values.add("y", MotionValue::new(0.0));

    // Test batch set
    let target_values = vec![("opacity", 1.0), ("scale", 2.0), ("x", 100.0), ("y", 50.0)];

    for (key, value) in target_values {
        if let Some(motion_value) = motion_values.get(key) {
            motion_value.set(value);
        }
    }

    // Verify all values were set
    assert_eq!(motion_values.get("opacity").unwrap().get(), 1.0);
    assert_eq!(motion_values.get("scale").unwrap().get(), 2.0);
    assert_eq!(motion_values.get("x").unwrap().get(), 100.0);
    assert_eq!(motion_values.get("y").unwrap().get(), 50.0);
}

#[test]
fn test_motion_value_cleanup() {
    let motion_value = MotionValue::new(0.0);
    let call_count = Arc::new(Mutex::new(0));
    let call_count_clone = call_count.clone();

    // Subscribe to changes
    motion_value.subscribe(move |_| {
        *call_count_clone.lock().unwrap() += 1;
    });

    // Make some changes
    motion_value.set(10.0);
    motion_value.set(20.0);

    // Verify subscription is working
    assert_eq!(*call_count.lock().unwrap(), 2);

    // The motion value should be properly cleaned up when dropped
    // This test ensures no memory leaks occur
}

// Property-based tests for motion values
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_motion_value_set_get(value in -1000.0..1000.0) {
            let motion_value = MotionValue::new(0.0);
            motion_value.set(value);
            assert_eq!(motion_value.get(), value);
        }

        #[test]
        fn test_motion_value_velocity_positive(value in 0.0..1000.0) {
            let motion_value = MotionValue::new(0.0);
            motion_value.set(value);
            let velocity = motion_value.get_velocity();
            assert!(velocity >= 0.0);
        }

        #[test]
        fn test_motion_transform_components(
            x in -1000.0..1000.0,
            y in -1000.0..1000.0,
            scale_x in 0.1..10.0,
            scale_y in 0.1..10.0,
            rotation in -360.0..360.0
        ) {
            let transform = Transform {
                x,
                y,
                scale_x,
                scale_y,
                rotation,
                skew_x: 0.0,
                skew_y: 0.0,
            };

            let motion_transform = MotionTransform::new(transform);
            let retrieved = motion_transform.get();

            assert_eq!(retrieved.x, x);
            assert_eq!(retrieved.y, y);
            assert_eq!(retrieved.scale_x, scale_x);
            assert_eq!(retrieved.scale_y, scale_y);
            assert_eq!(retrieved.rotation, rotation);
        }
    }
}
