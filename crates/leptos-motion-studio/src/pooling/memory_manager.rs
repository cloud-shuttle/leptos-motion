//! Memory manager for pooling

use super::*;
use crate::{Result, StudioError};

/// Memory manager for the animation pool
pub struct MemoryManager {
    /// Pool configuration
    config: PoolConfig,
    /// Memory statistics
    memory_stats: MemoryStats,
    /// Performance metrics
    performance_metrics: PerformanceMetrics,
    /// Memory pressure level
    memory_pressure: MemoryPressure,
    /// Last cleanup time
    last_cleanup: std::time::Instant,
}

/// Memory pressure levels
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryPressure {
    /// Low memory pressure
    Low,
    /// Medium memory pressure
    Medium,
    /// High memory pressure
    High,
    /// Critical memory pressure
    Critical,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            memory_stats: MemoryStats::new(),
            performance_metrics: PerformanceMetrics::new(),
            memory_pressure: MemoryPressure::Low,
            last_cleanup: std::time::Instant::now(),
        }
    }

    /// Update memory statistics
    pub fn update_stats(&mut self, allocated: usize, in_use: usize, active: usize, pooled: usize) {
        self.memory_stats.update(allocated, in_use, active, pooled);
        self.update_memory_pressure();
    }

    /// Update memory pressure based on current usage
    fn update_memory_pressure(&mut self) {
        let usage_percent = self.memory_stats.usage_percentage();
        
        self.memory_pressure = if usage_percent >= 90.0 {
            MemoryPressure::Critical
        } else if usage_percent >= 75.0 {
            MemoryPressure::High
        } else if usage_percent >= 50.0 {
            MemoryPressure::Medium
        } else {
            MemoryPressure::Low
        };
    }

    /// Check if cleanup is needed
    pub fn should_cleanup(&self) -> bool {
        let elapsed = self.last_cleanup.elapsed().as_secs_f64();
        elapsed >= self.config.cleanup_interval
    }

    /// Perform memory cleanup
    pub fn cleanup(&mut self) -> Result<()> {
        self.last_cleanup = std::time::Instant::now();
        
        match self.memory_pressure {
            MemoryPressure::Critical => self.aggressive_cleanup(),
            MemoryPressure::High => self.moderate_cleanup(),
            MemoryPressure::Medium => self.light_cleanup(),
            MemoryPressure::Low => Ok(()),
        }
    }

    /// Light cleanup - remove unused animations
    fn light_cleanup(&mut self) -> Result<()> {
        // Remove 10% of unused animations
        let target_reduction = (self.memory_stats.pooled_animations as f32 * 0.1) as usize;
        self.memory_stats.pooled_animations = self.memory_stats.pooled_animations.saturating_sub(target_reduction);
        Ok(())
    }

    /// Moderate cleanup - remove more unused animations
    fn moderate_cleanup(&mut self) -> Result<()> {
        // Remove 25% of unused animations
        let target_reduction = (self.memory_stats.pooled_animations as f32 * 0.25) as usize;
        self.memory_stats.pooled_animations = self.memory_stats.pooled_animations.saturating_sub(target_reduction);
        Ok(())
    }

    /// Aggressive cleanup - remove most unused animations
    fn aggressive_cleanup(&mut self) -> Result<()> {
        // Remove 50% of unused animations
        let target_reduction = (self.memory_stats.pooled_animations as f32 * 0.5) as usize;
        self.memory_stats.pooled_animations = self.memory_stats.pooled_animations.saturating_sub(target_reduction);
        Ok(())
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> &MemoryStats {
        &self.memory_stats
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// Get memory pressure level
    pub fn memory_pressure(&self) -> &MemoryPressure {
        &self.memory_pressure
    }

    /// Check if memory usage is within limits
    pub fn is_memory_usage_acceptable(&self) -> bool {
        self.memory_stats.total_allocated <= self.config.max_memory_bytes
    }

    /// Get memory usage percentage
    pub fn memory_usage_percentage(&self) -> f64 {
        if self.config.max_memory_bytes > 0 {
            (self.memory_stats.total_allocated as f64 / self.config.max_memory_bytes as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get memory recommendations
    pub fn get_memory_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.memory_usage_percentage() > 80.0 {
            recommendations.push("Consider reducing pool size or increasing memory limit".to_string());
        }

        if self.memory_stats.fragmentation_percent > 50.0 {
            recommendations.push("High memory fragmentation detected - consider cleanup".to_string());
        }

        if self.performance_metrics.cache_hit_rate < 0.5 {
            recommendations.push("Low cache hit rate - consider increasing pool size".to_string());
        }

        if self.memory_stats.efficiency() < 30.0 {
            recommendations.push("Low memory efficiency - consider optimizing animation usage".to_string());
        }

        recommendations
    }
}
