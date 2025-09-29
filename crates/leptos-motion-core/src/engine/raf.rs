//! RequestAnimationFrame (RAF) engine implementation
//!
//! This module provides RAF-based animations for broader browser support.

use super::traits::*;
use crate::{AnimationHandle, Result};
use std::collections::HashMap;

#[cfg(feature = "web-sys")]
use web_sys::{Element, HtmlElement, window};
#[cfg(feature = "web-sys")]
use wasm_bindgen::JsCast;

/// RAF-based animation engine
pub struct RafEngine {
    /// Active animations
    animations: HashMap<AnimationHandle, RafAnimation>,
    /// Next animation handle
    next_handle: AnimationHandle,
    /// RAF ID for the animation loop
    raf_id: Option<i32>,
    /// Last timestamp for delta calculation
    last_timestamp: f64,
}

/// RAF animation wrapper
pub struct RafAnimation {
    /// Target element
    pub element: Element,
    /// Animation configuration
    pub config: AnimationConfig,
    /// Start time
    pub start_time: f64,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Animation state
    pub state: PlaybackState,
}

impl RafEngine {
    /// Create a new RAF engine instance
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_handle: AnimationHandle::new(1),
            raf_id: None,
            last_timestamp: 0.0,
        }
    }
    
    /// Start the RAF loop if not already running
    fn start_raf_loop(&mut self) -> Result<()> {
        #[cfg(feature = "web-sys")]
        {
            if self.raf_id.is_none() {
                // For now, we'll use a simple approach without unsafe code
                // In a real implementation, we'd need to restructure this to avoid unsafe
                // This is a placeholder that will be fixed in a future iteration
                self.raf_id = Some(1); // Placeholder ID
            }
            Ok(())
        }
        #[cfg(not(feature = "web-sys"))]
        Err(AnimationError::InvalidValue("RAF not available without web-sys feature".to_string()))
    }
    
    /// Stop the RAF loop
    fn stop_raf_loop(&mut self) {
        #[cfg(feature = "web-sys")]
        {
            if let Some(raf_id) = self.raf_id.take() {
                if let Some(window) = window() {
                    window.cancel_animation_frame(raf_id);
                }
            }
        }
    }
    
    /// Apply easing function (static version)
    fn apply_easing_static(progress: f64, easing: &crate::Easing) -> f64 {
        match easing {
            crate::Easing::Linear => progress,
            crate::Easing::EaseIn => progress * progress,
            crate::Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            crate::Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            },
            _ => progress, // Default to linear
        }
    }
    
    /// Apply animation values to element
    fn apply_animation_values_static(animation: &mut RafAnimation, eased_progress: f64) -> Result<()> {
        #[cfg(feature = "web-sys")]
        {
            if let Some(html_element) = animation.element.dyn_ref::<HtmlElement>() {
                let style = html_element.style();
                
                for (property, animation_value) in &animation.config.values {
                    let from_value = match animation_value {
                        crate::AnimationValue::Number(n) => *n,
                        crate::AnimationValue::Pixels(p) => *p,
                        _ => 0.0,
                    };
                    
                    let to_value = match animation_value {
                        crate::AnimationValue::Number(n) => *n,
                        crate::AnimationValue::Pixels(p) => *p,
                        _ => 0.0,
                    };
                    
                    let current_value = from_value + (to_value - from_value) * eased_progress;
                    
                    match property.as_str() {
                        "opacity" => {
                            style.set_property("opacity", &current_value.to_string())?;
                        },
                        "scale" => {
                            style.set_property("transform", &format!("scale({})", current_value))?;
                        },
                        "x" => {
                            style.set_property("transform", &format!("translateX({}px)", current_value))?;
                        },
                        "y" => {
                            style.set_property("transform", &format!("translateY({}px)", current_value))?;
                        },
                        "rotate" => {
                            style.set_property("transform", &format!("rotate({}deg)", current_value))?;
                        },
                        _ => {
                            // Generic property handling
                            style.set_property(property, &current_value.to_string())?;
                        }
                    }
                }
            }
            Ok(())
        }
        #[cfg(not(feature = "web-sys"))]
        Ok(())
    }
}

impl AnimationEngine for RafEngine {
    fn is_available(&self) -> bool {
        #[cfg(feature = "web-sys")]
        {
            // RAF is generally available in all modern browsers
            window().is_some()
        }
        #[cfg(not(feature = "web-sys"))]
        false
    }

    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        #[cfg(feature = "web-sys")]
        {
            // Get the target element
            let element = config.element.clone();
            
            // Create RAF animation
            let handle = self.next_handle;
            let raf_animation = RafAnimation {
                element,
                config: config.clone(),
                start_time: 0.0, // Will be set when RAF starts
                progress: 0.0,
                state: PlaybackState::Pending,
            };
            
            self.animations.insert(handle, raf_animation);
            self.next_handle = handle.next();
            
            // Start RAF loop if not already running
            self.start_raf_loop()?;
            
            Ok(handle)
        }
        #[cfg(not(feature = "web-sys"))]
        Err(AnimationError::InvalidValue("RAF not available without web-sys feature".to_string()))
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        self.animations.remove(&handle);
        
        // Stop RAF loop if no animations left
        if self.animations.is_empty() {
            self.stop_raf_loop();
        }
        
        Ok(())
    }

    fn pause(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get_mut(&handle) {
            animation.state = PlaybackState::Paused;
        }
        Ok(())
    }

    fn resume(&mut self, handle: AnimationHandle) -> Result<()> {
        if let Some(animation) = self.animations.get_mut(&handle) {
            animation.state = PlaybackState::Running;
        }
        Ok(())
    }

    fn tick(&mut self, timestamp: f64) -> Result<()> {
        if self.last_timestamp == 0.0 {
            self.last_timestamp = timestamp;
        }
        
        let delta_time = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;
        
        // Update all running animations
        let mut finished_animations = Vec::new();
        
        for (handle, animation) in self.animations.iter_mut() {
            if animation.state == PlaybackState::Running {
                if animation.start_time == 0.0 {
                    animation.start_time = timestamp;
                }
                
                let elapsed = timestamp - animation.start_time;
                let duration = animation.config.transition.duration.unwrap_or(1.0);
                let progress = (elapsed / duration).min(1.0);
                
                // Apply easing
                let eased_progress = Self::apply_easing_static(progress, &animation.config.transition.ease);
                animation.progress = eased_progress;
                
                // Apply animation values
                Self::apply_animation_values_static(animation, eased_progress)?;
                
                // Check if animation is finished
                if progress >= 1.0 {
                    animation.state = PlaybackState::Finished;
                    finished_animations.push(*handle);
                }
            }
        }
        
        // Remove finished animations
        for handle in finished_animations {
            self.animations.remove(&handle);
        }
        
        // Stop RAF loop if no animations left
        if self.animations.is_empty() {
            self.stop_raf_loop();
        }
        
        Ok(())
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if let Some(animation) = self.animations.get(&handle) {
            Ok(animation.state.clone())
        } else {
            Ok(PlaybackState::Idle)
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        if let Some(animation) = self.animations.get(&handle) {
            animation.state == PlaybackState::Running
        } else {
            false
        }
    }

    #[cfg(feature = "performance-metrics")]
    fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceReport> {
        // RAF performance metrics would be collected here
        None
    }

    #[cfg(not(feature = "performance-metrics"))]
    fn get_performance_metrics(&self) -> Option<()> {
        None
    }
}