// Spring Physics Tests
//
// These tests define the expected behavior of the spring physics engine
// and will guide the implementation using Test-Driven Development.

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Spring configuration for physics-based animations
#[derive(Debug, Clone, PartialEq)]
pub struct SpringConfig {
    /// Spring tension (stiffness) - higher values = stiffer spring
    pub tension: f64,
    /// Spring friction (damping) - higher values = more damped
    pub friction: f64,
    /// Spring mass - higher values = heavier spring
    pub mass: f64,
    /// Initial velocity
    pub velocity: f64,
    /// Rest position
    pub rest: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            tension: 300.0,
            friction: 30.0,
            mass: 1.0,
            velocity: 0.0,
            rest: 0.0,
        }
    }
}

/// Spring physics engine for realistic animations
pub struct SpringPhysics {
    config: SpringConfig,
    position: f64,
    velocity: f64,
    time: f64,
}

impl SpringPhysics {
    pub fn new(config: SpringConfig) -> Self {
        Self {
            position: config.rest,
            velocity: config.velocity,
            time: 0.0,
            config,
        }
    }

    /// Update spring physics for one frame
    pub fn update(&mut self, delta_time: f64) -> f64 {
        self.time += delta_time;

        // Spring force calculation (Hooke's law)
        let spring_force = -self.config.tension * (self.position - self.config.rest);

        // Damping force (proportional to velocity)
        let damping_force = -self.config.friction * self.velocity;

        // Total force
        let total_force = spring_force + damping_force;

        // Update velocity and position using physics
        let acceleration = total_force / self.config.mass;
        self.velocity += acceleration * delta_time;
        self.position += self.velocity * delta_time;

        self.position
    }

    /// Check if spring is at rest (within tolerance)
    pub fn is_at_rest(&self, tolerance: f64) -> bool {
        (self.position - self.config.rest).abs() < tolerance && self.velocity.abs() < tolerance
    }

    /// Get current position
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Get current velocity
    pub fn velocity(&self) -> f64 {
        self.velocity
    }
}

/// Test basic spring physics behavior
#[test]
fn test_spring_physics_basic() {
    let config = SpringConfig {
        tension: 300.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 100.0,
    };

    let mut spring = SpringPhysics::new(config);

    // Spring should start at rest position
    assert_eq!(spring.position(), 100.0);
    assert_eq!(spring.velocity(), 0.0);

    // Displace spring and let it oscillate
    spring.position = 200.0;

    let mut positions = Vec::new();
    positions.push(spring.position()); // Add initial displaced position
    for _ in 0..100 {
        let pos = spring.update(0.016); // 60fps
        positions.push(pos);
    }

    // Spring should oscillate and eventually settle
    assert!(positions.len() == 101);
    assert!(positions[0] == 200.0); // Start displaced
    assert!(positions[100] < 200.0); // Should settle toward rest
}

/// Test spring with different tension values
#[test]
fn test_spring_physics_tension() {
    let mut high_tension = SpringPhysics::new(SpringConfig {
        tension: 1000.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    let mut low_tension = SpringPhysics::new(SpringConfig {
        tension: 100.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    // Displace both springs
    high_tension.position = 100.0;
    low_tension.position = 100.0;

    // High tension should settle faster
    for _ in 0..50 {
        high_tension.update(0.016);
        low_tension.update(0.016);
    }

    assert!(high_tension.position().abs() < low_tension.position().abs());
}

/// Test spring with different friction values
#[test]
fn test_spring_physics_friction() {
    let mut high_friction = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 50.0, // Moderate friction
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    let mut low_friction = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 20.0, // Lower friction
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    // Displace both springs
    high_friction.position = 100.0;
    low_friction.position = 100.0;

    // High friction should have less oscillation
    for _ in 0..200 {
        high_friction.update(0.016);
        low_friction.update(0.016);
    }

    // Both springs should be at rest after sufficient time
    let high_friction_distance = (high_friction.position() - 0.0).abs();
    let low_friction_distance = (low_friction.position() - 0.0).abs();

    // Both should be very close to rest position
    assert!(
        high_friction_distance < 0.1,
        "High friction spring should be at rest, distance: {}",
        high_friction_distance
    );
    assert!(
        low_friction_distance < 0.1,
        "Low friction spring should be at rest, distance: {}",
        low_friction_distance
    );

    // Both should have very low velocity (nearly at rest)
    assert!(
        high_friction.velocity().abs() < 0.1,
        "High friction spring should have low velocity: {}",
        high_friction.velocity().abs()
    );
    assert!(
        low_friction.velocity().abs() < 0.1,
        "Low friction spring should have low velocity: {}",
        low_friction.velocity().abs()
    );
}

/// Test spring with different mass values
#[test]
fn test_spring_physics_mass() {
    let mut light_mass = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 30.0,
        mass: 0.5,
        velocity: 0.0,
        rest: 0.0,
    });

    let mut heavy_mass = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 30.0,
        mass: 2.0,
        velocity: 0.0,
        rest: 0.0,
    });

    // Displace both springs
    light_mass.position = 100.0;
    heavy_mass.position = 100.0;

    // Light mass should respond faster
    for _ in 0..50 {
        light_mass.update(0.016);
        heavy_mass.update(0.016);
    }

    assert!(light_mass.position().abs() < heavy_mass.position().abs());
}

/// Test spring rest detection
#[test]
fn test_spring_physics_rest_detection() {
    let mut spring = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 50.0,
    });

    // Spring should start at rest
    assert!(spring.is_at_rest(0.1));

    // Displace spring
    spring.position = 100.0;
    assert!(!spring.is_at_rest(0.1));

    // Let it settle
    for _ in 0..200 {
        spring.update(0.016);
    }

    // Should be at rest again
    assert!(spring.is_at_rest(0.1));
}

/// Test spring with initial velocity
#[test]
fn test_spring_physics_initial_velocity() {
    let mut spring = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 100.0, // Initial velocity
        rest: 0.0,
    });

    // Spring should start with velocity
    assert_eq!(spring.velocity(), 100.0);

    // Should oscillate and settle
    for _ in 0..100 {
        spring.update(0.016);
    }

    // Should eventually settle
    assert!(spring.is_at_rest(0.1));
}

/// Test spring physics integration with MotionDiv
#[test]
fn test_spring_physics_motion_div_integration() {
    // This test will verify that spring physics can be integrated
    // with the MotionDiv component for smooth animations

    let config = SpringConfig {
        tension: 400.0,
        friction: 40.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    };

    let mut spring = SpringPhysics::new(config);

    // Simulate animation target
    let mut animation_target = HashMap::new();
    animation_target.insert("x".to_string(), AnimationValue::Number(0.0));

    // Update spring and verify smooth motion
    let mut positions = Vec::new();
    for _ in 0..60 {
        // 1 second at 60fps
        let pos = spring.update(0.016);
        positions.push(pos);
    }

    // Should have smooth, continuous motion
    assert!(positions.len() == 60);

    // Check for smoothness (no sudden jumps)
    for i in 1..positions.len() {
        let diff = (positions[i] - positions[i - 1]).abs();
        assert!(diff < 10.0, "Position jump too large: {}", diff);
    }
}

/// Test spring physics performance
#[test]
fn test_spring_physics_performance() {
    let config = SpringConfig::default();
    let mut springs: Vec<SpringPhysics> = (0..1000)
        .map(|_| SpringPhysics::new(config.clone()))
        .collect();

    let start_time = std::time::Instant::now();

    // Update 1000 springs for 100 frames
    for _ in 0..100 {
        for spring in &mut springs {
            spring.update(0.016);
        }
    }

    let duration = start_time.elapsed();

    // Should complete in reasonable time (< 10ms)
    assert!(
        duration.as_millis() < 10,
        "Spring physics too slow: {:?}",
        duration
    );
}

/// Test spring physics edge cases
#[test]
fn test_spring_physics_edge_cases() {
    // Test with zero tension (should not oscillate)
    let mut zero_tension = SpringPhysics::new(SpringConfig {
        tension: 0.0,
        friction: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    zero_tension.position = 100.0;
    zero_tension.update(0.016);

    // Should not move with zero tension
    assert_eq!(zero_tension.position(), 100.0);

    // Test with zero friction (should oscillate forever)
    let mut zero_friction = SpringPhysics::new(SpringConfig {
        tension: 300.0,
        friction: 0.0,
        mass: 1.0,
        velocity: 0.0,
        rest: 0.0,
    });

    zero_friction.position = 100.0;

    // Should oscillate
    for _ in 0..100 {
        zero_friction.update(0.016);
    }

    // Should still be oscillating (not at rest)
    assert!(!zero_friction.is_at_rest(0.1));
}

/// Test spring physics with different time steps
#[test]
fn test_spring_physics_time_steps() {
    let config = SpringConfig::default();
    let mut spring1 = SpringPhysics::new(config.clone());
    let mut spring2 = SpringPhysics::new(config);

    spring1.position = 100.0;
    spring2.position = 100.0;

    // Update with different time steps
    for _ in 0..50 {
        spring1.update(0.016); // 60fps
        spring2.update(0.033); // 30fps
    }

    // Both should be at rest (within tolerance)
    assert!(spring1.is_at_rest(0.1));
    assert!(spring2.is_at_rest(0.1));
}
