//! Garbage collection strategies and implementation

use crate::animation_trait::{Animation, AnimationResult};
use super::memory_stats::MemoryPressure;
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::time::{Duration, Instant};

/// Garbage collection strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCStrategy {
    /// Conservative garbage collection
    Conservative,
    /// Aggressive garbage collection
    Aggressive,
    /// Emergency garbage collection
    Emergency,
}

impl GCStrategy {
    /// Get the recommended GC interval for this strategy
    pub fn get_interval(&self) -> Duration {
        match self {
            GCStrategy::Conservative => Duration::from_secs(30),
            GCStrategy::Aggressive => Duration::from_secs(10),
            GCStrategy::Emergency => Duration::from_secs(1),
        }
    }

    /// Get the maximum completed animations before cleanup
    pub fn get_max_completed(&self) -> usize {
        match self {
            GCStrategy::Conservative => 1000,
            GCStrategy::Aggressive => 100,
            GCStrategy::Emergency => 10,
        }
    }

    /// Check if this strategy should run based on memory pressure
    pub fn should_run(&self, pressure: MemoryPressure) -> bool {
        match self {
            GCStrategy::Conservative => matches!(pressure, MemoryPressure::Medium | MemoryPressure::High | MemoryPressure::Critical),
            GCStrategy::Aggressive => matches!(pressure, MemoryPressure::High | MemoryPressure::Critical),
            GCStrategy::Emergency => matches!(pressure, MemoryPressure::Critical),
        }
    }
}

/// Garbage collection implementation
pub struct GarbageCollector {
    /// Active animations with weak references for cleanup
    active_animations: HashMap<String, Weak<RefCell<Box<dyn Animation>>>>,
    /// Completed animations waiting for cleanup
    completed_animations: VecDeque<String>,
    /// Last garbage collection time
    last_gc_time: Instant,
    /// Garbage collection interval
    gc_interval: Duration,
    /// Memory pressure threshold
    memory_pressure_threshold: f64,
    /// Maximum animations before cleanup
    max_animations: usize,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            active_animations: HashMap::new(),
            completed_animations: VecDeque::new(),
            last_gc_time: Instant::now(),
            gc_interval: Duration::from_secs(30),
            memory_pressure_threshold: 0.7,
            max_animations: 1000,
        }
    }

    /// Register an animation for tracking
    pub fn register_animation(&mut self, id: String, animation: Rc<RefCell<Box<dyn Animation>>>) {
        let weak_ref = Rc::downgrade(&animation);
        self.active_animations.insert(id, weak_ref);
    }

    /// Mark an animation as completed
    pub fn mark_completed(&mut self, id: String) {
        if self.active_animations.contains_key(&id) {
            self.completed_animations.push_back(id);
        }
    }

    /// Run garbage collection with the specified strategy
    pub fn garbage_collect(&mut self, strategy: GCStrategy) -> AnimationResult<()> {
        // Check if enough time has passed since last GC
        if strategy != GCStrategy::Emergency && self.last_gc_time.elapsed() < self.gc_interval {
            return Ok(());
        }

        match strategy {
            GCStrategy::Conservative => self.conservative_gc(),
            GCStrategy::Aggressive => self.aggressive_gc(),
            GCStrategy::Emergency => self.emergency_gc(),
        }?;

        self.last_gc_time = Instant::now();
        Ok(())
    }

    /// Conservative garbage collection - clean up only dead references
    fn conservative_gc(&mut self) -> AnimationResult<()> {
        let mut dead_animations = Vec::new();

        // Find dead animations
        for (id, weak_ref) in &self.active_animations {
            if weak_ref.upgrade().is_none() {
                dead_animations.push(id.clone());
            }
        }

        // Remove dead animations
        for id in dead_animations {
            self.remove_animation(&id);
        }

        // Clean up some completed animations
        self.cleanup_completed_animations(10)?;

        Ok(())
    }

    /// Aggressive garbage collection - clean up more aggressively
    fn aggressive_gc(&mut self) -> AnimationResult<()> {
        // First do conservative cleanup
        self.conservative_gc()?;

        // Clean up animations that have been completed for a while
        let mut to_remove = Vec::new();
        let completed_count = self.completed_animations.len();
        
        // Remove oldest completed animations if we have too many
        if completed_count > 100 {
            for _ in 0..(completed_count - 50) {
                if let Some(id) = self.completed_animations.pop_front() {
                    to_remove.push(id);
                }
            }
        }

        for id in to_remove {
            self.remove_animation(&id);
        }

        Ok(())
    }

    /// Emergency garbage collection - clean up everything possible
    fn emergency_gc(&mut self) -> AnimationResult<()> {
        // Clear all completed animations
        self.completed_animations.clear();

        // Remove all dead animations
        let mut dead_animations = Vec::new();
        for (id, weak_ref) in &self.active_animations {
            if weak_ref.upgrade().is_none() {
                dead_animations.push(id.clone());
            }
        }

        for id in dead_animations {
            self.remove_animation(&id);
        }

        // Force cleanup of any remaining completed animations
        self.cleanup_completed_animations(100)?;

        Ok(())
    }

    /// Clean up completed animations
    fn cleanup_completed_animations(&mut self, max_cleanup: usize) -> AnimationResult<()> {
        let mut cleaned_count = 0;
        
        while let Some(id) = self.completed_animations.pop_front() {
            self.remove_animation(&id);
            cleaned_count += 1;
            
            // Limit cleanup per cycle to avoid blocking
            if cleaned_count >= max_cleanup {
                break;
            }
        }

        Ok(())
    }

    /// Remove an animation from tracking
    fn remove_animation(&mut self, id: &str) {
        self.active_animations.remove(id);
    }

    /// Check if garbage collection should run
    pub fn should_run_gc(&self, pressure: MemoryPressure) -> bool {
        match pressure {
            MemoryPressure::Low => false,
            MemoryPressure::Medium => self.last_gc_time.elapsed() >= self.gc_interval,
            MemoryPressure::High => self.last_gc_time.elapsed() >= Duration::from_secs(10),
            MemoryPressure::Critical => true,
        }
    }

    /// Get the recommended GC strategy based on memory pressure
    pub fn get_recommended_strategy(&self, pressure: MemoryPressure) -> GCStrategy {
        match pressure {
            MemoryPressure::Low => GCStrategy::Conservative,
            MemoryPressure::Medium => GCStrategy::Conservative,
            MemoryPressure::High => GCStrategy::Aggressive,
            MemoryPressure::Critical => GCStrategy::Emergency,
        }
    }

    /// Set garbage collection interval
    pub fn set_gc_interval(&mut self, interval: Duration) {
        self.gc_interval = interval;
    }

    /// Set memory pressure threshold
    pub fn set_memory_pressure_threshold(&mut self, threshold: f64) {
        self.memory_pressure_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set maximum animations before cleanup
    pub fn set_max_animations(&mut self, max: usize) {
        self.max_animations = max;
    }

    /// Get active animation count
    pub fn get_active_count(&self) -> usize {
        self.active_animations.len()
    }

    /// Get completed animation count
    pub fn get_completed_count(&self) -> usize {
        self.completed_animations.len()
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        self.active_animations.clear();
        self.completed_animations.clear();
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
