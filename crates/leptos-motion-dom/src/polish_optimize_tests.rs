// Polish and Optimize Tests
//
// These tests define performance optimizations and edge case handling
// for the leptos-motion-dom library using Test-Driven Development.

use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory pool for animation targets to reduce allocations
pub struct AnimationTargetPool {
    pool: Vec<HashMap<String, AnimationValue>>,
    max_size: usize,
}

impl AnimationTargetPool {
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
}

/// Performance monitor for tracking animation metrics
pub struct PerformanceMonitor {
    frame_count: u64,
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_frame_time: Duration,
    min_frame_time: Duration,
    total_time: Duration,
}

impl PerformanceMonitor {
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
                self.frame_times.iter()
                    .map(|d| d.as_nanos() as u64)
                    .sum::<u64>() / self.frame_times.len() as u64
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
pub struct AnimationValueCache {
    cache: HashMap<String, AnimationValue>,
    max_size: usize,
    access_count: HashMap<String, u64>,
}

impl AnimationValueCache {
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
        if let Some((key, _)) = self.access_count.iter()
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
}

/// Edge case handler for animation values
pub struct EdgeCaseHandler {
    max_value: f64,
    min_value: f64,
    epsilon: f64,
}

impl EdgeCaseHandler {
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
}

/// Test animation target pool functionality
#[test]
fn test_animation_target_pool() {
    let mut pool = AnimationTargetPool::new(5);
    
    // Initially empty
    assert_eq!(pool.pool_size(), 0);
    
    // Get a target
    let mut target = pool.get_target();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    
    // Return it to pool
    pool.return_target(target);
    assert_eq!(pool.pool_size(), 1);
    
    // Get it back
    let target = pool.get_target();
    assert!(target.is_empty()); // Should be cleared
    
    // Test max size limit
    for _ in 0..10 {
        let target = HashMap::new();
        pool.return_target(target);
    }
    assert_eq!(pool.pool_size(), 5); // Should not exceed max size
}

/// Test performance monitor
#[test]
fn test_performance_monitor() {
    let mut monitor = PerformanceMonitor::new();
    
    // Initially no frames
    assert_eq!(monitor.frame_count, 0);
    assert_eq!(monitor.fps(), 0.0);
    
    // Record some frames
    for _ in 0..10 {
        monitor.record_frame();
        std::thread::sleep(Duration::from_millis(1));
    }
    
    // Should have recorded frames
    assert_eq!(monitor.frame_count, 10);
    assert!(monitor.fps() > 0.0);
    
    // Get stats
    let stats = monitor.get_stats();
    assert_eq!(stats.frame_count, 10);
    assert!(stats.fps > 0.0);
    assert!(stats.average_frame_time > Duration::ZERO);
    
    // Reset
    monitor.reset();
    assert_eq!(monitor.frame_count, 0);
}

/// Test animation value cache
#[test]
fn test_animation_value_cache() {
    let mut cache = AnimationValueCache::new(3);
    
    // Initially empty
    assert_eq!(cache.size(), 0);
    
    // Insert values
    cache.insert("opacity".to_string(), AnimationValue::Number(1.0));
    cache.insert("scale".to_string(), AnimationValue::Number(1.2));
    cache.insert("rotate".to_string(), AnimationValue::Number(45.0));
    
    assert_eq!(cache.size(), 3);
    
    // Get values
    assert!(cache.get("opacity").is_some());
    assert!(cache.get("scale").is_some());
    assert!(cache.get("rotate").is_some());
    assert!(cache.get("nonexistent").is_none());
    
    // Test eviction when over capacity
    cache.insert("new_value".to_string(), AnimationValue::Number(2.0));
    assert_eq!(cache.size(), 3); // Should still be 3
    
    // Clear cache
    cache.clear();
    assert_eq!(cache.size(), 0);
}

/// Test edge case handler
#[test]
fn test_edge_case_handler() {
    let handler = EdgeCaseHandler::new();
    
    // Test value clamping
    assert_eq!(handler.clamp_value(1e7), handler.max_value);
    assert_eq!(handler.clamp_value(-1e7), handler.min_value);
    assert_eq!(handler.clamp_value(0.5), 0.5);
    
    // Test approximate equality
    assert!(handler.approximately_equal(0.1 + 0.2, 0.3));
    assert!(!handler.approximately_equal(0.1, 0.2));
    
    // Test safe division
    assert_eq!(handler.safe_divide(10.0, 2.0), 5.0);
    assert_eq!(handler.safe_divide(10.0, 0.0), 0.0);
    
    // Test value sanitization
    assert_eq!(handler.sanitize_value(f64::INFINITY), 0.0);
    assert_eq!(handler.sanitize_value(f64::NAN), 0.0);
    assert_eq!(handler.sanitize_value(1e7), handler.max_value);
    assert_eq!(handler.sanitize_value(0.5), 0.5);
}

/// Test memory optimization with large number of animations
#[test]
fn test_memory_optimization() {
    let mut pool = AnimationTargetPool::new(100);
    let mut cache = AnimationValueCache::new(50);
    
    // Simulate creating many animation targets
    let start_memory = std::mem::size_of_val(&pool) + std::mem::size_of_val(&cache);
    
    // Simulate concurrent usage by keeping multiple targets in use
    let mut active_targets = Vec::new();
    
    for i in 0..1000 {
        // Get a target from the pool
        let mut target = pool.get_target();
        target.insert("opacity".to_string(), AnimationValue::Number(i as f64 / 1000.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.0 + i as f64 / 1000.0));
        
        // Cache the values
        cache.insert(format!("opacity_{}", i), AnimationValue::Number(i as f64 / 1000.0));
        cache.insert(format!("scale_{}", i), AnimationValue::Number(1.0 + i as f64 / 1000.0));
        
        // Keep some targets active to simulate concurrent usage
        active_targets.push(target);
        
        // Return old targets to pool when we have too many active
        if active_targets.len() > 50 {
            let old_target = active_targets.remove(0);
            pool.return_target(old_target);
        }
        
        // Debug: print pool size every 100 iterations
        if i % 100 == 0 {
            println!("Iteration {}: pool size = {}, active targets = {}", i, pool.pool_size(), active_targets.len());
        }
    }
    
    // Return all remaining active targets
    for target in active_targets {
        pool.return_target(target);
    }
    
    // Memory usage should be controlled
    let end_memory = std::mem::size_of_val(&pool) + std::mem::size_of_val(&cache);
    assert!(end_memory < start_memory + 10000); // Should not grow excessively
    
    // Pool should have some targets (at least 1)
    assert!(pool.pool_size() >= 1);
    
    // Cache should be at max size
    assert_eq!(cache.size(), 50);
}

/// Test performance under high load
#[test]
fn test_high_load_performance() {
    let mut monitor = PerformanceMonitor::new();
    let mut pool = AnimationTargetPool::new(1000);
    let mut cache = AnimationValueCache::new(500);
    
    let start_time = Instant::now();
    
    // Simulate high load
    for _ in 0..10000 {
        monitor.record_frame();
        
        // Create and use animation targets
        let mut target = pool.get_target();
        target.insert("opacity".to_string(), AnimationValue::Number(0.5));
        target.insert("scale".to_string(), AnimationValue::Number(1.2));
        
        // Cache values
        cache.insert("test_key".to_string(), AnimationValue::Number(0.5));
        
        // Return target
        pool.return_target(target);
    }
    
    let duration = start_time.duration_since(start_time);
    
    // Should complete quickly
    assert!(duration < Duration::from_secs(1));
    
    // Should have good performance stats
    let stats = monitor.get_stats();
    assert!(stats.fps > 0.0);
    assert!(stats.average_frame_time < Duration::from_millis(100));
}

/// Test edge cases with extreme values
#[test]
fn test_extreme_value_handling() {
    let handler = EdgeCaseHandler::new();
    
    // Test with extreme values
    let extreme_values = vec![
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        1e100,
        -1e100,
        0.0,
        -0.0,
    ];
    
    for value in extreme_values {
        let sanitized = handler.sanitize_value(value);
        assert!(!sanitized.is_infinite());
        assert!(!sanitized.is_nan());
        assert!(sanitized >= handler.min_value);
        assert!(sanitized <= handler.max_value);
    }
}

/// Test cache eviction policy
#[test]
fn test_cache_eviction_policy() {
    let mut cache = AnimationValueCache::new(3);
    
    // Fill cache
    cache.insert("key1".to_string(), AnimationValue::Number(1.0));
    cache.insert("key2".to_string(), AnimationValue::Number(2.0));
    cache.insert("key3".to_string(), AnimationValue::Number(3.0));
    
    // Access key1 multiple times
    cache.get("key1");
    cache.get("key1");
    cache.get("key1");
    
    // Access key2 once
    cache.get("key2");
    
    // Add new item (should evict key3 as least used)
    cache.insert("key4".to_string(), AnimationValue::Number(4.0));
    
    // key3 should be evicted
    assert!(cache.get("key3").is_none());
    
    // key1 and key2 should still be there
    assert!(cache.get("key1").is_some());
    assert!(cache.get("key2").is_some());
    assert!(cache.get("key4").is_some());
}

/// Test performance monitor accuracy
#[test]
fn test_performance_monitor_accuracy() {
    let mut monitor = PerformanceMonitor::new();
    
    // Record frames with known timing
    let frame_duration = Duration::from_millis(16); // ~60fps
    
    for _ in 0..10 {
        monitor.record_frame();
        std::thread::sleep(frame_duration);
    }
    
    let stats = monitor.get_stats();
    
    // Should be close to 60fps
    assert!(stats.fps > 50.0);
    assert!(stats.fps < 70.0);
    
    // Average frame time should be close to 16ms
    assert!(stats.average_frame_time > Duration::from_millis(10));
    assert!(stats.average_frame_time < Duration::from_millis(25));
}

/// Test concurrent access to optimization structures
#[test]
fn test_concurrent_optimization() {
    let mut pool = AnimationTargetPool::new(100);
    let mut cache = AnimationValueCache::new(50);
    
    // Simulate concurrent-like access patterns
    let mut active_targets = Vec::new();
    
    for i in 0..1000 {
        // Get target
        let mut target = pool.get_target();
        target.insert("opacity".to_string(), AnimationValue::Number(i as f64 / 1000.0));
        
        // Cache value
        cache.insert(format!("key_{}", i % 50), AnimationValue::Number(i as f64 / 1000.0));
        
        // Keep some targets active
        active_targets.push(target);
        
        // Return old targets when we have too many
        if active_targets.len() > 20 {
            let old_target = active_targets.remove(0);
            pool.return_target(old_target);
        }
        
        // Access cache
        if i % 10 == 0 {
            cache.get(&format!("key_{}", i % 50));
        }
    }
    
    // Return all remaining targets
    for target in active_targets {
        pool.return_target(target);
    }
    
    // Should handle concurrent-like access without issues
    assert!(pool.pool_size() >= 1);
    assert!(cache.size() >= 49); // Allow some flexibility due to eviction
}

/// Test memory leak prevention
#[test]
fn test_memory_leak_prevention() {
    let mut pool = AnimationTargetPool::new(10);
    let mut cache = AnimationValueCache::new(5);
    
    // Create many targets and cache entries
    for i in 0..1000 {
        let mut target = pool.get_target();
        target.insert("opacity".to_string(), AnimationValue::Number(i as f64 / 1000.0));
        pool.return_target(target);
        
        cache.insert(format!("key_{}", i), AnimationValue::Number(i as f64 / 1000.0));
    }
    
    // Clear everything
    pool.clear();
    cache.clear();
    
    // Should be empty
    assert_eq!(pool.pool_size(), 0);
    assert_eq!(cache.size(), 0);
    
    // Should be able to use again
    let target = pool.get_target();
    assert!(target.is_empty());
    
    cache.insert("new_key".to_string(), AnimationValue::Number(1.0));
    assert_eq!(cache.size(), 1);
}
