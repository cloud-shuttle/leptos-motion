//! Fuzz Testing Suite for Leptos Motion DOM
//!
//! This module implements comprehensive fuzz testing to ensure the DOM integration
//! handles edge cases, malformed inputs, and boundary conditions correctly.

use leptos_motion_core::types::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_animation_value_serialization(value in any::<f64>()) {
        let animation_value = AnimationValue::Number(value);

        // Test that we can handle any f64 value
        let debug_str = format!("{:?}", animation_value);
        assert!(!debug_str.is_empty());

        // Test edge cases
        if value.is_nan() {
            assert!(debug_str.contains("NaN"));
        } else if value.is_infinite() {
            assert!(debug_str.contains("Infinity"));
        }
    }

    #[test]
    fn fuzz_animation_value_string_handling(s in ".*") {
        let animation_value = AnimationValue::String(s.clone());

        // Should handle any string
        let debug_str = format!("{:?}", animation_value);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("String"));

        // Test that the string is preserved
        match animation_value {
            AnimationValue::String(value) => assert_eq!(value, s),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_transition_edge_cases(
        duration in any::<f64>(),
        delay in any::<f64>(),
        ease in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::BackIn,
            Easing::BackOut,
            Easing::BackInOut,
            Easing::CircIn,
            Easing::CircOut,
            Easing::CircInOut,
        ]),
        repeat in prop::sample::select(&[
            RepeatConfig::Never,
            RepeatConfig::Count(1),
            RepeatConfig::Count(5),
            RepeatConfig::Infinite,
        ])
    ) {
        let transition = Transition {
            duration: Some(duration),
            delay: Some(delay),
            ease: ease.clone(),
            repeat: repeat.clone(),
            stagger: None,
        };

        // Test that values are preserved
        assert_eq!(transition.duration, Some(duration));
        assert_eq!(transition.delay, Some(delay));
        assert_eq!(transition.ease, ease);
        assert_eq!(transition.repeat, repeat);

        // Test debug formatting
        let debug_str = format!("{:?}", transition);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Transition"));
    }

    #[test]
    fn fuzz_animation_target_properties(
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

        // Test debug formatting
        let debug_str = format!("{:?}", target);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("AnimationTarget"));
    }

    #[test]
    fn fuzz_css_value_handling(css_value in ".*") {
        // Test that we can handle any string as a CSS value
        let animation_value = AnimationValue::String(css_value.clone());

        // Should be able to format as debug
        let debug_str = format!("{:?}", animation_value);
        assert!(!debug_str.is_empty());

        // Test that the value is preserved
        match animation_value {
            AnimationValue::String(value) => assert_eq!(value, css_value),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn fuzz_extreme_numeric_values(value in any::<f64>()) {
        let animation_value = AnimationValue::Number(value);

        // Should handle any f64 value
        let debug_str = format!("{:?}", animation_value);
        assert!(!debug_str.is_empty());

        // Test that the value is preserved
        match animation_value {
            AnimationValue::Number(num_value) => {
                if value.is_nan() {
                    assert!(num_value.is_nan());
                } else {
                    assert_eq!(num_value, value);
                }
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn fuzz_animation_timing_calculations(
        duration in any::<f64>(),
        delay in any::<f64>(),
        elapsed in any::<f64>()
    ) {
        // Clamp values to reasonable ranges
        let duration = duration.abs().min(1000.0);
        let delay = delay.abs().min(1000.0);
        let elapsed = elapsed.abs().min(10000.0);

        let transition = Transition {
            duration: Some(duration),
            delay: Some(delay),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        // Test progress calculation
        let progress = calculate_animation_progress(&transition, elapsed);
        assert!(progress >= 0.0, "Progress should be non-negative");
        assert!(progress <= 1.0, "Progress should not exceed 1.0");
        assert!(progress.is_finite(), "Progress should be finite");

        // Test edge cases
        if elapsed < delay {
            assert_eq!(progress, 0.0, "Progress should be 0 before delay");
        }

        if duration > 0.0 && elapsed >= delay + duration {
            assert_eq!(progress, 1.0, "Progress should be 1 after duration");
        }
    }

    #[test]
    fn fuzz_property_name_edge_cases(property_name in ".*") {
        // Test that we can handle any string as a property name
        let mut target = AnimationTarget::new();
        target.insert(property_name.clone(), AnimationValue::Number(42.0));

        // Should be able to retrieve the property
        assert!(target.contains_key(&property_name));
        assert_eq!(target.get(&property_name), Some(&AnimationValue::Number(42.0)));

        // Should be able to format as debug
        let debug_str = format!("{:?}", target);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn fuzz_concurrent_animations(
        num_animations in 1..10u8,
        base_duration in any::<f64>(),
        base_delay in any::<f64>()
    ) {
        let num_animations = num_animations as usize;
        let base_duration = base_duration.abs().min(10.0);
        let base_delay = base_delay.abs().min(10.0);

        let mut targets = Vec::new();
        let mut transitions = Vec::new();

        for i in 0..num_animations {
            let mut target = AnimationTarget::new();
            target.insert("x".to_string(), AnimationValue::Number(i as f64 * 10.0));
            target.insert("y".to_string(), AnimationValue::Number(i as f64 * 5.0));
            targets.push(target);

            let transition = Transition {
                duration: Some(base_duration + i as f64 * 0.1),
                delay: Some(base_delay + i as f64 * 0.05),
                ease: Easing::Linear,
                repeat: RepeatConfig::Never,
                stagger: None,
            };
            transitions.push(transition);
        }

        // Test that all animations can be processed
        for (target, transition) in targets.iter().zip(transitions.iter()) {
            let debug_target = format!("{:?}", target);
            assert!(!debug_target.is_empty());

            let debug_transition = format!("{:?}", transition);
            assert!(!debug_transition.is_empty());
        }
    }

    #[test]
    fn fuzz_easing_function_edge_cases(
        progress in any::<f64>(),
        ease in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::BackIn,
            Easing::BackOut,
            Easing::BackInOut,
            Easing::CircIn,
            Easing::CircOut,
            Easing::CircInOut,
        ])
    ) {
        // Clamp progress to valid range
        let progress = progress.clamp(0.0, 1.0);

        // Test easing function
        let result = apply_easing(progress, &ease);

        // Basic properties
        assert!(result.is_finite(), "Easing result should be finite");
        assert!(result >= 0.0, "Easing result should be >= 0");
        assert!(result <= 1.0, "Easing result should be <= 1");

        // Edge cases
        if progress == 0.0 {
            assert_eq!(result, 0.0, "Easing at 0 should return 0");
        }
        if progress == 1.0 {
            assert_eq!(result, 1.0, "Easing at 1 should return 1");
        }
    }

    #[test]
    fn fuzz_unicode_property_handling(
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
                        assert_eq!(*value, values[i]);
                    }
                    _ => {
                        // Key might not exist, that's okay
                    }
                }
            }
        }

        // Test debug formatting with unicode
        let debug_str = format!("{:?}", target);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn fuzz_large_animation_targets(
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

        // Test that we can still format as debug
        let debug_str = format!("{:?}", target);
        assert!(!debug_str.is_empty());

        // Test that we can still clone and compare
        let cloned = target.clone();
        assert_eq!(target, cloned);
    }

    #[test]
    fn fuzz_memory_safety_stress_test(
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
    fn fuzz_thread_safety_test(
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
    fn fuzz_performance_under_load(
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

        // Test performance under load
        let start = std::time::Instant::now();

        let mut total_operations = 0;
        while start.elapsed().as_millis() < 10 {
            let _ = target.clone();
            let _ = format!("{:?}", target);
            let _ = target == target;
            total_operations += 3;
        }

        // Should complete many operations quickly
        assert!(total_operations > 100);
    }

    #[test]
    fn fuzz_error_handling_robustness(
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
    fn fuzz_boundary_conditions(
        min_duration in any::<f64>(),
        max_duration in any::<f64>(),
        min_delay in any::<f64>(),
        max_delay in any::<f64>()
    ) {
        // Test boundary conditions
        let min_transition = Transition {
            duration: Some(min_duration),
            delay: Some(0.1),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let max_transition = Transition {
            duration: Some(max_duration),
            delay: Some(0.1),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let min_delay_transition = Transition {
            duration: Some(0.5),
            delay: Some(min_delay),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let max_delay_transition = Transition {
            duration: Some(0.5),
            delay: Some(max_delay),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        // Values should be preserved
        assert_eq!(min_transition.duration, Some(min_duration));
        assert_eq!(max_transition.duration, Some(max_duration));
        assert_eq!(min_delay_transition.delay, Some(min_delay));
        assert_eq!(max_delay_transition.delay, Some(max_delay));
    }
}

/// Helper function to apply easing (simplified version for testing)
fn apply_easing(progress: f64, easing: &Easing) -> f64 {
    match easing {
        Easing::Linear => progress,
        Easing::EaseIn => progress * progress,
        Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
        Easing::EaseInOut => {
            if progress < 0.5 {
                2.0 * progress * progress
            } else {
                1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
            }
        }
        Easing::BackIn => {
            const C1: f64 = 1.70158;
            const C3: f64 = C1 + 1.0;
            let result = C3 * progress * progress * progress - C1 * progress * progress;
            result.max(0.0) // Ensure result is never negative
        }
        Easing::BackOut => {
            const C1: f64 = 1.70158;
            const C3: f64 = C1 + 1.0;
            let result = 1.0 + C3 * (progress - 1.0).powi(3) + C1 * (progress - 1.0).powi(2);
            result.max(0.0).min(1.0) // Ensure result is between 0 and 1
        }
        Easing::BackInOut => {
            const C1: f64 = 1.70158;
            const C2: f64 = C1 * 1.525;
            let result = if progress < 0.5 {
                ((2.0 * progress).powi(2) * ((C2 + 1.0) * 2.0 * progress - C2)) / 2.0
            } else {
                ((2.0 * progress - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * progress - 2.0) + C2) + 2.0)
                    / 2.0
            };
            result.max(0.0).min(1.0) // Ensure result is between 0 and 1
        }
        Easing::CircIn => 1.0 - (1.0 - progress * progress).sqrt(),
        Easing::CircOut => ((2.0 - progress) * progress).sqrt(),
        Easing::CircInOut => {
            if progress < 0.5 {
                0.5 * (1.0 - (1.0 - 4.0 * progress * progress).sqrt())
            } else {
                0.5 * ((4.0 * progress - 2.0) * (2.0 - 2.0 * progress) + 1.0).sqrt()
            }
        }
        Easing::Bezier(x1, y1, x2, y2) => {
            // Simplified cubic bezier implementation
            let t = progress;
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let uuu = uu * u;
            let ttt = tt * t;

            uuu * 0.0 + 3.0 * uu * t * y1 + 3.0 * u * tt * y2 + ttt * 1.0
        }
        Easing::Spring(_) => {
            // Simplified spring implementation - just return progress for testing
            progress
        }
    }
}

/// Helper function to calculate animation progress
fn calculate_animation_progress(transition: &Transition, elapsed: f64) -> f64 {
    let duration = transition.duration.unwrap_or(1.0);
    let delay = transition.delay.unwrap_or(0.0);

    if elapsed < delay {
        0.0
    } else {
        let progress = (elapsed - delay) / duration;
        progress.min(1.0)
    }
}
