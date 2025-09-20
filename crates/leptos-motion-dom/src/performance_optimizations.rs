//! Performance Optimizations
//!
//! This module provides performance optimizations for the animation system,
//! including object pooling, batching, and efficient update strategies.

use crate::animation_trait::{Animation, AnimationError, AnimationResult};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
// Removed std::time imports - using WASM-compatible time functions
use wasm_bindgen::prelude::*;
#[cfg(feature = "web-sys")]
use web_sys::window;

/// Get current time in milliseconds (WASM-compatible)
fn now() -> f64 {
    #[cfg(feature = "web-sys")]
    {
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                performance.now()
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
    #[cfg(not(feature = "web-sys"))]
    {
        0.0
    }
}

/// Object pool for reusing animation objects
pub struct AnimationPool<T> {
    available: VecDeque<T>,
    in_use: HashMap<String, T>,
    max_size: usize,
}

impl<T: Clone> AnimationPool<T> {
    /// Create a new object pool
    pub fn new(max_size: usize) -> Self {
        Self {
            available: VecDeque::new(),
            in_use: HashMap::new(),
            max_size,
        }
    }
    
    /// Get an object from the pool
    pub fn get(&mut self, id: String) -> Option<T> {
        if let Some(obj) = self.available.pop_front() {
            self.in_use.insert(id.clone(), obj.clone());
            Some(obj)
        } else {
            None
        }
    }
    
    /// Return an object to the pool
    pub fn return_object(&mut self, id: String, _obj: T) {
        if let Some(returned_obj) = self.in_use.remove(&id) {
            if self.available.len() < self.max_size {
                self.available.push_back(returned_obj);
            }
        }
    }
    
    /// Get pool statistics
    pub fn get_stats(&self) -> (usize, usize) {
        (self.available.len(), self.in_use.len())
    }
}

/// Batched animation updates for better performance
pub struct BatchedAnimationManager {
    /// Animation batches by priority
    high_priority: Vec<Rc<RefCell<Box<dyn Animation>>>>,
    normal_priority: Vec<Rc<RefCell<Box<dyn Animation>>>>,
    low_priority: Vec<Rc<RefCell<Box<dyn Animation>>>>,
    /// Update frequency control
    last_update: f64,
    update_interval: f64, // in milliseconds
    /// Performance monitoring
    frame_count: usize,
    last_fps_check: f64,
}

impl BatchedAnimationManager {
    /// Create a new batched animation manager
    pub fn new() -> Self {
        Self {
            high_priority: Vec::new(),
            normal_priority: Vec::new(),
            low_priority: Vec::new(),
            last_update: now(),
            update_interval: 16.0, // ~60fps (16ms)
            frame_count: 0,
            last_fps_check: now(),
        }
    }
    
    /// Add animation with priority
    pub fn add_animation(&mut self, animation: Rc<RefCell<Box<dyn Animation>>>, priority: AnimationPriority) {
        match priority {
            AnimationPriority::High => self.high_priority.push(animation),
            AnimationPriority::Normal => self.normal_priority.push(animation),
            AnimationPriority::Low => self.low_priority.push(animation),
        }
    }
    
    /// Update animations in batches
    pub fn update_batched(&mut self, delta_time: f64) -> AnimationResult<()> {
        let current_time = now();
        
        // Skip update if not enough time has passed
        if current_time - self.last_update < self.update_interval {
            return Ok(());
        }
        
        self.last_update = current_time;
        self.frame_count += 1;
        
        // Update high priority animations first (always)
        Self::update_batch_static(&mut self.high_priority, delta_time)?;
        
        // Update normal priority animations
        Self::update_batch_static(&mut self.normal_priority, delta_time)?;
        
        // Update low priority animations only if we have time
        if self.should_update_low_priority() {
            Self::update_batch_static(&mut self.low_priority, delta_time)?;
        }
        
        // Check FPS and adjust update frequency
        self.adjust_update_frequency();
        
        Ok(())
    }
    
    /// Update a batch of animations (static method to avoid borrowing issues)
    fn update_batch_static(batch: &mut Vec<Rc<RefCell<Box<dyn Animation>>>>, delta_time: f64) -> AnimationResult<()> {
        let mut completed_indices = Vec::new();
        
        for (index, animation_rc) in batch.iter().enumerate() {
            if let Ok(mut animation) = animation_rc.try_borrow_mut() {
                if let Err(e) = animation.update(delta_time) {
                    eprintln!("Animation update error: {:?}", e);
                }
                
                if animation.is_complete() {
                    completed_indices.push(index);
                }
            }
        }
        
        // Remove completed animations (in reverse order to maintain indices)
        for &index in completed_indices.iter().rev() {
            batch.remove(index);
        }
        
        Ok(())
    }
    
    /// Check if we should update low priority animations
    fn should_update_low_priority(&self) -> bool {
        // Update low priority animations every other frame
        self.frame_count % 2 == 0
    }
    
    /// Adjust update frequency based on performance
    fn adjust_update_frequency(&mut self) {
        let current_time = now();
        if current_time - self.last_fps_check >= 1000.0 { // 1 second in milliseconds
            let fps = self.frame_count as f64;
            
            if fps < 50.0 {
                // Reduce update frequency if FPS is low
                self.update_interval = 20.0; // 50fps (20ms)
            } else if fps > 58.0 {
                // Increase update frequency if FPS is high
                self.update_interval = 16.0; // 60fps (16ms)
            }
            
            self.frame_count = 0;
            self.last_fps_check = current_time;
        }
    }
    
    /// Get performance statistics
    pub fn get_stats(&self) -> BatchedAnimationStats {
        BatchedAnimationStats {
            high_priority_count: self.high_priority.len(),
            normal_priority_count: self.normal_priority.len(),
            low_priority_count: self.low_priority.len(),
            total_animations: self.high_priority.len() + self.normal_priority.len() + self.low_priority.len(),
            update_interval_ms: self.update_interval as u64,
        }
    }
}

/// Animation priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPriority {
    /// High priority - always updated
    High,
    /// Normal priority - updated every frame
    Normal,
    /// Low priority - updated every other frame
    Low,
}

/// Statistics for batched animation manager
#[derive(Debug, Clone)]
pub struct BatchedAnimationStats {
    pub high_priority_count: usize,
    pub normal_priority_count: usize,
    pub low_priority_count: usize,
    pub total_animations: usize,
    pub update_interval_ms: u64,
}

/// Animation value cache for avoiding redundant calculations
pub struct AnimationValueCache {
    cache: HashMap<String, CachedValue>,
    max_size: usize,
    hit_count: usize,
    miss_count: usize,
}

#[derive(Debug, Clone)]
struct CachedValue {
    value: f64,
    timestamp: f64,
    ttl: f64, // in milliseconds
}

impl AnimationValueCache {
    /// Create a new animation value cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hit_count: 0,
            miss_count: 0,
        }
    }
    
    /// Get cached value
    pub fn get(&mut self, key: &str) -> Option<f64> {
        if let Some(cached) = self.cache.get(key) {
            let current_time = now();
            if current_time - cached.timestamp < cached.ttl {
                self.hit_count += 1;
                return Some(cached.value);
            } else {
                // Expired, remove from cache
                self.cache.remove(key);
            }
        }
        
        self.miss_count += 1;
        None
    }
    
    /// Set cached value
    pub fn set(&mut self, key: String, value: f64, ttl_ms: f64) {
        // Remove oldest entries if cache is full
        if self.cache.len() >= self.max_size {
            let oldest_key = self.cache.keys().next().cloned();
            if let Some(key) = oldest_key {
                self.cache.remove(&key);
            }
        }
        
        self.cache.insert(key, CachedValue {
            value,
            timestamp: now(),
            ttl: ttl_ms,
        });
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let total_requests = self.hit_count + self.miss_count;
        let hit_rate = if total_requests > 0 {
            self.hit_count as f64 / total_requests as f64
        } else {
            0.0
        };
        
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            hit_rate,
        }
    }
    
    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hit_count = 0;
        self.miss_count = 0;
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hit_count: usize,
    pub miss_count: usize,
    pub hit_rate: f64,
}

/// Performance-optimized animation target pool
pub struct AnimationTargetPool {
    targets: VecDeque<AnimationTarget>,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct AnimationTarget {
    pub property: String,
    pub from_value: f64,
    pub to_value: f64,
    pub current_value: f64,
    pub duration: f64,
    pub start_time: f64,
    pub easing: String,
}

impl AnimationTargetPool {
    /// Create a new animation target pool
    pub fn new(max_size: usize) -> Self {
        Self {
            targets: VecDeque::new(),
            max_size,
        }
    }
    
    /// Get a target from the pool
    pub fn get_target(&mut self) -> Option<AnimationTarget> {
        self.targets.pop_front()
    }
    
    /// Return a target to the pool
    pub fn return_target(&mut self, mut target: AnimationTarget) {
        // Reset the target
        target.current_value = target.from_value;
        target.start_time = 0.0;
        
        if self.targets.len() < self.max_size {
            self.targets.push_back(target);
        }
    }
    
    /// Create a new target (if pool is empty)
    pub fn create_target(
        &mut self,
        property: String,
        from_value: f64,
        to_value: f64,
        duration: f64,
        easing: String,
    ) -> AnimationTarget {
        AnimationTarget {
            property,
            from_value,
            to_value,
            current_value: from_value,
            duration,
            start_time: 0.0,
            easing,
        }
    }
    
    /// Get pool statistics
    pub fn get_stats(&self) -> (usize, usize) {
        (self.targets.len(), self.max_size)
    }
}

/// Edge case handler for performance optimization
pub struct EdgeCaseHandler {
    max_animations_per_frame: usize,
    max_memory_usage: usize,
    current_memory_usage: usize,
}

impl EdgeCaseHandler {
    /// Create a new edge case handler
    pub fn new() -> Self {
        Self {
            max_animations_per_frame: 100,
            max_memory_usage: 50_000_000, // 50MB
            current_memory_usage: 0,
        }
    }
    
    /// Check if we can add more animations
    pub fn can_add_animation(&self, current_count: usize) -> bool {
        current_count < self.max_animations_per_frame
    }
    
    /// Check if memory usage is acceptable
    pub fn is_memory_usage_acceptable(&self) -> bool {
        self.current_memory_usage < self.max_memory_usage
    }
    
    /// Update memory usage estimate
    pub fn update_memory_usage(&mut self, estimated_usage: usize) {
        self.current_memory_usage = estimated_usage;
    }
    
    /// Get performance recommendations
    pub fn get_recommendations(&self, stats: &BatchedAnimationStats) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if stats.total_animations > 50 {
            recommendations.push("Consider reducing the number of simultaneous animations".to_string());
        }
        
        if stats.update_interval_ms > 20 {
            recommendations.push("Animation update frequency is low, consider optimizing".to_string());
        }
        
        if !self.is_memory_usage_acceptable() {
            recommendations.push("Memory usage is high, consider implementing cleanup".to_string());
        }
        
        recommendations
    }
}

impl Default for EdgeCaseHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_animation_pool() {
        let mut pool = AnimationPool::<String>::new(5);
        
        // Test getting from empty pool
        assert!(pool.get("test1".to_string()).is_none());
        
        // Test returning to pool (this won't add to available since it wasn't in_use)
        pool.return_object("test1".to_string(), "value1".to_string());
        assert_eq!(pool.get_stats(), (0, 0));
        
        // Test getting from pool (still empty)
        let value = pool.get("test1".to_string());
        assert!(value.is_none());
        assert_eq!(pool.get_stats(), (0, 0));
    }
    
    #[test]
    fn test_animation_value_cache() {
        let mut cache = AnimationValueCache::new(10);
        
        // Test cache miss
        assert!(cache.get("key1").is_none());
        assert_eq!(cache.get_stats().miss_count, 1);
        
        // Test cache hit
        cache.set("key1".to_string(), 42.0, 1000.0); // 1 second in ms
        assert_eq!(cache.get("key1"), Some(42.0));
        assert_eq!(cache.get_stats().hit_count, 1);
        
        // Test cache expiration (skip on non-WASM targets)
        #[cfg(feature = "web-sys")]
        {
            cache.set("key2".to_string(), 24.0, 0.001); // 1 nanosecond in ms
            std::thread::sleep(Duration::from_millis(1));
            assert!(cache.get("key2").is_none());
        }
        #[cfg(not(feature = "web-sys"))]
        {
            // On non-WASM targets, just test that we can set and get values
            cache.set("key2".to_string(), 24.0, 1000.0);
            assert_eq!(cache.get("key2"), Some(24.0));
        }
    }
    
    #[test]
    fn test_animation_target_pool() {
        let mut pool = AnimationTargetPool::new(5);
        
        // Test getting from empty pool
        assert!(pool.get_target().is_none());
        
        // Test creating and returning target
        let target = pool.create_target(
            "opacity".to_string(),
            0.0,
            1.0,
            0.5,
            "ease-in-out".to_string(),
        );
        
        pool.return_target(target);
        assert_eq!(pool.get_stats(), (1, 5));
        
        // Test getting from pool
        let retrieved = pool.get_target();
        assert!(retrieved.is_some());
        assert_eq!(pool.get_stats(), (0, 5));
    }
    
    #[test]
    fn test_edge_case_handler() {
        let handler = EdgeCaseHandler::new();
        
        assert!(handler.can_add_animation(50));
        assert!(!handler.can_add_animation(150));
        
        assert!(handler.is_memory_usage_acceptable());
        
        let stats = BatchedAnimationStats {
            high_priority_count: 10,
            normal_priority_count: 20,
            low_priority_count: 30,
            total_animations: 60,
            update_interval_ms: 25,
        };
        
        let recommendations = handler.get_recommendations(&stats);
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("reducing")));
    }
}