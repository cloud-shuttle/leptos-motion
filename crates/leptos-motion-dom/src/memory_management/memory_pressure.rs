//! Memory pressure monitoring and management

use super::memory_stats::{MemoryStats, MemoryPressure};
use std::time::{Duration, Instant};

/// Memory pressure monitor
pub struct MemoryPressureMonitor {
    /// Current memory pressure level
    current_pressure: MemoryPressure,
    /// Pressure history for trend analysis
    pressure_history: Vec<(Instant, MemoryPressure)>,
    /// Maximum history size
    max_history_size: usize,
    /// Pressure change threshold
    pressure_change_threshold: f64,
    /// Last pressure check time
    last_check_time: Instant,
    /// Check interval
    check_interval: Duration,
}

impl MemoryPressureMonitor {
    /// Create a new memory pressure monitor
    pub fn new() -> Self {
        Self {
            current_pressure: MemoryPressure::Low,
            pressure_history: Vec::new(),
            max_history_size: 100,
            pressure_change_threshold: 0.1,
            last_check_time: Instant::now(),
            check_interval: Duration::from_secs(1),
        }
    }

    /// Update memory pressure based on current statistics
    pub fn update_pressure(&mut self, stats: &MemoryStats, max_animations: usize) -> MemoryPressure {
        // Only update if enough time has passed
        if self.last_check_time.elapsed() < self.check_interval {
            return self.current_pressure;
        }

        let new_pressure = self.calculate_pressure(stats, max_animations);
        
        // Record pressure change
        self.record_pressure_change(new_pressure);
        
        // Update current pressure
        self.current_pressure = new_pressure;
        self.last_check_time = Instant::now();
        
        new_pressure
    }

    /// Calculate memory pressure based on statistics
    fn calculate_pressure(&self, stats: &MemoryStats, max_animations: usize) -> MemoryPressure {
        let total_animations = stats.active_animations + stats.completed_animations;
        
        if total_animations == 0 {
            return MemoryPressure::Low;
        }

        let pressure_ratio = total_animations as f64 / max_animations as f64;
        let memory_ratio = if stats.total_allocated > 0 {
            stats.memory_in_use as f64 / stats.total_allocated as f64
        } else {
            0.0
        };

        // Combine animation count pressure and memory usage pressure
        let combined_pressure = (pressure_ratio * 0.7 + memory_ratio * 0.3).min(1.0);

        if combined_pressure >= 0.9 {
            MemoryPressure::Critical
        } else if combined_pressure >= 0.7 {
            MemoryPressure::High
        } else if combined_pressure >= 0.5 {
            MemoryPressure::Medium
        } else {
            MemoryPressure::Low
        }
    }

    /// Record a pressure change in history
    fn record_pressure_change(&mut self, pressure: MemoryPressure) {
        let now = Instant::now();
        self.pressure_history.push((now, pressure));
        
        // Remove old entries if history is too large
        if self.pressure_history.len() > self.max_history_size {
            self.pressure_history.remove(0);
        }
    }

    /// Get current memory pressure
    pub fn get_current_pressure(&self) -> MemoryPressure {
        self.current_pressure
    }

    /// Get pressure trend over time
    pub fn get_pressure_trend(&self, duration: Duration) -> PressureTrend {
        let cutoff_time = Instant::now() - duration;
        let recent_pressures: Vec<MemoryPressure> = self.pressure_history
            .iter()
            .filter(|(time, _)| *time >= cutoff_time)
            .map(|(_, pressure)| *pressure)
            .collect();

        if recent_pressures.is_empty() {
            return PressureTrend::Stable;
        }

        let low_count = recent_pressures.iter().filter(|&&p| p == MemoryPressure::Low).count();
        let medium_count = recent_pressures.iter().filter(|&&p| p == MemoryPressure::Medium).count();
        let high_count = recent_pressures.iter().filter(|&&p| p == MemoryPressure::High).count();
        let critical_count = recent_pressures.iter().filter(|&&p| p == MemoryPressure::Critical).count();

        let total = recent_pressures.len();
        let low_ratio = low_count as f64 / total as f64;
        let medium_ratio = medium_count as f64 / total as f64;
        let high_ratio = high_count as f64 / total as f64;
        let critical_ratio = critical_count as f64 / total as f64;

        // Determine trend based on ratios
        if critical_ratio > 0.3 {
            PressureTrend::Critical
        } else if high_ratio > 0.5 {
            PressureTrend::Increasing
        } else if medium_ratio > 0.5 {
            PressureTrend::Stable
        } else if low_ratio > 0.7 {
            PressureTrend::Decreasing
        } else {
            PressureTrend::Stable
        }
    }

    /// Check if pressure is increasing
    pub fn is_pressure_increasing(&self) -> bool {
        matches!(self.get_pressure_trend(Duration::from_secs(30)), PressureTrend::Increasing)
    }

    /// Check if pressure is critical
    pub fn is_pressure_critical(&self) -> bool {
        matches!(self.current_pressure, MemoryPressure::Critical)
    }

    /// Get pressure history
    pub fn get_pressure_history(&self) -> &[(Instant, MemoryPressure)] {
        &self.pressure_history
    }

    /// Clear pressure history
    pub fn clear_history(&mut self) {
        self.pressure_history.clear();
    }

    /// Set maximum history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        
        // Trim history if necessary
        if self.pressure_history.len() > size {
            let excess = self.pressure_history.len() - size;
            self.pressure_history.drain(0..excess);
        }
    }

    /// Set pressure change threshold
    pub fn set_pressure_change_threshold(&mut self, threshold: f64) {
        self.pressure_change_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set check interval
    pub fn set_check_interval(&mut self, interval: Duration) {
        self.check_interval = interval;
    }

    /// Get pressure statistics
    pub fn get_pressure_stats(&self) -> PressureStats {
        let total_checks = self.pressure_history.len();
        if total_checks == 0 {
            return PressureStats::default();
        }

        let low_count = self.pressure_history.iter().filter(|(_, p)| *p == MemoryPressure::Low).count();
        let medium_count = self.pressure_history.iter().filter(|(_, p)| *p == MemoryPressure::Medium).count();
        let high_count = self.pressure_history.iter().filter(|(_, p)| *p == MemoryPressure::High).count();
        let critical_count = self.pressure_history.iter().filter(|(_, p)| *p == MemoryPressure::Critical).count();

        PressureStats {
            total_checks,
            low_percentage: (low_count as f64 / total_checks as f64) * 100.0,
            medium_percentage: (medium_count as f64 / total_checks as f64) * 100.0,
            high_percentage: (high_count as f64 / total_checks as f64) * 100.0,
            critical_percentage: (critical_count as f64 / total_checks as f64) * 100.0,
            current_pressure: self.current_pressure,
            trend: self.get_pressure_trend(Duration::from_secs(60)),
        }
    }
}

impl Default for MemoryPressureMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Pressure trend over time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureTrend {
    /// Pressure is decreasing
    Decreasing,
    /// Pressure is stable
    Stable,
    /// Pressure is increasing
    Increasing,
    /// Pressure is critical
    Critical,
}

/// Pressure statistics
#[derive(Debug, Clone)]
pub struct PressureStats {
    /// Total number of pressure checks
    pub total_checks: usize,
    /// Percentage of time at low pressure
    pub low_percentage: f64,
    /// Percentage of time at medium pressure
    pub medium_percentage: f64,
    /// Percentage of time at high pressure
    pub high_percentage: f64,
    /// Percentage of time at critical pressure
    pub critical_percentage: f64,
    /// Current pressure level
    pub current_pressure: MemoryPressure,
    /// Current trend
    pub trend: PressureTrend,
}

impl Default for PressureStats {
    fn default() -> Self {
        Self {
            total_checks: 0,
            low_percentage: 0.0,
            medium_percentage: 0.0,
            high_percentage: 0.0,
            critical_percentage: 0.0,
            current_pressure: MemoryPressure::Low,
            trend: PressureTrend::Stable,
        }
    }
}
