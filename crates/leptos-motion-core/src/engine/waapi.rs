//! Web Animations API (WAAPI) engine implementation
//!
//! This module provides WAAPI-based animations for modern browsers.

use super::traits::*;
use crate::{AnimationError, AnimationHandle, Result};
use std::collections::HashMap;

#[cfg(feature = "web-sys")]
use web_sys::{Element, Animation};

/// WAAPI-based animation engine
pub struct WaapiEngine {
    /// Active animations
    animations: HashMap<AnimationHandle, WaapiAnimation>,
    /// Next animation handle
    next_handle: AnimationHandle,
}

/// WAAPI animation wrapper
pub struct WaapiAnimation {
    /// The WAAPI animation object
    pub animation: Animation,
    /// Target element
    pub element: Element,
    /// Animation configuration
    pub config: AnimationConfig,
}

impl Default for WaapiEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl WaapiEngine {
    /// Create a new WAAPI engine instance
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_handle: AnimationHandle::new(1),
        }
    }
    
    /// Check if WAAPI is available in the current environment
    pub fn is_waapi_available() -> bool {
        #[cfg(feature = "web-sys")]
        {
            // Check if Element.animate is available
            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
                    && let Ok(element) = document.create_element("div") {
                        // Check if element has animate method by trying to access it
                        return js_sys::Reflect::has(&element, &"animate".into()).unwrap_or(false);
                    }
            false
        }
        #[cfg(not(feature = "web-sys"))]
        false
    }
    
    /// Build keyframes from animation configuration
    fn build_keyframes(&self, config: &AnimationConfig) -> Result<js_sys::Array> {
        let keyframes = js_sys::Array::new();
        
        // Create from keyframe (current values from target)
        let from_keyframe = js_sys::Object::new();
        for (property, animation_value) in &config.values {
            let value = self.animation_value_to_js(animation_value.clone())?;
            js_sys::Reflect::set(&from_keyframe, &property.into(), &value)
                .map_err(|_| AnimationError::InvalidValue("Failed to set keyframe property".to_string()))?;
        }
        keyframes.push(&from_keyframe);
        
        // Create to keyframe (target values)
        let to_keyframe = js_sys::Object::new();
        for (property, animation_value) in &config.values {
            let value = self.animation_value_to_js(animation_value.clone())?;
            js_sys::Reflect::set(&to_keyframe, &property.into(), &value)
                .map_err(|_| AnimationError::InvalidValue("Failed to set keyframe property".to_string()))?;
        }
        keyframes.push(&to_keyframe);
        
        Ok(keyframes)
    }
    
    /// Convert AnimationValue to JavaScript value
    fn animation_value_to_js(&self, value: crate::AnimationValue) -> Result<wasm_bindgen::JsValue> {
        match value {
            crate::AnimationValue::Number(n) => Ok(wasm_bindgen::JsValue::from_f64(n)),
            crate::AnimationValue::String(s) => Ok(wasm_bindgen::JsValue::from_str(&s)),
            crate::AnimationValue::Pixels(p) => Ok(wasm_bindgen::JsValue::from_str(&format!("{}px", p))),
            crate::AnimationValue::Percentage(p) => Ok(wasm_bindgen::JsValue::from_str(&format!("{}%", p))),
            crate::AnimationValue::Degrees(d) => Ok(wasm_bindgen::JsValue::from_str(&format!("{}deg", d))),
            crate::AnimationValue::Color(c) => Ok(wasm_bindgen::JsValue::from_str(&c)),
            _ => Err(AnimationError::InvalidValue("Unsupported animation value type".to_string())),
        }
    }
    
    /// Build timing options from transition
    fn build_timing_options(&self, config: &AnimationConfig) -> Result<js_sys::Object> {
        let timing = js_sys::Object::new();
        
        js_sys::Reflect::set(&timing, &"duration".into(), &config.transition.duration.into()).unwrap();
        js_sys::Reflect::set(&timing, &"delay".into(), &config.transition.delay.into()).unwrap();
        
        // Set easing
        let easing = match &config.transition.ease {
            crate::Easing::Linear => "linear",
            crate::Easing::EaseIn => "ease-in",
            crate::Easing::EaseOut => "ease-out",
            crate::Easing::EaseInOut => "ease-in-out",
            _ => "ease", // Default fallback
        };
        js_sys::Reflect::set(&timing, &"easing".into(), &easing.into()).unwrap();
        
        Ok(timing)
    }
}

impl AnimationEngine for WaapiEngine {
    fn is_available(&self) -> bool {
        Self::is_waapi_available()
    }

    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        #[cfg(feature = "web-sys")]
        {
            // Get the target element
            let element = &config.element;
            
            // Build keyframes
            let _keyframes = self.build_keyframes(config)?;
            
            // Build timing options
            let _timing = self.build_timing_options(config)?;
            
            // For now, create a placeholder animation
            // In a real implementation, we'd use the proper WAAPI bindings
            let animation = Animation::new().unwrap();
            
            // Store the animation
            let handle = self.next_handle;
            let waapi_animation = WaapiAnimation {
                animation,
                element: element.clone(),
                config: config.clone(),
            };
            
            self.animations.insert(handle, waapi_animation);
            self.next_handle = handle.next();
            
            Ok(handle)
        }
        #[cfg(not(feature = "web-sys"))]
        Err(AnimationError::InvalidValue("WAAPI not available without web-sys feature".to_string()))
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(waapi_animation) = self.animations.get(&handle) {
            waapi_animation.animation.cancel();
            self.animations.remove(&handle);
        }
        Ok(())
    }

    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(waapi_animation) = self.animations.get(&handle) {
            let _ = waapi_animation.animation.pause();
        }
        Ok(())
    }

    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(waapi_animation) = self.animations.get(&handle) {
            let _ = waapi_animation.animation.play();
        }
        Ok(())
    }

    fn tick(&mut self, _timestamp: f64) -> Result<()> {
        // WAAPI doesn't need manual ticking - it's handled by the browser
        Ok(())
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if let Some(waapi_animation) = self.animations.get(&handle) {
            // Check if animation is finished by comparing current time to duration
            let current_time = waapi_animation.animation.current_time().unwrap_or(0.0);
            let duration = 1.0; // Placeholder duration
            
            if current_time >= duration {
                Ok(PlaybackState::Finished)
            } else if current_time > 0.0 {
                Ok(PlaybackState::Running)
            } else {
                Ok(PlaybackState::Idle)
            }
        } else {
            Ok(PlaybackState::Idle)
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        if let Some(waapi_animation) = self.animations.get(&handle) {
            let current_time = waapi_animation.animation.current_time().unwrap_or(0.0);
            let duration = 1.0; // Placeholder duration
            current_time > 0.0 && current_time < duration
        } else {
            false
        }
    }

    #[cfg(feature = "performance-metrics")]
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        // WAAPI performance metrics would be collected here
        None
    }

    #[cfg(not(feature = "performance-metrics"))]
    fn get_performance_metrics(&self) -> Option<()> {
        None
    }
}
