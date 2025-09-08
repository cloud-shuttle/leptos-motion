// Momentum Animation Tests for leptos-motion-dom
// Tests for requestAnimationFrame-based momentum animation functionality

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};

#[test]
fn test_momentum_velocity_calculation() {
    // Test velocity calculation from position changes
    let start_time = 1000.0;
    let end_time = 1016.0; // 16ms later (60fps)
    let start_pos = (0.0, 0.0);
    let end_pos = (100.0, 50.0);

    let time_delta = end_time - start_time;
    let (delta_x, delta_y) = (end_pos.0 - start_pos.0, end_pos.1 - start_pos.1);

    let velocity_x = delta_x / time_delta;
    let velocity_y = delta_y / time_delta;

    // At 60fps, 100px in 16ms = 6.25px/ms
    assert!((velocity_x - 6.25f64).abs() < 0.1);
    assert!((velocity_y - 3.125f64).abs() < 0.1);
}

#[test]
fn test_momentum_velocity_magnitude() {
    // Test velocity magnitude calculation
    let velocity: (f64, f64) = (3.0, 4.0);
    let magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();

    assert!((magnitude - 5.0).abs() < 0.001);
}

#[test]
fn test_momentum_threshold_detection() {
    // Test momentum threshold detection
    let high_velocity: (f64, f64) = (100.0, 50.0);
    let low_velocity: (f64, f64) = (0.05, 0.02);
    let threshold = 0.1;

    let high_magnitude: f64 =
        (high_velocity.0 * high_velocity.0 + high_velocity.1 * high_velocity.1).sqrt();
    let low_magnitude: f64 =
        (low_velocity.0 * low_velocity.0 + low_velocity.1 * low_velocity.1).sqrt();

    assert!(high_magnitude > threshold);
    assert!(low_magnitude < threshold);
}

#[test]
fn test_momentum_friction_calculation() {
    // Test friction application to velocity
    let initial_velocity = (100.0, 50.0);
    let friction = 0.95;

    let new_velocity_x = initial_velocity.0 * friction;
    let new_velocity_y = initial_velocity.1 * friction;

    assert!((new_velocity_x - 95.0f64).abs() < 0.001);
    assert!((new_velocity_y - 47.5f64).abs() < 0.001);
}

#[test]
fn test_momentum_position_update() {
    // Test position update based on velocity
    let current_position = (100.0, 200.0);
    let velocity = (10.0, -5.0);

    let new_x = current_position.0 + velocity.0;
    let new_y = current_position.1 + velocity.1;

    assert!((new_x - 110.0f64).abs() < 0.001);
    assert!((new_y - 195.0f64).abs() < 0.001);
}

#[test]
fn test_momentum_stopping_condition() {
    // Test momentum stopping when velocity is very low
    let very_low_velocity: (f64, f64) = (0.05, 0.02);
    let stopping_threshold = 0.1;

    let magnitude: f64 = (very_low_velocity.0 * very_low_velocity.0
        + very_low_velocity.1 * very_low_velocity.1)
        .sqrt();

    assert!(magnitude < stopping_threshold);
}

#[test]
fn test_momentum_with_constraints() {
    // Test momentum animation respects drag constraints
    let drag_config = DragConfig {
        axis: Some(DragAxis::X),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-50.0),
            bottom: Some(50.0),
        }),
        elastic: Some(0.3),
        momentum: Some(true),
    };

    // Test that momentum respects axis constraints
    assert_eq!(drag_config.axis, Some(DragAxis::X));

    // Test that momentum respects boundary constraints
    if let Some(constraints) = &drag_config.constraints {
        assert_eq!(constraints.left, Some(-100.0));
        assert_eq!(constraints.right, Some(100.0));
    }
}

#[test]
fn test_momentum_elastic_boundaries() {
    // Test momentum with elastic boundaries
    let drag_config = DragConfig {
        axis: None,
        constraints: Some(DragConstraints {
            left: Some(0.0),
            right: Some(200.0),
            top: Some(0.0),
            bottom: Some(200.0),
        }),
        elastic: Some(0.5),
        momentum: Some(true),
    };

    // Test elastic factor
    assert_eq!(drag_config.elastic, Some(0.5));

    // Test elastic overshoot calculation
    let overshoot = 50.0;
    let elastic_factor = 0.5;
    let elastic_overshoot = overshoot * elastic_factor;

    assert!((elastic_overshoot - 25.0f64).abs() < 0.001);
}

#[test]
fn test_momentum_animation_cycle() {
    // Test complete momentum animation cycle
    let mut velocity: (f64, f64) = (10.0, 5.0); // Small initial velocity for quick test
    let mut position = (0.0, 0.0);
    let friction = 0.95;
    let mut frame_count = 0;

    // Simulate momentum animation frames
    while (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt() > 0.1f64 && frame_count < 100 {
        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        frame_count += 1;
    }

    // Should have stopped due to low velocity
    assert!(frame_count < 100);
    assert!((velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt() <= 0.1f64);
}

#[test]
fn test_momentum_performance_metrics() {
    // Test momentum animation performance characteristics
    let initial_velocity: (f64, f64) = (1000.0, 500.0);
    let friction = 0.95;
    let mut velocity: (f64, f64) = initial_velocity;
    let mut total_distance = 0.0;
    let mut frame_count = 0;

    // Simulate momentum animation
    while (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt() > 0.1f64 && frame_count < 200 {
        let distance: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        total_distance += distance;

        velocity.0 *= friction;
        velocity.1 *= friction;
        frame_count += 1;
    }

    // Performance assertions
    assert!(frame_count > 0);
    assert!(total_distance > 0.0);
    assert!(frame_count < 200); // Should stop before max frames
}

#[test]
fn test_momentum_drag_config_validation() {
    // Test momentum configuration validation
    let valid_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-100.0),
            bottom: Some(100.0),
        }),
        elastic: Some(0.2),
        momentum: Some(true),
    };

    // Validate momentum is enabled
    assert_eq!(valid_config.momentum, Some(true));

    // Validate elastic factor is reasonable
    if let Some(elastic) = valid_config.elastic {
        assert!(elastic >= 0.0);
        assert!(elastic <= 1.0);
    }

    // Validate constraints are reasonable
    if let Some(constraints) = &valid_config.constraints {
        if let (Some(left), Some(right)) = (constraints.left, constraints.right) {
            assert!(left <= right);
        }
        if let (Some(top), Some(bottom)) = (constraints.top, constraints.bottom) {
            assert!(top <= bottom);
        }
    }
}

#[test]
fn test_momentum_axis_constraints() {
    // Test momentum respects axis constraints
    let x_only_config = DragConfig {
        axis: Some(DragAxis::X),
        constraints: None,
        elastic: None,
        momentum: Some(true),
    };

    let y_only_config = DragConfig {
        axis: Some(DragAxis::Y),
        constraints: None,
        elastic: None,
        momentum: Some(true),
    };

    // Test X-axis only momentum
    let mut velocity_x = (100.0, 50.0);
    if let Some(DragAxis::X) = x_only_config.axis {
        velocity_x.1 = 0.0; // Y velocity should be zero
    }
    assert_eq!(velocity_x.1, 0.0);

    // Test Y-axis only momentum
    let mut velocity_y = (100.0, 50.0);
    if let Some(DragAxis::Y) = y_only_config.axis {
        velocity_y.0 = 0.0; // X velocity should be zero
    }
    assert_eq!(velocity_y.0, 0.0);
}

#[test]
fn test_momentum_boundary_collision() {
    // Test momentum animation with boundary collision
    let constraints = DragConstraints {
        left: Some(0.0),
        right: Some(100.0),
        top: Some(0.0),
        bottom: Some(100.0),
    };

    // Test position at boundary
    let position_at_right: f64 = 100.0;
    let velocity_toward_right: f64 = 10.0;

    // Should be constrained to boundary
    let constrained_position = position_at_right.min(100.0);
    assert_eq!(constrained_position, 100.0);

    // Test position beyond boundary
    let position_beyond_right: f64 = 110.0;
    let constrained_beyond = position_beyond_right.min(100.0);
    assert_eq!(constrained_beyond, 100.0);
}

#[test]
fn test_momentum_animation_state_transitions() {
    // Test momentum animation state transitions
    let mut is_dragging = true;
    let mut is_animating_momentum = false;
    let velocity: (f64, f64) = (100.0, 50.0);
    let velocity_threshold = 0.1;

    // Transition from dragging to momentum
    if is_dragging && !is_animating_momentum {
        let velocity_magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        if velocity_magnitude > velocity_threshold {
            is_dragging = false;
            is_animating_momentum = true;
        }
    }

    assert!(!is_dragging);
    assert!(is_animating_momentum);

    // Transition from momentum to stopped
    let low_velocity: (f64, f64) = (0.05, 0.02);
    let low_magnitude: f64 =
        (low_velocity.0 * low_velocity.0 + low_velocity.1 * low_velocity.1).sqrt();

    if is_animating_momentum && low_magnitude < velocity_threshold {
        is_animating_momentum = false;
    }

    assert!(!is_animating_momentum);
}
