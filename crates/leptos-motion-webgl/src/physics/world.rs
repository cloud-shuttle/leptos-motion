//! Physics world implementation

use super::*;
use crate::{Result, WebGLError};

/// Physics world for managing rigid bodies and collisions
pub struct PhysicsWorld {
    /// Configuration
    config: PhysicsWorldConfig,
    /// Rigid bodies
    bodies: std::collections::HashMap<u64, RigidBody>,
    /// Collision detector
    collision_detector: CollisionDetector,
    /// Active collisions
    active_collisions: Vec<Collision>,
    /// Next body ID
    next_body_id: u64,
    /// Current time
    current_time: f64,
    /// Accumulated time
    accumulated_time: f64,
    /// Performance metrics
    performance_metrics: PhysicsPerformanceMetrics,
}

/// Physics performance metrics
#[derive(Debug, Clone, Default)]
pub struct PhysicsPerformanceMetrics {
    /// Total simulation steps
    pub total_steps: u64,
    /// Average step time in milliseconds
    pub avg_step_time_ms: f64,
    /// Total simulation time in milliseconds
    pub total_simulation_time_ms: f64,
    /// Number of collision detections
    pub collision_detections: u64,
    /// Number of active bodies
    pub active_bodies: usize,
    /// Number of active collisions
    pub active_collisions: usize,
}

impl PhysicsWorld {
    /// Create a new physics world
    pub fn new(config: PhysicsWorldConfig) -> Result<Self> {
        config.validate()?;
        
        Ok(Self {
            config,
            bodies: std::collections::HashMap::new(),
            collision_detector: CollisionDetector::new(),
            active_collisions: Vec::new(),
            next_body_id: 1,
            current_time: 0.0,
            accumulated_time: 0.0,
            performance_metrics: PhysicsPerformanceMetrics::default(),
        })
    }

    /// Create a new physics world with default configuration
    pub fn new_default() -> Result<Self> {
        Self::new(PhysicsWorldConfig::default())
    }

    /// Add a rigid body to the world
    pub fn add_body(&mut self, mut body: RigidBody) -> u64 {
        let id = self.next_body_id;
        self.next_body_id += 1;
        body.id = id;
        self.bodies.insert(id, body);
        id
    }

    /// Remove a rigid body from the world
    pub fn remove_body(&mut self, body_id: u64) -> Option<RigidBody> {
        self.bodies.remove(&body_id)
    }

    /// Get a rigid body by ID
    pub fn get_body(&self, body_id: u64) -> Option<&RigidBody> {
        self.bodies.get(&body_id)
    }

    /// Get a mutable rigid body by ID
    pub fn get_body_mut(&mut self, body_id: u64) -> Option<&mut RigidBody> {
        self.bodies.get_mut(&body_id)
    }

    /// Get all rigid bodies
    pub fn bodies(&self) -> &std::collections::HashMap<u64, RigidBody> {
        &self.bodies
    }

    /// Get all rigid bodies as a vector
    pub fn bodies_vec(&self) -> Vec<&RigidBody> {
        self.bodies.values().collect()
    }

    /// Get all active rigid bodies
    pub fn active_bodies(&self) -> Vec<&RigidBody> {
        self.bodies.values().filter(|b| b.is_active).collect()
    }

    /// Get all active rigid bodies as mutable references
    pub fn active_bodies_mut(&mut self) -> Vec<&mut RigidBody> {
        self.bodies.values_mut().filter(|b| b.is_active).collect()
    }

    /// Step the physics simulation
    pub fn step(&mut self, delta_time: f64) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        self.current_time += delta_time;
        self.accumulated_time += delta_time;

        // Fixed time step simulation
        while self.accumulated_time >= self.config.effective_time_step() as f64 {
            self.fixed_step()?;
            self.accumulated_time -= self.config.effective_time_step() as f64;
        }

        // Update performance metrics
        let step_time = start_time.elapsed().as_secs_f64() * 1000.0;
        self.update_performance_metrics(step_time);

        Ok(())
    }

    /// Fixed time step simulation
    fn fixed_step(&mut self) -> Result<()> {
        let time_step = self.config.effective_time_step();

        // Update rigid bodies
        for body in self.bodies.values_mut() {
            if body.is_active && !body.is_sleeping {
                // Apply gravity
                if self.config.has_gravity() && body.is_dynamic() {
                    let gravity_force = (
                        self.config.gravity.0 * body.mass,
                        self.config.gravity.1 * body.mass,
                        self.config.gravity.2 * body.mass,
                    );
                    body.apply_force(gravity_force);
                }

                // Update body
                body.update(time_step);
            }
        }

        // Detect collisions
        let bodies_vec: Vec<RigidBody> = self.bodies.values().cloned().collect();
        self.active_collisions = self.collision_detector.detect_collisions(&bodies_vec)?;

        // Resolve collisions
        self.resolve_collisions()?;

        // Update performance metrics
        self.performance_metrics.active_bodies = self.active_bodies().len();
        self.performance_metrics.active_collisions = self.active_collisions.len();
        self.performance_metrics.collision_detections += 1;

        Ok(())
    }

    /// Resolve collisions
    fn resolve_collisions(&mut self) -> Result<()> {
        for collision in &self.active_collisions {
            // Get body IDs
            let body_a_id = collision.body_a_id.clone();
            let body_b_id = collision.body_b_id.clone();
            
            // Process each body separately to avoid borrowing conflicts
            if let Some(body_a) = self.bodies.get_mut(&body_a_id) {
                for contact in &collision.contact_points {
                    if contact.is_valid() {
                        // Apply impulse to body A
                        let impulse = contact.impulse;
                        let normal = contact.normal;

                        if body_a.is_dynamic() {
                            let impulse_a = (
                                -normal.0 * impulse,
                                -normal.1 * impulse,
                                -normal.2 * impulse,
                            );
                            body_a.apply_impulse(impulse_a);
                        }
                    }
                }
            }
            
            // Process body B separately
            if let Some(body_b) = self.bodies.get_mut(&body_b_id) {
                for contact in &collision.contact_points {
                    if contact.is_valid() {
                        let impulse = contact.impulse;
                        let normal = contact.normal;
                        
                        if body_b.is_dynamic() {
                            let impulse_b = (
                                normal.0 * impulse,
                                normal.1 * impulse,
                                normal.2 * impulse,
                            );
                            body_b.apply_impulse(impulse_b);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self, step_time: f64) {
        self.performance_metrics.total_steps += 1;
        self.performance_metrics.total_simulation_time_ms += step_time;
        self.performance_metrics.avg_step_time_ms = 
            self.performance_metrics.total_simulation_time_ms / self.performance_metrics.total_steps as f64;
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &PhysicsPerformanceMetrics {
        &self.performance_metrics
    }

    /// Get configuration
    pub fn config(&self) -> &PhysicsWorldConfig {
        &self.config
    }

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut PhysicsWorldConfig {
        &mut self.config
    }

    /// Get active collisions
    pub fn active_collisions(&self) -> &Vec<Collision> {
        &self.active_collisions
    }

    /// Get current time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Set gravity
    pub fn set_gravity(&mut self, gravity: (f32, f32, f32)) {
        self.config.gravity = gravity;
    }

    /// Get gravity
    pub fn gravity(&self) -> (f32, f32, f32) {
        self.config.gravity
    }

    /// Set time step
    pub fn set_time_step(&mut self, time_step: f32) -> Result<()> {
        self.config.time_step = time_step;
        self.config.validate()
    }

    /// Get time step
    pub fn time_step(&self) -> f32 {
        self.config.time_step
    }

    /// Enable/disable sleeping
    pub fn set_sleeping_enabled(&mut self, enabled: bool) {
        self.config.enable_sleeping = enabled;
    }

    /// Check if sleeping is enabled
    pub fn is_sleeping_enabled(&self) -> bool {
        self.config.enable_sleeping
    }

    /// Wake up all bodies
    pub fn wake_up_all_bodies(&mut self) {
        for body in self.bodies.values_mut() {
            body.wake_up();
        }
    }

    /// Clear all bodies
    pub fn clear_bodies(&mut self) {
        self.bodies.clear();
        self.active_collisions.clear();
        self.next_body_id = 1;
    }

    /// Get the number of bodies
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    /// Get the number of active bodies
    pub fn active_body_count(&self) -> usize {
        self.bodies.values().filter(|b| b.is_active).count()
    }

    /// Get the number of sleeping bodies
    pub fn sleeping_body_count(&self) -> usize {
        self.bodies.values().filter(|b| b.is_sleeping).count()
    }

    /// Get the total kinetic energy
    pub fn total_kinetic_energy(&self) -> f32 {
        self.bodies.values().map(|b| b.kinetic_energy()).sum()
    }

    /// Check if the world is stable (low kinetic energy)
    pub fn is_stable(&self) -> bool {
        self.total_kinetic_energy() < 0.1
    }

    /// Get world statistics
    pub fn get_statistics(&self) -> PhysicsWorldStatistics {
        PhysicsWorldStatistics {
            total_bodies: self.body_count(),
            active_bodies: self.active_body_count(),
            sleeping_bodies: self.sleeping_body_count(),
            active_collisions: self.active_collisions.len(),
            total_kinetic_energy: self.total_kinetic_energy(),
            is_stable: self.is_stable(),
            current_time: self.current_time,
            performance_metrics: self.performance_metrics.clone(),
        }
    }
}

/// Physics world statistics
#[derive(Debug, Clone)]
pub struct PhysicsWorldStatistics {
    /// Total number of bodies
    pub total_bodies: usize,
    /// Number of active bodies
    pub active_bodies: usize,
    /// Number of sleeping bodies
    pub sleeping_bodies: usize,
    /// Number of active collisions
    pub active_collisions: usize,
    /// Total kinetic energy
    pub total_kinetic_energy: f32,
    /// Whether the world is stable
    pub is_stable: bool,
    /// Current simulation time
    pub current_time: f64,
    /// Performance metrics
    pub performance_metrics: PhysicsPerformanceMetrics,
}
