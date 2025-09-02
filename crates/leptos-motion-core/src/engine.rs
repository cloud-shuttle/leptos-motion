//! Animation engine traits and implementations

use crate::{AnimationHandle, AnimationError, Result, AnimationTarget, Transition};
use crate::performance::{PerformanceMonitor, AnimationScheduler, GPULayerManager, AnimationPool, PerformanceBudget, AnimationPriority};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, Performance};

/// Core animation engine trait
pub trait AnimationEngine {
    /// Check if this engine is available in current environment
    fn is_available(&self) -> bool;
    
    /// Start an animation and return a handle
    fn animate(&mut self, animation: &AnimationConfig) -> Result<AnimationHandle>;
    
    /// Stop an animation by handle
    fn stop(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Pause an animation
    fn pause(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Resume a paused animation
    fn resume(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Update all animations (for RAF-based engines)
    fn tick(&mut self, timestamp: f64) -> Result<()>;
    
    /// Get current playback state
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState>;
    
    /// Check if an animation is running
    fn is_running(&self, handle: AnimationHandle) -> bool;
    
    /// Get performance metrics
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport>;
}

/// Animation configuration
#[derive(Clone)]
pub struct AnimationConfig {
    /// Target element
    pub element: Element,
    /// Animation properties
    pub from: AnimationTarget,
    /// Target values
    pub to: AnimationTarget,
    /// Transition configuration
    pub transition: Transition,
    /// Completion callback ID (for now, simplified)
    pub on_complete_id: Option<u64>,
    /// Update callback ID (for now, simplified)
    pub on_update_id: Option<u64>,
}

impl std::fmt::Debug for AnimationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimationConfig")
            .field("element", &"<Element>")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("transition", &self.transition)
            .field("on_complete", &if self.on_complete_id.is_some() { "Some(<callback>)" } else { "None" })
            .field("on_update", &if self.on_update_id.is_some() { "Some(<callback>)" } else { "None" })
            .finish()
    }
}

/// Animation playback state
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    /// Animation is running
    Running,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
    /// Animation was cancelled
    Cancelled,
}

/// Optimized hybrid animation engine with performance monitoring
pub struct OptimizedHybridEngine {
    waapi_engine: WaapiEngine,
    raf_engine: RafEngine,
    feature_detector: FeatureDetector,
    performance_monitor: Option<PerformanceMonitor>,
    scheduler: AnimationScheduler,
    gpu_manager: GPULayerManager,
    animation_pool: AnimationPool<RafAnimation>,
    current_handle: u64,
    frame_count: u64,
}

impl OptimizedHybridEngine {
    /// Create a new optimized hybrid engine
    pub fn new() -> Self {
        let budget = PerformanceBudget::default();
        let frame_budget = std::time::Duration::from_millis(16); // 60fps target
        
        Self {
            waapi_engine: WaapiEngine::new(),
            raf_engine: RafEngine::new(),
            feature_detector: FeatureDetector::new(),
            performance_monitor: PerformanceMonitor::new(budget),
            scheduler: AnimationScheduler::new(frame_budget),
            gpu_manager: GPULayerManager::new(50), // Max 50 GPU layers
            animation_pool: AnimationPool::new(),
            current_handle: 0,
            frame_count: 0,
        }
    }
    
    /// Start performance monitoring
    pub fn start_performance_monitoring(&mut self) {
        if let Some(monitor) = &mut self.performance_monitor {
            monitor.start_frame();
        }
    }
    
    /// End performance monitoring
    pub fn end_performance_monitoring(&mut self, animations_updated: usize) {
        if let Some(monitor) = &mut self.performance_monitor {
            let memory_usage = self.animation_pool.active_count() * 1024; // Rough estimate
            monitor.end_frame(animations_updated, memory_usage);
        }
    }
    
    /// Get performance report
    pub fn get_performance_report(&self) -> Option<crate::performance::PerformanceReport> {
        self.performance_monitor.as_ref().map(|m| m.get_report())
    }
    
    /// Optimize element for GPU acceleration
    pub fn optimize_for_gpu(&mut self, element: &Element) -> bool {
        // For now, skip GPU optimization to avoid compilation issues
        // In a real implementation, this would check element attributes
        false
    }
    
    /// Select the appropriate engine for an animation
    fn select_engine(&self, config: &AnimationConfig) -> EngineChoice {
        if self.feature_detector.supports_waapi() && 
           self.feature_detector.can_use_waapi_for(config) {
            EngineChoice::Waapi
        } else {
            EngineChoice::Raf
        }
    }
    
    /// Get next animation handle
    fn next_handle(&mut self) -> AnimationHandle {
        self.current_handle += 1;
        AnimationHandle(self.current_handle)
    }
}

impl AnimationEngine for OptimizedHybridEngine {
    fn is_available(&self) -> bool {
        self.waapi_engine.is_available() || self.raf_engine.is_available()
    }
    
    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        let handle = self.next_handle();
        
        // Start performance monitoring
        self.start_performance_monitoring();
        
        // Optimize element for GPU if possible
        self.optimize_for_gpu(&config.element);
        
        // Select engine based on performance and capabilities
        let engine_choice = self.select_engine(config);
        
        let result = match engine_choice {
            EngineChoice::Waapi => {
                self.waapi_engine.animate_with_handle(handle, config.clone())
            }
            EngineChoice::Raf => {
                // Create RAF animation directly
                let start_time = web_sys::window().unwrap().performance().unwrap().now();
                let animation = RafAnimation::new(config.clone(), start_time);
                self.raf_engine.animations.insert(handle, animation);
                
                // Start RAF loop if not already running
                if self.raf_engine.raf_handle.is_none() {
                    self.raf_engine.start_raf_loop()?;
                }
                
                Ok(())
            }
        };
        
        // End performance monitoring
        self.end_performance_monitoring(1);
        
        result?;
        Ok(handle)
    }
    
    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try WAAPI first
        if let Ok(()) = self.waapi_engine.stop(handle) {
            return Ok(());
        }
        
        // Try RAF
        if let Ok(()) = self.raf_engine.stop(handle) {
            // Return animation to pool
            if let Some(_animation) = self.animation_pool.release(handle) {
                // Animation returned to pool
            }
            return Ok(());
        }
        
        Err(AnimationError::NotFound { handle })
    }
    
    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try WAAPI first
        if let Ok(()) = self.waapi_engine.pause(handle) {
            return Ok(());
        }
        
        // Try RAF
        self.raf_engine.pause(handle)
    }
    
    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        // Try WAAPI first
        if let Ok(()) = self.waapi_engine.resume(handle) {
            return Ok(());
        }
        
        // Try RAF
        self.raf_engine.resume(handle)
    }
    
    fn tick(&mut self, timestamp: f64) -> Result<()> {
        self.frame_count += 1;
        
        // Start performance monitoring
        self.start_performance_monitoring();
        
        // Update RAF engine first
        let raf_result = self.raf_engine.tick(timestamp);
        
        // End performance monitoring
        self.end_performance_monitoring(self.raf_engine.animations.len());
        
        raf_result
    }
    
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        // Try WAAPI first
        if let Ok(state) = self.waapi_engine.get_state(handle) {
            return Ok(state);
        }
        
        // Try RAF
        self.raf_engine.get_state(handle)
    }
    
    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.waapi_engine.is_running(handle) || self.raf_engine.is_running(handle)
    }
    
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        self.get_performance_report()
    }
}

/// Web Animations API engine
pub struct WaapiEngine {
    animations: HashMap<AnimationHandle, web_sys::Animation>,
}

impl WaapiEngine {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
    
    pub fn animate_with_handle(&mut self, handle: AnimationHandle, _config: AnimationConfig) -> Result<()> {
        // Implementation would create Web Animation API animation
        // This is a simplified version
        self.animations.insert(handle, web_sys::Animation::new().unwrap_throw());
        Ok(())
    }
}

impl AnimationEngine for WaapiEngine {
    fn is_available(&self) -> bool {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("div").ok())
            .and_then(|e| js_sys::Reflect::has(&e, &"animate".into()).ok())
            .unwrap_or(false)
    }
    
    fn animate(&mut self, _config: &AnimationConfig) -> Result<AnimationHandle> {
        // Create Web Animation would go here
        Err(AnimationError::EngineUnavailable("WAAPI not implemented".to_string()))
    }
    
    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.remove(&handle) {
            animation.cancel();
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get(&handle) {
            let _ = animation.pause();
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get(&handle) {
            let _ = animation.play();
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn tick(&mut self, _timestamp: f64) -> Result<()> {
        // WAAPI doesn't need manual ticking
        Ok(())
    }
    
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if let Some(_animation) = self.animations.get(&handle) {
            // Would check animation.playState
            Ok(PlaybackState::Running)
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.animations.contains_key(&handle)
    }
    
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        None // WAAPI doesn't provide performance metrics
    }
}

/// RequestAnimationFrame-based engine
pub struct RafEngine {
    animations: HashMap<AnimationHandle, RafAnimation>,
    performance: Performance,
    raf_handle: Option<i32>,
}

impl RafEngine {
    pub fn new() -> Self {
        let performance = window()
            .and_then(|w| w.performance())
            .expect("Performance API not available");
            
        Self {
            animations: HashMap::new(),
            performance,
            raf_handle: None,
        }
    }
    
    pub fn animate_with_handle(&mut self, handle: AnimationHandle, config: AnimationConfig) -> Result<()> {
        let start_time = self.performance.now();
        let animation = RafAnimation::new(config, start_time);
        self.animations.insert(handle, animation);
        
        // Start RAF loop if not already running
        if self.raf_handle.is_none() {
            self.start_raf_loop()?;
        }
        
        Ok(())
    }
    
    fn start_raf_loop(&mut self) -> Result<()> {
        // This would set up the RAF callback
        // Simplified for now
        Ok(())
    }
}

impl AnimationEngine for RafEngine {
    fn is_available(&self) -> bool {
        window().is_some()
    }
    
    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        let handle = AnimationHandle(rand::random());
        self.animate_with_handle(handle, config.clone())?;
        Ok(handle)
    }
    
    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        if self.animations.remove(&handle).is_some() {
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get_mut(&handle) {
            animation.state = PlaybackState::Paused;
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get_mut(&handle) {
            animation.state = PlaybackState::Running;
            Ok(())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn tick(&mut self, timestamp: f64) -> Result<()> {
        let mut completed = Vec::new();
        
        for (&handle, animation) in self.animations.iter_mut() {
            if animation.state == PlaybackState::Running {
                animation.update(timestamp);
                
                if animation.is_complete() {
                    animation.state = PlaybackState::Completed;
                    completed.push(handle);
                }
            }
        }
        
        // Clean up completed animations
        for handle in completed {
            self.animations.remove(&handle);
        }
        
        Ok(())
    }
    
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if let Some(animation) = self.animations.get(&handle) {
            Ok(animation.state.clone())
        } else {
            Err(AnimationError::NotFound { handle })
        }
    }
    
    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.animations
            .get(&handle)
            .map(|a| a.state == PlaybackState::Running)
            .unwrap_or(false)
    }
    
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        None // RAF engine doesn't provide performance metrics
    }
}

/// RAF animation state
struct RafAnimation {
    config: AnimationConfig,
    start_time: f64,
    state: PlaybackState,
}

impl RafAnimation {
    fn new(config: AnimationConfig, start_time: f64) -> Self {
        Self {
            config,
            start_time,
            state: PlaybackState::Running,
        }
    }
    
    fn update(&mut self, timestamp: f64) {
        let elapsed = timestamp - self.start_time;
        let duration = self.config.transition.duration.unwrap_or(1.0) * 1000.0; // Convert to ms
        
        if elapsed >= duration {
            self.state = PlaybackState::Completed;
        }
        
        let progress = (elapsed / duration).clamp(0.0, 1.0);
        let eased_progress = self.config.transition.ease.evaluate(progress);
        
        // Apply animation values to element
        self.apply_values(eased_progress);
        
        if let Some(callback_id) = self.config.on_update_id {
            // Call callback by ID (simplified for now)
            // In a real implementation, this would look up the callback by ID
        }
    }
    
    fn apply_values(&self, _progress: f64) {
        // This would interpolate between from and to values
        // and apply them to the element's style
    }
    
    fn is_complete(&self) -> bool {
        matches!(self.state, PlaybackState::Completed | PlaybackState::Cancelled)
    }
}

/// Feature detection for animation capabilities
pub struct FeatureDetector {
    waapi_available: Option<bool>,
}

impl FeatureDetector {
    pub fn new() -> Self {
        Self {
            waapi_available: None,
        }
    }
    
    pub fn supports_waapi(&self) -> bool {
        // Cache the result since it won't change during runtime
        if let Some(available) = self.waapi_available {
            return available;
        }
        
        // Check for Web Animations API support
        let available = window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("div").ok())
            .and_then(|e| js_sys::Reflect::has(&e, &"animate".into()).ok())
            .unwrap_or(false);
            
        available
    }
    
    pub fn can_use_waapi_for(&self, config: &AnimationConfig) -> bool {
        // Check if the specific animation properties are supported by WAAPI
        for property in config.to.keys() {
            if !self.is_waapi_property(property) {
                return false;
            }
        }
        true
    }
    
    fn is_waapi_property(&self, property: &str) -> bool {
        // List of properties that are well-supported by WAAPI
        matches!(
            property,
            "opacity" | "transform" | "left" | "top" | "width" | "height" | 
            "background-color" | "color" | "border-radius" | "scale" | "rotate" | 
            "translateX" | "translateY" | "translateZ" | "rotateX" | "rotateY" | "rotateZ"
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum EngineChoice {
    Waapi,
    Raf,
}

impl Default for OptimizedHybridEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WaapiEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RafEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature_detector() {
        let detector = FeatureDetector::new();
        
        // WAAPI support varies by browser, so we just test the method exists
        let _supports_waapi = detector.supports_waapi();
    }
    
    #[test]
    fn test_playback_states() {
        let states = vec![
            PlaybackState::Running,
            PlaybackState::Paused,
            PlaybackState::Completed,
            PlaybackState::Cancelled,
        ];
        
        for state in states {
            // Just ensure the states can be cloned and compared
            let cloned = state.clone();
            assert_eq!(state, cloned);
        }
    }
}

// Temporary random function for handle generation
mod rand {
    pub fn random<T: From<u64>>() -> T {
        // Simple PRNG for demo purposes
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        web_sys::js_sys::Date::now().to_bits().hash(&mut hasher);
        T::from(hasher.finish())
    }
}