//! Basic functionality tests that can run in standard Rust test environment
//! 
//! These tests verify core functionality without requiring WASM environment

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;

    /// Test that basic types can be created and used
    #[test]
    fn test_basic_types_creation() {
        // Test AnimationHandle
        let handle = AnimationHandle(123);
        assert_eq!(handle.0, 123);
        
        // Test AnimationValue
        let number_value = AnimationValue::Number(42.0);
        match number_value {
            AnimationValue::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected Number variant"),
        }
        
        let string_value = AnimationValue::String("test".to_string());
        match string_value {
            AnimationValue::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected String variant"),
        }
    }

    /// Test that we can create animation targets
    #[test]
    fn test_animation_target_creation() {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.2));
        target.insert("color".to_string(), AnimationValue::String("#ff0000".to_string()));
        
        assert_eq!(target.len(), 3);
        assert!(target.contains_key("opacity"));
        assert!(target.contains_key("scale"));
        assert!(target.contains_key("color"));
        
        // Test getting values
        if let Some(AnimationValue::Number(opacity)) = target.get("opacity") {
            assert_eq!(*opacity, 1.0);
        } else {
            panic!("Expected opacity to be a number");
        }
    }

    /// Test that we can create transitions
    #[test]
    fn test_transition_creation() {
        let transition = Transition {
            duration: Some(1.0),
            ease: Easing::Linear,
            delay: Some(0.1),
            repeat: RepeatConfig::Never,
            ..Default::default()
        };
        
        assert_eq!(transition.duration, Some(1.0));
        assert_eq!(transition.delay, Some(0.1));
        assert_eq!(transition.ease, Easing::Linear);
    }

    /// Test that we can create a minimal engine
    #[test]
    fn test_minimal_engine_creation() {
        let engine = MinimalEngine::new();
        assert_eq!(engine.active_count(), 0);
    }

    /// Test that we can create a simplified engine
    #[test]
    fn test_simplified_engine_creation() {
        let _engine = SimplifiedAnimationEngine::new();
        // The engine should be created successfully
        // We can't test much more without WASM environment
    }

    /// Test that we can create an optimized hybrid engine
    /// Note: This test requires WASM environment, so it's commented out for standard tests
    // #[test]
    // fn test_optimized_hybrid_engine_creation() {
    //     let _engine = OptimizedHybridEngine::new();
    //     // The engine should be created successfully
    //     // We can't test much more without WASM environment
    // }

    /// Test error handling types
    #[test]
    fn test_error_types() {
        let error = AnimationError::InvalidValue("test error".to_string());
        match error {
            AnimationError::InvalidValue(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Expected InvalidValue error"),
        }
    }

    /// Test that we can create motion values (if leptos-integration feature is enabled)
    #[cfg(feature = "leptos-integration")]
    #[test]
    fn test_motion_values_creation() {
        use crate::values::*;
        
        let motion_number = MotionNumber::new(42.0);
        assert_eq!(motion_number.get(), 42.0);
        
        let motion_transform = MotionTransform::identity();
        let transform = motion_transform.get();
        assert!(transform.is_identity());
    }
}
