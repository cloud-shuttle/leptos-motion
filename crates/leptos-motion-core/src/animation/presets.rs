//! Comprehensive animation presets for common use cases

use crate::Variants;
use crate::animation::Keyframes;
use crate::{
    AnimationBuilder, AnimationConfig, AnimationValue, Easing, RepeatConfig, SpringConfig,
    StaggerConfig, StaggerFrom, Transition,
};

/// Animation presets for common UI patterns
pub struct AnimationPresets;

impl AnimationPresets {
    // ===== ENTRANCE ANIMATIONS =====

    /// Fade in animation
    pub fn fade_in() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!("opacity" => AnimationValue::Number(0.0)))
            .animate(animate!("opacity" => AnimationValue::Number(1.0)))
            .transition(Transition {
                duration: Some(0.3),
                ease: Easing::EaseOut,
                ..Default::default()
            })
            .build()
    }

    /// Slide up animation
    pub fn slide_up(distance: f64) -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "y" => AnimationValue::Pixels(distance)
            ))
            .animate(animate!(
                "opacity" => AnimationValue::Number(1.0),
                "y" => AnimationValue::Pixels(0.0)
            ))
            .transition(Transition {
                duration: Some(0.5),
                ease: Easing::Spring(SpringConfig::default()),
                ..Default::default()
            })
            .build()
    }

    /// Scale in animation
    pub fn scale_in() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8)
            ))
            .animate(animate!(
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(1.0)
            ))
            .transition(Transition {
                duration: Some(0.3),
                ease: Easing::BackOut,
                ..Default::default()
            })
            .build()
    }

    /// Pop in animation
    pub fn pop_in() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.0)
            ))
            .animate(animate!(
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(1.0)
            ))
            .transition(Transition {
                duration: Some(0.4),
                ease: Easing::Spring(SpringConfig {
                    stiffness: 200.0,
                    damping: 15.0,
                    mass: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build()
    }

    /// Rotate in animation
    pub fn rotate_in() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "rotate" => AnimationValue::Degrees(-180.0)
            ))
            .animate(animate!(
                "opacity" => AnimationValue::Number(1.0),
                "rotate" => AnimationValue::Degrees(0.0)
            ))
            .transition(Transition {
                duration: Some(0.6),
                ease: Easing::BackOut,
                ..Default::default()
            })
            .build()
    }

    /// Flip in animation
    pub fn flip_in() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "rotateY" => AnimationValue::Degrees(-90.0)
            ))
            .animate(animate!(
                "opacity" => AnimationValue::Number(1.0),
                "rotateY" => AnimationValue::Degrees(0.0)
            ))
            .transition(Transition {
                duration: Some(0.7),
                ease: Easing::EaseOut,
                ..Default::default()
            })
            .build()
    }

    // ===== EXIT ANIMATIONS =====

    /// Fade out animation
    pub fn fade_out() -> AnimationConfig {
        AnimationBuilder::new()
            .exit(animate!("opacity" => AnimationValue::Number(0.0)))
            .transition(Transition {
                duration: Some(0.2),
                ease: Easing::EaseIn,
                ..Default::default()
            })
            .build()
    }

    /// Scale out animation
    pub fn scale_out() -> AnimationConfig {
        AnimationBuilder::new()
            .exit(animate!(
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.8)
            ))
            .transition(Transition {
                duration: Some(0.2),
                ease: Easing::EaseIn,
                ..Default::default()
            })
            .build()
    }

    // ===== INTERACTION ANIMATIONS =====

    /// Hover lift animation
    pub fn hover_lift() -> Variants {
        Variants::new()
            .add_variant("initial", animate!("y" => AnimationValue::Pixels(0.0)))
            .add_variant("hover", animate!("y" => AnimationValue::Pixels(-8.0)))
            .initial("initial")
    }

    /// Hover scale animation
    pub fn hover_scale(scale: f64) -> Variants {
        Variants::new()
            .add_variant("initial", animate!("scale" => AnimationValue::Number(1.0)))
            .add_variant("hover", animate!("scale" => AnimationValue::Number(scale)))
            .initial("initial")
    }

    /// Tap press animation
    pub fn tap_press() -> Variants {
        Variants::new()
            .add_variant("initial", animate!("scale" => AnimationValue::Number(1.0)))
            .add_variant("tap", animate!("scale" => AnimationValue::Number(0.95)))
            .initial("initial")
    }

    // ===== ATTENTION ANIMATIONS =====

    /// Pulse animation
    pub fn pulse() -> Keyframes {
        Keyframes::new()
            .add(
                0.0,
                animate!("scale" => AnimationValue::Number(1.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.5,
                animate!("scale" => AnimationValue::Number(1.05)),
                Some(Easing::EaseInOut),
            )
            .add(1.0, animate!("scale" => AnimationValue::Number(1.0)), None)
    }

    /// Bounce animation
    pub fn bounce() -> Keyframes {
        Keyframes::new()
            .add(
                0.0,
                animate!("y" => AnimationValue::Pixels(0.0)),
                Some(Easing::EaseOut),
            )
            .add(
                0.5,
                animate!("y" => AnimationValue::Pixels(-20.0)),
                Some(Easing::EaseIn),
            )
            .add(1.0, animate!("y" => AnimationValue::Pixels(0.0)), None)
    }

    /// Shake animation
    pub fn shake() -> Keyframes {
        Keyframes::new()
            .add(
                0.0,
                animate!("x" => AnimationValue::Pixels(0.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.1,
                animate!("x" => AnimationValue::Pixels(-10.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.2,
                animate!("x" => AnimationValue::Pixels(10.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.3,
                animate!("x" => AnimationValue::Pixels(-10.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.4,
                animate!("x" => AnimationValue::Pixels(10.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.5,
                animate!("x" => AnimationValue::Pixels(-5.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.6,
                animate!("x" => AnimationValue::Pixels(5.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.7,
                animate!("x" => AnimationValue::Pixels(-2.0)),
                Some(Easing::EaseInOut),
            )
            .add(
                0.8,
                animate!("x" => AnimationValue::Pixels(2.0)),
                Some(Easing::EaseInOut),
            )
            .add(1.0, animate!("x" => AnimationValue::Pixels(0.0)), None)
    }

    // ===== LOADING ANIMATIONS =====

    /// Spin animation
    pub fn spin() -> AnimationConfig {
        AnimationBuilder::new()
            .animate(animate!("rotate" => AnimationValue::Degrees(360.0)))
            .transition(Transition {
                duration: Some(1.0),
                ease: Easing::Linear,
                repeat: RepeatConfig::Infinite,
                ..Default::default()
            })
            .build()
    }

    // ===== PAGE TRANSITIONS =====

    /// Page fade transition
    pub fn page_fade() -> AnimationConfig {
        AnimationBuilder::new()
            .initial(animate!("opacity" => AnimationValue::Number(0.0)))
            .animate(animate!("opacity" => AnimationValue::Number(1.0)))
            .exit(animate!("opacity" => AnimationValue::Number(0.0)))
            .transition(Transition {
                duration: Some(0.2),
                ease: Easing::EaseInOut,
                ..Default::default()
            })
            .build()
    }

    // ===== LIST ANIMATIONS =====

    /// Stagger children animation
    pub fn stagger_children(delay: f64) -> Transition {
        Transition {
            duration: Some(0.4),
            ease: Easing::EaseOut,
            stagger: Some(StaggerConfig {
                delay,
                from: StaggerFrom::First,
            }),
            ..Default::default()
        }
    }
}

/// Slide direction for page transitions
#[derive(Debug, Clone, Copy)]
pub enum SlideDirection {
    /// Slide from left to right
    Left,
    /// Slide from right to left
    Right,
}

/// Spring configuration presets
pub mod springs {
    use super::*;

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

/// Easing presets
pub mod easings {
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

    #[test]
    fn test_fade_in_preset() {
        let config = AnimationPresets::fade_in();
        assert_eq!(
            config.initial.get("opacity"),
            Some(&AnimationValue::Number(0.0))
        );
        assert_eq!(
            config.animate.get("opacity"),
            Some(&AnimationValue::Number(1.0))
        );
    }

    #[test]
    fn test_slide_up_preset() {
        let config = AnimationPresets::slide_up(50.0);
        assert_eq!(config.initial.get("y"), Some(&AnimationValue::Pixels(50.0)));
        assert_eq!(config.animate.get("y"), Some(&AnimationValue::Pixels(0.0)));
    }

    #[test]
    fn test_hover_lift_variants() {
        let variants = AnimationPresets::hover_lift();
        assert_eq!(
            variants.get_variant("initial").unwrap().get("y"),
            Some(&AnimationValue::Pixels(0.0))
        );
        assert_eq!(
            variants.get_variant("hover").unwrap().get("y"),
            Some(&AnimationValue::Pixels(-8.0))
        );
    }
}
