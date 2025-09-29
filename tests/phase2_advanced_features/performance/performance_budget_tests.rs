//! Performance Budget Tests
//!
//! Tests for animation performance budgeting functionality.
//! Extracted from the monolithic performance_monitoring.rs file.

use leptos_motion_core::{AnimationConfig, AnimationEngine, AnimationError, AnimationHandle};
use rstest::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Animation performance budgeting
/// This will FAIL initially - need budget management system
#[rstest]
#[case::conservative_budget(5.0)] // 5ms per frame budget
#[case::moderate_budget(10.0)] // 10ms per frame budget
#[case::generous_budget(16.0)] // 16ms per frame budget (60fps target)
#[wasm_bindgen_test]
fn test_animation_performance_budgets(#[case] budget_ms: f64) {
    // Arrange: Create performance monitor with budget
    let mut monitor = PerformanceMonitor::new();
    monitor.set_frame_budget(Duration::from_secs_f64(budget_ms / 1000.0));
    monitor.enable_budget_monitoring(true);

    let engine = AnimationEngine::new();

    // Create animations that should fit within budget
    let light_animation_count = (budget_ms / 2.0) as usize; // 2ms per animation

    // Act: Start animations within budget
    let mut handles = Vec::new();
    let budget_start = Instant::now();

    for i in 0..light_animation_count {
        let config = create_lightweight_animation_config(i);
        if let Ok(handle) = engine.start_animation(config) {
            handles.push(handle);
        }
    }

    let budget_used = budget_start.elapsed();
    monitor.record_budget_usage("animation_startup", budget_used);

    // Assert: Should stay within budget
    assert!(
        budget_used.as_secs_f64() * 1000.0 < budget_ms,
        "Animation startup exceeded budget: {}ms used, {}ms budget",
        budget_used.as_secs_f64() * 1000.0,
        budget_ms
    );

    // Act: Test budget warning system
    let over_budget_count = light_animation_count * 3; // Intentionally exceed budget
    let warning_start = Instant::now();

    for i in light_animation_count..over_budget_count {
        let config = create_heavyweight_animation_config(i); // Expensive animations
        let _ = engine.start_animation(config);
    }

    let warning_duration = warning_start.elapsed();
    let budget_exceeded = warning_duration.as_secs_f64() * 1000.0 > budget_ms;

    // Assert: Should detect budget violations
    if budget_exceeded {
        let budget_report = monitor.get_budget_report();
        assert!(
            budget_report.violations > 0,
            "Should detect budget violations when exceeded"
        );
        assert!(
            budget_report.worst_violation_ms > budget_ms,
            "Should track worst violation: {}ms vs {}ms budget",
            budget_report.worst_violation_ms,
            budget_ms
        );
    }

    // Assert: Should provide budget recommendations
    let recommendations = monitor.get_budget_recommendations();
    assert!(
        !recommendations.is_empty(),
        "Should provide budget optimization recommendations"
    );
}

// Extended PerformanceMonitor with budget functionality
impl PerformanceMonitor {
    pub fn set_frame_budget(&mut self, budget: Duration) {
        // Stub implementation
    }

    pub fn enable_budget_monitoring(&mut self, enabled: bool) {
        // Stub implementation
    }

    pub fn record_budget_usage(&mut self, category: &str, duration: Duration) {
        // Stub implementation
    }

    pub fn get_budget_report(&self) -> BudgetReport {
        // Stub implementation - return empty report
        BudgetReport {
            violations: 0,
            worst_violation_ms: 0.0,
            total_budget_used_ms: 0.0,
        }
    }

    pub fn get_budget_recommendations(&self) -> Vec<String> {
        // Stub implementation - return some recommendations
        vec![
            "Reduce animation complexity".to_string(),
            "Implement object pooling".to_string(),
            "Use hardware acceleration".to_string(),
        ]
    }
}

#[derive(Debug)]
pub struct BudgetReport {
    pub violations: usize,
    pub worst_violation_ms: f64,
    pub total_budget_used_ms: f64,
}

/// Create a lightweight animation config for testing
fn create_lightweight_animation_config(index: usize) -> AnimationConfig {
    // Stub implementation - create minimal config
    AnimationConfig {
        duration: Some(0.5),
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}

/// Create a heavyweight animation config for testing
fn create_heavyweight_animation_config(index: usize) -> AnimationConfig {
    // Stub implementation - create complex config
    AnimationConfig {
        duration: Some(2.0),
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}
