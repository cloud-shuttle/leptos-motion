//! Animation state management for the animation engine

use leptos_motion_core::*;
use std::collections::HashMap;
use std::rc::Rc;

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
    /// Create a new animation state
    pub fn new(initial: f64, target: f64) -> Self {
        Self {
            current: initial,
            target,
            initial,
            velocity: 0.0,
            is_complete: false,
        }
    }

    /// Reset the animation state
    pub fn reset(&mut self, initial: f64, target: f64) {
        self.current = initial;
        self.target = target;
        self.initial = initial;
        self.velocity = 0.0;
        self.is_complete = false;
    }

    /// Check if the animation is at rest
    pub fn is_at_rest(&self, threshold: f64) -> bool {
        (self.current - self.target).abs() < threshold && self.velocity.abs() < threshold
    }

    /// Get the progress of the animation (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        if self.target == self.initial {
            return 1.0;
        }
        ((self.current - self.initial) / (self.target - self.initial)).clamp(0.0, 1.0)
    }
}

/// Property animation with state and configuration
#[derive(Debug, Clone)]
pub struct PropertyAnimation {
    /// Animation state
    pub state: AnimationState,
    /// Transition configuration
    pub transition: Transition,
    /// Current time in the animation
    pub current_time: f64,
    /// Whether this is a spring animation
    pub is_spring: bool,
    /// Spring configuration (if applicable)
    pub spring_config: Option<SpringConfig>,
}

impl PropertyAnimation {
    /// Create a new property animation
    pub fn new(initial: f64, target: f64, transition: Transition) -> Self {
        let is_spring = matches!(transition.ease, Easing::Spring(_));
        let spring_config = if is_spring {
            match &transition.ease {
                Easing::Spring(config) => Some(config.clone()),
                _ => None,
            }
        } else {
            None
        };

        Self {
            state: AnimationState::new(initial, target),
            transition,
            current_time: 0.0,
            is_spring,
            spring_config,
        }
    }

    /// Update the animation state
    pub fn update(&mut self, delta_time: f64) {
        self.current_time += delta_time;
        
        if self.is_spring {
            self.update_spring(delta_time);
        } else {
            self.update_eased();
        }
    }

    /// Update spring-based animation
    fn update_spring(&mut self, delta_time: f64) {
        if let Some(config) = &self.spring_config {
            let force = -config.stiffness * (self.state.current - self.state.target);
            let damping_force = -config.damping * self.state.velocity;
            let acceleration = (force + damping_force) / config.mass;

            self.state.velocity += acceleration * delta_time;
            self.state.current += self.state.velocity * delta_time;

            // Check if animation is complete
            if self.state.is_at_rest(config.rest_delta) {
                self.state.current = self.state.target;
                self.state.velocity = 0.0;
                self.state.is_complete = true;
            }
        }
    }

    /// Update eased animation
    fn update_eased(&mut self) {
        let duration = self.transition.duration.unwrap_or(0.3);
        let progress = (self.current_time / duration).min(1.0);
        
        if progress >= 1.0 {
            self.state.current = self.state.target;
            self.state.is_complete = true;
        } else {
            let eased_progress = self.apply_easing(progress);
            self.state.current = self.state.initial + (self.state.target - self.state.initial) * eased_progress;
        }
    }

    /// Apply easing function to progress
    fn apply_easing(&self, progress: f64) -> f64 {
        match &self.transition.ease {
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
            Easing::CircIn => 1.0 - (1.0 - progress * progress).sqrt(),
            Easing::CircOut => ((2.0 - progress) * progress).sqrt(),
            Easing::CircInOut => {
                if progress < 0.5 {
                    (1.0 - (1.0 - 4.0 * progress * progress).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - 4.0 * (1.0 - progress) * (1.0 - progress)).sqrt()) / 2.0
                }
            }
            Easing::BackIn => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                C3 * progress * progress * progress - C1 * progress * progress
            }
            Easing::BackOut => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                1.0 + C3 * (progress - 1.0).powi(3) + C1 * (progress - 1.0) * (progress - 1.0)
            }
            Easing::BackInOut => {
                const C1: f64 = 1.70158;
                const C2: f64 = C1 * 1.525;
                if progress < 0.5 {
                    (2.0 * progress).powi(2) * ((C2 + 1.0) * 2.0 * progress - C2) / 2.0
                } else {
                    ((2.0 * progress - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * progress - 2.0) + C2) + 2.0) / 2.0
                }
            }
            Easing::Spring(_) => progress, // Handled separately in spring physics
            Easing::Bezier(x1, y1, x2, y2) => Self::cubic_bezier(0.0, *x1, *x2, 1.0, progress),
            Easing::CubicBezier(cb) => Self::cubic_bezier(0.0, cb.0, cb.2, 1.0, progress),
        }
    }

    /// Cubic bezier interpolation
    fn cubic_bezier(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.state.is_complete
    }

    /// Get current value
    pub fn get_current_value(&self) -> f64 {
        self.state.current
    }

    /// Get target value
    pub fn get_target_value(&self) -> f64 {
        self.state.target
    }

    /// Get animation progress
    pub fn get_progress(&self) -> f64 {
        self.state.get_progress()
    }
}

/// Animation state manager for tracking multiple animations
pub struct AnimationStateManager {
    /// Active animations
    animations: HashMap<String, PropertyAnimation>,
    /// Animation callbacks
    on_update: Option<Rc<dyn Fn(&HashMap<String, f64>)>>,
    on_complete: Option<Rc<dyn Fn()>>,
}

impl AnimationStateManager {
    /// Create a new animation state manager
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            on_update: None,
            on_complete: None,
        }
    }

    /// Add an animation
    pub fn add_animation(&mut self, property: String, animation: PropertyAnimation) {
        self.animations.insert(property, animation);
    }

    /// Remove an animation
    pub fn remove_animation(&mut self, property: &str) -> Option<PropertyAnimation> {
        self.animations.remove(property)
    }

    /// Get an animation
    pub fn get_animation(&self, property: &str) -> Option<&PropertyAnimation> {
        self.animations.get(property)
    }

    /// Get mutable animation
    pub fn get_animation_mut(&mut self, property: &str) -> Option<&mut PropertyAnimation> {
        self.animations.get_mut(property)
    }

    /// Get all animations
    pub fn get_animations(&self) -> &HashMap<String, PropertyAnimation> {
        &self.animations
    }

    /// Get mutable animations
    pub fn get_animations_mut(&mut self) -> &mut HashMap<String, PropertyAnimation> {
        &mut self.animations
    }

    /// Check if animations are empty
    pub fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }

    /// Get animation count
    pub fn len(&self) -> usize {
        self.animations.len()
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        self.animations.clear();
    }

    /// Set update callback
    pub fn set_on_update<F>(&mut self, callback: F)
    where
        F: Fn(&HashMap<String, f64>) + 'static,
    {
        self.on_update = Some(Rc::new(callback));
    }

    /// Set complete callback
    pub fn set_on_complete<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_complete = Some(Rc::new(callback));
    }

    /// Get update callback
    pub fn get_on_update(&self) -> Option<&Rc<dyn Fn(&HashMap<String, f64>)>> {
        self.on_update.as_ref()
    }

    /// Get complete callback
    pub fn get_on_complete(&self) -> Option<&Rc<dyn Fn()>> {
        self.on_complete.as_ref()
    }

    /// Update all animations
    pub fn update_animations(&mut self, delta_time: f64) {
        for animation in self.animations.values_mut() {
            animation.update(delta_time);
        }
    }

    /// Get current values of all animations
    pub fn get_current_values(&self) -> HashMap<String, f64> {
        self.animations
            .iter()
            .map(|(key, anim)| (key.clone(), anim.get_current_value()))
            .collect()
    }

    /// Check if all animations are complete
    pub fn all_complete(&self) -> bool {
        self.animations.values().all(|anim| anim.is_complete())
    }

    /// Remove completed animations
    pub fn remove_completed(&mut self) -> Vec<String> {
        let mut completed = Vec::new();
        let mut to_remove = Vec::new();

        for (key, animation) in &self.animations {
            if animation.is_complete() {
                to_remove.push(key.clone());
                completed.push(key.clone());
            }
        }

        for key in to_remove {
            self.animations.remove(&key);
        }

        completed
    }
}

impl Default for AnimationStateManager {
    fn default() -> Self {
        Self::new()
    }
}
