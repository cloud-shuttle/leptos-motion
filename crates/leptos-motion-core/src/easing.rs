//! Easing functions for smooth animation transitions

use crate::{Easing, SpringConfig};

/// Easing function type
pub type EasingFn = fn(f64) -> f64;

impl Easing {
    /// Evaluate the easing function at time t (0.0 to 1.0)
    pub fn evaluate(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => linear(t),
            Easing::EaseIn => ease_in_quad(t),
            Easing::EaseOut => ease_out_quad(t),
            Easing::EaseInOut => ease_in_out_quad(t),
            Easing::CircIn => circ_in(t),
            Easing::CircOut => circ_out(t),
            Easing::CircInOut => circ_in_out(t),
            Easing::BackIn => back_in(t),
            Easing::BackOut => back_out(t),
            Easing::BackInOut => back_in_out(t),
            Easing::Spring(config) => {
                // For UI purposes, we approximate spring with a ease-out curve
                // Real spring physics is handled by the spring module
                let factor = (config.stiffness / 100.0).clamp(0.5, 2.0);
                ease_out_expo(t).powf(1.0 / factor)
            }
            Easing::Bezier(x1, y1, x2, y2) => cubic_bezier(*x1, *y1, *x2, *y2, t),
        }
    }

    /// Get a function pointer for this easing
    pub fn as_fn(&self) -> Box<dyn Fn(f64) -> f64> {
        match self {
            Easing::Linear => Box::new(linear),
            Easing::EaseIn => Box::new(ease_in_quad),
            Easing::EaseOut => Box::new(ease_out_quad),
            Easing::EaseInOut => Box::new(ease_in_out_quad),
            Easing::CircIn => Box::new(circ_in),
            Easing::CircOut => Box::new(circ_out),
            Easing::CircInOut => Box::new(circ_in_out),
            Easing::BackIn => Box::new(back_in),
            Easing::BackOut => Box::new(back_out),
            Easing::BackInOut => Box::new(back_in_out),
            Easing::Spring(config) => {
                let factor = (config.stiffness / 100.0).clamp(0.5, 2.0);
                Box::new(move |t| ease_out_expo(t).powf(1.0 / factor))
            }
            Easing::Bezier(x1, y1, x2, y2) => {
                let x1 = *x1;
                let y1 = *y1;
                let x2 = *x2;
                let y2 = *y2;
                Box::new(move |t| cubic_bezier(x1, y1, x2, y2, t))
            }
        }
    }
}

// Basic easing functions

/// Linear interpolation (no easing)
pub fn linear(t: f64) -> f64 {
    t
}

/// Quadratic ease in (accelerating from zero velocity)
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Quadratic ease out (decelerating to zero velocity)
pub fn ease_out_quad(t: f64) -> f64 {
    t * (2.0 - t)
}

/// Quadratic ease in and out
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Cubic ease in
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// Cubic ease out
pub fn ease_out_cubic(t: f64) -> f64 {
    let t = t - 1.0;
    t * t * t + 1.0
}

/// Cubic ease in and out
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        1.0 + t * t * t / 2.0
    }
}

/// Exponential ease out
pub fn ease_out_expo(t: f64) -> f64 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

// Circular easing functions

/// Circular ease in
pub fn circ_in(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease out
pub fn circ_out(t: f64) -> f64 {
    let t = t - 1.0;
    (1.0 - t * t).sqrt()
}

/// Circular ease in and out
pub fn circ_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
    } else {
        let t = -2.0 * t + 2.0;
        ((1.0 - t * t).sqrt() + 1.0) / 2.0
    }
}

// Back easing functions (overshoot)

const BACK_CONSTANT: f64 = 1.70158;

/// Back ease in (overshoot at start)
pub fn back_in(t: f64) -> f64 {
    let c1 = BACK_CONSTANT;
    let c3 = c1 + 1.0;
    c3 * t * t * t - c1 * t * t
}

/// Back ease out (overshoot at end)
pub fn back_out(t: f64) -> f64 {
    let c1 = BACK_CONSTANT;
    let c3 = c1 + 1.0;
    let t = t - 1.0;
    1.0 + c3 * t * t * t + c1 * t * t
}

/// Back ease in and out (overshoot at both ends)
pub fn back_in_out(t: f64) -> f64 {
    let c1 = BACK_CONSTANT;
    let c2 = c1 * 1.525;

    if t < 0.5 {
        let t = 2.0 * t;
        (t * t * ((c2 + 1.0) * t - c2)) / 2.0
    } else {
        let t = 2.0 * t - 2.0;
        (t * t * ((c2 + 1.0) * t + c2) + 2.0) / 2.0
    }
}

/// Cubic bezier easing function
pub fn cubic_bezier(_x1: f64, y1: f64, _x2: f64, y2: f64, t: f64) -> f64 {
    // Simplified cubic bezier approximation
    // For production, we'd use Newton-Raphson method for precision
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let _mt3 = mt2 * mt;

    // Cubic bezier curve: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
    // P₀ = (0,0), P₃ = (1,1), P₁ = (x1,y1), P₂ = (x2,y2)
    3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
}

/// Common easing presets
pub mod presets {
    use super::*;

    /// Material Design ease
    pub const EASE: Easing = Easing::Bezier(0.4, 0.0, 0.2, 1.0);

    /// Material Design ease in
    pub const EASE_IN: Easing = Easing::Bezier(0.4, 0.0, 1.0, 1.0);

    /// Material Design ease out
    pub const EASE_OUT: Easing = Easing::Bezier(0.0, 0.0, 0.2, 1.0);

    /// Material Design ease in out
    pub const EASE_IN_OUT: Easing = Easing::Bezier(0.4, 0.0, 0.2, 1.0);

    /// Smooth spring
    pub const SPRING_SMOOTH: Easing = Easing::Spring(SpringConfig {
        stiffness: 100.0,
        damping: 20.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    });

    /// Bouncy spring
    pub const SPRING_BOUNCY: Easing = Easing::Spring(SpringConfig {
        stiffness: 200.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    });

    /// Gentle spring
    pub const SPRING_GENTLE: Easing = Easing::Spring(SpringConfig {
        stiffness: 50.0,
        damping: 15.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "approx")]
    use approx::assert_relative_eq;

    #[test]
    fn test_linear_easing() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_ease_bounds() {
        let easings = vec![
            ease_in_quad,
            ease_out_quad,
            ease_in_out_quad,
            ease_in_cubic,
            ease_out_cubic,
            ease_in_out_cubic,
            circ_in,
            circ_out,
            circ_in_out,
        ];

        for easing in easings {
            // All easing functions should start at 0 and end at 1
            assert_relative_eq!(easing(0.0), 0.0, epsilon = 1e-10);
            assert_relative_eq!(easing(1.0), 1.0, epsilon = 1e-10);

            // Should be monotonic (non-decreasing)
            let mut prev = easing(0.0);
            for i in 1..=10 {
                let t = i as f64 / 10.0;
                let current = easing(t);
                assert!(current >= prev - 1e-10, "Easing function not monotonic");
                prev = current;
            }
        }
    }

    #[test]
    fn test_back_easing_overshoot() {
        // Back easing should overshoot (go beyond 1.0 before settling)
        let mut max: f64 = 0.0;
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let value = back_out(t);
            max = max.max(value);
        }
        assert!(max > 1.0, "Back easing should overshoot");
    }

    #[cfg(feature = "approx")]
    #[test]
    fn test_cubic_bezier() {
        // Standard ease
        let ease = |t| cubic_bezier(0.25, 0.1, 0.25, 1.0, t);
        assert_relative_eq!(ease(0.0), 0.0, epsilon = 1e-3);
        assert_relative_eq!(ease(1.0), 1.0, epsilon = 1e-3);
    }

    #[test]
    fn test_easing_evaluate() {
        let easing = Easing::EaseInOut;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(1.0), 1.0);

        // Test clamping
        assert_eq!(easing.evaluate(-0.1), 0.0);
        assert_eq!(easing.evaluate(1.1), 1.0);
    }
}
