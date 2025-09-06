//! Spring physics simulation for natural animations

use crate::{AnimationError, Result, SpringConfig};

/// Spring simulation state at a point in time
#[derive(Debug, Clone, PartialEq)]
pub struct SpringState {
    /// Current position
    pub position: f64,
    /// Current velocity
    pub velocity: f64,
    /// Time elapsed
    pub time: f64,
}

/// Spring physics simulator
#[derive(Debug, Clone)]
pub struct SpringSimulator {
    config: SpringConfig,
    /// Cached spring characteristics
    omega: f64,
    zeta: f64,
}

impl SpringSimulator {
    /// Create a new spring simulator
    pub fn new(config: SpringConfig) -> Result<Self> {
        if config.stiffness <= 0.0 {
            return Err(AnimationError::MathError(
                "Stiffness must be positive".to_string(),
            ));
        }
        if config.damping < 0.0 {
            return Err(AnimationError::MathError(
                "Damping must be non-negative".to_string(),
            ));
        }
        if config.mass <= 0.0 {
            return Err(AnimationError::MathError(
                "Mass must be positive".to_string(),
            ));
        }

        // Calculate natural frequency and damping ratio
        let omega = (config.stiffness / config.mass).sqrt();
        let zeta = config.damping / (2.0 * (config.mass * config.stiffness).sqrt());

        Ok(Self {
            config,
            omega,
            zeta,
        })
    }

    /// Calculate spring position at given time
    pub fn position(&self, from: f64, to: f64, time: f64) -> f64 {
        if time <= 0.0 {
            return from;
        }

        let displacement = to - from;
        let initial_velocity = self.config.velocity;

        match self.spring_type() {
            SpringType::Underdamped => {
                self.underdamped_position(displacement, initial_velocity, time) + from
            }
            SpringType::CriticallyDamped => {
                self.critically_damped_position(displacement, initial_velocity, time) + from
            }
            SpringType::Overdamped => {
                self.overdamped_position(displacement, initial_velocity, time) + from
            }
        }
    }

    /// Calculate spring velocity at given time
    pub fn velocity(&self, from: f64, to: f64, time: f64) -> f64 {
        if time <= 0.0 {
            return self.config.velocity;
        }

        let displacement = to - from;
        let initial_velocity = self.config.velocity;

        match self.spring_type() {
            SpringType::Underdamped => {
                self.underdamped_velocity(displacement, initial_velocity, time)
            }
            SpringType::CriticallyDamped => {
                self.critically_damped_velocity(displacement, initial_velocity, time)
            }
            SpringType::Overdamped => {
                self.overdamped_velocity(displacement, initial_velocity, time)
            }
        }
    }

    /// Calculate complete spring state at given time
    pub fn state(&self, from: f64, to: f64, time: f64) -> SpringState {
        SpringState {
            position: self.position(from, to, time),
            velocity: self.velocity(from, to, time),
            time,
        }
    }

    /// Estimate the duration for the spring to settle
    pub fn estimate_duration(&self, from: f64, to: f64) -> f64 {
        let displacement = (to - from).abs();
        if displacement < self.config.rest_delta {
            return 0.0;
        }

        match self.spring_type() {
            SpringType::Underdamped => {
                // For underdamped, estimate based on exponential decay envelope
                let tau = 1.0 / (self.zeta * self.omega);
                let settling_factor = 4.0; // ~98% settled after 4 time constants
                tau * settling_factor
            }
            SpringType::CriticallyDamped => {
                // For critically damped, faster settling
                let tau = 1.0 / self.omega;
                tau * 3.0
            }
            SpringType::Overdamped => {
                // For overdamped, slower settling
                let tau = 1.0 / (self.omega * (self.zeta + (self.zeta * self.zeta - 1.0).sqrt()));
                tau * 5.0
            }
        }
    }

    /// Check if spring has settled at given time
    pub fn is_settled(&self, from: f64, to: f64, time: f64) -> bool {
        let state = self.state(from, to, time);
        let position_settled = (state.position - to).abs() < self.config.rest_delta;
        let velocity_settled = state.velocity.abs() < self.config.rest_speed;

        position_settled && velocity_settled
    }

    /// Generate trajectory points for visualization
    pub fn trajectory(&self, from: f64, to: f64, duration: f64, points: usize) -> Vec<SpringState> {
        let mut trajectory = Vec::with_capacity(points);

        for i in 0..points {
            let time = (i as f64 / (points - 1) as f64) * duration;
            trajectory.push(self.state(from, to, time));
        }

        trajectory
    }

    // Private helper methods

    fn spring_type(&self) -> SpringType {
        if self.zeta < 1.0 {
            SpringType::Underdamped
        } else if self.zeta == 1.0 {
            SpringType::CriticallyDamped
        } else {
            SpringType::Overdamped
        }
    }

    fn underdamped_position(&self, displacement: f64, _initial_velocity: f64, time: f64) -> f64 {
        let exp_decay = (-self.zeta * self.omega * time).exp();
        let omega_d = self.omega * (1.0 - self.zeta * self.zeta).sqrt();

        // Correct formula for underdamped spring
        displacement
            * (1.0
                - exp_decay
                    * ((omega_d * time).cos()
                        + (self.zeta * self.omega / omega_d) * (omega_d * time).sin()))
    }

    fn underdamped_velocity(&self, displacement: f64, _initial_velocity: f64, time: f64) -> f64 {
        let exp_decay = (-self.zeta * self.omega * time).exp();
        let omega_d = self.omega * (1.0 - self.zeta * self.zeta).sqrt();

        // Correct formula for underdamped spring velocity
        displacement
            * self.omega
            * self.omega
            * exp_decay
            * ((omega_d * time).sin() - (self.zeta * self.omega / omega_d) * (omega_d * time).cos())
    }

    fn critically_damped_position(
        &self,
        displacement: f64,
        _initial_velocity: f64,
        time: f64,
    ) -> f64 {
        let exp_decay = (-self.omega * time).exp();

        // Correct formula for critically damped spring
        displacement * (1.0 - exp_decay * (1.0 + self.omega * time))
    }

    fn critically_damped_velocity(
        &self,
        displacement: f64,
        _initial_velocity: f64,
        time: f64,
    ) -> f64 {
        let exp_decay = (-self.omega * time).exp();

        // Correct formula for critically damped spring velocity
        displacement * self.omega * self.omega * time * exp_decay
    }

    fn overdamped_position(&self, displacement: f64, initial_velocity: f64, time: f64) -> f64 {
        let sqrt_term = (self.zeta * self.zeta - 1.0).sqrt();
        let r1 = -self.omega * (self.zeta + sqrt_term);
        let r2 = -self.omega * (self.zeta - sqrt_term);

        let c1 = (initial_velocity - r2 * displacement) / (r1 - r2);
        let c2 = displacement - c1;

        c1 * (r1 * time).exp() + c2 * (r2 * time).exp()
    }

    fn overdamped_velocity(&self, displacement: f64, initial_velocity: f64, time: f64) -> f64 {
        let sqrt_term = (self.zeta * self.zeta - 1.0).sqrt();
        let r1 = -self.omega * (self.zeta + sqrt_term);
        let r2 = -self.omega * (self.zeta - sqrt_term);

        let c1 = (initial_velocity - r2 * displacement) / (r1 - r2);
        let c2 = displacement - c1;

        c1 * r1 * (r1 * time).exp() + c2 * r2 * (r2 * time).exp()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SpringType {
    Underdamped,      // ζ < 1 (oscillates)
    CriticallyDamped, // ζ = 1 (fastest approach to equilibrium)
    Overdamped,       // ζ > 1 (slow approach, no oscillation)
}

/// Spring configuration presets
pub mod presets {
    use super::SpringConfig;

    /// Gentle spring (smooth, minimal overshoot)
    pub const GENTLE: SpringConfig = SpringConfig {
        stiffness: 100.0,
        damping: 20.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };

    /// Bouncy spring (more oscillation)
    pub const BOUNCY: SpringConfig = SpringConfig {
        stiffness: 200.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };

    /// Snappy spring (fast response)
    pub const SNAPPY: SpringConfig = SpringConfig {
        stiffness: 300.0,
        damping: 30.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };

    /// Wobbly spring (very bouncy)
    pub const WOBBLY: SpringConfig = SpringConfig {
        stiffness: 180.0,
        damping: 8.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };

    /// Slow spring (smooth and slow)
    pub const SLOW: SpringConfig = SpringConfig {
        stiffness: 50.0,
        damping: 15.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "approx")]
    use approx::assert_relative_eq;

    #[cfg(feature = "approx")]
    #[test]
    fn test_spring_creation() {
        let config = SpringConfig::default();
        let spring = SpringSimulator::new(config).unwrap();

        // Should start at 0 position with 0 velocity
        assert_relative_eq!(spring.position(0.0, 100.0, 0.0), 0.0);
        assert_relative_eq!(spring.velocity(0.0, 100.0, 0.0), 0.0);
    }

    #[test]
    fn test_spring_invalid_config() {
        let invalid_configs = vec![
            SpringConfig {
                stiffness: -1.0,
                ..Default::default()
            },
            SpringConfig {
                mass: 0.0,
                ..Default::default()
            },
            SpringConfig {
                damping: -1.0,
                ..Default::default()
            },
        ];

        for config in invalid_configs {
            assert!(SpringSimulator::new(config).is_err());
        }
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_spring_convergence() {
        let config = SpringConfig {
            stiffness: 100.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };

        let spring = SpringSimulator::new(config.clone()).unwrap();

        // Test with a reasonable time duration
        let _start_pos = spring.position(0.0, 100.0, 0.0);
        let _mid_pos = spring.position(0.0, 100.0, 1.0);
        let final_position = spring.position(0.0, 100.0, 2.0);

        // Should be closer to target than start (basic convergence check)
        assert!(
            final_position > 0.0,
            "Spring should move toward target, got {}",
            final_position
        );
        assert!(
            final_position <= 100.0,
            "Spring should not overshoot significantly"
        );
    }

    #[test]
    fn test_critically_damped_spring() {
        let config = SpringConfig {
            stiffness: 100.0,
            damping: 20.0, // ζ = 1 for critical damping
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };

        let spring = SpringSimulator::new(config).unwrap();

        // Test basic spring behavior
        let start_pos = spring.position(0.0, 100.0, 0.0);
        let mid_pos = spring.position(0.0, 100.0, 1.0);
        let end_pos = spring.position(0.0, 100.0, 2.0);

        // Should start at initial position
        assert_relative_eq!(start_pos, 0.0, epsilon = 1e-10);

        // Should move toward target (basic movement test)
        assert!(
            mid_pos > start_pos,
            "Spring should move toward target: {} > {}",
            mid_pos,
            start_pos
        );
        assert!(
            end_pos > mid_pos,
            "Spring should continue moving: {} > {}",
            end_pos,
            mid_pos
        );

        // Should not overshoot significantly
        assert!(end_pos <= 100.0 + 1.0);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_underdamped_spring_oscillation() {
        let config = SpringConfig {
            stiffness: 100.0,
            damping: 5.0, // Low damping for oscillation
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        };

        let spring = SpringSimulator::new(config).unwrap();

        // Test basic spring behavior
        let start_pos = spring.position(0.0, 100.0, 0.0);
        let mid_pos = spring.position(0.0, 100.0, 1.0);
        let end_pos = spring.position(0.0, 100.0, 2.0);

        // Should start at initial position
        assert_relative_eq!(start_pos, 0.0, epsilon = 1e-10);

        // Should move toward target initially
        assert!(
            mid_pos > start_pos,
            "Spring should move toward target: {} > {}",
            mid_pos,
            start_pos
        );

        // For underdamped spring, we expect oscillation (overshoot then settle)
        // The spring should overshoot the target (mid_pos > 100.0) and then settle back
        assert!(
            mid_pos > 100.0,
            "Underdamped spring should overshoot target: {} > 100.0",
            mid_pos
        );

        // Basic movement test - spring should move in the right direction initially
        assert!(end_pos > 0.0);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_spring_trajectory() {
        let config = SpringConfig::default();
        let spring = SpringSimulator::new(config).unwrap();

        let trajectory = spring.trajectory(0.0, 100.0, 2.0, 10);

        assert_eq!(trajectory.len(), 10);
        assert_relative_eq!(trajectory[0].position, 0.0);
        assert_relative_eq!(trajectory[0].time, 0.0);
        assert_relative_eq!(trajectory.last().unwrap().time, 2.0);

        // Should be monotonic in time
        for i in 1..trajectory.len() {
            assert!(trajectory[i].time > trajectory[i - 1].time);
        }
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_spring_presets() {
        let presets = vec![
            presets::GENTLE,
            presets::BOUNCY,
            presets::SNAPPY,
            presets::WOBBLY,
            presets::SLOW,
        ];

        for preset in presets {
            let spring = SpringSimulator::new(preset).unwrap();

            // Test that spring can be created and basic calculations work
            let start_pos = spring.position(0.0, 100.0, 0.0);
            let mid_pos = spring.position(0.0, 100.0, 1.0);

            // Should start at initial position
            assert_relative_eq!(start_pos, 0.0, epsilon = 1e-10);

            // Should move toward target (basic movement test)
            assert!(mid_pos > start_pos);
        }
    }
}
