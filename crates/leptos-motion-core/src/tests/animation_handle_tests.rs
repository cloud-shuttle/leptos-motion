//! Unit tests for AnimationHandle type

use crate::types::*;
use std::collections::HashMap;

#[cfg(test)]
mod animation_handle_tests {
    use super::*;

    #[test]
    fn test_animation_handle_creation() {
        let handle = AnimationHandle(123);
        assert_eq!(handle.0, 123);
    }

    #[test]
    fn test_animation_handle_equality() {
        let handle1 = AnimationHandle(123);
        let handle2 = AnimationHandle(123);
        let handle3 = AnimationHandle(456);

        assert_eq!(handle1, handle2);
        assert_ne!(handle1, handle3);
    }

    #[test]
    fn test_animation_handle_hash() {
        let handle1 = AnimationHandle(123);
        let handle2 = AnimationHandle(123);
        let handle3 = AnimationHandle(456);

        let mut map = HashMap::new();
        map.insert(handle1, "test1");
        map.insert(handle3, "test3");

        assert_eq!(map.get(&handle2), Some(&"test1"));
        assert_eq!(map.get(&handle3), Some(&"test3"));
    }

    #[test]
    fn test_animation_handle_debug() {
        let handle = AnimationHandle(123);
        let debug_str = format!("{:?}", handle);
        assert!(debug_str.contains("123"));
    }

    #[test]
    fn test_animation_handle_clone() {
        let handle1 = AnimationHandle(123);
        let handle2 = handle1.clone();
        assert_eq!(handle1, handle2);
    }
}
