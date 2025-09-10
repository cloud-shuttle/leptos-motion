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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

    /// Start an animation for a property
    pub fn animate_property(
        &mut self,
        property: String,
        initial: f64,
        target: f64,
        transition: Transition,
    ) {
        let animation = PropertyAnimation::new(initial, target, transition);
        self.animations.insert(property, animation);

        if !self.is_running {
            self.start_animation_loop();
        }
    }

    /// Start animations for multiple properties
    pub fn animate_properties(&mut self, properties: HashMap<String, (f64, f64, Transition)>) {
        for (property, (initial, target, transition)) in properties {
            self.animate_property(property, initial, target, transition);
        }
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

    /// Start the animation loop
    #[cfg(target_arch = "wasm32")]
    fn start_animation_loop(&mut self) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        let engine_ref = Rc::new(RefCell::new(self));
        let engine_clone = engine_ref.clone();

        let closure = Closure::wrap(Box::new(move || {
            let mut engine = engine_clone.borrow_mut();
            engine.update_animations();
        }) as Box<dyn FnMut()>);

        let handle = web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();

        // Store the handle for cleanup
        if let Ok(mut engine) = engine_ref.try_borrow_mut() {
            engine.animation_handle = Some(handle);
        }
    }

    /// Start the animation loop (non-WASM version)
    #[cfg(not(target_arch = "wasm32"))]
    fn start_animation_loop(&mut self) {
        if self.is_running {
            return;
        }

        self.is_running = true;
        // No-op for non-WASM targets
        // In a real implementation, this might use a different animation system
    }

    /// Stop the animation loop
    #[cfg(target_arch = "wasm32")]
    fn stop_animation_loop(&mut self) {
        if let Some(handle) = self.animation_handle.take() {
            web_sys::window()
                .unwrap()
                .cancel_animation_frame(handle)
                .unwrap();
        }
        self.is_running = false;
    }

    /// Stop the animation loop (non-WASM version)
    #[cfg(not(target_arch = "wasm32"))]
    fn stop_animation_loop(&mut self) {
        self.animation_handle = None;
        self.is_running = false;
    }

    /// Update all animations
    fn update_animations(&mut self) {
        let mut completed_animations = Vec::new();
        let mut current_values = HashMap::new();

        // First pass: update animations and collect values
        for (property, animation) in &mut self.animations {
            let was_complete = animation.state.is_complete;

            if !was_complete {
                // Update the animation
                let delta_time = 1.0 / 60.0; // Assume 60fps
                animation.current_time += delta_time;

                if animation.is_spring {
                    Self::update_spring_animation_static(animation, delta_time);
                } else {
                    Self::update_eased_animation_static(animation);
                }
            }

            current_values.insert(property.clone(), animation.state.current);

            if animation.state.is_complete && !was_complete {
                completed_animations.push(property.clone());
            }
        }

        // Notify of updates
        if let Some(ref on_update) = self.on_update {
            on_update(&current_values);
        }

        // Remove completed animations
        for property in completed_animations {
            self.animations.remove(&property);
        }

        // Check if all animations are complete
        if self.animations.is_empty() {
            self.stop_animation_loop();
            if let Some(ref on_complete) = self.on_complete {
                on_complete();
            }
        } else if self.is_running {
            // Continue animation loop
            self.start_animation_loop();
        }
    }

    /// Update a single animation
    fn update_single_animation(&self, animation: &mut PropertyAnimation) {
        let delta_time = 1.0 / 60.0; // Assume 60fps
        animation.current_time += delta_time;

        if animation.is_spring {
            Self::update_spring_animation_static(animation, delta_time);
        } else {
            Self::update_eased_animation_static(animation);
        }
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
        }
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
