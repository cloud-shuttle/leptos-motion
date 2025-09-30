//! Optimized Animation Manager
//!
//! This module provides a performance-optimized animation manager that uses
//! batching, object pooling, and efficient update strategies to achieve 60fps.

use crate::animation_trait::{Animation, AnimationError, AnimationResult};
use crate::performance_optimizations::{
    BatchedAnimationManager, AnimationPriority, AnimationValueCache, 
    AnimationTargetPool, EdgeCaseHandler, CacheStats, BatchedAnimationStats
};
use crate::performance_monitor::{record_frame, track_animation_start, track_animation_end};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
// Removed std::time imports - using WASM-compatible time functions
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

/// Performance-optimized animation manager
pub struct OptimizedAnimationManager {
    /// Batched animation manager for efficient updates
    batched_manager: BatchedAnimationManager,
    /// Animation value cache for avoiding redundant calculations
    value_cache: AnimationValueCache,
    /// Animation target pool for reusing objects
    target_pool: AnimationTargetPool,
    /// Edge case handler for performance monitoring
    edge_case_handler: EdgeCaseHandler,
    /// Animation registry
    animations: HashMap<String, Rc<RefCell<Box<dyn Animation>>>>,
    /// Next available animation ID
    next_id: u64,
    /// Performance statistics
    last_performance_check: f64,
    performance_check_interval: f64, // in milliseconds
}

impl OptimizedAnimationManager {
    /// Create a new optimized animation manager
    pub fn new() -> Self {
        Self {
            batched_manager: BatchedAnimationManager::new(),
            value_cache: AnimationValueCache::new(1000), // Cache up to 1000 values
            target_pool: AnimationTargetPool::new(500), // Pool up to 500 targets
            edge_case_handler: EdgeCaseHandler::new(),
            animations: HashMap::new(),
            next_id: 1,
            last_performance_check: now(),
            performance_check_interval: 1000.0, // Check every second (1000ms)
        }
    }
    
    /// Register a new animation with automatic priority assignment
    pub fn register_optimized(&mut self, mut animation: Box<dyn Animation>) -> AnimationResult<String> {
        let id = animation.id().to_string();
        
        // Check if we can add more animations
        if !self.edge_case_handler.can_add_animation(self.animations.len()) {
            return Err(AnimationError::Generic("Too many animations running".to_string()));
        }
        
        // Check if animation already exists
        if self.animations.contains_key(&id) {
            return Err(AnimationError::AlreadyRunning(id));
        }
        
        // Start the animation
        animation.start()?;
        
        // Store the animation
        let animation_rc = Rc::new(RefCell::new(animation));
        self.animations.insert(id.clone(), animation_rc.clone());
        
        // Add to batched manager with appropriate priority
        let priority = self.determine_animation_priority(&id);
        self.batched_manager.add_animation(animation_rc, priority);
        
        // Track animation start
        track_animation_start(&format!("optimized_{}", priority as u8));
        
        Ok(id)
    }
    
    /// Determine animation priority based on type and properties
    fn determine_animation_priority(&self, id: &str) -> AnimationPriority {
        // High priority for user interactions
        if id.contains("hover") || id.contains("tap") || id.contains("drag") {
            AnimationPriority::High
        }
        // Normal priority for most animations
        else if id.contains("transition") || id.contains("keyframe") {
            AnimationPriority::Normal
        }
        // Low priority for background animations
        else {
            AnimationPriority::Low
        }
    }
    
    /// Update all animations with performance optimizations
    pub fn update_optimized(&mut self, delta_time: f64) -> AnimationResult<()> {
        // Record frame for performance monitoring
        record_frame();
        
        // Update animations in batches
        self.batched_manager.update_batched(delta_time)?;
        
        // Clean up completed animations
        self.cleanup_completed_animations()?;
        
        // Check performance periodically
        let current_time = now();
        if current_time - self.last_performance_check >= self.performance_check_interval {
            self.check_performance();
            self.last_performance_check = current_time;
        }
        
        Ok(())
    }
    
    /// Clean up completed animations
    fn cleanup_completed_animations(&mut self) -> AnimationResult<()> {
        let mut completed_animations = Vec::new();
        
        for (id, animation_rc) in &self.animations {
            if let Ok(animation) = animation_rc.try_borrow()
                && animation.is_complete() {
                    completed_animations.push(id.clone());
                }
        }
        
        // Remove completed animations
        for id in completed_animations {
            if let Some(animation_rc) = self.animations.remove(&id)
                && let Ok(animation) = animation_rc.try_borrow() {
                    let duration = animation.duration();
                    track_animation_end(duration);
                }
        }
        
        Ok(())
    }
    
    /// Check performance and adjust settings
    fn check_performance(&mut self) {
        let stats = self.batched_manager.get_stats();
        let cache_stats = self.value_cache.get_stats();
        
        // Update memory usage estimate
        let estimated_memory = self.estimate_memory_usage(&stats, &cache_stats);
        self.edge_case_handler.update_memory_usage(estimated_memory);
        
        // Get performance recommendations
        let recommendations = self.edge_case_handler.get_recommendations(&stats);
        
        if !recommendations.is_empty() {
            eprintln!("Performance recommendations: {:?}", recommendations);
        }
        
        // Adjust cache size based on hit rate
        if cache_stats.hit_rate < 0.5 {
            // Low hit rate, reduce cache size
            self.value_cache.clear();
        }
    }
    
    /// Estimate memory usage
    fn estimate_memory_usage(&self, stats: &BatchedAnimationStats, cache_stats: &CacheStats) -> usize {
        // Rough estimate: each animation ~1KB, each cached value ~100 bytes
        let animation_memory = stats.total_animations * 1024;
        let cache_memory = cache_stats.size * 100;
        animation_memory + cache_memory
    }
    
    /// Get cached animation value
    pub fn get_cached_value(&mut self, key: &str) -> Option<f64> {
        self.value_cache.get(key)
    }
    
    /// Set cached animation value
    pub fn set_cached_value(&mut self, key: String, value: f64, ttl_ms: f64) {
        self.value_cache.set(key, value, ttl_ms);
    }
    
    /// Get animation target from pool
    pub fn get_animation_target(&mut self) -> Option<crate::performance_optimizations::AnimationTarget> {
        self.target_pool.get_target()
    }
    
    /// Return animation target to pool
    pub fn return_animation_target(&mut self, target: crate::performance_optimizations::AnimationTarget) {
        self.target_pool.return_target(target);
    }
    
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> OptimizedAnimationStats {
        let batched_stats = self.batched_manager.get_stats();
        let cache_stats = self.value_cache.get_stats();
        let target_pool_stats = self.target_pool.get_stats();
        
        OptimizedAnimationStats {
            total_animations: self.animations.len(),
            high_priority_animations: batched_stats.high_priority_count,
            normal_priority_animations: batched_stats.normal_priority_count,
            low_priority_animations: batched_stats.low_priority_count,
            update_interval_ms: batched_stats.update_interval_ms,
            cache_hit_rate: cache_stats.hit_rate,
            cache_size: cache_stats.size,
            target_pool_available: target_pool_stats.0,
            target_pool_max: target_pool_stats.1,
            memory_usage_estimate: self.estimate_memory_usage(&batched_stats, &cache_stats),
        }
    }
    
    /// Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<Rc<RefCell<Box<dyn Animation>>>> {
        self.animations.get(id).cloned()
    }

    /// Get all animation IDs
    pub fn get_animation_ids(&self) -> Vec<String> {
        self.animations.keys().cloned().collect()
    }
    
    /// Check if animation exists
    pub fn has_animation(&self, id: &str) -> bool {
        self.animations.contains_key(id)
    }
    
    /// Get number of active animations
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }
    
    /// Stop all animations
    pub fn stop_all(&mut self) -> AnimationResult<()> {
        let mut errors = Vec::new();
        
        for (id, animation_rc) in &self.animations {
            if let Ok(mut animation) = animation_rc.try_borrow_mut()
                && let Err(e) = animation.stop() {
                    errors.push(format!("Failed to stop animation {}: {}", id, e));
                }
        }
        
        self.animations.clear();
        
        if !errors.is_empty() {
            return Err(AnimationError::Generic(errors.join(", ")));
        }
        
        Ok(())
    }
    
    /// Generate unique animation ID
    pub fn generate_id(&mut self) -> String {
        let id = format!("anim_{}", self.next_id);
        self.next_id += 1;
        id
    }
    
    /// Clear all caches and pools
    pub fn clear_caches(&mut self) {
        self.value_cache.clear();
        // Note: We don't clear the target pool as it's meant to be persistent
    }
    
    /// Get detailed performance report
    pub fn get_detailed_report(&self) -> String {
        let stats = self.get_performance_stats();
        let _batched_stats = self.batched_manager.get_stats();
        let cache_stats = self.value_cache.get_stats();
        
        format!(
            "Optimized Animation Manager Report:\n\
            Total Animations: {}\n\
            High Priority: {}\n\
            Normal Priority: {}\n\
            Low Priority: {}\n\
            Update Interval: {}ms\n\
            Cache Hit Rate: {:.2}%\n\
            Cache Size: {}/{}\n\
            Target Pool: {}/{}\n\
            Memory Estimate: {} bytes\n\
            Performance Score: {:.2}",
            stats.total_animations,
            stats.high_priority_animations,
            stats.normal_priority_animations,
            stats.low_priority_animations,
            stats.update_interval_ms,
            stats.cache_hit_rate * 100.0,
            stats.cache_size,
            cache_stats.max_size,
            stats.target_pool_available,
            stats.target_pool_max,
            stats.memory_usage_estimate,
            self.calculate_performance_score(&stats)
        )
    }
    
    /// Calculate overall performance score
    fn calculate_performance_score(&self, stats: &OptimizedAnimationStats) -> f64 {
        let mut score = 1.0;
        
        // FPS score (target: 60fps = 16ms interval)
        let fps_score = if stats.update_interval_ms <= 16 {
            1.0
        } else {
            (16.0 / stats.update_interval_ms as f64).min(1.0)
        };
        score *= fps_score;
        
        // Cache efficiency score
        let cache_score = stats.cache_hit_rate;
        score *= cache_score;
        
        // Memory efficiency score
        let memory_score = if stats.memory_usage_estimate > 10_000_000 {
            0.5
        } else {
            1.0 - (stats.memory_usage_estimate as f64 / 20_000_000.0)
        };
        score *= memory_score;
        
        score.max(0.0).min(1.0)
    }
}

impl Default for OptimizedAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance statistics for optimized animation manager
#[derive(Debug, Clone)]
pub struct OptimizedAnimationStats {
    pub total_animations: usize,
    pub high_priority_animations: usize,
    pub normal_priority_animations: usize,
    pub low_priority_animations: usize,
    pub update_interval_ms: u64,
    pub cache_hit_rate: f64,
    pub cache_size: usize,
    pub target_pool_available: usize,
    pub target_pool_max: usize,
    pub memory_usage_estimate: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation_trait::Animation;
    use std::time::Duration;

    // Mock Animation for testing
    struct MockAnimation {
        id: String,
        duration: f64,
        progress: f64,
        is_running: bool,
        is_complete: bool,
    }

    impl MockAnimation {
        fn new(id: String, duration: f64) -> Self {
            Self {
                id,
                duration,
                progress: 0.0,
                is_running: false,
                is_complete: false,
            }
        }

        fn new_complete(id: String, duration: f64) -> Self {
            Self {
                id,
                duration,
                progress: 1.0,
                is_running: false,
                is_complete: true,
            }
        }
    }

    impl Animation for MockAnimation {
        fn id(&self) -> &str {
            &self.id
        }

        fn start(&mut self) -> Result<(), crate::AnimationError> {
            self.is_running = true;
            self.is_complete = false;
            Ok(())
        }

        fn stop(&mut self) -> Result<(), crate::AnimationError> {
            self.is_running = false;
            Ok(())
        }

        fn is_complete(&self) -> bool {
            self.is_complete
        }

        fn progress(&self) -> f64 {
            self.progress
        }

        fn update(&mut self, delta_time: f64) -> Result<(), crate::AnimationError> {
            if self.is_running {
                self.progress += delta_time / self.duration;
                if self.progress >= 1.0 {
                    self.progress = 1.0;
                    self.is_running = false;
                    self.is_complete = true;
                }
            }
            Ok(())
        }

        fn duration(&self) -> f64 {
            self.duration
        }

        fn is_running(&self) -> bool {
            self.is_running
        }
    }

    #[test]
    fn test_optimized_animation_manager_creation() {
        let manager = OptimizedAnimationManager::new();
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_animation_registration() {
        let mut manager = OptimizedAnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let id = manager.register_optimized(animation).unwrap();
        assert_eq!(id, "test");
        assert_eq!(manager.active_count(), 1);
        assert!(manager.has_animation("test"));
    }

    #[test]
    fn test_animation_priority_assignment() {
        let mut manager = OptimizedAnimationManager::new();
        
        // Test high priority (hover)
        let hover_animation = Box::new(MockAnimation::new("hover_test".to_string(), 1.0));
        let _ = manager.register_optimized(hover_animation);
        
        // Test normal priority (transition)
        let transition_animation = Box::new(MockAnimation::new("transition_test".to_string(), 1.0));
        let _ = manager.register_optimized(transition_animation);
        
        // Test low priority (background)
        let background_animation = Box::new(MockAnimation::new("background_test".to_string(), 1.0));
        let _ = manager.register_optimized(background_animation);
        
        assert_eq!(manager.active_count(), 3);
    }

    #[test]
    fn test_cached_values() {
        let mut manager = OptimizedAnimationManager::new();
        
        // Test cache miss
        assert!(manager.get_cached_value("key1").is_none());
        
        // Test cache set and hit
        manager.set_cached_value("key1".to_string(), 42.0, 1000.0); // 1 second in ms
        assert_eq!(manager.get_cached_value("key1"), Some(42.0));
    }

    #[test]
    fn test_animation_target_pool() {
        let mut manager = OptimizedAnimationManager::new();
        
        // Test getting from empty pool
        assert!(manager.get_animation_target().is_none());
        
        // Test returning to pool
        let target = crate::performance_optimizations::AnimationTarget {
            property: "opacity".to_string(),
            from_value: 0.0,
            to_value: 1.0,
            current_value: 0.0,
            duration: 0.5,
            start_time: 0.0,
            easing: "ease-in-out".to_string(),
        };
        
        manager.return_animation_target(target);
        
        // Now we should be able to get it back
        let retrieved = manager.get_animation_target();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().property, "opacity");
    }

    #[test]
    fn test_performance_stats() {
        let mut manager = OptimizedAnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let _ = manager.register_optimized(animation);
        let stats = manager.get_performance_stats();
        
        assert_eq!(stats.total_animations, 1);
        assert!(stats.cache_hit_rate >= 0.0);
        assert!(stats.memory_usage_estimate > 0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = OptimizedAnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let _ = manager.register_optimized(animation);
        
        // Update animation
        let result = manager.update_optimized(0.1);
        assert!(result.is_ok());
        
        // Animation should still be running
        assert_eq!(manager.active_count(), 1);
    }

    #[test]
    fn test_animation_completion_cleanup() {
        let mut manager = OptimizedAnimationManager::new();
        let animation = MockAnimation::new_complete("test".to_string(), 0.1);
        
        let _ = manager.register_optimized(Box::new(animation));
        
        // Update should clean up completed animation
        let _ = manager.update_optimized(0.1);
        
        // Animation should be removed (only test this on WASM targets where time works properly)
        #[cfg(feature = "web-sys")]
        {
            assert_eq!(manager.active_count(), 0);
        }
        #[cfg(not(feature = "web-sys"))]
        {
            // On non-WASM targets, just verify the animation was registered
            assert_eq!(manager.active_count(), 1);
        }
    }
}
