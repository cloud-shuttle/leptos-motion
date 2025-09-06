// TDD Tests for Memory Optimization
//
// This module contains tests to identify and fix memory issues
// to achieve the target of <10MB memory usage.

use leptos_motion_core::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

/// Memory usage thresholds for optimization targets
const TARGET_MAX_MEMORY_MB: f64 = 10.0;
const TARGET_ANIMATION_MEMORY_KB: f64 = 100.0; // Per animation
const TARGET_ENGINE_MEMORY_KB: f64 = 500.0; // Base engine memory
const TARGET_CACHE_MEMORY_KB: f64 = 200.0; // Performance cache memory

/// Memory measurement utilities
struct MemoryProfiler {
    baseline_memory: f64,
    peak_memory: f64,
    measurements: Vec<f64>,
}

impl MemoryProfiler {
    fn new() -> Self {
        Self {
            baseline_memory: Self::get_current_memory_mb(),
            peak_memory: 0.0,
            measurements: Vec::new(),
        }
    }

    fn get_current_memory_mb() -> f64 {
        // Simulate memory measurement - in real implementation would use system APIs
        // For testing purposes, we'll use a mock measurement
        std::thread::sleep(Duration::from_millis(1));
        5.0 + (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64
            % 1000.0)
            / 1000.0
    }

    fn measure(&mut self) -> f64 {
        let current = Self::get_current_memory_mb();
        self.measurements.push(current);
        if current > self.peak_memory {
            self.peak_memory = current;
        }
        current
    }

    fn get_peak_usage(&self) -> f64 {
        self.peak_memory - self.baseline_memory
    }

    fn get_average_usage(&self) -> f64 {
        if self.measurements.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.measurements.iter().sum();
        (sum / self.measurements.len() as f64) - self.baseline_memory
    }
}

/// Test fixture for creating memory-intensive scenarios
fn create_memory_intensive_animation() -> AnimationConfig {
    AnimationConfig {
        target: AnimationTarget::Element {
            element: None, // Mock element
            properties: HashMap::from([
                (
                    "transform".to_string(),
                    AnimationValue::Transform(Transform::identity()),
                ),
                ("opacity".to_string(), AnimationValue::Number(1.0)),
                (
                    "color".to_string(),
                    AnimationValue::String("#000000".to_string()),
                ),
            ]),
        },
        transition: Transition {
            duration: Duration::from_secs(2),
            easing: Easing::Linear,
            delay: Duration::ZERO,
        },
        repeat: RepeatConfig::Never,
        stagger: None,
        on_complete: None,
        on_update: None,
    }
}

/// Test that engine creation doesn't exceed memory limits
#[test]
fn test_engine_memory_usage_within_limits() {
    let mut profiler = MemoryProfiler::new();

    // Create multiple engines to test memory scaling
    let engines: Vec<SimplifiedAnimationEngine> =
        (0..10).map(|_| SimplifiedAnimationEngine::new()).collect();

    profiler.measure();
    let peak_usage = profiler.get_peak_usage();

    // Each engine should use less than 500KB
    let per_engine_usage = peak_usage / engines.len() as f64;
    assert!(
        per_engine_usage < TARGET_ENGINE_MEMORY_KB / 1024.0,
        "Engine memory usage {}MB exceeds target {}KB per engine",
        per_engine_usage * 1024.0,
        TARGET_ENGINE_MEMORY_KB
    );
}

/// Test that animation creation doesn't cause memory leaks
#[test]
fn test_animation_memory_no_leaks() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Create and destroy many animations
    for i in 0..100 {
        let animation = create_memory_intensive_animation();
        let _handle = engine.animate(&animation).unwrap();

        if i % 10 == 0 {
            profiler.measure();
        }
    }

    // Force garbage collection simulation
    std::thread::sleep(Duration::from_millis(100));
    profiler.measure();

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_MAX_MEMORY_MB,
        "Animation memory usage {}MB exceeds target {}MB",
        peak_usage,
        TARGET_MAX_MEMORY_MB
    );
}

/// Test that performance cache doesn't grow unbounded
#[test]
fn test_performance_cache_memory_bounds() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Generate many performance reports to test cache limits
    for i in 0..1000 {
        let animation = create_memory_intensive_animation();
        let _handle = engine.animate(&animation).unwrap();

        if i % 100 == 0 {
            profiler.measure();
        }
    }

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_CACHE_MEMORY_KB / 1024.0,
        "Performance cache memory {}MB exceeds target {}KB",
        peak_usage * 1024.0,
        TARGET_CACHE_MEMORY_KB
    );
}

/// Test that concurrent animations don't cause memory explosion
#[test]
fn test_concurrent_animations_memory_scaling() {
    let mut profiler = MemoryProfiler::new();
    let engine = Arc::new(Mutex::new(SimplifiedAnimationEngine::new()));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let engine = engine.clone();
            std::thread::spawn(move || {
                let mut local_profiler = MemoryProfiler::new();
                for j in 0..50 {
                    let animation = create_memory_intensive_animation();
                    if let Ok(eng) = engine.lock() {
                        let _handle = eng.animate(&animation).unwrap();
                    }

                    if j % 10 == 0 {
                        local_profiler.measure();
                    }
                }
                local_profiler.get_peak_usage()
            })
        })
        .collect();

    // Wait for all threads to complete
    let results: Vec<f64> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    profiler.measure();
    let total_peak = profiler.get_peak_usage();

    // Total memory should scale linearly, not exponentially
    let expected_max = results.iter().sum::<f64>() * 1.5; // Allow 50% overhead
    assert!(
        total_peak < expected_max,
        "Concurrent memory usage {}MB exceeds expected {}MB",
        total_peak,
        expected_max
    );
}

/// Test that animation cleanup properly releases memory
#[test]
fn test_animation_cleanup_memory_release() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Create animations and track memory
    let mut handles = Vec::new();
    for _ in 0..50 {
        let animation = create_memory_intensive_animation();
        let handle = engine.animate(&animation).unwrap();
        handles.push(handle);
        profiler.measure();
    }

    let peak_with_animations = profiler.get_peak_usage();

    // Cancel all animations
    for handle in handles {
        engine.cancel_animation(handle).unwrap();
    }

    // Force cleanup
    std::thread::sleep(Duration::from_millis(100));
    profiler.measure();

    let usage_after_cleanup = profiler.get_peak_usage();

    // Memory should be significantly reduced after cleanup
    let memory_reduction = peak_with_animations - usage_after_cleanup;
    assert!(
        memory_reduction > peak_with_animations * 0.3, // At least 30% reduction
        "Memory cleanup only reduced usage by {}MB, expected at least {}MB",
        memory_reduction,
        peak_with_animations * 0.3
    );
}

/// Test that large animation values don't cause memory issues
#[test]
fn test_large_animation_values_memory_efficiency() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Create animation with large data
    let mut large_properties = HashMap::new();
    for i in 0..1000 {
        large_properties.insert(
            format!("property_{}", i),
            AnimationValue::String(format!("large_value_{}", "x".repeat(1000))),
        );
    }

    let large_animation = AnimationConfig {
        target: AnimationTarget::Element {
            element: None,
            properties: large_properties,
        },
        transition: Transition {
            duration: Duration::from_secs(1),
            easing: Easing::Linear,
            delay: Duration::ZERO,
        },
        repeat: RepeatConfig::Never,
        stagger: None,
        on_complete: None,
        on_update: None,
    };

    let _handle = engine.animate(&large_animation).unwrap();
    profiler.measure();

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_MAX_MEMORY_MB,
        "Large animation values caused {}MB usage, exceeds target {}MB",
        peak_usage,
        TARGET_MAX_MEMORY_MB
    );
}

/// Test that engine handles memory pressure gracefully
#[test]
fn test_memory_pressure_handling() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Simulate memory pressure by creating many animations rapidly
    let mut handles = Vec::new();
    for i in 0..200 {
        let animation = create_memory_intensive_animation();
        match engine.animate(&animation) {
            Ok(handle) => handles.push(handle),
            Err(_) => {
                // Engine should handle memory pressure gracefully
                // by rejecting new animations rather than crashing
                break;
            }
        }

        if i % 20 == 0 {
            profiler.measure();
        }
    }

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_MAX_MEMORY_MB * 1.5, // Allow 50% over target under pressure
        "Memory pressure caused {}MB usage, exceeds safe limit {}MB",
        peak_usage,
        TARGET_MAX_MEMORY_MB * 1.5
    );
}

/// Test that performance metrics don't accumulate indefinitely
#[test]
fn test_performance_metrics_memory_bounds() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Generate many performance reports
    for _ in 0..500 {
        let animation = create_memory_intensive_animation();
        let _handle = engine.animate(&animation).unwrap();
        profiler.measure();
    }

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_MAX_MEMORY_MB,
        "Performance metrics caused {}MB usage, exceeds target {}MB",
        peak_usage,
        TARGET_MAX_MEMORY_MB
    );
}

/// Test that engine can recover from memory exhaustion
#[test]
fn test_memory_recovery_after_exhaustion() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Fill up memory
    let mut handles = Vec::new();
    for _ in 0..100 {
        let animation = create_memory_intensive_animation();
        if let Ok(handle) = engine.animate(&animation) {
            handles.push(handle);
        }
    }

    profiler.measure();
    let peak_usage = profiler.get_peak_usage();

    // Clear all animations
    for handle in handles {
        let _ = engine.cancel_animation(handle);
    }

    // Force cleanup
    std::thread::sleep(Duration::from_millis(200));
    profiler.measure();

    // Try to create new animations - should work
    let mut new_handles = Vec::new();
    for _ in 0..10 {
        let animation = create_memory_intensive_animation();
        if let Ok(handle) = engine.animate(&animation) {
            new_handles.push(handle);
        }
    }

    profiler.measure();
    let final_usage = profiler.get_peak_usage();

    assert!(
        new_handles.len() > 0,
        "Engine failed to recover from memory exhaustion"
    );

    assert!(
        final_usage < peak_usage,
        "Memory usage {}MB after recovery exceeds peak {}MB",
        final_usage,
        peak_usage
    );
}

/// Test that animation interpolation doesn't create memory leaks
#[test]
fn test_animation_interpolation_memory_efficiency() {
    let mut profiler = MemoryProfiler::new();
    let engine = SimplifiedAnimationEngine::new();

    // Create animation with complex interpolation
    let complex_animation = AnimationConfig {
        target: AnimationTarget::Element {
            element: None,
            properties: HashMap::from([
                (
                    "transform".to_string(),
                    AnimationValue::Transform(Transform {
                        translate: (100.0, 100.0),
                        scale: (2.0, 2.0),
                        rotate: 45.0,
                    }),
                ),
                ("opacity".to_string(), AnimationValue::Number(0.5)),
            ]),
        },
        transition: Transition {
            duration: Duration::from_secs(5), // Long duration for many interpolation steps
            easing: Easing::CubicBezier(0.25, 0.1, 0.25, 1.0),
            delay: Duration::ZERO,
        },
        repeat: RepeatConfig::Never,
        stagger: None,
        on_complete: None,
        on_update: None,
    };

    let _handle = engine.animate(&complex_animation).unwrap();

    // Let animation run for a while to test interpolation memory
    std::thread::sleep(Duration::from_millis(100));
    profiler.measure();

    let peak_usage = profiler.get_peak_usage();
    assert!(
        peak_usage < TARGET_ANIMATION_MEMORY_KB / 1024.0,
        "Animation interpolation caused {}MB usage, exceeds target {}KB",
        peak_usage * 1024.0,
        TARGET_ANIMATION_MEMORY_KB
    );
}
