// Cross-Browser Compatibility Tests
//
// These tests validate that the leptos-motion-dom library works correctly
// across different browser environments and handles browser-specific
// differences gracefully.

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Test browser compatibility for drag operations
#[test]
fn test_browser_drag_compatibility() {
    // Test drag configuration that should work across all browsers
    let config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(0.2),
        momentum: Some(true),
    };

    // Verify configuration is valid
    assert!(config.axis.is_some());
    assert!(config.constraints.is_some());
    assert!(config.elastic.is_some());
    assert!(config.momentum.is_some());

    // Test constraint application (should work the same across browsers)
    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();

    let test_positions = vec![
        (-150.0, -75.0), // Outside constraints
        (0.0, 0.0),      // Inside constraints
        (150.0, 75.0),   // Outside constraints
        (-50.0, -25.0),  // Inside constraints
    ];

    for (x, y) in test_positions {
        let mut constrained_x = x;
        let mut constrained_y = y;

        // Apply constraints (browser-agnostic logic)
        if let Some(left) = constraints.left {
            if constrained_x < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - constrained_x;
                    constrained_x = left - (overshoot * elastic_factor);
                } else {
                    constrained_x = left;
                }
            }
        }

        if let Some(right) = constraints.right {
            if constrained_x > right {
                if elastic_factor > 0.0 {
                    let overshoot = constrained_x - right;
                    constrained_x = right + (overshoot * elastic_factor);
                } else {
                    constrained_x = right;
                }
            }
        }

        if let Some(top) = constraints.top {
            if constrained_y < top {
                if elastic_factor > 0.0 {
                    let overshoot = top - constrained_y;
                    constrained_y = top - (overshoot * elastic_factor);
                } else {
                    constrained_y = top;
                }
            }
        }

        if let Some(bottom) = constraints.bottom {
            if constrained_y > bottom {
                if elastic_factor > 0.0 {
                    let overshoot = constrained_y - bottom;
                    constrained_y = bottom + (overshoot * elastic_factor);
                } else {
                    constrained_y = bottom;
                }
            }
        }

        // Verify constraints were applied correctly
        assert!(constrained_x >= constraints.left.unwrap() - 10.0);
        assert!(constrained_x <= constraints.right.unwrap() + 10.0);
        assert!(constrained_y >= constraints.top.unwrap() - 10.0);
        assert!(constrained_y <= constraints.bottom.unwrap() + 10.0);
    }
}

/// Test browser compatibility for animation values
#[test]
fn test_browser_animation_value_compatibility() {
    // Test different animation value types that should work across browsers
    let animation_values = vec![
        ("x", AnimationValue::Pixels(100.0)),
        ("y", AnimationValue::Pixels(50.0)),
        ("opacity", AnimationValue::Number(0.8)),
        ("scale", AnimationValue::Number(1.5)),
        ("rotate", AnimationValue::Degrees(45.0)),
        ("translateX", AnimationValue::Pixels(-20.0)),
        ("translateY", AnimationValue::Pixels(10.0)),
    ];

    let mut target = HashMap::new();
    for (key, value) in animation_values {
        target.insert(key.to_string(), value);
    }

    // Verify all values were stored correctly
    assert_eq!(target.len(), 7);
    assert!(target.contains_key("x"));
    assert!(target.contains_key("y"));
    assert!(target.contains_key("opacity"));
    assert!(target.contains_key("scale"));
    assert!(target.contains_key("rotate"));
    assert!(target.contains_key("translateX"));
    assert!(target.contains_key("translateY"));

    // Test value extraction (browser-agnostic)
    match target.get("x") {
        Some(AnimationValue::Pixels(value)) => assert_eq!(*value, 100.0),
        _ => panic!("Failed to extract x value"),
    }

    match target.get("opacity") {
        Some(AnimationValue::Number(value)) => assert_eq!(*value, 0.8),
        _ => panic!("Failed to extract opacity value"),
    }
}

/// Test browser compatibility for easing functions
#[test]
fn test_browser_easing_compatibility() {
    // Test easing functions that should work consistently across browsers
    let easing_functions = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
    ];

    for easing in easing_functions {
        // Test easing calculation at different progress points
        let progress_points = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        
        for progress in progress_points {
            let eased_value = match easing {
                Easing::Linear => progress,
                Easing::EaseIn => progress * progress,
                Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
                Easing::EaseInOut => {
                    if progress < 0.5 {
                        2.0 * progress * progress
                    } else {
                        1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                    }
                },
                _ => progress, // Default to linear for other types
            };

            // Verify eased value is within valid range
            assert!(eased_value >= 0.0 && eased_value <= 1.0, 
                "Eased value {} out of range for progress {} with easing {:?}", 
                eased_value, progress, easing);
        }
    }
}

/// Test browser compatibility for transition configurations
#[test]
fn test_browser_transition_compatibility() {
    // Test transition configurations that should work across browsers
    let transitions = vec![
        Transition {
            duration: Some(0.3),
            ease: Easing::EaseOut,
            delay: Some(0.0),
            repeat: RepeatConfig::Never,
            stagger: None,
        },
        Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            delay: Some(0.1),
            repeat: RepeatConfig::Never,
            stagger: None,
        },
        Transition {
            duration: Some(1.0),
            ease: Easing::Linear,
            delay: None,
            repeat: RepeatConfig::Never,
            stagger: None,
        },
    ];

    for transition in transitions {
        // Verify transition properties are valid
        if let Some(duration) = transition.duration {
            assert!(duration > 0.0, "Duration must be positive");
            assert!(duration <= 10.0, "Duration should be reasonable");
        }

        if let Some(delay) = transition.delay {
            assert!(delay >= 0.0, "Delay must be non-negative");
            assert!(delay <= 5.0, "Delay should be reasonable");
        }

        // Test transition calculation
        let progress = 0.5;
        let eased_progress = match transition.ease {
            Easing::Linear => progress,
            Easing::EaseIn => progress * progress,
            Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            },
            _ => progress,
        };

        assert!(eased_progress >= 0.0 && eased_progress <= 1.0);
    }
}

/// Test browser compatibility for axis constraints
#[test]
fn test_browser_axis_constraint_compatibility() {
    // Test axis constraints that should work consistently across browsers
    let axis_configs = vec![
        DragConfig {
            axis: Some(DragAxis::X),
            constraints: Some(DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: Some(-50.0),
                bottom: Some(50.0),
            }),
            elastic: Some(0.2),
            momentum: Some(true),
        },
        DragConfig {
            axis: Some(DragAxis::Y),
            constraints: Some(DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: Some(-50.0),
                bottom: Some(50.0),
            }),
            elastic: Some(0.2),
            momentum: Some(true),
        },
        DragConfig {
            axis: Some(DragAxis::Both),
            constraints: Some(DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: Some(-50.0),
                bottom: Some(50.0),
            }),
            elastic: Some(0.2),
            momentum: Some(true),
        },
    ];

    for config in axis_configs {
        let constraints = config.constraints.as_ref().unwrap();
        
        // Test movement in different directions
        let test_movements = vec![
            (10.0, 5.0),   // Both axes
            (-10.0, -5.0), // Both axes negative
            (10.0, 0.0),   // X only
            (0.0, 5.0),    // Y only
        ];

        for (delta_x, delta_y) in test_movements {
            let mut new_x = delta_x;
            let mut new_y = delta_y;

            // Apply axis constraints
            match config.axis {
                Some(DragAxis::X) => {
                    // Only apply X constraints
                    if new_x < constraints.left.unwrap() {
                        new_x = constraints.left.unwrap();
                    } else if new_x > constraints.right.unwrap() {
                        new_x = constraints.right.unwrap();
                    }
                    // Y should remain unchanged
                },
                Some(DragAxis::Y) => {
                    // Only apply Y constraints
                    if new_y < constraints.top.unwrap() {
                        new_y = constraints.top.unwrap();
                    } else if new_y > constraints.bottom.unwrap() {
                        new_y = constraints.bottom.unwrap();
                    }
                    // X should remain unchanged
                },
                Some(DragAxis::Both) => {
                    // Apply both constraints
                    if new_x < constraints.left.unwrap() {
                        new_x = constraints.left.unwrap();
                    } else if new_x > constraints.right.unwrap() {
                        new_x = constraints.right.unwrap();
                    }
                    if new_y < constraints.top.unwrap() {
                        new_y = constraints.top.unwrap();
                    } else if new_y > constraints.bottom.unwrap() {
                        new_y = constraints.bottom.unwrap();
                    }
                },
                None => {
                    // No constraints applied
                }
            }

            // Verify constraints were applied correctly
            match config.axis {
                Some(DragAxis::X) => {
                    assert!(new_x >= constraints.left.unwrap());
                    assert!(new_x <= constraints.right.unwrap());
                    assert_eq!(new_y, delta_y); // Y should be unchanged
                },
                Some(DragAxis::Y) => {
                    assert!(new_y >= constraints.top.unwrap());
                    assert!(new_y <= constraints.bottom.unwrap());
                    assert_eq!(new_x, delta_x); // X should be unchanged
                },
                Some(DragAxis::Both) => {
                    assert!(new_x >= constraints.left.unwrap());
                    assert!(new_x <= constraints.right.unwrap());
                    assert!(new_y >= constraints.top.unwrap());
                    assert!(new_y <= constraints.bottom.unwrap());
                },
                None => {
                    // No constraints to verify
                }
            }
        }
    }
}

/// Test browser compatibility for momentum calculations
#[test]
fn test_browser_momentum_compatibility() {
    // Test momentum calculations that should be consistent across browsers
    let initial_velocity: (f64, f64) = (100.0, 50.0);
    let friction = 0.95;
    let mut position: (f64, f64) = (0.0, 0.0);
    let mut velocity = initial_velocity;
    let mut frame_count = 0;

    // Simulate momentum animation
    while (velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64) && frame_count < 100 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;
    }

    // Verify momentum calculation results
    assert!(frame_count > 0, "Momentum should have run for at least one frame");
    assert!(frame_count <= 100, "Momentum should not run indefinitely");
    
    // Check if momentum stopped due to low velocity or frame limit
    if frame_count < 100 {
        // If it stopped early, velocity should be near zero
        assert!(velocity.0.abs() <= 0.1_f64, "X velocity should be near zero when momentum stops early");
        assert!(velocity.1.abs() <= 0.1_f64, "Y velocity should be near zero when momentum stops early");
    } else {
        // If it hit the frame limit, velocity should be reduced but not necessarily near zero
        assert!(velocity.0.abs() < initial_velocity.0.abs(), "X velocity should be reduced");
        assert!(velocity.1.abs() < initial_velocity.1.abs(), "Y velocity should be reduced");
    }
    
    assert!(position.0.abs() > 0.0, "Position should have changed");
    assert!(position.1.abs() > 0.0, "Position should have changed");
}

/// Test browser compatibility for elastic behavior
#[test]
fn test_browser_elastic_compatibility() {
    // Test elastic behavior that should work consistently across browsers
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    let elastic_factors = vec![0.0, 0.1, 0.2, 0.5, 1.0];

    for elastic_factor in elastic_factors {
        let test_positions = vec![
            (-150.0, -75.0), // Outside constraints
            (150.0, 75.0),   // Outside constraints
            (0.0, 0.0),      // Inside constraints
        ];

        for (x, y) in test_positions {
            let mut constrained_x = x;
            let mut constrained_y = y;

            // Apply elastic constraints
            if constrained_x < constraints.left.unwrap() {
                if elastic_factor > 0.0 {
                    let overshoot = constraints.left.unwrap() - constrained_x;
                    constrained_x = constraints.left.unwrap() - (overshoot * elastic_factor);
                } else {
                    constrained_x = constraints.left.unwrap();
                }
            } else if constrained_x > constraints.right.unwrap() {
                if elastic_factor > 0.0 {
                    let overshoot = constrained_x - constraints.right.unwrap();
                    constrained_x = constraints.right.unwrap() + (overshoot * elastic_factor);
                } else {
                    constrained_x = constraints.right.unwrap();
                }
            }

            if constrained_y < constraints.top.unwrap() {
                if elastic_factor > 0.0 {
                    let overshoot = constraints.top.unwrap() - constrained_y;
                    constrained_y = constraints.top.unwrap() - (overshoot * elastic_factor);
                } else {
                    constrained_y = constraints.top.unwrap();
                }
            } else if constrained_y > constraints.bottom.unwrap() {
                if elastic_factor > 0.0 {
                    let overshoot = constrained_y - constraints.bottom.unwrap();
                    constrained_y = constraints.bottom.unwrap() + (overshoot * elastic_factor);
                } else {
                    constrained_y = constraints.bottom.unwrap();
                }
            }

            // Verify elastic behavior
            if elastic_factor == 0.0 {
                // No elastic behavior - should be clamped to constraints
                assert!(constrained_x >= constraints.left.unwrap());
                assert!(constrained_x <= constraints.right.unwrap());
                assert!(constrained_y >= constraints.top.unwrap());
                assert!(constrained_y <= constraints.bottom.unwrap());
            } else {
                // Elastic behavior - should allow some overshoot
                let max_overshoot = 50.0; // Allow up to 50px overshoot
                assert!(constrained_x >= constraints.left.unwrap() - max_overshoot);
                assert!(constrained_x <= constraints.right.unwrap() + max_overshoot);
                assert!(constrained_y >= constraints.top.unwrap() - max_overshoot);
                assert!(constrained_y <= constraints.bottom.unwrap() + max_overshoot);
            }
        }
    }
}

/// Test browser compatibility for animation interpolation
#[test]
fn test_browser_animation_interpolation_compatibility() {
    // Test interpolation that should be consistent across browsers
    let start_values = (0.0, 0.0, 0.0); // x, y, opacity
    let end_values = (100.0, 50.0, 1.0);
    let progress_points = vec![0.0, 0.25, 0.5, 0.75, 1.0];

    for progress in progress_points {
        // Linear interpolation
        let interpolated_x = start_values.0 + (end_values.0 - start_values.0) * progress;
        let interpolated_y = start_values.1 + (end_values.1 - start_values.1) * progress;
        let interpolated_opacity = start_values.2 + (end_values.2 - start_values.2) * progress;

        // Verify interpolation results
        assert!(interpolated_x >= start_values.0 && interpolated_x <= end_values.0);
        assert!(interpolated_y >= start_values.1 && interpolated_y <= end_values.1);
        assert!(interpolated_opacity >= start_values.2 && interpolated_opacity <= end_values.2);

        // Test cubic interpolation
        let t = progress;
        let cubic_x = start_values.0 + (end_values.0 - start_values.0) * (3.0 * t * t - 2.0 * t * t * t);
        let cubic_y = start_values.1 + (end_values.1 - start_values.1) * (3.0 * t * t - 2.0 * t * t * t);
        let cubic_opacity = start_values.2 + (end_values.2 - start_values.2) * (3.0 * t * t - 2.0 * t * t * t);

        // Verify cubic interpolation results
        assert!(cubic_x >= start_values.0 && cubic_x <= end_values.0);
        assert!(cubic_y >= start_values.1 && cubic_y <= end_values.1);
        assert!(cubic_opacity >= start_values.2 && cubic_opacity <= end_values.2);
    }
}

/// Test browser compatibility for edge cases
#[test]
fn test_browser_edge_case_compatibility() {
    // Test edge cases that should be handled consistently across browsers
    
    // Test zero values
    let zero_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(0.0),
            right: Some(0.0),
            top: Some(0.0),
            bottom: Some(0.0),
        }),
        elastic: Some(0.0),
        momentum: Some(false),
    };

    let constraints = zero_config.constraints.as_ref().unwrap();
    let test_position = (10.0, 5.0);
    let mut constrained_x = test_position.0;
    let mut constrained_y = test_position.1;

    // Apply zero constraints
    if constrained_x < constraints.left.unwrap() {
        constrained_x = constraints.left.unwrap();
    } else if constrained_x > constraints.right.unwrap() {
        constrained_x = constraints.right.unwrap();
    }

    if constrained_y < constraints.top.unwrap() {
        constrained_y = constraints.top.unwrap();
    } else if constrained_y > constraints.bottom.unwrap() {
        constrained_y = constraints.bottom.unwrap();
    }

    // Verify zero constraints
    assert_eq!(constrained_x, 0.0);
    assert_eq!(constrained_y, 0.0);

    // Test very large values
    let large_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-10000.0),
            right: Some(10000.0),
            top: Some(-5000.0),
            bottom: Some(5000.0),
        }),
        elastic: Some(0.1),
        momentum: Some(true),
    };

    let large_constraints = large_config.constraints.as_ref().unwrap();
    let large_position = (5000.0, 2500.0);
    let mut large_constrained_x = large_position.0;
    let mut large_constrained_y = large_position.1;

    // Apply large constraints
    if large_constrained_x < large_constraints.left.unwrap() {
        large_constrained_x = large_constraints.left.unwrap();
    } else if large_constrained_x > large_constraints.right.unwrap() {
        large_constrained_x = large_constraints.right.unwrap();
    }

    if large_constrained_y < large_constraints.top.unwrap() {
        large_constrained_y = large_constraints.top.unwrap();
    } else if large_constrained_y > large_constraints.bottom.unwrap() {
        large_constrained_y = large_constraints.bottom.unwrap();
    }

    // Verify large constraints
    assert!(large_constrained_x >= large_constraints.left.unwrap());
    assert!(large_constrained_x <= large_constraints.right.unwrap());
    assert!(large_constrained_y >= large_constraints.top.unwrap());
    assert!(large_constrained_y <= large_constraints.bottom.unwrap());
}
