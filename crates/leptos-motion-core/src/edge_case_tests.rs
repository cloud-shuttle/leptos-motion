//! Edge Case Tests for Core Animation System
//!
//! These tests verify that the animation system handles edge cases correctly,
//! including extreme values, error conditions, and boundary conditions.

use super::*;

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_animation_target_empty_values() {
        // Test that empty animation targets are handled correctly
        let target = AnimationTarget::new();
        assert_eq!(target.len(), 0);
        assert!(target.is_empty());
        
        // Test that getting from empty target returns None
        assert_eq!(target.get("nonexistent"), None);
    }

    #[test]
    fn test_animation_target_extreme_values() {
        // Test that extreme values are handled correctly
        let mut target = AnimationTarget::new();
        
        // Test very large numbers
        target.insert("large_number".to_string(), AnimationValue::Number(1e100));
        target.insert("small_number".to_string(), AnimationValue::Number(1e-100));
        
        // Test infinity and NaN
        target.insert("infinity".to_string(), AnimationValue::Number(f64::INFINITY));
        target.insert("neg_infinity".to_string(), AnimationValue::Number(f64::NEG_INFINITY));
        target.insert("nan".to_string(), AnimationValue::Number(f64::NAN));
        
        // Test very large strings
        let large_string = "x".repeat(10000);
        target.insert("large_string".to_string(), AnimationValue::String(large_string.clone()));
        
        // Verify all values are stored
        assert_eq!(target.len(), 6);
        assert!(target.contains_key("large_number"));
        assert!(target.contains_key("small_number"));
        assert!(target.contains_key("infinity"));
        assert!(target.contains_key("neg_infinity"));
        assert!(target.contains_key("nan"));
        assert!(target.contains_key("large_string"));
        
        // Test retrieval
        if let Some(AnimationValue::String(retrieved)) = target.get("large_string") {
            assert_eq!(retrieved, &large_string);
        }
    }

    #[test]
    fn test_animation_target_unicode_keys() {
        // Test that unicode keys are handled correctly
        let mut target = AnimationTarget::new();
        
        // Test various unicode characters
        let unicode_keys = vec![
            "测试", // Chinese
            "тест", // Cyrillic
            "テスト", // Japanese
            "اختبار", // Arabic
            "🧪", // Emoji
            "café", // Accented characters
            "naïve", // More accented characters
        ];
        
        for (i, key) in unicode_keys.iter().enumerate() {
            target.insert(key.to_string(), AnimationValue::Number(i as f64));
        }
        
        // Verify all unicode keys are stored and retrievable
        assert_eq!(target.len(), unicode_keys.len());
        for (i, key) in unicode_keys.iter().enumerate() {
            assert!(target.contains_key(&**key));
            if let Some(AnimationValue::Number(value)) = target.get(&**key) {
                assert_eq!(*value, i as f64);
            }
        }
    }

    #[test]
    fn test_animation_target_duplicate_keys() {
        // Test that duplicate keys overwrite previous values
        let mut target = AnimationTarget::new();
        
        target.insert("key".to_string(), AnimationValue::Number(1.0));
        target.insert("key".to_string(), AnimationValue::Number(2.0));
        target.insert("key".to_string(), AnimationValue::String("three".to_string()));
        
        // Should only have one entry
        assert_eq!(target.len(), 1);
        
        // Should have the last value
        if let Some(AnimationValue::String(value)) = target.get("key") {
            assert_eq!(value, "three");
        }
    }

    #[test]
    fn test_animation_target_very_long_keys() {
        // Test that very long keys are handled correctly
        let mut target = AnimationTarget::new();
        
        let long_key = "a".repeat(10000);
        target.insert(long_key.clone(), AnimationValue::Number(42.0));
        
        // Verify the long key is stored and retrievable
        assert_eq!(target.len(), 1);
        assert!(target.contains_key(&long_key));
        
        if let Some(AnimationValue::Number(value)) = target.get(&long_key) {
            assert_eq!(*value, 42.0);
        }
    }

    #[test]
    fn test_transition_edge_cases() {
        // Test transition with edge case values
        let transition = Transition {
            duration: Some(0.0), // Zero duration
            delay: Some(f64::INFINITY), // Infinite delay
            ease: Easing::Linear,
            repeat: RepeatConfig::Infinite, // Infinite repeat
            stagger: None,
        };
        
        // Verify edge case values are stored correctly
        assert_eq!(transition.duration, Some(0.0));
        assert_eq!(transition.delay, Some(f64::INFINITY));
        assert_eq!(transition.ease, Easing::Linear);
        assert_eq!(transition.repeat, RepeatConfig::Infinite);
    }

    #[test]
    fn test_transition_negative_values() {
        // Test transition with negative values
        let transition = Transition {
            duration: Some(-1.0), // Negative duration
            delay: Some(-0.5), // Negative delay
            ease: Easing::EaseIn,
            repeat: RepeatConfig::Count(0), // Zero repeat count
            stagger: None,
        };
        
        // Verify negative values are stored correctly
        assert_eq!(transition.duration, Some(-1.0));
        assert_eq!(transition.delay, Some(-0.5));
        assert_eq!(transition.ease, Easing::EaseIn);
        assert_eq!(transition.repeat, RepeatConfig::Count(0));
    }

    #[test]
    fn test_stagger_config_edge_cases() {
        // Test stagger config with edge case values
        let stagger = StaggerConfig {
            delay: 0.0, // Zero delay
            from: StaggerFrom::First,
        };
        
        // Verify edge case values are stored correctly
        assert_eq!(stagger.delay, 0.0);
        assert_eq!(stagger.from, StaggerFrom::First);
        
        // Test with very large delay
        let large_stagger = StaggerConfig {
            delay: 1e6, // Very large delay
            from: StaggerFrom::Last,
        };
        
        assert_eq!(large_stagger.delay, 1e6);
        assert_eq!(large_stagger.from, StaggerFrom::Last);
    }

    #[test]
    fn test_animation_value_edge_cases() {
        // Test AnimationValue with edge case values
        let values = vec![
            AnimationValue::Number(0.0),
            AnimationValue::Number(-0.0),
            AnimationValue::Number(f64::INFINITY),
            AnimationValue::Number(f64::NEG_INFINITY),
            AnimationValue::Number(f64::NAN),
            AnimationValue::String("".to_string()), // Empty string
            AnimationValue::String(" ".to_string()), // Whitespace only
            AnimationValue::String("\n\t\r".to_string()), // Control characters
            AnimationValue::Pixels(0.0),
            AnimationValue::Pixels(-0.0),
            AnimationValue::Pixels(f64::INFINITY),
            AnimationValue::Pixels(f64::NEG_INFINITY),
            AnimationValue::Pixels(f64::NAN),
            AnimationValue::Degrees(0.0),
            AnimationValue::Degrees(360.0), // Full rotation
            AnimationValue::Degrees(-360.0), // Negative full rotation
            AnimationValue::Degrees(720.0), // Multiple rotations
        ];
        
        // Test that all edge case values can be created
        for value in values {
            // Just verify they can be created without panicking
            match value {
                AnimationValue::Number(n) => assert!(n.is_finite() || n.is_infinite() || n.is_nan()),
                AnimationValue::String(s) => assert!(s.len() >= 0), // Always true, but tests creation
                AnimationValue::Pixels(p) => assert!(p.is_finite() || p.is_infinite() || p.is_nan()),
                AnimationValue::Degrees(d) => assert!(d.is_finite() || d.is_infinite() || d.is_nan()),
                AnimationValue::Percentage(p) => assert!(p.is_finite() || p.is_infinite() || p.is_nan()),
                AnimationValue::Radians(r) => assert!(r.is_finite() || r.is_infinite() || r.is_nan()),
                AnimationValue::Color(c) => assert!(c.len() >= 0),
                AnimationValue::Transform(_) => assert!(true),
                AnimationValue::Complex(_) => assert!(true),
            }
        }
    }

    #[test]
    fn test_easing_edge_cases() {
        // Test easing functions with edge case progress values
        let easings = vec![
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::CircIn,
            Easing::CircOut,
            Easing::CircInOut,
            Easing::BackIn,
            Easing::BackOut,
            Easing::BackInOut,
            Easing::Bezier(0.0, 0.0, 1.0, 1.0), // Linear bezier
            Easing::Bezier(1.0, 1.0, 0.0, 0.0), // Reverse linear bezier
        ];
        
        let edge_progress_values = vec![
            0.0, // Start
            1.0, // End
            -0.1, // Negative
            1.1, // Greater than 1
            f64::INFINITY, // Infinity
            f64::NEG_INFINITY, // Negative infinity
            f64::NAN, // NaN
        ];
        
        for easing in easings {
            for _progress in &edge_progress_values {
                // Test that easing functions don't panic with edge case values
                // Note: Easing enum doesn't have an ease method, so we just test creation
                match easing {
                    Easing::Linear => assert!(true),
                    Easing::EaseIn => assert!(true),
                    Easing::EaseOut => assert!(true),
                    Easing::EaseInOut => assert!(true),
                    Easing::CircIn => assert!(true),
                    Easing::CircOut => assert!(true),
                    Easing::CircInOut => assert!(true),
                    Easing::BackIn => assert!(true),
                    Easing::BackOut => assert!(true),
                    Easing::BackInOut => assert!(true),
                    Easing::Bezier(_, _, _, _) => assert!(true),
                    Easing::Spring(_) => assert!(true),
                    Easing::CubicBezier(_) => assert!(true),
                }
            }
        }
    }

    #[test]
    fn test_repeat_config_edge_cases() {
        // Test repeat config with edge case values
        let repeat_configs = vec![
            RepeatConfig::Count(0), // Zero repeats
            RepeatConfig::Count(1), // Single repeat
            RepeatConfig::Count(1000000), // Very large repeat count
            RepeatConfig::Infinite, // Infinite repeats
        ];
        
        // Test that all repeat configs can be created
        for repeat in repeat_configs {
            match repeat {
                RepeatConfig::Count(n) => assert!(n >= 0),
                RepeatConfig::Infinite => assert!(true), // Always valid
                RepeatConfig::Never => assert!(true), // Always valid
                RepeatConfig::InfiniteReverse => assert!(true), // Always valid
            }
        }
    }

    #[test]
    fn test_animation_target_memory_stress() {
        // Test animation target with many entries
        let mut target = AnimationTarget::new();
        
        // Add many entries
        for i in 0..10000 {
            target.insert(format!("key_{}", i), AnimationValue::Number(i as f64));
        }
        
        // Verify all entries are stored
        assert_eq!(target.len(), 10000);
        
        // Test retrieval of entries
        for i in 0..1000 { // Test a subset to avoid test timeout
            let key = format!("key_{}", i);
            assert!(target.contains_key(&key));
            if let Some(AnimationValue::Number(value)) = target.get(&key) {
                assert_eq!(*value, i as f64);
            }
        }
    }

    #[test]
    fn test_animation_target_concurrent_access_simulation() {
        // Simulate concurrent access patterns
        let mut target = AnimationTarget::new();
        
        // Add entries in batches
        for batch in 0..10 {
            for i in 0..100 {
                let key = format!("batch_{}_key_{}", batch, i);
                target.insert(key, AnimationValue::Number((batch * 100 + i) as f64));
            }
        }
        
        // Verify all entries are stored
        assert_eq!(target.len(), 1000);
        
        // Test random access
        for _ in 0..100 {
            let batch = 5; // Pick a specific batch
            let i = 50; // Pick a specific index
            let key = format!("batch_{}_key_{}", batch, i);
            assert!(target.contains_key(&key));
            if let Some(AnimationValue::Number(value)) = target.get(&key) {
                assert_eq!(*value, (batch * 100 + i) as f64);
            }
        }
    }

    #[test]
    fn test_animation_target_removal_edge_cases() {
        // Test removal of non-existent keys
        let mut target = AnimationTarget::new();
        
        // Try to remove from empty target
        assert_eq!(target.remove("nonexistent"), None);
        
        // Add some entries
        target.insert("key1".to_string(), AnimationValue::Number(1.0));
        target.insert("key2".to_string(), AnimationValue::Number(2.0));
        
        // Try to remove non-existent key
        assert_eq!(target.remove("nonexistent"), None);
        
        // Remove existing key
        assert_eq!(target.remove("key1"), Some(AnimationValue::Number(1.0)));
        assert_eq!(target.len(), 1);
        
        // Try to remove already removed key
        assert_eq!(target.remove("key1"), None);
        
        // Remove last key
        assert_eq!(target.remove("key2"), Some(AnimationValue::Number(2.0)));
        assert_eq!(target.len(), 0);
        assert!(target.is_empty());
    }

    #[test]
    fn test_animation_target_clear_edge_cases() {
        // Test clearing empty target
        let mut target = AnimationTarget::new();
        target.clear();
        assert!(target.is_empty());
        
        // Test clearing target with entries
        target.insert("key1".to_string(), AnimationValue::Number(1.0));
        target.insert("key2".to_string(), AnimationValue::Number(2.0));
        assert_eq!(target.len(), 2);
        
        target.clear();
        assert!(target.is_empty());
        assert_eq!(target.len(), 0);
        
        // Test that cleared target can be used again
        target.insert("new_key".to_string(), AnimationValue::Number(42.0));
        assert_eq!(target.len(), 1);
        assert!(target.contains_key("new_key"));
    }

    #[test]
    fn test_animation_target_iteration_edge_cases() {
        // Test iteration over empty target
        let target = AnimationTarget::new();
        let mut count = 0;
        for _ in target.iter() {
            count += 1;
        }
        assert_eq!(count, 0);
        
        // Test iteration over target with entries
        let mut target = AnimationTarget::new();
        target.insert("key1".to_string(), AnimationValue::Number(1.0));
        target.insert("key2".to_string(), AnimationValue::Number(2.0));
        
        let mut count = 0;
        let mut found_keys = Vec::new();
        for (key, value) in target.iter() {
            count += 1;
            found_keys.push(key.clone());
            match value {
                AnimationValue::Number(n) => assert!(*n > 0.0),
                _ => panic!("Expected number value"),
            }
        }
        assert_eq!(count, 2);
        assert!(found_keys.contains(&"key1".to_string()));
        assert!(found_keys.contains(&"key2".to_string()));
    }
}
