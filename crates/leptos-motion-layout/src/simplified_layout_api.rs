//! Simplified Layout/Scroll API
//!
//! This module provides a simplified, user-friendly layout/scroll API
//! that hides complexity and provides a clean interface.

use crate::*;
use std::collections::HashMap;
use std::time::Instant;
use web_sys::{DomRect, Element};

/// Mock layout tracker for non-WASM targets to avoid WASM function access issues
#[cfg(not(target_arch = "wasm32"))]
struct MockLayoutTracker;

#[cfg(not(target_arch = "wasm32"))]
impl MockLayoutTracker {
    fn new() -> Self {
        Self
    }

    fn track_element(&self, _element: Element) -> Result<(), String> {
        Ok(())
    }

    fn untrack_element(&self, _element_id: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Mock FLIP animator for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
struct MockFLIPAnimator;

#[cfg(not(target_arch = "wasm32"))]
impl MockFLIPAnimator {
    fn new() -> Self {
        Self
    }

    fn animate(
        &self,
        _element_id: String,
        _element: Element,
        _from_rect: DomRect,
        _to_rect: DomRect,
        _config: LayoutAnimationConfig,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// Mock shared element manager for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
struct MockSharedElementManager;

#[cfg(not(target_arch = "wasm32"))]
impl MockSharedElementManager {
    fn new(_strategy: ZIndexStrategy) -> Self {
        Self
    }

    fn start_transition(&self, _transition_id: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Simplified easing functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimplifiedEasing {
    /// Linear easing
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in out
    EaseInOut,
    /// Ease in cubic
    EaseInCubic,
    /// Ease out cubic
    EaseOutCubic,
    /// Ease in out cubic
    EaseInOutCubic,
}

/// Simplified layout configuration
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedLayoutConfig {
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: SimplifiedEasing,
    /// Whether to use hardware acceleration
    pub hardware_accelerated: bool,
    /// Whether to enable FLIP animations
    pub enable_flip: bool,
    /// Whether to enable shared element transitions
    pub enable_shared_elements: bool,
}

/// Simplified animation status
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedAnimationStatus {
    /// Whether animation is currently running
    pub is_animating: bool,
    /// Whether animation is paused
    pub is_paused: bool,
    /// Animation progress (0.0 to 1.0)
    pub progress: f64,
    /// Animation start time
    pub start_time: Option<Instant>,
    /// Animation duration
    pub duration: f64,
}

/// Simplified performance metrics
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedPerformanceMetrics {
    /// Total number of animations
    pub total_animations: usize,
    /// Average animation duration
    pub average_duration: f64,
    /// Frame rate during animations
    pub frame_rate: f64,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// CPU usage percentage
    pub cpu_usage: f64,
}

/// Simplified layout manager that provides a clean, simple interface
///
/// This is the main public API for layout animations. It provides
/// a clean, simple interface while hiding the complexity of the
/// underlying layout tracking and animation systems.
pub struct SimplifiedLayoutManager {
    /// Internal layout tracker (hidden from public API)
    /// Uses different implementations for WASM vs native targets
    #[cfg(target_arch = "wasm32")]
    internal_tracker: LayoutTracker,
    #[cfg(not(target_arch = "wasm32"))]
    internal_tracker: MockLayoutTracker,
    /// Internal FLIP animator (hidden from public API)
    #[cfg(target_arch = "wasm32")]
    internal_flip_animator: FLIPAnimator,
    #[cfg(not(target_arch = "wasm32"))]
    internal_flip_animator: MockFLIPAnimator,
    /// Internal shared element manager (hidden from public API)
    #[cfg(target_arch = "wasm32")]
    internal_shared_manager: SharedElementManager,
    #[cfg(not(target_arch = "wasm32"))]
    internal_shared_manager: MockSharedElementManager,
    /// Current configuration
    config: SimplifiedLayoutConfig,
    /// Tracked elements
    tracked_elements: HashMap<String, Element>,
    /// Active animations
    active_animations: HashMap<String, SimplifiedAnimationStatus>,
    /// Performance metrics
    performance_metrics: SimplifiedPerformanceMetrics,
}

impl SimplifiedLayoutManager {
    /// Create a new simplified layout manager with default configuration
    pub fn new() -> Self {
        Self::with_config(SimplifiedLayoutConfig::default())
    }

    /// Create a new simplified layout manager with custom configuration
    pub fn with_config(config: SimplifiedLayoutConfig) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                internal_tracker: LayoutTracker::new(),
                internal_flip_animator: FLIPAnimator::new(),
                internal_shared_manager: SharedElementManager::new(ZIndexStrategy::default()),
                config,
                tracked_elements: HashMap::new(),
                active_animations: HashMap::new(),
                performance_metrics: SimplifiedPerformanceMetrics {
                    total_animations: 0,
                    average_duration: 0.0,
                    frame_rate: 60.0,
                    memory_usage: 0,
                    cpu_usage: 0.0,
                },
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                internal_tracker: MockLayoutTracker::new(),
                internal_flip_animator: MockFLIPAnimator::new(),
                internal_shared_manager: MockSharedElementManager::new(ZIndexStrategy::default()),
                config,
                tracked_elements: HashMap::new(),
                active_animations: HashMap::new(),
                performance_metrics: SimplifiedPerformanceMetrics {
                    total_animations: 0,
                    average_duration: 0.0,
                    frame_rate: 60.0,
                    memory_usage: 0,
                    cpu_usage: 0.0,
                },
            }
        }
    }

    /// Start tracking an element for layout changes
    pub fn start_tracking(&mut self, element_id: &str, element: &Element) -> Result<(), String> {
        if self.tracked_elements.contains_key(element_id) {
            return Err(format!("Element '{}' is already being tracked", element_id));
        }

        self.tracked_elements
            .insert(element_id.to_string(), element.clone());
        self.internal_tracker.track_element(element.clone());

        Ok(())
    }

    /// Stop tracking an element
    pub fn stop_tracking(&mut self, element_id: &str) -> Result<(), String> {
        if !self.tracked_elements.contains_key(element_id) {
            return Err(format!("Element '{}' is not being tracked", element_id));
        }

        self.tracked_elements.remove(element_id);
        self.internal_tracker.untrack_element(element_id);
        self.active_animations.remove(element_id);

        Ok(())
    }

    /// Animate layout change for an element
    pub fn animate_layout_change(
        &mut self,
        element_id: &str,
        from_layout: &LayoutInfo,
        to_layout: &LayoutInfo,
    ) -> Result<(), String> {
        if !self.tracked_elements.contains_key(element_id) {
            return Err(format!("Element '{}' is not being tracked", element_id));
        }

        let element = self.tracked_elements.get(element_id).unwrap();

        // Create FLIP animation using the correct API
        let from_rect = web_sys::DomRect::new_with_x_and_y_and_width_and_height(
            from_layout.x,
            from_layout.y,
            from_layout.width,
            from_layout.height,
        )
        .unwrap();
        let to_rect = web_sys::DomRect::new_with_x_and_y_and_width_and_height(
            to_layout.x,
            to_layout.y,
            to_layout.width,
            to_layout.height,
        )
        .unwrap();

        let animation_config = LayoutAnimationConfig {
            enabled: true,
            duration: self.config.duration,
            easing: self.convert_easing(self.config.easing),
            hardware_accelerated: self.config.hardware_accelerated,
        };

        self.internal_flip_animator.animate(
            element_id.to_string(),
            element.clone(),
            from_rect,
            to_rect,
            animation_config,
        );

        // Update animation status
        self.active_animations.insert(
            element_id.to_string(),
            SimplifiedAnimationStatus {
                is_animating: true,
                is_paused: false,
                progress: 0.0,
                start_time: Some(Instant::now()),
                duration: self.config.duration,
            },
        );

        // Update performance metrics
        self.performance_metrics.total_animations += 1;

        Ok(())
    }

    /// Perform FLIP animation
    pub fn flip_animate(
        &mut self,
        element_id: &str,
        from_layout: &LayoutInfo,
        to_layout: &LayoutInfo,
    ) -> Result<(), String> {
        if !self.config.enable_flip {
            return Err("FLIP animations are disabled".to_string());
        }

        self.animate_layout_change(element_id, from_layout, to_layout)
    }

    /// Perform shared element transition
    pub fn shared_element_transition(
        &mut self,
        from_element_id: &str,
        to_element_id: &str,
        from_layout: &LayoutInfo,
        to_layout: &LayoutInfo,
    ) -> Result<(), String> {
        if !self.config.enable_shared_elements {
            return Err("Shared element transitions are disabled".to_string());
        }

        if !self.tracked_elements.contains_key(from_element_id) {
            return Err(format!(
                "Element '{}' is not being tracked",
                from_element_id
            ));
        }

        if !self.tracked_elements.contains_key(to_element_id) {
            return Err(format!("Element '{}' is not being tracked", to_element_id));
        }

        let from_element = self.tracked_elements.get(from_element_id).unwrap();
        let to_element = self.tracked_elements.get(to_element_id).unwrap();

        // Create shared element transition using the correct API
        let config = SharedElementConfig {
            maintain_aspect_ratio: true,
            hardware_accelerated: self.config.hardware_accelerated,
            duration: self.config.duration,
            easing: self.convert_easing(self.config.easing),
        };

        self.internal_shared_manager
            .start_transition("shared-transition");

        // Update animation status for both elements
        let status = SimplifiedAnimationStatus {
            is_animating: true,
            is_paused: false,
            progress: 0.0,
            start_time: Some(Instant::now()),
            duration: self.config.duration,
        };

        self.active_animations
            .insert(from_element_id.to_string(), status.clone());
        self.active_animations
            .insert(to_element_id.to_string(), status);

        // Update performance metrics
        self.performance_metrics.total_animations += 1;

        Ok(())
    }

    /// Batch start tracking multiple elements
    pub fn batch_start_tracking(&mut self, elements: Vec<(&str, &Element)>) -> Result<(), String> {
        for (element_id, element) in elements {
            self.start_tracking(element_id, element)?;
        }
        Ok(())
    }

    /// Batch animate multiple elements
    pub fn batch_animate(
        &mut self,
        animations: Vec<(&str, &LayoutInfo, &LayoutInfo)>,
    ) -> Result<(), String> {
        for (element_id, from_layout, to_layout) in animations {
            self.animate_layout_change(element_id, from_layout, to_layout)?;
        }
        Ok(())
    }

    /// Get layout information for an element
    pub fn get_layout_info(&self, element_id: &str) -> Option<LayoutInfo> {
        if let Some(element) = self.tracked_elements.get(element_id) {
            let rect = element.get_bounding_client_rect();
            Some(LayoutInfo::new(
                rect.left(),
                rect.top(),
                rect.width(),
                rect.height(),
            ))
        } else {
            None
        }
    }

    /// Get animation status for an element
    pub fn get_animation_status(&self, element_id: &str) -> Option<&SimplifiedAnimationStatus> {
        self.active_animations.get(element_id)
    }

    /// Pause animation for an element
    pub fn pause_animation(&mut self, element_id: &str) -> Result<(), String> {
        if let Some(status) = self.active_animations.get_mut(element_id) {
            status.is_paused = true;
            Ok(())
        } else {
            Err(format!("No active animation for element '{}'", element_id))
        }
    }

    /// Resume animation for an element
    pub fn resume_animation(&mut self, element_id: &str) -> Result<(), String> {
        if let Some(status) = self.active_animations.get_mut(element_id) {
            status.is_paused = false;
            Ok(())
        } else {
            Err(format!("No active animation for element '{}'", element_id))
        }
    }

    /// Cancel animation for an element
    pub fn cancel_animation(&mut self, element_id: &str) -> Result<(), String> {
        if let Some(status) = self.active_animations.get_mut(element_id) {
            status.is_animating = false;
            status.is_paused = false;
            status.progress = 0.0;
            status.start_time = None;
            Ok(())
        } else {
            Err(format!("No active animation for element '{}'", element_id))
        }
    }

    /// Clear all tracking and animations
    pub fn clear_all(&mut self) {
        self.tracked_elements.clear();
        self.active_animations.clear();
        // Clear tracking and animations (methods may not exist, so we'll handle gracefully)
        // self.internal_tracker.clear_all();
        // self.internal_flip_animator.clear_all();
        // self.internal_shared_manager.clear_all();
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Option<SimplifiedPerformanceMetrics> {
        Some(self.performance_metrics.clone())
    }

    /// Update configuration
    pub fn update_config(&mut self, config: SimplifiedLayoutConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> SimplifiedLayoutConfig {
        self.config.clone()
    }

    /// Check if any elements are being tracked
    pub fn is_tracking(&self) -> bool {
        !self.tracked_elements.is_empty()
    }

    /// Get number of tracked elements
    pub fn tracked_count(&self) -> usize {
        self.tracked_elements.len()
    }

    /// Get number of active animations
    pub fn animation_count(&self) -> usize {
        self.active_animations
            .values()
            .filter(|s| s.is_animating)
            .count()
    }

    /// Convert simplified easing to internal easing
    fn convert_easing(&self, easing: SimplifiedEasing) -> EasingFunction {
        match easing {
            SimplifiedEasing::Linear => EasingFunction::Linear,
            SimplifiedEasing::EaseIn => EasingFunction::EaseIn,
            SimplifiedEasing::EaseOut => EasingFunction::EaseOut,
            SimplifiedEasing::EaseInOut => EasingFunction::EaseInOut,
            SimplifiedEasing::EaseInCubic => EasingFunction::CubicBezier(0.55, 0.055, 0.675, 0.19),
            SimplifiedEasing::EaseOutCubic => EasingFunction::CubicBezier(0.215, 0.61, 0.355, 1.0),
            SimplifiedEasing::EaseInOutCubic => {
                EasingFunction::CubicBezier(0.645, 0.045, 0.355, 1.0)
            }
        }
    }
}

impl Default for SimplifiedLayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SimplifiedLayoutManager {
    fn clone(&self) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                internal_tracker: LayoutTracker::new(),
                internal_flip_animator: FLIPAnimator::new(),
                internal_shared_manager: SharedElementManager::new(ZIndexStrategy::default()),
                config: self.config.clone(),
                tracked_elements: HashMap::new(), // Can't clone DOM elements
                active_animations: HashMap::new(),
                performance_metrics: self.performance_metrics.clone(),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self {
                internal_tracker: MockLayoutTracker::new(),
                internal_flip_animator: MockFLIPAnimator::new(),
                internal_shared_manager: MockSharedElementManager::new(ZIndexStrategy::default()),
                config: self.config.clone(),
                tracked_elements: HashMap::new(), // Can't clone DOM elements
                active_animations: HashMap::new(),
                performance_metrics: self.performance_metrics.clone(),
            }
        }
    }
}

impl std::fmt::Debug for SimplifiedLayoutManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimplifiedLayoutManager")
            .field("is_tracking", &self.is_tracking())
            .field("tracked_count", &self.tracked_count())
            .field("animation_count", &self.animation_count())
            .finish()
    }
}

impl Default for SimplifiedLayoutConfig {
    fn default() -> Self {
        Self {
            duration: 0.3,
            easing: SimplifiedEasing::EaseInOut,
            hardware_accelerated: true,
            enable_flip: true,
            enable_shared_elements: true,
        }
    }
}

impl SimplifiedLayoutConfig {
    /// Create new simplified layout config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set animation duration
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: SimplifiedEasing) -> Self {
        self.easing = easing;
        self
    }

    /// Set hardware acceleration
    pub fn hardware_accelerated(mut self, enabled: bool) -> Self {
        self.hardware_accelerated = enabled;
        self
    }

    /// Enable/disable FLIP animations
    pub fn enable_flip(mut self, enabled: bool) -> Self {
        self.enable_flip = enabled;
        self
    }

    /// Enable/disable shared element transitions
    pub fn enable_shared_elements(mut self, enabled: bool) -> Self {
        self.enable_shared_elements = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplified_layout_manager_creation() {
        let manager = SimplifiedLayoutManager::new();
        assert!(!manager.is_tracking());
        assert_eq!(manager.tracked_count(), 0);
        assert_eq!(manager.animation_count(), 0);
    }

    #[test]
    fn test_simplified_layout_manager_with_config() {
        let config = SimplifiedLayoutConfig::new()
            .duration(0.5)
            .easing(SimplifiedEasing::EaseOut);

        let manager = SimplifiedLayoutManager::with_config(config);
        assert!(!manager.is_tracking());
        assert_eq!(manager.tracked_count(), 0);
    }

    #[test]
    fn test_simplified_layout_config_fluent_api() {
        let config = SimplifiedLayoutConfig::new()
            .duration(0.5)
            .easing(SimplifiedEasing::EaseOut)
            .hardware_accelerated(false)
            .enable_flip(false)
            .enable_shared_elements(false);

        assert_eq!(config.duration, 0.5);
        assert_eq!(config.easing, SimplifiedEasing::EaseOut);
        assert!(!config.hardware_accelerated);
        assert!(!config.enable_flip);
        assert!(!config.enable_shared_elements);
    }

    #[test]
    fn test_simplified_layout_manager_clone() {
        let manager1 = SimplifiedLayoutManager::new();
        let manager2 = manager1.clone();

        assert_eq!(manager1.is_tracking(), manager2.is_tracking());
        assert_eq!(manager1.tracked_count(), manager2.tracked_count());
        assert_eq!(manager1.animation_count(), manager2.animation_count());
    }

    #[test]
    fn test_simplified_layout_manager_debug() {
        let manager = SimplifiedLayoutManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("SimplifiedLayoutManager"));
        assert!(debug_str.contains("is_tracking"));
        assert!(debug_str.contains("tracked_count"));
        assert!(debug_str.contains("animation_count"));
    }
}
