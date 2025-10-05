//! Unit tests for StaggerConfig type

use crate::types::*;

#[cfg(test)]
mod stagger_config_tests {
    use super::*;

    #[test]
    fn test_stagger_config_default() {
        let config = StaggerConfig::default();
        assert_eq!(config.delay, 0.1);
        assert_eq!(config.from, StaggerFrom::Start);
        assert_eq!(config.ease, Easing::Linear);
    }

    #[test]
    fn test_stagger_config_new() {
        let config = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };

        assert_eq!(config.delay, 0.2);
        assert_eq!(config.from, StaggerFrom::End);
        assert_eq!(config.ease, Easing::EaseIn);
    }

    #[test]
    fn test_stagger_config_equality() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config2 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config3 = StaggerConfig {
            delay: 0.3,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_stagger_config_clone() {
        let config1 = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_stagger_config_debug() {
        let config = StaggerConfig {
            delay: 0.2,
            from: StaggerFrom::End,
            ease: Easing::EaseIn,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("StaggerConfig"));
    }
}
