//! Keyframe Animation
//!
//! This module implements keyframe-based animations that support multiple
//! animation steps with different timing and easing for each keyframe.

use crate::animation_trait::{Animation, AnimationError, AnimationResult, animation_utils};
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

/// Keyframe definition
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Time offset (0.0 to 1.0)
    pub offset: f64,
    /// Properties and values at this keyframe
    pub properties: HashMap<String, AnimationValue>,
    /// Easing function for this keyframe
    pub easing: Option<Easing>,
}

/// Keyframe animation implementation
#[derive(Debug)]
pub struct KeyframeAnimation {
    /// Unique animation ID
    id: String,
    /// DOM element to animate
    element: Element,
    /// Keyframes in order
    keyframes: Vec<Keyframe>,
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
    /// Current keyframe index
    current_keyframe: usize,
}

impl KeyframeAnimation {
    /// Create a new keyframe animation
    pub fn new(
        id: String,
        element: Element,
        keyframes: Vec<Keyframe>,
        transition: Transition,
    ) -> Self {
        let duration = transition.duration.unwrap_or(1.0);
        
        // Sort keyframes by offset
        let mut sorted_keyframes = keyframes;
        sorted_keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
        
        Self {
            id,
            element,
            keyframes: sorted_keyframes,
            transition,
            start_time: None,
            duration,
            is_complete: false,
            is_running: false,
            progress: 0.0,
            current_keyframe: 0,
        }
    }
    
    /// Create keyframes from animation targets
    pub fn from_targets(
        id: String,
        element: Element,
        targets: Vec<AnimationTarget>,
        transition: Transition,
    ) -> Self {
        let mut keyframes = Vec::new();
        
        for (i, target) in targets.iter().enumerate() {
            let offset = if targets.len() == 1 {
                1.0
            } else {
                i as f64 / (targets.len() - 1) as f64
            };
            
            let mut properties = HashMap::new();
            for (property, value) in target {
                properties.insert(property.clone(), value.clone());
            }
            
            keyframes.push(Keyframe {
                offset,
                properties,
                easing: None, // Default easing for keyframes
            });
        }
        
        Self::new(id, element, keyframes, transition)
    }
    
    /// Apply CSS transition to element
    fn apply_css_transition(&self) -> AnimationResult<()> {
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        let style = html_element.style();
        let easing = animation_utils::easing_to_css(&self.transition.ease);
        
        // Set CSS transition property
        let _ = style.set_property("transition", &format!("all {}s {}", self.duration, easing));
        
        Ok(())
    }
    
    /// Apply properties for current keyframe
    fn apply_current_keyframe(&self) -> AnimationResult<()> {
        if self.current_keyframe >= self.keyframes.len() {
            return Ok(());
        }
        
        let keyframe = &self.keyframes[self.current_keyframe];
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        // Apply each property
        for (property, value) in &keyframe.properties {
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
    
    /// Find the current keyframe based on progress
    fn find_current_keyframe(&self, progress: f64) -> usize {
        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if progress <= keyframe.offset {
                return i;
            }
        }
        self.keyframes.len() - 1
    }
    
    /// Interpolate between two keyframes
    fn interpolate_keyframes(&self, from_idx: usize, to_idx: usize, local_progress: f64) -> AnimationResult<()> {
        if from_idx >= self.keyframes.len() || to_idx >= self.keyframes.len() {
            return Ok(());
        }
        
        let from_keyframe = &self.keyframes[from_idx];
        let to_keyframe = &self.keyframes[to_idx];
        
        let html_element = self.element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| AnimationError::DomError("Element is not an HtmlElement".to_string()))?;
        
        // Get all unique properties from both keyframes
        let mut all_properties = HashMap::new();
        for (property, value) in &from_keyframe.properties {
            all_properties.insert(property.clone(), value.clone());
        }
        for (property, value) in &to_keyframe.properties {
            all_properties.insert(property.clone(), value.clone());
        }
        
        // Interpolate each property
        for property in all_properties.keys() {
            let from_value = from_keyframe.properties.get(property);
            let to_value = to_keyframe.properties.get(property);
            
            if let (Some(from), Some(to)) = (from_value, to_value)
                && let (Some(from_num), Some(to_num)) = (
                    Self::extract_numeric_value(from),
                    Self::extract_numeric_value(to)
                ) {
                    let interpolated = animation_utils::interpolate(from_num, to_num, local_progress);
                    let interpolated_value = AnimationValue::Number(interpolated);
                    Self::apply_property_to_dom(html_element, property, &interpolated_value)?;
                }
        }
        
        Ok(())
    }
}

impl Animation for KeyframeAnimation {
    fn start(&mut self) -> AnimationResult<()> {
        if self.is_running {
            return Err(AnimationError::AlreadyRunning(self.id.clone()));
        }
        
        if self.keyframes.is_empty() {
            return Err(AnimationError::InvalidConfig("No keyframes provided".to_string()));
        }
        
        // Apply CSS transition
        self.apply_css_transition()?;
        
        // Apply initial keyframe
        self.current_keyframe = 0;
        self.apply_current_keyframe()?;
        
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
            
            // Find current keyframe
            let new_keyframe = self.find_current_keyframe(self.progress);
            
            if new_keyframe != self.current_keyframe {
                self.current_keyframe = new_keyframe;
                self.apply_current_keyframe()?;
            }
            
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

impl Clone for KeyframeAnimation {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            element: self.element.clone(),
            keyframes: self.keyframes.clone(),
            transition: self.transition.clone(),
            start_time: self.start_time,
            duration: self.duration,
            is_complete: self.is_complete,
            is_running: self.is_running,
            progress: self.progress,
            current_keyframe: self.current_keyframe,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_keyframe_animation_creation() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut keyframes = Vec::new();
        
        let mut properties1 = HashMap::new();
        properties1.insert("opacity".to_string(), AnimationValue::Number(0.0));
        keyframes.push(Keyframe {
            offset: 0.0,
            properties: properties1,
            easing: None,
        });
        
        let mut properties2 = HashMap::new();
        properties2.insert("opacity".to_string(), AnimationValue::Number(1.0));
        keyframes.push(Keyframe {
            offset: 1.0,
            properties: properties2,
            easing: None,
        });
        
        let transition = Transition {
            duration: Some(1.0),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let animation = KeyframeAnimation::new(
            "test".to_string(),
            element,
            keyframes,
            transition,
        );
        
        assert_eq!(animation.id(), "test");
        assert_eq!(animation.duration(), 1.0);
        assert!(!animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_keyframe_animation_start() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        
        let mut keyframes = Vec::new();
        
        let mut properties1 = HashMap::new();
        properties1.insert("opacity".to_string(), AnimationValue::Number(0.0));
        keyframes.push(Keyframe {
            offset: 0.0,
            properties: properties1,
            easing: None,
        });
        
        let mut properties2 = HashMap::new();
        properties2.insert("opacity".to_string(), AnimationValue::Number(1.0));
        keyframes.push(Keyframe {
            offset: 1.0,
            properties: properties2,
            easing: None,
        });
        
        let transition = Transition {
            duration: Some(1.0),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let mut animation = KeyframeAnimation::new(
            "test".to_string(),
            element,
            keyframes,
            transition,
        );
        
        // Start animation
        animation.start().unwrap();
        
        assert!(animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
}
