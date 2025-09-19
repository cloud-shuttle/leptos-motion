//! Memory statistics and pressure monitoring

use std::sync::atomic::{AtomicUsize, Ordering};

/// Memory management statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total memory allocated (estimated)
    pub total_allocated: usize,
    /// Memory currently in use
    pub memory_in_use: usize,
    /// Memory freed through cleanup
    pub memory_freed: usize,
    /// Number of active animations
    pub active_animations: usize,
    /// Number of completed animations
    pub completed_animations: usize,
    /// Number of leaked animations (should be 0)
    pub leaked_animations: usize,
    /// Memory pressure level (0.0 to 1.0)
    pub memory_pressure: f64,
    /// Garbage collection cycles performed
    pub gc_cycles: usize,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            total_allocated: 0,
            memory_in_use: 0,
            memory_freed: 0,
            active_animations: 0,
            completed_animations: 0,
            leaked_animations: 0,
            memory_pressure: 0.0,
            gc_cycles: 0,
        }
    }
}

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    /// Low memory pressure - normal operation
    Low,
    /// Medium memory pressure - start cleanup
    Medium,
    /// High memory pressure - aggressive cleanup
    High,
    /// Critical memory pressure - emergency cleanup
    Critical,
}

impl MemoryPressure {
    /// Get memory pressure as a percentage
    pub fn as_percentage(&self) -> f64 {
        match self {
            MemoryPressure::Low => 0.0,
            MemoryPressure::Medium => 0.5,
            MemoryPressure::High => 0.7,
            MemoryPressure::Critical => 0.9,
        }
    }

    /// Check if memory pressure is high
    pub fn is_high(&self) -> bool {
        matches!(self, MemoryPressure::High | MemoryPressure::Critical)
    }

    /// Check if memory pressure is critical
    pub fn is_critical(&self) -> bool {
        matches!(self, MemoryPressure::Critical)
    }
}

/// Memory tracker for monitoring usage
pub struct MemoryTracker {
    total_allocated: AtomicUsize,
    memory_in_use: AtomicUsize,
    memory_freed: AtomicUsize,
    gc_cycles: AtomicUsize,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            memory_in_use: AtomicUsize::new(0),
            memory_freed: AtomicUsize::new(0),
            gc_cycles: AtomicUsize::new(0),
        }
    }

    /// Record memory allocation
    pub fn allocate(&self, size: usize) {
        self.total_allocated.fetch_add(size, Ordering::Relaxed);
        self.memory_in_use.fetch_add(size, Ordering::Relaxed);
    }

    /// Record memory deallocation
    pub fn deallocate(&self, size: usize) {
        self.memory_freed.fetch_add(size, Ordering::Relaxed);
        self.memory_in_use.fetch_sub(size, Ordering::Relaxed);
    }

    /// Record garbage collection cycle
    pub fn record_gc_cycle(&self) {
        self.gc_cycles.fetch_add(1, Ordering::Relaxed);
    }

    /// Get total allocated memory
    pub fn get_total_allocated(&self) -> usize {
        self.total_allocated.load(Ordering::Relaxed)
    }

    /// Get memory currently in use
    pub fn get_memory_in_use(&self) -> usize {
        self.memory_in_use.load(Ordering::Relaxed)
    }

    /// Get memory freed
    pub fn get_memory_freed(&self) -> usize {
        self.memory_freed.load(Ordering::Relaxed)
    }

    /// Get garbage collection cycles
    pub fn get_gc_cycles(&self) -> usize {
        self.gc_cycles.load(Ordering::Relaxed)
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_allocated: self.get_total_allocated(),
            memory_in_use: self.get_memory_in_use(),
            memory_freed: self.get_memory_freed(),
            active_animations: 0, // Will be set by the manager
            completed_animations: 0, // Will be set by the manager
            leaked_animations: 0, // Will be set by the manager
            memory_pressure: 0.0, // Will be calculated by the manager
            gc_cycles: self.get_gc_cycles(),
        }
    }

    /// Reset all counters
    pub fn reset(&self) {
        self.total_allocated.store(0, Ordering::Relaxed);
        self.memory_in_use.store(0, Ordering::Relaxed);
        self.memory_freed.store(0, Ordering::Relaxed);
        self.gc_cycles.store(0, Ordering::Relaxed);
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}
