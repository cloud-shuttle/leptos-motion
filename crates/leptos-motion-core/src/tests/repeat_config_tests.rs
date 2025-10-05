//! Unit tests for RepeatConfig type

use crate::types::*;

#[cfg(test)]
mod repeat_config_tests {
    use super::*;

    #[test]
    fn test_repeat_config_default() {
        let config = RepeatConfig::default();
        assert_eq!(config.count, 1);
        assert_eq!(config.direction, RepeatDirection::Normal);
    }

    #[test]
    fn test_repeat_config_new() {
        let config = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };

        assert_eq!(config.count, 5);
        assert_eq!(config.direction, RepeatDirection::Alternate);
    }

    #[test]
    fn test_repeat_config_equality() {
        let config1 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config2 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config3 = RepeatConfig {
            count: 6,
            direction: RepeatDirection::Alternate,
        };

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_repeat_config_clone() {
        let config1 = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_repeat_config_debug() {
        let config = RepeatConfig {
            count: 5,
            direction: RepeatDirection::Alternate,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RepeatConfig"));
    }
}
