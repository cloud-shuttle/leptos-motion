//! Spring physics implementation for natural animations

use leptos_motion_core::*;

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

impl SpringConfig {
    /// Create a new spring configuration
    pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            stiffness: stiffness.max(0.0),
            damping: damping.max(0.0),
            mass: mass.max(0.001), // Prevent division by zero
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Create a gentle spring configuration
    pub fn gentle() -> Self {
        Self {
            stiffness: 50.0,
            damping: 15.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Create a wobbly spring configuration
    pub fn wobbly() -> Self {
        Self {
            stiffness: 180.0,
            damping: 12.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Create a stiff spring configuration
    pub fn stiff() -> Self {
        Self {
            stiffness: 210.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Create a slow spring configuration
    pub fn slow() -> Self {
        Self {
            stiffness: 280.0,
            damping: 60.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Create a bouncy spring configuration
    pub fn bouncy() -> Self {
        Self {
            stiffness: 300.0,
            damping: 8.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }
    }

    /// Set stiffness
    pub fn with_stiffness(mut self, stiffness: f64) -> Self {
        self.stiffness = stiffness.max(0.0);
        self
    }

    /// Set damping
    pub fn with_damping(mut self, damping: f64) -> Self {
        self.damping = damping.max(0.0);
        self
    }

    /// Set mass
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.mass = mass.max(0.001);
        self
    }

    /// Set initial velocity
    pub fn with_velocity(mut self, velocity: f64) -> Self {
        self.velocity = velocity;
        self
    }

    /// Set rest delta threshold
    pub fn with_rest_delta(mut self, rest_delta: f64) -> Self {
        self.rest_delta = rest_delta.max(0.0);
        self
    }

    /// Set rest speed threshold
    pub fn with_rest_speed(mut self, rest_speed: f64) -> Self {
        self.rest_speed = rest_speed.max(0.0);
        self
    }

    /// Check if the spring is at rest
    pub fn is_at_rest(&self, position: f64, target: f64, velocity: f64) -> bool {
        (position - target).abs() < self.rest_delta && velocity.abs() < self.rest_speed
    }

    /// Calculate the natural frequency of the spring
    pub fn natural_frequency(&self) -> f64 {
        (self.stiffness / self.mass).sqrt()
    }

    /// Calculate the damping ratio
    pub fn damping_ratio(&self) -> f64 {
        self.damping / (2.0 * (self.stiffness * self.mass).sqrt())
    }

    /// Check if the spring is underdamped (will oscillate)
    pub fn is_underdamped(&self) -> bool {
        self.damping_ratio() < 1.0
    }

    /// Check if the spring is critically damped
    pub fn is_critically_damped(&self) -> bool {
        (self.damping_ratio() - 1.0).abs() < 0.01
    }

    /// Check if the spring is overdamped (no oscillation)
    pub fn is_overdamped(&self) -> bool {
        self.damping_ratio() > 1.0
    }
}

/// Spring physics solver
pub struct SpringPhysics {
    /// Current position
    position: f64,
    /// Current velocity
    velocity: f64,
    /// Target position
    target: f64,
    /// Spring configuration
    config: SpringConfig,
}

impl SpringPhysics {
    /// Create a new spring physics solver
    pub fn new(initial_position: f64, target: f64, config: SpringConfig) -> Self {
        Self {
            position: initial_position,
            velocity: config.velocity,
            target,
            config,
        }
    }

    /// Update the spring physics
    pub fn update(&mut self, delta_time: f64) {
        // Calculate spring force (Hooke's law)
        let spring_force = -self.config.stiffness * (self.position - self.target);
        
        // Calculate damping force
        let damping_force = -self.config.damping * self.velocity;
        
        // Calculate acceleration (F = ma)
        let acceleration = (spring_force + damping_force) / self.config.mass;
        
        // Update velocity and position using Euler integration
        self.velocity += acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }

    /// Get current position
    pub fn get_position(&self) -> f64 {
        self.position
    }

    /// Get current velocity
    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }

    /// Get target position
    pub fn get_target(&self) -> f64 {
        self.target
    }

    /// Set target position
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Set position
    pub fn set_position(&mut self, position: f64) {
        self.position = position;
    }

    /// Set velocity
    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    /// Check if the spring is at rest
    pub fn is_at_rest(&self) -> bool {
        self.config.is_at_rest(self.position, self.target, self.velocity)
    }

    /// Get spring configuration
    pub fn get_config(&self) -> &SpringConfig {
        &self.config
    }

    /// Update spring configuration
    pub fn update_config(&mut self, config: SpringConfig) {
        self.config = config;
    }

    /// Reset the spring to initial state
    pub fn reset(&mut self, initial_position: f64, target: f64) {
        self.position = initial_position;
        self.velocity = self.config.velocity;
        self.target = target;
    }

    /// Get the energy of the spring system
    pub fn get_energy(&self) -> f64 {
        // Kinetic energy: 0.5 * m * v^2
        let kinetic_energy = 0.5 * self.config.mass * self.velocity * self.velocity;
        
        // Potential energy: 0.5 * k * x^2
        let displacement = self.position - self.target;
        let potential_energy = 0.5 * self.config.stiffness * displacement * displacement;
        
        kinetic_energy + potential_energy
    }

    /// Get the amplitude of oscillation (for underdamped springs)
    pub fn get_amplitude(&self) -> f64 {
        if !self.config.is_underdamped() {
            return 0.0;
        }

        let displacement = self.position - self.target;
        let natural_freq = self.config.natural_frequency();
        let damping_ratio = self.config.damping_ratio();
        let damped_freq = natural_freq * (1.0 - damping_ratio * damping_ratio).sqrt();
        
        if damped_freq > 0.0 {
            displacement.abs() / (1.0 - damping_ratio * damping_ratio).sqrt()
        } else {
            displacement.abs()
        }
    }

    /// Get the period of oscillation (for underdamped springs)
    pub fn get_period(&self) -> f64 {
        if !self.config.is_underdamped() {
            return f64::INFINITY;
        }

        let natural_freq = self.config.natural_frequency();
        let damping_ratio = self.config.damping_ratio();
        let damped_freq = natural_freq * (1.0 - damping_ratio * damping_ratio).sqrt();
        
        if damped_freq > 0.0 {
            2.0 * std::f64::consts::PI / damped_freq
        } else {
            f64::INFINITY
        }
    }
}

/// Spring animation manager for handling multiple spring animations
pub struct SpringAnimationManager {
    /// Active spring animations
    springs: std::collections::HashMap<String, SpringPhysics>,
    /// Animation callbacks
    on_update: Option<std::rc::Rc<dyn Fn(&std::collections::HashMap<String, f64>)>>,
    on_complete: Option<std::rc::Rc<dyn Fn()>>,
}

impl SpringAnimationManager {
    /// Create a new spring animation manager
    pub fn new() -> Self {
        Self {
            springs: std::collections::HashMap::new(),
            on_update: None,
            on_complete: None,
        }
    }

    /// Add a spring animation
    pub fn add_spring(&mut self, id: String, spring: SpringPhysics) {
        self.springs.insert(id, spring);
    }

    /// Remove a spring animation
    pub fn remove_spring(&mut self, id: &str) -> Option<SpringPhysics> {
        self.springs.remove(id)
    }

    /// Get a spring animation
    pub fn get_spring(&self, id: &str) -> Option<&SpringPhysics> {
        self.springs.get(id)
    }

    /// Get mutable spring animation
    pub fn get_spring_mut(&mut self, id: &str) -> Option<&mut SpringPhysics> {
        self.springs.get_mut(id)
    }

    /// Update all spring animations
    pub fn update(&mut self, delta_time: f64) {
        for spring in self.springs.values_mut() {
            spring.update(delta_time);
        }

        // Notify of updates
        if let Some(ref on_update) = self.on_update {
            let values: std::collections::HashMap<String, f64> = self.springs
                .iter()
                .map(|(id, spring)| (id.clone(), spring.get_position()))
                .collect();
            on_update(&values);
        }

        // Remove completed animations
        let mut completed = Vec::new();
        for (id, spring) in &self.springs {
            if spring.is_at_rest() {
                completed.push(id.clone());
            }
        }

        for id in completed {
            self.springs.remove(&id);
        }

        // Check if all animations are complete
        if self.springs.is_empty() {
            if let Some(ref on_complete) = self.on_complete {
                on_complete();
            }
        }
    }

    /// Set update callback
    pub fn set_on_update<F>(&mut self, callback: F)
    where
        F: Fn(&std::collections::HashMap<String, f64>) + 'static,
    {
        self.on_update = Some(std::rc::Rc::new(callback));
    }

    /// Set complete callback
    pub fn set_on_complete<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_complete = Some(std::rc::Rc::new(callback));
    }

    /// Check if any animations are active
    pub fn is_active(&self) -> bool {
        !self.springs.is_empty()
    }

    /// Get number of active animations
    pub fn count(&self) -> usize {
        self.springs.len()
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        self.springs.clear();
    }
}

impl Default for SpringAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
