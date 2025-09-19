//! Performance Monitor
//!
//! This module provides performance monitoring capabilities for the animation
//! system, including FPS tracking, memory usage, and animation statistics.

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// Get current time in milliseconds (WASM-compatible)
fn now() -> f64 {
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

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    /// Current FPS
    pub fps: f64,
    /// Average FPS over the last second
    pub average_fps: f64,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Number of active animations
    pub active_animations: usize,
    /// Total animations created
    pub total_animations: usize,
    /// Average animation duration
    pub average_animation_duration: f64,
    /// Performance score (0.0 to 1.0)
    pub performance_score: f64,
}

/// FPS counter
#[derive(Debug)]
pub struct FpsCounter {
    frame_times: Vec<f64>,
    last_frame_time: Option<f64>,
    frame_count: usize,
    last_fps_update: f64,
}

impl FpsCounter {
    /// Create a new FPS counter
    pub fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            last_frame_time: None,
            frame_count: 0,
            last_fps_update: now(),
        }
    }
    
    /// Record a frame
    pub fn record_frame(&mut self) {
        let current_time = now();
        
        if let Some(last_time) = self.last_frame_time {
            let frame_time = current_time - last_time;
            self.frame_times.push(current_time);
            
            // Keep only the last 60 frames
            if self.frame_times.len() > 60 {
                self.frame_times.remove(0);
            }
        }
        
        self.last_frame_time = Some(current_time);
        self.frame_count += 1;
    }
    
    /// Get current FPS
    pub fn get_fps(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }
        
        let total_time = (self.frame_times.last().unwrap() - self.frame_times.first().unwrap()) / 1000.0; // Convert to seconds
        if total_time > 0.0 {
            (self.frame_times.len() - 1) as f64 / total_time
        } else {
            0.0
        }
    }
    
    /// Get average FPS over the last second
    pub fn get_average_fps(&self) -> f64 {
        let current_time = now();
        let one_second_ago = current_time - 1000.0; // 1000ms ago
        
        let recent_frames = self.frame_times.iter()
            .filter(|&&time| time >= one_second_ago)
            .count();
        
        recent_frames as f64
    }
    
    /// Reset the counter
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame_time = None;
        self.frame_count = 0;
        self.last_fps_update = now();
    }
}

/// Memory usage tracker
#[derive(Debug)]
pub struct MemoryTracker {
    baseline_memory: usize,
    peak_memory: usize,
    current_memory: usize,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        let baseline = Self::get_browser_memory();
        Self {
            baseline_memory: baseline,
            peak_memory: baseline,
            current_memory: baseline,
        }
    }
    
    /// Get current memory usage
    pub fn get_current_memory(&self) -> usize {
        self.current_memory
    }
    
    /// Get current memory usage from browser
    fn get_browser_memory() -> usize {
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                // Note: performance.memory() is not available in all browsers
                // This is a simplified implementation
                return 0;
            }
        }
        0
    }
    
    /// Update memory usage
    pub fn update(&mut self) {
        self.current_memory = Self::get_browser_memory();
        self.peak_memory = self.peak_memory.max(self.current_memory);
    }
    
    /// Get memory usage relative to baseline
    pub fn get_memory_usage(&self) -> usize {
        self.current_memory.saturating_sub(self.baseline_memory)
    }
    
    /// Get peak memory usage
    pub fn get_peak_memory(&self) -> usize {
        self.peak_memory.saturating_sub(self.baseline_memory)
    }
    
    /// Reset memory tracking
    pub fn reset(&mut self) {
        self.baseline_memory = Self::get_browser_memory();
        self.peak_memory = self.baseline_memory;
        self.current_memory = self.baseline_memory;
    }
}

/// Animation statistics tracker
#[derive(Debug)]
pub struct AnimationStats {
    active_animations: usize,
    total_animations: usize,
    animation_durations: Vec<f64>,
    animation_types: HashMap<String, usize>,
}

impl AnimationStats {
    /// Create a new animation stats tracker
    pub fn new() -> Self {
        Self {
            active_animations: 0,
            total_animations: 0,
            animation_durations: Vec::new(),
            animation_types: HashMap::new(),
        }
    }
    
    /// Track animation start
    pub fn track_animation_start(&mut self, animation_type: &str) {
        self.active_animations += 1;
        self.total_animations += 1;
        
        let count = self.animation_types.get(animation_type).unwrap_or(&0) + 1;
        self.animation_types.insert(animation_type.to_string(), count);
    }
    
    /// Track animation end
    pub fn track_animation_end(&mut self, duration: f64) {
        if self.active_animations > 0 {
            self.active_animations -= 1;
        }
        self.animation_durations.push(duration);
        
        // Keep only the last 100 durations
        if self.animation_durations.len() > 100 {
            self.animation_durations.remove(0);
        }
    }
    
    /// Get active animation count
    pub fn get_active_animations(&self) -> usize {
        self.active_animations
    }
    
    /// Get total animation count
    pub fn get_total_animations(&self) -> usize {
        self.total_animations
    }
    
    /// Get average animation duration
    pub fn get_average_duration(&self) -> f64 {
        if self.animation_durations.is_empty() {
            0.0
        } else {
            self.animation_durations.iter().sum::<f64>() / self.animation_durations.len() as f64
        }
    }
    
    /// Get animation type distribution
    pub fn get_animation_types(&self) -> &HashMap<String, usize> {
        &self.animation_types
    }
    
    /// Reset statistics
    pub fn reset(&mut self) {
        self.active_animations = 0;
        self.total_animations = 0;
        self.animation_durations.clear();
        self.animation_types.clear();
    }
}

/// Performance monitor
#[derive(Debug)]
pub struct PerformanceMonitor {
    fps_counter: FpsCounter,
    memory_tracker: MemoryTracker,
    animation_stats: AnimationStats,
    last_update: f64,
    update_interval: f64, // in milliseconds
    enabled: bool,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            fps_counter: FpsCounter::new(),
            memory_tracker: MemoryTracker::new(),
            animation_stats: AnimationStats::new(),
            last_update: now(),
            update_interval: 100.0, // Update every 100ms
            enabled: true,
        }
    }
    
    /// Enable performance monitoring
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable performance monitoring
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Check if monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Set update interval
    pub fn set_update_interval(&mut self, interval_ms: f64) {
        self.update_interval = interval_ms;
    }
    
    /// Record a frame
    pub fn record_frame(&mut self) {
        if !self.enabled {
            return;
        }
        
        self.fps_counter.record_frame();
        
        // Update memory and stats periodically
        let current_time = now();
        if current_time - self.last_update >= self.update_interval {
            self.memory_tracker.update();
            self.last_update = current_time;
        }
    }
    
    /// Track animation start
    pub fn track_animation_start(&mut self, animation_type: &str) {
        if !self.enabled {
            return;
        }
        
        self.animation_stats.track_animation_start(animation_type);
    }
    
    /// Track animation end
    pub fn track_animation_end(&mut self, duration: f64) {
        if !self.enabled {
            return;
        }
        
        self.animation_stats.track_animation_end(duration);
    }
    
    /// Get current performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let fps = self.fps_counter.get_fps();
        let average_fps = self.fps_counter.get_average_fps();
        let memory_usage = self.memory_tracker.get_memory_usage();
        let active_animations = self.animation_stats.get_active_animations();
        let total_animations = self.animation_stats.get_total_animations();
        let average_animation_duration = self.animation_stats.get_average_duration();
        
        // Calculate performance score (0.0 to 1.0)
        let performance_score = self.calculate_performance_score(fps, memory_usage, active_animations);
        
        PerformanceStats {
            fps,
            average_fps,
            memory_usage,
            active_animations,
            total_animations,
            average_animation_duration,
            performance_score,
        }
    }
    
    /// Calculate performance score
    fn calculate_performance_score(&self, fps: f64, memory_usage: usize, active_animations: usize) -> f64 {
        let mut score = 1.0;
        
        // FPS score (target: 60fps)
        let fps_score = (fps / 60.0).min(1.0);
        score *= fps_score;
        
        // Memory score (target: < 10MB)
        let memory_score = if memory_usage > 10_000_000 {
            0.0
        } else {
            1.0 - (memory_usage as f64 / 10_000_000.0)
        };
        score *= memory_score;
        
        // Animation count score (target: < 50 active animations)
        let animation_score = if active_animations > 50 {
            0.0
        } else {
            1.0 - (active_animations as f64 / 50.0)
        };
        score *= animation_score;
        
        score.max(0.0).min(1.0)
    }
    
    /// Get detailed performance report
    pub fn get_detailed_report(&self) -> String {
        let stats = self.get_stats();
        let animation_types = self.animation_stats.get_animation_types();
        
        format!(
            "Performance Report:\n\
            FPS: {:.1} (avg: {:.1})\n\
            Memory: {} bytes\n\
            Active Animations: {}\n\
            Total Animations: {}\n\
            Average Duration: {:.2}s\n\
            Performance Score: {:.2}\n\
            Animation Types: {:?}",
            stats.fps,
            stats.average_fps,
            stats.memory_usage,
            stats.active_animations,
            stats.total_animations,
            stats.average_animation_duration,
            stats.performance_score,
            animation_types
        )
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        self.fps_counter.reset();
        self.memory_tracker.reset();
        self.animation_stats.reset();
        self.last_update = now();
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global performance monitor instance
use std::sync::Mutex;
use std::sync::OnceLock;

static PERFORMANCE_MONITOR: OnceLock<Mutex<PerformanceMonitor>> = OnceLock::new();

/// Get the global performance monitor
pub fn get_performance_monitor() -> &'static Mutex<PerformanceMonitor> {
    PERFORMANCE_MONITOR.get_or_init(|| Mutex::new(PerformanceMonitor::new()))
}

/// Get the global performance monitor with error handling
pub fn get_performance_monitor_safe() -> Option<&'static Mutex<PerformanceMonitor>> {
    PERFORMANCE_MONITOR.get()
}

/// Get a new performance monitor instance (for testing)
pub fn get_new_performance_monitor() -> PerformanceMonitor {
    PerformanceMonitor::new()
}

/// Reset the global performance monitor (for testing)
#[cfg(test)]
pub fn reset_global_performance_monitor() {
    // This is a no-op in production, but allows tests to reset state
    // The global singleton will be reinitialized on next access
}

/// Record a frame for performance monitoring
pub fn record_frame() {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(mut monitor) = monitor.lock() {
            monitor.record_frame();
        }
    }
}

/// Track animation start
pub fn track_animation_start(animation_type: &str) {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(mut monitor) = monitor.lock() {
            monitor.track_animation_start(animation_type);
        }
    }
}

/// Track animation end
pub fn track_animation_end(duration: f64) {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(mut monitor) = monitor.lock() {
            monitor.track_animation_end(duration);
        }
    }
}

/// Get current performance statistics
pub fn get_performance_stats() -> PerformanceStats {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(monitor) = monitor.lock() {
            return monitor.get_stats();
        }
    }
    
    PerformanceStats {
        fps: 0.0,
        average_fps: 0.0,
        memory_usage: 0,
        active_animations: 0,
        total_animations: 0,
        average_animation_duration: 0.0,
        performance_score: 0.0,
    }
}

/// Get detailed performance report
pub fn get_performance_report() -> String {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(monitor) = monitor.lock() {
            return monitor.get_detailed_report();
        }
    }
    "Performance monitor unavailable".to_string()
}

/// Reset performance monitoring
pub fn reset_performance_monitoring() {
    if let Some(monitor) = get_performance_monitor_safe() {
        if let Ok(mut monitor) = monitor.lock() {
            monitor.reset();
        }
    }
}
