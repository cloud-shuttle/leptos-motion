//! Automatic memory manager that runs cleanup in the background

use crate::animation_trait::{Animation, AnimationError, AnimationResult};
use super::{
    memory_stats::{MemoryStats, MemoryPressure},
    gc_strategy::GCStrategy,
    animation_memory_manager::AnimationMemoryManager,
};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};

/// Automatic memory manager that runs cleanup in the background
pub struct AutoMemoryManager {
    memory_manager: AnimationMemoryManager,
    auto_cleanup_enabled: bool,
    last_cleanup_check: Instant,
    cleanup_check_interval: Duration,
}

impl AutoMemoryManager {
    /// Create a new automatic memory manager
    pub fn new() -> Self {
        Self {
            memory_manager: AnimationMemoryManager::new(),
            auto_cleanup_enabled: true,
            last_cleanup_check: Instant::now(),
            cleanup_check_interval: Duration::from_secs(2), // Check every 2 seconds
        }
    }

    /// Register an animation
    pub fn register_animation(&mut self, id: String, animation: Rc<RefCell<Box<dyn Animation>>>) {
        self.memory_manager.register_animation(id, animation);
    }

    /// Mark an animation as completed
    pub fn mark_completed(&mut self, id: String) {
        self.memory_manager.mark_completed(id);
    }

    /// Update the automatic memory manager
    /// This should be called regularly to trigger automatic cleanup
    pub fn update(&mut self) -> AnimationResult<()> {
        if !self.auto_cleanup_enabled {
            return Ok(());
        }

        // Check if enough time has passed since last cleanup check
        if self.last_cleanup_check.elapsed() < self.cleanup_check_interval {
            return Ok(());
        }

        // Check memory pressure and run appropriate cleanup
        let pressure = self.memory_manager.check_memory_pressure();
        
        if self.memory_manager.gc.should_run_gc(pressure) {
            let strategy = self.memory_manager.gc.get_recommended_strategy(pressure);
            self.memory_manager.garbage_collect(strategy)?;
        }

        self.last_cleanup_check = Instant::now();
        Ok(())
    }

    /// Enable automatic cleanup
    pub fn enable_auto_cleanup(&mut self) {
        self.auto_cleanup_enabled = true;
    }

    /// Disable automatic cleanup
    pub fn disable_auto_cleanup(&mut self) {
        self.auto_cleanup_enabled = false;
    }

    /// Check if automatic cleanup is enabled
    pub fn is_auto_cleanup_enabled(&self) -> bool {
        self.auto_cleanup_enabled
    }

    /// Set cleanup check interval
    pub fn set_cleanup_check_interval(&mut self, interval: Duration) {
        self.cleanup_check_interval = interval;
    }

    /// Get cleanup check interval
    pub fn get_cleanup_check_interval(&self) -> Duration {
        self.cleanup_check_interval
    }

    /// Get memory statistics
    pub fn get_memory_stats(&mut self) -> MemoryStats {
        self.memory_manager.get_memory_stats()
    }

    /// Get memory pressure level
    pub fn get_memory_pressure_level(&self) -> MemoryPressure {
        self.memory_manager.get_memory_pressure_level()
    }

    /// Get memory usage report
    pub fn get_memory_report(&mut self) -> String {
        let mut report = self.memory_manager.get_memory_report();
        report.push_str(&format!(
            "\nAuto Cleanup: {}\n\
            Cleanup Interval: {:?}\n\
            Last Check: {:?}",
            if self.auto_cleanup_enabled { "Enabled" } else { "Disabled" },
            self.cleanup_check_interval,
            self.last_cleanup_check.elapsed()
        ));
        report
    }

    /// Force cleanup
    pub fn force_cleanup(&mut self) -> AnimationResult<()> {
        self.memory_manager.force_cleanup()
    }

    /// Set garbage collection interval
    pub fn set_gc_interval(&mut self, interval: Duration) {
        self.memory_manager.set_gc_interval(interval);
    }

    /// Set memory pressure threshold
    pub fn set_memory_pressure_threshold(&mut self, threshold: f64) {
        self.memory_manager.set_memory_pressure_threshold(threshold);
    }

    /// Set maximum animations before cleanup
    pub fn set_max_animations(&mut self, max: usize) {
        self.memory_manager.set_max_animations(max);
    }

    /// Check if memory management is healthy
    pub fn is_healthy(&self) -> bool {
        self.memory_manager.is_healthy()
    }

    /// Get memory health status
    pub fn get_health_status(&mut self) -> String {
        let mut status = self.memory_manager.get_health_status();
        
        if !self.auto_cleanup_enabled {
            status.push_str(" (Auto cleanup disabled)");
        }
        
        status
    }

    /// Get the underlying memory manager
    pub fn get_memory_manager(&self) -> &AnimationMemoryManager {
        &self.memory_manager
    }

    /// Get mutable access to the underlying memory manager
    pub fn get_memory_manager_mut(&mut self) -> &mut AnimationMemoryManager {
        &mut self.memory_manager
    }

    /// Run garbage collection
    pub fn garbage_collect(&mut self, strategy: GCStrategy) -> AnimationResult<()> {
        self.memory_manager.garbage_collect(strategy)
    }

    /// Get memory pressure level
    pub fn get_memory_pressure(&self) -> MemoryPressure {
        self.memory_manager.get_memory_pressure_level()
    }
}

impl Default for AutoMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
