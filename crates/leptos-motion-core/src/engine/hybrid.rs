//! Optimized hybrid animation engine
//!
//! This engine intelligently chooses between WAAPI and RAF-based animations
//! based on browser support and animation requirements.

use super::traits::*;
use super::waapi::WaapiEngine;
use super::raf::RafEngine;
use super::feature_detector::FeatureDetector;
use crate::{AnimationError, AnimationHandle, Result};

#[cfg(feature = "performance-metrics")]
use crate::performance::{
    AnimationPool, AnimationScheduler, GPULayerManager, PerformanceBudget, PerformanceMonitor,
};

/// Engine choice for animations
#[derive(Debug, Clone, Copy, PartialEq)]
enum EngineChoice {
    /// Use WAAPI engine
    Waapi,
    /// Use RAF engine
    Raf,
}

/// Optimized hybrid engine that chooses the best animation method
pub struct OptimizedHybridEngine {
    #[cfg(feature = "web-sys")]
    waapi_engine: WaapiEngine,
    raf_engine: RafEngine,
    feature_detector: FeatureDetector,
    #[cfg(feature = "performance-metrics")]
    performance_monitor: Option<PerformanceMonitor>,
    #[cfg(feature = "performance-metrics")]
    _scheduler: AnimationScheduler,
    #[cfg(feature = "performance-metrics")]
    gpu_manager: GPULayerManager,
    #[cfg(feature = "performance-metrics")]
    animation_pool: AnimationPool,
    current_handle: u64,
    frame_count: u64,
}

impl OptimizedHybridEngine {
    /// Create a new optimized hybrid engine instance
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "web-sys")]
            waapi_engine: WaapiEngine::new(),
            raf_engine: RafEngine::new(),
            feature_detector: FeatureDetector::new(),
            #[cfg(feature = "performance-metrics")]
            performance_monitor: {
                let budget = PerformanceBudget::default();
                Some(PerformanceMonitor::new(budget))
            },
            #[cfg(feature = "performance-metrics")]
            _scheduler: AnimationScheduler::new(),
            #[cfg(feature = "performance-metrics")]
            gpu_manager: GPULayerManager::new(50), // Max 50 GPU layers
            #[cfg(feature = "performance-metrics")]
            animation_pool: AnimationPool::new(100),
            current_handle: 0,
            frame_count: 0,
        }
    }

    /// Start performance monitoring
    #[cfg(feature = "performance-metrics")]
    pub fn start_performance_monitoring(&mut self) {
        if let Some(monitor) = &mut self.performance_monitor {
            monitor.record_frame_timestamp(std::time::Instant::now());
        }
    }

    /// End performance monitoring
    #[cfg(feature = "performance-metrics")]
    pub fn end_performance_monitoring(&mut self, animations_updated: usize) {
        if let Some(monitor) = &mut self.performance_monitor {
            let memory_usage = self.animation_pool.in_use_count() * 1024; // Rough estimate
            let gpu_layers = self.gpu_manager.layer_count();
            let _report = monitor.generate_report(animations_updated, memory_usage, gpu_layers);
        }
    }

    /// Start performance monitoring (no-op when feature disabled)
    #[cfg(not(feature = "performance-metrics"))]
    pub fn start_performance_monitoring(&mut self) {
        // No-op when performance metrics are disabled
    }

    /// End performance monitoring (no-op when feature disabled)
    #[cfg(not(feature = "performance-metrics"))]
    pub fn end_performance_monitoring(&mut self, _animations_updated: usize) {
        // No-op when performance metrics are disabled
    }

    /// Get performance report
    #[cfg(feature = "performance-metrics")]
    pub fn get_performance_report(&self) -> Option<crate::performance::PerformanceReport> {
        // We can't generate a report here since we need mutable access
        // This is a limitation of the current design
        None
    }

    /// Get performance report (no-op when feature disabled)
    #[cfg(not(feature = "performance-metrics"))]
    pub fn get_performance_report(&self) -> Option<()> {
        None
    }

    /// Optimize element for GPU acceleration
    #[cfg(feature = "web-sys")]
    pub fn optimize_for_gpu(&mut self, _element: &web_sys::Element) -> bool {
        // For now, skip GPU optimization to avoid compilation issues
        // In a real implementation, this would check element attributes
        false
    }

    /// Select the appropriate engine for an animation
    fn select_engine(&self, config: &AnimationConfig) -> EngineChoice {
        if self.feature_detector.supports_waapi() && self.feature_detector.can_use_waapi_for(config)
        {
            EngineChoice::Waapi
        } else {
            EngineChoice::Raf
        }
    }

    /// Generate a new animation handle
    fn generate_handle(&mut self) -> AnimationHandle {
        self.current_handle += 1;
        AnimationHandle(self.current_handle)
    }
}

impl AnimationEngine for OptimizedHybridEngine {
    fn is_available(&self) -> bool {
        self.raf_engine.is_available() || 
        {
            #[cfg(feature = "web-sys")]
            {
                self.waapi_engine.is_available()
            }
            #[cfg(not(feature = "web-sys"))]
            {
                false
            }
        }
    }

    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        self.start_performance_monitoring();
        
        let _handle = self.generate_handle();
        let engine_choice = self.select_engine(config);
        
        let result = match engine_choice {
            EngineChoice::Waapi => {
                #[cfg(feature = "web-sys")]
                {
                    self.waapi_engine.animate(config)
                }
                #[cfg(not(feature = "web-sys"))]
                {
                    Err(AnimationError::InvalidValue("WAAPI not available".to_string()))
                }
            }
            EngineChoice::Raf => self.raf_engine.animate(config),
        };
        
        self.end_performance_monitoring(1);
        result
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try both engines since we don't track which one was used
        let raf_result = self.raf_engine.stop(handle);
        
        #[cfg(feature = "web-sys")]
        let waapi_result = self.waapi_engine.stop(handle);
        
        #[cfg(feature = "web-sys")]
        if raf_result.is_ok() || waapi_result.is_ok() {
            Ok(())
        } else {
            raf_result
        }
        
        #[cfg(not(feature = "web-sys"))]
        raf_result
    }

    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try both engines
        let raf_result = self.raf_engine.pause(handle);
        
        #[cfg(feature = "web-sys")]
        let waapi_result = self.waapi_engine.pause(handle);
        
        #[cfg(feature = "web-sys")]
        if raf_result.is_ok() || waapi_result.is_ok() {
            Ok(())
        } else {
            raf_result
        }
        
        #[cfg(not(feature = "web-sys"))]
        raf_result
    }

    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try both engines
        let raf_result = self.raf_engine.resume(handle);
        
        #[cfg(feature = "web-sys")]
        let waapi_result = self.waapi_engine.resume(handle);
        
        #[cfg(feature = "web-sys")]
        if raf_result.is_ok() || waapi_result.is_ok() {
            Ok(())
        } else {
            raf_result
        }
        
        #[cfg(not(feature = "web-sys"))]
        raf_result
    }

    fn tick(&mut self, timestamp: f64) -> Result<()> {
        self.frame_count += 1;
        self.start_performance_monitoring();
        
        let raf_result = self.raf_engine.tick(timestamp);
        
        self.end_performance_monitoring(1);
        raf_result
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        // Try RAF engine first
        match self.raf_engine.get_state(handle) {
            Ok(state) => Ok(state),
            Err(_) => {
                #[cfg(feature = "web-sys")]
                {
                    self.waapi_engine.get_state(handle)
                }
                #[cfg(not(feature = "web-sys"))]
                {
                    Err(AnimationError::NotFound)
                }
            }
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.raf_engine.is_running(handle) || 
        {
            #[cfg(feature = "web-sys")]
            {
                self.waapi_engine.is_running(handle)
            }
            #[cfg(not(feature = "web-sys"))]
            {
                false
            }
        }
    }

    #[cfg(feature = "performance-metrics")]
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        self.get_performance_report()
    }

    #[cfg(not(feature = "performance-metrics"))]
    fn get_performance_metrics(&self) -> Option<()> {
        self.get_performance_report()
    }
}
