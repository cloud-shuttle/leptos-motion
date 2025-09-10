//! TDD Tests for Gesture-Animation Integration
//!
//! This module contains comprehensive tests for connecting gesture recognition
//! to animation system for smooth, responsive user interactions.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_error, console_log};
use web_sys::{Element, MouseEvent, PointerEvent, TouchEvent, CssStyleDeclaration};

/// Gesture animation configuration
#[derive(Clone, Debug)]
pub struct GestureAnimationConfig {
    /// Whether drag gestures are enabled
    pub drag_enabled: bool,
    /// Whether hover gestures are enabled
    pub hover_enabled: bool,
    /// Whether tap gestures are enabled
    pub tap_enabled: bool,
    /// Whether pinch/zoom gestures are enabled
    pub pinch_enabled: bool,
    /// Animation duration for gesture transitions
    pub transition_duration: f64,
    /// Easing function for gesture animations
    pub easing: Easing,
    /// Spring physics configuration
    pub spring_config: SpringConfig,
    /// Drag constraints
    pub drag_constraints: DragConstraints,
    /// Hover animation properties
    pub hover_properties: HashMap<String, AnimationValue>,
    /// Tap animation properties
    pub tap_properties: HashMap<String, AnimationValue>,
}

impl Default for GestureAnimationConfig {
    fn default() -> Self {
        let mut hover_props = HashMap::new();
        hover_props.insert("scale".to_string(), AnimationValue::Number(1.05));
        hover_props.insert("opacity".to_string(), AnimationValue::Number(0.9));

        let mut tap_props = HashMap::new();
        tap_props.insert("scale".to_string(), AnimationValue::Number(0.95));
        tap_props.insert("opacity".to_string(), AnimationValue::Number(0.8));

        Self {
            drag_enabled: true,
            hover_enabled: true,
            tap_enabled: true,
            pinch_enabled: false,
            transition_duration: 0.2,
            easing: Easing::EaseOut,
            spring_config: SpringConfig::default(),
            drag_constraints: DragConstraints::default(),
            hover_properties: hover_props,
            tap_properties: tap_props,
        }
    }
}

/// Drag constraints for gesture animations
#[derive(Clone, Debug)]
pub struct DragConstraints {
    /// Minimum X position
    pub min_x: Option<f64>,
    /// Maximum X position
    pub max_x: Option<f64>,
    /// Minimum Y position
    pub min_y: Option<f64>,
    /// Maximum Y position
    pub max_y: Option<f64>,
    /// Whether to constrain to parent bounds
    pub constrain_to_parent: bool,
}

impl Default for DragConstraints {
    fn default() -> Self {
        Self {
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            constrain_to_parent: false,
        }
    }
}

/// Gesture state for animation integration
#[derive(Clone, Debug, PartialEq)]
pub enum GestureState {
    /// No active gesture
    Idle,
    /// Hovering over element
    Hovering,
    /// Dragging element
    Dragging,
    /// Tapping element
    Tapping,
    /// Pinching/zooming element
    Pinching,
    /// Gesture completed, returning to idle
    Returning,
}

/// Gesture data for animation calculations
#[derive(Clone, Debug)]
pub struct GestureData {
    /// Current gesture state
    pub state: GestureState,
    /// Current position (x, y)
    pub position: (f64, f64),
    /// Previous position (x, y)
    pub previous_position: (f64, f64),
    /// Velocity (x, y)
    pub velocity: (f64, f64),
    /// Scale factor for pinch gestures
    pub scale: f64,
    /// Rotation angle for rotation gestures
    pub rotation: f64,
    /// Gesture start time
    pub start_time: f64,
    /// Gesture duration
    pub duration: f64,
}

impl Default for GestureData {
    fn default() -> Self {
        Self {
            state: GestureState::Idle,
            position: (0.0, 0.0),
            previous_position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
            start_time: 0.0,
            duration: 0.0,
        }
    }
}

/// Gesture animation manager
pub struct GestureAnimationManager {
    config: GestureAnimationConfig,
    gesture_data: GestureData,
    element: Option<Element>,
    animation_target: HashMap<String, AnimationValue>,
    is_animating: bool,
    animation_id: Option<i32>,
}

impl GestureAnimationManager {
    /// Create a new gesture animation manager
    pub fn new(config: GestureAnimationConfig) -> Self {
        Self {
            config,
            gesture_data: GestureData::default(),
            element: None,
            animation_target: HashMap::new(),
            is_animating: false,
            animation_id: None,
        }
    }

    /// Set the target element for gestures
    pub fn set_element(&mut self, element: &Element) {
        self.element = Some(element.clone());
    }

    /// Handle mouse enter event (hover start)
    pub fn handle_mouse_enter(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if !self.config.hover_enabled {
            return Ok(());
        }

        self.gesture_data.state = GestureState::Hovering;
        self.gesture_data.start_time = js_sys::Date::now();

        // Apply hover animation
        self.apply_hover_animation()?;

        Ok(())
    }

    /// Handle mouse leave event (hover end)
    pub fn handle_mouse_leave(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if !self.config.hover_enabled {
            return Ok(());
        }

        self.gesture_data.state = GestureState::Returning;

        // Return to idle state
        self.return_to_idle()?;

        Ok(())
    }

    /// Handle mouse down event (drag start)
    pub fn handle_mouse_down(&mut self, event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if !self.config.drag_enabled {
            return Ok(());
        }

        self.gesture_data.state = GestureState::Dragging;
        self.gesture_data.start_time = js_sys::Date::now();
        self.gesture_data.position = (event.client_x() as f64, event.client_y() as f64);
        self.gesture_data.previous_position = self.gesture_data.position;

        // Start drag animation
        self.start_drag_animation()?;

        Ok(())
    }

    /// Handle mouse move event (drag update)
    pub fn handle_mouse_move(&mut self, event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.gesture_data.state != GestureState::Dragging {
            return Ok(());
        }

        let new_position = (event.client_x() as f64, event.client_y() as f64);
        let delta_x = new_position.0 - self.gesture_data.position.0;
        let delta_y = new_position.1 - self.gesture_data.position.1;

        // Update velocity
        let current_time = js_sys::Date::now();
        let time_delta = current_time - self.gesture_data.start_time;
        if time_delta > 0.0 {
            self.gesture_data.velocity = (delta_x / time_delta, delta_y / time_delta);
        }

        self.gesture_data.previous_position = self.gesture_data.position;
        self.gesture_data.position = new_position;

        // Update drag animation
        self.update_drag_animation(delta_x, delta_y)?;

        Ok(())
    }

    /// Handle mouse up event (drag end)
    pub fn handle_mouse_up(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.gesture_data.state != GestureState::Dragging {
            return Ok(());
        }

        self.gesture_data.state = GestureState::Returning;

        // Apply spring animation based on velocity
        self.apply_spring_animation()?;

        Ok(())
    }

    /// Handle click event (tap)
    pub fn handle_click(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if !self.config.tap_enabled {
            return Ok(());
        }

        self.gesture_data.state = GestureState::Tapping;
        self.gesture_data.start_time = js_sys::Date::now();

        // Apply tap animation
        self.apply_tap_animation()?;

        Ok(())
    }

    /// Apply hover animation
    fn apply_hover_animation(&mut self) -> std::result::Result<(), JsValue> {
        self.animation_target = self.config.hover_properties.clone();
        self.is_animating = true;

        // Start CSS transition
        self.start_css_transition()?;

        Ok(())
    }

    /// Apply tap animation
    fn apply_tap_animation(&mut self) -> std::result::Result<(), JsValue> {
        self.animation_target = self.config.tap_properties.clone();
        self.is_animating = true;

        // Start CSS transition
        self.start_css_transition()?;

        // Schedule return to idle after tap duration
        let element = self
            .element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;
        let callback = Closure::wrap(Box::new(move || {
            console_log!("Tap animation completed");
        }) as Box<dyn FnMut()>);

        element
            .add_event_listener_with_callback("transitionend", callback.as_ref().unchecked_ref())?;
        callback.forget();

        Ok(())
    }

    /// Start drag animation
    fn start_drag_animation(&mut self) -> std::result::Result<(), JsValue> {
        self.animation_target = HashMap::new();
        self.is_animating = true;

        // Disable CSS transitions during drag
        self.disable_css_transitions()?;

        Ok(())
    }

    /// Update drag animation
    fn update_drag_animation(
        &mut self,
        delta_x: f64,
        delta_y: f64,
    ) -> std::result::Result<(), JsValue> {
        let element = self
            .element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;
        let style: CssStyleDeclaration = element.style();

        // Apply drag constraints
        let (constrained_x, constrained_y) = self.apply_drag_constraints(delta_x, delta_y);

        // Update element position
        style.set_property(
            "transform",
            &format!("translate({}px, {}px)", constrained_x, constrained_y),
        )?;

        Ok(())
    }

    /// Apply spring animation
    fn apply_spring_animation(&mut self) -> std::result::Result<(), JsValue> {
        let element = self
            .element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;
        let style: CssStyleDeclaration = element.style();

        // Re-enable CSS transitions
        self.enable_css_transitions()?;

        // Apply spring physics to return to original position
        let spring_target = self.calculate_spring_target();
        style.set_property("transform", &spring_target)?;

        Ok(())
    }

    /// Return to idle state
    fn return_to_idle(&mut self) -> std::result::Result<(), JsValue> {
        self.animation_target = HashMap::new();
        self.is_animating = true;

        // Start CSS transition to return to original state
        self.start_css_transition()?;

        Ok(())
    }

    /// Apply drag constraints
    pub fn apply_drag_constraints(&self, delta_x: f64, delta_y: f64) -> (f64, f64) {
        let mut constrained_x = delta_x;
        let mut constrained_y = delta_y;

        if let Some(min_x) = self.config.drag_constraints.min_x {
            constrained_x = constrained_x.max(min_x);
        }
        if let Some(max_x) = self.config.drag_constraints.max_x {
            constrained_x = constrained_x.min(max_x);
        }
        if let Some(min_y) = self.config.drag_constraints.min_y {
            constrained_y = constrained_y.max(min_y);
        }
        if let Some(max_y) = self.config.drag_constraints.max_y {
            constrained_y = constrained_y.min(max_y);
        }

        (constrained_x, constrained_y)
    }

    /// Calculate spring target position
    pub fn calculate_spring_target(&self) -> String {
        // Simple spring calculation - in real implementation, use proper spring physics
        let velocity_factor = 0.1;
        let spring_x = self.gesture_data.velocity.0 * velocity_factor;
        let spring_y = self.gesture_data.velocity.1 * velocity_factor;

        format!("translate({}px, {}px)", spring_x, spring_y)
    }

    /// Start CSS transition
    fn start_css_transition(&mut self) -> std::result::Result<(), JsValue> {
        let element = self
            .element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;
        let style: CssStyleDeclaration = element.style();

        style.set_property(
            "transition",
            &format!(
                "transform {}s ease-out, opacity {}s ease-out, scale {}s ease-out",
                self.config.transition_duration,
                self.config.transition_duration,
                self.config.transition_duration
            ),
        )?;

        Ok(())
    }

    /// Disable CSS transitions
    fn disable_css_transitions(&mut self) -> std::result::Result<(), JsValue> {
        let element = self
            .element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;
        let style: CssStyleDeclaration = element.style();

        style.set_property("transition", "none")?;

        Ok(())
    }

    /// Enable CSS transitions
    fn enable_css_transitions(&mut self) -> std::result::Result<(), JsValue> {
        self.start_css_transition()
    }

    /// Get current gesture state
    pub fn get_gesture_state(&self) -> &GestureState {
        &self.gesture_data.state
    }

    /// Get current animation target
    pub fn get_animation_target(&self) -> &HashMap<String, AnimationValue> {
        &self.animation_target
    }

    /// Check if currently animating
    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    /// Reset gesture manager
    pub fn reset(&mut self) {
        self.gesture_data = GestureData::default();
        self.animation_target = HashMap::new();
        self.is_animating = false;
        self.animation_id = None;
    }
}

// Gesture MotionDiv component will be implemented in the DOM crate to avoid circular dependencies

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_gesture_animation_config_default() {
        let config = GestureAnimationConfig::default();

        assert!(config.drag_enabled);
        assert!(config.hover_enabled);
        assert!(config.tap_enabled);
        assert!(!config.pinch_enabled);
        assert_eq!(config.transition_duration, 0.2);
        assert_eq!(config.easing, Easing::EaseOut);
    }

    #[wasm_bindgen_test]
    fn test_gesture_animation_config_custom() {
        let mut hover_props = HashMap::new();
        hover_props.insert("scale".to_string(), AnimationValue::Number(1.1));

        let config = GestureAnimationConfig {
            drag_enabled: false,
            hover_enabled: true,
            tap_enabled: false,
            pinch_enabled: true,
            transition_duration: 0.5,
            easing: Easing::EaseInOut,
            spring_config: SpringConfig::default(),
            drag_constraints: DragConstraints::default(),
            hover_properties: hover_props,
            tap_properties: HashMap::new(),
        };

        assert!(!config.drag_enabled);
        assert!(config.hover_enabled);
        assert!(!config.tap_enabled);
        assert!(config.pinch_enabled);
        assert_eq!(config.transition_duration, 0.5);
        assert_eq!(config.easing, Easing::EaseInOut);
    }

    #[wasm_bindgen_test]
    fn test_drag_constraints_default() {
        let constraints = DragConstraints::default();

        assert_eq!(constraints.min_x, None);
        assert_eq!(constraints.max_x, None);
        assert_eq!(constraints.min_y, None);
        assert_eq!(constraints.max_y, None);
        assert!(!constraints.constrain_to_parent);
    }

    #[wasm_bindgen_test]
    fn test_drag_constraints_custom() {
        let constraints = DragConstraints {
            min_x: Some(-100.0),
            max_x: Some(100.0),
            min_y: Some(-50.0),
            max_y: Some(50.0),
            constrain_to_parent: true,
        };

        assert_eq!(constraints.min_x, Some(-100.0));
        assert_eq!(constraints.max_x, Some(100.0));
        assert_eq!(constraints.min_y, Some(-50.0));
        assert_eq!(constraints.max_y, Some(50.0));
        assert!(constraints.constrain_to_parent);
    }

    #[wasm_bindgen_test]
    fn test_gesture_data_default() {
        let data = GestureData::default();

        assert_eq!(data.state, GestureState::Idle);
        assert_eq!(data.position, (0.0, 0.0));
        assert_eq!(data.previous_position, (0.0, 0.0));
        assert_eq!(data.velocity, (0.0, 0.0));
        assert_eq!(data.scale, 1.0);
        assert_eq!(data.rotation, 0.0);
        assert_eq!(data.start_time, 0.0);
        assert_eq!(data.duration, 0.0);
    }

    #[wasm_bindgen_test]
    fn test_gesture_animation_manager_creation() {
        let config = GestureAnimationConfig::default();
        let manager = GestureAnimationManager::new(config);

        assert_eq!(manager.get_gesture_state(), &GestureState::Idle);
        assert!(!manager.is_animating());
        assert!(manager.get_animation_target().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_gesture_animation_manager_reset() {
        let config = GestureAnimationConfig::default();
        let mut manager = GestureAnimationManager::new(config);

        manager.reset();

        assert_eq!(manager.get_gesture_state(), &GestureState::Idle);
        assert!(!manager.is_animating());
        assert!(manager.get_animation_target().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_gesture_states() {
        assert_eq!(GestureState::Idle, GestureState::Idle);
        assert_eq!(GestureState::Hovering, GestureState::Hovering);
        assert_eq!(GestureState::Dragging, GestureState::Dragging);
        assert_eq!(GestureState::Tapping, GestureState::Tapping);
        assert_eq!(GestureState::Pinching, GestureState::Pinching);
        assert_eq!(GestureState::Returning, GestureState::Returning);
    }

    #[wasm_bindgen_test]
    fn test_drag_constraints_application() {
        let config = GestureAnimationConfig {
            drag_constraints: DragConstraints {
                min_x: Some(-50.0),
                max_x: Some(50.0),
                min_y: Some(-25.0),
                max_y: Some(25.0),
                constrain_to_parent: false,
            },
            ..Default::default()
        };

        let manager = GestureAnimationManager::new(config);

        // Test constraint application
        let (constrained_x, constrained_y) = manager.apply_drag_constraints(100.0, 100.0);
        assert_eq!(constrained_x, 50.0); // Clamped to max_x
        assert_eq!(constrained_y, 25.0); // Clamped to max_y

        let (constrained_x, constrained_y) = manager.apply_drag_constraints(-100.0, -100.0);
        assert_eq!(constrained_x, -50.0); // Clamped to min_x
        assert_eq!(constrained_y, -25.0); // Clamped to min_y
    }

    #[wasm_bindgen_test]
    fn test_spring_target_calculation() {
        let config = GestureAnimationConfig::default();
        let mut manager = GestureAnimationManager::new(config);

        // Set some velocity
        manager.gesture_data.velocity = (10.0, 20.0);

        let spring_target = manager.calculate_spring_target();
        assert!(spring_target.contains("translate"));
        assert!(spring_target.contains("1px")); // 10.0 * 0.1
        assert!(spring_target.contains("2px")); // 20.0 * 0.1
    }
}

// Integration tests will be added in the DOM crate to avoid circular dependencies
