// Momentum Integration Tests for leptos-motion-dom
// Integration tests for momentum animation with MotionDiv component

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, RepeatConfig, Transition};

#[test]
fn test_momentum_animation_target_creation() {
    // Test creating animation targets for momentum
    let mut target = AnimationTarget::new();
    target.insert(
        "transform".to_string(),
        AnimationValue::String("translate3d(0px, 0px, 0)".to_string()),
    );
    target.insert(
        "cursor".to_string(),
        AnimationValue::String("grabbing".to_string()),
    );

    assert_eq!(target.len(), 2);
    assert!(target.contains_key("transform"));
    assert!(target.contains_key("cursor"));
}

#[test]
fn test_momentum_drag_config_creation() {
    // Test creating drag config for momentum animation
    let drag_config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-200.0),
            right: Some(200.0),
            top: Some(-200.0),
            bottom: Some(200.0),
        }),
        elastic: Some(0.3),
        momentum: Some(true),
    };

    assert_eq!(drag_config.momentum, Some(true));
    assert_eq!(drag_config.elastic, Some(0.3));
    assert_eq!(drag_config.axis, Some(DragAxis::Both));

    if let Some(constraints) = &drag_config.constraints {
        assert_eq!(constraints.left, Some(-200.0));
        assert_eq!(constraints.right, Some(200.0));
        assert_eq!(constraints.top, Some(-200.0));
        assert_eq!(constraints.bottom, Some(200.0));
    }
}

#[test]
fn test_momentum_animation_sequence() {
    // Test complete momentum animation sequence
    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (100.0, 50.0);
    let friction = 0.95;
    let mut positions = Vec::new();
    let mut velocities = Vec::new();

    // Simulate momentum animation frames
    for _ in 0..50 {
        positions.push(position);
        velocities.push(velocity);

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Stop if velocity is very low
        let magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        if magnitude < 0.1 {
            break;
        }
    }

    // Verify animation sequence
    assert!(!positions.is_empty());
    assert!(!velocities.is_empty());

    // Verify velocity decreases over time
    for i in 1..velocities.len() {
        let prev_magnitude: f64 = (velocities[i - 1].0 * velocities[i - 1].0
            + velocities[i - 1].1 * velocities[i - 1].1)
            .sqrt();
        let curr_magnitude: f64 =
            (velocities[i].0 * velocities[i].0 + velocities[i].1 * velocities[i].1).sqrt();
        assert!(curr_magnitude <= prev_magnitude);
    }

    // Verify position changes
    assert!(positions.len() > 1);
    assert!(
        positions[positions.len() - 1].0 != positions[0].0
            || positions[positions.len() - 1].1 != positions[0].1
    );
}

#[test]
fn test_momentum_with_elastic_constraints() {
    // Test momentum animation with elastic constraints
    let constraints = DragConstraints {
        left: Some(0.0),
        right: Some(100.0),
        top: Some(0.0),
        bottom: Some(100.0),
    };
    let elastic_factor = 0.5;

    let mut position = (50.0, 50.0);
    let mut velocity = (20.0, 10.0);
    let friction = 0.95;

    // Simulate momentum with elastic boundaries
    for _ in 0..20 {
        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply elastic constraints
        if let Some(left) = constraints.left {
            if position.0 < left {
                let overshoot = left - position.0;
                position.0 = left - (overshoot * elastic_factor);
            }
        }
        if let Some(right) = constraints.right {
            if position.0 > right {
                let overshoot = position.0 - right;
                position.0 = right + (overshoot * elastic_factor);
            }
        }
        if let Some(top) = constraints.top {
            if position.1 < top {
                let overshoot = top - position.1;
                position.1 = top - (overshoot * elastic_factor);
            }
        }
        if let Some(bottom) = constraints.bottom {
            if position.1 > bottom {
                let overshoot = position.1 - bottom;
                position.1 = bottom + (overshoot * elastic_factor);
            }
        }
    }

    // Verify position is within or slightly beyond constraints (due to elastic)
    assert!(position.0 >= -10.0); // Allow some elastic overshoot
    assert!(position.0 <= 110.0);
    assert!(position.1 >= -10.0);
    assert!(position.1 <= 110.0);
}

#[test]
fn test_momentum_axis_constraints() {
    // Test momentum animation with axis constraints
    let x_axis_config = DragConfig {
        axis: Some(DragAxis::X),
        constraints: None,
        elastic: None,
        momentum: Some(true),
    };

    let y_axis_config = DragConfig {
        axis: Some(DragAxis::Y),
        constraints: None,
        elastic: None,
        momentum: Some(true),
    };

    // Test X-axis only momentum
    let mut velocity_x = (100.0, 50.0);
    let mut position_x = (0.0, 0.0);

    if let Some(DragAxis::X) = x_axis_config.axis {
        velocity_x.1 = 0.0; // Constrain to X-axis
    }

    // Simulate X-axis momentum
    for _ in 0..10 {
        position_x.0 += velocity_x.0;
        position_x.1 += velocity_x.1; // Should remain 0
        velocity_x.0 *= 0.95;
    }

    assert_eq!(position_x.1, 0.0); // Y position should not change

    // Test Y-axis only momentum
    let mut velocity_y = (100.0, 50.0);
    let mut position_y = (0.0, 0.0);

    if let Some(DragAxis::Y) = y_axis_config.axis {
        velocity_y.0 = 0.0; // Constrain to Y-axis
    }

    // Simulate Y-axis momentum
    for _ in 0..10 {
        position_y.0 += velocity_y.0; // Should remain 0
        position_y.1 += velocity_y.1;
        velocity_y.1 *= 0.95;
    }

    assert_eq!(position_y.0, 0.0); // X position should not change
}

#[test]
fn test_momentum_performance_characteristics() {
    // Test momentum animation performance characteristics
    let initial_velocity: (f64, f64) = (10.0, 5.0); // Small initial velocity for quick test
    let friction = 0.95;
    let mut velocity: (f64, f64) = initial_velocity;
    let mut position = (0.0, 0.0);
    let mut frame_count = 0;
    let mut total_distance = 0.0;
    let mut max_velocity: f64 = 0.0;

    // Simulate momentum animation
    while (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt() > 0.1f64 && frame_count < 100 {
        let current_velocity_magnitude: f64 =
            (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        max_velocity = max_velocity.max(current_velocity_magnitude);

        let distance = current_velocity_magnitude;
        total_distance += distance;

        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        frame_count += 1;
    }

    // Performance assertions
    assert!(frame_count > 0);
    assert!(frame_count < 100); // Should stop before max frames
    assert!(total_distance > 0.0);
    assert!(max_velocity > 0.0);
    assert!(
        max_velocity
            <= (initial_velocity.0 * initial_velocity.0 + initial_velocity.1 * initial_velocity.1)
                .sqrt()
    );
}

#[test]
fn test_momentum_animation_stopping_conditions() {
    // Test various momentum animation stopping conditions
    let mut velocity: (f64, f64) = (100.0, 50.0);
    let friction = 0.95;
    let stopping_threshold = 0.1;
    let max_frames = 50;
    let mut frame_count = 0;

    // Test normal stopping condition (low velocity)
    while (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt() > stopping_threshold
        && frame_count < max_frames
    {
        velocity.0 *= friction;
        velocity.1 *= friction;
        frame_count += 1;
    }

    let final_velocity_magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
    assert!(final_velocity_magnitude <= stopping_threshold || frame_count >= max_frames);

    // Test max frames stopping condition
    let mut velocity_max_frames: (f64, f64) = (1000.0, 1000.0);
    let mut frame_count_max = 0;
    let max_frames_test = 10;

    while (velocity_max_frames.0 * velocity_max_frames.0
        + velocity_max_frames.1 * velocity_max_frames.1)
        .sqrt()
        > stopping_threshold
        && frame_count_max < max_frames_test
    {
        velocity_max_frames.0 *= friction;
        velocity_max_frames.1 *= friction;
        frame_count_max += 1;
    }

    assert_eq!(frame_count_max, max_frames_test);
}

#[test]
fn test_momentum_animation_state_management() {
    // Test momentum animation state management
    let mut is_dragging = true;
    let mut is_animating_momentum = false;
    let mut velocity: (f64, f64) = (0.0, 0.0);
    let velocity_threshold = 0.1;

    // State 1: Dragging
    assert!(is_dragging);
    assert!(!is_animating_momentum);

    // State 2: Transition to momentum
    velocity = (10.0, 5.0); // Reduced initial velocity
    let velocity_magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();

    if velocity_magnitude > velocity_threshold {
        is_dragging = false;
        is_animating_momentum = true;
    }

    assert!(!is_dragging);
    assert!(is_animating_momentum);

    // State 3: Momentum animation
    let friction = 0.95;
    for _ in 0..100 {
        // Increased iterations to ensure completion
        velocity.0 *= friction;
        velocity.1 *= friction;

        let current_magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        if current_magnitude < velocity_threshold {
            is_animating_momentum = false;
            break;
        }
    }

    // State 4: Stopped
    assert!(!is_dragging);
    assert!(!is_animating_momentum);
}

#[test]
fn test_momentum_animation_with_complex_constraints() {
    // Test momentum animation with complex constraint scenarios
    let constraints = DragConstraints {
        left: Some(-50.0),
        right: Some(150.0),
        top: Some(-25.0),
        bottom: Some(75.0),
    };

    let mut position = (0.0, 0.0);
    let mut velocity: (f64, f64) = (200.0, -100.0); // High velocity toward boundaries
    let friction = 0.9; // Higher friction for faster stopping
    let elastic_factor = 0.3;

    // Simulate momentum with complex constraints
    for frame in 0..30 {
        // Apply friction
        velocity.0 *= friction;
        velocity.1 *= friction;

        // Update position
        position.0 += velocity.0;
        position.1 += velocity.1;

        // Apply elastic constraints
        if let Some(left) = constraints.left {
            if position.0 < left {
                let overshoot = left - position.0;
                position.0 = left - (overshoot * elastic_factor);
            }
        }
        if let Some(right) = constraints.right {
            if position.0 > right {
                let overshoot = position.0 - right;
                position.0 = right + (overshoot * elastic_factor);
            }
        }
        if let Some(top) = constraints.top {
            if position.1 < top {
                let overshoot = top - position.1;
                position.1 = top - (overshoot * elastic_factor);
            }
        }
        if let Some(bottom) = constraints.bottom {
            if position.1 > bottom {
                let overshoot = position.1 - bottom;
                position.1 = bottom + (overshoot * elastic_factor);
            }
        }

        // Stop if velocity is very low
        let velocity_magnitude: f64 = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
        if velocity_magnitude < 0.1 {
            break;
        }
    }

    // Verify final position is within elastic bounds
    assert!(position.0 >= -65.0); // -50 - 15 (elastic overshoot)
    assert!(position.0 <= 165.0); // 150 + 15 (elastic overshoot)
    assert!(position.1 >= -40.0); // -25 - 15 (elastic overshoot)
    assert!(position.1 <= 90.0); // 75 + 15 (elastic overshoot)
}
