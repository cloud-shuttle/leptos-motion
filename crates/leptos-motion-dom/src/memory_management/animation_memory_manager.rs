//! Animation memory manager for tracking and cleaning up animations

use crate::animation_trait::{Animation, AnimationResult};
use super::{
    memory_stats::{MemoryStats, MemoryPressure, MemoryTracker},
    gc_strategy::{GCStrategy, GarbageCollector},
};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::sync::{Arc, Mutex};

/// Memory manager for animations
pub struct AnimationMemoryManager {
    /// Garbage collector
    pub gc: GarbageCollector,
    /// Memory statistics
    stats: MemoryStats,
    /// Memory tracker
    memory_tracker: Arc<Mutex<MemoryTracker>>,
    /// Memory pressure threshold
    memory_pressure_threshold: f64,
    /// Maximum animations before cleanup
    max_animations: usize,
}

impl AnimationMemoryManager {
    /// Create a new animation memory manager
    pub fn new() -> Self {
        Self {
            gc: GarbageCollector::new(),
            stats: MemoryStats::default(),
            memory_tracker: Arc::new(Mutex::new(MemoryTracker::new())),
            memory_pressure_threshold: 0.7,
            max_animations: 1000,
        }
    }

    /// Register an animation for memory tracking
    pub fn register_animation(&mut self, id: String, animation: Rc<RefCell<Box<dyn Animation>>>) {
        // Estimate memory usage
        let estimated_size = 1024; // Rough estimate per animation
        
        if let Ok(tracker) = self.memory_tracker.lock() {
            tracker.allocate(estimated_size);
        }

        self.stats.total_allocated += estimated_size;
        self.stats.memory_in_use += estimated_size;
        self.stats.active_animations += 1;

        // Register with garbage collector
        self.gc.register_animation(id, animation);
    }

    /// Mark an animation as completed
    pub fn mark_completed(&mut self, id: String) {
        self.stats.completed_animations += 1;
        self.gc.mark_completed(id);
    }

    /// Run garbage collection
    pub fn garbage_collect(&mut self, strategy: GCStrategy) -> AnimationResult<()> {
        let pressure = self.check_memory_pressure();
        
        // Only run GC if the strategy is appropriate for the current pressure
        if !strategy.should_run(pressure) {
            return Ok(());
        }

        // Run garbage collection
        self.gc.garbage_collect(strategy)?;

        // Update statistics
        self.stats.gc_cycles += 1;
        if let Ok(tracker) = self.memory_tracker.lock() {
            tracker.record_gc_cycle();
        }

        // Update memory statistics
        self.update_memory_stats();

        Ok(())
    }

    /// Check memory pressure level
    pub fn check_memory_pressure(&self) -> MemoryPressure {
        let active_count = self.stats.active_animations;
        let completed_count = self.stats.completed_animations;
        let total_count = active_count + completed_count;

        if total_count == 0 {
            return MemoryPressure::Low;
        }

        let pressure_ratio = total_count as f64 / self.max_animations as f64;

        if pressure_ratio >= 0.9 {
            MemoryPressure::Critical
        } else if pressure_ratio >= 0.7 {
            MemoryPressure::High
        } else if pressure_ratio >= 0.5 {
            MemoryPressure::Medium
        } else {
            MemoryPressure::Low
        }
    }

    /// Get memory pressure level
    pub fn get_memory_pressure_level(&self) -> MemoryPressure {
        self.check_memory_pressure()
    }

    /// Get memory statistics
    pub fn get_memory_stats(&mut self) -> MemoryStats {
        self.update_memory_stats();
        self.stats.clone()
    }

    /// Update memory statistics
    fn update_memory_stats(&mut self) {
        if let Ok(tracker) = self.memory_tracker.lock() {
            let tracker_stats = tracker.get_stats();
            self.stats.total_allocated = tracker_stats.total_allocated;
            self.stats.memory_in_use = tracker_stats.memory_in_use;
            self.stats.memory_freed = tracker_stats.memory_freed;
            self.stats.gc_cycles = tracker_stats.gc_cycles;
        }

        // Update animation counts
        self.stats.active_animations = self.gc.get_active_count();
        self.stats.completed_animations = self.gc.get_completed_count();

        // Calculate memory pressure
        let pressure = self.check_memory_pressure();
        self.stats.memory_pressure = match pressure {
            MemoryPressure::Low => 0.0,
            MemoryPressure::Medium => 0.5,
            MemoryPressure::High => 0.7,
            MemoryPressure::Critical => 0.9,
        };
    }

    /// Force cleanup of all completed animations
    pub fn force_cleanup(&mut self) -> AnimationResult<()> {
        // Clear all completed animations
        self.gc.clear();

        // Reset memory tracker
        if let Ok(tracker) = self.memory_tracker.lock() {
            tracker.reset();
        }

        // Reset statistics
        self.stats = MemoryStats::default();

        Ok(())
    }

    /// Set garbage collection interval
    pub fn set_gc_interval(&mut self, interval: Duration) {
        self.gc.set_gc_interval(interval);
    }

    /// Set memory pressure threshold
    pub fn set_memory_pressure_threshold(&mut self, threshold: f64) {
        self.memory_pressure_threshold = threshold.clamp(0.0, 1.0);
        self.gc.set_memory_pressure_threshold(threshold);
    }

    /// Set maximum animations before cleanup
    pub fn set_max_animations(&mut self, max: usize) {
        self.max_animations = max;
        self.gc.set_max_animations(max);
    }

    /// Get memory usage report
    pub fn get_memory_report(&mut self) -> String {
        let stats = self.get_memory_stats();
        let pressure = self.get_memory_pressure_level();
        
        format!(
            "Memory Management Report:\n\
            Total Allocated: {} bytes ({:.2} MB)\n\
            Memory In Use: {} bytes ({:.2} MB)\n\
            Memory Freed: {} bytes ({:.2} MB)\n\
            Active Animations: {}\n\
            Completed Animations: {}\n\
            Leaked Animations: {}\n\
            Memory Pressure: {:.2}% ({:?})\n\
            GC Cycles: {}\n\
            GC Interval: {:?}",
            stats.total_allocated,
            stats.total_allocated as f64 / 1_000_000.0,
            stats.memory_in_use,
            stats.memory_in_use as f64 / 1_000_000.0,
            stats.memory_freed,
            stats.memory_freed as f64 / 1_000_000.0,
            stats.active_animations,
            stats.completed_animations,
            stats.leaked_animations,
            stats.memory_pressure * 100.0,
            pressure,
            stats.gc_cycles,
            Duration::from_secs(30) // Default interval
        )
    }

    /// Check if memory management is healthy
    pub fn is_healthy(&self) -> bool {
        let pressure = self.check_memory_pressure();
        !pressure.is_critical() && self.stats.leaked_animations == 0
    }

    /// Get memory health status
    pub fn get_health_status(&mut self) -> String {
        let pressure = self.check_memory_pressure();
        let stats = self.get_memory_stats();
        
        if pressure.is_critical() {
            "CRITICAL: Memory pressure is too high".to_string()
        } else if self.stats.leaked_animations > 0 {
            format!("WARNING: {} leaked animations detected", self.stats.leaked_animations)
        } else if pressure.is_high() {
            "WARNING: High memory pressure".to_string()
        } else {
            "HEALTHY: Memory management is normal".to_string()
        }
    }
}

impl Default for AnimationMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
