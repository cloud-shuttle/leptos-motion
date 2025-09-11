//! Animation pooling and memory management for optimized performance

use crate::{Result, StudioError, timeline::AnimationValue, transforms::Transform3D};
use leptos::*;
use leptos::prelude::{ElementChild, NodeRefAttribute, StyleAttribute, OnAttribute, create_signal, create_effect, set_interval, Get, Set};
use leptos::attr::global::ClassAttribute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Memory-pooled animation instance
#[derive(Debug, Clone)]
pub struct PooledAnimation {
    /// Animation ID
    pub id: Uuid,
    /// Animation type
    pub animation_type: AnimationType,
    /// Current state
    pub state: AnimationState,
    /// Properties being animated
    pub properties: HashMap<String, AnimationValue>,
    /// Start time
    pub start_time: Option<Instant>,
    /// Duration
    pub duration: Duration,
    /// Progress (0.0 to 1.0)
    pub progress: f32,
    /// Pool reference (for returning to pool)
    pool_ref: Option<Weak<Mutex<AnimationPoolInner>>>,
    /// Reference count for shared ownership
    pub ref_count: usize,
}

impl PooledAnimation {
    /// Create new pooled animation
    fn new(
        animation_type: AnimationType,
        duration: Duration,
        pool_ref: Weak<Mutex<AnimationPoolInner>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            animation_type,
            state: AnimationState::Ready,
            properties: HashMap::new(),
            start_time: None,
            duration,
            progress: 0.0,
            pool_ref: Some(pool_ref),
            ref_count: 1,
        }
    }

    /// Start animation
    pub fn start(&mut self) -> Result<()> {
        if self.state == AnimationState::Playing {
            return Err(StudioError::PoolExhausted(
                "Animation is already playing".to_string(),
            ));
        }

        self.state = AnimationState::Playing;
        self.start_time = Some(Instant::now());
        self.progress = 0.0;
        Ok(())
    }

    /// Pause animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
        }
    }

    /// Stop animation and return to pool
    pub fn stop(&mut self) {
        self.state = AnimationState::Completed;
        self.start_time = None;
        self.progress = 0.0;
        self.properties.clear();

        // Return to pool if possible
        if let Some(pool_ref) = &self.pool_ref {
            if let Some(pool) = pool_ref.upgrade() {
                if let Ok(mut inner) = pool.lock() {
                    inner.return_animation(self.clone());
                }
            }
        }
    }

    /// Update animation frame
    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        if self.state != AnimationState::Playing {
            return Ok(());
        }

        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            self.progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

            if self.progress >= 1.0 {
                self.state = AnimationState::Completed;
                self.stop();
            }
        }

        Ok(())
    }

    /// Set property value
    pub fn set_property(&mut self, key: String, value: AnimationValue) {
        self.properties.insert(key, value);
    }

    /// Get property value
    pub fn get_property(&self, key: &str) -> Option<&AnimationValue> {
        self.properties.get(key)
    }

    /// Check if animation is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, AnimationState::Playing | AnimationState::Paused)
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Increment reference count
    pub fn add_ref(&mut self) {
        self.ref_count += 1;
    }

    /// Decrement reference count
    pub fn release(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }
}

/// Animation type categories for pooling
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationType {
    Transform,
    Opacity,
    Color,
    Path,
    Custom(String),
}

/// Animation state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationState {
    Ready,
    Playing,
    Paused,
    Completed,
}

/// Internal pool state
#[derive(Debug)]
struct AnimationPoolInner {
    /// Available animations by type
    available: HashMap<AnimationType, VecDeque<PooledAnimation>>,
    /// Currently active animations
    active: HashMap<Uuid, PooledAnimation>,
    /// Pool configuration
    config: PoolConfig,
    /// Memory usage tracking
    memory_stats: MemoryStats,
    /// Performance metrics
    performance_metrics: PerformanceMetrics,
}

impl AnimationPoolInner {
    fn new(config: PoolConfig) -> Self {
        Self {
            available: HashMap::new(),
            active: HashMap::new(),
            config,
            memory_stats: MemoryStats::default(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// Pre-allocate animations of a specific type
    fn preallocate(
        &mut self,
        animation_type: AnimationType,
        count: usize,
        pool_ref: Weak<Mutex<AnimationPoolInner>>,
    ) {
        let queue = self
            .available
            .entry(animation_type.clone())
            .or_insert_with(VecDeque::new);

        for _ in 0..count {
            let animation = PooledAnimation::new(
                animation_type.clone(),
                Duration::from_secs(1),
                pool_ref.clone(),
            );
            queue.push_back(animation);
        }

        self.memory_stats.total_allocated += count;
    }

    /// Allocate animation from pool
    fn allocate(
        &mut self,
        animation_type: AnimationType,
        duration: Duration,
    ) -> Result<PooledAnimation> {
        // Try to reuse existing animation
        if let Some(queue) = self.available.get_mut(&animation_type) {
            if let Some(mut animation) = queue.pop_front() {
                animation.id = Uuid::new_v4();
                animation.duration = duration;
                animation.state = AnimationState::Ready;
                animation.properties.clear();
                animation.start_time = None;
                animation.progress = 0.0;
                animation.ref_count = 1;

                self.active.insert(animation.id, animation.clone());
                self.memory_stats.active_count += 1;
                self.memory_stats.available_count -= 1;
                self.performance_metrics.cache_hits += 1;

                return Ok(animation);
            }
        }

        // Check capacity before creating new
        if self.active.len() >= self.config.max_capacity {
            return Err(StudioError::PoolExhausted(format!(
                "Animation pool capacity ({}) exceeded",
                self.config.max_capacity
            )));
        }

        // Create new animation
        let pool_ref = match &self.config.pool_ref {
            Some(weak_ref) => weak_ref.clone(),
            None => {
                return Err(StudioError::PoolExhausted(
                    "Pool reference not available".to_string(),
                ));
            }
        };

        let animation = PooledAnimation::new(animation_type, duration, pool_ref);
        self.active.insert(animation.id, animation.clone());
        self.memory_stats.active_count += 1;
        self.memory_stats.total_allocated += 1;
        self.performance_metrics.cache_misses += 1;

        Ok(animation)
    }

    /// Return animation to pool
    fn return_animation(&mut self, mut animation: PooledAnimation) {
        // Remove from active set
        if self.active.remove(&animation.id).is_some() {
            self.memory_stats.active_count -= 1;
        }

        // Reset animation state
        animation.state = AnimationState::Ready;
        animation.properties.clear();
        animation.start_time = None;
        animation.progress = 0.0;
        animation.ref_count = 0;

        // Return to available pool if under limit
        let type_key = animation.animation_type.clone();
        let queue = self.available.entry(type_key).or_insert_with(VecDeque::new);

        if queue.len() < self.config.max_per_type {
            queue.push_back(animation);
            self.memory_stats.available_count += 1;
        } else {
            // Pool is full, just drop the animation
            self.memory_stats.total_allocated -= 1;
        }
    }

    /// Cleanup expired animations
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        let mut expired = Vec::new();

        for (id, animation) in &self.active {
            if let Some(start_time) = animation.start_time {
                if now.duration_since(start_time) > self.config.max_lifetime {
                    expired.push(*id);
                }
            }
        }

        for id in expired {
            if let Some(mut animation) = self.active.remove(&id) {
                animation.stop();
                self.memory_stats.expired_count += 1;
            }
        }
    }

    /// Force garbage collection
    fn garbage_collect(&mut self) {
        let target_size = (self.config.max_capacity as f32 * 0.8) as usize;

        // Remove excess available animations
        for queue in self.available.values_mut() {
            let excess = queue.len().saturating_sub(self.config.max_per_type / 2);
            for _ in 0..excess {
                if queue.pop_front().is_some() {
                    self.memory_stats.available_count -= 1;
                    self.memory_stats.total_allocated -= 1;
                }
            }
        }

        self.performance_metrics.gc_cycles += 1;
    }
}

/// Pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum total capacity
    pub max_capacity: usize,
    /// Maximum animations per type
    pub max_per_type: usize,
    /// Maximum animation lifetime
    pub max_lifetime: Duration,
    /// Cleanup interval
    pub cleanup_interval: Duration,
    /// Enable automatic garbage collection
    pub auto_gc: bool,
    /// GC threshold (as percentage of max_capacity)
    pub gc_threshold: f32,
    /// Pool reference (internal)
    pool_ref: Option<Weak<Mutex<AnimationPoolInner>>>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_capacity: 1000,
            max_per_type: 100,
            max_lifetime: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(5),
            auto_gc: true,
            gc_threshold: 0.9,
            pool_ref: None,
        }
    }
}

/// Memory usage statistics
#[derive(Debug, Default, Clone)]
pub struct MemoryStats {
    /// Total animations allocated
    pub total_allocated: usize,
    /// Currently active animations
    pub active_count: usize,
    /// Available (pooled) animations
    pub available_count: usize,
    /// Expired animations cleaned up
    pub expired_count: usize,
}

impl MemoryStats {
    /// Get memory utilization percentage
    pub fn utilization(&self, max_capacity: usize) -> f32 {
        if max_capacity == 0 {
            0.0
        } else {
            (self.active_count as f32 / max_capacity as f32) * 100.0
        }
    }

    /// Get cache efficiency
    pub fn cache_efficiency(&self) -> f32 {
        if self.total_allocated == 0 {
            0.0
        } else {
            (self.available_count as f32 / self.total_allocated as f32) * 100.0
        }
    }
}

/// Performance metrics
#[derive(Debug, Default, Clone)]
pub struct PerformanceMetrics {
    /// Cache hits (reused animations)
    pub cache_hits: usize,
    /// Cache misses (new allocations)
    pub cache_misses: usize,
    /// Garbage collection cycles
    pub gc_cycles: usize,
    /// Average allocation time (microseconds)
    pub avg_allocation_time: f32,
}

impl PerformanceMetrics {
    /// Get cache hit ratio
    pub fn hit_ratio(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            (self.cache_hits as f32 / total as f32) * 100.0
        }
    }
}

/// Main animation pool
#[derive(Debug)]
pub struct AnimationPool {
    /// Internal pool state
    inner: Arc<Mutex<AnimationPoolInner>>,
    /// Last cleanup time
    last_cleanup: Instant,
}

impl AnimationPool {
    /// Create new animation pool
    pub fn new(capacity: usize) -> Self {
        let mut config = PoolConfig::default();
        config.max_capacity = capacity;

        let inner = Arc::new(Mutex::new(AnimationPoolInner::new(config)));

        // Set pool reference in config
        if let Ok(mut pool_inner) = inner.lock() {
            pool_inner.config.pool_ref = Some(Arc::downgrade(&inner));
        }

        Self {
            inner,
            last_cleanup: Instant::now(),
        }
    }

    /// Create pool with custom configuration
    pub fn with_config(mut config: PoolConfig) -> Self {
        let inner = Arc::new(Mutex::new(AnimationPoolInner::new(config.clone())));

        // Set pool reference in config
        if let Ok(mut pool_inner) = inner.lock() {
            pool_inner.config.pool_ref = Some(Arc::downgrade(&inner));
        }

        Self {
            inner,
            last_cleanup: Instant::now(),
        }
    }

    /// Pre-warm pool with animations
    pub fn prewarm(&self, prealloc_config: &HashMap<AnimationType, usize>) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| StudioError::PoolExhausted("Failed to lock pool".to_string()))?;

        for (animation_type, count) in prealloc_config {
            inner.preallocate(animation_type.clone(), *count, Arc::downgrade(&self.inner));
        }

        Ok(())
    }

    /// Allocate animation from pool
    pub fn allocate(&self) -> Result<PooledAnimation> {
        self.allocate_with_type(AnimationType::Transform, Duration::from_secs(1))
    }

    /// Allocate specific animation type
    pub fn allocate_with_type(
        &self,
        animation_type: AnimationType,
        duration: Duration,
    ) -> Result<PooledAnimation> {
        let start_time = Instant::now();

        let mut inner = self
            .inner
            .lock()
            .map_err(|_| StudioError::PoolExhausted("Failed to lock pool".to_string()))?;

        // Periodic cleanup
        if self.last_cleanup.elapsed() > inner.config.cleanup_interval {
            inner.cleanup_expired();
        }

        // Auto garbage collection
        if inner.config.auto_gc {
            let utilization = inner.memory_stats.utilization(inner.config.max_capacity);
            if utilization > inner.config.gc_threshold * 100.0 {
                inner.garbage_collect();
            }
        }

        let animation = inner.allocate(animation_type, duration)?;

        // Update performance metrics
        let allocation_time = start_time.elapsed().as_micros() as f32;
        inner.performance_metrics.avg_allocation_time =
            (inner.performance_metrics.avg_allocation_time * 0.9) + (allocation_time * 0.1);

        Ok(animation)
    }

    /// Return animation to pool
    pub fn deallocate(&self, animation: PooledAnimation) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| StudioError::PoolExhausted("Failed to lock pool".to_string()))?;

        inner.return_animation(animation);
        Ok(())
    }

    /// Get pool capacity
    pub fn capacity(&self) -> usize {
        self.inner
            .lock()
            .map(|inner| inner.config.max_capacity)
            .unwrap_or(0)
    }

    /// Get active animation count
    pub fn active_count(&self) -> usize {
        self.inner
            .lock()
            .map(|inner| inner.memory_stats.active_count)
            .unwrap_or(0)
    }

    /// Get available animation count
    pub fn available_count(&self) -> usize {
        self.inner
            .lock()
            .map(|inner| inner.memory_stats.available_count)
            .unwrap_or(0)
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> Option<MemoryStats> {
        self.inner
            .lock()
            .map(|inner| inner.memory_stats.clone())
            .ok()
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> Option<PerformanceMetrics> {
        self.inner
            .lock()
            .map(|inner| inner.performance_metrics.clone())
            .ok()
    }

    /// Force cleanup of expired animations
    pub fn cleanup(&mut self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.cleanup_expired();
            self.last_cleanup = Instant::now();
        }
    }

    /// Force garbage collection
    pub fn garbage_collect(&self) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| StudioError::PoolExhausted("Failed to lock pool".to_string()))?;

        inner.garbage_collect();
        Ok(())
    }

    /// Shrink pool to fit current usage
    pub fn shrink_to_fit(&self) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| StudioError::PoolExhausted("Failed to lock pool".to_string()))?;

        // Remove all excess available animations
        let mut total_removed = 0;
        for queue in inner.available.values_mut() {
            let target_size = (queue.len() / 2).max(1);
            while queue.len() > target_size {
                if queue.pop_front().is_some() {
                    total_removed += 1;
                }
            }
        }
        
        // Update stats after the loop
        inner.memory_stats.available_count -= total_removed;
        inner.memory_stats.total_allocated -= total_removed;

        Ok(())
    }
}

impl Clone for AnimationPool {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            last_cleanup: self.last_cleanup,
        }
    }
}

/// Memory manager for coordinating multiple pools
#[derive(Debug)]
pub struct MemoryManager {
    /// Animation pools by type
    pools: HashMap<String, AnimationPool>,
    /// Global memory limit
    memory_limit: usize,
    /// Memory pressure threshold
    pressure_threshold: f32,
}

impl MemoryManager {
    /// Create new memory manager
    pub fn new(memory_limit: usize) -> Self {
        Self {
            pools: HashMap::new(),
            memory_limit,
            pressure_threshold: 0.85,
        }
    }

    /// Register animation pool
    pub fn register_pool(&mut self, name: String, pool: AnimationPool) {
        self.pools.insert(name, pool);
    }

    /// Get pool by name
    pub fn get_pool(&self, name: &str) -> Option<&AnimationPool> {
        self.pools.get(name)
    }

    /// Check memory pressure
    pub fn check_memory_pressure(&self) -> f32 {
        let total_active: usize = self.pools.values().map(|pool| pool.active_count()).sum();

        (total_active as f32 / self.memory_limit as f32) * 100.0
    }

    /// Handle memory pressure
    pub fn handle_memory_pressure(&mut self) -> Result<()> {
        let pressure = self.check_memory_pressure();

        if pressure > self.pressure_threshold * 100.0 {
            // Force garbage collection on all pools
            for pool in self.pools.values() {
                pool.garbage_collect()?;
            }
        }

        Ok(())
    }

    /// Get global memory statistics
    pub fn global_stats(&self) -> MemoryStats {
        let mut stats = MemoryStats::default();

        for pool in self.pools.values() {
            if let Some(pool_stats) = pool.memory_stats() {
                stats.total_allocated += pool_stats.total_allocated;
                stats.active_count += pool_stats.active_count;
                stats.available_count += pool_stats.available_count;
                stats.expired_count += pool_stats.expired_count;
            }
        }

        stats
    }
}

/// Animation Pool Monitor Component
#[component]
pub fn PoolMonitor(
    /// Pool to monitor
    pool: AnimationPool,

    /// Update interval in milliseconds
    #[prop(default = 1000)]
    update_interval: u32,
) -> impl IntoView {
    let (stats, set_stats) = create_signal(pool.memory_stats().unwrap_or_default());
    let (metrics, set_metrics) = create_signal(pool.performance_metrics().unwrap_or_default());

    // Update stats periodically
    let pool_clone = pool.clone();
    create_effect(move |_| {
        let interval = update_interval;
        let pool = pool_clone.clone();

        set_interval(
            move || {
                if let Some(new_stats) = pool.memory_stats() {
                    set_stats.set(new_stats);
                }
                if let Some(new_metrics) = pool.performance_metrics() {
                    set_metrics.set(new_metrics);
                }
            },
            std::time::Duration::from_millis(interval as u64),
        );
    });

    view! {
        <div class="pool-monitor">
            <h4>"Animation Pool Status"</h4>

            <div class="pool-stats">
                <div class="stat-item">
                    <label>"Active:"</label>
                    <span>{move || stats.get().active_count}</span>
                </div>

                <div class="stat-item">
                    <label>"Available:"</label>
                    <span>{move || stats.get().available_count}</span>
                </div>

                <div class="stat-item">
                    <label>"Total Allocated:"</label>
                    <span>{move || stats.get().total_allocated}</span>
                </div>

                <div class="stat-item">
                    <label>"Utilization:"</label>
                    <span>{move || format!("{:.1}%", stats.get().utilization(pool.capacity()))}</span>
                </div>

                <div class="stat-item">
                    <label>"Cache Hit Ratio:"</label>
                    <span>{move || format!("{:.1}%", metrics.get().hit_ratio())}</span>
                </div>
            </div>

            <div class="pool-actions">
                <button
                    class="pool-action-btn"
                    on:click=move |_| {
                        // Temporarily disabled until pool is properly accessible
                        // let _ = pool.garbage_collect();
                    }
                >
                    "Force GC"
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_animation_pool_creation() {
        let pool = AnimationPool::new(10);
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn test_pool_allocation() {
        let pool = AnimationPool::new(10);

        let animation = pool.allocate().unwrap();
        assert_eq!(pool.active_count(), 1);
        assert_eq!(animation.ref_count, 1);
        assert!(!animation.is_active());
    }

    #[test]
    fn test_pool_deallocation() {
        let pool = AnimationPool::new(10);

        let animation = pool.allocate().unwrap();
        assert_eq!(pool.active_count(), 1);

        pool.deallocate(animation).unwrap();
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.available_count(), 1);
    }

    #[test]
    fn test_pool_reuse() {
        let pool = AnimationPool::new(10);

        // Allocate and deallocate
        let animation1 = pool.allocate().unwrap();
        let id1 = animation1.id;
        pool.deallocate(animation1).unwrap();

        // Allocate again - should reuse
        let animation2 = pool.allocate().unwrap();
        assert_ne!(animation2.id, id1); // IDs are regenerated
        assert_eq!(pool.available_count(), 0);

        // Verify cache hit metrics
        let metrics = pool.performance_metrics().unwrap();
        assert_eq!(metrics.cache_hits, 1);
        assert_eq!(metrics.cache_misses, 1);
    }

    #[test]
    fn test_pool_capacity_limit() {
        let pool = AnimationPool::new(2);

        let _anim1 = pool.allocate().unwrap();
        let _anim2 = pool.allocate().unwrap();

        // Third allocation should fail
        let result = pool.allocate();
        assert!(result.is_err());
    }

    #[test]
    fn test_animation_lifecycle() {
        let pool = AnimationPool::new(10);
        let mut animation = pool.allocate().unwrap();

        // Initial state
        assert!(!animation.is_active());
        assert!(!animation.is_completed());

        // Start animation
        animation.start().unwrap();
        assert!(animation.is_active());
        assert!(!animation.is_completed());

        // Pause animation
        animation.pause();
        assert!(animation.is_active());

        // Stop animation
        animation.stop();
        assert!(!animation.is_active());
        assert!(animation.is_completed());
    }

    #[test]
    fn test_memory_stats() {
        let pool = AnimationPool::new(10);

        let _anim1 = pool.allocate().unwrap();
        let _anim2 = pool.allocate().unwrap();

        let stats = pool.memory_stats().unwrap();
        assert_eq!(stats.active_count, 2);
        assert_eq!(stats.total_allocated, 2);
        assert!(stats.utilization(10) > 0.0);
    }

    #[test]
    fn test_pool_prewarm() {
        let pool = AnimationPool::new(100);

        let mut prealloc = HashMap::new();
        prealloc.insert(AnimationType::Transform, 10);
        prealloc.insert(AnimationType::Opacity, 5);

        pool.prewarm(&prealloc).unwrap();

        let stats = pool.memory_stats().unwrap();
        assert_eq!(stats.available_count, 15);
        assert_eq!(stats.total_allocated, 15);
    }

    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new(100);

        let pool1 = AnimationPool::new(50);
        let pool2 = AnimationPool::new(50);

        manager.register_pool("transforms".to_string(), pool1);
        manager.register_pool("colors".to_string(), pool2);

        assert!(manager.get_pool("transforms").is_some());
        assert!(manager.get_pool("colors").is_some());
        assert!(manager.get_pool("nonexistent").is_none());
    }

    #[test]
    fn test_animation_properties() {
        let pool = AnimationPool::new(10);
        let mut animation = pool.allocate().unwrap();

        animation.set_property("opacity".to_string(), AnimationValue::Number(0.5));
        animation.set_property("color".to_string(), AnimationValue::Color([255, 0, 0, 255]));

        assert!(animation.get_property("opacity").is_some());
        assert!(animation.get_property("color").is_some());
        assert!(animation.get_property("unknown").is_none());

        if let Some(AnimationValue::Number(opacity)) = animation.get_property("opacity") {
            assert!((opacity - 0.5).abs() < f32::EPSILON);
        } else {
            panic!("Expected Number value for opacity");
        }
    }

    #[test]
    fn test_performance_metrics() {
        let pool = AnimationPool::new(10);

        // Generate some allocations
        let _anim1 = pool.allocate().unwrap(); // Cache miss
        let anim2 = pool.allocate().unwrap(); // Cache miss
        pool.deallocate(anim2).unwrap();
        let _anim3 = pool.allocate().unwrap(); // Cache hit

        let metrics = pool.performance_metrics().unwrap();
        assert_eq!(metrics.cache_hits, 1);
        assert_eq!(metrics.cache_misses, 2);
        assert!(metrics.hit_ratio() > 0.0);
    }
}
