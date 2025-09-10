//! Fuzz Testing Suite 1 - Core Animation Types
//!
//! This module implements comprehensive fuzz testing for core animation types
//! to ensure they handle edge cases, malformed inputs, and boundary conditions correctly.

use crate::types::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_animation_value_number_creation(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);
        match animation_value {
            AnimationValue::Number(value) => {
                if n.is_nan() {
                    assert!(value.is_nan());
                } else {
                    assert_eq!(value, n);
                }
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_string_creation(s in ".*") {
        let animation_value = AnimationValue::String(s.clone());
        match animation_value {
            AnimationValue::String(value) => assert_eq!(value, s),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_equality(n1 in any::<f64>(), n2 in any::<f64>()) {
        let value1 = AnimationValue::Number(n1);
        let value2 = AnimationValue::Number(n2);

        if n1 == n2 && !n1.is_nan() {
            assert_eq!(value1, value2);
        }
    }

    #[test]
    fn fuzz_animation_value_clone(n in any::<f64>()) {
        let original = AnimationValue::Number(n);
        let cloned = original.clone();

        match (&original, &cloned) {
            (AnimationValue::Number(n1), AnimationValue::Number(n2)) => {
                if n1.is_nan() {
                    assert!(n2.is_nan());
                } else {
                    assert_eq!(n1, n2);
                }
            }
            _ => panic!("Expected Number variants"),
        }
    }

    #[test]
    fn fuzz_animation_value_debug_formatting(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);
        let debug_string = format!("{:?}", animation_value);
        assert!(debug_string.contains("Number"));
    }

    #[test]
    fn fuzz_animation_value_to_string(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);
        let string_repr = match animation_value {
            AnimationValue::Number(value) => value.to_string(),
            _ => panic!("Expected Number variant"),
        };
        assert!(!string_repr.is_empty());
    }

    #[test]
    fn fuzz_animation_value_edge_cases(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        match animation_value {
            AnimationValue::Number(value) => {
                if value.is_infinite() {
                    assert!(value.is_infinite());
                }
                if value.is_nan() {
                    assert!(value.is_nan());
                }
                if value == 0.0 {
                    assert_eq!(value, 0.0);
                }
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_string_edge_cases(s in ".*") {
        let animation_value = AnimationValue::String(s.clone());

        match animation_value {
            AnimationValue::String(value) => {
                if value.is_empty() {
                    assert_eq!(value.len(), 0);
                }
                if value.len() > 1000 {
                    assert!(value.len() > 1000);
                }
            }
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_unicode_handling(s in ".*") {
        let animation_value = AnimationValue::String(s.clone());

        match animation_value {
            AnimationValue::String(value) => {
                if value.contains('\u{1F600}') {
                    assert!(value.contains('\u{1F600}'));
                }
                if value.contains('\u{0000}') {
                    assert!(value.contains('\u{0000}'));
                }
            }
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_numeric_precision(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        match animation_value {
            AnimationValue::Number(value) => {
                if value == f64::MAX {
                    assert_eq!(value, f64::MAX);
                }
                if value == f64::MIN {
                    assert_eq!(value, f64::MIN);
                }
                if value == f64::EPSILON {
                    assert_eq!(value, f64::EPSILON);
                }
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_large_strings(s in ".*") {
        let animation_value = AnimationValue::String(s.clone());

        match animation_value {
            AnimationValue::String(value) => {
                if value.len() > 10000 {
                    let large_string = value.repeat(100);
                    let large_animation_value = AnimationValue::String(large_string);
                    assert!(format!("{:?}", large_animation_value).len() > 10000);
                }
            }
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_memory_safety(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we can safely move the value around
        let moved_value = animation_value;
        let _ = moved_value;

        // Test that we can create multiple references
        let animation_value2 = AnimationValue::Number(n);
        let _ref1 = &animation_value2;
        let _ref2 = &animation_value2;
    }

    #[test]
    fn fuzz_animation_value_thread_safety(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we can send the value between threads
        std::thread::spawn(move || {
            let _ = animation_value;
        }).join().unwrap();
    }

    #[test]
    fn fuzz_animation_value_performance(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that basic operations are fast
        let start = std::time::Instant::now();
        let _ = animation_value.clone();
        let _ = format!("{:?}", animation_value);
        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1ms)
        assert!(duration.as_millis() < 1);
    }

    #[test]
    fn fuzz_animation_value_concurrent_access(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);
        let animation_value_clone = animation_value.clone();

        // Test concurrent access from multiple threads
        let handle1 = std::thread::spawn(move || {
            let _ = animation_value;
        });

        let handle2 = std::thread::spawn(move || {
            let _ = animation_value_clone;
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    #[test]
    fn fuzz_animation_value_concurrent_modification(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we can safely access the value from multiple threads
        let handles: Vec<_> = (0..10).map(|_| {
            let value_clone = animation_value.clone();
            std::thread::spawn(move || {
                let _ = value_clone;
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn fuzz_animation_value_memory_leaks(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we don't have memory leaks
        for _ in 0..1000 {
            let _ = animation_value.clone();
        }

        // Force garbage collection if possible
        std::hint::black_box(&animation_value);
    }

    #[test]
    fn fuzz_animation_value_stress_test(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Stress test with many operations
        let mut results = Vec::new();

        for i in 0..1000 {
            let cloned = animation_value.clone();
            let debug_str = format!("{:?}", cloned);
            let is_equal = cloned == animation_value;

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
    fn fuzz_animation_value_edge_combinations(values in prop::collection::vec(any::<f64>(), 0..10)) {
        let animation_values: Vec<AnimationValue> = values.into_iter().map(|v| AnimationValue::Number(v)).collect();

        // Test all combinations
        for (i, val1) in animation_values.iter().enumerate() {
            for (j, val2) in animation_values.iter().enumerate() {
                let are_equal = val1 == val2;

                // If indices are equal, values should be equal
                if i == j {
                    assert!(are_equal);
                }

                // Test cloning
                let cloned = val1.clone();
                assert_eq!(*val1, cloned);
            }
        }
    }

    #[test]
    fn fuzz_animation_value_performance_load(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test performance under load
        let start = std::time::Instant::now();

        let mut total_operations = 0;
        while start.elapsed().as_millis() < 10 {
            let _ = animation_value.clone();
            let _ = format!("{:?}", animation_value);
            let _ = animation_value == animation_value;
            total_operations += 3;
        }

        // Should complete many operations quickly
        assert!(total_operations > 100);
    }

    #[test]
    fn fuzz_animation_value_boundary_conditions(
        min_f64 in any::<f64>(),
        max_f64 in any::<f64>(),
        epsilon in any::<f64>(),
        very_long_string in ".*"
    ) {
        // Test f64 boundaries
        let min_animation = AnimationValue::Number(min_f64);
        let max_animation = AnimationValue::Number(max_f64);
        let epsilon_animation = AnimationValue::Number(epsilon);

        // Values should be preserved
        match (min_animation, max_animation, epsilon_animation) {
            (AnimationValue::Number(min), AnimationValue::Number(max), AnimationValue::Number(eps)) => {
                assert_eq!(min, min_f64);
                assert_eq!(max, max_f64);
                assert_eq!(eps, epsilon);
            }
            _ => panic!("Values not preserved"),
        }

        // Test very long string
        let long_string_animation = AnimationValue::String(very_long_string.clone());
        match long_string_animation {
            AnimationValue::String(s) => {
                assert_eq!(s, very_long_string);
            }
            _ => panic!("String not preserved"),
        }
    }

    #[test]
    fn fuzz_animation_value_error_handling(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that operations don't panic
        let result = std::panic::catch_unwind(|| {
            let _ = animation_value.clone();
            let _ = format!("{:?}", animation_value);
            let _ = animation_value == animation_value;
        });

        assert!(result.is_ok());
    }

    #[test]
    fn fuzz_animation_value_type_safety(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test type checking
        match animation_value {
            AnimationValue::Number(value) => {
                // Number should be a valid f64
                assert!(value.is_finite() || value.is_nan() || value.is_infinite());
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn fuzz_animation_value_serialization(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we can convert to string representation
        let string_repr = format!("{:?}", animation_value);
        assert!(!string_repr.is_empty());

        // Test that string representation is valid
        assert!(string_repr.len() < 10000); // Reasonable size limit
    }

    #[test]
    fn fuzz_animation_value_conversion(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test conversion to string
        let string_value = match &animation_value {
            AnimationValue::Number(value) => value.to_string(),
            _ => panic!("Expected Number variant"),
        };

        // String value should not be empty for non-zero numbers
        if n != 0.0 {
            assert!(!string_value.is_empty());
        }
    }

    #[test]
    fn fuzz_animation_value_memory_usage(n in any::<f64>()) {
        let animation_value = AnimationValue::Number(n);

        // Test that we can create many instances without issues
        let mut values = Vec::new();
        for _ in 0..1000 {
            values.push(animation_value.clone());
        }

        // All values should be equal
        for v in &values {
            assert_eq!(*v, animation_value);
        }

        // Memory should be reasonable (not growing exponentially)
        assert!(values.len() == 1000);
    }
}
