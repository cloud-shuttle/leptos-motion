//! Phase 1 TDD Implementation: Core Engine Refinement
//!
//! Red Phase: Write failing tests for production-ready animation engine
//! Green Phase: Implement minimal code to make tests pass
//! Refactor Phase: Optimize and improve code quality

use leptos_motion_core::{
    AnimationConfig, AnimationEngine, AnimationError, AnimationHandle, AnimationTarget,
    AnimationValue, Easing, RepeatConfig,
};
use rstest::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Animation engine handles concurrent animations gracefully
/// This test will FAIL initially (Red Phase) until we implement proper concurrent handling
#[rstest]
#[case::two_animations(2)]
#[case::ten_animations(10)]
#[case::fifty_animations(50)]
#[wasm_bindgen_test]
fn test_animation_engine_handles_concurrent_animations(#[case] animation_count: usize) {
    // Arrange: Create engine and multiple animations
    let engine = AnimationEngine::new();
    let animations: Vec<_> = (0..animation_count)
        .map(|i| create_test_animation_config(i))
        .collect();

    // Act: Start all animations simultaneously
    let handles: Vec<_> = animations
        .iter()
        .map(|anim| {
            engine
                .start_animation(anim.clone())
                .expect("Should start animation")
        })
        .collect();

    // Assert: All animations should be active and tracked
    assert_eq!(
        engine.active_animations_count(),
        animation_count,
        "Engine should track all {} concurrent animations",
        animation_count
    );

    // Assert: Each handle should be valid and active
    for (i, handle) in handles.iter().enumerate() {
        assert!(
            handle.is_active(),
            "Animation {} should be active after starting",
            i
        );
    }

    // Assert: Engine should remain stable under load
    assert!(
        engine.is_stable(),
        "Engine should maintain stable state with {} animations",
        animation_count
    );
}

/// Test: Memory cleanup after animation completion
/// This will FAIL initially - we need to implement proper cleanup
#[wasm_bindgen_test]
fn test_memory_cleanup_after_animation_completion() {
    // Arrange: Create engine and get baseline memory
    let engine = AnimationEngine::new();
    let baseline_memory = get_memory_usage();
    let test_animations = 50; // Reasonable number for memory test

    // Act: Start and complete multiple animations
    let mut handles = Vec::new();
    for i in 0..test_animations {
        let config = create_short_animation_config(i); // Very short duration
        let handle = engine
            .start_animation(config)
            .expect("Should start animation");
        handles.push(handle);
    }

    // Wait for all animations to complete
    wait_for_animations_completion(&handles, 1000); // 1 second max wait

    // Force garbage collection
    force_garbage_collection();

    // Assert: Memory should be cleaned up
    let final_memory = get_memory_usage();
    let memory_growth = final_memory.saturating_sub(baseline_memory);

    // Allow for some reasonable memory overhead (100KB)
    const MAX_ACCEPTABLE_GROWTH: usize = 100 * 1024;
    assert!(
        memory_growth < MAX_ACCEPTABLE_GROWTH,
        "Memory leak detected: grew by {}KB (max allowed: {}KB)",
        memory_growth / 1024,
        MAX_ACCEPTABLE_GROWTH / 1024
    );

    // Assert: No animations should remain active
    assert_eq!(
        engine.active_animations_count(),
        0,
        "All animations should be cleaned up after completion"
    );
}

/// Test: Error recovery from invalid animation values
/// This will FAIL initially - we need robust error handling
#[rstest]
#[case::nan_opacity(create_invalid_target_with_nan())]
#[case::infinite_scale(create_invalid_target_with_infinity())]
#[case::negative_infinite(create_invalid_target_with_neg_infinity())]
#[wasm_bindgen_test]
fn test_error_recovery_from_invalid_animation_values(#[case] invalid_target: AnimationTarget) {
    // Arrange: Create engine and invalid animation config
    let engine = AnimationEngine::new();
    let invalid_config = AnimationConfig {
        target: invalid_target,
        duration: Some(0.5),
        ease: Easing::Linear,
        ..Default::default()
    };

    // Act: Attempt to start invalid animation
    let result = engine.start_animation(invalid_config);

    // Assert: Should return appropriate error, not panic
    assert!(
        result.is_err(),
        "Engine should reject invalid animation values"
    );

    if let Err(error) = result {
        assert!(
            matches!(error, AnimationError::InvalidValue(_)),
            "Should return InvalidValue error for invalid animation values"
        );
    }

    // Assert: Engine should remain functional after error
    let valid_config = create_test_animation_config(1);
    assert!(
        engine.start_animation(valid_config).is_ok(),
        "Engine should remain functional after handling invalid input"
    );
}

/// Test: Invalid animation duration handling
/// This will FAIL initially - need duration validation
#[rstest]
#[case::negative_duration(-1.0)]
#[case::zero_duration(0.0)]
#[case::nan_duration(f64::NAN)]
#[case::infinite_duration(f64::INFINITY)]
#[wasm_bindgen_test]
fn test_error_recovery_from_invalid_duration(#[case] invalid_duration: f64) {
    // Arrange
    let engine = AnimationEngine::new();
    let config = AnimationConfig {
        target: create_valid_animation_target(),
        duration: Some(invalid_duration),
        ease: Easing::Linear,
        ..Default::default()
    };

    // Act
    let result = engine.start_animation(config);

    // Assert: Should handle invalid duration gracefully
    if invalid_duration <= 0.0 || !invalid_duration.is_finite() {
        assert!(
            result.is_err(),
            "Should reject invalid duration: {}",
            invalid_duration
        );
    }
}

/// Test: Engine state consistency under rapid start/stop operations
/// This will FAIL initially - need proper state management
#[wasm_bindgen_test]
fn test_engine_state_consistency_under_rapid_operations() {
    // Arrange
    let engine = AnimationEngine::new();
    let operation_count = 100;

    // Act: Rapid start/stop operations
    for i in 0..operation_count {
        let config = create_test_animation_config(i);
        if let Ok(handle) = engine.start_animation(config) {
            // Immediately stop some animations
            if i % 3 == 0 {
                handle.stop();
            }
        }
    }

    // Assert: Engine should maintain consistent state
    let final_count = engine.active_animations_count();
    assert!(
        final_count <= operation_count,
        "Active animation count should not exceed started animations"
    );

    // Assert: Engine should be stable after rapid operations
    assert!(
        engine.is_stable(),
        "Engine should remain stable after rapid start/stop operations"
    );
}

/// Test: Performance under load (benchmark-style test)
/// This will FAIL initially - need performance optimizations
#[wasm_bindgen_test]
fn test_animation_engine_performance_under_load() {
    // Arrange
    let engine = AnimationEngine::new();
    let animation_count = 200;
    let start_time = Instant::now();

    // Act: Start many animations
    let mut successful_starts = 0;
    for i in 0..animation_count {
        let config = create_test_animation_config(i);
        if engine.start_animation(config).is_ok() {
            successful_starts += 1;
        }
    }

    let elapsed = start_time.elapsed();

    // Assert: Performance should be acceptable
    assert!(
        elapsed.as_millis() < 100,
        "Starting {} animations took too long: {}ms (max: 100ms)",
        animation_count,
        elapsed.as_millis()
    );

    assert_eq!(
        successful_starts, animation_count,
        "All animations should start successfully under load"
    );
}

// ============================================================================
// Helper Functions for Tests
// ============================================================================

fn create_test_animation_config(id: usize) -> AnimationConfig {
    let mut target = AnimationTarget::new();
    target
        .values
        .insert("opacity".to_string(), AnimationValue::Number(1.0));
    target
        .values
        .insert("scale".to_string(), AnimationValue::Number(1.1));
    target
        .values
        .insert("x".to_string(), AnimationValue::Pixels(100.0));

    AnimationConfig {
        id: Some(format!("test_animation_{}", id)),
        target,
        duration: Some(0.2), // Short duration for fast tests
        ease: Easing::Linear,
        delay: None,
        repeat: RepeatConfig::None,
    }
}

fn create_short_animation_config(id: usize) -> AnimationConfig {
    let mut config = create_test_animation_config(id);
    config.duration = Some(0.05); // Very short for cleanup tests
    config
}

fn create_valid_animation_target() -> AnimationTarget {
    let mut target = AnimationTarget::new();
    target
        .values
        .insert("opacity".to_string(), AnimationValue::Number(0.8));
    target
        .values
        .insert("scale".to_string(), AnimationValue::Number(1.2));
    target
}

fn create_invalid_target_with_nan() -> AnimationTarget {
    let mut target = AnimationTarget::new();
    target
        .values
        .insert("opacity".to_string(), AnimationValue::Number(f64::NAN));
    target
}

fn create_invalid_target_with_infinity() -> AnimationTarget {
    let mut target = AnimationTarget::new();
    target
        .values
        .insert("scale".to_string(), AnimationValue::Number(f64::INFINITY));
    target
}

fn create_invalid_target_with_neg_infinity() -> AnimationTarget {
    let mut target = AnimationTarget::new();
    target
        .values
        .insert("x".to_string(), AnimationValue::Pixels(f64::NEG_INFINITY));
    target
}

fn get_memory_usage() -> usize {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.performance())
            .and_then(|performance| js_sys::Reflect::get(&performance, &"memory".into()).ok())
            .and_then(|memory| js_sys::Reflect::get(&memory, &"usedJSHeapSize".into()).ok())
            .and_then(|heap_size| heap_size.as_f64())
            .map(|size| size as usize)
            .unwrap_or(0)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM environments, return a mock value
        use std::sync::atomic::{AtomicUsize, Ordering};
        static MOCK_MEMORY: AtomicUsize = AtomicUsize::new(1024 * 1024); // 1MB baseline
        MOCK_MEMORY.fetch_add(1024, Ordering::SeqCst) // Simulate small growth
    }
}

fn force_garbage_collection() {
    #[cfg(target_arch = "wasm32")]
    {
        // Attempt to trigger GC if available
        if let Ok(global) = js_sys::global().dyn_into::<web_sys::Window>() {
            if let Ok(gc) = js_sys::Reflect::get(&global, &"gc".into()) {
                if let Ok(gc_fn) = gc.dyn_into::<js_sys::Function>() {
                    let _ = gc_fn.call0(&global);
                }
            }
        }

        // Alternative: Create and release large objects to encourage GC
        for _ in 0..10 {
            let _large_array = js_sys::Array::new_with_length(1000);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native tests, we can't force GC but we can simulate cleanup
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn wait_for_animations_completion(handles: &[AnimationHandle], max_wait_ms: u64) {
    let start = Instant::now();
    while start.elapsed().as_millis() < max_wait_ms as u128 {
        let active_count = handles.iter().filter(|h| h.is_active()).count();
        if active_count == 0 {
            break;
        }

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, we'll use a simple busy wait for now
            // In a real implementation, this would yield to the event loop
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

// ============================================================================
// Real Implementation Tests (Green Phase)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_helper_functions_work() {
        // Test that our helper functions are working
        let config = create_test_animation_config(1);
        assert!(config.duration.is_some());
        assert!(!config.target.values.is_empty());

        let memory = get_memory_usage();
        assert!(memory >= 0); // Should return some value
    }
}
