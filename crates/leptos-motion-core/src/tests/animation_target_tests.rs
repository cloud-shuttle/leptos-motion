//! Unit tests for AnimationTarget type

use crate::types::*;

#[cfg(test)]
mod animation_target_tests {
    use super::*;

    #[test]
    fn test_animation_target_new() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));

        assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(0.5)));
        assert_eq!(target.get("x"), Some(&AnimationValue::Pixels(100.0)));
        assert_eq!(target.len(), 2);
    }

    #[test]
    fn test_animation_target_is_empty() {
        let target = AnimationTarget::new();
        assert!(target.is_empty());

        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        assert!(!target.is_empty());
    }

    #[test]
    fn test_animation_target_contains_key() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));

        assert!(target.contains_key("opacity"));
        assert!(!target.contains_key("x"));
    }

    #[test]
    fn test_animation_target_remove() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));

        assert_eq!(target.remove("opacity"), Some(AnimationValue::Number(0.5)));
        assert_eq!(target.remove("opacity"), None);
        assert!(target.is_empty());
    }

    #[test]
    fn test_animation_target_iter() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));

        let mut keys: Vec<_> = target.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["opacity", "x"]);

        let mut values: Vec<_> = target.values().collect();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_animation_target_clone() {
        let mut target1 = AnimationTarget::new();
        target1.insert("opacity".to_string(), AnimationValue::Number(0.5));

        let target2 = target1.clone();
        assert_eq!(target1, target2);
    }

    #[test]
    fn test_animation_target_debug() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));

        let debug_str = format!("{:?}", target);
        assert!(debug_str.contains("AnimationTarget"));
    }
}
