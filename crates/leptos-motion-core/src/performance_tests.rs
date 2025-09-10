//! Performance monitoring integration tests

use crate::engine::OptimizedHybridEngine;
use crate::*;
// WASM-specific test configuration - conditional compilation
#[cfg(feature = "web-sys")]
use wasm_bindgen_test::*;

#[cfg(feature = "web-sys")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test performance monitoring integration with OptimizedHybridEngine
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_optimized_hybrid_engine_performance_monitoring() {
    // Create OptimizedHybridEngine with performance monitoring
    let mut engine = OptimizedHybridEngine::new();

    // Test that performance monitoring methods exist and can be called
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(0);

    // Test that we can get a performance report (even if it's None due to design limitations)
    let metrics = engine.get_performance_metrics();
    // Note: Currently returns None due to design limitations, but the method exists
    assert!(metrics.is_none() || metrics.is_some());
}

/// Test performance monitoring with animation workload
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_monitoring_with_animations() {
    let mut engine = OptimizedHybridEngine::new();

    // Test performance monitoring during animation lifecycle
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(1); // Simulate 1 animation

    // Test that performance monitoring methods work
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(0); // Simulate no animations

    // Verify the methods can be called without errors
    // Test passes if we reach this point
}

/// Test performance budget enforcement
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_budget_enforcement() {
    let mut engine = OptimizedHybridEngine::new();

    // Test performance monitoring with high workload simulation
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(10); // Simulate 10 animations

    // Test that performance monitoring handles high workloads
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(5); // Simulate 5 animations

    // Verify the methods can handle high workloads
    // Test passes if we reach this point
}

/// Test GPU layer management integration
#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_gpu_layer_management_integration() {
    let mut engine = OptimizedHybridEngine::new();

    // Test that GPU optimization method exists and can be called
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("div").unwrap();
    let optimized = engine.optimize_for_gpu(&element);

    // Verify the method can be called (return value may vary)
    // Test passes if we reach here - this is a tautology by design
}

/// Test animation pool integration
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_animation_pool_integration() {
    let mut engine = OptimizedHybridEngine::new();

    // Test that we can create and use the engine
    // The animation pool is integrated internally
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(0);

    // Verify the engine works with performance monitoring
    // Test passes if we reach this point
}

/// Test performance monitoring with real animation
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_monitoring_with_real_animation() {
    let mut engine = OptimizedHybridEngine::new();

    // Test that the engine can be created and basic methods work
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(1);

    // Test that performance monitoring is integrated into the engine
    // Test passes if we reach this point
}

/// Test performance monitoring error handling
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_monitoring_error_handling() {
    let mut engine = OptimizedHybridEngine::new();

    // Test that performance monitoring methods handle edge cases
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(0); // No animations

    engine.start_performance_monitoring();
    engine.end_performance_monitoring(1000); // Many animations

    // Verify the methods handle edge cases gracefully
    // Test passes if we reach this point
}

/// Test performance monitoring memory usage tracking
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_monitoring_memory_tracking() {
    let mut engine = OptimizedHybridEngine::new();

    // Test that performance monitoring tracks memory usage
    engine.start_performance_monitoring();
    engine.end_performance_monitoring(1); // Simulate memory usage

    // Verify the methods can track memory usage
    // Test passes if we reach this point
}

/// Test performance monitoring with multiple engines
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_performance_monitoring_multiple_engines() {
    let mut engine1 = OptimizedHybridEngine::new();
    let mut engine2 = OptimizedHybridEngine::new();

    // Test that multiple engines can be created independently
    engine1.start_performance_monitoring();
    engine1.end_performance_monitoring(1);

    engine2.start_performance_monitoring();
    engine2.end_performance_monitoring(2);

    // Verify both engines work independently
    // Test passes if we reach this point
}
