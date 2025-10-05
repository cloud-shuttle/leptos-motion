//! Unit tests for Transition type

use crate::types::*;

#[cfg(test)]
mod transition_tests {
    use super::*;

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert_eq!(transition.duration, 0.3);
        assert_eq!(transition.easing, Easing::EaseOut);
        assert_eq!(transition.delay, 0.0);
        assert_eq!(transition.repeat, None);
        assert_eq!(transition.yoyo, false);
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };

        assert_eq!(transition.duration, 0.5);
        assert_eq!(transition.easing, Easing::Linear);
        assert_eq!(transition.delay, 0.1);
        assert_eq!(transition.repeat, Some(3));
        assert_eq!(transition.yoyo, true);
    }

    #[test]
    fn test_transition_equality() {
        let transition1 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition2 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition3 = Transition {
            duration: 0.6,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };

        assert_eq!(transition1, transition2);
        assert_ne!(transition1, transition3);
    }

    #[test]
    fn test_transition_clone() {
        let transition1 = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let transition2 = transition1.clone();
        assert_eq!(transition1, transition2);
    }

    #[test]
    fn test_transition_debug() {
        let transition = Transition {
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        let debug_str = format!("{:?}", transition);
        assert!(debug_str.contains("Transition"));
    }
}
