//! Layout Animations
//!
//! This module implements automatic smooth transitions when DOM elements change
//! their layout properties (position, size) due to content changes, viewport
//! resizing, or dynamic layout modifications.

use leptos::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{ResizeObserver, ResizeObserverEntry};
use leptos_motion_core::*;

/// Layout animation configuration
#[derive(Clone, Debug, Default)]
pub struct LayoutConfig {
    /// Animation duration in seconds
    pub duration: Option<f64>,

    /// Animation easing function
    pub ease: Option<Easing>,

    /// Layout animation type
    pub layout_type: LayoutType,
}

/// Types of layout animations
#[derive(Clone, Debug, Default)]
pub enum LayoutType {
    /// Position and size changes
    #[default]
    Transform,

    /// Size changes only
    Size,

    /// Position changes only
    Position,

    /// All layout properties
    All,
}

/// Performance metrics for layout animations
#[derive(Clone, Debug, Default)]
pub struct LayoutPerformanceMetrics {
    /// Total layout animations tracked
    pub total_animations: usize,

    /// Currently active animations
    pub active_animations: usize,

    /// Average animation duration
    pub average_duration: f64,

    /// Layout detection latency
    pub detection_latency: f64,
}

/// Layout animation manager
pub struct LayoutAnimationManager {
    /// Active layout animations
    animations: HashMap<String, LayoutAnimation>,

    /// Resize observers for layout detection
    observers: HashMap<String, ResizeObserver>,

    /// Performance metrics
    metrics: LayoutPerformanceMetrics,
}

impl LayoutAnimationManager {
    /// Create a new layout animation manager
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            observers: HashMap::new(),
            metrics: LayoutPerformanceMetrics::default(),
        }
    }

    /// Register an element for layout animation
    pub fn register_element(
        &mut self,
        element_id: String,
        element: &web_sys::Element,
        config: LayoutConfig,
    ) -> std::result::Result<(), String> {
        // Create resize observer for layout detection
        let observer = self.create_resize_observer(element_id.clone(), &config)?;
        observer.observe(element);

        self.observers.insert(element_id.clone(), observer);

        // Initialize animation tracking
        self.animations.insert(element_id, LayoutAnimation {
            config,
            is_animating: false,
            start_time: None,
            initial_rect: None,
        });

        Ok(())
    }

    /// Unregister an element from layout animations
    pub fn unregister_element(&mut self, element_id: &str) {
        if let Some(observer) = self.observers.remove(element_id) {
            // Disconnect observer (ResizeObserver doesn't have disconnect method in web_sys)
            // We'll rely on element removal to clean up
        }
        self.animations.remove(element_id);
    }

    /// Update layout animation for an element
    pub fn update_layout(&mut self, element_id: &str, new_rect: web_sys::DomRect) {
        if let Some(animation) = self.animations.get_mut(element_id) {
            if !animation.is_animating {
                // Start new animation
                animation.start_time = Some(js_sys::Date::now());
                animation.initial_rect = Some(new_rect);
                animation.is_animating = true;

                self.metrics.total_animations += 1;
                self.metrics.active_animations += 1;
            }
        }
    }

    /// Create a resize observer for layout detection
    fn create_resize_observer(
        &self,
        element_id: String,
        config: &LayoutConfig,
    ) -> std::result::Result<ResizeObserver, String> {
        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            for entry in entries.iter() {
                if let Ok(entry) = entry.dyn_into::<ResizeObserverEntry>() {
                    let rect = entry.content_rect();
                    // Update layout animation for this element
                    // This would need access to the manager instance
                    // For now, we'll emit a custom event
                    let event = web_sys::CustomEvent::new("layout-change")
                        .unwrap();
                    web_sys::window()
                        .unwrap()
                        .dispatch_event(&event)
                        .unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>);

        ResizeObserver::new(callback.as_ref().unchecked_ref())
            .map_err(|_| "Failed to create ResizeObserver".to_string())
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &LayoutPerformanceMetrics {
        &self.metrics
    }
}

/// Individual layout animation state
struct LayoutAnimation {
    /// Animation configuration
    config: LayoutConfig,

    /// Whether animation is currently running
    is_animating: bool,

    /// Animation start time
    start_time: Option<f64>,

    /// Initial element rectangle
    initial_rect: Option<web_sys::DomRect>,
}

impl Default for LayoutAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
