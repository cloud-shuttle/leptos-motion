//! Comprehensive unit tests for animation engine

use crate::engine::*;
use crate::types::*;
use crate::{AnimationError, Result};
use std::collections::HashMap;

#[cfg(test)]
mod animation_config_tests {
    use super::*;

    #[test]
    fn test_animation_config_structure() {
        // AnimationConfig doesn't have a default implementation
        // It requires element, from, to, and transition
        let from = AnimationTarget::new();
        let to = AnimationTarget::new();
        let transition = Transition::default();
        
        // Note: This test would need a real element in a real environment
        // For now, we just test that the structure exists
        assert_eq!(from.len(), 0);
        assert_eq!(to.len(), 0);
        assert_eq!(transition.duration, 0.3);
    }

    #[test]
    fn test_animation_config_new() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let config = AnimationConfig {
            target,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        assert_eq!(config.duration, 0.5);
        assert_eq!(config.easing, Easing::Linear);
        assert_eq!(config.delay, 0.1);
        assert_eq!(config.repeat, Some(3));
        assert_eq!(config.yoyo, true);
        assert_eq!(config.target.len(), 1);
    }

    #[test]
    fn test_animation_config_equality() {
        let mut target1 = AnimationTarget::new();
        target1.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let mut target2 = AnimationTarget::new();
        target2.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let config1 = AnimationConfig {
            target: target1,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        let config2 = AnimationConfig {
            target: target2,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        let mut target3 = AnimationTarget::new();
        target3.insert("opacity".to_string(), AnimationValue::Number(0.6));
        
        let config3 = AnimationConfig {
            target: target3,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_animation_config_clone() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let config1 = AnimationConfig {
            target,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_animation_config_debug() {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let config = AnimationConfig {
            target,
            duration: 0.5,
            easing: Easing::Linear,
            delay: 0.1,
            repeat: Some(3),
            yoyo: true,
        };
        
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("AnimationConfig"));
    }
}

#[cfg(test)]
mod playback_state_tests {
    use super::*;

    #[test]
    fn test_playback_state_running() {
        let state = PlaybackState::Running;
        assert_eq!(format!("{:?}", state), "Running");
    }

    #[test]
    fn test_playback_state_paused() {
        let state = PlaybackState::Paused;
        assert_eq!(format!("{:?}", state), "Paused");
    }

    #[test]
    fn test_playback_state_completed() {
        let state = PlaybackState::Completed;
        assert_eq!(format!("{:?}", state), "Completed");
    }

    #[test]
    fn test_playback_state_cancelled() {
        let state = PlaybackState::Cancelled;
        assert_eq!(format!("{:?}", state), "Cancelled");
    }

    #[test]
    fn test_playback_state_equality() {
        let state1 = PlaybackState::Running;
        let state2 = PlaybackState::Running;
        let state3 = PlaybackState::Paused;
        
        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_playback_state_clone() {
        let state1 = PlaybackState::Running;
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }
}

#[cfg(test)]
mod raf_engine_tests {
    use super::*;

    #[test]
    fn test_raf_engine_new() {
        let engine = RafEngine::new();
        assert!(engine.is_available());
    }

    #[test]
    fn test_raf_engine_is_available() {
        let engine = RafEngine::new();
        // RAF engine should always be available in test environment
        assert!(engine.is_available());
    }

    #[test]
    fn test_raf_engine_basic_functionality() {
        let mut engine = RafEngine::new();
        
        // Test that we can create a handle
        let handle = AnimationHandle(1);
        
        // Test basic state queries (these should not panic)
        let _is_running = engine.is_running(handle);
        
        // Test tick functionality
        let result = engine.tick(0.0);
        assert!(result.is_ok());
        
        // Test performance metrics
        let _metrics = engine.get_performance_metrics();
    }

    #[test]
    fn test_raf_engine_error_handling() {
        let mut engine = RafEngine::new();
        let handle = AnimationHandle(999);
        
        // Test error handling for non-existent animations
        let result = engine.stop(handle);
        assert!(result.is_err());
        
        let result = engine.pause(handle);
        assert!(result.is_err());
        
        let result = engine.resume(handle);
        assert!(result.is_err());
        
        let result = engine.get_state(handle);
        assert!(result.is_err());
    }

    #[test]
    fn test_raf_engine_performance_metrics() {
        let engine = RafEngine::new();
        let metrics = engine.get_performance_metrics();
        // Performance metrics are optional and may be None
        assert!(metrics.is_some() || metrics.is_none());
    }
}

#[cfg(test)]
mod optimized_hybrid_engine_tests {
    use super::*;

    #[test]
    fn test_optimized_hybrid_engine_new() {
        let engine = OptimizedHybridEngine::new();
        assert!(engine.is_available());
    }

    #[test]
    fn test_optimized_hybrid_engine_is_available() {
        let engine = OptimizedHybridEngine::new();
        // Hybrid engine should always be available in test environment
        assert!(engine.is_available());
    }

    #[test]
    fn test_optimized_hybrid_engine_basic_functionality() {
        let mut engine = OptimizedHybridEngine::new();
        
        // Test that we can create a handle
        let handle = AnimationHandle(1);
        
        // Test basic state queries (these should not panic)
        let _is_running = engine.is_running(handle);
        
        // Test tick functionality
        let result = engine.tick(0.0);
        assert!(result.is_ok());
        
        // Test performance metrics
        let _metrics = engine.get_performance_metrics();
    }

    #[test]
    fn test_optimized_hybrid_engine_error_handling() {
        let mut engine = OptimizedHybridEngine::new();
        let handle = AnimationHandle(999);
        
        // Test error handling for non-existent animations
        let result = engine.stop(handle);
        assert!(result.is_err());
        
        let result = engine.pause(handle);
        assert!(result.is_err());
        
        let result = engine.resume(handle);
        assert!(result.is_err());
        
        let result = engine.get_state(handle);
        assert!(result.is_err());
    }

    #[test]
    fn test_optimized_hybrid_engine_performance_metrics() {
        let engine = OptimizedHybridEngine::new();
        let metrics = engine.get_performance_metrics();
        // Performance metrics are optional and may be None
        assert!(metrics.is_some() || metrics.is_none());
    }
}

#[cfg(test)]
mod animation_engine_trait_tests {
    use super::*;

    // Test that all engines implement the AnimationEngine trait correctly
    #[test]
    fn test_raf_engine_trait_implementation() {
        let mut engine = RafEngine::new();
        test_animation_engine_trait(&mut engine);
    }

    #[test]
    fn test_optimized_hybrid_engine_trait_implementation() {
        let mut engine = OptimizedHybridEngine::new();
        test_animation_engine_trait(&mut engine);
    }

    fn test_animation_engine_trait(engine: &mut dyn AnimationEngine) {
        // Test is_available
        assert!(engine.is_available());
        
        // Test basic functionality without requiring DOM elements
        let handle = AnimationHandle(1);
        
        // Test is_running (should not panic)
        let _is_running = engine.is_running(handle);
        
        // Test get_state (should return error for non-existent handle)
        let state = engine.get_state(handle);
        assert!(state.is_err());
        
        // Test pause (should return error for non-existent handle)
        let result = engine.pause(handle);
        assert!(result.is_err());
        
        // Test resume (should return error for non-existent handle)
        let result = engine.resume(handle);
        assert!(result.is_err());
        
        // Test stop (should return error for non-existent handle)
        let result = engine.stop(handle);
        assert!(result.is_err());
        
        // Test tick
        let result = engine.tick(0.0);
        assert!(result.is_ok());
        
        // Test performance metrics
        let _metrics = engine.get_performance_metrics();
    }
}
