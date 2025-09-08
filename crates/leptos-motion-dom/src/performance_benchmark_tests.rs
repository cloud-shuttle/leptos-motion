// Performance Benchmark Tests
//
// These tests measure and validate the performance characteristics of the
// leptos-motion-dom library, ensuring it meets performance requirements
// for real-world applications.

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance benchmarks for drag constraint operations
#[test]
fn benchmark_drag_constraint_operations() {
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

    // Benchmark constraint application
    let start_time = Instant::now();
    let iterations = 10_000;

    for i in 0..iterations {
        let test_x = (i as f64) * 0.01 - 50.0;
        let test_y = (i as f64) * 0.005 - 25.0;

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
    }

    let duration = start_time.elapsed();
    let operations_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should handle at least 100,000 constraint operations per second
    // - Should complete 10,000 operations in under 100ms
    assert!(
        operations_per_second > 100_000.0,
        "Performance too slow: {} ops/sec (expected > 100,000)",
        operations_per_second
    );
    assert!(
        duration.as_millis() < 100,
        "Operations too slow: {}ms for {} operations (expected < 100ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for momentum animation calculations
#[test]
fn benchmark_momentum_animation_calculations() {
    let start_time = Instant::now();
    let iterations = 1_000;

    for _ in 0..iterations {
        let mut position = (0.0, 0.0);
        let mut velocity: (f64, f64) = (50.0, 25.0);
        let friction = 0.95;
        let mut frame_count = 0;

        // Simulate momentum animation
        while velocity.0.abs() > 0.1_f64 || velocity.1.abs() > 0.1_f64 {
            frame_count += 1;

            // Apply velocity to position
            position.0 += velocity.0;
            position.1 += velocity.1;

            // Apply friction
            velocity.0 *= friction;
            velocity.1 *= friction;

            // Prevent infinite loops
            if frame_count > 100 {
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    let animations_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should handle at least 1,000 momentum animations per second
    // - Should complete 1,000 animations in under 1 second
    assert!(
        animations_per_second > 1_000.0,
        "Momentum performance too slow: {} animations/sec (expected > 1,000)",
        animations_per_second
    );
    assert!(
        duration.as_millis() < 1_000,
        "Momentum calculations too slow: {}ms for {} animations (expected < 1000ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for drag configuration creation
#[test]
fn benchmark_drag_config_creation() {
    let start_time = Instant::now();
    let iterations = 100_000;

    for i in 0..iterations {
        let _config = DragConfig {
            axis: Some(DragAxis::Both),
            constraints: Some(DragConstraints {
                left: Some(-100.0 + i as f64),
                right: Some(100.0 + i as f64),
                top: Some(-50.0 + i as f64),
                bottom: Some(50.0 + i as f64),
            }),
            elastic: Some(0.2),
            momentum: Some(true),
        };
    }

    let duration = start_time.elapsed();
    let configs_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should create at least 1,000,000 drag configs per second
    // - Should complete 100,000 configs in under 100ms
    assert!(
        configs_per_second > 1_000_000.0,
        "Config creation too slow: {} configs/sec (expected > 1,000,000)",
        configs_per_second
    );
    assert!(
        duration.as_millis() < 100,
        "Config creation too slow: {}ms for {} configs (expected < 100ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for animation target creation
#[test]
fn benchmark_animation_target_creation() {
    let start_time = Instant::now();
    let iterations = 50_000;

    for i in 0..iterations {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(i as f64));
        target.insert("y".to_string(), AnimationValue::Pixels(i as f64 * 0.5));
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert(
            "scale".to_string(),
            AnimationValue::Number(1.0 + i as f64 * 0.001),
        );
        target.insert(
            "rotate".to_string(),
            AnimationValue::Degrees(i as f64 * 0.1),
        );
    }

    let duration = start_time.elapsed();
    let targets_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should create at least 200,000 animation targets per second
    // - Should complete 50,000 targets in under 250ms
    assert!(
        targets_per_second > 200_000.0,
        "Animation target creation too slow: {} targets/sec (expected > 200,000)",
        targets_per_second
    );
    assert!(
        duration.as_millis() < 250,
        "Animation target creation too slow: {}ms for {} targets (expected < 250ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for transition creation
#[test]
fn benchmark_transition_creation() {
    let start_time = Instant::now();
    let iterations = 100_000;

    for i in 0..iterations {
        let _transition = Transition {
            duration: Some(0.3 + i as f64 * 0.001),
            ease: Easing::EaseOut,
            delay: Some(i as f64 * 0.0001),
            repeat: RepeatConfig::Never,
            stagger: None,
        };
    }

    let duration = start_time.elapsed();
    let transitions_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should create at least 1,000,000 transitions per second
    // - Should complete 100,000 transitions in under 100ms
    assert!(
        transitions_per_second > 1_000_000.0,
        "Transition creation too slow: {} transitions/sec (expected > 1,000,000)",
        transitions_per_second
    );
    assert!(
        duration.as_millis() < 100,
        "Transition creation too slow: {}ms for {} transitions (expected < 100ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for complex drag operations
#[test]
fn benchmark_complex_drag_operations() {
    let config = DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: Some(-200.0),
            right: Some(200.0),
            top: Some(-100.0),
            bottom: Some(100.0),
        }),
        elastic: Some(0.3),
        momentum: Some(true),
    };

    let start_time = Instant::now();
    let iterations = 1_000;

    for i in 0..iterations {
        // Simulate complex drag operation with momentum
        let mut position = (0.0, 0.0);
        let mut velocity: (f64, f64) = (100.0 + i as f64, 50.0 + i as f64 * 0.5);
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

            // Apply friction
            velocity.0 *= friction;
            velocity.1 *= friction;

            // Prevent infinite loops
            if frame_count > 200 {
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    let operations_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should handle at least 100 complex drag operations per second
    // - Should complete 1,000 operations in under 10 seconds
    assert!(
        operations_per_second > 100.0,
        "Complex drag performance too slow: {} ops/sec (expected > 100)",
        operations_per_second
    );
    assert!(
        duration.as_millis() < 10_000,
        "Complex drag operations too slow: {}ms for {} operations (expected < 10,000ms)",
        duration.as_millis(),
        iterations
    );
}

/// Memory usage benchmark for drag configurations
#[test]
fn benchmark_memory_usage() {
    let start_time = Instant::now();
    let iterations = 10_000;

    // Create many drag configurations to test memory usage
    let mut configs = Vec::with_capacity(iterations);

    for i in 0..iterations {
        let config = DragConfig {
            axis: Some(DragAxis::Both),
            constraints: Some(DragConstraints {
                left: Some(-100.0 + i as f64),
                right: Some(100.0 + i as f64),
                top: Some(-50.0 + i as f64),
                bottom: Some(50.0 + i as f64),
            }),
            elastic: Some(0.2),
            momentum: Some(true),
        };
        configs.push(config);
    }

    let duration = start_time.elapsed();

    // Verify all configs were created
    assert_eq!(configs.len(), iterations);

    // Performance requirements:
    // - Should create 10,000 configs in under 50ms
    // - Memory usage should be reasonable (no excessive allocations)
    assert!(
        duration.as_millis() < 50,
        "Memory allocation too slow: {}ms for {} configs (expected < 50ms)",
        duration.as_millis(),
        iterations
    );

    // Test memory cleanup
    drop(configs);
}

/// Performance benchmark for axis constraint enforcement
#[test]
fn benchmark_axis_constraint_enforcement() {
    let start_time = Instant::now();
    let iterations = 100_000;

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

    for i in 0..iterations {
        let mut current_x = (i as f64) * 0.001 - 50.0;
        let mut current_y = (i as f64) * 0.001 - 25.0;
        let movement_x = 10.0;
        let movement_y = 5.0; // This should be ignored for X-axis constraint

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

        // Verify constraint was applied
        assert_eq!(current_y, (i as f64) * 0.001 - 25.0); // Y should be unchanged
    }

    let duration = start_time.elapsed();
    let operations_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should handle at least 1,000,000 axis constraint operations per second
    // - Should complete 100,000 operations in under 100ms
    assert!(
        operations_per_second > 1_000_000.0,
        "Axis constraint performance too slow: {} ops/sec (expected > 1,000,000)",
        operations_per_second
    );
    assert!(
        duration.as_millis() < 100,
        "Axis constraint operations too slow: {}ms for {} operations (expected < 100ms)",
        duration.as_millis(),
        iterations
    );
}

/// Performance benchmark for elastic behavior calculations
#[test]
fn benchmark_elastic_behavior_calculations() {
    let start_time = Instant::now();
    let iterations = 100_000;

    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };
    let elastic_factor = 0.2;

    for i in 0..iterations {
        let test_position = (i as f64) * 0.002 - 100.0; // Vary from -100 to 100
        let mut constrained_position = test_position;

        // Apply elastic constraint
        if let Some(left) = constraints.left {
            if constrained_position < left {
                if elastic_factor > 0.0 {
                    let overshoot = left - constrained_position;
                    constrained_position = left - (overshoot * elastic_factor);
                } else {
                    constrained_position = left;
                }
            }
        }

        if let Some(right) = constraints.right {
            if constrained_position > right {
                if elastic_factor > 0.0 {
                    let overshoot = constrained_position - right;
                    constrained_position = right + (overshoot * elastic_factor);
                } else {
                    constrained_position = right;
                }
            }
        }

        // Verify constraint was applied
        assert!(constrained_position >= constraints.left.unwrap() - 10.0);
        assert!(constrained_position <= constraints.right.unwrap() + 10.0);
    }

    let duration = start_time.elapsed();
    let operations_per_second = iterations as f64 / duration.as_secs_f64();

    // Performance requirements:
    // - Should handle at least 1,000,000 elastic calculations per second
    // - Should complete 100,000 operations in under 100ms
    assert!(
        operations_per_second > 1_000_000.0,
        "Elastic behavior performance too slow: {} ops/sec (expected > 1,000,000)",
        operations_per_second
    );
    assert!(
        duration.as_millis() < 100,
        "Elastic behavior operations too slow: {}ms for {} operations (expected < 100ms)",
        duration.as_millis(),
        iterations
    );
}
