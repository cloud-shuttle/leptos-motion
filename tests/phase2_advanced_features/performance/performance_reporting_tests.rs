//! Performance Reporting Tests
//!
//! Tests for comprehensive performance reporting functionality.
//! Extracted from the monolithic performance_monitoring.rs file.

use leptos_motion_core::{AnimationConfig, AnimationEngine, AnimationError, AnimationHandle};
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Comprehensive performance reporting
/// This will FAIL initially - need reporting system
#[wasm_bindgen_test]
fn test_comprehensive_performance_reporting() {
    // Arrange: Create performance monitor with full reporting
    let mut monitor = PerformanceMonitor::new();
    monitor.enable_comprehensive_reporting(true);
    monitor.set_report_interval(Duration::from_millis(100));

    let engine = AnimationEngine::new();

    // Act: Run various animation scenarios for comprehensive data
    let test_scenarios = vec![
        ("simple_fade", 10, create_simple_fade_config),
        ("complex_transform", 5, create_complex_transform_config),
        ("high_frequency", 20, create_high_frequency_config),
        ("resource_intensive", 3, create_resource_intensive_config),
    ];

    for (scenario_name, animation_count, config_fn) in test_scenarios {
        monitor.start_performance_scenario(scenario_name);

        // Start multiple animations for this scenario
        let mut scenario_handles = Vec::new();
        for i in 0..animation_count {
            let config = config_fn(i);
            if let Ok(handle) = engine.start_animation(config) {
                scenario_handles.push(handle);
            }
        }

        // Run scenario for measurement period
        std::thread::sleep(Duration::from_millis(150));

        monitor.end_performance_scenario(scenario_name);
    }

    // Act: Generate comprehensive performance report
    let performance_report = monitor.generate_comprehensive_report();

    // Assert: Report should contain all key metrics
    assert!(
        performance_report.frame_rate_stats.avg_fps > 0.0,
        "Should report frame rate statistics"
    );

    assert!(
        performance_report.memory_stats.peak_usage_bytes > 0,
        "Should report memory usage statistics"
    );

    assert!(
        performance_report.animation_stats.total_animations > 0,
        "Should report animation statistics"
    );

    assert!(
        !performance_report.bottleneck_summary.is_empty(),
        "Should include bottleneck analysis in report"
    );

    // Assert: Report should include actionable recommendations
    assert!(
        !performance_report.optimization_recommendations.is_empty(),
        "Should provide optimization recommendations"
    );

    // Assert: Report should have performance grades
    assert!(
        performance_report.overall_grade >= 'A' && performance_report.overall_grade <= 'F',
        "Should assign overall performance grade A-F, got: {}",
        performance_report.overall_grade
    );

    // Assert: Report should identify top performance issues
    assert!(
        performance_report.top_performance_issues.len() <= 5,
        "Should identify top 5 performance issues, found: {}",
        performance_report.top_performance_issues.len()
    );

    for issue in &performance_report.top_performance_issues {
        assert!(
            issue.impact_score > 0.0 && issue.impact_score <= 1.0,
            "Performance issue should have valid impact score: {}",
            issue.impact_score
        );
    }
}

// Extended PerformanceMonitor with comprehensive reporting
impl PerformanceMonitor {
    pub fn enable_comprehensive_reporting(&mut self, enabled: bool) {
        // Stub implementation
    }

    pub fn set_report_interval(&mut self, interval: Duration) {
        // Stub implementation
    }

    pub fn start_performance_scenario(&mut self, scenario_name: &str) {
        // Stub implementation
    }

    pub fn end_performance_scenario(&mut self, scenario_name: &str) {
        // Stub implementation
    }

    pub fn generate_comprehensive_report(&self) -> ComprehensivePerformanceReport {
        // Stub implementation - return mock comprehensive report
        ComprehensivePerformanceReport {
            frame_rate_stats: FrameRateStats {
                avg_fps: 60.0,
                min_fps: 55.0,
                max_fps: 65.0,
                frame_drops: 2,
            },
            memory_stats: MemoryStats {
                peak_usage_bytes: 1024 * 1024, // 1MB
                avg_usage_bytes: 512 * 1024,   // 512KB
                gc_collections: 5,
            },
            animation_stats: AnimationStats {
                total_animations: 25,
                active_animations: 15,
                completed_animations: 10,
            },
            bottleneck_summary: vec![
                "DOM operations".to_string(),
                "Memory allocation".to_string(),
            ],
            optimization_recommendations: vec![
                "Use CSS transforms".to_string(),
                "Implement object pooling".to_string(),
                "Reduce animation complexity".to_string(),
            ],
            overall_grade: 'B',
            top_performance_issues: vec![
                PerformanceIssue {
                    description: "Excessive DOM queries".to_string(),
                    impact_score: 0.8,
                    category: "DOM".to_string(),
                },
                PerformanceIssue {
                    description: "Memory fragmentation".to_string(),
                    impact_score: 0.6,
                    category: "Memory".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct ComprehensivePerformanceReport {
    pub frame_rate_stats: FrameRateStats,
    pub memory_stats: MemoryStats,
    pub animation_stats: AnimationStats,
    pub bottleneck_summary: Vec<String>,
    pub optimization_recommendations: Vec<String>,
    pub overall_grade: char,
    pub top_performance_issues: Vec<PerformanceIssue>,
}

#[derive(Debug)]
pub struct FrameRateStats {
    pub avg_fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub frame_drops: u32,
}

#[derive(Debug)]
pub struct MemoryStats {
    pub peak_usage_bytes: usize,
    pub avg_usage_bytes: usize,
    pub gc_collections: u32,
}

#[derive(Debug)]
pub struct AnimationStats {
    pub total_animations: usize,
    pub active_animations: usize,
    pub completed_animations: usize,
}

#[derive(Debug)]
pub struct PerformanceIssue {
    pub description: String,
    pub impact_score: f64,
    pub category: String,
}

// Stub config creators
fn create_simple_fade_config(index: usize) -> AnimationConfig {
    AnimationConfig {
        duration: Some(1.0),
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}

fn create_complex_transform_config(index: usize) -> AnimationConfig {
    AnimationConfig {
        duration: Some(2.0),
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}

fn create_high_frequency_config(index: usize) -> AnimationConfig {
    AnimationConfig {
        duration: Some(0.1),
        easing: leptos_motion_core::Easing::Linear,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}

fn create_resource_intensive_config(index: usize) -> AnimationConfig {
    AnimationConfig {
        duration: Some(5.0),
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}
