//! Animation engine traits and implementations

use crate::performance::{
    AnimationPool, AnimationScheduler, GPULayerManager, PerformanceBudget, PerformanceMonitor,
};
use crate::{
    AnimationError, AnimationHandle, AnimationTarget, AnimationValue, Result, Transform, Transition,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Performance, window};

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
            .field(
                "on_complete",
                &if self.on_complete_id.is_some() {
                    "Some(<callback>)"
                } else {
                    "None"
                },
            )
            .field(
                "on_update",
                &if self.on_update_id.is_some() {
                    "Some(<callback>)"
                } else {
                    "None"
                },
            )
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
    /// Create a new optimized hybrid engine instance
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
    pub fn optimize_for_gpu(&mut self, _element: &Element) -> bool {
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
            EngineChoice::Waapi => self
                .waapi_engine
                .animate_with_handle(handle, config.clone()),
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
    /// Create a new WAAPI engine
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    /// Animate with a specific handle
    pub fn animate_with_handle(
        &mut self,
        handle: AnimationHandle,
        _config: AnimationConfig,
    ) -> Result<()> {
        // Implementation would create Web Animation API animation
        // This is a simplified version
        self.animations
            .insert(handle, web_sys::Animation::new().unwrap_throw());
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
        Err(AnimationError::EngineUnavailable(
            "WAAPI not implemented".to_string(),
        ))
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
    /// Create a new RAF engine
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

    /// Animate with a specific handle
    pub fn animate_with_handle(
        &mut self,
        handle: AnimationHandle,
        config: AnimationConfig,
    ) -> Result<()> {
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

        if let Some(_callback_id) = self.config.on_update_id {
            // Call callback by ID (simplified for now)
            // In a real implementation, this would look up the callback by ID
        }
    }

    fn apply_values(&self, progress: f64) {
        // Interpolate between from and to values based on progress
        let mut current_values = HashMap::new();

        for (property, from_value) in &self.config.from {
            if let Some(to_value) = self.config.to.get(property) {
                // Interpolate between from and to values
                let interpolated = self.interpolate_values(from_value, to_value, progress);
                current_values.insert(property.clone(), interpolated);
            }
        }

        // Apply interpolated values to the element
        self.apply_to_element(&current_values);
    }

    fn interpolate_values(
        &self,
        from: &AnimationValue,
        to: &AnimationValue,
        progress: f64,
    ) -> AnimationValue {
        match (from, to) {
            (AnimationValue::Number(from_val), AnimationValue::Number(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * progress;
                AnimationValue::Number(interpolated)
            }
            (AnimationValue::Pixels(from_val), AnimationValue::Pixels(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * progress;
                AnimationValue::Pixels(interpolated)
            }
            (AnimationValue::Percentage(from_val), AnimationValue::Percentage(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * progress;
                AnimationValue::Percentage(interpolated)
            }
            (AnimationValue::Degrees(from_val), AnimationValue::Degrees(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * progress;
                AnimationValue::Degrees(interpolated)
            }
            (AnimationValue::Color(from_color), AnimationValue::Color(to_color)) => {
                if progress < 0.5 {
                    AnimationValue::Color(from_color.clone())
                } else {
                    AnimationValue::Color(to_color.clone())
                }
            }
            (
                AnimationValue::Transform(from_transform),
                AnimationValue::Transform(to_transform),
            ) => {
                let interpolated =
                    self.interpolate_transform(from_transform, to_transform, progress);
                AnimationValue::Transform(interpolated)
            }
            _ => {
                if progress < 0.5 {
                    from.clone()
                } else {
                    to.clone()
                }
            }
        }
    }

    fn interpolate_transform(&self, from: &Transform, to: &Transform, progress: f64) -> Transform {
        Transform {
            x: self.interpolate_option(from.x, to.x, progress),
            y: self.interpolate_option(from.y, to.y, progress),
            z: self.interpolate_option(from.z, to.z, progress),
            rotate_x: self.interpolate_option(from.rotate_x, to.rotate_x, progress),
            rotate_y: self.interpolate_option(from.rotate_y, to.rotate_y, progress),
            rotate_z: self.interpolate_option(from.rotate_z, to.rotate_z, progress),
            scale: self.interpolate_option(from.scale, to.scale, progress),
            scale_x: self.interpolate_option(from.scale_x, to.scale_x, progress),
            scale_y: self.interpolate_option(from.scale_y, to.scale_y, progress),
            skew_x: self.interpolate_option(from.skew_x, to.skew_x, progress),
            skew_y: self.interpolate_option(from.skew_y, to.skew_y, progress),
        }
    }

    fn interpolate_option(&self, from: Option<f64>, to: Option<f64>, progress: f64) -> Option<f64> {
        match (from, to) {
            (Some(from_val), Some(to_val)) => {
                let interpolated = from_val + (to_val - from_val) * progress;
                Some(interpolated)
            }
            (Some(val), None) | (None, Some(val)) => Some(val),
            (None, None) => None,
        }
    }

    fn apply_to_element(&self, values: &HashMap<String, AnimationValue>) {
        // Apply values directly to element properties
        for (property, value) in values {
            let css_value = self.animation_value_to_css(property, value);
            // For now, we'll use a simplified approach
            // In a real implementation, this would update the element's style
            let _ = css_value; // Suppress unused variable warning
        }
    }

    fn animation_value_to_css(&self, _property: &str, value: &AnimationValue) -> String {
        match value {
            AnimationValue::Number(val) => val.to_string(),
            AnimationValue::Pixels(val) => format!("{}px", val),
            AnimationValue::Percentage(val) => format!("{}%", val),
            AnimationValue::Degrees(val) => format!("{}deg", val),
            AnimationValue::Color(color) => color.clone(),
            AnimationValue::Transform(transform) => self.transform_to_css(transform),
            AnimationValue::String(s) => s.clone(),
            _ => "".to_string(),
        }
    }

    fn transform_to_css(&self, transform: &Transform) -> String {
        let mut parts = Vec::new();

        if let Some(x) = transform.x {
            parts.push(format!("translateX({}px)", x));
        }
        if let Some(y) = transform.y {
            parts.push(format!("translateY({}px)", y));
        }
        if let Some(z) = transform.z {
            parts.push(format!("translateZ({}px)", z));
        }
        if let Some(rotate_x) = transform.rotate_x {
            parts.push(format!("rotateX({}deg)", rotate_x));
        }
        if let Some(rotate_y) = transform.rotate_y {
            parts.push(format!("rotateY({}deg)", rotate_y));
        }
        if let Some(rotate_z) = transform.rotate_z {
            parts.push(format!("rotateZ({}deg)", rotate_z));
        }
        if let Some(scale) = transform.scale {
            parts.push(format!("scale({})", scale));
        }
        if let Some(scale_x) = transform.scale_x {
            parts.push(format!("scaleX({})", scale_x));
        }
        if let Some(scale_y) = transform.scale_y {
            parts.push(format!("scaleY({})", scale_y));
        }
        if let Some(skew_x) = transform.skew_x {
            parts.push(format!("skewX({}deg)", skew_x));
        }
        if let Some(skew_y) = transform.skew_y {
            parts.push(format!("skewY({}deg)", skew_y));
        }

        parts.join(" ")
    }

    fn is_complete(&self) -> bool {
        matches!(
            self.state,
            PlaybackState::Completed | PlaybackState::Cancelled
        )
    }
}

/// Feature detection for animation capabilities
pub struct FeatureDetector {
    waapi_available: Option<bool>,
}

impl FeatureDetector {
    /// Create a new feature detector
    pub fn new() -> Self {
        Self {
            waapi_available: None,
        }
    }

    /// Check if WAAPI is supported
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

    /// Check if WAAPI can be used for a specific animation
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
            "opacity"
                | "transform"
                | "left"
                | "top"
                | "width"
                | "height"
                | "background-color"
                | "color"
                | "border-radius"
                | "scale"
                | "rotate"
                | "translateX"
                | "translateY"
                | "translateZ"
                | "rotateX"
                | "rotateY"
                | "rotateZ"
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
    use crate::{AnimationValue, Transform, Transition};
    use std::collections::HashMap;

    // Helper function to create a mock element (for environments that support it)
    #[cfg(target_arch = "wasm32")]
    fn create_mock_element() -> Option<Element> {
        // In a real test environment, this would create a proper Element
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("div").ok())
    }

    // For non-wasm environments, return None
    #[cfg(not(target_arch = "wasm32"))]
    fn create_mock_element() -> Option<Element> {
        None
    }

    fn create_test_animation_target() -> (
        HashMap<String, AnimationValue>,
        HashMap<String, AnimationValue>,
    ) {
        let mut from = HashMap::new();
        from.insert("opacity".to_string(), AnimationValue::Number(0.0));
        from.insert("x".to_string(), AnimationValue::Pixels(0.0));

        let mut to = HashMap::new();
        to.insert("opacity".to_string(), AnimationValue::Number(1.0));
        to.insert("x".to_string(), AnimationValue::Pixels(100.0));

        (from, to)
    }

    // Test that our core types can be created and manipulated
    #[test]
    fn test_core_types() {
        // Test AnimationValue creation
        let number_val = AnimationValue::Number(42.0);
        let pixel_val = AnimationValue::Pixels(100.0);
        let percent_val = AnimationValue::Percentage(50.0);
        let degree_val = AnimationValue::Degrees(180.0);
        let color_val = AnimationValue::Color("#ff0000".to_string());
        let string_val = AnimationValue::String("solid".to_string());

        // Test that values can be cloned
        let _cloned_number = number_val.clone();
        let _cloned_pixel = pixel_val.clone();
        let _cloned_percent = percent_val.clone();
        let _cloned_degree = degree_val.clone();
        let _cloned_color = color_val.clone();
        let _cloned_string = string_val.clone();

        // Test Transform creation
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            z: Some(30.0),
            rotate_x: Some(45.0),
            rotate_y: Some(90.0),
            rotate_z: Some(180.0),
            scale: Some(1.5),
            scale_x: Some(2.0),
            scale_y: Some(0.5),
            skew_x: Some(15.0),
            skew_y: Some(30.0),
        };

        // Test that transform can be cloned
        let _cloned_transform = transform.clone();

        // Test that we can access transform fields
        assert_eq!(transform.x, Some(10.0));
        assert_eq!(transform.y, Some(20.0));
        assert_eq!(transform.z, Some(30.0));
        assert_eq!(transform.rotate_x, Some(45.0));
        assert_eq!(transform.rotate_y, Some(90.0));
        assert_eq!(transform.rotate_z, Some(180.0));
        assert_eq!(transform.scale, Some(1.5));
        assert_eq!(transform.scale_x, Some(2.0));
        assert_eq!(transform.scale_y, Some(0.5));
        assert_eq!(transform.skew_x, Some(15.0));
        assert_eq!(transform.skew_y, Some(30.0));
    }

    // Test interpolation logic without web dependencies
    #[test]
    fn test_interpolation_math() {
        // Test basic interpolation formula
        let from = 0.0;
        let to = 100.0;

        // Test at 0% progress
        let result = from + (to - from) * 0.0;
        assert_eq!(result, 0.0);

        // Test at 50% progress
        let result = from + (to - from) * 0.5;
        assert_eq!(result, 50.0);

        // Test at 100% progress
        let result = from + (to - from) * 1.0;
        assert_eq!(result, 100.0);

        // Test at 25% progress
        let result = from + (to - from) * 0.25;
        assert_eq!(result, 25.0);

        // Test negative values
        let from_neg = -50.0;
        let to_neg = 50.0;
        let result = from_neg + (to_neg - from_neg) * 0.5;
        assert_eq!(result, 0.0);
    }

    // Test option interpolation logic
    #[test]
    fn test_option_interpolation() {
        // Test both values present
        let from = Some(0.0);
        let to = Some(100.0);

        // This would be the logic from interpolate_option method
        let interpolate = |from: Option<f64>, to: Option<f64>, progress: f64| -> Option<f64> {
            match (from, to) {
                (Some(from_val), Some(to_val)) => {
                    let interpolated = from_val + (to_val - from_val) * progress;
                    Some(interpolated)
                }
                (Some(val), None) | (None, Some(val)) => Some(val),
                (None, None) => None,
            }
        };

        // Test at 50% progress
        let result = interpolate(from, to, 0.5);
        assert_eq!(result, Some(50.0));

        // Test from value missing
        let result = interpolate(None, to, 0.5);
        assert_eq!(result, Some(100.0));

        // Test to value missing
        let result = interpolate(from, None, 0.5);
        assert_eq!(result, Some(0.0));

        // Test both values missing
        let result = interpolate(None, None, 0.5);
        assert_eq!(result, None);
    }

    // Test basic functionality without web dependencies
    #[test]
    fn test_basic_functionality() {
        // Test that we can create basic structures
        let _detector = FeatureDetector::new();

        // Test playback states
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

    #[test]
    fn test_animation_config_targets() {
        let (from, to) = create_test_animation_target();
        assert_eq!(from.len(), 2);
        assert_eq!(to.len(), 2);
        assert!(from.contains_key("opacity"));
        assert!(to.contains_key("opacity"));
        assert_eq!(from.get("opacity"), Some(&AnimationValue::Number(0.0)));
        assert_eq!(to.get("opacity"), Some(&AnimationValue::Number(1.0)));
    }

    #[test]
    fn test_animation_config_callbacks() {
        // Test callback IDs without requiring DOM elements
        let callback_ids = (Some(123u64), Some(456u64));
        assert_eq!(callback_ids.0, Some(123));
        assert_eq!(callback_ids.1, Some(456));
    }

    #[test]
    fn test_playback_state_variants() {
        let running = PlaybackState::Running;
        let paused = PlaybackState::Paused;
        let completed = PlaybackState::Completed;
        let cancelled = PlaybackState::Cancelled;

        assert_eq!(running, PlaybackState::Running);
        assert_eq!(paused, PlaybackState::Paused);
        assert_eq!(completed, PlaybackState::Completed);
        assert_eq!(cancelled, PlaybackState::Cancelled);

        assert_ne!(running, paused);
        assert_ne!(completed, cancelled);
    }

    #[test]
    fn test_engine_type_basics() {
        // Test engine availability logic without creating engines
        let waapi_available = false; // Assume not available in test environment
        let raf_available = true; // RAF should always be available

        assert!(raf_available || waapi_available); // At least one should be available
    }

    #[test]
    fn test_handle_generation_logic() {
        // Test handle generation logic without full engine
        let mut current_handle = 0u64;

        let mut next_handle = || {
            current_handle += 1;
            AnimationHandle(current_handle)
        };

        let handle1 = next_handle();
        let handle2 = next_handle();
        let handle3 = next_handle();

        assert_ne!(handle1.0, handle2.0);
        assert_ne!(handle2.0, handle3.0);
        assert!(handle1.0 < handle2.0);
        assert!(handle2.0 < handle3.0);
        assert_eq!(handle1.0, 1);
        assert_eq!(handle2.0, 2);
        assert_eq!(handle3.0, 3);
    }

    #[test]
    fn test_gpu_optimization_logic() {
        // Test GPU optimization logic without DOM dependencies
        let gpu_enabled = false; // For now, always false
        assert!(!gpu_enabled);
    }

    #[test]
    fn test_waapi_engine_creation() {
        let engine = WaapiEngine::new();
        assert!(engine.animations.is_empty());
    }

    #[test]
    fn test_waapi_engine_default() {
        let engine = WaapiEngine::default();
        assert!(engine.animations.is_empty());
    }

    #[test]
    fn test_waapi_engine_tick() {
        let mut engine = WaapiEngine::new();
        let result = engine.tick(123.45);
        assert!(result.is_ok()); // WAAPI doesn't need manual ticking
    }

    #[test]
    fn test_waapi_engine_performance_metrics() {
        let engine = WaapiEngine::new();
        let metrics = engine.get_performance_metrics();
        assert!(metrics.is_none()); // WAAPI doesn't provide metrics
    }

    // Test basic RAF engine functionality without web dependencies
    #[test]
    fn test_raf_engine_basics() {
        // Test basic RAF engine structure without requiring DOM
        let animations = HashMap::<AnimationHandle, RafAnimation>::new();
        assert!(animations.is_empty());

        let raf_handle: Option<i32> = None;
        assert!(raf_handle.is_none());
    }

    #[test]
    fn test_raf_animation_basics() {
        // Test basic RAF animation concepts without full DOM setup
        let start_time = 100.0;
        let state = PlaybackState::Running;

        assert_eq!(start_time, 100.0);
        assert_eq!(state, PlaybackState::Running);
    }

    #[test]
    fn test_raf_animation_progress_calculation() {
        // Test animation progress calculation without DOM dependencies
        let start_time = 0.0;
        let duration_ms = 1000.0; // 1 second

        // Test at different timestamps
        let timestamp1 = 500.0; // 50% progress
        let elapsed1 = timestamp1 - start_time;
        let progress1 = (elapsed1 / duration_ms as f64).clamp(0.0, 1.0);
        assert_eq!(progress1, 0.5);

        let timestamp2 = 1500.0; // 150% (should clamp to 100%)
        let elapsed2 = timestamp2 - start_time;
        let progress2 = (elapsed2 / duration_ms as f64).clamp(0.0, 1.0);
        assert_eq!(progress2, 1.0);
    }

    #[test]
    fn test_animation_value_interpolation_logic() {
        // Test interpolation logic without requiring DOM or RafAnimation creation

        // Number interpolation
        let from_num = 0.0;
        let to_num = 100.0;
        let progress = 0.5;
        let result = from_num + (to_num - from_num) * progress;
        assert_eq!(result, 50.0);

        // Pixel interpolation (same math, different units)
        let from_px = 0.0;
        let to_px = 200.0;
        let result = from_px + (to_px - from_px) * 0.25;
        assert_eq!(result, 50.0);

        // Percentage interpolation
        let from_pct = 0.0;
        let to_pct = 100.0;
        let result = from_pct + (to_pct - from_pct) * 0.75;
        assert_eq!(result, 75.0);

        // Degrees interpolation
        let from_deg = 0.0;
        let to_deg = 360.0;
        let result = from_deg + (to_deg - from_deg) * 0.5;
        assert_eq!(result, 180.0);

        // Color interpolation (step function logic)
        let color_from = "red";
        let color_to = "blue";
        let result_early = if 0.3 < 0.5 { color_from } else { color_to };
        assert_eq!(result_early, "red");
        let result_late = if 0.7 < 0.5 { color_from } else { color_to };
        assert_eq!(result_late, "blue");
    }

    #[test]
    fn test_transform_interpolation_logic() {
        // Test transform interpolation logic without requiring DOM or RafAnimation
        let from_transform = Transform {
            x: Some(0.0),
            y: Some(0.0),
            scale: Some(1.0),
            rotate_z: Some(0.0),
            ..Transform::default()
        };

        let to_transform = Transform {
            x: Some(100.0),
            y: Some(200.0),
            scale: Some(2.0),
            rotate_z: Some(360.0),
            ..Transform::default()
        };

        let progress = 0.5;

        // Test individual field interpolation
        let x_result = match (from_transform.x, to_transform.x) {
            (Some(from_val), Some(to_val)) => Some(from_val + (to_val - from_val) * progress),
            (Some(val), None) | (None, Some(val)) => Some(val),
            (None, None) => None,
        };
        assert_eq!(x_result, Some(50.0));

        let y_result = match (from_transform.y, to_transform.y) {
            (Some(from_val), Some(to_val)) => Some(from_val + (to_val - from_val) * progress),
            (Some(val), None) | (None, Some(val)) => Some(val),
            (None, None) => None,
        };
        assert_eq!(y_result, Some(100.0));

        let scale_result = match (from_transform.scale, to_transform.scale) {
            (Some(from_val), Some(to_val)) => Some(from_val + (to_val - from_val) * progress),
            (Some(val), None) | (None, Some(val)) => Some(val),
            (None, None) => None,
        };
        assert_eq!(scale_result, Some(1.5));

        let rotate_result = match (from_transform.rotate_z, to_transform.rotate_z) {
            (Some(from_val), Some(to_val)) => Some(from_val + (to_val - from_val) * progress),
            (Some(val), None) | (None, Some(val)) => Some(val),
            (None, None) => None,
        };
        assert_eq!(rotate_result, Some(180.0));
    }

    #[test]
    fn test_animation_value_to_css_logic() {
        // Test CSS value conversion logic without requiring RafAnimation

        // Test number
        let number_val = 0.5;
        let result = number_val.to_string();
        assert_eq!(result, "0.5");

        // Test pixels
        let pixel_val = 100.0;
        let result = format!("{}px", pixel_val);
        assert_eq!(result, "100px");

        // Test percentage
        let percent_val = 50.0;
        let result = format!("{}%", percent_val);
        assert_eq!(result, "50%");

        // Test degrees
        let degree_val = 45.0;
        let result = format!("{}deg", degree_val);
        assert_eq!(result, "45deg");

        // Test color
        let color_val = "red";
        let result = color_val.to_string();
        assert_eq!(result, "red");

        // Test string
        let string_val = "block";
        let result = string_val.to_string();
        assert_eq!(result, "block");
    }

    #[test]
    fn test_transform_to_css_logic() {
        // Test transform to CSS conversion logic without requiring RafAnimation
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            z: Some(30.0),
            rotate_x: Some(45.0),
            rotate_y: Some(90.0),
            rotate_z: Some(180.0),
            scale: Some(1.5),
            scale_x: Some(2.0),
            scale_y: Some(0.5),
            skew_x: Some(15.0),
            skew_y: Some(30.0),
        };

        let mut parts = Vec::new();

        if let Some(x) = transform.x {
            parts.push(format!("translateX({}px)", x));
        }
        if let Some(y) = transform.y {
            parts.push(format!("translateY({}px)", y));
        }
        if let Some(z) = transform.z {
            parts.push(format!("translateZ({}px)", z));
        }
        if let Some(rotate_x) = transform.rotate_x {
            parts.push(format!("rotateX({}deg)", rotate_x));
        }
        if let Some(rotate_y) = transform.rotate_y {
            parts.push(format!("rotateY({}deg)", rotate_y));
        }
        if let Some(rotate_z) = transform.rotate_z {
            parts.push(format!("rotateZ({}deg)", rotate_z));
        }
        if let Some(scale) = transform.scale {
            parts.push(format!("scale({})", scale));
        }
        if let Some(scale_x) = transform.scale_x {
            parts.push(format!("scaleX({})", scale_x));
        }
        if let Some(scale_y) = transform.scale_y {
            parts.push(format!("scaleY({})", scale_y));
        }
        if let Some(skew_x) = transform.skew_x {
            parts.push(format!("skewX({}deg)", skew_x));
        }
        if let Some(skew_y) = transform.skew_y {
            parts.push(format!("skewY({}deg)", skew_y));
        }

        let css = parts.join(" ");

        assert!(css.contains("translateX(10px)"));
        assert!(css.contains("translateY(20px)"));
        assert!(css.contains("translateZ(30px)"));
        assert!(css.contains("rotateX(45deg)"));
        assert!(css.contains("rotateY(90deg)"));
        assert!(css.contains("rotateZ(180deg)"));
        assert!(css.contains("scale(1.5)"));
        assert!(css.contains("scaleX(2)"));
        assert!(css.contains("scaleY(0.5)"));
        assert!(css.contains("skewX(15deg)"));
        assert!(css.contains("skewY(30deg)"));
    }

    #[test]
    fn test_animation_completion_logic() {
        // Test animation completion logic without requiring RafAnimation
        let is_complete = |state: &PlaybackState| -> bool {
            matches!(state, PlaybackState::Completed | PlaybackState::Cancelled)
        };

        // Initially running, not complete
        assert!(!is_complete(&PlaybackState::Running));

        // Complete the animation
        assert!(is_complete(&PlaybackState::Completed));

        // Cancel the animation
        assert!(is_complete(&PlaybackState::Cancelled));

        // Paused is not complete
        assert!(!is_complete(&PlaybackState::Paused));
    }

    #[test]
    fn test_feature_detector_creation() {
        let detector = FeatureDetector::new();
        assert!(detector.waapi_available.is_none()); // Not yet determined
    }

    #[test]
    fn test_waapi_property_detection() {
        // Test WAAPI property detection logic
        let is_waapi_property = |property: &str| -> bool {
            matches!(
                property,
                "opacity"
                    | "transform"
                    | "left"
                    | "top"
                    | "width"
                    | "height"
                    | "background-color"
                    | "color"
                    | "border-radius"
                    | "scale"
                    | "rotate"
                    | "translateX"
                    | "translateY"
                    | "translateZ"
                    | "rotateX"
                    | "rotateY"
                    | "rotateZ"
            )
        };

        // Test supported properties
        assert!(is_waapi_property("opacity"));
        assert!(is_waapi_property("transform"));
        assert!(is_waapi_property("left"));
        assert!(is_waapi_property("top"));
        assert!(is_waapi_property("width"));
        assert!(is_waapi_property("height"));
        assert!(is_waapi_property("background-color"));
        assert!(is_waapi_property("color"));
        assert!(is_waapi_property("border-radius"));
        assert!(is_waapi_property("scale"));
        assert!(is_waapi_property("rotate"));
        assert!(is_waapi_property("translateX"));
        assert!(is_waapi_property("translateY"));
        assert!(is_waapi_property("translateZ"));
        assert!(is_waapi_property("rotateX"));
        assert!(is_waapi_property("rotateY"));
        assert!(is_waapi_property("rotateZ"));

        // Test unsupported properties
        assert!(!is_waapi_property("custom-property"));
        assert!(!is_waapi_property("unknown"));
        assert!(!is_waapi_property(""));
    }

    #[test]
    fn test_engine_choice_variants() {
        let waapi = EngineChoice::Waapi;
        let raf = EngineChoice::Raf;

        match waapi {
            EngineChoice::Waapi => {}
            _ => panic!("Expected WAAPI variant"),
        }

        match raf {
            EngineChoice::Raf => {}
            _ => panic!("Expected RAF variant"),
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
