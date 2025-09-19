//! Memory statistics for pooling

/// Memory usage statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    /// Total memory allocated in bytes
    pub total_allocated: usize,
    /// Memory currently in use in bytes
    pub memory_in_use: usize,
    /// Memory available for allocation in bytes
    pub memory_available: usize,
    /// Number of active animations
    pub active_animations: usize,
    /// Number of pooled animations
    pub pooled_animations: usize,
    /// Memory fragmentation percentage
    pub fragmentation_percent: f64,
    /// Peak memory usage in bytes
    pub peak_memory_usage: usize,
    /// Number of memory allocations
    pub allocation_count: usize,
    /// Number of memory deallocations
    pub deallocation_count: usize,
}

impl MemoryStats {
    /// Create new memory statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Update memory statistics
    pub fn update(&mut self, allocated: usize, in_use: usize, active: usize, pooled: usize) {
        self.total_allocated = allocated;
        self.memory_in_use = in_use;
        self.memory_available = allocated.saturating_sub(in_use);
        self.active_animations = active;
        self.pooled_animations = pooled;
        
        if allocated > 0 {
            self.fragmentation_percent = (in_use as f64 / allocated as f64) * 100.0;
        } else {
            self.fragmentation_percent = 0.0;
        }

        if in_use > self.peak_memory_usage {
            self.peak_memory_usage = in_use;
        }
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, size: usize) {
        self.allocation_count += 1;
        self.total_allocated += size;
        self.memory_in_use += size;
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, size: usize) {
        self.deallocation_count += 1;
        self.memory_in_use = self.memory_in_use.saturating_sub(size);
    }

    /// Get memory usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.total_allocated > 0 {
            (self.memory_in_use as f64 / self.total_allocated as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Check if memory usage is high
    pub fn is_high_usage(&self, threshold: f64) -> bool {
        self.usage_percentage() > threshold
    }

    /// Get memory efficiency (active vs pooled)
    pub fn efficiency(&self) -> f64 {
        let total = self.active_animations + self.pooled_animations;
        if total > 0 {
            (self.active_animations as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }
}
