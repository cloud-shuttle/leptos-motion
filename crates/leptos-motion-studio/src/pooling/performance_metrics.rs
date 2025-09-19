//! Performance metrics for pooling

/// Performance metrics for the animation pool
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Total number of animations created
    pub total_animations_created: usize,
    /// Total number of animations reused
    pub total_animations_reused: usize,
    /// Average time to create a new animation (microseconds)
    pub avg_creation_time_us: u64,
    /// Average time to reuse an animation (microseconds)
    pub avg_reuse_time_us: u64,
    /// Total time spent creating animations (microseconds)
    pub total_creation_time_us: u64,
    /// Total time spent reusing animations (microseconds)
    pub total_reuse_time_us: u64,
    /// Number of pool expansions
    pub pool_expansions: usize,
    /// Number of pool contractions
    pub pool_contractions: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Memory allocation rate (bytes per second)
    pub allocation_rate: f64,
    /// Memory deallocation rate (bytes per second)
    pub deallocation_rate: f64,
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record animation creation
    pub fn record_creation(&mut self, time_us: u64) {
        self.total_animations_created += 1;
        self.total_creation_time_us += time_us;
        self.avg_creation_time_us = self.total_creation_time_us / self.total_animations_created as u64;
    }

    /// Record animation reuse
    pub fn record_reuse(&mut self, time_us: u64) {
        self.total_animations_reused += 1;
        self.total_reuse_time_us += time_us;
        self.avg_reuse_time_us = self.total_reuse_time_us / self.total_animations_reused as u64;
    }

    /// Record pool expansion
    pub fn record_expansion(&mut self) {
        self.pool_expansions += 1;
    }

    /// Record pool contraction
    pub fn record_contraction(&mut self) {
        self.pool_contractions += 1;
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&mut self, hits: usize, total: usize) {
        if total > 0 {
            self.cache_hit_rate = hits as f64 / total as f64;
        }
    }

    /// Update allocation rates
    pub fn update_allocation_rates(&mut self, allocated: usize, deallocated: usize, time_seconds: f64) {
        if time_seconds > 0.0 {
            self.allocation_rate = allocated as f64 / time_seconds;
            self.deallocation_rate = deallocated as f64 / time_seconds;
        }
    }

    /// Get reuse efficiency (reused vs created)
    pub fn reuse_efficiency(&self) -> f64 {
        let total = self.total_animations_created + self.total_animations_reused;
        if total > 0 {
            (self.total_animations_reused as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get performance score (0.0 to 1.0)
    pub fn performance_score(&self) -> f64 {
        let reuse_efficiency = self.reuse_efficiency() / 100.0;
        let cache_efficiency = self.cache_hit_rate;
        let time_efficiency = if self.avg_reuse_time_us > 0 && self.avg_creation_time_us > 0 {
            (self.avg_creation_time_us as f64 / self.avg_reuse_time_us as f64).min(1.0)
        } else {
            0.0
        };

        (reuse_efficiency + cache_efficiency + time_efficiency) / 3.0
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
