//! Performance monitoring and optimization for animations
//!
//! This module provides performance tracking, GPU layer management,
//! animation pooling, and performance budgeting to ensure smooth
//! 60fps animations.

use std::collections::HashMap;
use std::time::{Duration, Instant};
#[cfg(feature = "web-sys")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web-sys")]
use web_sys::{Element, HtmlElement};

/// Performance report containing metrics about animation performance
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Average frame time in milliseconds
    pub average_frame_time: f64,
    /// Frames per second
    pub fps: f64,
    /// Number of active animations
    pub active_animations: usize,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// GPU layer count
    pub gpu_layers: usize,
    /// Performance budget utilization (0.0 to 1.0)
    pub budget_utilization: f64,
    /// Timestamp when report was generated
    pub timestamp: Instant,
}

impl Default for PerformanceReport {
    fn default() -> Self {
        Self {
            average_frame_time: 16.67, // 60fps target
            fps: 60.0,
            active_animations: 0,
            memory_usage: 0,
            gpu_layers: 0,
            budget_utilization: 0.0,
            timestamp: Instant::now(),
        }
    }
}

/// Performance budget for managing animation resources
#[derive(Debug, Clone)]
pub struct PerformanceBudget {
    /// Maximum frame time in milliseconds
    pub max_frame_time: f64,
    /// Maximum number of simultaneous animations
    pub max_animations: usize,
    /// Maximum memory usage in bytes
    pub max_memory: usize,
    /// Maximum GPU layers
    pub max_gpu_layers: usize,
    /// Target FPS
    pub target_fps: f64,
}

impl Default for PerformanceBudget {
    fn default() -> Self {
        Self {
            max_frame_time: 16.67, // 60fps
            max_animations: 100,
            max_memory: 10 * 1024 * 1024, // 10MB
            max_gpu_layers: 50,
            target_fps: 60.0,
        }
    }
}

impl PerformanceBudget {
    /// Check if current performance is within budget
    pub fn is_within_budget(&self, report: &PerformanceReport) -> bool {
        report.average_frame_time <= self.max_frame_time
            && report.active_animations <= self.max_animations
            && report.memory_usage <= self.max_memory
            && report.gpu_layers <= self.max_gpu_layers
    }

    /// Calculate budget utilization (0.0 to 1.0)
    pub fn calculate_utilization(&self, report: &PerformanceReport) -> f64 {
        let frame_util = (report.average_frame_time / self.max_frame_time).min(1.0);
        let animation_util =
            (report.active_animations as f64 / self.max_animations as f64).min(1.0);
        let memory_util = (report.memory_usage as f64 / self.max_memory as f64).min(1.0);
        let gpu_util = (report.gpu_layers as f64 / self.max_gpu_layers as f64).min(1.0);

        (frame_util + animation_util + memory_util + gpu_util) / 4.0
    }
}

/// Performance monitor for tracking animation performance
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Performance budget
    budget: PerformanceBudget,
    /// Frame time samples
    frame_times: Vec<f64>,
    /// Maximum number of samples to keep
    max_samples: usize,
    /// Last frame timestamp
    last_frame_time: Option<Instant>,
    /// Performance report cache
    cached_report: Option<PerformanceReport>,
    /// Cache expiration time
    cache_expiry: Option<Instant>,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new(PerformanceBudget::default())
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(budget: PerformanceBudget) -> Self {
        Self {
            budget,
            frame_times: Vec::new(),
            max_samples: 60, // Keep 1 second of samples at 60fps
            last_frame_time: None,
            cached_report: None,
            cache_expiry: None,
        }
    }

    /// Record a frame time
    pub fn record_frame(&mut self, frame_time: f64) {
        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }
    }

    /// Record a frame with timestamp
    pub fn record_frame_timestamp(&mut self, timestamp: Instant) {
        if let Some(last_time) = self.last_frame_time {
            let frame_time = timestamp.duration_since(last_time).as_secs_f64() * 1000.0;
            self.record_frame(frame_time);
        }
        self.last_frame_time = Some(timestamp);
    }

    /// Generate a performance report
    pub fn generate_report(
        &mut self,
        active_animations: usize,
        memory_usage: usize,
        gpu_layers: usize,
    ) -> PerformanceReport {
        // Check if we have a cached report that's still valid
        if let (Some(cached), Some(expiry)) = (&self.cached_report, &self.cache_expiry) {
            if Instant::now() < *expiry {
                return cached.clone();
            }
        }

        let average_frame_time = if self.frame_times.is_empty() {
            16.67 // Default to 60fps
        } else {
            self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64
        };

        let fps = if average_frame_time > 0.0 {
            1000.0 / average_frame_time
        } else {
            60.0
        };

        let mut report = PerformanceReport {
            average_frame_time,
            fps,
            active_animations,
            memory_usage,
            gpu_layers,
            budget_utilization: 0.0,
            timestamp: Instant::now(),
        };

        report.budget_utilization = self.budget.calculate_utilization(&report);

        // Cache the report for 100ms
        self.cached_report = Some(report.clone());
        self.cache_expiry = Some(Instant::now() + Duration::from_millis(100));

        report
    }

    /// Check if performance is within budget
    pub fn is_within_budget(&self, report: &PerformanceReport) -> bool {
        self.budget.is_within_budget(report)
    }

    /// Get the performance budget
    pub fn budget(&self) -> &PerformanceBudget {
        &self.budget
    }

    /// Update the performance budget
    pub fn update_budget(&mut self, budget: PerformanceBudget) {
        self.budget = budget;
        // Invalidate cache when budget changes
        self.cached_report = None;
        self.cache_expiry = None;
    }
}

/// GPU layer manager for optimizing GPU usage
#[derive(Debug)]
pub struct GPULayerManager {
    /// Active GPU layers
    #[cfg(feature = "web-sys")]
    layers: HashMap<String, Element>,
    /// Maximum number of layers
    max_layers: usize,
    /// Layer usage tracking
    usage_count: HashMap<String, usize>,
}

impl Default for GPULayerManager {
    fn default() -> Self {
        Self::new(50) // Default max layers
    }
}

impl GPULayerManager {
    /// Create a new GPU layer manager
    pub fn new(max_layers: usize) -> Self {
        Self {
            #[cfg(feature = "web-sys")]
            layers: HashMap::new(),
            max_layers,
            usage_count: HashMap::new(),
        }
    }

    /// Request a GPU layer for an element
    #[cfg(feature = "web-sys")]
    pub fn request_layer(&mut self, element: &Element, layer_id: String) -> bool {
        #[cfg(feature = "web-sys")]
        if self.layers.len() >= self.max_layers {
            return false;
        }

        // Enable GPU acceleration - cast to HtmlElement to access style
        #[cfg(feature = "web-sys")]
        if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
            let style = html_element.style();
            let _ = style.set_property("transform", "translateZ(0)");
            let _ = style.set_property("will-change", "transform");
        }

        self.layers.insert(layer_id.clone(), element.clone());
        *self.usage_count.entry(layer_id).or_insert(0) += 1;
        true
    }

    /// Release a GPU layer
    pub fn release_layer(&mut self, layer_id: &str) {
        if let Some(count) = self.usage_count.get_mut(layer_id) {
            *count -= 1;
            if *count == 0 {
                #[cfg(feature = "web-sys")]
                self.layers.remove(layer_id);
                self.usage_count.remove(layer_id);
            }
        }
    }

    /// Get current layer count
    pub fn layer_count(&self) -> usize {
        #[cfg(feature = "web-sys")]
        {
            self.layers.len()
        }
        #[cfg(not(feature = "web-sys"))]
        {
            0
        }
    }

    /// Check if we can allocate more layers
    pub fn can_allocate(&self) -> bool {
        #[cfg(feature = "web-sys")]
        {
            self.layers.len() < self.max_layers
        }
        #[cfg(not(feature = "web-sys"))]
        {
            false
        }
    }

    /// Get maximum layers
    pub fn max_layers(&self) -> usize {
        self.max_layers
    }
}

/// Animation pool for reusing animation objects
#[derive(Debug)]
pub struct AnimationPool {
    /// Pool of available animations
    pool: Vec<PooledAnimation>,
    /// Maximum pool size
    max_size: usize,
    /// Current pool size
    current_size: usize,
}

#[derive(Debug, Clone)]
struct PooledAnimation {
    /// Animation handle
    handle: u64,
    /// Whether the animation is in use
    in_use: bool,
    /// Last used timestamp
    last_used: Instant,
}

impl Default for AnimationPool {
    fn default() -> Self {
        Self::new(100) // Default max pool size
    }
}

impl AnimationPool {
    /// Create a new animation pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::new(),
            max_size,
            current_size: 0,
        }
    }

    /// Get an animation from the pool
    pub fn get_animation(&mut self) -> Option<u64> {
        // Try to find an unused animation
        for animation in &mut self.pool {
            if !animation.in_use {
                animation.in_use = true;
                animation.last_used = Instant::now();
                return Some(animation.handle);
            }
        }

        // If no unused animation found and we haven't reached max size, create new one
        if self.current_size < self.max_size {
            let handle = self.current_size as u64 + 1;
            self.pool.push(PooledAnimation {
                handle,
                in_use: true,
                last_used: Instant::now(),
            });
            self.current_size += 1;
            Some(handle)
        } else {
            None
        }
    }

    /// Return an animation to the pool
    pub fn return_animation(&mut self, handle: u64) {
        for animation in &mut self.pool {
            if animation.handle == handle {
                animation.in_use = false;
                animation.last_used = Instant::now();
                break;
            }
        }
    }

    /// Get current pool size
    pub fn size(&self) -> usize {
        self.current_size
    }

    /// Get number of animations in use
    pub fn in_use_count(&self) -> usize {
        self.pool.iter().filter(|a| a.in_use).count()
    }

    /// Clean up old unused animations
    pub fn cleanup(&mut self, max_age: Duration) {
        let now = Instant::now();
        self.pool.retain(|animation| {
            if !animation.in_use && now.duration_since(animation.last_used) > max_age {
                self.current_size -= 1;
                false
            } else {
                true
            }
        });
    }
}

/// Animation scheduler for managing animation timing
#[derive(Debug)]
pub struct AnimationScheduler {
    /// Scheduled animations
    scheduled: HashMap<u64, ScheduledAnimation>,
    /// Next animation handle
    next_handle: u64,
    /// Current time
    current_time: f64,
}

#[derive(Debug, Clone)]
struct ScheduledAnimation {
    /// Animation handle
    handle: u64,
    /// Start time
    start_time: f64,
    /// Duration
    duration: f64,
    /// Whether the animation is active
    active: bool,
}

impl Default for AnimationScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationScheduler {
    /// Create a new animation scheduler
    pub fn new() -> Self {
        Self {
            scheduled: HashMap::new(),
            next_handle: 1,
            current_time: 0.0,
        }
    }

    /// Schedule an animation
    pub fn schedule(&mut self, duration: f64) -> u64 {
        let handle = self.next_handle;
        self.next_handle += 1;

        self.scheduled.insert(
            handle,
            ScheduledAnimation {
                handle,
                start_time: self.current_time,
                duration,
                active: true,
            },
        );

        handle
    }

    /// Update the scheduler with current time
    pub fn update(&mut self, current_time: f64) {
        self.current_time = current_time;
    }

    /// Get animations that should be running at current time
    pub fn get_active_animations(&self) -> Vec<u64> {
        self.scheduled
            .values()
            .filter(|animation| {
                animation.active
                    && self.current_time >= animation.start_time
                    && self.current_time <= animation.start_time + animation.duration
            })
            .map(|animation| animation.handle)
            .collect()
    }

    /// Get animations that have completed
    pub fn get_completed_animations(&self) -> Vec<u64> {
        self.scheduled
            .values()
            .filter(|animation| {
                animation.active && self.current_time > animation.start_time + animation.duration
            })
            .map(|animation| animation.handle)
            .collect()
    }

    /// Cancel an animation
    pub fn cancel(&mut self, handle: u64) -> bool {
        if let Some(animation) = self.scheduled.get_mut(&handle) {
            animation.active = false;
            true
        } else {
            false
        }
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn get_progress(&self, handle: u64) -> Option<f64> {
        if let Some(animation) = self.scheduled.get(&handle) {
            if !animation.active {
                return None;
            }

            let elapsed = self.current_time - animation.start_time;
            if elapsed < 0.0 {
                return Some(0.0);
            }
            if elapsed >= animation.duration {
                return Some(1.0);
            }

            Some(elapsed / animation.duration)
        } else {
            None
        }
    }

    /// Get number of scheduled animations
    pub fn count(&self) -> usize {
        self.scheduled.len()
    }

    /// Get number of active animations
    pub fn active_count(&self) -> usize {
        self.scheduled.values().filter(|a| a.active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_report_default() {
        let report = PerformanceReport::default();
        assert_eq!(report.fps, 60.0);
        assert_eq!(report.average_frame_time, 16.67);
        assert_eq!(report.active_animations, 0);
    }

    #[test]
    fn test_performance_budget_default() {
        let budget = PerformanceBudget::default();
        assert_eq!(budget.max_frame_time, 16.67);
        assert_eq!(budget.max_animations, 100);
        assert_eq!(budget.target_fps, 60.0);
    }

    #[test]
    fn test_performance_budget_within_budget() {
        let budget = PerformanceBudget::default();
        let report = PerformanceReport {
            average_frame_time: 10.0,
            active_animations: 50,
            memory_usage: 5 * 1024 * 1024,
            gpu_layers: 25,
            ..Default::default()
        };

        assert!(budget.is_within_budget(&report));
    }

    #[test]
    fn test_performance_budget_exceeded() {
        let budget = PerformanceBudget::default();
        let report = PerformanceReport {
            average_frame_time: 20.0,       // Exceeds 16.67ms
            active_animations: 150,         // Exceeds 100
            memory_usage: 15 * 1024 * 1024, // Exceeds 10MB
            gpu_layers: 60,                 // Exceeds 50
            ..Default::default()
        };

        assert!(!budget.is_within_budget(&report));
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new(PerformanceBudget::default());

        // Record some frame times
        monitor.record_frame(16.0);
        monitor.record_frame(17.0);
        monitor.record_frame(15.0);

        let report = monitor.generate_report(10, 1024, 5);
        assert_eq!(report.active_animations, 10);
        assert_eq!(report.memory_usage, 1024);
        assert_eq!(report.gpu_layers, 5);
        assert!(report.average_frame_time > 0.0);
    }

    #[test]
    fn test_animation_pool() {
        let mut pool = AnimationPool::new(5);

        // Get animations from pool
        let handle1 = pool.get_animation().unwrap();
        let _handle2 = pool.get_animation().unwrap();

        assert_eq!(pool.size(), 2);
        assert_eq!(pool.in_use_count(), 2);

        // Return one animation
        pool.return_animation(handle1);
        assert_eq!(pool.in_use_count(), 1);

        // Get another animation (should reuse the returned one)
        let handle3 = pool.get_animation().unwrap();
        assert_eq!(handle3, handle1);
    }

    #[test]
    fn test_animation_scheduler() {
        let mut scheduler = AnimationScheduler::new();

        // Schedule an animation
        let handle = scheduler.schedule(1000.0); // 1 second duration

        // Update time to start of animation
        scheduler.update(0.0);
        assert_eq!(scheduler.get_active_animations(), vec![handle]);
        assert_eq!(scheduler.get_progress(handle), Some(0.0));

        // Update time to middle of animation
        scheduler.update(500.0);
        assert_eq!(scheduler.get_active_animations(), vec![handle]);
        assert_eq!(scheduler.get_progress(handle), Some(0.5));

        // Update time to end of animation
        scheduler.update(1000.0);
        assert_eq!(scheduler.get_active_animations(), vec![handle]);
        assert_eq!(scheduler.get_progress(handle), Some(1.0));

        // Update time past end of animation
        scheduler.update(1500.0);
        assert_eq!(scheduler.get_active_animations(), Vec::<u64>::new());
        assert_eq!(scheduler.get_completed_animations(), vec![handle]);
    }

    #[test]
    fn test_gpu_layer_manager() {
        let manager = GPULayerManager::new(10);

        // Mock element (we can't create real DOM elements in tests)
        // This test would need to be adapted for actual DOM testing

        assert_eq!(manager.layer_count(), 0);
        assert!(manager.can_allocate());
        assert_eq!(manager.max_layers(), 10);
    }
}
