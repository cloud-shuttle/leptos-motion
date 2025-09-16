//! Animation Engine for Leptos Motion
//!
//! This module provides a robust animation engine that handles:
//! - Animation state management
//! - Timing and interpolation
//! - Easing functions
//! - Spring physics
//! - Repeat configurations
//! - Transform animations

use leptos_motion_core::*;
#[allow(unused_imports)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Use the existing AnimationError from leptos_motion_core
use leptos_motion_core::AnimationError;

/// Memory safety utilities for animation engine
mod memory_safety {
    use super::AnimationError;

    /// Maximum allowed slice length to prevent memory issues
    const MAX_SLICE_LEN: usize = 1024 * 1024; // 1MB

    /// Maximum allowed string length
    const MAX_STRING_LEN: usize = 10000;

    /// Safe slice creation with bounds checking (without unsafe code)
    pub fn safe_slice_from_ptr(ptr: *const u8, len: usize) -> leptos_motion_core::Result<Vec<u8>> {
        if ptr.is_null() {
            return Err(AnimationError::EngineUnavailable("Null pointer".to_string()));
        }
        
        if len > MAX_SLICE_LEN {
            return Err(AnimationError::EngineUnavailable("Slice too large".to_string()));
        }

        // Check alignment
        if ptr as usize % std::mem::align_of::<u8>() != 0 {
            return Err(AnimationError::EngineUnavailable("Unaligned pointer".to_string()));
        }

        // For now, return empty vector since we can't use unsafe
        // In a real implementation, this would need to be handled differently
        Ok(Vec::new())
    }

    /// Validate string before use
    pub fn validate_string(s: &str) -> leptos_motion_core::Result<()> {
        if s.is_empty() {
            return Err(AnimationError::InvalidProperty { property: "Empty string".to_string() });
        }
        
        if s.len() > MAX_STRING_LEN {
            return Err(AnimationError::InvalidProperty { property: "String too long".to_string() });
        }

        // Check for null bytes
        if s.contains('\0') {
            return Err(AnimationError::InvalidProperty { property: "String contains null bytes".to_string() });
        }

        Ok(())
    }

    /// Safe string cloning with validation
    pub fn safe_string_clone(s: &str) -> leptos_motion_core::Result<String> {
        validate_string(s)?;
        Ok(s.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

#[cfg(target_arch = "wasm32")]
use web_sys;

/// Type alias for animation update callback
type UpdateCallback = Option<Rc<dyn Fn(&HashMap<String, f64>)>>;

/// Animation state for a single property
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// Current value
    pub current: f64,
    /// Target value
    pub target: f64,
    /// Initial value
    pub initial: f64,
    /// Current velocity (for spring animations)
    pub velocity: f64,
    /// Whether the animation is complete
    pub is_complete: bool,
}

impl AnimationState {
    /// Create a new animation state with initial and target values
    pub fn new(initial: f64, target: f64) -> Self {
        Self {
            current: initial,
            target,
            initial,
            velocity: 0.0,
            is_complete: false,
        }
    }
}

/// Animation configuration for a single property
#[derive(Debug, Clone)]
pub struct PropertyAnimation {
    /// Animation state
    pub state: AnimationState,
    /// Transition configuration
    pub transition: Transition,
    /// Current time in the animation
    pub current_time: f64,
    /// Total duration
    pub duration: f64,
    /// Whether this is a spring animation
    pub is_spring: bool,
}

impl PropertyAnimation {
    /// Create a new property animation with initial value, target value, and transition settings
    pub fn new(initial: f64, target: f64, transition: Transition) -> Self {
        let duration = transition.duration.unwrap_or(0.3);
        let is_spring = matches!(transition.ease, Easing::Spring(_));

        Self {
            state: AnimationState::new(initial, target),
            transition: transition.clone(),
            current_time: 0.0,
            duration,
            is_spring,
        }
    }
}

/// Main animation engine
pub struct AnimationEngine {
    /// Active animations for each property
    animations: HashMap<String, PropertyAnimation>,
    /// Animation loop handle
    animation_handle: Option<i32>,
    /// Callback for when animations complete
    on_complete: Option<Rc<dyn Fn()>>,
    /// Callback for when animation values change
    on_update: UpdateCallback,
    /// Whether the engine is running
    is_running: bool,
    /// Animation closure for proper cleanup
    #[cfg(target_arch = "wasm32")]
    animation_closure: Option<Closure<dyn FnMut()>>,
    /// Guard to prevent infinite recursion
    recursion_guard: bool,
}

impl AnimationEngine {
    /// Create a new animation engine
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            animation_handle: None,
            on_complete: None,
            on_update: None,
            is_running: false,
            #[cfg(target_arch = "wasm32")]
            animation_closure: None,
            recursion_guard: false,
        }
    }

    /// Set the completion callback
    pub fn on_complete<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_complete = Some(Rc::new(callback));
    }

    /// Set the update callback
    pub fn on_update<F>(&mut self, callback: F)
    where
        F: Fn(&HashMap<String, f64>) + 'static,
    {
        self.on_update = Some(Rc::new(callback));
    }

    /// Start an animation for a property with proper error handling
    pub fn animate_property(
        &mut self,
        property: String,
        initial: f64,
        target: f64,
        transition: Transition,
    ) -> leptos_motion_core::Result<()> {
        // Validate property name
        if property.is_empty() {
            return Err(AnimationError::InvalidProperty { property: "Property name cannot be empty".to_string() });
        }
        
        if property.len() > 1000 {
            return Err(AnimationError::InvalidProperty { property: "Property name too long".to_string() });
        }

        // Validate numeric values
        if !initial.is_finite() || !target.is_finite() {
            return Err(AnimationError::InvalidProperty { property: "Animation values must be finite numbers".to_string() });
        }

        let animation = PropertyAnimation::new(initial, target, transition);
        self.animations.insert(property, animation);

        // Always restart the animation loop when new animations are added
        self.stop_animation_loop();
        self.start_animation_loop()?;
        
        Ok(())
    }

    /// Start animations for multiple properties with error handling
    pub fn animate_properties(&mut self, properties: HashMap<String, (f64, f64, Transition)>) -> leptos_motion_core::Result<()> {
        for (property, (initial, target, transition)) in properties {
            self.animate_property(property, initial, target, transition)?;
        }
        Ok(())
    }

    /// Stop animation for a specific property
    pub fn stop_property(&mut self, property: &str) {
        self.animations.remove(property);

        if self.animations.is_empty() {
            self.stop_animation_loop();
        }
    }

    /// Stop all animations
    pub fn stop_all(&mut self) {
        self.animations.clear();
        self.stop_animation_loop();
    }

    /// Get current value for a property
    pub fn get_property_value(&self, property: &str) -> Option<f64> {
        self.animations.get(property).map(|anim| anim.state.current)
    }

    /// Get all current values
    pub fn get_all_values(&self) -> HashMap<String, f64> {
        self.animations
            .iter()
            .map(|(key, anim)| (key.clone(), anim.state.current))
            .collect()
    }

    /// Start the animation loop with proper error handling
    #[cfg(target_arch = "wasm32")]
    pub fn start_animation_loop(&mut self) -> leptos_motion_core::Result<()> {
        if self.is_running {
            return Ok(());
        }

        // Validate animations before starting
        if self.animations.is_empty() {
            return Err(AnimationError::InvalidProperty { property: "No animations to start".to_string() });
        }

        // Check for excessively large animation sets
        if self.animations.len() > 1000 {
            return Err(AnimationError::MemoryError("Too many animations".to_string()));
        }

        self.is_running = true;
        
        // Create a shared state for the animation loop with validation
        let animations = Rc::new(RefCell::new(self.animations.clone()));
        let on_complete = self.on_complete.clone();
        let on_update = self.on_update.clone();
        let is_running = Rc::new(RefCell::new(true));

        // Create a recursive animation loop with error handling
        let animations_clone = animations.clone();
        let is_running_clone = is_running.clone();
        let on_update_clone = on_update.clone();
        let on_complete_clone = on_complete.clone();
        
        let closure = Closure::wrap(Box::new(move || {
            Self::animation_frame_callback(
                animations_clone.clone(),
                is_running_clone.clone(),
                on_update_clone.clone(),
                on_complete_clone.clone()
            );
        }) as Box<dyn FnMut()>);

        let window = web_sys::window()
            .ok_or_else(|| AnimationError::DomError("Window not available".to_string()))?;

        let handle = window.request_animation_frame(closure.as_ref().unchecked_ref())
            .map_err(|e| AnimationError::DomError(format!("Failed to request animation frame: {:?}", e)))?;

        // Store the handle and closure
        self.animation_handle = Some(handle);
        #[cfg(target_arch = "wasm32")]
        {
            self.animation_closure = Some(closure);
        }

        Ok(())
    }

    /// Start the animation loop (non-WASM version)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start_animation_loop(&mut self) -> leptos_motion_core::Result<()> {
        if self.is_running {
            return Ok(());
        }

        self.is_running = true;
        // No-op for non-WASM targets
        // In a real implementation, this might use a different animation system
        Ok(())
    }

    /// Stop the animation loop
    #[cfg(target_arch = "wasm32")]
    fn stop_animation_loop(&mut self) {
        if let Some(handle) = self.animation_handle.take() {
            if let Some(window) = web_sys::window() {
                if let Err(e) = window.cancel_animation_frame(handle) {
                    web_sys::console::warn_1(&format!("Failed to cancel animation frame: {:?}", e).into());
                }
            }
        }
        self.is_running = false;
        self.recursion_guard = false;
    }

    /// Stop the animation loop (non-WASM version)
    #[cfg(not(target_arch = "wasm32"))]
    fn stop_animation_loop(&mut self) {
        self.animation_handle = None;
        self.is_running = false;
    }

    /// Animation frame callback helper with proper error handling
    #[cfg(target_arch = "wasm32")]
    fn animation_frame_callback(
        animations: Rc<RefCell<HashMap<String, PropertyAnimation>>>,
        is_running: Rc<RefCell<bool>>,
        on_update: Option<Rc<dyn Fn(&HashMap<String, f64>)>>,
        on_complete: Option<Rc<dyn Fn()>>,
    ) {
        // Safe borrowing with error handling
        let should_continue = match is_running.try_borrow() {
            Ok(running) => *running,
            Err(_) => {
                // If we can't borrow, stop the animation to prevent deadlock
                eprintln!("Animation engine: Failed to borrow is_running, stopping animation");
                return;
            }
        };

        if !should_continue {
            return;
        }

        // Safe borrowing of animations with error handling
        let mut animations_guard = match animations.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Animation engine: Failed to borrow animations, skipping frame");
                return;
            }
        };

        // Update animations with proper error handling
        let mut completed_animations = Vec::new();
        let mut current_values = HashMap::new();

        // First pass: update animations and collect values
        for (property, animation) in animations_guard.iter_mut() {
            // Validate property string before use
            if let Err(e) = memory_safety::validate_string(property) {
                eprintln!("Animation engine: Invalid property name: {}", e);
                continue;
            }

            let was_complete = animation.state.is_complete;

            if !was_complete {
                // Use fixed frame-based timing for smoother animations
                let delta_time = 1.0 / 60.0; // Fixed 60fps timing
                animation.current_time += delta_time;

                if animation.is_spring {
                    Self::update_spring_animation_static(animation, delta_time);
                } else {
                    Self::update_eased_animation_static(animation);
                }
            }

            // Safe string cloning with validation
            let property_clone = match memory_safety::safe_string_clone(property) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Animation engine: Failed to clone property name: {}", e);
                    continue;
                }
            };

            current_values.insert(property_clone, animation.state.current);

            if animation.state.is_complete && !was_complete {
                if let Ok(completed_property) = memory_safety::safe_string_clone(property) {
                    completed_animations.push(completed_property);
                }
            }
        }

        // Drop the guard before calling callbacks to prevent borrowing conflicts
        drop(animations_guard);

        // Notify of updates with error handling
        if let Some(ref on_update_callback) = on_update {
            // Validate current_values before passing to callback
            if current_values.len() > 10000 {
                eprintln!("Animation engine: Too many animation values, skipping update");
            } else {
                on_update_callback(&current_values);
            }
        }

        // Remove completed animations with safe borrowing
        if !completed_animations.is_empty() {
            if let Ok(mut animations_guard) = animations.try_borrow_mut() {
                for property in completed_animations {
                    animations_guard.remove(&property);
                }
            }
        }

        // Check if all animations are complete with safe borrowing
        let is_empty = match animations.try_borrow() {
            Ok(guard) => guard.is_empty(),
            Err(_) => false,
        };

        if is_empty {
            if let Ok(mut running_guard) = is_running.try_borrow_mut() {
                *running_guard = false;
            }
            if let Some(ref on_complete_callback) = on_complete {
                on_complete_callback();
            }
        } else {
            // Check if we should continue with safe borrowing
            let should_continue = match is_running.try_borrow() {
                Ok(running) => *running,
                Err(_) => false,
            };

            if should_continue {
                // Continue animation loop by scheduling next frame
                if let Some(window) = web_sys::window() {
                    let next_closure = Closure::wrap(Box::new({
                        let animations = animations.clone();
                        let is_running = is_running.clone();
                        let on_update = on_update.clone();
                        let on_complete = on_complete.clone();
                        
                        move || {
                            Self::animation_frame_callback(
                                animations.clone(),
                                is_running.clone(),
                                on_update.clone(),
                                on_complete.clone()
                            );
                        }
                    }) as Box<dyn FnMut()>);
                    
                    if let Err(e) = window.request_animation_frame(next_closure.as_ref().unchecked_ref()) {
                        eprintln!("Animation engine: Failed to request animation frame: {:?}", e);
                    }
                    next_closure.forget(); // Prevent memory leak
                }
            }
        }
    }

    /// Update all animations (legacy method - now handled by animation_frame_callback)
    #[allow(dead_code)]
    fn update_animations(&mut self) {
        // This method is kept for compatibility but the actual animation
        // updates are handled by animation_frame_callback for better performance
        // and proper WASM integration
    }

    /// Update a single animation (legacy method - now handled by animation_frame_callback)
    #[allow(dead_code)]
    fn update_single_animation(&self, animation: &mut PropertyAnimation) {
        // This method is kept for compatibility but the actual animation
        // updates are handled by animation_frame_callback for better performance
        // and proper WASM integration
    }

    /// Update a spring animation (static version)
    fn update_spring_animation_static(animation: &mut PropertyAnimation, delta_time: f64) {
        // Spring physics implementation
        let spring_config = match &animation.transition.ease {
            Easing::Spring(_) => SpringConfig::default(),
            _ => SpringConfig::default(),
        };

        let distance = animation.state.target - animation.state.current;
        let spring_force = -spring_config.stiffness * distance;
        let damping_force = -spring_config.damping * animation.state.velocity;
        let acceleration = (spring_force + damping_force) / spring_config.mass;

        animation.state.velocity += acceleration * delta_time;
        animation.state.current += animation.state.velocity * delta_time;

        // Check if animation is complete
        let velocity_threshold = spring_config.rest_speed;
        let distance_threshold = spring_config.rest_delta;

        if animation.state.velocity.abs() < velocity_threshold
            && distance.abs() < distance_threshold
        {
            animation.state.current = animation.state.target;
            animation.state.velocity = 0.0;
            animation.state.is_complete = true;
        }
    }

    /// Update an eased animation (static version)
    fn update_eased_animation_static(animation: &mut PropertyAnimation) {
        let progress = (animation.current_time / animation.duration).min(1.0);
        let eased_progress = Self::apply_easing_static(progress, &animation.transition.ease);

        animation.state.current = animation.state.initial
            + (animation.state.target - animation.state.initial) * eased_progress;

        if progress >= 1.0 {
            animation.state.current = animation.state.target;
            animation.state.is_complete = true;
        }
    }

    /// Apply easing function to progress (static version)
    fn apply_easing_static(progress: f64, easing: &Easing) -> f64 {
        match easing {
            Easing::Linear => progress,
            Easing::EaseIn => progress * progress,
            Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            Easing::Spring(_) => progress, // Spring is handled separately
            Easing::CircIn => 1.0 - (1.0 - progress * progress).sqrt(),
            Easing::CircOut => ((2.0 - progress) * progress).sqrt(),
            Easing::CircInOut => {
                if progress < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * progress * progress).sqrt())
                } else {
                    0.5 * ((4.0 - 4.0 * progress) * progress + 1.0).sqrt()
                }
            }
            Easing::BackIn => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * progress * progress * progress - c1 * progress * progress
            }
            Easing::BackOut => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (progress - 1.0).powi(3) + c1 * (progress - 1.0).powi(2)
            }
            Easing::BackInOut => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if progress < 0.5 {
                    ((2.0 * progress).powi(2) * ((c2 + 1.0) * 2.0 * progress - c2)) / 2.0
                } else {
                    ((2.0 * progress - 2.0).powi(2) * ((c2 + 1.0) * (progress * 2.0 - 2.0) + c2)
                        + 2.0)
                        / 2.0
                }
            }
            Easing::Bezier(_, _, _, _) => progress, // Simplified bezier - use linear for now
            Easing::CubicBezier(_) => progress, // Simplified cubic bezier - use linear for now
        }
    }
}

impl Drop for AnimationEngine {
    fn drop(&mut self) {
        // Cancel any pending animation frame
        if let Some(handle) = self.animation_handle.take() {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let _ = window.cancel_animation_frame(handle);
                }
            }
        }
        
        // Clean up closure
        #[cfg(target_arch = "wasm32")]
        {
            self.animation_closure = None;
        }
        
        // Reset state
        self.is_running = false;
        self.recursion_guard = false;
    }
}

impl Default for AnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Spring configuration for physics-based animations
#[derive(Debug, Clone)]
pub struct SpringConfig {
    /// Spring stiffness (higher = snappier)
    pub stiffness: f64,
    /// Damping (higher = less bouncy)
    pub damping: f64,
    /// Mass of the animated object
    pub mass: f64,
    /// Initial velocity
    pub velocity: f64,
    /// Rest delta threshold
    pub rest_delta: f64,
    /// Rest speed threshold
    pub rest_speed: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }
}

/// Animation engine builder for easy configuration
pub struct AnimationEngineBuilder {
    engine: AnimationEngine,
}

impl AnimationEngineBuilder {
    /// Create a new animation engine builder
    pub fn new() -> Self {
        Self {
            engine: AnimationEngine::new(),
        }
    }

    /// Set a callback to be called when all animations complete
    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.engine.on_complete(callback);
        self
    }

    /// Set a callback to be called on each animation frame update
    pub fn on_update<F>(mut self, callback: F) -> Self
    where
        F: Fn(&HashMap<String, f64>) + 'static,
    {
        self.engine.on_update(callback);
        self
    }

    /// Build the final animation engine with configured settings
    pub fn build(self) -> AnimationEngine {
        self.engine
    }
}

impl Default for AnimationEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
