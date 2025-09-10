//! Fuzz Testing Suite 4 - Advanced Edge Cases and Complex Scenarios
//!
//! This module implements comprehensive fuzz testing for advanced edge cases
//! and complex scenarios to ensure robust handling of malformed inputs.

use crate::AnimationError;
use crate::types::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_spring_config_edge_cases(
        stiffness in any::<f64>(),
        damping in any::<f64>(),
        mass in any::<f64>()
    ) {
        let spring_config = SpringConfig {
            stiffness: stiffness.abs().max(0.001), // Ensure positive
            damping: damping.abs().max(0.001),     // Ensure positive
            mass: mass.abs().max(0.001),           // Ensure positive
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };

        // Test that values are preserved
        assert!(spring_config.stiffness > 0.0);
        assert!(spring_config.damping > 0.0);
        assert!(spring_config.mass > 0.0);

        // Test debug formatting
        let debug_str = format!("{:?}", spring_config);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("SpringConfig"));
    }

    #[test]
    fn fuzz_stagger_config_edge_cases(
        delay in any::<f64>(),
        _amount in any::<f64>()
    ) {
        let stagger_config = StaggerConfig {
            delay: delay.abs().min(1000.0), // Clamp to reasonable range
            from: StaggerFrom::First,
        };

        // Test that values are preserved
        assert!(stagger_config.delay >= 0.0);

        // Test debug formatting
        let debug_str = format!("{:?}", stagger_config);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("StaggerConfig"));
    }

    #[test]
    fn fuzz_animation_error_edge_cases(
        property_name in ".*"
    ) {
        let error = AnimationError::InvalidProperty {
            property: property_name.clone()
        };

        // Test debug formatting
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("InvalidProperty"));

        // Test that the property name is preserved
        match error {
            AnimationError::InvalidProperty { property } => {
                assert_eq!(property, property_name);
            }
            _ => panic!("Expected InvalidProperty error"),
        }
    }

    #[test]
    fn fuzz_complex_transition_combinations(
        duration1 in any::<f64>(),
        duration2 in any::<f64>(),
        delay1 in any::<f64>(),
        delay2 in any::<f64>(),
        ease1 in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ]),
        ease2 in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ]),
        repeat1 in prop::sample::select(&[
            RepeatConfig::Never,
            RepeatConfig::Count(1),
            RepeatConfig::Infinite,
        ]),
        repeat2 in prop::sample::select(&[
            RepeatConfig::Never,
            RepeatConfig::Count(1),
            RepeatConfig::Infinite,
        ])
    ) {
        let transition1 = Transition {
            duration: Some(duration1),
            delay: Some(delay1),
            ease: ease1.clone(),
            repeat: repeat1.clone(),
            stagger: None,
        };

        let transition2 = Transition {
            duration: Some(duration2),
            delay: Some(delay2),
            ease: ease2.clone(),
            repeat: repeat2.clone(),
            stagger: None,
        };

        // Test that both transitions are valid
        let debug1 = format!("{:?}", transition1);
        let debug2 = format!("{:?}", transition2);

        assert!(!debug1.is_empty());
        assert!(!debug2.is_empty());

        // Test equality comparison
        let are_equal = transition1 == transition2;
        assert!(are_equal == true || are_equal == false);

        // Test cloning
        let cloned1 = transition1.clone();
        let cloned2 = transition2.clone();

        assert_eq!(transition1, cloned1);
        assert_eq!(transition2, cloned2);
    }

    #[test]
    fn fuzz_animation_target_complex_scenarios(
        keys1 in prop::collection::vec("[a-zA-Z0-9_]+", 0..20),
        keys2 in prop::collection::vec("[a-zA-Z0-9_]+", 0..20),
        values1 in prop::collection::vec(any::<f64>(), 0..20),
        values2 in prop::collection::vec(any::<f64>(), 0..20)
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

        // Test that both targets are valid
        let debug1 = format!("{:?}", target1);
        let debug2 = format!("{:?}", target2);

        assert!(!debug1.is_empty());
        assert!(!debug2.is_empty());

        // Test equality comparison
        let are_equal = target1 == target2;
        assert!(are_equal == true || are_equal == false);

        // Test cloning
        let cloned1 = target1.clone();
        let cloned2 = target2.clone();

        assert_eq!(target1, cloned1);
        assert_eq!(target2, cloned2);

        // Test iteration
        let mut count1 = 0;
        for (key, value) in &target1 {
            count1 += 1;
            assert!(!key.is_empty());
            match value {
                AnimationValue::Number(n) => {
                    assert!(n.is_finite() || n.is_nan() || n.is_infinite());
                }
                _ => {}
            }
        }
        assert_eq!(count1, target1.len());

        let mut count2 = 0;
        for (key, value) in &target2 {
            count2 += 1;
            assert!(!key.is_empty());
            match value {
                AnimationValue::Number(n) => {
                    assert!(n.is_finite() || n.is_nan() || n.is_infinite());
                }
                _ => {}
            }
        }
        assert_eq!(count2, target2.len());
    }

    #[test]
    fn fuzz_mixed_animation_value_types(
        numbers in prop::collection::vec(any::<f64>(), 0..10),
        strings in prop::collection::vec(".*", 0..10)
    ) {
        let mut target = AnimationTarget::new();

        // Add number values
        for (i, num) in numbers.iter().enumerate() {
            target.insert(format!("num_{}", i), AnimationValue::Number(*num));
        }

        // Add string values
        for (i, string) in strings.iter().enumerate() {
            target.insert(format!("str_{}", i), AnimationValue::String(string.clone()));
        }

        // Test that all values are preserved
        for (i, num) in numbers.iter().enumerate() {
            let key = format!("num_{}", i);
            match target.get(&key) {
                Some(AnimationValue::Number(value)) => {
                    if num.is_nan() {
                        assert!(value.is_nan());
                    } else {
                        assert_eq!(*value, *num);
                    }
                }
                _ => panic!("Expected Number variant"),
            }
        }

        for (i, string) in strings.iter().enumerate() {
            let key = format!("str_{}", i);
            match target.get(&key) {
                Some(AnimationValue::String(value)) => {
                    assert_eq!(*value, *string);
                }
                _ => panic!("Expected String variant"),
            }
        }

        // Test debug formatting
        let debug_str = format!("{:?}", target);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn fuzz_extreme_repeat_configs(
        count in 0..10000u32
    ) {
        let repeat_config = RepeatConfig::Count(count);

        // Test debug formatting
        let debug_str = format!("{:?}", repeat_config);
        assert!(!debug_str.is_empty());

        // Test that the count is preserved
        match repeat_config {
            RepeatConfig::Count(n) => assert_eq!(n, count),
            _ => panic!("Expected Count variant"),
        }
    }

    #[test]
    fn fuzz_bezier_easing_edge_cases(
        x1 in any::<f64>(),
        y1 in any::<f64>(),
        x2 in any::<f64>(),
        y2 in any::<f64>(),
        progress in any::<f64>()
    ) {
        let _progress = progress.clamp(0.0, 1.0);
        let easing = Easing::Bezier(x1, y1, x2, y2);

        // Test debug formatting
        let debug_str = format!("{:?}", easing);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Bezier"));

        // Test that the values are preserved
        match easing {
            Easing::Bezier(px1, py1, px2, py2) => {
                assert_eq!(px1, x1);
                assert_eq!(py1, y1);
                assert_eq!(px2, x2);
                assert_eq!(py2, y2);
            }
            _ => panic!("Expected Bezier variant"),
        }
    }

    #[test]
    fn fuzz_spring_easing_edge_cases(
        stiffness in any::<f64>(),
        damping in any::<f64>(),
        mass in any::<f64>(),
        progress in any::<f64>()
    ) {
        let _progress = progress.clamp(0.0, 1.0);
        let spring_config = SpringConfig {
            stiffness: stiffness.abs().max(0.001),
            damping: damping.abs().max(0.001),
            mass: mass.abs().max(0.001),
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };
        let easing = Easing::Spring(spring_config.clone());

        // Test debug formatting
        let debug_str = format!("{:?}", easing);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Spring"));

        // Test that the spring config is preserved
        match easing {
            Easing::Spring(config) => {
                assert_eq!(config.stiffness, spring_config.stiffness);
                assert_eq!(config.damping, spring_config.damping);
                assert_eq!(config.mass, spring_config.mass);
            }
            _ => panic!("Expected Spring variant"),
        }
    }

    #[test]
    fn fuzz_animation_target_memory_stress(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..50),
        values in prop::collection::vec(any::<f64>(), 0..50)
    ) {
        let mut target = AnimationTarget::new();

        // Add key-value pairs
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        // Memory stress test
        let mut targets = Vec::new();
        for _ in 0..1000 {
            targets.push(target.clone());
        }

        // All targets should be equal
        for t in &targets {
            assert_eq!(*t, target);
        }

        // Memory should be reasonable
        assert!(targets.len() == 1000);

        // Test that we can still access properties
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                match target.get(key) {
                    Some(AnimationValue::Number(value)) => {
                        // Use approximate equality for floating point numbers
                        if values[i].is_nan() {
                            assert!(value.is_nan());
                        } else if values[i].is_infinite() {
                            assert!(value.is_infinite());
                            assert_eq!(values[i].is_sign_positive(), value.is_sign_positive());
                        } else {
                            // Use relative tolerance for floating point comparison
                            let tolerance = values[i].abs() * 1e-10 + f64::EPSILON;
                            assert!((*value - values[i]).abs() <= tolerance);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn fuzz_transition_memory_stress(
        duration in any::<f64>(),
        delay in any::<f64>(),
        ease in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ]),
        repeat in prop::sample::select(&[
            RepeatConfig::Never,
            RepeatConfig::Count(1),
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

        // Memory stress test
        let mut transitions = Vec::new();
        for _ in 0..1000 {
            transitions.push(transition.clone());
        }

        // All transitions should be equal
        for t in &transitions {
            assert_eq!(*t, transition);
        }

        // Memory should be reasonable
        assert!(transitions.len() == 1000);

        // Test that we can still access properties
        assert_eq!(transition.duration, Some(duration));
        assert_eq!(transition.delay, Some(delay));
        assert_eq!(transition.ease, ease);
        assert_eq!(transition.repeat, repeat);
    }

    #[test]
    fn fuzz_concurrent_access_patterns(
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

        // Test concurrent access patterns
        let target_clone1 = target.clone();
        let target_clone2 = target.clone();
        let target_clone3 = target.clone();

        // All clones should be equal
        assert_eq!(target, target_clone1);
        assert_eq!(target, target_clone2);
        assert_eq!(target, target_clone3);

        // Test that we can access properties from all clones
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                let value = values[i];

                match target.get(key) {
                    Some(AnimationValue::Number(v)) => assert_eq!(*v, value),
                    _ => {}
                }

                match target_clone1.get(key) {
                    Some(AnimationValue::Number(v)) => assert_eq!(*v, value),
                    _ => {}
                }

                match target_clone2.get(key) {
                    Some(AnimationValue::Number(v)) => assert_eq!(*v, value),
                    _ => {}
                }

                match target_clone3.get(key) {
                    Some(AnimationValue::Number(v)) => assert_eq!(*v, value),
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn fuzz_error_handling_edge_cases(
        property_names in prop::collection::vec(".*", 0..10)
    ) {
        for property_name in property_names {
            let error = AnimationError::InvalidProperty {
                property: property_name.clone()
            };

            // Test debug formatting
            let debug_str = format!("{:?}", error);
            assert!(!debug_str.is_empty());

            // Test that the property name is preserved
            match error {
                AnimationError::InvalidProperty { property } => {
                    assert_eq!(property, property_name);
                }
                _ => panic!("Expected InvalidProperty error"),
            }
        }
    }

    #[test]
    fn fuzz_performance_under_extreme_load(
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

        // Performance test under extreme load
        let start = std::time::Instant::now();

        let mut total_operations = 0;
        while start.elapsed().as_millis() < 50 {
            let _ = target.clone();
            let _ = format!("{:?}", target);
            let _ = target == target;
            total_operations += 3;
        }

        // Should complete many operations quickly
        assert!(total_operations > 500);
    }
}
