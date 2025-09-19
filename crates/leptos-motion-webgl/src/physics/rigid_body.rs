//! Rigid body physics

use super::*;
use crate::{Result, WebGLError};

/// Type of rigid body
#[derive(Debug, Clone, PartialEq)]
pub enum RigidBodyType {
    /// Static body (doesn't move)
    Static,
    /// Kinematic body (moves but not affected by forces)
    Kinematic,
    /// Dynamic body (affected by forces and collisions)
    Dynamic,
}

/// Rigid body for physics simulation
#[derive(Debug, Clone)]
pub struct RigidBody {
    /// Unique identifier
    pub id: u64,
    /// Body type
    pub body_type: RigidBodyType,
    /// Position (x, y, z)
    pub position: (f32, f32, f32),
    /// Rotation (quaternion: x, y, z, w)
    pub rotation: (f32, f32, f32, f32),
    /// Linear velocity (x, y, z)
    pub linear_velocity: (f32, f32, f32),
    /// Angular velocity (x, y, z)
    pub angular_velocity: (f32, f32, f32),
    /// Mass
    pub mass: f32,
    /// Inverse mass (for performance)
    pub inverse_mass: f32,
    /// Inertia tensor
    pub inertia: (f32, f32, f32),
    /// Inverse inertia tensor
    pub inverse_inertia: (f32, f32, f32),
    /// Linear damping
    pub linear_damping: f32,
    /// Angular damping
    pub angular_damping: f32,
    /// Whether the body is sleeping
    pub is_sleeping: bool,
    /// Sleep timer
    pub sleep_timer: f32,
    /// Whether the body is active
    pub is_active: bool,
    /// Collision shape
    pub collision_shape: CollisionShape,
    /// Bounding box
    pub bounding_box: BoundingBox,
}

impl RigidBody {
    /// Create a new rigid body
    pub fn new(id: u64, body_type: RigidBodyType, mass: f32) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let inertia = (1.0, 1.0, 1.0); // Simplified
        let inverse_inertia = (1.0 / inertia.0, 1.0 / inertia.1, 1.0 / inertia.2);

        Self {
            id,
            body_type,
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0), // Identity quaternion
            linear_velocity: (0.0, 0.0, 0.0),
            angular_velocity: (0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia,
            inverse_inertia,
            linear_damping: 0.99,
            angular_damping: 0.99,
            is_sleeping: false,
            sleep_timer: 0.0,
            is_active: true,
            collision_shape: CollisionShape::Box { width: 1.0, height: 1.0, depth: 1.0 },
            bounding_box: BoundingBox::new((0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
        }
    }

    /// Create a static rigid body
    pub fn new_static(id: u64) -> Self {
        Self::new(id, RigidBodyType::Static, 0.0)
    }

    /// Create a kinematic rigid body
    pub fn new_kinematic(id: u64) -> Self {
        Self::new(id, RigidBodyType::Kinematic, 0.0)
    }

    /// Create a dynamic rigid body
    pub fn new_dynamic(id: u64, mass: f32) -> Self {
        Self::new(id, RigidBodyType::Dynamic, mass)
    }

    /// Set position
    pub fn set_position(&mut self, position: (f32, f32, f32)) {
        self.position = position;
        self.update_bounding_box();
    }

    /// Set rotation
    pub fn set_rotation(&mut self, rotation: (f32, f32, f32, f32)) {
        self.rotation = rotation;
        self.update_bounding_box();
    }

    /// Set linear velocity
    pub fn set_linear_velocity(&mut self, velocity: (f32, f32, f32)) {
        self.linear_velocity = velocity;
        self.wake_up();
    }

    /// Set angular velocity
    pub fn set_angular_velocity(&mut self, velocity: (f32, f32, f32)) {
        self.angular_velocity = velocity;
        self.wake_up();
    }

    /// Apply force to the body
    pub fn apply_force(&mut self, force: (f32, f32, f32)) {
        if self.body_type == RigidBodyType::Dynamic && self.inverse_mass > 0.0 {
            let acceleration = (
                force.0 * self.inverse_mass,
                force.1 * self.inverse_mass,
                force.2 * self.inverse_mass,
            );
            self.linear_velocity.0 += acceleration.0;
            self.linear_velocity.1 += acceleration.1;
            self.linear_velocity.2 += acceleration.2;
            self.wake_up();
        }
    }

    /// Apply impulse to the body
    pub fn apply_impulse(&mut self, impulse: (f32, f32, f32)) {
        if self.body_type == RigidBodyType::Dynamic && self.inverse_mass > 0.0 {
            self.linear_velocity.0 += impulse.0 * self.inverse_mass;
            self.linear_velocity.1 += impulse.1 * self.inverse_mass;
            self.linear_velocity.2 += impulse.2 * self.inverse_mass;
            self.wake_up();
        }
    }

    /// Apply torque to the body
    pub fn apply_torque(&mut self, torque: (f32, f32, f32)) {
        if self.body_type == RigidBodyType::Dynamic {
            self.angular_velocity.0 += torque.0 * self.inverse_inertia.0;
            self.angular_velocity.1 += torque.1 * self.inverse_inertia.1;
            self.angular_velocity.2 += torque.2 * self.inverse_inertia.2;
            self.wake_up();
        }
    }

    /// Update the body's position and rotation
    pub fn update(&mut self, time_step: f32) {
        if !self.is_active || self.is_sleeping {
            return;
        }

        // Update position
        self.position.0 += self.linear_velocity.0 * time_step;
        self.position.1 += self.linear_velocity.1 * time_step;
        self.position.2 += self.linear_velocity.2 * time_step;

        // Update rotation (simplified)
        self.rotation.0 += self.angular_velocity.0 * time_step;
        self.rotation.1 += self.angular_velocity.1 * time_step;
        self.rotation.2 += self.angular_velocity.2 * time_step;

        // Apply damping
        self.linear_velocity.0 *= self.linear_damping;
        self.linear_velocity.1 *= self.linear_damping;
        self.linear_velocity.2 *= self.linear_damping;
        self.angular_velocity.0 *= self.angular_damping;
        self.angular_velocity.1 *= self.angular_damping;
        self.angular_velocity.2 *= self.angular_damping;

        // Update bounding box
        self.update_bounding_box();

        // Check for sleeping
        self.update_sleep_state(time_step);
    }

    /// Update bounding box based on position and collision shape
    fn update_bounding_box(&mut self) {
        let (x, y, z) = self.position;
        let (width, height, depth) = match &self.collision_shape {
            CollisionShape::Box { width, height, depth } => (*width, *height, *depth),
            CollisionShape::Sphere { radius } => (*radius * 2.0, *radius * 2.0, *radius * 2.0),
            CollisionShape::Capsule { radius, height } => (*radius * 2.0, *height, *radius * 2.0),
            CollisionShape::Cylinder { half_extents } => (half_extents[0] * 2.0, half_extents[1] * 2.0, half_extents[2] * 2.0),
            CollisionShape::Plane { .. } => (1000.0, 0.1, 1000.0), // Large plane
        };

        self.bounding_box = BoundingBox::new(
            (x - width / 2.0, y - height / 2.0, z - depth / 2.0),
            (x + width / 2.0, y + height / 2.0, z + depth / 2.0),
        );
    }

    /// Update sleep state
    fn update_sleep_state(&mut self, time_step: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }

        let linear_speed = (self.linear_velocity.0 * self.linear_velocity.0 +
                           self.linear_velocity.1 * self.linear_velocity.1 +
                           self.linear_velocity.2 * self.linear_velocity.2).sqrt();

        let angular_speed = (self.angular_velocity.0 * self.angular_velocity.0 +
                            self.angular_velocity.1 * self.angular_velocity.1 +
                            self.angular_velocity.2 * self.angular_velocity.2).sqrt();

        if linear_speed < 0.1 && angular_speed < 0.1 {
            self.sleep_timer += time_step;
            if self.sleep_timer > 2.0 { // Sleep after 2 seconds of inactivity
                self.is_sleeping = true;
            }
        } else {
            self.sleep_timer = 0.0;
            self.is_sleeping = false;
        }
    }

    /// Wake up the body
    pub fn wake_up(&mut self) {
        self.is_sleeping = false;
        self.sleep_timer = 0.0;
    }

    /// Set collision shape
    pub fn set_collision_shape(&mut self, shape: CollisionShape) {
        self.collision_shape = shape;
        self.update_bounding_box();
    }

    /// Get collision shape
    pub fn collision_shape(&self) -> &CollisionShape {
        &self.collision_shape
    }

    /// Get bounding box
    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    /// Check if body is static
    pub fn is_static(&self) -> bool {
        self.body_type == RigidBodyType::Static
    }

    /// Check if body is kinematic
    pub fn is_kinematic(&self) -> bool {
        self.body_type == RigidBodyType::Kinematic
    }

    /// Check if body is dynamic
    pub fn is_dynamic(&self) -> bool {
        self.body_type == RigidBodyType::Dynamic
    }

    /// Get kinetic energy
    pub fn kinetic_energy(&self) -> f32 {
        if self.body_type != RigidBodyType::Dynamic {
            return 0.0;
        }

        let linear_energy = 0.5 * self.mass * (
            self.linear_velocity.0 * self.linear_velocity.0 +
            self.linear_velocity.1 * self.linear_velocity.1 +
            self.linear_velocity.2 * self.linear_velocity.2
        );

        let angular_energy = 0.5 * (
            self.inertia.0 * self.angular_velocity.0 * self.angular_velocity.0 +
            self.inertia.1 * self.angular_velocity.1 * self.angular_velocity.1 +
            self.inertia.2 * self.angular_velocity.2 * self.angular_velocity.2
        );

        linear_energy + angular_energy
    }
}
