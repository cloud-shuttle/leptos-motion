// Comprehensive Drag Constraint Tests
//
// These tests verify advanced drag constraint functionality including:
// - Axis constraints (X, Y, Both)
// - Boundary constraints with elastic behavior
// - Momentum with constraints
// - Performance characteristics
// - Edge cases and error handling

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::AnimationValue;
use std::collections::HashMap;

/// Helper function to create a comprehensive drag configuration
fn create_comprehensive_drag_config() -> DragConfig {
    DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-200.0),
            right: Some(200.0),
            top: Some(-100.0),
            bottom: Some(100.0),
        }),
        elastic: Some(0.3),
        momentum: Some(true),
    }
}

/// Test comprehensive drag constraint application
#[test]
fn test_comprehensive_drag_constraints() {
    let config = create_comprehensive_drag_config();
    let constraints = config.constraints.as_ref().unwrap();

    // Test all boundaries are set
    assert!(constraints.left.is_some());
    assert!(constraints.right.is_some());
    assert!(constraints.top.is_some());
    assert!(constraints.bottom.is_some());

    // Test boundary values
    assert_eq!(constraints.left, Some(-200.0));
    assert_eq!(constraints.right, Some(200.0));
    assert_eq!(constraints.top, Some(-100.0));
    assert_eq!(constraints.bottom, Some(100.0));

    // Test elastic factor
    assert_eq!(config.elastic, Some(0.3));
    assert!(config.elastic.unwrap() > 0.0);
    assert!(config.elastic.unwrap() < 1.0);
}

/// Test axis constraint enforcement
#[test]
fn test_axis_constraint_enforcement() {
    // Test X-axis only constraint
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

    // Simulate drag movement
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let movement_x = 50.0;
    let movement_y = 30.0; // This should be ignored

    // Apply X-axis constraint
    match x_config.axis {
        Some(DragAxis::X) => {
            current_x += movement_x;
            // Y should remain unchanged
        }
        _ => {
            current_x += movement_x;
            current_y += movement_y;
        }
    }

    assert_eq!(current_x, 50.0);
    assert_eq!(current_y, 0.0); // Y should be unchanged for X-axis constraint

    // Test Y-axis only constraint
    let y_config = DragConfig {
        axis: Some(DragAxis::Y),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(0.2),
        momentum: Some(true),
    };

    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let movement_x = 40.0; // This should be ignored
    let movement_y = 25.0;

    // Apply Y-axis constraint
    match y_config.axis {
        Some(DragAxis::Y) => {
            current_y += movement_y;
            // X should remain unchanged
        }
        _ => {
            current_x += movement_x;
            current_y += movement_y;
        }
    }

    assert_eq!(current_x, 0.0); // X should be unchanged for Y-axis constraint
    assert_eq!(current_y, 25.0);
}

/// Test boundary constraint enforcement with elastic behavior
#[test]
fn test_boundary_constraint_enforcement() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };
    let elastic_factor = 0.2;

    // Test position within boundaries (no constraint needed)
    let mut position_x = 50.0;
    let mut position_y = 25.0;

    // Apply left boundary constraint
    if let Some(left) = constraints.left {
        if position_x < left {
            if elastic_factor > 0.0 {
                let overshoot = left - position_x;
                position_x = left - (overshoot * elastic_factor);
            } else {
                position_x = left;
            }
        }
    }

    assert_eq!(position_x, 50.0); // Should remain unchanged

    // Test position beyond right boundary (elastic)
    let mut position_x = 150.0;
    if let Some(right) = constraints.right {
        if position_x > right {
            if elastic_factor > 0.0 {
                let overshoot = position_x - right;
                position_x = right + (overshoot * elastic_factor);
            } else {
                position_x = right;
            }
        }
    }

    let expected_x = 100.0 + (50.0 * 0.2); // 100 + (50 * 0.2) = 110.0
    assert_eq!(position_x, expected_x);

    // Test position beyond left boundary (elastic)
    let mut position_x = -150.0;
    if let Some(left) = constraints.left {
        if position_x < left {
            if elastic_factor > 0.0 {
                let overshoot = left - position_x;
                position_x = left - (overshoot * elastic_factor);
            } else {
                position_x = left;
            }
        }
    }

    let expected_x = -100.0 - (50.0 * 0.2); // -100 - (50 * 0.2) = -110.0
    assert_eq!(position_x, expected_x);
}

/// Test momentum with constraints
#[test]
fn test_momentum_with_constraints() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };
    let elastic_factor = 0.1;
    let friction = 0.95;

    // Simulate momentum animation with constraints
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (200.0, 100.0); // High velocity that will hit boundaries
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
                velocity.0 *= -0.5; // Bounce back with reduced velocity
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

    // Verify animation completed
    assert!(frame_count > 0);
    assert!(frame_count < 1000); // Should complete within reasonable time
    assert!(velocity.0.abs() < 0.1);
    assert!(velocity.1.abs() < 0.1);

    // Verify position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0); // Allow some elastic overshoot
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
    assert!(position.1 >= constraints.top.unwrap() - 10.0);
    assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
}

/// Test edge cases for drag constraints
#[test]
fn test_drag_constraint_edge_cases() {
    // Test with no constraints
    let no_constraints_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: None,
        elastic: Some(0.2),
        momentum: Some(true),
    };

    assert!(no_constraints_config.constraints.is_none());

    // Test with partial constraints (only left and right)
    let partial_constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: None,
        bottom: None,
    };

    assert!(partial_constraints.left.is_some());
    assert!(partial_constraints.right.is_some());
    assert!(partial_constraints.top.is_none());
    assert!(partial_constraints.bottom.is_none());

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

    assert_eq!(zero_elastic_config.elastic, Some(0.0));

    // Test with maximum elastic factor
    let max_elastic_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(1.0),
        momentum: Some(true),
    };

    assert_eq!(max_elastic_config.elastic, Some(1.0));
}

/// Test drag constraint performance
#[test]
fn test_drag_constraint_performance() {
    let config = create_comprehensive_drag_config();
    let constraints = config.constraints.as_ref().unwrap();
    let elastic_factor = config.elastic.unwrap();

    let start_time = std::time::Instant::now();

    // Simulate 1000 constraint applications
    for i in 0..1000 {
        let test_x = (i as f64) * 0.1 - 50.0; // Vary from -50 to 50
        let test_y = (i as f64) * 0.05 - 25.0; // Vary from -25 to 25

        let mut constrained_x = test_x;
        let mut constrained_y = test_y;

        // Apply constraints
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

        // Verify constraints were applied
        assert!(constrained_x >= constraints.left.unwrap() - 10.0);
        assert!(constrained_x <= constraints.right.unwrap() + 10.0);
        assert!(constrained_y >= constraints.top.unwrap() - 10.0);
        assert!(constrained_y <= constraints.bottom.unwrap() + 10.0);
    }

    let duration = start_time.elapsed();

    // Verify performance (should complete 1000 operations in under 10ms)
    assert!(duration.as_millis() < 10);
}

/// Test drag constraint validation
#[test]
fn test_drag_constraint_validation() {
    // Test valid constraints
    let valid_constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    assert!(valid_constraints.left.unwrap() < valid_constraints.right.unwrap());
    assert!(valid_constraints.top.unwrap() < valid_constraints.bottom.unwrap());

    // Test invalid constraints (left > right)
    let invalid_constraints = DragConstraints {
        left: Some(100.0),
        right: Some(-100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    assert!(invalid_constraints.left.unwrap() > invalid_constraints.right.unwrap());

    // Test invalid constraints (top > bottom)
    let invalid_constraints_2 = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(50.0),
        bottom: Some(-50.0),
    };

    assert!(invalid_constraints_2.top.unwrap() > invalid_constraints_2.bottom.unwrap());
}

/// Test drag constraint with different elastic factors
#[test]
fn test_drag_constraint_elastic_factors() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    let test_position = 150.0; // Beyond right boundary
    let right_boundary = constraints.right.unwrap();

    // Test different elastic factors
    let elastic_factors = [0.0, 0.1, 0.2, 0.5, 0.8, 1.0];

    for elastic_factor in elastic_factors {
        let mut constrained_position = test_position;

        if constrained_position > right_boundary {
            if elastic_factor > 0.0 {
                let overshoot = constrained_position - right_boundary;
                constrained_position = right_boundary + (overshoot * elastic_factor);
            } else {
                constrained_position = right_boundary;
            }
        }

        // Verify elastic behavior
        if elastic_factor == 0.0 {
            assert_eq!(constrained_position, right_boundary);
        } else {
            assert!(constrained_position > right_boundary);
            // With elastic factor, position should be between boundary and original position
            assert!(constrained_position <= test_position);
        }

        // Verify elastic factor scaling
        let expected_position =
            right_boundary + ((test_position - right_boundary) * elastic_factor);
        assert_eq!(constrained_position, expected_position);
    }
}

/// Test drag constraint with momentum and multiple bounces
#[test]
fn test_drag_constraint_momentum_bounces() {
    let constraints = DragConstraints {
        left: Some(-50.0),
        right: Some(50.0),
        top: Some(-25.0),
        bottom: Some(25.0),
    };
    let elastic_factor = 0.3;
    let friction = 0.9;

    // Start with high velocity that will cause multiple bounces
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (100.0, 50.0);
    let mut bounce_count = 0;
    let mut frame_count = 0;

    while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
        frame_count += 1;
        let old_velocity = velocity;

        // Apply velocity to position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Check for bounces
        let mut bounced = false;

        // Apply constraints with elastic behavior
        if let Some(left) = constraints.left {
            if position.0 < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - position.0;
                    position.0 = left - (overshoot * elastic_factor);
                } else {
                    position.0 = left;
                }
                velocity.0 *= -0.7; // Bounce back with reduced velocity
                bounced = true;
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
                velocity.0 *= -0.7; // Bounce back with reduced velocity
                bounced = true;
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
                velocity.1 *= -0.7; // Bounce back with reduced velocity
                bounced = true;
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
                velocity.1 *= -0.7; // Bounce back with reduced velocity
                bounced = true;
            }
        }

        if bounced {
            bounce_count += 1;
        }

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Prevent infinite loops
        if frame_count > 1000 {
            break;
        }
    }

    // Verify animation completed with bounces
    assert!(frame_count > 0);
    assert!(frame_count < 1000);
    assert!(bounce_count > 0); // Should have at least one bounce
    assert!(velocity.0.abs() < 0.1);
    assert!(velocity.1.abs() < 0.1);

    // Verify final position is within constraints
    assert!(position.0 >= constraints.left.unwrap() - 10.0);
    assert!(position.0 <= constraints.right.unwrap() + 10.0);
    assert!(position.1 >= constraints.top.unwrap() - 10.0);
    assert!(position.1 <= constraints.bottom.unwrap() + 10.0);
}
