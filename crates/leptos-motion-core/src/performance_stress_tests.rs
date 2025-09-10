//! Performance and Stress Testing Suite
//!
//! This module implements comprehensive performance and stress testing
//! to ensure the animation system can handle high loads and edge cases.

use crate::types::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn stress_test_animation_value_creation(
        count in 1..1000u32,
        values in prop::collection::vec(any::<f64>(), 1..1000)
    ) {
        let count = count as usize;
        let values = &values[..count.min(values.len())];

        let start = std::time::Instant::now();

        let mut animation_values = Vec::new();
        for &value in values {
            animation_values.push(AnimationValue::Number(value));
        }

        let duration = start.elapsed();

        // Should create values quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(animation_values.len(), count);

        // Test that all values are correct
        for (i, animation_value) in animation_values.iter().enumerate() {
            match animation_value {
                AnimationValue::Number(n) => {
                    if values[i].is_nan() {
                        assert!(n.is_nan());
                    } else {
                        // Use approximate equality for floating point numbers
                        assert!((*n - values[i]).abs() < f64::EPSILON);
                    }
                }
                _ => panic!("Expected Number variant"),
            }
        }
    }

    #[test]
    fn stress_test_animation_target_operations(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..1000),
        values in prop::collection::vec(any::<f64>(), 1..1000)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        let start = std::time::Instant::now();

        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let duration = start.elapsed();

        // Should insert values quickly
        assert!(duration.as_millis() < 100);
        assert!(target.len() <= count);

        // Test that all values are accessible
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(n)) => {
                        if values[i].is_nan() {
                            assert!(n.is_nan());
                        } else {
                            // Use relative tolerance for floating point comparison
                            let tolerance = values[i].abs() * 1e-10 + f64::EPSILON;
                            assert!((*n - values[i]).abs() <= tolerance);
                        }
                    }
                    _ => panic!("Expected Number variant"),
                }
            }
        }
    }

    #[test]
    fn stress_test_transition_creation(
        count in 1..1000u32,
        durations in prop::collection::vec(any::<f64>(), 1..1000),
        delays in prop::collection::vec(any::<f64>(), 1..1000)
    ) {
        let count = count as usize;
        let durations = &durations[..count.min(durations.len())];
        let delays = &delays[..count.min(delays.len())];

        let start = std::time::Instant::now();

        let mut transitions = Vec::new();
        for i in 0..count {
            let duration = if i < durations.len() { durations[i] } else { 1.0 };
            let delay = if i < delays.len() { delays[i] } else { 0.0 };

            let transition = Transition {
                duration: Some(duration),
                delay: Some(delay),
                ease: Easing::Linear,
                repeat: RepeatConfig::Never,
                stagger: None,
            };
            transitions.push(transition);
        }

        let duration = start.elapsed();

        // Should create transitions quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(transitions.len(), count);

        // Test that all transitions are valid
        for (i, transition) in transitions.iter().enumerate() {
            let expected_duration = if i < durations.len() { durations[i] } else { 1.0 };
            let expected_delay = if i < delays.len() { delays[i] } else { 0.0 };

            assert_eq!(transition.duration, Some(expected_duration));
            assert_eq!(transition.delay, Some(expected_delay));
            assert_eq!(transition.ease, Easing::Linear);
            assert_eq!(transition.repeat, RepeatConfig::Never);
            assert_eq!(transition.stagger, None);
        }
    }

    #[test]
    fn stress_test_cloning_operations(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create a target with many properties
        let mut original_target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                original_target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Clone the target many times
        let mut cloned_targets = Vec::new();
        for _ in 0..count {
            cloned_targets.push(original_target.clone());
        }

        let duration = start.elapsed();

        // Should clone quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(cloned_targets.len(), count);

        // Test that all clones are equal to original
        for cloned_target in &cloned_targets {
            assert_eq!(*cloned_target, original_target);
        }
    }

    #[test]
    fn stress_test_debug_formatting(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create a target with many properties
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Format as debug many times
        let mut debug_strings = Vec::new();
        for _ in 0..count {
            debug_strings.push(format!("{:?}", target));
        }

        let duration = start.elapsed();

        // Should format quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(debug_strings.len(), count);

        // Test that all debug strings are non-empty
        for debug_str in &debug_strings {
            assert!(!debug_str.is_empty());
            assert!(debug_str.contains("AnimationTarget"));
        }
    }

    #[test]
    fn stress_test_equality_comparisons(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create two identical targets
        let mut target1 = AnimationTarget::new();
        let mut target2 = AnimationTarget::new();

        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target1.insert(key.clone(), AnimationValue::Number(values[i]));
                target2.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Compare many times
        let mut results = Vec::new();
        for _ in 0..count {
            results.push(target1 == target2);
        }

        let duration = start.elapsed();

        // Should compare quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(results.len(), count);

        // Test that all comparisons return true
        for result in &results {
            assert!(result);
        }
    }

    #[test]
    fn stress_test_memory_usage(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create a target with many properties
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Create many instances
        let mut targets = Vec::new();
        for _ in 0..count {
            targets.push(target.clone());
        }

        let duration = start.elapsed();

        // Should create instances quickly
        assert!(duration.as_millis() < 100);
        assert_eq!(targets.len(), count);

        // Test that all instances are equal
        for t in &targets {
            assert_eq!(*t, target);
        }

        // Test that we can still access properties
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(n)) => {
                        if values[i].is_nan() {
                            assert!(n.is_nan());
                        } else {
                            // Use relative tolerance for floating point comparison
                            let tolerance = values[i].abs() * 1e-10 + f64::EPSILON;
                            assert!((*n - values[i]).abs() <= tolerance);
                        }
                    }
                    _ => panic!("Expected Number variant"),
                }
            }
        }
    }

    #[test]
    fn stress_test_concurrent_access(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create a target with many properties
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Test concurrent access patterns
        let mut handles = Vec::new();
        for _ in 0..count {
            let target_clone = target.clone();
            let handle = std::thread::spawn(move || {
                let _ = target_clone.clone();
                let _ = format!("{:?}", target_clone);
                let _ = target_clone == target_clone;
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            assert!(handle.join().is_ok());
        }

        let duration = start.elapsed();

        // Should handle concurrent access quickly
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn stress_test_edge_case_handling(
        count in 1..1000u32,
        extreme_values in prop::collection::vec(any::<f64>(), 1..1000)
    ) {
        let count = count as usize;
        let extreme_values = &extreme_values[..count.min(extreme_values.len())];

        let start = std::time::Instant::now();

        let mut target = AnimationTarget::new();
        for (i, &value) in extreme_values.iter().enumerate() {
            target.insert(format!("prop_{}", i), AnimationValue::Number(value));
        }

        let duration = start.elapsed();

        // Should handle extreme values quickly
        assert!(duration.as_millis() < 100);
        assert!(target.len() <= count);

        // Test that all extreme values are preserved
        for (i, &expected_value) in extreme_values.iter().enumerate() {
            let key = format!("prop_{}", i);
            match target.get(&key) {
                Some(AnimationValue::Number(actual_value)) => {
                    if expected_value.is_nan() {
                        assert!(actual_value.is_nan());
                    } else if expected_value.is_infinite() {
                        assert!(actual_value.is_infinite());
                        assert_eq!(expected_value.is_sign_positive(), actual_value.is_sign_positive());
                    } else {
                        assert_eq!(*actual_value, expected_value);
                    }
                }
                _ => panic!("Expected Number variant"),
            }
        }
    }

    #[test]
    fn stress_test_performance_under_load(
        count in 1..1000u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        // Create a target with many properties
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let start = std::time::Instant::now();

        // Perform many operations under load
        let mut total_operations = 0;
        while start.elapsed().as_millis() < 100 {
            let _ = target.clone();
            let _ = format!("{:?}", target);
            let _ = target == target;
            total_operations += 3;
        }

        // Should complete many operations quickly
        assert!(total_operations > 1000);

        // Test that the target is still valid
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(n)) => {
                        if values[i].is_nan() {
                            assert!(n.is_nan());
                        } else {
                            // Use relative tolerance for floating point comparison
                            let tolerance = values[i].abs() * 1e-10 + f64::EPSILON;
                            assert!((*n - values[i]).abs() <= tolerance);
                        }
                    }
                    _ => panic!("Expected Number variant"),
                }
            }
        }
    }
}
