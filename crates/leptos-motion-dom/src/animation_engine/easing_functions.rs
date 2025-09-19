//! Easing functions for smooth animation transitions

use leptos_motion_core::Easing;

/// Easing function utilities
pub struct EasingFunctions;

impl EasingFunctions {
    /// Apply easing function to progress value
    pub fn apply_easing(progress: f64, easing: &Easing) -> f64 {
        let t = progress.clamp(0.0, 1.0);
        
        match easing {
            Easing::Linear => t,
            Easing::EaseIn => Self::ease_in(t),
            Easing::EaseOut => Self::ease_out(t),
            Easing::EaseInOut => Self::ease_in_out(t),
            Easing::CircIn => Self::ease_in_circ(t),
            Easing::CircOut => Self::ease_out_circ(t),
            Easing::CircInOut => Self::ease_in_out_circ(t),
            Easing::BackIn => Self::ease_in_back(t),
            Easing::BackOut => Self::ease_out_back(t),
            Easing::BackInOut => Self::ease_in_out_back(t),
            Easing::Spring(_) => t, // Handled separately in spring physics
            Easing::Bezier(x1, y1, x2, y2) => Self::cubic_bezier(0.0, *x1, *x2, 1.0, t),
            Easing::CubicBezier(cb) => Self::cubic_bezier(0.0, cb.0, cb.2, 1.0, t),
        }
    }

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

    /// Ease in-out (slow start and end)
    pub fn ease_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }

    /// Ease in sine
    pub fn ease_in_sine(t: f64) -> f64 {
        1.0 - (t * std::f64::consts::PI / 2.0).cos()
    }

    /// Ease out sine
    pub fn ease_out_sine(t: f64) -> f64 {
        (t * std::f64::consts::PI / 2.0).sin()
    }

    /// Ease in-out sine
    pub fn ease_in_out_sine(t: f64) -> f64 {
        -(std::f64::consts::PI * t).cos() / 2.0 + 0.5
    }

    /// Ease in quad
    pub fn ease_in_quad(t: f64) -> f64 {
        t * t
    }

    /// Ease out quad
    pub fn ease_out_quad(t: f64) -> f64 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease in-out quad
    pub fn ease_in_out_quad(t: f64) -> f64 {
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

    /// Ease in-out cubic
    pub fn ease_in_out_cubic(t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - 4.0 * (1.0 - t).powi(3)
        }
    }

    /// Ease in quart
    pub fn ease_in_quart(t: f64) -> f64 {
        t * t * t * t
    }

    /// Ease out quart
    pub fn ease_out_quart(t: f64) -> f64 {
        1.0 - (1.0 - t).powi(4)
    }

    /// Ease in-out quart
    pub fn ease_in_out_quart(t: f64) -> f64 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - 8.0 * (1.0 - t).powi(4)
        }
    }

    /// Ease in quint
    pub fn ease_in_quint(t: f64) -> f64 {
        t * t * t * t * t
    }

    /// Ease out quint
    pub fn ease_out_quint(t: f64) -> f64 {
        1.0 - (1.0 - t).powi(5)
    }

    /// Ease in-out quint
    pub fn ease_in_out_quint(t: f64) -> f64 {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - 16.0 * (1.0 - t).powi(5)
        }
    }

    /// Ease in expo
    pub fn ease_in_expo(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else {
            2.0_f64.powf(10.0 * (t - 1.0))
        }
    }

    /// Ease out expo
    pub fn ease_out_expo(t: f64) -> f64 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f64.powf(-10.0 * t)
        }
    }

    /// Ease in-out expo
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

    /// Ease in circ
    pub fn ease_in_circ(t: f64) -> f64 {
        1.0 - (1.0 - t * t).sqrt()
    }

    /// Ease out circ
    pub fn ease_out_circ(t: f64) -> f64 {
        ((2.0 - t) * t).sqrt()
    }

    /// Ease in-out circ
    pub fn ease_in_out_circ(t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
        } else {
            (1.0 + (1.0 - 4.0 * (1.0 - t) * (1.0 - t)).sqrt()) / 2.0
        }
    }

    /// Ease in back
    pub fn ease_in_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.0;
        C3 * t * t * t - C1 * t * t
    }

    /// Ease out back
    pub fn ease_out_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.0;
        1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0) * (t - 1.0)
    }

    /// Ease in-out back
    pub fn ease_in_out_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C2: f64 = C1 * 1.525;
        
        if t < 0.5 {
            (2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * t - 2.0) + C2) + 2.0) / 2.0
        }
    }

    /// Ease in elastic
    pub fn ease_in_elastic(t: f64) -> f64 {
        const C4: f64 = (2.0 * std::f64::consts::PI) / 3.0;
        
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            -(2.0_f64.powf(10.0 * t - 10.0)) * (t * 10.0 - 10.75).sin() * C4
        }
    }

    /// Ease out elastic
    pub fn ease_out_elastic(t: f64) -> f64 {
        const C4: f64 = (2.0 * std::f64::consts::PI) / 3.0;
        
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
        }
    }

    /// Ease in-out elastic
    pub fn ease_in_out_elastic(t: f64) -> f64 {
        const C5: f64 = (2.0 * std::f64::consts::PI) / 4.5;
        
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

    /// Ease in bounce
    pub fn ease_in_bounce(t: f64) -> f64 {
        1.0 - Self::ease_out_bounce(1.0 - t)
    }

    /// Ease out bounce
    pub fn ease_out_bounce(t: f64) -> f64 {
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;
        
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

    /// Ease in-out bounce
    pub fn ease_in_out_bounce(t: f64) -> f64 {
        if t < 0.5 {
            (1.0 - Self::ease_out_bounce(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + Self::ease_out_bounce(2.0 * t - 1.0)) / 2.0
        }
    }

    /// Cubic bezier interpolation
    pub fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        uuu * 0.0 + 3.0 * uu * t * x1 + 3.0 * u * tt * x2 + ttt * 1.0
    }

    /// Get easing function by name
    pub fn get_easing_by_name(name: &str) -> Option<Easing> {
        match name {
            "linear" => Some(Easing::Linear),
            "ease-in" => Some(Easing::EaseIn),
            "ease-out" => Some(Easing::EaseOut),
            "ease-in-out" => Some(Easing::EaseInOut),
            _ => None,
        }
    }

    /// Get all available easing function names
    pub fn get_easing_names() -> Vec<&'static str> {
        vec![
            "linear",
            "ease-in",
            "ease-out",
            "ease-in-out",
        ]
    }
}
