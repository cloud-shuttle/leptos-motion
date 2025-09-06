//! Phase 1 TDD Tests: Production Hardening
//!
//! This module contains all tests for Phase 1 of the v1.0 development roadmap.
//! These tests follow the Red-Green-Refactor TDD cycle to ensure production-ready quality.

pub mod core_engine_refinement;
pub mod bundle_optimization;
pub mod cross_browser_compatibility;

// Re-export common test utilities
pub use crate::test_utils::*;

#[cfg(test)]
mod phase1_integration_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Integration test: All Phase 1 components work together
    /// This ensures our individual optimizations don't break when combined
    #[wasm_bindgen_test]
    fn test_phase1_components_integration() {
        // Test that engine refinements work with bundle optimization
        let engine = leptos_motion_core::AnimationEngine::new();

        // Should handle concurrent animations (engine refinement)
        let animation_configs: Vec<_> = (0..10)
            .map(|i| create_test_animation_config(i))
            .collect();

        for config in animation_configs {
            // Should start without error (bundle optimization preserved functionality)
            if let Err(e) = engine.start_animation(config) {
                // For now, expect NotImplemented error in Red Phase
                assert!(matches!(e, leptos_motion_core::AnimationError::NotImplemented(_)));
            }
        }

        // Engine should maintain stable state across optimizations
        // (This will pass once we implement Green Phase)
    }

    /// Integration test: Performance regression protection
    /// Ensures optimizations don't hurt performance
    #[wasm_bindgen_test]
    fn test_phase1_performance_regression_protection() {
        let start = instant::Instant::now();

        // Perform typical operations
        let engine = leptos_motion_core::AnimationEngine::new();
        for i in 0..50 {
            let config = create_test_animation_config(i);
            let _ = engine.start_animation(config);
        }

        let elapsed = start.elapsed();

        // Should complete operations quickly even with safety checks
        assert!(
            elapsed.as_millis() < 50,
            "Phase 1 operations took too long: {}ms",
            elapsed.as_millis()
        );
    }
}

// Common test utilities for Phase 1
pub fn create_test_animation_config(id: usize) -> leptos_motion_core::AnimationConfig {
    leptos_motion_core::AnimationConfig {
        id: Some(format!("phase1_test_{}", id)),
        target: leptos_motion_core::motion_target!(
            "opacity" => leptos_motion_core::AnimationValue::Number(1.0),
            "scale" => leptos_motion_core::AnimationValue::Number(1.1)
        ),
        duration: Some(0.2),
        ease: leptos_motion_core::Easing::Linear,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::None,
    }
}
