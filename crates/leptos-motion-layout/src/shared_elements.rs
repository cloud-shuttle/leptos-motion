//! Shared element transitions for seamless view changes
//!
//! This module provides functionality for creating smooth transitions
//! where elements appear to move between different views or states.

use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;
use web_sys::Element;

/// Shared element configuration
#[derive(Debug, Clone)]
pub struct SharedElementConfig {
    /// Transition duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: crate::flip::EasingFunction,
    /// Whether to maintain aspect ratio
    pub maintain_aspect_ratio: bool,
    /// Whether to use hardware acceleration
    pub hardware_accelerated: bool,
}

impl Default for SharedElementConfig {
    fn default() -> Self {
        Self {
            duration: 0.3,
            easing: crate::flip::EasingFunction::EaseOut,
            maintain_aspect_ratio: false,
            hardware_accelerated: true,
        }
    }
}

/// Z-index strategy for shared elements
#[derive(Debug, Clone)]
pub enum ZIndexStrategy {
    /// Use fixed z-index values
    Fixed { base: i32, increment: i32 },
    /// Use dynamic z-index based on element depth
    Dynamic { base: i32, max: i32 },
    /// Use CSS custom properties for z-index
    CustomProperty { property: String },
    /// Elevate element during transition
    Elevate,
    /// Custom z-index value
    Custom(i32),
    /// Maintain current z-index
    Maintain,
}

impl Default for ZIndexStrategy {
    fn default() -> Self {
        ZIndexStrategy::Fixed {
            base: 1000,
            increment: 1,
        }
    }
}

/// Shared element transition
#[derive(Debug)]
pub struct SharedElementTransition {
    /// Unique transition ID
    pub id: String,
    /// Source element
    pub source_element: Element,
    /// Target element
    pub target_element: Element,
    /// Transition configuration
    pub config: SharedElementConfig,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Whether transition is active
    pub active: bool,
    /// Start time
    pub start_time: f64,
}

/// Manager for shared element transitions
pub struct SharedElementManager {
    /// Active transitions
    active_transitions: HashMap<String, SharedElementTransition>,
    /// Transition queue
    transition_queue: VecDeque<QueuedTransition>,
    /// Z-index strategy
    z_index_strategy: ZIndexStrategy,
    /// Performance tracking
    performance_metrics: SharedElementMetrics,
}

/// Queued transition waiting to start
#[derive(Debug)]
pub struct QueuedTransition {
    /// Source element
    pub source_element: Element,
    /// Target element
    pub target_element: Element,
    /// Transition configuration
    pub config: SharedElementConfig,
    /// Priority level
    pub priority: TransitionPriority,
}

/// Transition priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransitionPriority {
    /// Low priority (background elements)
    Low = 0,
    /// Normal priority (default)
    Normal = 1,
    /// High priority (foreground elements)
    High = 2,
    /// Critical priority (modal overlays)
    Critical = 3,
}

/// Transition record for tracking
#[derive(Debug, Clone)]
pub struct TransitionRecord {
    /// Transition ID
    pub id: String,
    /// Start time
    pub start_time: f64,
    /// End time
    pub end_time: Option<f64>,
    /// Duration
    pub duration: f64,
    /// Success status
    pub success: bool,
    /// Performance metrics
    pub performance: SharedElementMetrics,
}

/// Shared element performance metrics
#[derive(Debug, Clone)]
pub struct SharedElementMetrics {
    /// Total transitions
    pub total_transitions: usize,
    /// Successful transitions
    pub successful_transitions: usize,
    /// Average transition duration
    pub average_duration: f64,
    /// Frame rate during transitions
    pub frame_rate: f64,
}

impl SharedElementManager {
    /// Create a new shared element manager
    pub fn new(z_index_strategy: ZIndexStrategy) -> Self {
        Self {
            active_transitions: HashMap::new(),
            transition_queue: VecDeque::new(),
            z_index_strategy,
            performance_metrics: SharedElementMetrics::default(),
        }
    }

    /// Register a shared element
    pub fn register_element(
        &mut self,
        _element: &Element,
        config: &SharedElementConfig,
    ) -> Result<(), String> {
        // For now, just log the registration
        // In a real implementation, this would set up element tracking
        log::info!("Registering shared element with config: {:?}", config);

        Ok(())
    }

    /// Unregister a shared element
    pub fn unregister_element(&mut self, _element: &Element) -> Result<(), String> {
        // For now, just log the unregistration
        // In a real implementation, this would clean up element tracking
        log::info!("Unregistering shared element");

        Ok(())
    }

    /// Create a shared element transition
    pub fn create_transition(
        &mut self,
        source_element: &Element,
        target_element: &Element,
        config: &SharedElementConfig,
    ) -> Result<String, String> {
        // Generate transition ID
        let id = self.generate_transition_id();

        // Create queued transition
        let queued = QueuedTransition {
            source_element: source_element.clone(),
            target_element: target_element.clone(),
            config: config.clone(),
            priority: TransitionPriority::Normal,
        };

        // Add to queue
        self.transition_queue.push_back(queued);

        // Try to start transitions
        self.process_transition_queue()?;

        Ok(id)
    }

    /// Start a shared element transition
    pub fn start_transition(&mut self, transition_id: &str) -> Result<(), String> {
        // Find and start the transition
        if let Some(transition) = self.active_transitions.get_mut(transition_id) {
            transition.active = true;
            transition.start_time = js_sys::Date::now();

            // Clone transition to avoid borrow checker issues
            let transition_clone = SharedElementTransition {
                id: transition.id.clone(),
                source_element: transition.source_element.clone(),
                target_element: transition.target_element.clone(),
                config: transition.config.clone(),
                progress: transition.progress,
                active: transition.active,
                start_time: transition.start_time,
            };

            // Apply initial transforms
            self.apply_initial_transforms(&transition_clone)?;

            Ok(())
        } else {
            Err("Transition not found".to_string())
        }
    }

    /// Update transition progress
    pub fn update_transition(&mut self, transition_id: &str, progress: f64) -> Result<(), String> {
        if let Some(transition) = self.active_transitions.get_mut(transition_id) {
            transition.progress = progress.clamp(0.0, 1.0);

            // Clone transition to avoid borrow checker issues
            let transition_clone = SharedElementTransition {
                id: transition.id.clone(),
                source_element: transition.source_element.clone(),
                target_element: transition.target_element.clone(),
                config: transition.config.clone(),
                progress: transition.progress,
                active: transition.active,
                start_time: transition.start_time,
            };

            // Apply transforms based on progress
            self.apply_transition_transforms(&transition_clone)?;

            // Check if transition is complete
            if progress >= 1.0 {
                self.complete_transition(transition_id)?;
            }

            Ok(())
        } else {
            Err("Transition not found".to_string())
        }
    }

    /// Cancel a transition
    pub fn cancel_transition(&mut self, transition_id: &str) -> Result<(), String> {
        if let Some(transition) = self.active_transitions.remove(transition_id) {
            // Reset element transforms
            self.reset_element_transforms(&transition.source_element)?;
            self.reset_element_transforms(&transition.target_element)?;

            // Update metrics
            self.performance_metrics.total_transitions += 1;

            Ok(())
        } else {
            Err("Transition not found".to_string())
        }
    }

    /// Cancel all active transitions
    pub fn cancel_all_transitions(&mut self) {
        self.active_transitions.clear();
        self.transition_queue.clear();
    }

    /// Process the transition queue
    fn process_transition_queue(&mut self) -> Result<(), String> {
        while let Some(queued) = self.transition_queue.pop_front() {
            // Create transition
            let transition = SharedElementTransition {
                id: self.generate_transition_id(),
                source_element: queued.source_element.clone(),
                target_element: queued.target_element.clone(),
                config: queued.config.clone(),
                progress: 0.0,
                active: false,
                start_time: 0.0,
            };

            // Add to active transitions
            let id = transition.id.clone();
            self.active_transitions.insert(id.clone(), transition);

            // Start the transition
            self.start_transition(&id)?;
        }

        Ok(())
    }

    /// Apply initial transforms for transition
    fn apply_initial_transforms(&self, transition: &SharedElementTransition) -> Result<(), String> {
        // Set up initial transform state for both elements
        self.setup_transition_element(&transition.source_element, &transition.config)?;
        self.setup_transition_element(&transition.target_element, &transition.config)?;

        log::info!(
            "Applied initial transforms for transition {}",
            transition.id
        );

        Ok(())
    }

    /// Apply transforms during transition
    fn apply_transition_transforms(
        &self,
        transition: &SharedElementTransition,
    ) -> Result<(), String> {
        // Interpolate transforms based on progress
        let progress = transition.progress;
        let inverse_progress = 1.0 - progress;

        // Apply transforms to both source and target elements
        if let Some(source_html) = transition.source_element.dyn_ref::<web_sys::HtmlElement>() {
            let style = source_html.style();
            let opacity = inverse_progress;
            style
                .set_property("opacity", &opacity.to_string())
                .map_err(|_| "Failed to set source opacity")?;
        }

        if let Some(target_html) = transition.target_element.dyn_ref::<web_sys::HtmlElement>() {
            let style = target_html.style();
            let opacity = progress;
            style
                .set_property("opacity", &opacity.to_string())
                .map_err(|_| "Failed to set target opacity")?;
        }

        log::info!(
            "Applied transition transforms for {} at {}%",
            transition.id,
            (progress * 100.0) as i32
        );

        Ok(())
    }

    /// Set up an element for transition
    fn setup_transition_element(
        &self,
        element: &Element,
        _config: &SharedElementConfig,
    ) -> Result<(), String> {
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            let style = html_element.style();
            style
                .set_property("will-change", "transform, opacity")
                .map_err(|_| "Failed to set will-change")?;

            // Use default elevated z-index for transitions
            style
                .set_property("z-index", "9999")
                .map_err(|_| "Failed to set elevated z-index")?;
        } else {
            return Err("Element is not an HtmlElement".to_string());
        }

        Ok(())
    }

    /// Complete a transition
    fn complete_transition(&mut self, transition_id: &str) -> Result<(), String> {
        if let Some(transition) = self.active_transitions.remove(transition_id) {
            // Reset element transforms
            self.reset_element_transforms(&transition.source_element)?;
            self.reset_element_transforms(&transition.target_element)?;

            // Update performance metrics
            let duration = js_sys::Date::now() - transition.start_time;
            self.update_performance_metrics(duration);

            Ok(())
        } else {
            Err("Transition not found".to_string())
        }
    }

    /// Reset element transforms
    fn reset_element_transforms(&self, element: &Element) -> Result<(), String> {
        // Reset CSS transforms on the element
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            let style = html_element.style();
            style
                .remove_property("transform")
                .map_err(|_| "Failed to remove transform")?;
            style
                .remove_property("will-change")
                .map_err(|_| "Failed to remove will-change")?;
            style
                .remove_property("z-index")
                .map_err(|_| "Failed to remove z-index")?;
        } else {
            return Err("Element is not an HtmlElement".to_string());
        }

        log::info!("Reset element transforms");
        Ok(())
    }

    /// Generate unique transition ID
    fn generate_transition_id(&self) -> String {
        format!("shared_trans_{}", js_sys::Date::now())
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self, duration: f64) {
        self.performance_metrics.total_transitions += 1;
        self.performance_metrics.successful_transitions += 1;

        // Update average duration
        let total = self.performance_metrics.total_transitions as f64;
        let current_avg = self.performance_metrics.average_duration;
        self.performance_metrics.average_duration =
            (current_avg * (total - 1.0) + duration) / total;
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &SharedElementMetrics {
        &self.performance_metrics
    }

    /// Get active transition count
    pub fn get_active_transition_count(&self) -> usize {
        self.active_transitions.len()
    }

    /// Get queued transition count
    pub fn get_queued_transition_count(&self) -> usize {
        self.transition_queue.len()
    }
}

impl Default for SharedElementManager {
    fn default() -> Self {
        Self::new(ZIndexStrategy::default())
    }
}

impl Default for SharedElementMetrics {
    fn default() -> Self {
        Self {
            total_transitions: 0,
            successful_transitions: 0,
            average_duration: 0.0,
            frame_rate: 60.0,
        }
    }
}

impl Default for TransitionPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_shared_element_config_default() {
        let config = SharedElementConfig::default();
        assert_eq!(config.duration, 0.3);
        assert!(matches!(
            config.easing,
            crate::flip::EasingFunction::EaseOut
        ));
        assert!(!config.maintain_aspect_ratio);
        assert!(config.hardware_accelerated);
    }

    #[test]
    fn test_z_index_strategy_fixed() {
        let strategy = ZIndexStrategy::Fixed {
            base: 1000,
            increment: 10,
        };
        match strategy {
            ZIndexStrategy::Fixed { base, increment } => {
                assert_eq!(base, 1000);
                assert_eq!(increment, 10);
            }
            _ => panic!("Expected Fixed strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_dynamic() {
        let strategy = ZIndexStrategy::Dynamic {
            base: 500,
            max: 2000,
        };
        match strategy {
            ZIndexStrategy::Dynamic { base, max } => {
                assert_eq!(base, 500);
                assert_eq!(max, 2000);
            }
            _ => panic!("Expected Dynamic strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_custom_property() {
        let strategy = ZIndexStrategy::CustomProperty {
            property: "z-index".to_string(),
        };
        match strategy {
            ZIndexStrategy::CustomProperty { property } => {
                assert_eq!(property, "z-index");
            }
            _ => panic!("Expected CustomProperty strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_elevate() {
        let strategy = ZIndexStrategy::Elevate;
        match strategy {
            ZIndexStrategy::Elevate => {}
            _ => panic!("Expected Elevate strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_custom() {
        let strategy = ZIndexStrategy::Custom(5000);
        match strategy {
            ZIndexStrategy::Custom(value) => assert_eq!(value, 5000),
            _ => panic!("Expected Custom strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_maintain() {
        let strategy = ZIndexStrategy::Maintain;
        match strategy {
            ZIndexStrategy::Maintain => {}
            _ => panic!("Expected Maintain strategy"),
        }
    }

    #[test]
    fn test_z_index_strategy_default() {
        let strategy = ZIndexStrategy::default();
        match strategy {
            ZIndexStrategy::Fixed { base, increment } => {
                assert_eq!(base, 1000);
                assert_eq!(increment, 1);
            }
            _ => panic!("Expected default Fixed strategy"),
        }
    }

    #[test]
    fn test_transition_priority_low() {
        let priority = TransitionPriority::Low;
        assert_eq!(priority as i32, 0);
    }

    #[test]
    fn test_transition_priority_normal() {
        let priority = TransitionPriority::Normal;
        assert_eq!(priority as i32, 1);
    }

    #[test]
    fn test_transition_priority_high() {
        let priority = TransitionPriority::High;
        assert_eq!(priority as i32, 2);
    }

    #[test]
    fn test_transition_priority_critical() {
        let priority = TransitionPriority::Critical;
        assert_eq!(priority as i32, 3);
    }

    #[test]
    fn test_transition_priority_default() {
        let priority = TransitionPriority::default();
        assert_eq!(priority, TransitionPriority::Normal);
    }

    #[test]
    fn test_shared_element_metrics_default() {
        let metrics = SharedElementMetrics::default();
        assert_eq!(metrics.total_transitions, 0);
        assert_eq!(metrics.successful_transitions, 0);
        assert_eq!(metrics.average_duration, 0.0);
        assert_eq!(metrics.frame_rate, 60.0);
    }

    #[test]
    fn test_transition_record_creation() {
        let record = TransitionRecord {
            id: "test_trans_123".to_string(),
            start_time: 1000.0,
            end_time: Some(1300.0),
            duration: 0.3,
            success: true,
            performance: SharedElementMetrics::default(),
        };

        assert_eq!(record.id, "test_trans_123");
        assert_eq!(record.start_time, 1000.0);
        assert_eq!(record.end_time, Some(1300.0));
        assert_eq!(record.duration, 0.3);
        assert!(record.success);
    }

    #[wasm_bindgen_test]
    fn test_shared_element_manager_creation() {
        let manager = SharedElementManager::new(ZIndexStrategy::default());
        assert_eq!(manager.get_active_transition_count(), 0);
        assert_eq!(manager.get_queued_transition_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_shared_element_manager_with_fixed_strategy() {
        let strategy = ZIndexStrategy::Fixed {
            base: 1000,
            increment: 10,
        };
        let manager = SharedElementManager::new(strategy);
        assert_eq!(manager.get_active_transition_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_shared_element_manager_with_dynamic_strategy() {
        let strategy = ZIndexStrategy::Dynamic {
            base: 500,
            max: 2000,
        };
        let manager = SharedElementManager::new(strategy);
        assert_eq!(manager.get_active_transition_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_transition_priority_ordering() {
        assert!(TransitionPriority::Low < TransitionPriority::Normal);
        assert!(TransitionPriority::Normal < TransitionPriority::High);
        assert!(TransitionPriority::High < TransitionPriority::Critical);
    }

    #[wasm_bindgen_test]
    fn test_transition_id_generation() {
        let manager = SharedElementManager::new(ZIndexStrategy::default());
        let id1 = manager.generate_transition_id();
        let id2 = manager.generate_transition_id();

        assert_ne!(id1, id2);
        assert!(id1.starts_with("shared_trans_"));
        assert!(id2.starts_with("shared_trans_"));
    }

    #[wasm_bindgen_test]
    fn test_manager_cancel_all_transitions() {
        let mut manager = SharedElementManager::new(ZIndexStrategy::default());
        manager.cancel_all_transitions();
        assert_eq!(manager.get_active_transition_count(), 0);
        assert_eq!(manager.get_queued_transition_count(), 0);
    }
}
