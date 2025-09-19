//! Simple Animation Engine
//!
//! A WASM-compatible animation engine that focuses on core functionality
//! without complex time dependencies. Uses requestAnimationFrame for timing.

use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement};

/// Simple animation engine that uses requestAnimationFrame
pub struct SimpleAnimationEngine {
    /// Active animations
    animations: HashMap<String, SimpleAnimation>,
    /// Animation counter for unique IDs
    next_id: u64,
}

/// Simple animation structure
pub struct SimpleAnimation {
    /// Unique animation ID
    pub id: String,
    /// Target element
    pub element: Element,
    /// Animation properties
    pub properties: HashMap<String, AnimationProperty>,
    /// Animation duration in milliseconds
    pub duration: f64,
    /// Start time
    pub start_time: f64,
    /// Easing function
    pub easing: EasingFunction,
    /// Animation state
    pub state: AnimationState,
}

/// Animation property with from/to values
pub struct AnimationProperty {
    pub from: f64,
    pub to: f64,
    pub current: f64,
    pub unit: String,
}

/// Easing functions
#[derive(Debug, Clone)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring { stiffness: f64, damping: f64 },
}

/// Animation state
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    Pending,
    Running,
    Completed,
    Cancelled,
}

impl SimpleAnimationEngine {
    /// Create a new simple animation engine
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_id: 1,
        }
    }

    /// Start a new animation
    pub fn start_animation(
        &mut self,
        element: Element,
        properties: HashMap<String, (f64, f64, String)>, // (from, to, unit)
        duration: f64,
        easing: EasingFunction,
    ) -> String {
        let id = format!("anim_{}", self.next_id);
        self.next_id += 1;

        let start_time = Self::get_current_time();

        // Convert properties to AnimationProperty
        let mut animation_properties = HashMap::new();
        for (name, (from, to, unit)) in properties {
            animation_properties.insert(name, AnimationProperty {
                from,
                to,
                current: from,
                unit,
            });
        }

        let animation = SimpleAnimation {
            id: id.clone(),
            element,
            properties: animation_properties,
            duration,
            start_time,
            easing,
            state: AnimationState::Pending,
        };

        self.animations.insert(id.clone(), animation);
        self.schedule_update();

        id
    }

    /// Update all animations
    pub fn update(&mut self) {
        let current_time = Self::get_current_time();
        let mut completed_animations = Vec::new();

        for (id, animation) in self.animations.iter_mut() {
            if animation.state != AnimationState::Running {
                animation.state = AnimationState::Running;
            }

            let elapsed = current_time - animation.start_time;
            let progress = (elapsed / animation.duration).min(1.0);

            if progress >= 1.0 {
                // Animation completed
                animation.state = AnimationState::Completed;
                completed_animations.push(id.clone());
            } else {
                // Update animation properties directly
                let eased_progress = Self::apply_easing_static(progress, &animation.easing);

                for (name, property) in animation.properties.iter_mut() {
                    property.current = property.from + (property.to - property.from) * eased_progress;
                    
                    // Apply to DOM element
                    if let Some(html_element) = animation.element.dyn_ref::<HtmlElement>() {
                        let value = format!("{}{}", property.current, property.unit);
                        let _ = html_element.style().set_property(name, &value);
                    }
                }
            }
        }

        // Remove completed animations
        for id in completed_animations {
            self.animations.remove(&id);
        }

        // Schedule next update if there are active animations
        if !self.animations.is_empty() {
            self.schedule_update();
        }
    }

    /// Update animation properties based on progress
    fn update_animation_properties(&self, animation: &mut SimpleAnimation, progress: f64) {
        let eased_progress = self.apply_easing(progress, &animation.easing);

        for (name, property) in animation.properties.iter_mut() {
            property.current = property.from + (property.to - property.from) * eased_progress;
            
            // Apply to DOM element
            if let Some(html_element) = animation.element.dyn_ref::<HtmlElement>() {
                let value = format!("{}{}", property.current, property.unit);
                let _ = html_element.style().set_property(name, &value);
            }
        }
    }

    /// Apply easing function (static version)
    fn apply_easing_static(progress: f64, easing: &EasingFunction) -> f64 {
        match easing {
            EasingFunction::Linear => progress,
            EasingFunction::EaseIn => progress * progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            EasingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            EasingFunction::Spring { .. } => progress, // Simplified spring - use linear for now
        }
    }

    /// Apply easing function
    fn apply_easing(&self, progress: f64, easing: &EasingFunction) -> f64 {
        match easing {
            EasingFunction::Linear => progress,
            EasingFunction::EaseIn => progress * progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            EasingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            EasingFunction::Spring { stiffness, damping } => {
                // Simple spring approximation
                let omega = (stiffness / 1.0).sqrt();
                let zeta = damping / (2.0 * (stiffness * 1.0).sqrt());
                
                if zeta < 1.0 {
                    // Underdamped
                    let beta = omega * (1.0 - zeta * zeta).sqrt();
                    let phi = (zeta * omega * progress).exp() * (beta * progress).cos();
                    1.0 - phi
                } else {
                    // Overdamped or critically damped
                    let alpha = -zeta * omega + (zeta * zeta - 1.0).sqrt() * omega;
                    let beta = -zeta * omega - (zeta * zeta - 1.0).sqrt() * omega;
                    let c1 = 1.0 / (alpha - beta);
                    let c2 = -c1;
                    1.0 - (c1 * (alpha * progress).exp() + c2 * (beta * progress).exp())
                }
            }
        }
    }

    /// Get current time in milliseconds
    fn get_current_time() -> f64 {
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                performance.now()
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Schedule next animation frame update
    fn schedule_update(&self) {
        if let Some(window) = window() {
            let closure = Closure::wrap(Box::new(move || {
                // This will be called by requestAnimationFrame
                // In a real implementation, we'd need to pass the engine instance
                // For now, this is a placeholder
            }) as Box<dyn FnMut()>);
            
            if let Ok(_request_animation_frame) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
                closure.forget(); // Prevent cleanup
            }
        }
    }

    /// Cancel an animation
    pub fn cancel_animation(&mut self, id: &str) -> bool {
        if let Some(animation) = self.animations.get_mut(id) {
            animation.state = AnimationState::Cancelled;
            true
        } else {
            false
        }
    }

    /// Get animation count
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Check if engine has active animations
    pub fn has_active_animations(&self) -> bool {
        !self.animations.is_empty()
    }
}

impl Default for SimpleAnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global animation engine instance (thread-safe)
// Use a thread-local storage approach for WASM compatibility
thread_local! {
    static GLOBAL_ANIMATION_ENGINE: std::cell::RefCell<SimpleAnimationEngine> = 
        std::cell::RefCell::new(SimpleAnimationEngine::new());
}

/// Start a simple animation
pub fn start_simple_animation(
    element: Element,
    properties: HashMap<String, (f64, f64, String)>,
    duration: f64,
    easing: EasingFunction,
) -> String {
    GLOBAL_ANIMATION_ENGINE.with(|engine| {
        engine.borrow_mut().start_animation(element, properties, duration, easing)
    })
}

/// Cancel a simple animation
pub fn cancel_simple_animation(id: &str) -> bool {
    GLOBAL_ANIMATION_ENGINE.with(|engine| {
        engine.borrow_mut().cancel_animation(id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_engine_creation() {
        let engine = SimpleAnimationEngine::new();
        assert_eq!(engine.animation_count(), 0);
        assert!(!engine.has_active_animations());
    }

    #[test]
    fn test_easing_functions() {
        let engine = SimpleAnimationEngine::new();
        
        // Test linear easing
        assert_eq!(engine.apply_easing(0.5, &EasingFunction::Linear), 0.5);
        
        // Test ease in
        let ease_in_result = engine.apply_easing(0.5, &EasingFunction::EaseIn);
        assert!(ease_in_result < 0.5); // Should be less than linear
        
        // Test ease out
        let ease_out_result = engine.apply_easing(0.5, &EasingFunction::EaseOut);
        assert!(ease_out_result > 0.5); // Should be more than linear
    }
}

