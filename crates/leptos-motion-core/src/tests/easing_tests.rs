//! Unit tests for Easing type

use crate::types::*;

#[cfg(test)]
mod easing_tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = Easing::Linear;
        assert_eq!(format!("{:?}", easing), "Linear");
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = Easing::EaseIn;
        assert_eq!(format!("{:?}", easing), "EaseIn");
    }

    #[test]
    fn test_easing_ease_out() {
        let easing = Easing::EaseOut;
        assert_eq!(format!("{:?}", easing), "EaseOut");
    }

    #[test]
    fn test_easing_ease_in_out() {
        let easing = Easing::EaseInOut;
        assert_eq!(format!("{:?}", easing), "EaseInOut");
    }

    #[test]
    fn test_easing_spring() {
        let spring_config = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let easing = Easing::Spring(spring_config);
        match easing {
            Easing::Spring(config) => {
                assert_eq!(config.stiffness, 100.0);
                assert_eq!(config.damping, 10.0);
                assert_eq!(config.mass, 1.0);
            }
            _ => panic!("Expected Spring variant"),
        }
    }

    #[test]
    fn test_easing_cubic_bezier() {
        let easing = Easing::CubicBezier(0.25, 0.1, 0.25, 1.0);
        match easing {
            Easing::CubicBezier(x1, y1, x2, y2) => {
                assert_eq!(x1, 0.25);
                assert_eq!(y1, 0.1);
                assert_eq!(x2, 0.25);
                assert_eq!(y2, 1.0);
            }
            _ => panic!("Expected CubicBezier variant"),
        }
    }

    #[test]
    fn test_easing_equality() {
        let easing1 = Easing::Linear;
        let easing2 = Easing::Linear;
        let easing3 = Easing::EaseIn;

        assert_eq!(easing1, easing2);
        assert_ne!(easing1, easing3);
    }

    #[test]
    fn test_easing_clone() {
        let easing1 = Easing::Linear;
        let easing2 = easing1.clone();
        assert_eq!(easing1, easing2);
    }
}
