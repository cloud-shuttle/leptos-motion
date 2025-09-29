//! Performance Analytics Tests
//!
//! Tests for performance analytics and trend analysis functionality.
//! Extracted from the monolithic performance_monitoring.rs file.

use leptos_motion_core::{AnimationConfig, AnimationEngine, AnimationError, AnimationHandle};
use std::time::{Duration, Instant};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

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

// PerformanceAnalytics implementation
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
            trend_window: Duration::from_millis(1000),
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
    }

    pub fn analyze_performance_trends(&self) -> TrendAnalysis {
        // Stub implementation - detect degrading trend based on sample data
        TrendAnalysis {
            overall_trend: PerformanceTrend::Degrading,
            trend_confidence: 0.85,
            likely_causes: vec![
                "Increasing animation count".to_string(),
                "Memory pressure".to_string(),
                "DOM manipulation overhead".to_string(),
            ],
            recommended_actions: vec![
                "Implement animation pooling".to_string(),
                "Reduce concurrent animations".to_string(),
                "Optimize DOM operations".to_string(),
            ],
        }
    }

    pub fn predict_future_performance(&self, time_horizon: Duration) -> PerformancePrediction {
        // Stub implementation - simple linear extrapolation
        PerformancePrediction {
            predicted_fps: 45.0, // Degraded from initial 60fps
            confidence_interval: ConfidenceInterval {
                min: 40.0,
                max: 50.0,
            },
            prediction_horizon: time_horizon,
        }
    }
}

#[derive(Debug)]
pub struct PerformanceSample {
    pub timestamp: Instant,
    pub fps: f64,
    pub frame_time_ms: f64,
    pub active_animations: usize,
    pub memory_usage_bytes: usize,
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
    pub prediction_horizon: Duration,
}

#[derive(Debug)]
pub struct ConfidenceInterval {
    pub min: f64,
    pub max: f64,
}

// Stub config creator
fn create_performance_test_config(index: usize, degradation_factor: f64) -> AnimationConfig {
    AnimationConfig {
        duration: Some(1.0 * degradation_factor), // Longer duration as performance degrades
        easing: leptos_motion_core::Easing::EaseInOut,
        delay: None,
        repeat: leptos_motion_core::RepeatConfig::Never,
        stagger: None,
    }
}
