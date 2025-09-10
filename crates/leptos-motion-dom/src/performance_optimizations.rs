//! Performance Optimizations
//!
//! Advanced performance optimization systems for memory management and edge case handling

use leptos::prelude::*;
use leptos::reactive::signal::signal;
use leptos_motion_core::AnimationValue;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory pool for animation targets to reduce allocations
#[derive(Debug, Clone)]
pub struct AnimationTargetPool {
    pool: Vec<HashMap<String, AnimationValue>>,
    max_size: usize,
}

impl AnimationTargetPool {
    /// Create a new animation target pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Get a target from the pool or create a new one
    pub fn get_target(&mut self) -> HashMap<String, AnimationValue> {
        self.pool.pop().unwrap_or_else(HashMap::new)
    }

    /// Return a target to the pool for reuse
    pub fn return_target(&mut self, mut target: HashMap<String, AnimationValue>) {
        if self.pool.len() < self.max_size {
            target.clear();
            self.pool.push(target);
        }
    }

    /// Get current pool size
    pub fn pool_size(&self) -> usize {
        self.pool.len()
    }

    /// Clear the pool
    pub fn clear(&mut self) {
        self.pool.clear();
    }

    /// Get pool capacity
    pub fn capacity(&self) -> usize {
        self.max_size
    }
}

/// Performance monitor for tracking animation metrics
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    frame_count: u64,
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_frame_time: Duration,
    min_frame_time: Duration,
    total_time: Duration,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_frame_time: Instant::now(),
            frame_times: Vec::new(),
            max_frame_time: Duration::ZERO,
            min_frame_time: Duration::from_secs(1),
            total_time: Duration::ZERO,
        }
    }

    /// Record a frame
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);

        self.frame_count += 1;
        self.frame_times.push(frame_time);
        self.max_frame_time = self.max_frame_time.max(frame_time);
        self.min_frame_time = self.min_frame_time.min(frame_time);
        self.total_time += frame_time;
        self.last_frame_time = now;

        // Keep only last 100 frame times for rolling average
        if self.frame_times.len() > 100 {
            self.frame_times.remove(0);
        }
    }

    /// Get average frame time
    pub fn average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            Duration::ZERO
        } else {
            Duration::from_nanos(
                self.frame_times
                    .iter()
                    .map(|d| d.as_nanos() as u64)
                    .sum::<u64>()
                    / self.frame_times.len() as u64,
            )
        }
    }

    /// Get FPS
    pub fn fps(&self) -> f64 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time.as_nanos() > 0 {
            1_000_000_000.0 / avg_frame_time.as_nanos() as f64
        } else {
            0.0
        }
    }

    /// Get performance stats
    pub fn get_stats(&self) -> PerformanceStats {
        PerformanceStats {
            frame_count: self.frame_count,
            average_frame_time: self.average_frame_time(),
            max_frame_time: self.max_frame_time,
            min_frame_time: self.min_frame_time,
            fps: self.fps(),
            total_time: self.total_time,
        }
    }

    /// Reset the monitor
    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.last_frame_time = Instant::now();
        self.frame_times.clear();
        self.max_frame_time = Duration::ZERO;
        self.min_frame_time = Duration::from_secs(1);
        self.total_time = Duration::ZERO;
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub frame_count: u64,
    pub average_frame_time: Duration,
    pub max_frame_time: Duration,
    pub min_frame_time: Duration,
    pub fps: f64,
    pub total_time: Duration,
}

/// Optimized animation value cache
#[derive(Debug, Clone)]
pub struct AnimationValueCache {
    cache: HashMap<String, AnimationValue>,
    max_size: usize,
    access_count: HashMap<String, u64>,
}

impl AnimationValueCache {
    /// Create a new animation value cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            access_count: HashMap::new(),
        }
    }

    /// Get a value from cache
    pub fn get(&mut self, key: &str) -> Option<&AnimationValue> {
        if let Some(value) = self.cache.get(key) {
            *self.access_count.entry(key.to_string()).or_insert(0) += 1;
            Some(value)
        } else {
            None
        }
    }

    /// Insert a value into cache
    pub fn insert(&mut self, key: String, value: AnimationValue) {
        if self.cache.len() >= self.max_size {
            self.evict_least_used();
        }
        self.cache.insert(key.clone(), value);
        self.access_count.insert(key, 1);
    }

    /// Evict least used item
    fn evict_least_used(&mut self) {
        if let Some((key, _)) = self
            .access_count
            .iter()
            .min_by_key(|(_, count)| *count)
            .map(|(k, _)| (k.clone(), *self.access_count.get(k).unwrap()))
        {
            self.cache.remove(&key);
            self.access_count.remove(&key);
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_count.clear();
    }

    /// Get cache capacity
    pub fn capacity(&self) -> usize {
        self.max_size
    }
}

/// Edge case handler for animation values
#[derive(Debug, Clone)]
pub struct EdgeCaseHandler {
    max_value: f64,
    min_value: f64,
    epsilon: f64,
}

impl EdgeCaseHandler {
    /// Create a new edge case handler
    pub fn new() -> Self {
        Self {
            max_value: 1e6,
            min_value: -1e6,
            epsilon: 1e-10,
        }
    }

    /// Clamp a value to safe range
    pub fn clamp_value(&self, value: f64) -> f64 {
        value.clamp(self.min_value, self.max_value)
    }

    /// Check if two values are approximately equal
    pub fn approximately_equal(&self, a: f64, b: f64) -> bool {
        (a - b).abs() < self.epsilon
    }

    /// Handle division by zero
    pub fn safe_divide(&self, numerator: f64, denominator: f64) -> f64 {
        if self.approximately_equal(denominator, 0.0) {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Handle infinity and NaN
    pub fn sanitize_value(&self, value: f64) -> f64 {
        if value.is_infinite() || value.is_nan() {
            0.0
        } else {
            self.clamp_value(value)
        }
    }

    /// Set epsilon for approximate equality
    pub fn set_epsilon(&mut self, epsilon: f64) {
        self.epsilon = epsilon;
    }

    /// Set value range
    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = min;
        self.max_value = max;
    }
}

impl Default for EdgeCaseHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance optimization manager
#[derive(Debug, Clone)]
pub struct PerformanceManager {
    target_pool: AnimationTargetPool,
    value_cache: AnimationValueCache,
    monitor: PerformanceMonitor,
    edge_handler: EdgeCaseHandler,
}

impl PerformanceManager {
    /// Create a new performance manager
    pub fn new() -> Self {
        Self {
            target_pool: AnimationTargetPool::new(100),
            value_cache: AnimationValueCache::new(50),
            monitor: PerformanceMonitor::new(),
            edge_handler: EdgeCaseHandler::new(),
        }
    }

    /// Get animation target pool
    pub fn target_pool(&mut self) -> &mut AnimationTargetPool {
        &mut self.target_pool
    }

    /// Get animation value cache
    pub fn value_cache(&mut self) -> &mut AnimationValueCache {
        &mut self.value_cache
    }

    /// Get performance monitor
    pub fn monitor(&mut self) -> &mut PerformanceMonitor {
        &mut self.monitor
    }

    /// Get edge case handler
    pub fn edge_handler(&self) -> &EdgeCaseHandler {
        &self.edge_handler
    }

    /// Record a frame
    pub fn record_frame(&mut self) {
        self.monitor.record_frame();
    }

    /// Get performance stats
    pub fn get_stats(&self) -> PerformanceStats {
        self.monitor.get_stats()
    }

    /// Reset all performance tracking
    pub fn reset(&mut self) {
        self.monitor.reset();
        self.target_pool.clear();
        self.value_cache.clear();
    }

    /// Optimize animation value
    pub fn optimize_value(&self, value: f64) -> f64 {
        self.edge_handler.sanitize_value(value)
    }

    /// Check if values are approximately equal
    pub fn values_approximately_equal(&self, a: f64, b: f64) -> bool {
        self.edge_handler.approximately_equal(a, b)
    }
}

impl Default for PerformanceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook for using performance optimizations in Leptos components
pub fn use_performance_optimizations()
-> (ReadSignal<PerformanceStats>, WriteSignal<PerformanceStats>) {
    let (stats, set_stats) = signal(PerformanceStats {
        frame_count: 0,
        average_frame_time: Duration::ZERO,
        max_frame_time: Duration::ZERO,
        min_frame_time: Duration::ZERO,
        fps: 0.0,
        total_time: Duration::ZERO,
    });

    (stats, set_stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_target_pool() {
        let mut pool = AnimationTargetPool::new(5);

        assert_eq!(pool.pool_size(), 0);

        let mut target = pool.get_target();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));

        pool.return_target(target);
        assert_eq!(pool.pool_size(), 1);

        let target = pool.get_target();
        assert!(target.is_empty());
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();

        assert_eq!(monitor.frame_count, 0);

        monitor.record_frame();
        assert_eq!(monitor.frame_count, 1);

        let stats = monitor.get_stats();
        assert!(stats.fps > 0.0);
    }

    #[test]
    fn test_animation_value_cache() {
        let mut cache = AnimationValueCache::new(3);

        assert_eq!(cache.size(), 0);

        cache.insert("opacity".to_string(), AnimationValue::Number(1.0));
        assert_eq!(cache.size(), 1);

        assert!(cache.get("opacity").is_some());
        assert!(cache.get("nonexistent").is_none());
    }

    #[test]
    fn test_edge_case_handler() {
        let handler = EdgeCaseHandler::new();

        assert_eq!(handler.clamp_value(1e7), handler.max_value);
        assert_eq!(handler.clamp_value(-1e7), handler.min_value);
        assert_eq!(handler.clamp_value(0.5), 0.5);

        assert!(handler.approximately_equal(0.1 + 0.2, 0.3));
        assert!(!handler.approximately_equal(0.1, 0.2));

        assert_eq!(handler.safe_divide(10.0, 2.0), 5.0);
        assert_eq!(handler.safe_divide(10.0, 0.0), 0.0);

        assert_eq!(handler.sanitize_value(f64::INFINITY), 0.0);
        assert_eq!(handler.sanitize_value(f64::NAN), 0.0);
    }

    #[test]
    fn test_performance_manager() {
        let mut manager = PerformanceManager::new();

        manager.record_frame();
        let stats = manager.get_stats();
        assert_eq!(stats.frame_count, 1);

        let optimized_value = manager.optimize_value(1e7);
        assert_eq!(optimized_value, manager.edge_handler().max_value);

        assert!(manager.values_approximately_equal(0.1 + 0.2, 0.3));
    }
}
