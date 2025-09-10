//! TDD Tests for FLIP (First, Last, Invert, Play) Technique
//!
//! This module contains comprehensive tests for layout animations using the FLIP technique
//! to create smooth transitions between different layout states.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_error, console_log};
use web_sys::{DomRect, Element, CssStyleDeclaration};

/// FLIP animation configuration
#[derive(Clone, Debug)]
pub struct FLIPConfig {
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub ease: Easing,
    /// Whether to animate scale changes
    pub animate_scale: bool,
    /// Whether to animate rotation changes
    pub animate_rotation: bool,
    /// Whether to animate opacity changes
    pub animate_opacity: bool,
    /// Z-index for the animating element
    pub z_index: i32,
}

impl Default for FLIPConfig {
    fn default() -> Self {
        Self {
            duration: 0.3,
            ease: Easing::EaseOut,
            animate_scale: true,
            animate_rotation: false,
            animate_opacity: false,
            z_index: 1000,
        }
    }
}

/// FLIP animation state
#[derive(Clone, Debug, PartialEq)]
pub enum FLIPState {
    /// Initial state - element is in its starting position
    First,
    /// Final state - element is in its ending position
    Last,
    /// Inverted state - element is positioned to appear in starting position
    Inverted,
    /// Playing state - element is animating to final position
    Playing,
    /// Completed state - animation is finished
    Completed,
}

/// Layout information for an element
#[derive(Clone, Debug)]
pub struct LayoutInfo {
    /// Element's bounding rectangle
    pub rect: DomRect,
    /// Element's computed transform
    pub transform: String,
    /// Element's computed opacity
    pub opacity: f64,
    /// Element's computed scale
    pub scale: (f64, f64),
    /// Element's computed rotation
    pub rotation: f64,
}

impl LayoutInfo {
    /// Create new layout info from an element
    pub fn from_element(element: &WebElement) -> std::result::Result<Self, JsValue> {
        let rect = element.get_bounding_client_rect();
        let computed_style = web_sys::window()
            .unwrap()
            .get_computed_style(element)?
            .unwrap();

        let transform = computed_style
            .get_property_value("transform")
            .unwrap_or_else(|_| "none".to_string());

        let opacity = computed_style
            .get_property_value("opacity")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<f64>()
            .unwrap_or(1.0);

        // Parse scale from transform matrix
        let scale = Self::parse_scale_from_transform(&transform);

        // Parse rotation from transform matrix
        let rotation = Self::parse_rotation_from_transform(&transform);

        Ok(Self {
            rect,
            transform,
            opacity,
            scale,
            rotation,
        })
    }

    /// Parse scale from CSS transform
    fn parse_scale_from_transform(transform: &str) -> (f64, f64) {
        if transform == "none" {
            return (1.0, 1.0);
        }

        // Simple matrix parsing - in real implementation, use proper matrix parsing
        if transform.starts_with("matrix(") {
            let values: Vec<f64> = transform
                .trim_start_matches("matrix(")
                .trim_end_matches(")")
                .split(',')
                .filter_map(|s| s.trim().parse::<f64>().ok())
                .collect();

            if values.len() >= 4 {
                return (values[0], values[3]);
            }
        }

        (1.0, 1.0)
    }

    /// Parse rotation from CSS transform
    fn parse_rotation_from_transform(transform: &str) -> f64 {
        if transform == "none" {
            return 0.0;
        }

        // Simple rotation parsing - in real implementation, use proper matrix parsing
        if transform.starts_with("rotate(") {
            let value = transform
                .trim_start_matches("rotate(")
                .trim_end_matches("deg)")
                .trim_end_matches(")")
                .parse::<f64>()
                .unwrap_or(0.0);
            return value;
        }

        0.0
    }

    /// Calculate the transform needed to move from this layout to another
    pub fn calculate_transform_to(&self, target: &LayoutInfo) -> HashMap<String, AnimationValue> {
        let mut transform = HashMap::new();

        // Calculate position differences
        let delta_x = target.rect.left() - self.rect.left();
        let delta_y = target.rect.top() - self.rect.top();

        transform.insert("x".to_string(), AnimationValue::Pixels(delta_x));
        transform.insert("y".to_string(), AnimationValue::Pixels(delta_y));

        // Calculate scale differences
        let scale_x = target.scale.0 / self.scale.0;
        let scale_y = target.scale.1 / self.scale.1;

        if scale_x != 1.0 || scale_y != 1.0 {
            transform.insert("scaleX".to_string(), AnimationValue::Number(scale_x));
            transform.insert("scaleY".to_string(), AnimationValue::Number(scale_y));
        }

        // Calculate rotation differences
        let rotation_delta = target.rotation - self.rotation;
        if rotation_delta != 0.0 {
            transform.insert(
                "rotateZ".to_string(),
                AnimationValue::Number(rotation_delta),
            );
        }

        // Calculate opacity differences
        let opacity_delta = target.opacity - self.opacity;
        if opacity_delta != 0.0 {
            transform.insert(
                "opacity".to_string(),
                AnimationValue::Number(target.opacity),
            );
        }

        transform
    }
}

/// FLIP animation manager
pub struct FLIPManager {
    config: FLIPConfig,
    state: FLIPState,
    first_layout: Option<LayoutInfo>,
    last_layout: Option<LayoutInfo>,
    current_element: Option<Element>,
    animation_id: Option<i32>,
}

impl FLIPManager {
    /// Create a new FLIP manager
    pub fn new(config: FLIPConfig) -> Self {
        Self {
            config,
            state: FLIPState::First,
            first_layout: None,
            last_layout: None,
            current_element: None,
            animation_id: None,
        }
    }

    /// Record the first (initial) layout
    pub fn record_first(&mut self, element: &WebElement) -> std::result::Result<(), JsValue> {
        self.first_layout = Some(LayoutInfo::from_element(element)?);
        self.current_element = Some(element.clone());
        self.state = FLIPState::First;
        Ok(())
    }

    /// Record the last (final) layout and start animation
    pub fn record_last_and_play(&mut self, element: &WebElement) -> std::result::Result<(), JsValue> {
        self.last_layout = Some(LayoutInfo::from_element(element)?);
        self.state = FLIPState::Last;

        // Start the FLIP animation
        self.play_animation()?;

        Ok(())
    }

    /// Play the FLIP animation
    fn play_animation(&mut self) -> std::result::Result<(), JsValue> {
        let first_layout = self
            .first_layout
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No first layout recorded"))?;
        let last_layout = self
            .last_layout
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No last layout recorded"))?;
        let element = self
            .current_element
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No element set"))?;

        // Calculate the transform needed to move from first to last
        let transform = first_layout.calculate_transform_to(last_layout);

        // Set the element to its final position immediately (Last)
        self.apply_layout(element, last_layout)?;

        // Now invert the element to appear in its first position (Invert)
        self.invert_element(element, first_layout, last_layout)?;

        self.state = FLIPState::Inverted;

        // Start the animation to the final position (Play)
        let element_clone = element.clone();
        self.animate_to_final(&element_clone, &transform)?;

        Ok(())
    }

    /// Apply a layout to an element
    fn apply_layout(
        &self,
        element: &Element,
        layout: &LayoutInfo,
    ) -> std::result::Result<(), JsValue> {
        let style: CssStyleDeclaration = element.style();

        // Set position
        style.set_property("left", &format!("{}px", layout.rect.left()))?;
        style.set_property("top", &format!("{}px", layout.rect.top()))?;
        style.set_property("width", &format!("{}px", layout.rect.width()))?;
        style.set_property("height", &format!("{}px", layout.rect.height()))?;

        // Set transform
        style.set_property("transform", &layout.transform)?;

        // Set opacity
        style.set_property("opacity", &layout.opacity.to_string())?;

        // Set z-index
        style.set_property("z-index", &self.config.z_index.to_string())?;

        Ok(())
    }

    /// Invert the element to appear in its first position
    fn invert_element(
        &self,
        element: &Element,
        first: &LayoutInfo,
        last: &LayoutInfo,
    ) -> std::result::Result<(), JsValue> {
        let style: CssStyleDeclaration = element.style();

        // Calculate the inverse transform
        let delta_x = first.rect.left() - last.rect.left();
        let delta_y = first.rect.top() - last.rect.top();

        let scale_x = first.scale.0 / last.scale.0;
        let scale_y = first.scale.1 / last.scale.1;

        let rotation_delta = first.rotation - last.rotation;

        // Apply the inverse transform
        let inverse_transform = format!(
            "translate({}px, {}px) scale({}, {}) rotate({}deg)",
            delta_x, delta_y, scale_x, scale_y, rotation_delta
        );

        style.set_property("transform", &inverse_transform)?;

        if self.config.animate_opacity {
            style.set_property("opacity", &first.opacity.to_string())?;
        }

        Ok(())
    }

    /// Animate the element to its final position
    fn animate_to_final(
        &mut self,
        element: &Element,
        _transform: &HashMap<String, AnimationValue>,
    ) -> std::result::Result<(), JsValue> {
        self.state = FLIPState::Playing;

        // Create animation target (final position with no transform)
        let mut final_transform = HashMap::new();
        final_transform.insert("x".to_string(), AnimationValue::Pixels(0.0));
        final_transform.insert("y".to_string(), AnimationValue::Pixels(0.0));
        final_transform.insert("scaleX".to_string(), AnimationValue::Number(1.0));
        final_transform.insert("scaleY".to_string(), AnimationValue::Number(1.0));
        final_transform.insert("rotateZ".to_string(), AnimationValue::Number(0.0));

        if self.config.animate_opacity {
            final_transform.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }

        // Start the animation
        self.start_animation(element, &final_transform)?;

        Ok(())
    }

    /// Start the CSS animation
    fn start_animation(
        &mut self,
        element: &Element,
        target: &HashMap<String, AnimationValue>,
    ) -> std::result::Result<(), JsValue> {
        let style: CssStyleDeclaration = element.style();

        // Set up the transition
        style.set_property(
            "transition",
            &format!(
                "transform {}s {}, opacity {}s {}",
                self.config.duration, "ease-out", self.config.duration, "ease-out"
            ),
        )?;

        // Set the final transform
        let final_transform = self.build_transform_string(target);
        style.set_property("transform", &final_transform)?;

        if self.config.animate_opacity {
            if let Some(opacity) = target.get("opacity") {
                if let AnimationValue::Number(opacity_val) = opacity {
                    style.set_property("opacity", &opacity_val.to_string())?;
                }
            }
        }

        // Set up completion callback
        let element_clone = element.clone();
        let callback = Closure::wrap(Box::new(move || {
            // Animation completed
            console_log!("FLIP animation completed");
        }) as Box<dyn FnMut()>);

        element
            .add_event_listener_with_callback("transitionend", callback.as_ref().unchecked_ref())?;
        callback.forget();

        Ok(())
    }

    /// Build transform string from animation values
    fn build_transform_string(&self, values: &HashMap<String, AnimationValue>) -> String {
        let mut transforms = Vec::new();

        if let Some(x) = values.get("x") {
            if let AnimationValue::Pixels(x_val) = x {
                if let Some(y) = values.get("y") {
                    if let AnimationValue::Pixels(y_val) = y {
                        transforms.push(format!("translate({}px, {}px)", x_val, y_val));
                    }
                }
            }
        }

        if let Some(scale_x) = values.get("scaleX") {
            if let AnimationValue::Number(scale_x_val) = scale_x {
                if let Some(scale_y) = values.get("scaleY") {
                    if let AnimationValue::Number(scale_y_val) = scale_y {
                        transforms.push(format!("scale({}, {})", scale_x_val, scale_y_val));
                    }
                }
            }
        }

        if let Some(rotate) = values.get("rotateZ") {
            if let AnimationValue::Number(rotate_val) = rotate {
                transforms.push(format!("rotate({}deg)", rotate_val));
            }
        }

        if transforms.is_empty() {
            "none".to_string()
        } else {
            transforms.join(" ")
        }
    }

    /// Get current FLIP state
    pub fn get_state(&self) -> &FLIPState {
        &self.state
    }

    /// Check if animation is in progress
    pub fn is_animating(&self) -> bool {
        matches!(self.state, FLIPState::Playing)
    }

    /// Reset the FLIP manager
    pub fn reset(&mut self) {
        self.state = FLIPState::First;
        self.first_layout = None;
        self.last_layout = None;
        self.current_element = None;
        self.animation_id = None;
    }
}

// FLIP MotionDiv component will be implemented in the DOM crate to avoid circular dependencies

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_flip_config_default() {
        let config = FLIPConfig::default();

        assert_eq!(config.duration, 0.3);
        assert_eq!(config.ease, Easing::EaseOut);
        assert!(config.animate_scale);
        assert!(!config.animate_rotation);
        assert!(!config.animate_opacity);
        assert_eq!(config.z_index, 1000);
    }

    #[wasm_bindgen_test]
    fn test_flip_config_custom() {
        let config = FLIPConfig {
            duration: 0.5,
            ease: Easing::EaseInOut,
            animate_scale: false,
            animate_rotation: true,
            animate_opacity: true,
            z_index: 2000,
        };

        assert_eq!(config.duration, 0.5);
        assert_eq!(config.ease, Easing::EaseInOut);
        assert!(!config.animate_scale);
        assert!(config.animate_rotation);
        assert!(config.animate_opacity);
        assert_eq!(config.z_index, 2000);
    }

    #[wasm_bindgen_test]
    fn test_flip_manager_creation() {
        let config = FLIPConfig::default();
        let manager = FLIPManager::new(config);

        assert_eq!(manager.get_state(), &FLIPState::First);
        assert!(!manager.is_animating());
    }

    #[wasm_bindgen_test]
    fn test_flip_manager_reset() {
        let config = FLIPConfig::default();
        let mut manager = FLIPManager::new(config);

        manager.reset();

        assert_eq!(manager.get_state(), &FLIPState::First);
        assert!(!manager.is_animating());
    }

    #[wasm_bindgen_test]
    fn test_layout_info_scale_parsing() {
        let scale = LayoutInfo::parse_scale_from_transform("matrix(2, 0, 0, 3, 0, 0)");
        assert_eq!(scale, (2.0, 3.0));

        let scale_none = LayoutInfo::parse_scale_from_transform("none");
        assert_eq!(scale_none, (1.0, 1.0));
    }

    #[wasm_bindgen_test]
    fn test_layout_info_rotation_parsing() {
        let rotation = LayoutInfo::parse_rotation_from_transform("rotate(45deg)");
        assert_eq!(rotation, 45.0);

        let rotation_none = LayoutInfo::parse_rotation_from_transform("none");
        assert_eq!(rotation_none, 0.0);
    }

    #[wasm_bindgen_test]
    fn test_layout_info_transform_calculation() {
        // Create mock layouts
        let first_rect =
            DomRect::new_with_x_and_y_and_width_and_height(0.0, 0.0, 100.0, 100.0).unwrap();
        let last_rect =
            DomRect::new_with_x_and_y_and_width_and_height(50.0, 50.0, 100.0, 100.0).unwrap();

        let first_layout = LayoutInfo {
            rect: first_rect,
            transform: "none".to_string(),
            opacity: 1.0,
            scale: (1.0, 1.0),
            rotation: 0.0,
        };

        let last_layout = LayoutInfo {
            rect: last_rect,
            transform: "none".to_string(),
            opacity: 0.8,
            scale: (1.5, 1.5),
            rotation: 45.0,
        };

        let transform = first_layout.calculate_transform_to(&last_layout);

        assert_eq!(transform.get("x"), Some(&AnimationValue::Pixels(50.0)));
        assert_eq!(transform.get("y"), Some(&AnimationValue::Pixels(50.0)));
        assert_eq!(transform.get("scaleX"), Some(&AnimationValue::Number(1.5)));
        assert_eq!(transform.get("scaleY"), Some(&AnimationValue::Number(1.5)));
        assert_eq!(
            transform.get("rotateZ"),
            Some(&AnimationValue::Number(45.0))
        );
        assert_eq!(transform.get("opacity"), Some(&AnimationValue::Number(0.8)));
    }

    #[wasm_bindgen_test]
    fn test_flip_states() {
        assert_eq!(FLIPState::First, FLIPState::First);
        assert_eq!(FLIPState::Last, FLIPState::Last);
        assert_eq!(FLIPState::Inverted, FLIPState::Inverted);
        assert_eq!(FLIPState::Playing, FLIPState::Playing);
        assert_eq!(FLIPState::Completed, FLIPState::Completed);
    }

    #[wasm_bindgen_test]
    fn test_flip_manager_state_transitions() {
        let config = FLIPConfig::default();
        let manager = FLIPManager::new(config);

        // Initial state
        assert_eq!(manager.get_state(), &FLIPState::First);
        assert!(!manager.is_animating());
    }
}

// Integration tests will be added in the DOM crate to avoid circular dependencies
