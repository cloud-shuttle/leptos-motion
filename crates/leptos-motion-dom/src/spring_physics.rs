//! Spring Physics Engine
//!
//! A configurable spring physics system for natural motion animations

use leptos_motion_core::AnimationValue;
use std::collections::HashMap;

/// Configuration for spring physics
#[derive(Debug, Clone, PartialEq)]
pub struct SpringConfig {
    /// Spring tension (higher = stiffer spring)
    pub tension: f64,
    /// Spring friction (higher = more damping)
    pub friction: f64,
    /// Mass of the object (higher = slower response)
    pub mass: f64,
    /// Initial velocity
    pub velocity: f64,
    /// Rest position
    pub rest: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            tension: 300.0,
            friction: 30.0,
            mass: 1.0,
            velocity: 0.0,
            rest: 0.0,
        }
    }
}

/// Spring physics engine for natural motion
#[derive(Debug, Clone)]
pub struct SpringPhysics {
    config: SpringConfig,
    position: f64,
    velocity: f64,
}

impl SpringPhysics {
    /// Create a new spring physics engine
    pub fn new(config: SpringConfig) -> Self {
        Self {
            position: config.rest,
            velocity: config.velocity,
            config,
        }
    }

    /// Create a spring physics engine with default config
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(SpringConfig::default())
    }

    /// Update the spring physics simulation
    pub fn update(&mut self, target: f64, delta_time: f64) -> f64 {
        // Calculate spring force
        let displacement = target - self.position;
        let spring_force = self.config.tension * displacement;

        // Calculate damping force
        let damping_force = self.config.friction * self.velocity;

        // Calculate acceleration (F = ma, so a = F/m)
        let acceleration = (spring_force - damping_force) / self.config.mass;

        // Update velocity and position
        self.velocity += acceleration * delta_time;
        self.position += self.velocity * delta_time;

        self.position
    }

    /// Check if the spring is at rest (within threshold)
    pub fn is_at_rest(&self, threshold: f64) -> bool {
        self.velocity.abs() < threshold
    }

    /// Get current position
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Get current velocity
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    /// Set position
    pub fn set_position(&mut self, position: f64) {
        self.position = position;
    }

    /// Set velocity
    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    /// Update configuration
    pub fn update_config(&mut self, config: SpringConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &SpringConfig {
        &self.config
    }
}

/// Spring physics manager for multiple springs
#[derive(Debug, Clone)]
pub struct SpringManager {
    springs: HashMap<String, SpringPhysics>,
}

impl SpringManager {
    /// Create a new spring manager
    pub fn new() -> Self {
        Self {
            springs: HashMap::new(),
        }
    }

    /// Add a spring with the given name
    pub fn add_spring(&mut self, name: String, spring: SpringPhysics) {
        self.springs.insert(name, spring);
    }

    /// Get a spring by name
    pub fn get_spring(&self, name: &str) -> Option<&SpringPhysics> {
        self.springs.get(name)
    }

    /// Get a mutable spring by name
    pub fn get_spring_mut(&mut self, name: &str) -> Option<&mut SpringPhysics> {
        self.springs.get_mut(name)
    }

    /// Update all springs
    pub fn update_all(
        &mut self,
        targets: HashMap<String, f64>,
        delta_time: f64,
    ) -> HashMap<String, f64> {
        let mut results = HashMap::new();

        for (name, spring) in self.springs.iter_mut() {
            if let Some(&target) = targets.get(name) {
                let position = spring.update(target, delta_time);
                results.insert(name.clone(), position);
            }
        }

        results
    }

    /// Check if all springs are at rest
    pub fn all_at_rest(&self, threshold: f64) -> bool {
        self.springs
            .values()
            .all(|spring| spring.is_at_rest(threshold))
    }

    /// Remove a spring
    pub fn remove_spring(&mut self, name: &str) -> Option<SpringPhysics> {
        self.springs.remove(name)
    }

    /// Clear all springs
    pub fn clear(&mut self) {
        self.springs.clear();
    }

    /// Get spring count
    pub fn count(&self) -> usize {
        self.springs.len()
    }
}

impl Default for SpringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert animation values to spring targets
pub fn animation_values_to_spring_targets(
    values: &HashMap<String, AnimationValue>,
) -> HashMap<String, f64> {
    let mut targets = HashMap::new();

    for (key, value) in values {
        if let AnimationValue::Number(num) = value {
            targets.insert(key.clone(), *num);
        }
    }

    targets
}

/// Convert spring positions to animation values
pub fn spring_positions_to_animation_values(
    positions: &HashMap<String, f64>,
) -> HashMap<String, AnimationValue> {
    positions
        .iter()
        .map(|(key, &value)| (key.clone(), AnimationValue::Number(value)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_physics_basic() {
        let config = SpringConfig {
            tension: 300.0,
            friction: 30.0,
            mass: 1.0,
            velocity: 0.0,
            rest: 0.0,
        };

        let mut spring = SpringPhysics::new(config);
        spring.set_position(100.0); // Start displaced

        // Update towards rest position
        let position = spring.update(0.0, 0.016); // 60fps

        // Should move towards rest position
        assert!(position < 100.0);
        assert!(position > 0.0);
    }

    #[test]
    fn test_spring_physics_rest_detection() {
        let mut spring = SpringPhysics::default();
        spring.set_position(10.0);

        // Simulate until rest
        for _ in 0..1000 {
            spring.update(0.0, 0.016);
        }

        assert!(spring.is_at_rest(0.1));
    }

    #[test]
    fn test_spring_manager() {
        let mut manager = SpringManager::new();

        let spring1 = SpringPhysics::new(SpringConfig {
            tension: 300.0,
            friction: 30.0,
            mass: 1.0,
            velocity: 0.0,
            rest: 0.0,
        });

        let spring2 = SpringPhysics::new(SpringConfig {
            tension: 200.0,
            friction: 20.0,
            mass: 1.0,
            velocity: 0.0,
            rest: 0.0,
        });

        manager.add_spring("x".to_string(), spring1);
        manager.add_spring("y".to_string(), spring2);

        assert_eq!(manager.count(), 2);

        let mut targets = HashMap::new();
        targets.insert("x".to_string(), 100.0);
        targets.insert("y".to_string(), 50.0);

        let results = manager.update_all(targets, 0.016);

        assert_eq!(results.len(), 2);
        assert!(results.contains_key("x"));
        assert!(results.contains_key("y"));
    }
}
