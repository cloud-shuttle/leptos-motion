//! CSS Transition Animation
//!
//! This module implements CSS transition-based animations that avoid borrowing issues
//! by using native CSS transitions instead of JavaScript RAF loops.

use crate::animation_trait::{Animation, AnimationError, AnimationResult, animation_utils};
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

/// CSS transition-based animation
#[derive(Debug)]
pub struct CssTransitionAnimation {
    /// Unique animation ID
    id: String,
    /// DOM element to animate
    element: Element,
    /// Target properties and values
    properties: HashMap<String, AnimationValue>,
    /// Transition configuration
    transition: Transition,
    /// Animation start time
    start_time: Option<f64>,
    /// Animation duration in seconds
    duration: f64,
    /// Whether animation is complete
    is_complete: bool,
    /// Whether animation is currently running
    is_running: bool,
    /// Current progress (0.0 to 1.0)
    progress: f64,
}

impl CssTransitionAnimation {
    /// Create a new CSS transition animation
    pub fn new(
        id: String,
        element: Element,
        properties: HashMap<String, AnimationValue>,
        transition: Transition,
    ) -> Self {
        let duration = transition.duration.unwrap_or(0.3);
        
        Self {
            id,
            element,
            properties,
            transition,
            start_time: None,
            duration,
            is_complete: false,
            is_running: false,
            progress: 0.0,
        }
    }
    
    /// Apply CSS transition to element
    fn apply_css_transition(&self) -> AnimationResult<()> {
        // Cast to HtmlElement to access style
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        let style = html_element.style();
        let easing = animation_utils::easing_to_css(&self.transition.ease);
        
        // Set CSS transition property
        let _ = style.set_property("transition", &format!("all {}s {}", self.duration, easing));
        
        Ok(())
    }
    
    /// Apply target properties to element
    fn apply_target_properties(&self) -> AnimationResult<()> {
        // Cast to HtmlElement to access style
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        let style = html_element.style();
        
        // Apply each property
        for (property, value) in &self.properties {
            Self::apply_property_to_dom(html_element, property, value)?;
        }
        
        Ok(())
    }
    
    /// Apply a single property to DOM element
    fn apply_property_to_dom(
        element: &HtmlElement,
        property: &str,
        value: &AnimationValue,
    ) -> AnimationResult<()> {
        let style = element.style();
        
        match (property, value) {
            ("opacity", AnimationValue::Number(n)) => {
                let _ = style.set_property("opacity", &n.to_string());
            }
            ("scale", AnimationValue::Number(n)) => {
                let _ = style.set_property("transform", &format!("scale({})", n));
            }
            ("x", AnimationValue::Number(n)) => {
                let _ = style.set_property("transform", &format!("translateX({}px)", n));
            }
            ("y", AnimationValue::Number(n)) => {
                let _ = style.set_property("transform", &format!("translateY({}px)", n));
            }
            ("rotate", AnimationValue::Number(n)) => {
                let _ = style.set_property("transform", &format!("rotate({}deg)", n));
            }
            ("width", AnimationValue::Number(n)) => {
                let _ = style.set_property("width", &format!("{}px", n));
            }
            ("height", AnimationValue::Number(n)) => {
                let _ = style.set_property("height", &format!("{}px", n));
            }
            ("width", AnimationValue::Pixels(p)) => {
                let _ = style.set_property("width", &format!("{}px", p));
            }
            ("height", AnimationValue::Pixels(p)) => {
                let _ = style.set_property("height", &format!("{}px", p));
            }
            ("rotate", AnimationValue::Degrees(d)) => {
                let _ = style.set_property("transform", &format!("rotate({}deg)", d));
            }
            ("rotate", AnimationValue::Radians(r)) => {
                let degrees = r * 180.0 / std::f64::consts::PI;
                let _ = style.set_property("transform", &format!("rotate({}deg)", degrees));
            }
            (_, AnimationValue::String(s)) => {
                let _ = style.set_property(property, s);
            }
            (_, AnimationValue::Color(c)) => {
                let _ = style.set_property(property, c);
            }
            (_, AnimationValue::Percentage(p)) => {
                let _ = style.set_property(property, &format!("{}%", p));
            }
            _ => {
                // For other combinations, try to convert to string
                if let Some(numeric) = Self::extract_numeric_value(value) {
                    let _ = style.set_property(property, &numeric.to_string());
                }
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

impl Animation for CssTransitionAnimation {
    fn start(&mut self) -> AnimationResult<()> {
        if self.is_running {
            return Err(AnimationError::AlreadyRunning(self.id.clone()));
        }
        
        // Apply CSS transition
        self.apply_css_transition()?;
        
        // Apply target properties
        self.apply_target_properties()?;
        
        // Set start time
        self.start_time = Some(Self::get_current_time());
        self.is_running = true;
        self.is_complete = false;
        self.progress = 0.0;
        
        Ok(())
    }
    
    fn stop(&mut self) -> AnimationResult<()> {
        if !self.is_running {
            return Err(AnimationError::NotRunning(self.id.clone()));
        }
        
        // Remove CSS transition
        if let Some(html_element) = self.element.dyn_ref::<HtmlElement>() {
            let style = html_element.style();
            let _ = style.set_property("transition", "none");
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
        
        // Calculate progress based on elapsed time
        if let Some(start_time) = self.start_time {
            let current_time = Self::get_current_time();
            let elapsed = current_time - start_time;
            
            self.progress = (elapsed / self.duration).min(1.0);
            
            // Check if animation is complete
            if self.progress >= 1.0 {
                self.is_complete = true;
                self.is_running = false;
            }
        }
        
        Ok(())
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn duration(&self) -> f64 {
        self.duration
    }
    
    fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Clone for CssTransitionAnimation {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            element: self.element.clone(),
            properties: self.properties.clone(),
            transition: self.transition.clone(),
            start_time: self.start_time,
            duration: self.duration,
            is_complete: self.is_complete,
            is_running: self.is_running,
            progress: self.progress,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_css_transition_animation_creation() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let animation = CssTransitionAnimation::new(
            "test".to_string(),
            element,
            properties,
            transition,
        );
        
        assert_eq!(animation.id(), "test");
        assert_eq!(animation.duration(), 0.5);
        assert!(!animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_css_transition_animation_start() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let mut animation = CssTransitionAnimation::new(
            "test".to_string(),
            element,
            properties,
            transition,
        );
        
        // Start animation
        animation.start().unwrap();
        
        assert!(animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_css_transition_animation_stop() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(0.5));
        
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let mut animation = CssTransitionAnimation::new(
            "test".to_string(),
            element,
            properties,
            transition,
        );
        
        // Start animation
        animation.start().unwrap();
        assert!(animation.is_running());
        
        // Stop animation
        animation.stop().unwrap();
        assert!(!animation.is_running());
        assert!(animation.is_complete());
    }
}
