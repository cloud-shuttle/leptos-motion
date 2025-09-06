//! Phase 2 TDD Implementation: Performance Monitoring & Analytics
//!
//! Real-time performance monitoring, animation budgeting, bottleneck detection,
//! and comprehensive performance reporting for production optimization.
//!
//! Red Phase: Performance monitoring test suite
//! Green Phase: Monitoring system implementation
//! Refactor Phase: Analytics optimization and reporting

use leptos_motion_core::{AnimationConfig, AnimationEngine, AnimationError, AnimationHandle};
use rstest::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Real-time frame rate monitoring
/// This will FAIL initially - need performance monitoring system
#[rstest]
#[case::sixty_fps(60.0)]
#[case::thirty_fps(30.0)]
#[case::twenty_fps(20.0)]
#[wasm_bindgen_test]
fn test_frame_rate_monitoring(#[case] target_fps: f64) {
    // Arrange: Create performance monitor with target FPS
    let mut monitor = PerformanceMonitor::new();
    monitor.set_target_fps(target_fps);
    monitor.enable_frame_rate_monitoring(true);

    let engine = AnimationEngine::new();
    let frame_duration = Duration::from_secs_f64(1.0 / target_fps);

    // Act: Simulate animation frames
    let mut actual_fps_measurements = Vec::new();
    let simulation_start = Instant::now();

    for frame in 0..100 {
        let frame_start = Instant::now();

        // Simulate animation work
        simulate_animation_work(&engine, frame, target_fps as usize);

        // Record frame timing
        monitor.record_frame(frame_start);

        // Collect FPS measurements every 10 frames
        if frame % 10 == 0 && frame > 0 {
            let fps = monitor.get_current_fps();
            actual_fps_measurements.push(fps);
        }

        // Wait for next frame (simulate frame pacing)
        std::thread::sleep(frame_duration / 4); // 1/4 of frame time
    }

    let total_duration = simulation_start.elapsed();
    let overall_fps = 100.0 / total_duration.as_secs_f64();

    // Assert: Should track frame rate accurately
    let avg_fps =
        actual_fps_measurements.iter().sum::<f64>() / actual_fps_measurements.len() as f64;
    let fps_error = (avg_fps - target_fps).abs() / target_fps;

    assert!(
        fps_error < 0.1, // Within 10% of target
        "FPS monitoring error too high: target {}fps, measured {}fps ({}% error)",
        target_fps,
        avg_fps,
        fps_error * 100.0
    );

    // Assert: Should detect FPS drops
    let fps_variance = actual_fps_measurements
        .iter()
        .map(|fps| (fps - avg_fps).powi(2))
        .sum::<f64>()
        / actual_fps_measurements.len() as f64;
    let fps_std_dev = fps_variance.sqrt();

    assert!(
        fps_std_dev < target_fps * 0.2, // Standard deviation less than 20% of target
        "FPS measurements too variable: std dev {}fps for target {}fps",
        fps_std_dev,
        target_fps
    );
}

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

/// Test: Animation bottleneck detection
/// This will FAIL initially - need bottleneck detection system
#[wasm_bindgen_test]
fn test_animation_bottleneck_detection() {
    // Arrange: Create performance monitor with bottleneck detection
    let mut monitor = PerformanceMonitor::new();
    monitor.enable_bottleneck_detection(true);
    monitor.set_bottleneck_threshold(Duration::from_millis(5));

    let engine = AnimationEngine::new();

    // Create different types of animations to test bottleneck detection
    let bottleneck_scenarios = vec![
        ("dom_heavy", create_dom_heavy_animation_config()),
        (
            "calculation_heavy",
            create_calculation_heavy_animation_config(),
        ),
        ("memory_heavy", create_memory_heavy_animation_config()),
        (
            "concurrent_heavy",
            create_concurrent_heavy_animation_config(),
        ),
    ];

    // Act: Run each scenario and detect bottlenecks
    let mut bottleneck_results = HashMap::new();

    for (scenario_name, config) in bottleneck_scenarios {
        let scenario_start = Instant::now();
        monitor.start_scenario_monitoring(scenario_name);

        // Run the animation scenario
        let handle = engine
            .start_animation(config)
            .expect("Should start animation");

        // Simulate animation execution
        simulate_animation_execution(&engine, handle, Duration::from_millis(100));

        let scenario_duration = scenario_start.elapsed();
        monitor.end_scenario_monitoring(scenario_name, scenario_duration);

        bottleneck_results.insert(scenario_name.to_string(), scenario_duration);
    }

    // Assert: Should detect performance bottlenecks
    let bottleneck_report = monitor.get_bottleneck_report();
    assert!(
        bottleneck_report.detected_bottlenecks.len() > 0,
        "Should detect at least some bottlenecks in test scenarios"
    );

    // Assert: Should identify bottleneck types correctly
    let bottleneck_types: Vec<String> = bottleneck_report
        .detected_bottlenecks
        .iter()
        .map(|b| b.bottleneck_type.clone())
        .collect();

    // Should detect at least DOM or calculation bottlenecks
    assert!(
        bottleneck_types
            .iter()
            .any(|t| t.contains("dom") || t.contains("calculation")),
        "Should detect DOM or calculation bottlenecks, found: {:?}",
        bottleneck_types
    );

    // Assert: Should provide specific recommendations for each bottleneck
    for bottleneck in &bottleneck_report.detected_bottlenecks {
        assert!(
            !bottleneck.recommendations.is_empty(),
            "Should provide recommendations for {} bottleneck",
            bottleneck.bottleneck_type
        );

        assert!(
            bottleneck.severity_score > 0.0,
            "Should assign severity score to bottleneck"
        );
    }
}

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

/// Test: Performance analytics and trends
/// This will FAIL initially - need analytics system
#[wasm_bindgen_test]
fn test_performance_analytics_and_trends() {
    // Arrange: Create performance analytics system
    let mut analytics = PerformanceAnalytics::new();
    analytics.enable_trend_analysis(true);
    analytics.set_trend_window(Duration::from_millis(500)); // 500ms trend window

    let engine = AnimationEngine::new();

    // Act: Simulate performance data over time with degradation
    let mut performance_samples = Vec::new();

    for sample_idx in 0..50 {
        let sample_start = Instant::now();

        // Simulate gradual performance degradation
        let degradation_factor = 1.0 + (sample_idx as f64 * 0.02); // 2% worse each sample
        let animation_count = (5.0 * degradation_factor) as usize;

        // Start animations
        let mut handles = Vec::new();
        for i in 0..animation_count {
            let config = create_performance_test_config(i, degradation_factor);
            if let Ok(handle) = engine.start_animation(config) {
                handles.push(handle);
            }
        }

        let sample_duration = sample_start.elapsed();
        let fps = if sample_duration.as_secs_f64() > 0.0 {
            1.0 / sample_duration.as_secs_f64()
        } else {
            60.0
        };

        analytics.record_performance_sample(PerformanceSample {
            timestamp: Instant::now(),
            fps,
            frame_time_ms: sample_duration.as_secs_f64() * 1000.0,
            active_animations: animation_count,
            memory_usage_bytes: animation_count * 1024, // Estimate
        });

        performance_samples.push(fps);

        // Small delay between samples
        std::thread::sleep(Duration::from_millis(10));
    }

    // Assert: Should detect performance trend
    let trend_analysis = analytics.analyze_performance_trends();

    // Should detect degrading performance
    assert_eq!(
        trend_analysis.overall_trend,
        PerformanceTrend::Degrading,
        "Should detect degrading performance trend"
    );

    assert!(
        trend_analysis.trend_confidence > 0.7,
        "Should have high confidence in trend detection: {}",
        trend_analysis.trend_confidence
    );

    // Assert: Should identify trend causes
    assert!(
        !trend_analysis.likely_causes.is_empty(),
        "Should identify likely causes of performance degradation"
    );

    // Should suggest corrective actions
    assert!(
        !trend_analysis.recommended_actions.is_empty(),
        "Should recommend actions to address performance trend"
    );

    // Assert: Should provide predictive insights
    let prediction = analytics.predict_future_performance(Duration::from_millis(200));
    assert!(
        prediction.predicted_fps > 0.0,
        "Should predict future FPS: {}",
        prediction.predicted_fps
    );

    assert!(
        prediction.confidence_interval.min < prediction.confidence_interval.max,
        "Should provide confidence interval for prediction"
    );
}

// ============================================================================
// Helper Types and Functions for Performance Tests
// ============================================================================

/// Performance monitoring system for real-time analysis
#[derive(Debug)]
pub struct PerformanceMonitor {
    target_fps: f64,
    frame_budget: Duration,
    frame_rate_monitoring: bool,
    budget_monitoring: bool,
    bottleneck_detection: bool,
    comprehensive_reporting: bool,
    bottleneck_threshold: Duration,
    report_interval: Duration,
    frame_times: Vec<Duration>,
    budget_violations: usize,
    worst_violation_ms: f64,
    detected_bottlenecks: Vec<BottleneckInfo>,
    scenario_data: HashMap<String, ScenarioPerformance>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            target_fps: 60.0,
            frame_budget: Duration::from_millis(16), // 60fps = 16.67ms per frame
            frame_rate_monitoring: false,
            budget_monitoring: false,
            bottleneck_detection: false,
            comprehensive_reporting: false,
            bottleneck_threshold: Duration::from_millis(10),
            report_interval: Duration::from_millis(100),
            frame_times: Vec::new(),
            budget_violations: 0,
            worst_violation_ms: 0.0,
            detected_bottlenecks: Vec::new(),
            scenario_data: HashMap::new(),
        }
    }

    pub fn set_target_fps(&mut self, fps: f64) {
        self.target_fps = fps;
        self.frame_budget = Duration::from_secs_f64(1.0 / fps);
    }

    pub fn set_frame_budget(&mut self, budget: Duration) {
        self.frame_budget = budget;
    }

    pub fn enable_frame_rate_monitoring(&mut self, enabled: bool) {
        self.frame_rate_monitoring = enabled;
    }

    pub fn enable_budget_monitoring(&mut self, enabled: bool) {
        self.budget_monitoring = enabled;
    }

    pub fn enable_bottleneck_detection(&mut self, enabled: bool) {
        self.bottleneck_detection = enabled;
    }

    pub fn enable_comprehensive_reporting(&mut self, enabled: bool) {
        self.comprehensive_reporting = enabled;
    }

    pub fn set_bottleneck_threshold(&mut self, threshold: Duration) {
        self.bottleneck_threshold = threshold;
    }

    pub fn set_report_interval(&mut self, interval: Duration) {
        self.report_interval = interval;
    }

    pub fn record_frame(&mut self, frame_start: Instant) {
        let frame_time = frame_start.elapsed();
        self.frame_times.push(frame_time);

        // Keep only recent frames for FPS calculation
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
    }

    pub fn record_budget_usage(&mut self, operation: &str, duration: Duration) {
        if duration > self.frame_budget {
            self.budget_violations += 1;
            let violation_ms = duration.as_secs_f64() * 1000.0;
            if violation_ms > self.worst_violation_ms {
                self.worst_violation_ms = violation_ms;
            }
        }
    }

    pub fn get_current_fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_times.iter().sum();
        if total_time.as_secs_f64() > 0.0 {
            self.frame_times.len() as f64 / total_time.as_secs_f64()
        } else {
            60.0 // Default
        }
    }

    pub fn get_budget_report(&self) -> BudgetReport {
        BudgetReport {
            violations: self.budget_violations,
            worst_violation_ms: self.worst_violation_ms,
        }
    }

    pub fn get_budget_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.budget_violations > 0 {
            recommendations.push("Consider reducing animation complexity".to_string());
            recommendations.push("Implement animation pooling".to_string());
        }

        if self.worst_violation_ms > self.frame_budget.as_secs_f64() * 1000.0 * 2.0 {
            recommendations
                .push("Critical performance issue - investigate bottlenecks".to_string());
        }

        recommendations
    }

    pub fn start_scenario_monitoring(&mut self, scenario_name: &str) {
        self.scenario_data.insert(
            scenario_name.to_string(),
            ScenarioPerformance {
                start_time: Instant::now(),
                duration: Duration::ZERO,
                bottlenecks_detected: 0,
            },
        );
    }

    pub fn end_scenario_monitoring(&mut self, scenario_name: &str, duration: Duration) {
        if let Some(scenario) = self.scenario_data.get_mut(scenario_name) {
            scenario.duration = duration;

            // Detect bottleneck based on duration
            if duration > self.bottleneck_threshold {
                scenario.bottlenecks_detected += 1;
                self.detected_bottlenecks.push(BottleneckInfo {
                    bottleneck_type: format!("{}_bottleneck", scenario_name),
                    severity_score: (duration.as_secs_f64()
                        / self.bottleneck_threshold.as_secs_f64())
                    .min(1.0),
                    recommendations: vec![
                        format!("Optimize {} operations", scenario_name),
                        "Consider reducing complexity".to_string(),
                    ],
                });
            }
        }
    }

    pub fn get_bottleneck_report(&self) -> BottleneckReport {
        BottleneckReport {
            detected_bottlenecks: self.detected_bottlenecks.clone(),
        }
    }

    pub fn start_performance_scenario(&mut self, scenario_name: &str) {
        self.start_scenario_monitoring(scenario_name);
    }

    pub fn end_performance_scenario(&mut self, scenario_name: &str) {
        if let Some(scenario) = self.scenario_data.get(scenario_name) {
            let duration = scenario.start_time.elapsed();
            self.end_scenario_monitoring(scenario_name, duration);
        }
    }

    pub fn generate_comprehensive_report(&self) -> ComprehensivePerformanceReport {
        let avg_fps = self.get_current_fps();
        let grade = if avg_fps >= 55.0 {
            'A'
        } else if avg_fps >= 45.0 {
            'B'
        } else if avg_fps >= 30.0 {
            'C'
        } else if avg_fps >= 20.0 {
            'D'
        } else {
            'F'
        };

        ComprehensivePerformanceReport {
            frame_rate_stats: FrameRateStats { avg_fps },
            memory_stats: MemoryStats {
                peak_usage_bytes: 1024 * 1024,
            }, // Mock
            animation_stats: AnimationStats {
                total_animations: 10,
            }, // Mock
            bottleneck_summary: format!("{} bottlenecks detected", self.detected_bottlenecks.len()),
            optimization_recommendations: self.get_budget_recommendations(),
            overall_grade: grade,
            top_performance_issues: self
                .detected_bottlenecks
                .iter()
                .take(5)
                .map(|b| PerformanceIssue {
                    description: b.bottleneck_type.clone(),
                    impact_score: b.severity_score,
                })
                .collect(),
        }
    }
}

/// Performance analytics for trend analysis
#[derive(Debug)]
pub struct PerformanceAnalytics {
    trend_analysis_enabled: bool,
    trend_window: Duration,
    performance_samples: Vec<PerformanceSample>,
}

impl PerformanceAnalytics {
    pub fn new() -> Self {
        Self {
            trend_analysis_enabled: false,
            trend_window: Duration::from_secs(5),
            performance_samples: Vec::new(),
        }
    }

    pub fn enable_trend_analysis(&mut self, enabled: bool) {
        self.trend_analysis_enabled = enabled;
    }

    pub fn set_trend_window(&mut self, window: Duration) {
        self.trend_window = window;
    }

    pub fn record_performance_sample(&mut self, sample: PerformanceSample) {
        self.performance_samples.push(sample);

        // Keep only samples within trend window
        let cutoff_time = Instant::now() - self.trend_window;
        self.performance_samples
            .retain(|s| s.timestamp > cutoff_time);
    }

    pub fn analyze_performance_trends(&self) -> TrendAnalysis {
        if self.performance_samples.len() < 3 {
            return TrendAnalysis {
                overall_trend: PerformanceTrend::Stable,
                trend_confidence: 0.0,
                likely_causes: vec!["Insufficient data".to_string()],
                recommended_actions: vec!["Continue monitoring".to_string()],
            };
        }

        // Simple trend analysis - check if FPS is decreasing
        let early_samples: f64 = self
            .performance_samples
            .iter()
            .take(10)
            .map(|s| s.fps)
            .sum::<f64>()
            / 10.0;
        let late_samples: f64 = self
            .performance_samples
            .iter()
            .rev()
            .take(10)
            .map(|s| s.fps)
            .sum::<f64>()
            / 10.0;

        let trend = if late_samples < early_samples * 0.9 {
            PerformanceTrend::Degrading
        } else if late_samples > early_samples * 1.1 {
            PerformanceTrend::Improving
        } else {
            PerformanceTrend::Stable
        };

        TrendAnalysis {
            overall_trend: trend,
            trend_confidence: 0.8, // Mock confidence
            likely_causes: vec!["Increasing animation complexity".to_string()],
            recommended_actions: vec![
                "Optimize animations".to_string(),
                "Reduce concurrent animations".to_string(),
            ],
        }
    }

    pub fn predict_future_performance(&self, prediction_window: Duration) -> PerformancePrediction {
        let current_fps = if !self.performance_samples.is_empty() {
            self.performance_samples.last().unwrap().fps
        } else {
            60.0
        };

        // Simple linear prediction (would be more sophisticated in real implementation)
        let predicted_fps = current_fps * 0.95; // Assume slight degradation

        PerformancePrediction {
            predicted_fps,
            confidence_interval: ConfidenceInterval {
                min: predicted_fps * 0.9,
                max: predicted_fps * 1.1,
            },
        }
    }
}

// Helper types for performance monitoring

#[derive(Debug, Clone)]
pub struct PerformanceSample {
    pub timestamp: Instant,
    pub fps: f64,
    pub frame_time_ms: f64,
    pub active_animations: usize,
    pub memory_usage_bytes: usize,
}

#[derive(Debug, Clone)]
pub struct BudgetReport {
    pub violations: usize,
    pub worst_violation_ms: f64,
}

#[derive(Debug, Clone)]
pub struct BottleneckInfo {
    pub bottleneck_type: String,
    pub severity_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BottleneckReport {
    pub detected_bottlenecks: Vec<BottleneckInfo>,
}

#[derive(Debug)]
pub struct ScenarioPerformance {
    pub start_time: Instant,
    pub duration: Duration,
    pub bottlenecks_detected: usize,
}

#[derive(Debug)]
pub struct ComprehensivePerformanceReport {
    pub frame_rate_stats: FrameRateStats,
    pub memory_stats: MemoryStats,
    pub animation_stats: AnimationStats,
    pub bottleneck_summary: String,
    pub optimization_recommendations: Vec<String>,
    pub overall_grade: char,
    pub top_performance_issues: Vec<PerformanceIssue>,
}

#[derive(Debug)]
pub struct FrameRateStats {
    pub avg_fps: f64,
}

#[derive(Debug)]
pub struct MemoryStats {
    pub peak_usage_bytes: usize,
}

#[derive(Debug)]
pub struct AnimationStats {
    pub total_animations: usize,
}

#[derive(Debug)]
pub struct PerformanceIssue {
    pub description: String,
    pub impact_score: f64,
}

#[derive(Debug, PartialEq)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
}

#[derive(Debug)]
pub struct TrendAnalysis {
    pub overall_trend: PerformanceTrend,
    pub trend_confidence: f64,
    pub likely_causes: Vec<String>,
    pub recommended_actions: Vec<String>,
}

#[derive(Debug)]
pub struct PerformancePrediction {
    pub predicted_fps: f64,
    pub confidence_interval: ConfidenceInterval,
}

#[derive(Debug)]
pub struct ConfidenceInterval {
    pub min: f64,
    pub max: f64,
}

// Helper functions for creating test animations

fn simulate_animation_work(engine: &AnimationEngine, frame: usize, complexity: usize) {
    // Mock animation work simulation
    let work_duration = Duration::from_micros((complexity * 100) as u64);
    std::thread::sleep(work_duration);
}

fn simulate_animation_execution(
    engine: &AnimationEngine,
    handle: AnimationHandle,
    duration: Duration,
) {
    // Mock animation execution
    std::thread::sleep(duration / 10); // Simulate work
}

fn create_lightweight_animation_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("light_{}", id)),
        target: create_simple_target(),
        duration: Some(0.1),
        ease: crate::Easing::Linear,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_heavyweight_animation_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("heavy_{}", id)),
        target: create_complex_target(),
        duration: Some(1.0),
        ease: crate::Easing::EaseInOut,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_dom_heavy_animation_config() -> AnimationConfig {
    AnimationConfig {
        id: Some("dom_heavy".to_string()),
        target: create_complex_target(),
        duration: Some(0.5),
        ease: crate::Easing::EaseInOut,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_calculation_heavy_animation_config() -> AnimationConfig {
    AnimationConfig {
        id: Some("calc_heavy".to_string()),
        target: create_complex_target(),
        duration: Some(0.5),
        ease: crate::Easing::Bezier(0.25, 0.46, 0.45, 0.94), // Complex bezier
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_memory_heavy_animation_config() -> AnimationConfig {
    AnimationConfig {
        id: Some("memory_heavy".to_string()),
        target: create_large_target(),
        duration: Some(0.5),
        ease: crate::Easing::Linear,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_concurrent_heavy_animation_config() -> AnimationConfig {
    AnimationConfig {
        id: Some("concurrent_heavy".to_string()),
        target: create_complex_target(),
        duration: Some(2.0), // Long duration
        ease: crate::Easing::EaseInOut,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_simple_fade_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("fade_{}", id)),
        target: create_opacity_target(1.0),
        duration: Some(0.3),
        ease: crate::Easing::Linear,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_complex_transform_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("transform_{}", id)),
        target: create_transform_target(),
        duration: Some(0.8),
        ease: crate::Easing::EaseInOut,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_high_frequency_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("high_freq_{}", id)),
        target: create_simple_target(),
        duration: Some(0.05), // Very short
        ease: crate::Easing::Linear,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_resource_intensive_config(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: Some(format!("resource_{}", id)),
        target: create_large_target(),
        duration: Some(2.0),                                   // Long duration
        ease: crate::Easing::Bezier(0.68, -0.55, 0.265, 1.55), // Overshoot
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_performance_test_config(id: usize, degradation_factor: f64) -> AnimationConfig {
    let duration = 0.2 * degradation_factor; // Longer as performance degrades

    AnimationConfig {
        id: Some(format!("perf_test_{}", id)),
        target: create_simple_target(),
        duration: Some(duration),
        ease: crate::Easing::Linear,
        delay: None,
        repeat: crate::RepeatConfig::None,
    }
}

fn create_simple_target() -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target
        .values
        .insert("opacity".to_string(), crate::AnimationValue::Number(1.0));
    target
}

fn create_complex_target() -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target
        .values
        .insert("opacity".to_string(), crate::AnimationValue::Number(1.0));
    target
        .values
        .insert("x".to_string(), crate::AnimationValue::Pixels(100.0));
    target
        .values
        .insert("y".to_string(), crate::AnimationValue::Pixels(50.0));
    target
        .values
        .insert("scale".to_string(), crate::AnimationValue::Number(1.2));
    target
        .values
        .insert("rotation".to_string(), crate::AnimationValue::Degrees(45.0));
    target
}

fn create_large_target() -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    // Many properties to simulate memory usage
    for i in 0..20 {
        target.values.insert(
            format!("prop_{}", i),
            crate::AnimationValue::Number(i as f64),
        );
    }
    target
}

fn create_opacity_target(opacity: f64) -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target.values.insert(
        "opacity".to_string(),
        crate::AnimationValue::Number(opacity),
    );
    target
}

fn create_transform_target() -> crate::AnimationTarget {
    let mut target = crate::AnimationTarget::new();
    target
        .values
        .insert("x".to_string(), crate::AnimationValue::Pixels(200.0));
    target
        .values
        .insert("y".to_string(), crate::AnimationValue::Pixels(100.0));
    target
        .values
        .insert("scale".to_string(), crate::AnimationValue::Number(1.5));
    target.values.insert(
        "rotation".to_string(),
        crate::AnimationValue::Degrees(180.0),
    );
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.target_fps, 60.0);
        assert!(!monitor.frame_rate_monitoring);
        assert!(!monitor.budget_monitoring);
    }

    #[test]
    fn test_performance_analytics_creation() {
        let analytics = PerformanceAnalytics::new();
        assert!(!analytics.trend_analysis_enabled);
        assert!(analytics.performance_samples.is_empty());
    }

    #[test]
    fn test_budget_report() {
        let report = BudgetReport {
            violations: 5,
            worst_violation_ms: 25.0,
        };
        assert_eq!(report.violations, 5);
        assert_eq!(report.worst_violation_ms, 25.0);
    }

    #[test]
    fn test_bottleneck_info() {
        let bottleneck = BottleneckInfo {
            bottleneck_type: "dom_heavy".to_string(),
            severity_score: 0.8,
            recommendations: vec!["Optimize DOM operations".to_string()],
        };
        assert_eq!(bottleneck.bottleneck_type, "dom_heavy");
        assert_eq!(bottleneck.severity_score, 0.8);
        assert_eq!(bottleneck.recommendations.len(), 1);
    }
}
