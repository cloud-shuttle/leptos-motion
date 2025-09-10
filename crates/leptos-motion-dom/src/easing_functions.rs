//! Easing Functions for Leptos Motion
//!
//! This module provides comprehensive easing functions for smooth animations.
//! Includes standard easing curves, cubic bezier curves, and spring physics.

// Global PI import removed - now imported in specific modules where needed

/// Standard easing functions
pub mod standard {
    // use super::*; // Unused

    /// Linear easing (no easing)
    pub fn linear(t: f64) -> f64 {
        t
    }

    /// Ease in (slow start)
    pub fn ease_in(t: f64) -> f64 {
        t * t
    }

    /// Ease out (slow end)
    pub fn ease_out(t: f64) -> f64 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease in out (slow start and end)
    pub fn ease_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }

    /// Ease in cubic
    pub fn ease_in_cubic(t: f64) -> f64 {
        t * t * t
    }

    /// Ease out cubic
    pub fn ease_out_cubic(t: f64) -> f64 {
        1.0 - (1.0 - t).powi(3)
    }

    /// Ease in out cubic
    pub fn ease_in_out_cubic(t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - 4.0 * (1.0 - t).powi(3)
        }
    }

    /// Ease in quartic
    pub fn ease_in_quartic(t: f64) -> f64 {
        t * t * t * t
    }

    /// Ease out quartic
    pub fn ease_out_quartic(t: f64) -> f64 {
        1.0 - (1.0 - t).powi(4)
    }

    /// Ease in out quartic
    pub fn ease_in_out_quartic(t: f64) -> f64 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - 8.0 * (1.0 - t).powi(4)
        }
    }

    /// Ease in quintic
    pub fn ease_in_quintic(t: f64) -> f64 {
        t * t * t * t * t
    }

    /// Ease out quintic
    pub fn ease_out_quintic(t: f64) -> f64 {
        1.0 - (1.0 - t).powi(5)
    }

    /// Ease in out quintic
    pub fn ease_in_out_quintic(t: f64) -> f64 {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - 16.0 * (1.0 - t).powi(5)
        }
    }
}

/// Sine-based easing functions
pub mod sine {
    use std::f64::consts::PI;

    /// Ease in sine
    pub fn ease_in_sine(t: f64) -> f64 {
        1.0 - ((t * PI) / 2.0).cos()
    }

    /// Ease out sine
    pub fn ease_out_sine(t: f64) -> f64 {
        ((t * PI) / 2.0).sin()
    }

    /// Ease in out sine
    pub fn ease_in_out_sine(t: f64) -> f64 {
        -((PI * t).cos() - 1.0) / 2.0
    }
}

/// Exponential easing functions
pub mod exponential {
    // use super::*; // Unused

    /// Ease in exponential
    pub fn ease_in_expo(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else {
            2.0_f64.powf(10.0 * (t - 1.0))
        }
    }

    /// Ease out exponential
    pub fn ease_out_expo(t: f64) -> f64 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f64.powf(-10.0 * t)
        }
    }

    /// Ease in out exponential
    pub fn ease_in_out_expo(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            2.0_f64.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
        }
    }
}

/// Circular easing functions
pub mod circular {
    // use super::*; // Unused

    /// Ease in circular
    pub fn ease_in_circ(t: f64) -> f64 {
        1.0 - (1.0 - t * t).sqrt()
    }

    /// Ease out circular
    pub fn ease_out_circ(t: f64) -> f64 {
        (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
    }

    /// Ease in out circular
    pub fn ease_in_out_circ(t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - (1.0 - (2.0 * t) * (2.0 * t)).sqrt()) / 2.0
        } else {
            (1.0 + (1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0)).sqrt()) / 2.0
        }
    }
}

/// Back easing functions (overshoot)
pub mod back {
    // use super::*; // Unused

    const C1: f64 = 1.70158;
    const C2: f64 = C1 * 1.525;
    const C3: f64 = C1 + 1.0;

    /// Ease in back
    pub fn ease_in_back(t: f64) -> f64 {
        C3 * t * t * t - C1 * t * t
    }

    /// Ease out back
    pub fn ease_out_back(t: f64) -> f64 {
        1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0) * (t - 1.0)
    }

    /// Ease in out back
    pub fn ease_in_out_back(t: f64) -> f64 {
        if t < 0.5 {
            ((2.0 * t) * (2.0 * t) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
        } else {
            ((2.0 * t - 2.0) * (2.0 * t - 2.0) * ((C2 + 1.0) * (2.0 * t - 2.0) + C2) + 2.0) / 2.0
        }
    }
}

/// Elastic easing functions (bounce-like)
pub mod elastic {
    use std::f64::consts::PI;

    const C4: f64 = (2.0 * PI) / 3.0;
    const C5: f64 = (2.0 * PI) / 4.5;

    /// Ease in elastic
    pub fn ease_in_elastic(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            -(2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin())
        }
    }

    /// Ease out elastic
    pub fn ease_out_elastic(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
        }
    }

    /// Ease in out elastic
    pub fn ease_in_out_elastic(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
        } else {
            (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
        }
    }
}

/// Bounce easing functions
pub mod bounce {
    // use super::*; // Unused

    const N1: f64 = 7.5625;
    const D1: f64 = 2.75;

    /// Ease in bounce
    pub fn ease_in_bounce(t: f64) -> f64 {
        1.0 - ease_out_bounce(1.0 - t)
    }

    /// Ease out bounce
    pub fn ease_out_bounce(t: f64) -> f64 {
        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            N1 * (t - 1.5 / D1) * (t - 1.5 / D1) + 0.75
        } else if t < 2.5 / D1 {
            N1 * (t - 2.25 / D1) * (t - 2.25 / D1) + 0.9375
        } else {
            N1 * (t - 2.625 / D1) * (t - 2.625 / D1) + 0.984375
        }
    }

    /// Ease in out bounce
    pub fn ease_in_out_bounce(t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
        }
    }
}

/// Cubic Bezier curve implementation
pub struct CubicBezier {
    #[allow(dead_code)]
    x1: f64,
    y1: f64,
    #[allow(dead_code)]
    x2: f64,
    y2: f64,
}

impl CubicBezier {
    /// Create a new cubic bezier curve
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Evaluate the bezier curve at time t
    pub fn evaluate(&self, t: f64) -> f64 {
        self.cubic_bezier_y(t)
    }

    /// Find the x value for a given t using Newton's method
    fn cubic_bezier_x(&self, t: f64) -> f64 {
        let mut t = t;
        for _ in 0..8 {
            let x = self.cubic_bezier_x_t(t);
            if (x - t).abs() < 1e-6 {
                break;
            }
            t = t - (x - t) / self.cubic_bezier_x_derivative(t);
        }
        t
    }

    /// Calculate x value for given t
    fn cubic_bezier_x_t(&self, t: f64) -> f64 {
        3.0 * (1.0 - t) * (1.0 - t) * t * self.x1 + 3.0 * (1.0 - t) * t * t * self.x2 + t * t * t
    }

    /// Calculate derivative of x with respect to t
    fn cubic_bezier_x_derivative(&self, t: f64) -> f64 {
        3.0 * (1.0 - t) * (1.0 - t) * self.x1
            + 6.0 * (1.0 - t) * t * (self.x2 - self.x1)
            + 3.0 * t * t * (1.0 - self.x2)
    }

    /// Calculate y value for given t
    fn cubic_bezier_y(&self, t: f64) -> f64 {
        3.0 * (1.0 - t) * (1.0 - t) * t * self.y1 + 3.0 * (1.0 - t) * t * t * self.y2 + t * t * t
    }
}

/// Predefined cubic bezier curves
pub mod bezier {
    use super::CubicBezier;

    /// Ease (0.25, 0.1, 0.25, 1.0)
    pub fn ease() -> CubicBezier {
        CubicBezier::new(0.25, 0.1, 0.25, 1.0)
    }

    /// Ease in (0.42, 0.0, 1.0, 1.0)
    pub fn ease_in() -> CubicBezier {
        CubicBezier::new(0.42, 0.0, 1.0, 1.0)
    }

    /// Ease out (0.0, 0.0, 0.58, 1.0)
    pub fn ease_out() -> CubicBezier {
        CubicBezier::new(0.0, 0.0, 0.58, 1.0)
    }

    /// Ease in out (0.42, 0.0, 0.58, 1.0)
    pub fn ease_in_out() -> CubicBezier {
        CubicBezier::new(0.42, 0.0, 0.58, 1.0)
    }

    /// Custom bezier curve
    pub fn custom(x1: f64, y1: f64, x2: f64, y2: f64) -> CubicBezier {
        CubicBezier::new(x1, y1, x2, y2)
    }
}

/// Spring physics implementation
pub mod spring {
    // use super::*; // Unused

    /// Spring configuration
    #[derive(Debug, Clone)]
    pub struct SpringConfig {
        /// Spring stiffness (higher = snappier)
        pub stiffness: f64,
        /// Damping (higher = less bouncy)
        pub damping: f64,
        /// Mass of the animated object
        pub mass: f64,
        /// Initial velocity
        pub velocity: f64,
        /// Rest delta threshold
        pub rest_delta: f64,
        /// Rest speed threshold
        pub rest_speed: f64,
    }

    impl Default for SpringConfig {
        fn default() -> Self {
            Self {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }
    }

    /// Spring animation state
    #[derive(Debug, Clone)]
    pub struct SpringState {
        /// Current position of the spring
        pub position: f64,
        /// Current velocity of the spring
        pub velocity: f64,
        /// Target position for the spring
        pub target: f64,
        /// Whether the spring animation is complete
        pub is_complete: bool,
    }

    impl SpringState {
        /// Create a new spring state with initial position and target
        pub fn new(initial: f64, target: f64) -> Self {
            Self {
                position: initial,
                velocity: 0.0,
                target,
                is_complete: false,
            }
        }
    }

    /// Update spring physics
    pub fn update_spring(state: &mut SpringState, config: &SpringConfig, delta_time: f64) {
        let distance = state.target - state.position;
        let spring_force = -config.stiffness * distance;
        let damping_force = -config.damping * state.velocity;
        let acceleration = (spring_force + damping_force) / config.mass;

        state.velocity += acceleration * delta_time;
        state.position += state.velocity * delta_time;

        // Check if animation is complete
        let velocity_threshold = config.rest_speed;
        let distance_threshold = config.rest_delta;

        if state.velocity.abs() < velocity_threshold && distance.abs() < distance_threshold {
            state.position = state.target;
            state.velocity = 0.0;
            state.is_complete = true;
        }
    }

    /// Predefined spring configurations
    pub mod presets {
        use super::SpringConfig;

        /// Gentle spring
        pub fn gentle() -> SpringConfig {
            SpringConfig {
                stiffness: 120.0,
                damping: 14.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }

        /// Wobbly spring
        pub fn wobbly() -> SpringConfig {
            SpringConfig {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }

        /// Stiff spring
        pub fn stiff() -> SpringConfig {
            SpringConfig {
                stiffness: 210.0,
                damping: 20.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }

        /// Slow spring
        pub fn slow() -> SpringConfig {
            SpringConfig {
                stiffness: 280.0,
                damping: 60.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }

        /// Bouncy spring
        pub fn bouncy() -> SpringConfig {
            SpringConfig {
                stiffness: 400.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
                rest_delta: 0.01,
                rest_speed: 0.01,
            }
        }
    }
}

/// Easing function trait for extensibility
pub trait EasingFunction {
    /// Evaluate the easing function at time t (0.0 to 1.0), returning the eased value
    fn evaluate(&self, t: f64) -> f64;
}

impl EasingFunction for fn(f64) -> f64 {
    fn evaluate(&self, t: f64) -> f64 {
        self(t)
    }
}

impl EasingFunction for CubicBezier {
    fn evaluate(&self, t: f64) -> f64 {
        self.evaluate(t)
    }
}

/// Apply any easing function to a progress value
pub fn apply_easing<F: EasingFunction>(progress: f64, easing: &F) -> f64 {
    easing.evaluate(progress)
}
