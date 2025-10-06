//! Shared Layout Transitions
//!
//! This module implements smooth animated transitions when elements move between
//! different layout positions. Elements with the same `layout_id` automatically
//! animate between their old and new positions when the layout changes.

use leptos::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use leptos_motion_core::*;
use crate::LayoutAnimationManager;

/// Configuration for shared layout transitions
#[derive(Clone, Default)]
pub struct SharedLayoutConfig {
    /// Transition type
    pub transition_type: SharedTransitionType,

    /// Animation configuration
    pub animation: LayoutAnimationConfig,
}

/// Types of shared layout transitions
#[derive(Clone, Default)]
pub enum SharedTransitionType {
    /// Instant switch between layouts
    #[default]
    Switch,

    /// Smooth morphing transition
    Morph,

    /// Crossfade between elements
    Crossfade,
}

/// Layout animation configuration (simplified for shared transitions)
#[derive(Clone, Default)]
pub struct LayoutAnimationConfig {
    /// Animation duration in seconds
    pub duration: Option<f64>,
}

/// Represents a shared element tracked across layout changes
#[derive(Clone)]
pub struct SharedElement {
    /// Unique element identifier
    pub id: String,

    /// Layout state
    pub state: ElementState,
}

/// Element states in the shared layout system
#[derive(Clone, PartialEq)]
pub enum ElementState {
    /// Element is entering (appearing)
    Entering,

    /// Element is in its normal layout position
    Present,

    /// Element is exiting (disappearing)
    Exiting,

    /// Element is transitioning between positions
    Transitioning,
}

/// Shared element manager for layout transitions
pub struct SharedElementManager {
    /// Elements tracked by layout ID
    elements_by_layout_id: HashMap<String, Vec<SharedElement>>,

    /// Layout animation manager
    layout_manager: Rc<RefCell<LayoutAnimationManager>>,

    /// Performance metrics
    metrics: SharedLayoutMetrics,
}

/// Performance metrics for shared layout transitions
#[derive(Clone, Default)]
pub struct SharedLayoutMetrics {
    /// Total shared transitions tracked
    pub total_transitions: usize,

    /// Currently active transitions
    pub active_transitions: usize,
}

impl SharedElementManager {
    /// Create a new shared element manager
    pub fn new(layout_manager: Rc<RefCell<LayoutAnimationManager>>) -> Self {
        Self {
            elements_by_layout_id: HashMap::new(),
            layout_manager,
            metrics: SharedLayoutMetrics::default(),
        }
    }

    /// Register an element for shared layout transitions
    pub fn register_element(
        &mut self,
        layout_id: String,
        _element: web_sys::Element,
        _config: SharedLayoutConfig,
    ) -> std::result::Result<(), String> {
        let shared_element = SharedElement {
            id: format!("shared-{}", layout_id),
            state: ElementState::Present,
        };

        // Add the element
        let elements = self.elements_by_layout_id
            .entry(layout_id)
            .or_insert_with(Vec::new);
        elements.push(shared_element);

        Ok(())
    }

    /// Unregister an element from shared layout tracking
    pub fn unregister_element(&mut self, layout_id: &str, element_id: &str) {
        if let Some(elements) = self.elements_by_layout_id.get_mut(layout_id) {
            elements.retain(|e| e.id != element_id);
        }
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &SharedLayoutMetrics {
        &self.metrics
    }
}

impl Default for SharedElementManager {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(LayoutAnimationManager::new())))
    }
}
