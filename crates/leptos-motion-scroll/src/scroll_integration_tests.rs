//! TDD Tests for Scroll Integration
//!
//! This module contains comprehensive tests for scroll-triggered animations
//! using intersection observers and scroll progress tracking.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::{ReactiveMotionDiv, reactive_animate};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_error;
use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

/// Test configuration for scroll animations
#[derive(Clone, Debug)]
pub struct ScrollAnimationConfig {
    /// Threshold for intersection observer (0.0 to 1.0)
    pub threshold: f64,
    /// Root margin for intersection observer
    pub root_margin: String,
    /// Animation trigger point (0.0 to 1.0 of scroll progress)
    pub trigger_point: f64,
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: Easing,
}

impl Default for ScrollAnimationConfig {
    fn default() -> Self {
        Self {
            threshold: 0.1,
            root_margin: "0px".to_string(),
            trigger_point: 0.5,
            duration: 0.5,
            easing: Easing::EaseOut,
        }
    }
}

/// Scroll animation state
#[derive(Clone, Debug, PartialEq)]
pub enum ScrollAnimationState {
    /// Animation not yet triggered
    Waiting,
    /// Animation in progress
    Animating,
    /// Animation completed
    Completed,
    /// Animation reversed (scrolled back)
    Reversed,
}

/// Scroll animation manager - simplified for WASM compatibility
pub struct ScrollAnimationManager {
    config: ScrollAnimationConfig,
    state: ScrollAnimationState,
    progress: f64,
}

impl ScrollAnimationManager {
    /// Create a new scroll animation manager
    pub fn new(config: ScrollAnimationConfig) -> Self {
        Self {
            config,
            state: ScrollAnimationState::Waiting,
            progress: 0.0,
        }
    }

    /// Initialize intersection observer for an element
    pub fn observe_element(&mut self, element: &Element) -> std::result::Result<(), JsValue> {
        let config = self.config.clone();
        let mut state = self.state.clone();
        let mut progress = self.progress;

        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            for entry in entries.iter() {
                if let Some(entry) = entry.dyn_ref::<web_sys::IntersectionObserverEntry>() {
                    let is_intersecting = entry.is_intersecting();
                    let intersection_ratio = entry.intersection_ratio();

                    if is_intersecting {
                        // Element is visible, start animation
                        state = ScrollAnimationState::Animating;
                        progress = intersection_ratio;
                    } else {
                        // Element is not visible, reset state
                        state = ScrollAnimationState::Waiting;
                        progress = 0.0;
                    }
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);

        let mut init = IntersectionObserverInit::new();
        let threshold_js = js_sys::Array::new();
        threshold_js.push(&JsValue::from_f64(self.config.threshold));
        init.set_threshold(&threshold_js);
        init.set_root_margin(&self.config.root_margin);

        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &init)?;

        observer.observe(element);

        // Prevent callback from being dropped
        callback.forget();

        Ok(())
    }

    /// Get current animation state
    pub fn get_state(&self) -> &ScrollAnimationState {
        &self.state
    }

    /// Get current scroll progress
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    /// Set animation state (for testing)
    pub fn set_state(&mut self, state: ScrollAnimationState) {
        self.state = state;
    }

    /// Set progress (for testing)
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress;
    }

    /// Create animation target based on current state and progress
    pub fn create_animation_target(&self) -> AnimationTarget {
        match self.state {
            ScrollAnimationState::Waiting => {
                // Initial state - element is hidden/small
                HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(0.0)),
                    ("scale".to_string(), AnimationValue::Number(0.5)),
                    ("y".to_string(), AnimationValue::Pixels(50.0)),
                ])
            }
            ScrollAnimationState::Animating => {
                // Animate based on progress
                let eased_progress = self.config.easing.evaluate(self.progress);
                HashMap::from([
                    (
                        "opacity".to_string(),
                        AnimationValue::Number(eased_progress),
                    ),
                    (
                        "scale".to_string(),
                        AnimationValue::Number(0.5 + (eased_progress * 0.5)),
                    ),
                    (
                        "y".to_string(),
                        AnimationValue::Pixels(50.0 - (eased_progress * 50.0)),
                    ),
                ])
            }
            ScrollAnimationState::Completed => {
                // Final state - element is fully visible
                HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(1.0)),
                    ("scale".to_string(), AnimationValue::Number(1.0)),
                    ("y".to_string(), AnimationValue::Pixels(0.0)),
                ])
            }
            ScrollAnimationState::Reversed => {
                // Reversed state - element is hidden again
                HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(0.0)),
                    ("scale".to_string(), AnimationValue::Number(0.5)),
                    ("y".to_string(), AnimationValue::Pixels(50.0)),
                ])
            }
        }
    }
}

/// Scroll-triggered MotionDiv component
#[component]
pub fn ScrollMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for intersection observer
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Scroll animation configuration
    #[prop(optional)]
    scroll_config: Option<ScrollAnimationConfig>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let config = scroll_config.unwrap_or_default();
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // Create signals for animation state
    let (animation_state, _set_animation_state) = signal(ScrollAnimationState::Waiting);
    let (animation_progress, _set_animation_progress) = signal(0.0);

    // Clone config for use in closures
    let config_clone = config.clone();
    let config_transition = config.clone();

    // Initialize scroll manager when node ref is available
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let mut manager = ScrollAnimationManager::new(config_clone.clone());
            if let Err(e) = manager.observe_element(&element) {
                console_error!("Failed to observe element: {:?}", e);
            }
        }
    });

    // Create animation target based on scroll state
    let animation_target = move || {
        let state = animation_state.get();
        let progress = animation_progress.get();

        match state {
            ScrollAnimationState::Waiting => HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(0.0)),
                ("scale".to_string(), AnimationValue::Number(0.5)),
                ("y".to_string(), AnimationValue::Pixels(50.0)),
            ]),
            ScrollAnimationState::Animating => {
                let eased_progress = config.easing.evaluate(progress);
                HashMap::from([
                    (
                        "opacity".to_string(),
                        AnimationValue::Number(eased_progress),
                    ),
                    (
                        "scale".to_string(),
                        AnimationValue::Number(0.5 + (eased_progress * 0.5)),
                    ),
                    (
                        "y".to_string(),
                        AnimationValue::Pixels(50.0 - (eased_progress * 50.0)),
                    ),
                ])
            }
            ScrollAnimationState::Completed => HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(1.0)),
                ("scale".to_string(), AnimationValue::Number(1.0)),
                ("y".to_string(), AnimationValue::Pixels(0.0)),
            ]),
            ScrollAnimationState::Reversed => HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(0.0)),
                ("scale".to_string(), AnimationValue::Number(0.5)),
                ("y".to_string(), AnimationValue::Pixels(50.0)),
            ]),
        }
    };

    view! {
        <ReactiveMotionDiv
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            node_ref=node_ref
            animate=reactive_animate(animation_target)
            _transition=leptos_motion_core::Transition {
                duration: Some(config_transition.duration),
                ease: config_transition.easing,
                ..Default::default()
            }
        >
            {children()}
        </ReactiveMotionDiv>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_scroll_animation_manager_creation() {
        let config = ScrollAnimationConfig::default();
        let manager = ScrollAnimationManager::new(config);

        assert_eq!(manager.get_state(), &ScrollAnimationState::Waiting);
        assert_eq!(manager.get_progress(), 0.0);
    }

    #[wasm_bindgen_test]
    fn test_scroll_animation_target_waiting_state() {
        let config = ScrollAnimationConfig::default();
        let manager = ScrollAnimationManager::new(config);

        let target = manager.create_animation_target();

        assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(0.0)));
        assert_eq!(target.get("scale"), Some(&AnimationValue::Number(0.5)));
        assert_eq!(target.get("y"), Some(&AnimationValue::Pixels(50.0)));
    }

    #[wasm_bindgen_test]
    fn test_scroll_animation_target_completed_state() {
        let config = ScrollAnimationConfig::default();
        let mut manager = ScrollAnimationManager::new(config);

        // Simulate completed state
        manager.set_state(ScrollAnimationState::Completed);

        let target = manager.create_animation_target();

        assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));
        assert_eq!(target.get("scale"), Some(&AnimationValue::Number(1.0)));
        assert_eq!(target.get("y"), Some(&AnimationValue::Pixels(0.0)));
    }

    #[wasm_bindgen_test]
    fn test_scroll_animation_target_animating_state() {
        let config = ScrollAnimationConfig::default();
        let mut manager = ScrollAnimationManager::new(config);

        // Simulate animating state with 50% progress
        manager.set_state(ScrollAnimationState::Animating);
        manager.set_progress(0.5);

        let target = manager.create_animation_target();

        // With 50% progress and ease-out, we should have intermediate values
        let opacity = target.get("opacity").unwrap();
        if let AnimationValue::Number(opacity_val) = opacity {
            assert!(*opacity_val > 0.0 && *opacity_val < 1.0);
        } else {
            panic!("Expected opacity to be a number");
        }
    }

    #[wasm_bindgen_test]
    fn test_scroll_animation_config_default() {
        let config = ScrollAnimationConfig::default();

        assert_eq!(config.threshold, 0.1);
        assert_eq!(config.root_margin, "0px");
        assert_eq!(config.trigger_point, 0.5);
        assert_eq!(config.duration, 0.5);
        assert_eq!(config.easing, Easing::EaseOut);
    }

    #[wasm_bindgen_test]
    fn test_scroll_animation_config_custom() {
        let config = ScrollAnimationConfig {
            threshold: 0.5,
            root_margin: "100px".to_string(),
            trigger_point: 0.8,
            duration: 1.0,
            easing: Easing::EaseInOut,
        };

        assert_eq!(config.threshold, 0.5);
        assert_eq!(config.root_margin, "100px");
        assert_eq!(config.trigger_point, 0.8);
        assert_eq!(config.duration, 1.0);
        assert_eq!(config.easing, Easing::EaseInOut);
    }
}

// Integration tests will be added later when we have a working basic implementation
