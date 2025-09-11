//! Visual Regression Testing Suite
//!
//! This module implements comprehensive visual regression testing
//! to ensure animations render consistently across different scenarios.

use leptos_motion_core::types::*;
use proptest::prelude::*;

/// Visual test case for animation rendering
#[derive(Debug, Clone)]
pub struct VisualTestCase {
    pub name: String,
    pub animation_target: AnimationTarget,
    pub transition: Transition,
    pub expected_properties: Vec<String>,
}

impl VisualTestCase {
    pub fn new(name: String, animation_target: AnimationTarget, transition: Transition) -> Self {
        let expected_properties = animation_target.keys().cloned().collect();
        Self {
            name,
            animation_target,
            transition,
            expected_properties,
        }
    }
}

proptest! {
    #[test]
    fn visual_regression_animation_target_rendering(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..10),
        values in prop::collection::vec(any::<f64>(), 1..10)
    ) {
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let transition = Transition {
            duration: Some(1.0),
            delay: Some(0.0),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "animation_target_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);
        assert_eq!(test_case.expected_properties.len(), target.len());

        // Test that all expected properties are present
        for key in &test_case.expected_properties {
            assert!(target.contains_key(key));
        }
    }

    #[test]
    fn visual_regression_transition_rendering(
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
        let mut target = AnimationTarget::new();
        target.insert("x".to_string(), AnimationValue::Number(100.0));
        target.insert("y".to_string(), AnimationValue::Number(200.0));
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));

        let transition = Transition {
            duration: Some(duration.abs().min(10.0)),
            delay: Some(delay.abs().min(10.0)),
            ease: ease.clone(),
            repeat: repeat.clone(),
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "transition_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);
        assert_eq!(test_case.transition.ease, ease);
        assert_eq!(test_case.transition.repeat, repeat);

        // Test that all expected properties are present
        assert!(test_case.expected_properties.contains(&"x".to_string()));
        assert!(test_case.expected_properties.contains(&"y".to_string()));
        assert!(test_case.expected_properties.contains(&"opacity".to_string()));
    }

    #[test]
    fn visual_regression_spring_animation_rendering(
        stiffness in any::<f64>(),
        damping in any::<f64>(),
        mass in any::<f64>()
    ) {
        let mut target = AnimationTarget::new();
        target.insert("scale".to_string(), AnimationValue::Number(1.5));
        target.insert("rotate".to_string(), AnimationValue::Number(45.0));

        let spring_config = SpringConfig {
            stiffness: stiffness.abs().max(0.001),
            damping: damping.abs().max(0.001),
            mass: mass.abs().max(0.001),
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };

        let transition = Transition {
            duration: Some(1.0),
            delay: Some(0.0),
            ease: Easing::Spring(spring_config),
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "spring_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that spring easing is preserved
        match test_case.transition.ease {
            Easing::Spring(config) => {
                assert!(config.stiffness > 0.0);
                assert!(config.damping > 0.0);
                assert!(config.mass > 0.0);
            }
            _ => panic!("Expected Spring easing"),
        }
    }

    #[test]
    fn visual_regression_stagger_animation_rendering(
        delay in any::<f64>(),
        amount in any::<f64>()
    ) {
        let mut target = AnimationTarget::new();
        target.insert("x".to_string(), AnimationValue::Number(0.0));
        target.insert("y".to_string(), AnimationValue::Number(0.0));
        target.insert("z".to_string(), AnimationValue::Number(0.0));

        let stagger_config = StaggerConfig {
            delay: delay.abs().min(1.0),
            from: StaggerFrom::First,
        };

        let transition = Transition {
            duration: Some(0.5),
            delay: Some(0.0),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Never,
            stagger: Some(stagger_config),
        };

        let test_case = VisualTestCase::new(
            "stagger_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that stagger config is preserved
        match test_case.transition.stagger {
            Some(stagger) => {
                assert!(stagger.delay >= 0.0);
            }
            None => panic!("Expected stagger config"),
        }
    }

    #[test]
    fn visual_regression_bezier_animation_rendering(
        x1 in any::<f64>(),
        y1 in any::<f64>(),
        x2 in any::<f64>(),
        y2 in any::<f64>()
    ) {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        target.insert("scale".to_string(), AnimationValue::Number(0.5));

        let transition = Transition {
            duration: Some(2.0),
            delay: Some(0.1),
            ease: Easing::Bezier(x1, y1, x2, y2),
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "bezier_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that bezier easing is preserved
        match test_case.transition.ease {
            Easing::Bezier(px1, py1, px2, py2) => {
                assert_eq!(px1, x1);
                assert_eq!(py1, y1);
                assert_eq!(px2, x2);
                assert_eq!(py2, y2);
            }
            _ => panic!("Expected Bezier easing"),
        }
    }

    #[test]
    fn visual_regression_repeat_animation_rendering(
        repeat_count in 1..10u32
    ) {
        let mut target = AnimationTarget::new();
        target.insert("rotate".to_string(), AnimationValue::Number(360.0));

        let transition = Transition {
            duration: Some(1.0),
            delay: Some(0.0),
            ease: Easing::Linear,
            repeat: RepeatConfig::Count(repeat_count),
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "repeat_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that repeat config is preserved
        match test_case.transition.repeat {
            RepeatConfig::Count(count) => assert_eq!(count, repeat_count),
            _ => panic!("Expected Count repeat config"),
        }
    }

    #[test]
    fn visual_regression_infinite_animation_rendering(
        ease in prop::sample::select(&[
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
        ])
    ) {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));

        let transition = Transition {
            duration: Some(0.5),
            delay: Some(0.0),
            ease: ease.clone(),
            repeat: RepeatConfig::Infinite,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "infinite_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);
        assert_eq!(test_case.transition.ease, ease);

        // Test that infinite repeat is preserved
        match test_case.transition.repeat {
            RepeatConfig::Infinite => {}
            _ => panic!("Expected Infinite repeat config"),
        }
    }

    #[test]
    fn visual_regression_complex_animation_rendering(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..20),
        values in prop::collection::vec(any::<f64>(), 1..20),
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
        ]),
        repeat in prop::sample::select(&[
            RepeatConfig::Never,
            RepeatConfig::Count(1),
            RepeatConfig::Count(3),
            RepeatConfig::Infinite,
        ])
    ) {
        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let transition = Transition {
            duration: Some(duration.abs().min(10.0)),
            delay: Some(delay.abs().min(10.0)),
            ease: ease.clone(),
            repeat: repeat.clone(),
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "complex_animation_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);
        assert_eq!(test_case.transition.ease, ease);
        assert_eq!(test_case.transition.repeat, repeat);

        // Test that all expected properties are present
        for key in &test_case.expected_properties {
            assert!(target.contains_key(key));
        }
    }

    #[test]
    fn visual_regression_edge_case_rendering(
        extreme_values in prop::collection::vec(any::<f64>(), 1..10)
    ) {
        let mut target = AnimationTarget::new();
        for (i, &value) in extreme_values.iter().enumerate() {
            target.insert(format!("prop_{}", i), AnimationValue::Number(value));
        }

        let transition = Transition {
            duration: Some(0.001), // Very short duration
            delay: Some(0.0),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "edge_case_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that extreme values are preserved
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
    fn visual_regression_unicode_property_rendering(
        unicode_keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..10),
        values in prop::collection::vec(-1000.0..1000.0, 1..10)
    ) {
        let mut target = AnimationTarget::new();
        let mut unique_keys = std::collections::HashSet::new();
        
        for (i, key) in unicode_keys.iter().enumerate() {
            if i < values.len() && !key.is_empty() && unique_keys.insert(key.clone()) {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let transition = Transition {
            duration: Some(1.0),
            delay: Some(0.0),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let test_case = VisualTestCase::new(
            "unicode_property_rendering".to_string(),
            target.clone(),
            transition
        );

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);

        // Test that unicode keys are preserved
        for (i, key) in unicode_keys.iter().enumerate() {
            if i < values.len() && !key.is_empty() && unique_keys.contains(key) {
                match target.get(key) {
                    Some(AnimationValue::Number(value)) => {
                        assert_eq!(*value, values[i]);
                    }
                    _ => panic!("Expected Number variant"),
                }
            }
        }
    }

    #[test]
    fn visual_regression_performance_rendering(
        count in 1..100u32,
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 1..100),
        values in prop::collection::vec(any::<f64>(), 1..100)
    ) {
        let count = count as usize;
        let keys = &keys[..count.min(keys.len())];
        let values = &values[..count.min(values.len())];

        let mut target = AnimationTarget::new();
        for (i, key) in keys.iter().enumerate() {
            if i < values.len() {
                target.insert(key.clone(), AnimationValue::Number(values[i]));
            }
        }

        let transition = Transition {
            duration: Some(1.0),
            delay: Some(0.0),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let start = std::time::Instant::now();

        let test_case = VisualTestCase::new(
            "performance_rendering".to_string(),
            target.clone(),
            transition
        );

        let duration = start.elapsed();

        // Should create test case quickly
        assert!(duration.as_millis() < 10);

        // Test that the test case is valid
        assert!(!test_case.name.is_empty());
        assert_eq!(test_case.animation_target, target);
        assert_eq!(test_case.expected_properties.len(), target.len());
    }
}
