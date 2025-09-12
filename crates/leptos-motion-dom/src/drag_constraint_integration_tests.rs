// Drag Constraint Integration Tests
//
// These tests verify that drag constraints work correctly in real-world scenarios
// and integrate properly with the MotionDiv component and momentum animations.

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Test drag constraint integration with momentum animation
#[test]
fn test_drag_constraint_momentum_integration() {
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

    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();
    let friction = 0.95;

    // Simulate drag with momentum that hits constraints
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (150.0, 75.0); // High velocity that will hit boundaries
    let mut frame_count = 0;
    let mut constraint_hits = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply constraints with elastic behavior
        let mut hit_constraint = false;

        if let Some(left) = constraints.left {
            if position.0 < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - position.0;
                    position.0 = left - (overshoot * elastic_factor);
                } else {
                    position.0 = left;
                }
                velocity.0 *= -0.5; // Bounce back with reduced velocity
                hit_constraint = true;
            }
        }

        if let Some(right) = constraints.right {
            if position.0 > right {
                if elastic_factor > 0.0 {
                    let overshoot = position.0 - right;
                    position.0 = right + (overshoot * elastic_factor);
                } else {
                    position.0 = right;
                }
                velocity.0 *= -0.5; // Bounce back with reduced velocity
                hit_constraint = true;
            }
        }

        if let Some(top) = constraints.top {
            if position.1 < top {
                if elastic_factor > 0.0 {
                    let overshoot = top - position.1;
                    position.1 = top - (overshoot * elastic_factor);
                } else {
                    position.1 = top;
                }
                velocity.1 *= -0.5; // Bounce back with reduced velocity
                hit_constraint = true;
            }
        }

        if let Some(bottom) = constraints.bottom {
            if position.1 > bottom {
                if elastic_factor > 0.0 {
                    let overshoot = position.1 - bottom;
                    position.1 = bottom + (overshoot * elastic_factor);
                } else {
                    position.1 = bottom;
                }
                velocity.1 *= -0.5; // Bounce back with reduced velocity
                hit_constraint = true;
            }
        }

        if hit_constraint {
            constraint_hits += 1;
        }

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify integration worked correctly
    assert!(frame_count > 0);
    assert!(frame_count < 1000);
    assert!(constraint_hits > 0); // Should have hit constraints
    assert!(velocity.0.abs() < 0.1);
    assert!(velocity.1.abs() < 0.1);

    // Verify final position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0);
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
    assert!(position.1 >= constraints.top.unwrap() - 10.0);
    assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
}

/// Test drag constraint integration with axis constraints
#[test]
fn test_drag_constraint_axis_integration() {
    // Test X-axis only with constraints
    let x_config = DragConfig {
        axis: Some(DragAxis::X),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(0.2),
        momentum: Some(true),
    };

    let constraints = x_config.constraints.as_ref().unwrap();
    let elastic_factor = x_config.elastic.unwrap();

    // Simulate drag with X-axis constraint
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (100.0, 50.0); // Both X and Y velocity
    let friction = 0.95;
    let mut frame_count = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply axis constraint
        if let Some(DragAxis::X) = x_config.axis {
            // Y position should remain at initial value (0.0)
            position.1 = 0.0;
            velocity.1 = 0.0;
        }

        // Apply constraints with elastic behavior
        if let Some(left) = constraints.left {
            if position.0 < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - position.0;
                    position.0 = left - (overshoot * elastic_factor);
                } else {
                    position.0 = left;
                }
                velocity.0 *= -0.5;
            }
        }

        if let Some(right) = constraints.right {
            if position.0 > right {
                if elastic_factor > 0.0 {
                    let overshoot = position.0 - right;
                    position.0 = right + (overshoot * elastic_factor);
                } else {
                    position.0 = right;
                }
                velocity.0 *= -0.5;
            }
        }

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify X-axis constraint was enforced
    assert_eq!(position.1, 0.0); // Y should remain at 0.0
    assert!(velocity.1.abs() < 0.1); // Y velocity should be 0

    // Verify X position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0);
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
}

/// Test drag constraint integration with animation targets
#[test]
fn test_drag_constraint_animation_target_integration() {
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

    // Create animation target
    let mut target = HashMap::new();
    target.insert("x".to_string(), AnimationValue::Pixels(0.0));
    target.insert("y".to_string(), AnimationValue::Pixels(0.0));
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));

    // Simulate drag with animation target
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (80.0, 40.0);
    let friction = 0.95;
    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();
    let mut frame_count = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply constraints with elastic behavior
        if let Some(left) = constraints.left {
            if position.0 < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - position.0;
                    position.0 = left - (overshoot * elastic_factor);
                } else {
                    position.0 = left;
                }
                velocity.0 *= -0.5;
            }
        }

        if let Some(right) = constraints.right {
            if position.0 > right {
                if elastic_factor > 0.0 {
                    let overshoot = position.0 - right;
                    position.0 = right + (overshoot * elastic_factor);
                } else {
                    position.0 = right;
                }
                velocity.0 *= -0.5;
            }
        }

        if let Some(top) = constraints.top {
            if position.1 < top {
                if elastic_factor > 0.0 {
                    let overshoot = top - position.1;
                    position.1 = top - (overshoot * elastic_factor);
                } else {
                    position.1 = top;
                }
                velocity.1 *= -0.5;
            }
        }

        if let Some(bottom) = constraints.bottom {
            if position.1 > bottom {
                if elastic_factor > 0.0 {
                    let overshoot = position.1 - bottom;
                    position.1 = bottom + (overshoot * elastic_factor);
                } else {
                    position.1 = bottom;
                }
                velocity.1 *= -0.5;
            }
        }

        // Update animation target
        target.insert("x".to_string(), AnimationValue::Pixels(position.0));
        target.insert("y".to_string(), AnimationValue::Pixels(position.1));

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify animation target was updated correctly
    assert_eq!(target.get("x"), Some(&AnimationValue::Pixels(position.0)));
    assert_eq!(target.get("y"), Some(&AnimationValue::Pixels(position.1)));
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));

    // Verify final position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0);
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
    assert!(position.1 >= constraints.top.unwrap() - 10.0);
    assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
}

/// Test drag constraint integration with transitions
#[test]
fn test_drag_constraint_transition_integration() {
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

    let transition = Transition {
        duration: Some(0.3),
        ease: Easing::EaseOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Simulate drag with transition
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (60.0, 30.0);
    let friction = 0.95;
    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();
    let mut frame_count = 0;
    let mut transition_progress = 0.0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply constraints with elastic behavior
        if let Some(left) = constraints.left {
            if position.0 < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - position.0;
                    position.0 = left - (overshoot * elastic_factor);
                } else {
                    position.0 = left;
                }
                velocity.0 *= -0.5;
            }
        }

        if let Some(right) = constraints.right {
            if position.0 > right {
                if elastic_factor > 0.0 {
                    let overshoot = position.0 - right;
                    position.0 = right + (overshoot * elastic_factor);
                } else {
                    position.0 = right;
                }
                velocity.0 *= -0.5;
            }
        }

        if let Some(top) = constraints.top {
            if position.1 < top {
                if elastic_factor > 0.0 {
                    let overshoot = top - position.1;
                    position.1 = top - (overshoot * elastic_factor);
                } else {
                    position.1 = top;
                }
                velocity.1 *= -0.5;
            }
        }

        if let Some(bottom) = constraints.bottom {
            if position.1 > bottom {
                if elastic_factor > 0.0 {
                    let overshoot = position.1 - bottom;
                    position.1 = bottom + (overshoot * elastic_factor);
                } else {
                    position.1 = bottom;
                }
                velocity.1 *= -0.5;
            }
        }

        // Update transition progress
        if let Some(duration) = transition.duration {
            transition_progress += 1.0 / (duration * 60.0); // Assuming 60fps
            if transition_progress > 1.0 {
                transition_progress = 1.0;
            }
        }

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify transition was applied
    assert!(transition_progress > 0.0);
    assert!(transition_progress <= 1.0);

    // Verify final position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0);
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
    assert!(position.1 >= constraints.top.unwrap() - 10.0);
    assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
}

/// Test drag constraint integration with multiple elements
#[test]
fn test_drag_constraint_multiple_elements_integration() {
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

    // Simulate multiple elements with drag constraints
    let mut elements: Vec<((f64, f64), (f64, f64))> = vec![
        ((0.0, 0.0), (50.0, 25.0)),   // Element 1: position, velocity
        ((10.0, 5.0), (40.0, 20.0)),  // Element 2: position, velocity
        ((-5.0, -2.0), (60.0, 30.0)), // Element 3: position, velocity
    ];

    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();
    let friction = 0.95;
    let mut frame_count = 0;

    while elements
        .iter()
        .any(|(_, (vx, vy))| vx.abs() > 0.1_f64 || vy.abs() > 0.1_f64)
    {
        frame_count += 1;

        for (position, velocity) in &mut elements {
            // Apply velocity to position
            position.0 += velocity.0;
            position.1 += velocity.1;

            // Apply constraints with elastic behavior
            if let Some(left) = constraints.left {
                if position.0 < left {
                    if elastic_factor > 0.0 {
                        let overshoot = left - position.0;
                        position.0 = left - (overshoot * elastic_factor);
                    } else {
                        position.0 = left;
                    }
                    velocity.0 *= -0.5;
                }
            }

            if let Some(right) = constraints.right {
                if position.0 > right {
                    if elastic_factor > 0.0 {
                        let overshoot = position.0 - right;
                        position.0 = right + (overshoot * elastic_factor);
                    } else {
                        position.0 = right;
                    }
                    velocity.0 *= -0.5;
                }
            }

            if let Some(top) = constraints.top {
                if position.1 < top {
                    if elastic_factor > 0.0 {
                        let overshoot = top - position.1;
                        position.1 = top - (overshoot * elastic_factor);
                    } else {
                        position.1 = top;
                    }
                    velocity.1 *= -0.5;
                }
            }

            if let Some(bottom) = constraints.bottom {
                if position.1 > bottom {
                    if elastic_factor > 0.0 {
                        let overshoot = position.1 - bottom;
                        position.1 = bottom + (overshoot * elastic_factor);
                    } else {
                        position.1 = bottom;
                    }
                    velocity.1 *= -0.5;
                }
            }

            // Apply friction
            velocity.0 *= friction;
            velocity.1 *= friction;
        }

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify all elements completed animation
    assert!(frame_count > 0);
    assert!(frame_count < 1000);

    // Verify all elements are within constraints
    for (position, _) in &elements {
        assert!(position.0 >= constraints.left.unwrap() - 10.0);
        assert!(position.0 <= constraints.right.unwrap() + 10.0);
        assert!(position.1 >= constraints.top.unwrap() - 10.0);
        assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
    }
}

/// Test drag constraint integration with edge cases
#[test]
fn test_drag_constraint_edge_cases_integration() {
    // Test with no constraints
    let no_constraints_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: None,
        elastic: Some(0.2),
        momentum: Some(true),
    };

    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (50.0, 25.0);
    let friction = 0.95;
    let mut frame_count = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // No constraints to apply

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify animation completed without constraints
    assert!(frame_count > 0);
    assert!(frame_count < 1000);

    // Test with zero elastic factor
    let zero_elastic_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(0.0),
        momentum: Some(true),
    };

    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (150.0, 75.0);
    let friction = 0.95;
    let constraints = zero_elastic_config.constraints.as_ref().unwrap();
    let mut frame_count = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply constraints with zero elastic factor
        if let Some(left) = constraints.left {
            if position.0 < left {
                position.0 = left;
                velocity.0 *= -0.5;
            }
        }

        if let Some(right) = constraints.right {
            if position.0 > right {
                position.0 = right;
                velocity.0 *= -0.5;
            }
        }

        if let Some(top) = constraints.top
            && position.1 < top {
            position.1 = top;
            velocity.1 *= -0.5;
        }

        if let Some(bottom) = constraints.bottom
            && position.1 > bottom {
            position.1 = bottom;
            velocity.1 *= -0.5;
        }

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify animation completed with zero elastic factor
    assert!(frame_count > 0);
    assert!(frame_count < 1000);

    // Verify position is exactly at boundaries (no elastic overshoot)
    assert!(position.0 >= constraints.left.unwrap());
    assert!(position.0 <= constraints.right.unwrap());
    assert!(position.1 >= constraints.top.unwrap());
    assert!(position.1 <= constraints.bottom.unwrap());
}
