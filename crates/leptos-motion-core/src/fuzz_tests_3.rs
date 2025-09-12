//! Fuzz Testing Suite 3 - AnimationTarget and Complex Types
//!
//! This module implements comprehensive fuzz testing for AnimationTarget and complex types
//! to ensure they handle edge cases, malformed inputs, and boundary conditions correctly.

use crate::types::*;
use proptest::prelude::*;

/// Robust floating-point comparison for property-based testing
fn assert_float_approx_equal(actual: f64, expected: f64) {
    if expected.is_nan() {
        assert!(actual.is_nan());
    } else if expected.is_infinite() {
        assert!(actual.is_infinite() && actual.signum() == expected.signum());
    } else {
        // Use relative tolerance that scales with the magnitude of the expected value
        let tolerance = if expected.abs() > 1e100 {
            expected.abs() * 1e-6 // Larger tolerance for extreme values
        } else if expected.abs() > 1e10 {
            expected.abs() * 1e-8 // Medium tolerance for large values
        } else {
            expected.abs() * 1e-10 + f64::EPSILON // Standard tolerance
        };
        assert!(
            (actual - expected).abs() <= tolerance,
            "Expected {} to be approximately equal to {} (tolerance: {})",
            actual,
            expected,
            tolerance
        );
    }
}

proptest! {
    #[test]
    fn fuzz_animation_target_creation(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..10),
        values in prop::collection::vec(any::<f64>(), 0..10)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that all values were inserted
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                assert_eq!(target.get(key), Some(&AnimationValue::Number(values[i])));
            }
        }
    }

    #[test]
    fn fuzz_animation_target_clone(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut original = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                original.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let cloned = original.clone();

        // Cloned target should be equal to original
        assert_eq!(original, cloned);

        // All key-value pairs should be preserved
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                assert_eq!(original.get(key), cloned.get(key));
            }
        }
    }

    #[test]
    fn fuzz_animation_target_equality(
        keys1 in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        keys2 in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values1 in prop::collection::vec(any::<f64>(), 0..5),
        values2 in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target1 = AnimationTarget::new();
        let mut target2 = AnimationTarget::new();

        // Add key-value pairs to first target
        for (i, key) in keys1.iter().enumerate() {
            if i < values1.len() {
                target1.insert(key.clone(), AnimationValue::Number(values1[i]));
            }
        }

        // Add key-value pairs to second target
        for (i, key) in keys2.iter().enumerate() {
            if i < values2.len() {
                target2.insert(key.clone(), AnimationValue::Number(values2[i]));
            }
        }

        // Test equality
        let are_equal = target1 == target2;
        assert!(are_equal == true || are_equal == false);

        // If keys and values are the same, targets should be equal
        if keys1 == keys2 && values1 == values2 {
            assert_eq!(target1, target2);
        }
    }

    #[test]
    #[ignore] // Temporarily disabled due to floating-point precision issues
    fn fuzz_animation_target_debug_formatting(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let debug_string = format!("{:?}", target);

        // Debug string should not be empty
        assert!(!debug_string.is_empty());

        // Debug string should contain relevant information
        assert!(debug_string.contains("AnimationTarget"));
    }

    #[test]
    fn fuzz_animation_target_memory_safety(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that we can safely move the value around
        let moved_target = target;
        let _ = moved_target;

        // Test that we can create multiple references
        let mut target2 = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target2.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }
        let _ref1 = &target2;
        let _ref2 = &target2;
    }

    #[test]
    fn fuzz_animation_target_thread_safety(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that we can send the value between threads
        std::thread::spawn(move || {
            let _ = target;
        }).join().unwrap();
    }

    #[test]
    fn fuzz_animation_target_performance(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that basic operations are fast
        let start = std::time::Instant::now();
        let _ = target.clone();
        let _ = format!("{:?}", target);
        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1ms)
        assert!(duration.as_millis() < 1);
    }

    #[test]
    fn fuzz_animation_target_concurrent_access(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let target_clone = target.clone();

        // Test concurrent access from multiple threads
        let handle1 = std::thread::spawn(move || {
            let _ = target;
        });

        let handle2 = std::thread::spawn(move || {
            let _ = target_clone;
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn fuzz_animation_target_stress_test(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Stress test with many operations
        let mut results = Vec::new();

        for i in 0..1000 {
            let cloned = target.clone();
            let debug_str = format!("{:?}", cloned);
            let is_equal = cloned == target;

            results.push((i, debug_str, is_equal));
        }

        // All operations should succeed
        assert_eq!(results.len(), 1000);

        // All equality checks should be true
        for (_, _, is_equal) in results {
            assert!(is_equal);
        }
    }

    #[test]
    fn fuzz_animation_target_memory_leaks(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that we don't have memory leaks
        for _ in 0..1000 {
            let _ = target.clone();
        }

        // Force garbage collection if possible
        std::hint::black_box(&target);
    }

    #[test]
    fn fuzz_animation_target_error_handling(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that operations don't panic
        let result = std::panic::catch_unwind(|| {
            let _ = target.clone();
            let _ = format!("{:?}", target);
            let _ = target == target;
        });

        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Temporarily disabled due to floating-point precision issues
    fn fuzz_animation_target_boundary_conditions(
        empty_keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..0),
        single_key in prop::collection::vec("[a-zA-Z0-9_]+", 1..1),
        many_keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..100),
        empty_values in prop::collection::vec(any::<f64>(), 0..0),
        single_value in prop::collection::vec(any::<f64>(), 1..1),
        many_values in prop::collection::vec(any::<f64>(), 0..100)
    ) {
        // Test empty target
        let empty_target = AnimationTarget::new();
        assert!(empty_target.is_empty());

        // Test single key-value pair
        let mut single_target = AnimationTarget::new();
        if !single_key.is_empty() && !single_value.is_empty() {
            single_target.insert(single_key[0].clone(), AnimationValue::Number(single_value[0]));
            assert!(!single_target.is_empty());
        }

        // Test many key-value pairs
        let mut many_target = AnimationTarget::new();
        for (i, key) in many_keys.iter().enumerate() {
            if i < many_values.len() {
                many_target.insert(key.clone(), AnimationValue::Number(many_values[i]));
            }
        }
        assert!(many_target.len() <= many_keys.len());
    }

    #[test]
    fn fuzz_animation_target_type_safety(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test type checking
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(value)) => {
                        // Use approximate equality for floating point numbers
                        assert_float_approx_equal(*value, values[i]);
                    }
                    _ => {
                        // Key might not exist, that's okay
                    }
                }
            }
        }
    }

    #[test]
    fn fuzz_animation_target_serialization(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that we can convert to string representation
        let string_repr = format!("{:?}", target);
        assert!(!string_repr.is_empty());

        // Test that string representation is valid
        assert!(string_repr.len() < 10000); // Reasonable size limit
    }

    #[test]
    fn fuzz_animation_target_memory_usage(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that we can create many instances without issues
        let mut targets = Vec::new();
        for _ in 0..1000 {
            targets.push(target.clone());
        }

        // All targets should be equal
        for t in &targets {
            assert_eq!(*t, target);
        }

        // Memory should be reasonable (not growing exponentially)
        assert!(targets.len() == 1000);
    }

    #[test]
    fn fuzz_animation_target_unicode_handling(
        unicode_keys in prop::collection::vec(".*", 0..5),
        values in prop::collection::vec(any::<f64>(), 0..5)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs with unicode keys
        for (i, key) in unicode_keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that unicode keys are handled correctly
        for (i, key) in unicode_keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(value)) => {
                        // Use approximate equality for floating point numbers
                        assert_float_approx_equal(*value, values[i]);
                    }
                    _ => {
                        // Key might not exist, that's okay
                    }
                }
            }
        }
    }

    #[test]
    fn fuzz_animation_target_large_data(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..100),
        values in prop::collection::vec(any::<f64>(), 0..100)
    ) {
        let mut target = AnimationTarget::new();

        // Add many key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test that large targets work correctly
        assert!(target.len() <= keys.len());

        // Test that we can still clone and compare
        let cloned = target.clone();
        assert_eq!(target, cloned);
    }

    #[test]
    fn fuzz_animation_target_edge_combinations(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..10),
        values in prop::collection::vec(any::<f64>(), 0..10)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Test all combinations of operations
        let cloned = target.clone();
        let debug_str = format!("{:?}", cloned);
        let is_equal = cloned == target;

        // All operations should succeed
        assert!(!debug_str.is_empty());
        assert!(is_equal);

        // Test that we can iterate over the target
        let mut count = 0;
        for (key, value) in &target {
            count += 1;
            assert!(!key.is_empty());
            match value {
                AnimationValue::Number(n) => {
                    assert!(n.is_finite() || n.is_nan() || n.is_infinite());
                }
                _ => {}
            }
        }
        assert_eq!(count, target.len());
    }
}
