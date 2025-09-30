//! Integrated Memory Manager
//!
//! This module provides an integrated memory management system that works
//! seamlessly with the optimized animation manager and performance monitoring.

use crate::animation_trait::{Animation, AnimationResult};
use crate::memory_management::{
    AutoMemoryManager, MemoryStats, MemoryPressure, GCStrategy
};
use crate::optimized_animation_manager::OptimizedAnimationManager;
use std::time::{Duration, Instant};

/// Integrated memory and performance manager
pub struct IntegratedMemoryManager {
    /// Core memory manager
    memory_manager: AutoMemoryManager,
    /// Performance-optimized animation manager
    animation_manager: OptimizedAnimationManager,
    /// Memory pressure monitoring
    last_pressure_check: Instant,
    pressure_check_interval: Duration,
    /// Performance monitoring
    last_performance_check: Instant,
    performance_check_interval: Duration,
    /// Memory optimization settings
    settings: MemoryOptimizationSettings,
}

/// Memory optimization settings
#[derive(Debug, Clone)]
pub struct MemoryOptimizationSettings {
    /// Enable automatic memory cleanup
    pub auto_cleanup_enabled: bool,
    /// Memory pressure threshold for cleanup (0.0 to 1.0)
    pub memory_pressure_threshold: f64,
    /// Maximum animations before forced cleanup
    pub max_animations: usize,
    /// Garbage collection interval
    pub gc_interval: Duration,
    /// Cache size limit
    pub max_cache_size: usize,
    /// Target pool size limit
    pub max_target_pool_size: usize,
    /// Enable memory leak detection
    pub leak_detection_enabled: bool,
    /// Memory usage reporting interval
    pub reporting_interval: Duration,
}

impl Default for MemoryOptimizationSettings {
    fn default() -> Self {
        Self {
            auto_cleanup_enabled: true,
            memory_pressure_threshold: 0.8,
            max_animations: 1000,
            gc_interval: Duration::from_secs(5),
            max_cache_size: 1000,
            max_target_pool_size: 500,
            leak_detection_enabled: true,
            reporting_interval: Duration::from_secs(10),
        }
    }
}

impl IntegratedMemoryManager {
    /// Create a new integrated memory manager
    pub fn new() -> Self {
        Self::with_settings(MemoryOptimizationSettings::default())
    }

    /// Create a new integrated memory manager with custom settings
    pub fn with_settings(settings: MemoryOptimizationSettings) -> Self {
        Self {
            memory_manager: AutoMemoryManager::new(),
            animation_manager: OptimizedAnimationManager::new(),
            last_pressure_check: Instant::now(),
            pressure_check_interval: Duration::from_secs(2),
            last_performance_check: Instant::now(),
            performance_check_interval: Duration::from_secs(5),
            settings,
        }
    }

    /// Register an animation with integrated memory management
    pub fn register_animation(&mut self, animation: Box<dyn Animation>) -> AnimationResult<String> {
        let _id = animation.id().to_string();
        
        // Check memory pressure before registering
        let pressure = self.memory_manager.get_memory_pressure();
        if pressure == MemoryPressure::Critical {
            // Force cleanup before registering new animation
            self.memory_manager.force_cleanup()?;
        }

        // Register with animation manager
        let animation_id = self.animation_manager.register_optimized(animation)?;
        
        // Get the animation back for memory tracking
        if let Some(animation_rc) = self.animation_manager.get_animation(&animation_id) {
            // Register with memory manager
            self.memory_manager.register_animation(animation_id.clone(), animation_rc);
        }

        Ok(animation_id)
    }

    /// Update all systems (call this regularly)
    pub fn update(&mut self, delta_time: f64) -> AnimationResult<()> {
        // Update animation manager
        self.animation_manager.update_optimized(delta_time)?;

        // Update memory manager
        self.memory_manager.update()?;

        // Check memory pressure periodically
        self.check_memory_pressure()?;

        // Check performance periodically
        self.check_performance()?;

        // Apply memory optimizations
        self.apply_memory_optimizations()?;

        Ok(())
    }

    /// Check memory pressure and trigger cleanup if needed
    fn check_memory_pressure(&mut self) -> AnimationResult<()> {
        let now = Instant::now();
        if now.duration_since(self.last_pressure_check) >= self.pressure_check_interval {
            self.last_pressure_check = now;

            let pressure = self.memory_manager.get_memory_pressure();
            let stats = self.memory_manager.get_memory_stats();

            // Trigger cleanup based on pressure level
            match pressure {
                MemoryPressure::Low => {
                    // No action needed
                }
                MemoryPressure::Medium => {
                    // Conservative cleanup
                    self.memory_manager.garbage_collect(GCStrategy::Conservative)?;
                }
                MemoryPressure::High => {
                    // Aggressive cleanup
                    self.memory_manager.garbage_collect(GCStrategy::Aggressive)?;
                    self.optimize_caches();
                }
                MemoryPressure::Critical => {
                    // Emergency cleanup
                    self.memory_manager.garbage_collect(GCStrategy::Emergency)?;
                    self.emergency_optimization()?;
                }
            }

            // Check for memory leaks
            if self.settings.leak_detection_enabled && stats.leaked_animations > 0 {
                eprintln!("Warning: {} leaked animations detected", stats.leaked_animations);
            }
        }

        Ok(())
    }

    /// Check performance and adjust settings
    fn check_performance(&mut self) -> AnimationResult<()> {
        let now = Instant::now();
        if now.duration_since(self.last_performance_check) >= self.performance_check_interval {
            self.last_performance_check = now;

            let stats = self.animation_manager.get_performance_stats();
            let memory_stats = self.memory_manager.get_memory_stats();

            // Adjust settings based on performance
            if stats.memory_usage_estimate > 10_000_000 { // 10MB
                // High memory usage - reduce cache sizes
                self.reduce_cache_sizes();
            }

            if stats.cache_hit_rate < 0.5 {
                // Low cache hit rate - clear caches
                self.animation_manager.clear_caches();
            }

            // Log performance warnings
            if memory_stats.memory_pressure > 0.9 {
                eprintln!("Warning: High memory pressure detected: {:.2}%", 
                         memory_stats.memory_pressure * 100.0);
            }
        }

        Ok(())
    }

    /// Apply memory optimizations
    fn apply_memory_optimizations(&mut self) -> AnimationResult<()> {
        let stats = self.memory_manager.get_memory_stats();

        // Optimize based on current state
        if stats.active_animations > self.settings.max_animations {
            // Too many animations - force cleanup
            self.memory_manager.force_cleanup()?;
        }

        if stats.memory_pressure > self.settings.memory_pressure_threshold {
            // High memory pressure - optimize caches
            self.optimize_caches();
        }

        Ok(())
    }

    /// Optimize caches for memory efficiency
    fn optimize_caches(&mut self) {
        // Clear animation value cache if it's too large
        // Note: This would require exposing cache management methods
        // For now, we'll just clear all caches
        self.animation_manager.clear_caches();
    }

    /// Reduce cache sizes for memory efficiency
    fn reduce_cache_sizes(&mut self) {
        // This would require modifying the cache implementations
        // For now, we'll clear caches as a simple optimization
        self.animation_manager.clear_caches();
    }

    /// Emergency optimization when memory pressure is critical
    fn emergency_optimization(&mut self) -> AnimationResult<()> {
        // Clear all caches
        self.animation_manager.clear_caches();

        // Force cleanup of all completed animations
        self.memory_manager.force_cleanup()?;

        // Reduce animation limits temporarily
        let original_max = self.settings.max_animations;
        self.settings.max_animations = 100; // Reduce to 100

        // Restore original limit after a delay
        // Note: In a real implementation, you'd use a timer or async task
        std::thread::sleep(Duration::from_millis(100));
        self.settings.max_animations = original_max;

        Ok(())
    }

    /// Get comprehensive memory and performance statistics
    pub fn get_comprehensive_stats(&mut self) -> ComprehensiveStats {
        let memory_stats = self.memory_manager.get_memory_stats();
        let performance_stats = self.animation_manager.get_performance_stats();

        ComprehensiveStats {
            memory: memory_stats,
            performance: performance_stats,
            memory_pressure: self.memory_manager.get_memory_pressure(),
            optimization_settings: self.settings.clone(),
        }
    }

    /// Get memory report
    pub fn get_memory_report(&mut self) -> String {
        self.memory_manager.get_memory_report()
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> String {
        self.animation_manager.get_detailed_report()
    }

    /// Get comprehensive report
    pub fn get_comprehensive_report(&mut self) -> String {
        let stats = self.get_comprehensive_stats();
        
        format!(
            "=== COMPREHENSIVE ANIMATION SYSTEM REPORT ===\n\n\
            MEMORY MANAGEMENT:\n{}\n\n\
            PERFORMANCE OPTIMIZATION:\n{}\n\n\
            SYSTEM STATUS:\n\
            Memory Pressure: {:?}\n\
            Auto Cleanup: {}\n\
            Leak Detection: {}\n\
            Max Animations: {}\n\
            GC Interval: {:?}\n\
            Cache Size Limit: {}\n\
            Target Pool Limit: {}",
            "Memory report available", // Simplified for now
            self.get_performance_report(),
            stats.memory_pressure,
            stats.optimization_settings.auto_cleanup_enabled,
            stats.optimization_settings.leak_detection_enabled,
            stats.optimization_settings.max_animations,
            stats.optimization_settings.gc_interval,
            stats.optimization_settings.max_cache_size,
            stats.optimization_settings.max_target_pool_size
        )
    }

    /// Force cleanup of all systems
    pub fn force_cleanup(&mut self) -> AnimationResult<()> {
        // Stop all animations
        self.animation_manager.stop_all()?;

        // Mark all animations as completed in memory manager
        let animation_ids = self.animation_manager.get_animation_ids();
        for id in animation_ids {
            self.memory_manager.mark_completed(id);
        }

        // Force memory cleanup
        self.memory_manager.force_cleanup()?;

        // Clear all caches
        self.animation_manager.clear_caches();

        Ok(())
    }

    /// Update optimization settings
    pub fn update_settings(&mut self, settings: MemoryOptimizationSettings) {
        self.settings = settings;
        
        // Apply new settings to memory manager
        self.memory_manager.set_gc_interval(self.settings.gc_interval);
        self.memory_manager.set_memory_pressure_threshold(self.settings.memory_pressure_threshold);
        self.memory_manager.set_max_animations(self.settings.max_animations);
    }

    /// Get current settings
    pub fn get_settings(&self) -> &MemoryOptimizationSettings {
        &self.settings
    }

    /// Enable/disable auto cleanup
    pub fn set_auto_cleanup(&mut self, enabled: bool) {
        self.settings.auto_cleanup_enabled = enabled;
        if enabled {
            self.memory_manager.enable_auto_cleanup();
        } else {
            self.memory_manager.disable_auto_cleanup();
        }
    }

    /// Get animation manager reference for direct access
    pub fn animation_manager(&self) -> &OptimizedAnimationManager {
        &self.animation_manager
    }

    /// Get mutable animation manager reference for direct access
    pub fn animation_manager_mut(&mut self) -> &mut OptimizedAnimationManager {
        &mut self.animation_manager
    }
}

impl Default for IntegratedMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive statistics combining memory and performance data
#[derive(Debug, Clone)]
pub struct ComprehensiveStats {
    pub memory: MemoryStats,
    pub performance: crate::optimized_animation_manager::OptimizedAnimationStats,
    pub memory_pressure: MemoryPressure,
    pub optimization_settings: MemoryOptimizationSettings,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation_trait::Animation;
    use std::time::Duration;

    // Mock Animation for testing
    struct MockAnimation {
        id: String,
        duration: f64,
        progress: f64,
        is_running: bool,
        is_complete: bool,
    }

    impl MockAnimation {
        fn new(id: String, duration: f64) -> Self {
            Self {
                id,
                duration,
                progress: 0.0,
                is_running: false,
                is_complete: false,
            }
        }
    }

    impl Animation for MockAnimation {
        fn id(&self) -> &str {
            &self.id
        }

        fn start(&mut self) -> Result<(), crate::AnimationError> {
            self.is_running = true;
            self.is_complete = false;
            Ok(())
        }

        fn stop(&mut self) -> Result<(), crate::AnimationError> {
            self.is_running = false;
            Ok(())
        }

        fn is_complete(&self) -> bool {
            self.is_complete
        }

        fn progress(&self) -> f64 {
            self.progress
        }

        fn update(&mut self, delta_time: f64) -> Result<(), crate::AnimationError> {
            if self.is_running {
                self.progress += delta_time / self.duration;
                if self.progress >= 1.0 {
                    self.progress = 1.0;
                    self.is_running = false;
                    self.is_complete = true;
                }
            }
            Ok(())
        }

        fn duration(&self) -> f64 {
            self.duration
        }

        fn is_running(&self) -> bool {
            self.is_running
        }
    }

    #[test]
    fn test_integrated_memory_manager_creation() {
        let mut manager = IntegratedMemoryManager::new();
        let stats = manager.get_comprehensive_stats();
        
        assert_eq!(stats.memory.active_animations, 0);
        assert_eq!(stats.performance.total_animations, 0);
        assert!(stats.optimization_settings.auto_cleanup_enabled);
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_animation_registration() {
        let mut manager = IntegratedMemoryManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let id = manager.register_animation(animation).unwrap();
        assert_eq!(id, "test");
        
        let stats = manager.get_comprehensive_stats();
        assert_eq!(stats.memory.active_animations, 1);
    }

    #[test]
    fn test_memory_pressure_checking() {
        let mut manager = IntegratedMemoryManager::new();
        
        // Add many animations to trigger memory pressure
        for i in 0..10 {
            let animation = Box::new(MockAnimation::new(format!("test_{}", i), 1.0));
            let _ = manager.register_animation(animation);
        }
        
        // Update should trigger memory pressure checking
        manager.update(0.1).unwrap();
        
        let stats = manager.get_comprehensive_stats();
        assert!(stats.memory.active_animations > 0);
    }

    #[test]
    fn test_comprehensive_report() {
        let mut manager = IntegratedMemoryManager::new();
        let report = manager.get_comprehensive_report();
        
        assert!(report.contains("COMPREHENSIVE ANIMATION SYSTEM REPORT"));
        assert!(report.contains("MEMORY MANAGEMENT"));
        assert!(report.contains("PERFORMANCE OPTIMIZATION"));
        assert!(report.contains("SYSTEM STATUS"));
    }

    #[test]
    fn test_settings_update() {
        let mut manager = IntegratedMemoryManager::new();
        let mut settings = MemoryOptimizationSettings::default();
        settings.max_animations = 500;
        settings.auto_cleanup_enabled = false;
        
        manager.update_settings(settings);
        
        let current_settings = manager.get_settings();
        assert_eq!(current_settings.max_animations, 500);
        assert!(!current_settings.auto_cleanup_enabled);
    }

    #[test]
    fn test_force_cleanup() {
        let mut manager = IntegratedMemoryManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let _ = manager.register_animation(animation);
        manager.force_cleanup().unwrap();
        
        let stats = manager.get_comprehensive_stats();
        assert_eq!(stats.memory.active_animations, 0);
    }

    #[test]
    fn test_auto_cleanup_toggle() {
        let mut manager = IntegratedMemoryManager::new();
        
        // Disable auto cleanup
        manager.set_auto_cleanup(false);
        let settings = manager.get_settings();
        assert!(!settings.auto_cleanup_enabled);
        
        // Enable auto cleanup
        manager.set_auto_cleanup(true);
        let settings = manager.get_settings();
        assert!(settings.auto_cleanup_enabled);
    }
}
