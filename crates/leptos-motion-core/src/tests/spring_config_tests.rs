//! Unit tests for SpringConfig type

use crate::types::*;

#[cfg(test)]
mod spring_config_tests {
    use super::*;

    #[test]
    fn test_spring_config_default() {
        let config = SpringConfig::default();
        assert_eq!(config.stiffness, 100.0);
        assert_eq!(config.damping, 10.0);
        assert_eq!(config.mass, 1.0);
    }

    #[test]
    fn test_spring_config_new() {
        let config = SpringConfig {
            stiffness: 200.0,
            damping: 20.0,
            mass: 2.0,
        };

        assert_eq!(config.stiffness, 200.0);
        assert_eq!(config.damping, 20.0);
        assert_eq!(config.mass, 2.0);
    }

    #[test]
    fn test_spring_config_equality() {
        let config1 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config2 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config3 = SpringConfig {
            stiffness: 200.0,
            damping: 10.0,
            mass: 1.0,
        };

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_spring_config_clone() {
        let config1 = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_spring_config_debug() {
        let config = SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("SpringConfig"));
    }
}
