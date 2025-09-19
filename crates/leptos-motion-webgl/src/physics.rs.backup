//! Physics simulation system for WebGL rendering

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use uuid::Uuid;

/// Physics world configuration
#[derive(Debug, Clone)]
pub struct PhysicsWorldConfig {
    /// Gravity vector
    pub gravity: [f32; 3],
    /// Time step for physics simulation
    pub time_step: f32,
    /// Maximum number of iterations per frame
    pub max_iterations: u32,
    /// Collision detection tolerance
    pub collision_tolerance: f32,
    /// Enable continuous collision detection
    pub continuous_collision_detection: bool,
}

impl Default for PhysicsWorldConfig {
    fn default() -> Self {
        Self {
            gravity: [0.0, -9.81, 0.0],
            time_step: 1.0 / 60.0, // 60 FPS
            max_iterations: 10,
            collision_tolerance: 0.001,
            continuous_collision_detection: true,
        }
    }
}

/// Rigid body types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBodyType {
    /// Static body (doesn't move)
    Static,
    /// Dynamic body (affected by forces)
    Dynamic,
    /// Kinematic body (moved by code, not forces)
    Kinematic,
}

/// Collision shapes
#[derive(Debug, Clone)]
pub enum CollisionShape {
    /// Box collision shape
    Box {
        /// Half extents
        half_extents: [f32; 3],
    },
    /// Sphere collision shape
    Sphere {
        /// Radius
        radius: f32,
    },
    /// Plane collision shape
    Plane {
        /// Normal vector
        normal: [f32; 3],
        /// Distance from origin
        distance: f32,
    },
    /// Capsule collision shape
    Capsule {
        /// Radius
        radius: f32,
        /// Height
        height: f32,
    },
    /// Cylinder collision shape
    Cylinder {
        /// Half extents
        half_extents: [f32; 3],
    },
}

impl CollisionShape {
    /// Get the volume of the collision shape
    pub fn get_volume(&self) -> f32 {
        match self {
            CollisionShape::Box { half_extents } => {
                8.0 * half_extents[0] * half_extents[1] * half_extents[2]
            }
            CollisionShape::Sphere { radius } => {
                (4.0 / 3.0) * std::f32::consts::PI * radius * radius * radius
            }
            CollisionShape::Plane { .. } => 0.0,
            CollisionShape::Capsule { radius, height } => {
                let sphere_volume = (4.0 / 3.0) * std::f32::consts::PI * radius * radius * radius;
                let cylinder_volume = std::f32::consts::PI * radius * radius * height;
                sphere_volume + cylinder_volume
            }
            CollisionShape::Cylinder { half_extents } => {
                std::f32::consts::PI * half_extents[0] * half_extents[0] * (2.0 * half_extents[1])
            }
        }
    }

    /// Get the bounding box of the collision shape
    pub fn get_bounding_box(&self) -> BoundingBox {
        match self {
            CollisionShape::Box { half_extents } => BoundingBox {
                min: [-half_extents[0], -half_extents[1], -half_extents[2]],
                max: [half_extents[0], half_extents[1], half_extents[2]],
            },
            CollisionShape::Sphere { radius } => BoundingBox {
                min: [-*radius, -*radius, -*radius],
                max: [*radius, *radius, *radius],
            },
            CollisionShape::Plane { .. } => BoundingBox {
                min: [-f32::INFINITY, -f32::INFINITY, -f32::INFINITY],
                max: [f32::INFINITY, f32::INFINITY, f32::INFINITY],
            },
            CollisionShape::Capsule { radius, height } => {
                let half_height = height * 0.5;
                BoundingBox {
                    min: [-*radius, -half_height, -*radius],
                    max: [*radius, half_height, *radius],
                }
            }
            CollisionShape::Cylinder { half_extents } => BoundingBox {
                min: [-half_extents[0], -half_extents[1], -half_extents[2]],
                max: [half_extents[0], half_extents[1], half_extents[2]],
            },
        }
    }
}

/// Bounding box for collision detection
#[derive(Debug, Clone)]
pub struct BoundingBox {
    /// Minimum bounds
    pub min: [f32; 3],
    /// Maximum bounds
    pub max: [f32; 3],
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new() -> Self {
        Self {
            min: [f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: [-f32::INFINITY, -f32::INFINITY, -f32::INFINITY],
        }
    }

    /// Create a bounding box from min and max points
    pub fn from_min_max(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max }
    }

    /// Expand the bounding box to include a point
    pub fn expand(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);

        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
    }

    /// Check if this bounding box intersects with another
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min[0] <= other.max[0]
            && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1]
            && self.max[1] >= other.min[1]
            && self.min[2] <= other.max[2]
            && self.max[2] >= other.min[2]
    }

    /// Get the center of the bounding box
    pub fn get_center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Get the size of the bounding box
    pub fn get_size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }
}

/// Rigid body for physics simulation
#[derive(Debug, Clone)]
pub struct RigidBody {
    /// Unique identifier
    pub id: String,
    /// Body name
    pub name: String,
    /// Body type
    pub body_type: RigidBodyType,
    /// Position
    pub position: [f32; 3],
    /// Rotation (quaternion)
    pub rotation: [f32; 4],
    /// Linear velocity
    pub linear_velocity: [f32; 3],
    /// Angular velocity
    pub angular_velocity: [f32; 3],
    /// Mass
    pub mass: f32,
    /// Inverse mass (for performance)
    pub inverse_mass: f32,
    /// Inertia tensor
    pub inertia: [f32; 9],
    /// Inverse inertia tensor
    pub inverse_inertia: [f32; 9],
    /// Collision shape
    pub collision_shape: CollisionShape,
    /// Bounding box
    pub bounding_box: BoundingBox,
    /// Friction coefficient
    pub friction: f32,
    /// Restitution (bounciness)
    pub restitution: f32,
    /// Linear damping
    pub linear_damping: f32,
    /// Angular damping
    pub angular_damping: f32,
    /// Forces applied to the body
    pub forces: [f32; 3],
    /// Torques applied to the body
    pub torques: [f32; 3],
    /// Sleep threshold
    pub sleep_threshold: f32,
    /// Is the body sleeping
    pub is_sleeping: bool,
    /// Sleep time
    pub sleep_time: f32,
}

impl RigidBody {
    /// Create a new rigid body
    pub fn new(name: &str, body_type: RigidBodyType, collision_shape: CollisionShape) -> Self {
        let id = Uuid::new_v4().to_string();
        let mass = match body_type {
            RigidBodyType::Static => 0.0,
            RigidBodyType::Dynamic => 1.0,
            RigidBodyType::Kinematic => 0.0,
        };

        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };

        // Calculate inertia tensor based on collision shape
        let inertia = Self::calculate_inertia_tensor(&collision_shape, mass);
        let inverse_inertia = Self::calculate_inverse_inertia_tensor(&inertia);

        let bounding_box = collision_shape.get_bounding_box();

        Self {
            id,
            name: name.to_string(),
            body_type,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            linear_velocity: [0.0, 0.0, 0.0],
            angular_velocity: [0.0, 0.0, 0.0],
            mass,
            inverse_mass,
            inertia,
            inverse_inertia,
            collision_shape,
            bounding_box,
            friction: 0.5,
            restitution: 0.3,
            linear_damping: 0.99,
            angular_damping: 0.99,
            forces: [0.0, 0.0, 0.0],
            torques: [0.0, 0.0, 0.0],
            sleep_threshold: 0.1,
            is_sleeping: false,
            sleep_time: 0.0,
        }
    }

    /// Calculate inertia tensor for a collision shape
    fn calculate_inertia_tensor(shape: &CollisionShape, mass: f32) -> [f32; 9] {
        match shape {
            CollisionShape::Box { half_extents } => {
                let x = half_extents[0];
                let y = half_extents[1];
                let z = half_extents[2];
                let factor = mass / 12.0;

                [
                    factor * (y * y + z * z),
                    0.0,
                    0.0,
                    0.0,
                    factor * (x * x + z * z),
                    0.0,
                    0.0,
                    0.0,
                    factor * (x * x + y * y),
                ]
            }
            CollisionShape::Sphere { radius } => {
                let factor = 2.0 * mass * radius * radius / 5.0;
                [factor, 0.0, 0.0, 0.0, factor, 0.0, 0.0, 0.0, factor]
            }
            CollisionShape::Plane { .. } => {
                [0.0; 9] // Infinite inertia for planes
            }
            CollisionShape::Capsule { radius, height } => {
                let r = radius;
                let h = height;
                let factor = mass / 12.0;

                [
                    factor * (3.0 * r * r + h * h),
                    0.0,
                    0.0,
                    0.0,
                    factor * (3.0 * r * r + h * h),
                    0.0,
                    0.0,
                    0.0,
                    factor * (6.0 * r * r),
                ]
            }
            CollisionShape::Cylinder { half_extents } => {
                let x = half_extents[0];
                let y = half_extents[1];
                let z = half_extents[2];
                let factor = mass / 12.0;

                [
                    factor * (y * y + z * z),
                    0.0,
                    0.0,
                    0.0,
                    factor * (x * x + z * z),
                    0.0,
                    0.0,
                    0.0,
                    factor * (x * x + y * y),
                ]
            }
        }
    }

    /// Calculate inverse inertia tensor
    fn calculate_inverse_inertia_tensor(inertia: &[f32; 9]) -> [f32; 9] {
        // For diagonal matrices, just invert the diagonal elements
        let mut inverse = [0.0; 9];
        for i in 0..3 {
            let idx = i * 3 + i;
            if inertia[idx] > 0.0 {
                inverse[idx] = 1.0 / inertia[idx];
            }
        }
        inverse
    }

    /// Apply a force to the body
    pub fn apply_force(&mut self, force: [f32; 3]) {
        if self.body_type == RigidBodyType::Dynamic {
            self.forces[0] += force[0];
            self.forces[1] += force[1];
            self.forces[2] += force[2];
        }
    }

    /// Apply a force at a specific point
    pub fn apply_force_at_point(&mut self, force: [f32; 3], point: [f32; 3]) {
        if self.body_type == RigidBodyType::Dynamic {
            self.apply_force(force);

            // Calculate torque
            let r = [
                point[0] - self.position[0],
                point[1] - self.position[1],
                point[2] - self.position[2],
            ];
            let torque = [
                r[1] * force[2] - r[2] * force[1],
                r[2] * force[0] - r[0] * force[2],
                r[0] * force[1] - r[1] * force[0],
            ];
            self.apply_torque(torque);
        }
    }

    /// Apply a torque to the body
    pub fn apply_torque(&mut self, torque: [f32; 3]) {
        if self.body_type == RigidBodyType::Dynamic {
            self.torques[0] += torque[0];
            self.torques[1] += torque[1];
            self.torques[2] += torque[2];
        }
    }

    /// Set the position of the body
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
        self.update_bounding_box();
    }

    /// Set the rotation of the body
    pub fn set_rotation(&mut self, rotation: [f32; 4]) {
        self.rotation = rotation;
        self.update_bounding_box();
    }

    /// Update the bounding box based on position and rotation
    fn update_bounding_box(&mut self) {
        // For now, just translate the collision shape's bounding box
        // In a full implementation, we would also apply rotation
        let shape_bbox = self.collision_shape.get_bounding_box();
        self.bounding_box = BoundingBox {
            min: [
                shape_bbox.min[0] + self.position[0],
                shape_bbox.min[1] + self.position[1],
                shape_bbox.min[2] + self.position[2],
            ],
            max: [
                shape_bbox.max[0] + self.position[0],
                shape_bbox.max[1] + self.position[1],
                shape_bbox.max[2] + self.position[2],
            ],
        };
    }

    /// Check if the body is moving slowly enough to sleep
    fn should_sleep(&self) -> bool {
        let linear_speed = (self.linear_velocity[0] * self.linear_velocity[0]
            + self.linear_velocity[1] * self.linear_velocity[1]
            + self.linear_velocity[2] * self.linear_velocity[2])
            .sqrt();

        let angular_speed = (self.angular_velocity[0] * self.angular_velocity[0]
            + self.angular_velocity[1] * self.angular_velocity[1]
            + self.angular_velocity[2] * self.angular_velocity[2])
            .sqrt();

        linear_speed < self.sleep_threshold && angular_speed < self.sleep_threshold
    }

    /// Update the body's sleep state
    pub fn update_sleep_state(&mut self, delta_time: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }

        if self.should_sleep() {
            self.sleep_time += delta_time;
            if self.sleep_time > 1.0 {
                // Sleep after 1 second of inactivity
                self.is_sleeping = true;
            }
        } else {
            self.sleep_time = 0.0;
            self.is_sleeping = false;
        }
    }
}

/// Contact point between two rigid bodies
#[derive(Debug, Clone)]
pub struct ContactPoint {
    /// Position of the contact
    pub position: [f32; 3],
    /// Normal vector at the contact
    pub normal: [f32; 3],
    /// Penetration depth
    pub penetration: f32,
    /// Contact point on body A
    pub point_a: [f32; 3],
    /// Contact point on body B
    pub point_b: [f32; 3],
}

/// Collision between two rigid bodies
#[derive(Debug, Clone)]
pub struct Collision {
    /// First rigid body
    pub body_a: String,
    /// Second rigid body
    pub body_b: String,
    /// Contact points
    pub contacts: Vec<ContactPoint>,
    /// Combined friction
    pub friction: f32,
    /// Combined restitution
    pub restitution: f32,
}

/// Physics world for simulation
pub struct PhysicsWorld {
    /// World configuration
    pub config: PhysicsWorldConfig,
    /// Rigid bodies in the world
    pub bodies: HashMap<String, RigidBody>,
    /// Collisions detected this frame
    pub collisions: Vec<Collision>,
    /// Accumulated time
    pub accumulated_time: f32,
    /// Current time step
    pub current_time_step: f32,
}

impl PhysicsWorld {
    /// Create a new physics world
    pub fn new(config: PhysicsWorldConfig) -> Self {
        Self {
            config,
            bodies: HashMap::new(),
            collisions: Vec::new(),
            accumulated_time: 0.0,
            current_time_step: 0.0,
        }
    }

    /// Add a rigid body to the world
    pub fn add_body(&mut self, body: RigidBody) -> Result<()> {
        if self.bodies.contains_key(&body.id) {
            return Err(WebGLError::lighting_error(
                "Body with this ID already exists",
            ));
        }

        self.bodies.insert(body.id.clone(), body);
        Ok(())
    }

    /// Remove a rigid body from the world
    pub fn remove_body(&mut self, id: &str) -> Option<RigidBody> {
        self.bodies.remove(id)
    }

    /// Get a rigid body by ID
    pub fn get_body(&self, id: &str) -> Option<&RigidBody> {
        self.bodies.get(id)
    }

    /// Get a mutable rigid body by ID
    pub fn get_body_mut(&mut self, id: &str) -> Option<&mut RigidBody> {
        self.bodies.get_mut(id)
    }

    /// Get all rigid bodies
    pub fn get_bodies(&self) -> &HashMap<String, RigidBody> {
        &self.bodies
    }

    /// Get all collisions
    pub fn get_collisions(&self) -> &Vec<Collision> {
        &self.collisions
    }

    /// Clear all collisions
    pub fn clear_collisions(&mut self) {
        self.collisions.clear();
    }

    /// Step the physics simulation
    pub fn step(&mut self, delta_time: f32) -> Result<()> {
        self.accumulated_time += delta_time;

        // Fixed time step simulation
        while self.accumulated_time >= self.config.time_step {
            self.current_time_step = self.config.time_step;
            self.accumulated_time -= self.config.time_step;

            // Update bodies
            self.update_bodies()?;

            // Detect collisions
            self.detect_collisions()?;

            // Resolve collisions
            self.resolve_collisions()?;
        }

        Ok(())
    }

    /// Update all rigid bodies
    fn update_bodies(&mut self) -> Result<()> {
        for (_, body) in self.bodies.iter_mut() {
            if body.body_type == RigidBodyType::Dynamic && !body.is_sleeping {
                // Apply gravity
                body.apply_force([
                    self.config.gravity[0] * body.mass,
                    self.config.gravity[1] * body.mass,
                    self.config.gravity[2] * body.mass,
                ]);

                // Update linear velocity
                body.linear_velocity[0] +=
                    body.forces[0] * body.inverse_mass * self.current_time_step;
                body.linear_velocity[1] +=
                    body.forces[1] * body.inverse_mass * self.current_time_step;
                body.linear_velocity[2] +=
                    body.forces[2] * body.inverse_mass * self.current_time_step;

                // Apply linear damping
                body.linear_velocity[0] *= body.linear_damping;
                body.linear_velocity[1] *= body.linear_damping;
                body.linear_velocity[2] *= body.linear_damping;

                // Update angular velocity
                body.angular_velocity[0] +=
                    body.torques[0] * body.inverse_inertia[0] * self.current_time_step;
                body.angular_velocity[1] +=
                    body.torques[1] * body.inverse_inertia[4] * self.current_time_step;
                body.angular_velocity[2] +=
                    body.torques[2] * body.inverse_inertia[8] * self.current_time_step;

                // Apply angular damping
                body.angular_velocity[0] *= body.angular_damping;
                body.angular_velocity[1] *= body.angular_damping;
                body.angular_velocity[2] *= body.angular_damping;

                // Update position
                body.position[0] += body.linear_velocity[0] * self.current_time_step;
                body.position[1] += body.linear_velocity[1] * self.current_time_step;
                body.position[2] += body.linear_velocity[2] * self.current_time_step;

                // Update bounding box
                body.update_bounding_box();

                // Clear forces and torques
                body.forces = [0.0, 0.0, 0.0];
                body.torques = [0.0, 0.0, 0.0];

                // Update sleep state
                body.update_sleep_state(self.current_time_step);
            }
        }

        Ok(())
    }

    /// Detect collisions between rigid bodies
    fn detect_collisions(&mut self) -> Result<()> {
        self.collisions.clear();

        let body_ids: Vec<String> = self.bodies.keys().cloned().collect();

        for i in 0..body_ids.len() {
            for j in (i + 1)..body_ids.len() {
                let id_a = &body_ids[i];
                let id_b = &body_ids[j];

                if let (Some(body_a), Some(body_b)) = (self.bodies.get(id_a), self.bodies.get(id_b))
                {
                    // Skip if both bodies are static
                    if body_a.body_type == RigidBodyType::Static
                        && body_b.body_type == RigidBodyType::Static
                    {
                        continue;
                    }

                    // Skip if both bodies are sleeping
                    if body_a.is_sleeping && body_b.is_sleeping {
                        continue;
                    }

                    // Check bounding box intersection first
                    if body_a.bounding_box.intersects(&body_b.bounding_box) {
                        // Perform detailed collision detection
                        if let Some(contact) = self.detect_collision_detailed(body_a, body_b) {
                            self.collisions.push(contact);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Perform detailed collision detection between two bodies
    fn detect_collision_detailed(
        &self,
        body_a: &RigidBody,
        body_b: &RigidBody,
    ) -> Option<Collision> {
        // Simple sphere-sphere collision detection for now
        if let (CollisionShape::Sphere { radius: r_a }, CollisionShape::Sphere { radius: r_b }) =
            (&body_a.collision_shape, &body_b.collision_shape)
        {
            let distance = ((body_a.position[0] - body_b.position[0]).powi(2)
                + (body_a.position[1] - body_b.position[1]).powi(2)
                + (body_a.position[2] - body_b.position[2]).powi(2))
            .sqrt();

            let penetration = (r_a + r_b) - distance;

            if penetration > 0.0 {
                let normal = [
                    (body_b.position[0] - body_a.position[0]) / distance,
                    (body_b.position[1] - body_a.position[1]) / distance,
                    (body_b.position[2] - body_a.position[2]) / distance,
                ];

                let contact_point = [
                    body_a.position[0] + normal[0] * r_a,
                    body_a.position[1] + normal[1] * r_a,
                    body_a.position[2] + normal[2] * r_a,
                ];

                let contact = ContactPoint {
                    position: contact_point,
                    normal,
                    penetration,
                    point_a: contact_point,
                    point_b: contact_point,
                };

                return Some(Collision {
                    body_a: body_a.id.clone(),
                    body_b: body_b.id.clone(),
                    contacts: vec![contact],
                    friction: (body_a.friction + body_b.friction) * 0.5,
                    restitution: (body_a.restitution + body_b.restitution) * 0.5,
                });
            }
        }

        None
    }

    /// Resolve collisions
    fn resolve_collisions(&mut self) -> Result<()> {
        for collision in &self.collisions {
            let body_a_id = collision.body_a.clone();
            let body_b_id = collision.body_b.clone();

            // Get velocities first to avoid borrow checker issues
            let (velocity_a, velocity_b) = if let (Some(body_a), Some(body_b)) =
                (self.bodies.get(&body_a_id), self.bodies.get(&body_b_id))
            {
                (body_a.linear_velocity, body_b.linear_velocity)
            } else {
                continue;
            };

            for contact in &collision.contacts {
                // Calculate relative velocity
                let relative_velocity = [
                    velocity_b[0] - velocity_a[0],
                    velocity_b[1] - velocity_a[1],
                    velocity_b[2] - velocity_a[2],
                ];

                // Calculate relative velocity along normal
                let velocity_along_normal = relative_velocity[0] * contact.normal[0]
                    + relative_velocity[1] * contact.normal[1]
                    + relative_velocity[2] * contact.normal[2];

                // Don't resolve if velocities are separating
                if velocity_along_normal > 0.0 {
                    continue;
                }

                // Calculate restitution
                let restitution = collision.restitution;

                // Get masses for impulse calculation
                let (mass_a, mass_b, type_a, type_b) = if let (Some(body_a), Some(body_b)) =
                    (self.bodies.get(&body_a_id), self.bodies.get(&body_b_id))
                {
                    (
                        body_a.inverse_mass,
                        body_b.inverse_mass,
                        body_a.body_type,
                        body_b.body_type,
                    )
                } else {
                    continue;
                };

                // Calculate impulse scalar
                let mut impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
                impulse_scalar /= mass_a + mass_b;

                // Apply impulse
                let impulse = [
                    impulse_scalar * contact.normal[0],
                    impulse_scalar * contact.normal[1],
                    impulse_scalar * contact.normal[2],
                ];

                // Apply velocity changes
                if let Some(body_a) = self.bodies.get_mut(&body_a_id) {
                    if type_a == RigidBodyType::Dynamic {
                        body_a.linear_velocity[0] -= impulse[0] * mass_a;
                        body_a.linear_velocity[1] -= impulse[1] * mass_a;
                        body_a.linear_velocity[2] -= impulse[2] * mass_a;
                    }
                }

                if let Some(body_b) = self.bodies.get_mut(&body_b_id) {
                    if type_b == RigidBodyType::Dynamic {
                        body_b.linear_velocity[0] += impulse[0] * mass_b;
                        body_b.linear_velocity[1] += impulse[1] * mass_b;
                        body_b.linear_velocity[2] += impulse[2] * mass_b;
                    }
                }

                // Position correction
                let correction_percent = 0.8;
                let correction_slop = 0.01;
                let correction = contact.penetration - correction_slop;

                if correction > 0.0 {
                    let correction_vector = [
                        correction * contact.normal[0] * correction_percent,
                        correction * contact.normal[1] * correction_percent,
                        correction * contact.normal[2] * correction_percent,
                    ];

                    if let Some(body_a) = self.bodies.get_mut(&body_a_id) {
                        if type_a == RigidBodyType::Dynamic {
                            let correction_factor = mass_a / (mass_a + mass_b);
                            body_a.position[0] -= correction_vector[0] * correction_factor;
                            body_a.position[1] -= correction_vector[1] * correction_factor;
                            body_a.position[2] -= correction_vector[2] * correction_factor;
                        }
                    }

                    if let Some(body_b) = self.bodies.get_mut(&body_b_id) {
                        if type_b == RigidBodyType::Dynamic {
                            let correction_factor = mass_b / (mass_a + mass_b);
                            body_b.position[0] += correction_vector[0] * correction_factor;
                            body_b.position[1] += correction_vector[1] * correction_factor;
                            body_b.position[2] += correction_vector[2] * correction_factor;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the total number of bodies
    pub fn get_body_count(&self) -> usize {
        self.bodies.len()
    }

    /// Get the total number of collisions
    pub fn get_collision_count(&self) -> usize {
        self.collisions.len()
    }

    /// Clear all bodies
    pub fn clear(&mut self) {
        self.bodies.clear();
        self.collisions.clear();
    }
}
