//! Simplified Animation Engine API
//!
//! This module provides a simplified, user-friendly animation engine API
//! that hides implementation details and provides a clean interface.

#[cfg(feature = "performance-metrics")]
use crate::performance::PerformanceReport;
use crate::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Mock engine for non-WASM targets to avoid WASM static access issues
#[cfg(not(target_arch = "wasm32"))]
struct MockEngine {
    animations: HashMap<AnimationHandle, bool>,
    current_handle: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl MockEngine {
    fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_handle: 1,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AnimationEngine for MockEngine {
    fn is_available(&self) -> bool {
        true
    }

    fn animate(&mut self, _animation: &engine::AnimationConfig) -> Result<AnimationHandle> {
        let handle = AnimationHandle(self.current_handle);
        self.current_handle += 1;
        self.animations.insert(handle, true);
        Ok(handle)
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        self.animations.remove(&handle);
        Ok(())
    }

    fn pause(&mut self, _handle: AnimationHandle) -> Result<()> {
        Ok(())
    }

    fn resume(&mut self, _handle: AnimationHandle) -> Result<()> {
        Ok(())
    }

    fn tick(&mut self, _timestamp: f64) -> Result<()> {
        Ok(())
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if self.animations.contains_key(&handle) {
            Ok(PlaybackState::Running)
        } else {
            Ok(PlaybackState::Completed)
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.animations.contains_key(&handle)
    }

    #[cfg(feature = "performance-metrics")]
    fn get_performance_metrics(&self) -> Option<PerformanceReport> {
        None
    }

    #[cfg(not(feature = "performance-metrics"))]
    fn get_performance_metrics(&self) -> Option<()> {
        None
    }
}

/// Simplified animation engine that hides implementation details
///
/// This is the main public API for animation operations. It provides
/// a clean, simple interface while hiding the complexity of the
/// underlying hybrid engine implementation.
pub struct SimplifiedAnimationEngine {
    /// Internal engine (hidden from public API)
    /// Uses different implementations for WASM vs native targets
    #[cfg(target_arch = "wasm32")]
    internal_engine: Arc<Mutex<OptimizedHybridEngine>>,
    #[cfg(not(target_arch = "wasm32"))]
    internal_engine: Arc<Mutex<MockEngine>>,
    /// Performance metrics cache
    #[cfg(feature = "performance-metrics")]
    performance_cache: Arc<Mutex<Option<PerformanceReport>>>,
    /// Global state for batch operations
    global_state: Arc<Mutex<GlobalState>>,
}

/// Global state for batch operations
#[derive(Debug, Default)]
struct GlobalState {
    /// All active animation handles
    active_handles: Vec<AnimationHandle>,
    /// Global pause state
    globally_paused: bool,
    /// Global stop state
    globally_stopped: bool,
}

impl SimplifiedAnimationEngine {
    /// Create a new simplified animation engine
    ///
    /// This is the main entry point for creating an animation engine.
    /// The implementation details are hidden from the user.
    pub fn new() -> Self {
        // Use conditional compilation to handle WASM vs native targets
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                internal_engine: Arc::new(Mutex::new(OptimizedHybridEngine::new())),
                #[cfg(feature = "performance-metrics")]
                performance_cache: Arc::new(Mutex::new(None)),
                global_state: Arc::new(Mutex::new(GlobalState::default())),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For non-WASM targets, create a mock engine that doesn't access WASM statics
            Self {
                internal_engine: Arc::new(Mutex::new(MockEngine::new())),
                #[cfg(feature = "performance-metrics")]
                performance_cache: Arc::new(Mutex::new(None)),
                global_state: Arc::new(Mutex::new(GlobalState::default())),
            }
        }
    }

    /// Check if the animation engine is available
    ///
    /// Returns true if the engine can be used in the current environment.
    pub fn is_available(&self) -> bool {
        // For now, always available in WASM environment
        true
    }

    /// Start an animation
    ///
    /// # Arguments
    /// * `element` - The DOM element to animate
    /// * `target` - The target animation values
    /// * `transition` - The transition configuration
    ///
    /// # Returns
    /// * `Result<AnimationHandle>` - Handle to control the animation
    pub fn animate(
        &mut self,
        #[cfg(feature = "web-sys")] element: &web_sys::Element,
        target: &AnimationTarget,
        transition: &Transition,
    ) -> Result<AnimationHandle> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Create animation config for the engine
        let config = engine::AnimationConfig {
            #[cfg(feature = "web-sys")]
            element: element.clone(),
            from: HashMap::new(), // Start from current state
            to: target.clone(),
            transition: transition.clone(),
            on_complete_id: None,
            on_update_id: None,
        };

        // Start animation
        let handle = engine.animate(&config)?;

        // Track handle globally
        global_state.active_handles.push(handle);

        Ok(handle)
    }

    /// Stop an animation
    ///
    /// # Arguments
    /// * `handle` - The animation handle to stop
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Stop animation
        engine.stop(handle)?;

        // Remove from global tracking
        global_state.active_handles.retain(|&h| h != handle);

        Ok(())
    }

    /// Pause an animation
    ///
    /// # Arguments
    /// * `handle` - The animation handle to pause
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        engine.pause(handle)
    }

    /// Resume a paused animation
    ///
    /// # Arguments
    /// * `handle` - The animation handle to resume
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        engine.resume(handle)
    }

    /// Check if an animation is running
    ///
    /// # Arguments
    /// * `handle` - The animation handle to check
    ///
    /// # Returns
    /// * `bool` - True if animation is running
    pub fn is_running(&self, handle: AnimationHandle) -> bool {
        let engine = self.internal_engine.lock().unwrap();
        engine.is_running(handle)
    }

    /// Get the current playback state of an animation
    ///
    /// # Arguments
    /// * `handle` - The animation handle to check
    ///
    /// # Returns
    /// * `Result<PlaybackState>` - Current state or error
    pub fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        let engine = self.internal_engine.lock().unwrap();
        engine.get_state(handle)
    }

    /// Get performance metrics
    ///
    /// # Returns
    /// * `Option<PerformanceReport>` - Performance metrics if available
    #[cfg(feature = "performance-metrics")]
    pub fn get_performance_metrics(&self) -> Option<PerformanceReport> {
        let mut cache = self.performance_cache.lock().unwrap();

        // Return cached metrics if available
        if let Some(ref metrics) = *cache {
            return Some(metrics.clone());
        }

        // Get fresh metrics from internal engine
        let engine = self.internal_engine.lock().unwrap();
        if let Some(metrics) = engine.get_performance_metrics() {
            *cache = Some(metrics.clone());
            Some(metrics)
        } else {
            None
        }
    }

    /// Get performance metrics (no-op when feature disabled)
    ///
    /// # Returns
    /// * `Option<()>` - Always None when performance metrics are disabled
    #[cfg(not(feature = "performance-metrics"))]
    pub fn get_performance_metrics(&self) -> Option<()> {
        None
    }

    /// Cleanup all animations and reset engine state
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn cleanup(&mut self) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Stop all active animations
        for handle in global_state.active_handles.clone() {
            let _ = engine.stop(handle);
        }

        // Clear global state
        global_state.active_handles.clear();
        global_state.globally_paused = false;
        global_state.globally_stopped = false;

        // Clear performance cache
        #[cfg(feature = "performance-metrics")]
        {
            let mut cache = self.performance_cache.lock().unwrap();
            *cache = None;
        }

        Ok(())
    }

    /// Stop all active animations
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn stop_all(&mut self) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Stop all active animations
        for handle in global_state.active_handles.clone() {
            let _ = engine.stop(handle);
        }

        // Clear global state
        global_state.active_handles.clear();
        global_state.globally_stopped = true;

        Ok(())
    }

    /// Pause all active animations
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn pause_all(&mut self) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Pause all active animations
        for handle in global_state.active_handles.clone() {
            let _ = engine.pause(handle);
        }

        global_state.globally_paused = true;

        Ok(())
    }

    /// Resume all paused animations
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn resume_all(&mut self) -> Result<()> {
        let mut engine = self.internal_engine.lock().unwrap();
        let mut global_state = self.global_state.lock().unwrap();

        // Resume all active animations
        for handle in global_state.active_handles.clone() {
            let _ = engine.resume(handle);
        }

        global_state.globally_paused = false;

        Ok(())
    }

    /// Get the number of active animations
    ///
    /// # Returns
    /// * `usize` - Number of active animations
    pub fn active_animation_count(&self) -> usize {
        let global_state = self.global_state.lock().unwrap();
        global_state.active_handles.len()
    }

    /// Check if any animations are running
    ///
    /// # Returns
    /// * `bool` - True if any animations are running
    pub fn has_active_animations(&self) -> bool {
        self.active_animation_count() > 0
    }
}

impl Default for SimplifiedAnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SimplifiedAnimationEngine {
    fn clone(&self) -> Self {
        Self {
            internal_engine: Arc::clone(&self.internal_engine),
            #[cfg(feature = "performance-metrics")]
            performance_cache: Arc::clone(&self.performance_cache),
            global_state: Arc::clone(&self.global_state),
        }
    }
}

impl std::fmt::Debug for SimplifiedAnimationEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimplifiedAnimationEngine")
            .field("active_animations", &self.active_animation_count())
            .field("has_active_animations", &self.has_active_animations())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplified_engine_creation() {
        let engine = SimplifiedAnimationEngine::new();
        assert!(engine.is_available());
        assert_eq!(engine.active_animation_count(), 0);
        assert!(!engine.has_active_animations());
    }

    #[test]
    fn test_simplified_engine_clone() {
        let engine1 = SimplifiedAnimationEngine::new();
        let engine2 = engine1.clone();

        assert!(engine1.is_available());
        assert!(engine2.is_available());
        assert_eq!(
            engine1.active_animation_count(),
            engine2.active_animation_count()
        );
    }

    #[test]
    fn test_simplified_engine_debug() {
        let engine = SimplifiedAnimationEngine::new();
        let debug_str = format!("{:?}", engine);
        assert!(debug_str.contains("SimplifiedAnimationEngine"));
        assert!(debug_str.contains("active_animations"));
        assert!(debug_str.contains("has_active_animations"));
    }
}
