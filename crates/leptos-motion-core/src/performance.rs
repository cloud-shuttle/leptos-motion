//! Performance monitoring and optimization utilities

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use web_sys::{window, Performance};
use crate::{AnimationHandle, Result};
use crate::engine::{AnimationEngine, AnimationConfig};

/// Performance budget configuration
#[derive(Debug, Clone)]
pub struct PerformanceBudget {
    /// Maximum frame time in milliseconds (target: 16.67ms for 60fps)
    pub max_frame_time: f64,
    /// Maximum concurrent animations
    pub max_concurrent_animations: usize,
    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,
    /// Maximum animation duration in milliseconds
    pub max_animation_duration: f64,
}

impl Default for PerformanceBudget {
    fn default() -> Self {
        Self {
            max_frame_time: 16.67, // 60fps target
            max_concurrent_animations: 100,
            max_memory_usage: 10 * 1024 * 1024, // 10MB
            max_animation_duration: 5000.0, // 5 seconds
        }
    }
}

/// Performance metrics for a single frame
#[derive(Debug, Clone)]
pub struct FrameMetrics {
    /// Frame timestamp
    pub timestamp: f64,
    /// Frame duration in milliseconds
    pub duration: f64,
    /// Number of animations updated
    pub animations_updated: usize,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Whether frame was dropped
    pub dropped: bool,
}

/// Performance monitor for tracking animation performance
pub struct PerformanceMonitor {
    performance: Performance,
    frame_metrics: VecDeque<FrameMetrics>,
    max_samples: usize,
    budget: PerformanceBudget,
    frame_start: Option<f64>,
    total_frames: u64,
    dropped_frames: u64,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(budget: PerformanceBudget) -> Option<Self> {
        let performance = window()?.performance()?;
        
        Some(Self {
            performance,
            frame_metrics: VecDeque::with_capacity(60), // Store 1 second at 60fps
            max_samples: 60,
            budget,
            frame_start: None,
            total_frames: 0,
            dropped_frames: 0,
        })
    }
    
    /// Start measuring a frame
    pub fn start_frame(&mut self) {
        self.frame_start = Some(self.performance.now());
    }
    
    /// End measuring a frame and record metrics
    pub fn end_frame(&mut self, animations_updated: usize, memory_usage: usize) -> FrameMetrics {
        let end_time = self.performance.now();
        let frame_start = self.frame_start.unwrap_or(end_time);
        let duration = end_time - frame_start;
        
        let dropped = duration > self.budget.max_frame_time;
        if dropped {
            self.dropped_frames += 1;
        }
        self.total_frames += 1;
        
        let metrics = FrameMetrics {
            timestamp: end_time,
            duration,
            animations_updated,
            memory_usage,
            dropped,
        };
        
        // Add to history
        self.frame_metrics.push_back(metrics.clone());
        if self.frame_metrics.len() > self.max_samples {
            self.frame_metrics.pop_front();
        }
        
        metrics
    }
    
    /// Get current FPS
    pub fn get_fps(&self) -> f64 {
        if self.frame_metrics.len() < 2 {
            return 0.0;
        }
        
        let total_time = self.frame_metrics.back().unwrap().timestamp - 
                        self.frame_metrics.front().unwrap().timestamp;
        let frame_count = self.frame_metrics.len() - 1;
        
        if total_time > 0.0 {
            (frame_count as f64 * 1000.0) / total_time
        } else {
            0.0
        }
    }
    
    /// Get average frame time
    pub fn get_avg_frame_time(&self) -> f64 {
        if self.frame_metrics.is_empty() {
            return 0.0;
        }
        
        let total_duration: f64 = self.frame_metrics.iter().map(|m| m.duration).sum();
        total_duration / self.frame_metrics.len() as f64
    }
    
    /// Get frame drop rate
    pub fn get_frame_drop_rate(&self) -> f64 {
        if self.total_frames == 0 {
            return 0.0;
        }
        
        self.dropped_frames as f64 / self.total_frames as f64
    }
    
    /// Check if performance is within budget
    pub fn is_within_budget(&self) -> bool {
        self.get_avg_frame_time() <= self.budget.max_frame_time &&
        self.get_frame_drop_rate() < 0.1 // Less than 10% frame drops
    }
    
    /// Get performance report
    pub fn get_report(&self) -> PerformanceReport {
        PerformanceReport {
            fps: self.get_fps(),
            avg_frame_time: self.get_avg_frame_time(),
            frame_drop_rate: self.get_frame_drop_rate(),
            total_frames: self.total_frames,
            dropped_frames: self.dropped_frames,
            within_budget: self.is_within_budget(),
        }
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub fps: f64,
    pub avg_frame_time: f64,
    pub frame_drop_rate: f64,
    pub total_frames: u64,
    pub dropped_frames: u64,
    pub within_budget: bool,
}

/// Animation scheduler for batching and optimizing animations
pub struct AnimationScheduler {
    pending_animations: Vec<PendingAnimation>,
    active_animations: Vec<ActiveAnimation>,
    frame_budget: Duration,
    batch_size: usize,
    max_concurrent: usize,
}

/// Pending animation waiting to be started
#[derive(Debug)]
struct PendingAnimation {
    priority: AnimationPriority,
    config: AnimationConfig,
    created_at: Instant,
}

/// Active animation being processed
#[derive(Debug)]
struct ActiveAnimation {
    handle: AnimationHandle,
    config: AnimationConfig,
    start_time: Instant,
    priority: AnimationPriority,
}

/// Animation priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnimationPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Performance statistics for the animation scheduler
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub pending_count: usize,
    pub active_count: usize,
    pub batch_size: usize,
    pub max_concurrent: usize,
    pub frame_budget_ms: u64,
}

/// Pool statistics for memory optimization
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub available: usize,
    pub active: usize,
    pub max_pool_size: usize,
    pub total_allocations: usize,
    pub total_reuses: usize,
    pub reuse_rate: f64,
}

impl AnimationScheduler {
    /// Create a new animation scheduler
    pub fn new(frame_budget: Duration) -> Self {
        Self {
            pending_animations: Vec::new(),
            active_animations: Vec::new(),
            frame_budget,
            batch_size: 10, // Process up to 10 animations per frame
            max_concurrent: 100, // Allow up to 100 concurrent animations
        }
    }
    
    /// Set batch size for processing animations
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
    
    /// Set maximum concurrent animations
    pub fn with_max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.max_concurrent = max_concurrent;
        self
    }
    
    /// Schedule an animation
    pub fn schedule(&mut self, config: AnimationConfig, priority: AnimationPriority) -> AnimationHandle {
        let handle = AnimationHandle(0); // Simplified for now
        
        self.pending_animations.push(PendingAnimation {
            priority,
            config,
            created_at: Instant::now(),
        });
        
        handle
    }
    
    /// Process pending animations within frame budget
    pub fn process_pending(&mut self, engine: &mut dyn AnimationEngine) -> Result<()> {
        let frame_start = Instant::now();
        let mut processed = 0;
        
        // Check if we can process more animations
        if self.active_animations.len() >= self.max_concurrent {
            return Ok(()); // Skip processing if at capacity
        }
        
        // Sort by priority (highest first)
        self.pending_animations.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // Process animations in batches
        let mut to_process = std::cmp::min(
            self.batch_size,
            self.pending_animations.len()
        );
        
        while to_process > 0 && !self.pending_animations.is_empty() {
            // Check if we're still within frame budget
            if frame_start.elapsed() > self.frame_budget {
                break;
            }
            
            if let Some(pending) = self.pending_animations.pop() {
                // Start the animation
                let handle = engine.animate(&pending.config)?;
                
                self.active_animations.push(ActiveAnimation {
                    handle,
                    config: pending.config,
                    start_time: Instant::now(),
                    priority: pending.priority,
                });
                
                processed += 1;
                to_process -= 1;
            }
        }
        
        Ok(())
    }
    
    /// Clean up completed animations
    pub fn cleanup_completed(&mut self, engine: &dyn AnimationEngine) {
        self.active_animations.retain(|animation| {
            engine.is_running(animation.handle)
        });
    }
    
    /// Get current performance statistics
    pub fn get_stats(&self) -> SchedulerStats {
        SchedulerStats {
            pending_count: self.pending_animations.len(),
            active_count: self.active_animations.len(),
            batch_size: self.batch_size,
            max_concurrent: self.max_concurrent,
            frame_budget_ms: self.frame_budget.as_millis() as u64,
        }
    }
    
    /// Get memory usage estimate
    pub fn estimate_memory_usage(&self) -> usize {
        let pending_memory = self.pending_animations.len() * std::mem::size_of::<PendingAnimation>();
        let active_memory = self.active_animations.len() * std::mem::size_of::<ActiveAnimation>();
        let struct_memory = std::mem::size_of::<Self>();
        
        pending_memory + active_memory + struct_memory
    }
}

/// GPU layer manager for optimizing rendering
pub struct GPULayerManager {
    promoted_elements: std::collections::HashSet<String>,
    max_layers: usize,
    layer_count: usize,
}

impl GPULayerManager {
    /// Create a new GPU layer manager
    pub fn new(max_layers: usize) -> Self {
        Self {
            promoted_elements: std::collections::HashSet::new(),
            max_layers,
            layer_count: 0,
        }
    }
    
    /// Promote an element to GPU layer
    pub fn promote_to_gpu(&mut self, element_id: &str) -> bool {
        if self.layer_count >= self.max_layers {
            return false;
        }
        
        if self.promoted_elements.insert(element_id.to_string()) {
            self.layer_count += 1;
            true
        } else {
            false
        }
    }
    
    /// Demote an element from GPU layer
    pub fn demote_from_gpu(&mut self, element_id: &str) -> bool {
        if self.promoted_elements.remove(element_id) {
            self.layer_count -= 1;
            true
        } else {
            false
        }
    }
    
    /// Check if element is promoted
    pub fn is_promoted(&self, element_id: &str) -> bool {
        self.promoted_elements.contains(element_id)
    }
    
    /// Get current layer count
    pub fn layer_count(&self) -> usize {
        self.layer_count
    }
}

/// Memory pool for reusing animation objects to reduce allocations
pub struct AnimationPool<T> {
    available: Vec<T>,
    active: std::collections::HashMap<AnimationHandle, T>,
    max_pool_size: usize,
    total_allocations: usize,
    total_reuses: usize,
}

impl<T> AnimationPool<T> {
    /// Create a new animation pool
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            active: std::collections::HashMap::new(),
            max_pool_size: 1000, // Default max pool size
            total_allocations: 0,
            total_reuses: 0,
        }
    }
    
    /// Create a new animation pool with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            available: Vec::new(),
            active: std::collections::HashMap::new(),
            max_pool_size: max_size,
            total_allocations: 0,
            total_reuses: 0,
        }
    }
    
    /// Get an animation from the pool
    pub fn acquire(&mut self, handle: AnimationHandle, create: impl FnOnce() -> T) -> &mut T {
        if let Some(animation) = self.available.pop() {
            self.total_reuses += 1;
            self.active.insert(handle, animation);
        } else {
            self.total_allocations += 1;
            let animation = create();
            self.active.insert(handle, animation);
        }
        self.active.get_mut(&handle).unwrap()
    }
    
    /// Return an animation to the pool
    pub fn release(&mut self, handle: AnimationHandle) -> Option<T> {
        if let Some(animation) = self.active.remove(&handle) {
            if self.available.len() < self.max_pool_size {
                self.available.push(animation);
                None
            } else {
                Some(animation)
            }
        } else {
            None
        }
    }
    
    /// Get the number of active animations
    pub fn active_count(&self) -> usize {
        self.active.len()
    }
    
    /// Get the number of available animations
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            available: self.available.len(),
            active: self.active.len(),
            max_pool_size: self.max_pool_size,
            total_allocations: self.total_allocations,
            total_reuses: self.total_reuses,
            reuse_rate: if self.total_allocations > 0 {
                self.total_reuses as f64 / (self.total_allocations + self.total_reuses) as f64
            } else {
                0.0
            },
        }
    }
    
    /// Optimize pool memory usage
    pub fn optimize(&mut self) {
        // Shrink available pool if it's too large
        if self.available.len() > self.max_pool_size / 2 {
            self.available.truncate(self.max_pool_size / 2);
        }
        
        // Clear active animations that are no longer needed
        self.active.clear();
    }
    
    /// Get memory usage estimate
    pub fn estimate_memory_usage(&self) -> usize {
        let available_memory = self.available.len() * std::mem::size_of::<T>();
        let active_memory = self.active.len() * std::mem::size_of::<T>();
        let struct_memory = std::mem::size_of::<Self>();
        
        available_memory + active_memory + struct_memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor() {
        let budget = PerformanceBudget::default();
        let mut monitor = PerformanceMonitor::new(budget).unwrap();
        
        monitor.start_frame();
        std::thread::sleep(Duration::from_millis(16));
        let metrics = monitor.end_frame(10, 1024);
        
        assert!(metrics.duration > 0.0);
        assert_eq!(metrics.animations_updated, 10);
        assert_eq!(metrics.memory_usage, 1024);
    }
    
    #[test]
    fn test_animation_scheduler() {
        let frame_budget = Duration::from_millis(16);
        let mut scheduler = AnimationScheduler::new(frame_budget);
        
        // Test scheduling
        let config = AnimationConfig {
            element: web_sys::Element::new("div").unwrap(),
            from: AnimationTarget::new(),
            to: AnimationTarget::new(),
            transition: Transition::default(),
            on_complete: None,
            on_update: None,
        };
        
        let handle = scheduler.schedule(config, AnimationPriority::Normal);
        assert!(handle.0 > 0);
    }
    
    #[test]
    fn test_gpu_layer_manager() {
        let mut manager = GPULayerManager::new(10);
        
        assert!(manager.promote_to_gpu("element1"));
        assert!(manager.is_promoted("element1"));
        assert_eq!(manager.layer_count(), 1);
        
        assert!(manager.demote_from_gpu("element1"));
        assert!(!manager.is_promoted("element1"));
        assert_eq!(manager.layer_count(), 0);
    }
    
    #[test]
    fn test_animation_pool() {
        let mut pool = AnimationPool::<String>::new();
        let handle = AnimationHandle(1);
        
        let animation = pool.acquire(handle, || "new_animation".to_string());
        assert_eq!(animation, "new_animation");
        assert_eq!(pool.active_count(), 1);
        
        pool.release(handle);
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.available_count(), 1);
    }
}
