//! Performance Regression Tests
//!
//! These tests ensure that performance characteristics are maintained across
//! code changes and detect performance regressions early in development.
//!
//! The tests establish baseline performance metrics and fail if performance
//! degrades beyond acceptable thresholds.

use super::*;
use std::time::Instant;

#[cfg(test)]
mod performance_regression_tests {
    use super::*;

    /// Performance regression test for AnimationTarget creation
    #[test]
    fn test_animation_target_creation_performance_regression() {
        let iterations = 10_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let mut target = AnimationTarget::new();
            target.insert("x".to_string(), AnimationValue::Pixels(i as f64));
            target.insert("y".to_string(), AnimationValue::Pixels(i as f64 * 0.5));
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0 + i as f64 * 0.001));
            target.insert("rotate".to_string(), AnimationValue::Degrees(i as f64 * 0.1));
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should create at least 50,000 targets per second
        // This is more lenient than the original benchmark to account for CI variability
        assert!(
            operations_per_second > 50_000.0,
            "AnimationTarget creation performance regression: {} ops/sec (expected > 50,000)",
            operations_per_second
        );

        // Should complete 10,000 operations in under 200ms
        assert!(
            duration.as_millis() < 200,
            "AnimationTarget creation too slow: {}ms for {} operations (expected < 200ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for AnimationValue creation
    #[test]
    fn test_animation_value_creation_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let _values = vec![
                AnimationValue::Number(i as f64),
                AnimationValue::Pixels(i as f64 * 0.5),
                AnimationValue::Degrees(i as f64 * 0.1),
                AnimationValue::Percentage(i as f64 * 0.01),
                AnimationValue::Radians(i as f64 * 0.017),
                AnimationValue::String(format!("value_{}", i)),
                AnimationValue::Color(format!("#{:06x}", i % 0xffffff)),
            ];
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should create at least 500,000 values per second
        assert!(
            operations_per_second > 500_000.0,
            "AnimationValue creation performance regression: {} ops/sec (expected > 500,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 200ms
        assert!(
            duration.as_millis() < 200,
            "AnimationValue creation too slow: {}ms for {} operations (expected < 200ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for Transition creation
    #[test]
    fn test_transition_creation_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let _transition = Transition {
                duration: Some(0.3 + i as f64 * 0.001),
                ease: Easing::EaseOut,
                delay: Some(i as f64 * 0.0001),
                repeat: RepeatConfig::Count(1),
                stagger: None,
            };
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should create at least 1,000,000 transitions per second
        assert!(
            operations_per_second > 1_000_000.0,
            "Transition creation performance regression: {} ops/sec (expected > 1,000,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 100ms
        assert!(
            duration.as_millis() < 100,
            "Transition creation too slow: {}ms for {} operations (expected < 100ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for StaggerConfig creation
    #[test]
    fn test_stagger_config_creation_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let _stagger = StaggerConfig {
                delay: 0.1 + i as f64 * 0.0001,
                from: if i % 2 == 0 {
                    StaggerFrom::First
                } else {
                    StaggerFrom::Last
                },
            };
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should create at least 1,000,000 configs per second
        assert!(
            operations_per_second > 1_000_000.0,
            "StaggerConfig creation performance regression: {} ops/sec (expected > 1,000,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 100ms
        assert!(
            duration.as_millis() < 100,
            "StaggerConfig creation too slow: {}ms for {} operations (expected < 100ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for Transform creation
    #[test]
    fn test_transform_creation_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let _transform = Transform {
                x: Some(i as f64),
                y: Some(i as f64 * 0.5),
                z: Some(i as f64 * 0.1),
                scale_x: Some(1.0 + i as f64 * 0.001),
                scale_y: Some(1.0 + i as f64 * 0.001),
                scale: Some(1.0 + i as f64 * 0.001),
                rotate_x: Some(i as f64 * 0.1),
                rotate_y: Some(i as f64 * 0.1),
                rotate_z: Some(i as f64 * 0.1),
                skew_x: Some(i as f64 * 0.01),
                skew_y: Some(i as f64 * 0.01),
            };
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should create at least 500,000 transforms per second
        assert!(
            operations_per_second > 500_000.0,
            "Transform creation performance regression: {} ops/sec (expected > 500,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 200ms
        assert!(
            duration.as_millis() < 200,
            "Transform creation too slow: {}ms for {} operations (expected < 200ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for AnimationTarget operations
    #[test]
    fn test_animation_target_operations_performance_regression() {
        let iterations = 50_000;
        let mut target = AnimationTarget::new();
        
        // Pre-populate target with some data
        for i in 0..100 {
            target.insert(format!("prop_{}", i), AnimationValue::Number(i as f64));
        }

        let start_time = Instant::now();

        for i in 0..iterations {
            // Test various operations
            let key = format!("test_prop_{}", i % 1000);
            target.insert(key.clone(), AnimationValue::Number(i as f64));
            
            // Test retrieval
            let _value = target.get(&key);
            
            // Test contains check
            let _contains = target.contains_key(&key);
            
            // Test removal
            if i % 10 == 0 {
                target.remove(&key);
            }
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 100,000 operations per second
        assert!(
            operations_per_second > 100_000.0,
            "AnimationTarget operations performance regression: {} ops/sec (expected > 100,000)",
            operations_per_second
        );

        // Should complete 50,000 operations in under 250ms
        assert!(
            duration.as_millis() < 250,
            "AnimationTarget operations too slow: {}ms for {} operations (expected < 250ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for Easing function calculations
    #[test]
    fn test_easing_calculations_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let progress = (i as f64) / iterations as f64;
            
            // Test various easing functions
            let _results = vec![
                // Linear
                progress,
                // EaseIn
                progress * progress,
                // EaseOut
                1.0 - (1.0 - progress) * (1.0 - progress),
                // EaseInOut
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                },
                // CircIn
                1.0 - (1.0 - progress * progress).sqrt(),
                // CircOut
                (1.0 - (progress - 1.0) * (progress - 1.0)).sqrt(),
                // BackIn
                2.7 * progress * progress * progress - 1.7 * progress * progress,
                // BackOut
                1.0 + 2.7 * (progress - 1.0) * (progress - 1.0) * (progress - 1.0) + 1.7 * (progress - 1.0) * (progress - 1.0),
            ];
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should calculate at least 1,000,000 easing values per second
        assert!(
            operations_per_second > 1_000_000.0,
            "Easing calculations performance regression: {} ops/sec (expected > 1,000,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 100ms
        assert!(
            duration.as_millis() < 100,
            "Easing calculations too slow: {}ms for {} operations (expected < 100ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for memory allocation patterns
    #[test]
    fn test_memory_allocation_performance_regression() {
        let iterations = 10_000;
        let start_time = Instant::now();

        // Test memory allocation patterns that might occur in real usage
        let mut targets = Vec::with_capacity(iterations);
        
        for i in 0..iterations {
            let mut target = AnimationTarget::new();
            
            // Add multiple properties to simulate real usage
            for j in 0..10 {
                target.insert(
                    format!("prop_{}_{}", i, j),
                    AnimationValue::Number((i * 10 + j) as f64)
                );
            }
            
            targets.push(target);
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should allocate at least 20,000 targets per second
        assert!(
            operations_per_second > 20_000.0,
            "Memory allocation performance regression: {} ops/sec (expected > 20,000)",
            operations_per_second
        );

        // Should complete 10,000 allocations in under 500ms
        assert!(
            duration.as_millis() < 500,
            "Memory allocation too slow: {}ms for {} operations (expected < 500ms)",
            duration.as_millis(),
            iterations
        );

        // Verify all targets were created
        assert_eq!(targets.len(), iterations);
        
        // Test memory cleanup
        drop(targets);
    }

    /// Performance regression test for concurrent access simulation
    #[test]
    fn test_concurrent_access_simulation_performance_regression() {
        let iterations = 5_000;
        let start_time = Instant::now();

        // Simulate concurrent access patterns
        let mut shared_target = AnimationTarget::new();
        
        for i in 0..iterations {
            // Simulate multiple threads accessing the same target
            for thread_id in 0..4 {
                let key = format!("thread_{}_prop_{}", thread_id, i);
                shared_target.insert(key.clone(), AnimationValue::Number((i * 4 + thread_id) as f64));
                
                // Simulate read operations
                let _value = shared_target.get(&key);
                let _contains = shared_target.contains_key(&key);
            }
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 20,000 operations per second
        assert!(
            operations_per_second > 20_000.0,
            "Concurrent access simulation performance regression: {} ops/sec (expected > 20,000)",
            operations_per_second
        );

        // Should complete 5,000 iterations in under 200ms
        assert!(
            duration.as_millis() < 200,
            "Concurrent access simulation too slow: {}ms for {} operations (expected < 200ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for large dataset handling
    #[test]
    fn test_large_dataset_performance_regression() {
        let iterations = 1_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let mut large_target = AnimationTarget::new();
            
            // Create a large dataset
            for j in 0..100 {
                large_target.insert(
                    format!("large_prop_{}_{}", i, j),
                    AnimationValue::Number((i * 100 + j) as f64)
                );
            }
            
            // Test operations on large dataset
            let _size = large_target.len();
            let _is_empty = large_target.is_empty();
            
            // Test iteration
            let mut count = 0;
            for _ in large_target.iter() {
                count += 1;
            }
            assert_eq!(count, 100);
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 1,000 large datasets per second
        assert!(
            operations_per_second > 1_000.0,
            "Large dataset handling performance regression: {} ops/sec (expected > 1,000)",
            operations_per_second
        );

        // Should complete 1,000 iterations in under 1 second
        assert!(
            duration.as_millis() < 1_000,
            "Large dataset handling too slow: {}ms for {} operations (expected < 1000ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for string operations
    #[test]
    fn test_string_operations_performance_regression() {
        let iterations = 50_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            // Test string creation and manipulation
            let key = format!("string_prop_{}", i);
            let value = AnimationValue::String(format!("value_{}", i));
            
            // Test string operations
            let _key_len = key.len();
            let _key_contains = key.contains("prop");
            
            // Test string conversion
            let _value_str = match value {
                AnimationValue::String(s) => s,
                _ => String::new(),
            };
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 200,000 string operations per second
        assert!(
            operations_per_second > 200_000.0,
            "String operations performance regression: {} ops/sec (expected > 200,000)",
            operations_per_second
        );

        // Should complete 50,000 operations in under 250ms
        assert!(
            duration.as_millis() < 250,
            "String operations too slow: {}ms for {} operations (expected < 250ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for error handling overhead
    #[test]
    fn test_error_handling_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            // Test error creation and handling
            let error = AnimationError::InvalidValue(format!("invalid_{}", i));
            
            // Test error operations
            let _error_str = format!("{:?}", error);
            let _is_invalid_value = matches!(error, AnimationError::InvalidValue(_));
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 500,000 error operations per second
        assert!(
            operations_per_second > 500_000.0,
            "Error handling performance regression: {} ops/sec (expected > 500,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 200ms
        assert!(
            duration.as_millis() < 200,
            "Error handling too slow: {}ms for {} operations (expected < 200ms)",
            duration.as_millis(),
            iterations
        );
    }

    /// Performance regression test for basic operations simulation
    #[test]
    fn test_basic_operations_simulation_performance_regression() {
        let iterations = 100_000;
        let start_time = Instant::now();

        for i in 0..iterations {
            // Simulate basic operations that might occur in real usage
            let _value = i as f64;
            let _squared = _value * _value;
            let _sqrt = _squared.sqrt();
            let _sin = _value.sin();
            let _cos = _value.cos();
        }

        let duration = start_time.elapsed();
        let operations_per_second = iterations as f64 / duration.as_secs_f64();

        // Performance regression threshold: should handle at least 1,000,000 basic operations per second
        assert!(
            operations_per_second > 1_000_000.0,
            "Basic operations simulation performance regression: {} ops/sec (expected > 1,000,000)",
            operations_per_second
        );

        // Should complete 100,000 operations in under 100ms
        assert!(
            duration.as_millis() < 100,
            "Basic operations simulation too slow: {}ms for {} operations (expected < 100ms)",
            duration.as_millis(),
            iterations
        );
    }
}
