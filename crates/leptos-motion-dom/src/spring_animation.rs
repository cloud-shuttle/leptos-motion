//! Spring Animation
//!
//! This module implements spring physics-based animations that provide
//! natural, bouncy motion using spring physics simulation.

use crate::animation_trait::{Animation, AnimationError, AnimationResult, animation_utils};
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

/// Spring physics configuration
#[derive(Debug, Clone)]
pub struct SpringConfig {
    /// Spring stiffness (higher = more rigid)
    pub stiffness: f64,
    /// Damping ratio (higher = less bouncy)
    pub damping: f64,
    /// Mass of the object (higher = slower)
    pub mass: f64,
    /// Initial velocity
    pub initial_velocity: f64,
    /// Rest displacement threshold (when to stop)
    pub rest_displacement_threshold: f64,
    /// Rest velocity threshold (when to stop)
    pub rest_velocity_threshold: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            initial_velocity: 0.0,
            rest_displacement_threshold: 0.01,
            rest_velocity_threshold: 0.01,
        }
    }
}

/// Spring physics state for a single property
#[derive(Debug, Clone)]
struct SpringState {
    /// Current position
    position: f64,
    /// Current velocity
    velocity: f64,
    /// Target position
    target: f64,
    /// Initial position
    initial: f64,
}

/// Spring animation implementation
#[derive(Debug)]
pub struct SpringAnimation {
    /// Unique animation ID
    id: String,
    /// DOM element to animate
    element: Element,
    /// Target properties and values
    properties: HashMap<String, AnimationValue>,
    /// Spring configuration
    spring_config: SpringConfig,
    /// Spring states for each property
    spring_states: HashMap<String, SpringState>,
    /// Animation start time
    start_time: Option<f64>,
    /// Whether animation is complete
    is_complete: bool,
    /// Whether animation is currently running
    is_running: bool,
    /// Current progress (0.0 to 1.0)
    progress: f64,
    /// Last update time
    last_update_time: Option<f64>,
}

impl SpringAnimation {
    /// Create a new spring animation
    pub fn new(
        id: String,
        element: Element,
        properties: HashMap<String, AnimationValue>,
        spring_config: SpringConfig,
    ) -> Self {
        let mut spring_states = HashMap::new();
        
        // Initialize spring states for each property
        for (property, value) in &properties {
            if let Some(numeric_value) = Self::extract_numeric_value(value) {
                let current_value = Self::get_current_property_value(&element, property);
                
                spring_states.insert(property.clone(), SpringState {
                    position: current_value,
                    velocity: spring_config.initial_velocity,
                    target: numeric_value,
                    initial: current_value,
                });
            }
        }
        
        Self {
            id,
            element,
            properties,
            spring_config,
            spring_states,
            start_time: None,
            is_complete: false,
            is_running: false,
            progress: 0.0,
            last_update_time: None,
        }
    }
    
    /// Get current property value from DOM
    fn get_current_property_value(element: &Element, property: &str) -> f64 {
        if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
            let style = html_element.style();
            
            match property {
                "opacity" => {
                    if let Ok(value) = style.get_property_value("opacity") {
                        value.parse().unwrap_or(1.0)
                    } else {
                        1.0
                    }
                }
                "scale" => {
                    if let Ok(value) = style.get_property_value("transform") {
                        if value.contains("scale(") {
                            // Extract scale value from transform
                            if let Some(start) = value.find("scale(") {
                                if let Some(end) = value[start + 6..].find(')') {
                                    let scale_str = &value[start + 6..start + 6 + end];
                                    scale_str.parse().unwrap_or(1.0)
                                } else {
                                    1.0
                                }
                            } else {
                                1.0
                            }
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    }
                }
                "x" => {
                    if let Ok(value) = style.get_property_value("transform") {
                        if value.contains("translateX(") {
                            // Extract x value from transform
                            if let Some(start) = value.find("translateX(") {
                                if let Some(end) = value[start + 11..].find("px)") {
                                    let x_str = &value[start + 11..start + 11 + end];
                                    x_str.parse().unwrap_or(0.0)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                }
                "y" => {
                    if let Ok(value) = style.get_property_value("transform") {
                        if value.contains("translateY(") {
                            // Extract y value from transform
                            if let Some(start) = value.find("translateY(") {
                                if let Some(end) = value[start + 11..].find("px)") {
                                    let y_str = &value[start + 11..start + 11 + end];
                                    y_str.parse().unwrap_or(0.0)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                }
                "rotate" => {
                    if let Ok(value) = style.get_property_value("transform") {
                        if value.contains("rotate(") {
                            // Extract rotate value from transform
                            if let Some(start) = value.find("rotate(") {
                                if let Some(end) = value[start + 7..].find("deg)") {
                                    let rotate_str = &value[start + 7..start + 7 + end];
                                    rotate_str.parse().unwrap_or(0.0)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            }
        } else {
            0.0
        }
    }
    
    /// Apply spring physics update
    fn update_spring_physics(&mut self, delta_time: f64) -> AnimationResult<()> {
        let mut all_at_rest = true;
        
        let mut positions = HashMap::new();
        
        for (property, spring_state) in &mut self.spring_states {
            // Calculate spring force
            let displacement = spring_state.target - spring_state.position;
            let spring_force = self.spring_config.stiffness * displacement;
            
            // Calculate damping force
            let damping_force = self.spring_config.damping * spring_state.velocity;
            
            // Calculate acceleration (F = ma, so a = F/m)
            let acceleration = (spring_force - damping_force) / self.spring_config.mass;
            
            // Update velocity and position
            spring_state.velocity += acceleration * delta_time;
            spring_state.position += spring_state.velocity * delta_time;
            
            // Store position for later application
            positions.insert(property.clone(), spring_state.position);
            
            // Check if at rest
            let is_at_rest = displacement.abs() < self.spring_config.rest_displacement_threshold
                && spring_state.velocity.abs() < self.spring_config.rest_velocity_threshold;
            
            if !is_at_rest {
                all_at_rest = false;
            }
        }
        
        // Apply all positions to DOM
        for (property, position) in positions {
            self.apply_property_to_dom(&property, position)?;
        }
        
        // Update progress based on how close we are to targets
        let total_displacement: f64 = self.spring_states.values()
            .map(|state| (state.target - state.initial).abs())
            .sum();
        
        let current_displacement: f64 = self.spring_states.values()
            .map(|state| (state.target - state.position).abs())
            .sum();
        
        if total_displacement > 0.0 {
            self.progress = (1.0 - current_displacement / total_displacement).max(0.0).min(1.0);
        } else {
            self.progress = 1.0;
        }
        
        // Check if animation is complete
        if all_at_rest {
            self.is_complete = true;
            self.is_running = false;
            self.progress = 1.0;
        }
        
        Ok(())
    }
    
    /// Apply a single property to DOM element
    fn apply_property_to_dom(&self, property: &str, value: f64) -> AnimationResult<()> {
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        let style = html_element.style();
        
        match property {
            "opacity" => {
                let _ = style.set_property("opacity", &value.to_string());
            }
            "scale" => {
                let _ = style.set_property("transform", &format!("scale({})", value));
            }
            "x" => {
                let _ = style.set_property("transform", &format!("translateX({}px)", value));
            }
            "y" => {
                let _ = style.set_property("transform", &format!("translateY({}px)", value));
            }
            "rotate" => {
                let _ = style.set_property("transform", &format!("rotate({}deg)", value));
            }
            "width" => {
                let _ = style.set_property("width", &format!("{}px", value));
            }
            "height" => {
                let _ = style.set_property("height", &format!("{}px", value));
            }
            _ => {
                let _ = style.set_property(property, &value.to_string());
            }
        }
        
        Ok(())
    }
    
    /// Extract numeric value from AnimationValue
    fn extract_numeric_value(value: &AnimationValue) -> Option<f64> {
        match value {
            AnimationValue::Number(n) => Some(*n),
            AnimationValue::Pixels(p) => Some(*p),
            AnimationValue::Percentage(p) => Some(*p),
            AnimationValue::Degrees(d) => Some(*d),
            AnimationValue::Radians(r) => Some(*r),
            _ => None,
        }
    }
    
    /// Get current time in seconds
    fn get_current_time() -> f64 {
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                performance.now() / 1000.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

impl Animation for SpringAnimation {
    fn start(&mut self) -> AnimationResult<()> {
        if self.is_running {
            return Err(AnimationError::AlreadyRunning(self.id.clone()));
        }
        
        if self.spring_states.is_empty() {
            return Err(AnimationError::InvalidConfig("No spring states provided".to_string()));
        }
        
        // Set start time
        self.start_time = Some(Self::get_current_time());
        self.last_update_time = Some(Self::get_current_time());
        self.is_running = true;
        self.is_complete = false;
        self.progress = 0.0;
        
        Ok(())
    }
    
    fn stop(&mut self) -> AnimationResult<()> {
        if !self.is_running {
            return Err(AnimationError::NotRunning(self.id.clone()));
        }
        
        self.is_running = false;
        self.is_complete = true;
        
        Ok(())
    }
    
    fn is_complete(&self) -> bool {
        self.is_complete
    }
    
    fn progress(&self) -> f64 {
        self.progress
    }
    
    fn update(&mut self, _delta_time: f64) -> AnimationResult<()> {
        if !self.is_running || self.is_complete {
            return Ok(());
        }
        
        // Calculate actual delta time
        let current_time = Self::get_current_time();
        let delta_time = if let Some(last_time) = self.last_update_time {
            current_time - last_time
        } else {
            0.016 // ~60fps fallback
        };
        
        self.last_update_time = Some(current_time);
        
        // Update spring physics
        self.update_spring_physics(delta_time)?;
        
        Ok(())
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn duration(&self) -> f64 {
        // Spring animations don't have a fixed duration
        // Return estimated duration based on spring parameters
        let natural_frequency = (self.spring_config.stiffness / self.spring_config.mass).sqrt();
        let damping_ratio = self.spring_config.damping / (2.0 * (self.spring_config.stiffness * self.spring_config.mass).sqrt());
        
        if damping_ratio < 1.0 {
            // Underdamped - oscillatory
            2.0 * std::f64::consts::PI / (natural_frequency * (1.0 - damping_ratio * damping_ratio).sqrt())
        } else {
            // Overdamped or critically damped
            4.0 / (natural_frequency * damping_ratio)
        }
    }
    
    fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Clone for SpringAnimation {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            element: self.element.clone(),
            properties: self.properties.clone(),
            spring_config: self.spring_config.clone(),
            spring_states: self.spring_states.clone(),
            start_time: self.start_time,
            is_complete: self.is_complete,
            is_running: self.is_running,
            progress: self.progress,
            last_update_time: self.last_update_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_spring_animation_creation() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let spring_config = SpringConfig::default();
        
        let animation = SpringAnimation::new(
            "test".to_string(),
            element,
            properties,
            spring_config,
        );
        
        assert_eq!(animation.id(), "test");
        assert!(!animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_spring_config_default() {
        let config = SpringConfig::default();
        assert_eq!(config.stiffness, 100.0);
        assert_eq!(config.damping, 10.0);
        assert_eq!(config.mass, 1.0);
        assert_eq!(config.initial_velocity, 0.0);
        assert_eq!(config.rest_displacement_threshold, 0.01);
        assert_eq!(config.rest_velocity_threshold, 0.01);
    }
    
    #[wasm_bindgen_test]
    fn test_spring_animation_start() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let spring_config = SpringConfig::default();
        
        let mut animation = SpringAnimation::new(
            "test".to_string(),
            element,
            properties,
            spring_config,
        );
        
        // Start animation
        animation.start().unwrap();
        
        assert!(animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
}
