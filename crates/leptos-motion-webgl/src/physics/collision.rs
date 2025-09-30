//! Collision detection system

use super::*;
use crate::Result;

/// Collision detection system
#[derive(Debug)]
pub struct CollisionDetector {
    /// Broad phase collision detection
    broad_phase: BroadPhaseDetector,
    /// Narrow phase collision detection
    narrow_phase: NarrowPhaseDetector,
    /// Collision pairs
    collision_pairs: Vec<(u64, u64)>,
    /// Active collisions
    active_collisions: std::collections::HashMap<(u64, u64), Collision>,
}

/// Broad phase collision detection
#[derive(Debug, Clone)]
pub struct BroadPhaseDetector {
    /// Spatial partitioning grid
    grid: SpatialGrid,
    /// Whether to use spatial partitioning
    use_spatial_partitioning: bool,
}

/// Narrow phase collision detection
#[derive(Debug)]
pub struct NarrowPhaseDetector {
    /// Collision algorithms - using string keys instead of shape tuples
    algorithms: std::collections::HashMap<String, Box<dyn CollisionAlgorithm>>,
}

/// Spatial grid for broad phase detection
#[derive(Debug, Clone)]
pub struct SpatialGrid {
    /// Grid cell size
    cell_size: f32,
    /// Grid cells
    cells: std::collections::HashMap<(i32, i32, i32), Vec<u64>>,
}

/// Collision algorithm trait
pub trait CollisionAlgorithm: std::fmt::Debug {
    /// Detect collision between two shapes
    fn detect_collision(&self, shape_a: &CollisionShape, shape_b: &CollisionShape) -> Option<Collision>;
}

/// Box-Box collision algorithm
#[derive(Debug)]
pub struct BoxBoxCollision;

/// Sphere-Sphere collision algorithm
#[derive(Debug)]
pub struct SphereSphereCollision;

/// Box-Sphere collision algorithm
#[derive(Debug)]
pub struct BoxSphereCollision;

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl CollisionDetector {
    /// Create a new collision detector
    pub fn new() -> Self {
        Self {
            broad_phase: BroadPhaseDetector::new(),
            narrow_phase: NarrowPhaseDetector::new(),
            collision_pairs: Vec::new(),
            active_collisions: std::collections::HashMap::new(),
        }
    }

    /// Detect collisions between all bodies
    pub fn detect_collisions(&mut self, bodies: &[RigidBody]) -> Result<Vec<Collision>> {
        let mut collisions = Vec::new();

        // Broad phase: find potential collision pairs
        let pairs = self.broad_phase.find_potential_pairs(bodies)?;

        // Narrow phase: detailed collision detection
        for (body_a_id, body_b_id) in pairs {
            if let (Some(body_a), Some(body_b)) = (
                bodies.iter().find(|b| b.id == body_a_id),
                bodies.iter().find(|b| b.id == body_b_id),
            ) {
                if let Some(collision) = self.narrow_phase.detect_collision(body_a, body_b)? {
                    collisions.push(collision);
                }
            }
        }

        Ok(collisions)
    }

    /// Update collision pairs
    pub fn update_collision_pairs(&mut self, bodies: &[RigidBody]) -> Result<()> {
        self.collision_pairs = self.broad_phase.find_potential_pairs(bodies)?;
        Ok(())
    }

    /// Get active collisions
    pub fn active_collisions(&self) -> &std::collections::HashMap<(u64, u64), Collision> {
        &self.active_collisions
    }

    /// Clear all collisions
    pub fn clear_collisions(&mut self) {
        self.active_collisions.clear();
        self.collision_pairs.clear();
    }
}

impl Default for BroadPhaseDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl BroadPhaseDetector {
    /// Create a new broad phase detector
    pub fn new() -> Self {
        Self {
            grid: SpatialGrid::new(10.0),
            use_spatial_partitioning: true,
        }
    }

    /// Find potential collision pairs
    pub fn find_potential_pairs(&mut self, bodies: &[RigidBody]) -> Result<Vec<(u64, u64)>> {
        if self.use_spatial_partitioning {
            self.grid.clear();
            for body in bodies {
                if body.is_active {
                    self.grid.insert(body.id, &body.bounding_box);
                }
            }
            Ok(self.grid.find_potential_pairs())
        } else {
            // Brute force approach
            let mut pairs = Vec::new();
            for i in 0..bodies.len() {
                for j in (i + 1)..bodies.len() {
                    if bodies[i].is_active && bodies[j].is_active
                        && bodies[i].bounding_box.intersects(&bodies[j].bounding_box) {
                            pairs.push((bodies[i].id, bodies[j].id));
                        }
                }
            }
            Ok(pairs)
        }
    }
}

impl Default for NarrowPhaseDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl NarrowPhaseDetector {
    /// Create a new narrow phase detector
    pub fn new() -> Self {
        let mut algorithms = std::collections::HashMap::new();
        
        // Register collision algorithms
        algorithms.insert("box-box".to_string(), Box::new(BoxBoxCollision) as Box<dyn CollisionAlgorithm>);
        algorithms.insert("sphere-sphere".to_string(), Box::new(SphereSphereCollision) as Box<dyn CollisionAlgorithm>);
        algorithms.insert("box-sphere".to_string(), Box::new(BoxSphereCollision) as Box<dyn CollisionAlgorithm>);

        Self { algorithms }
    }

    /// Detect collision between two bodies
    pub fn detect_collision(&self, body_a: &RigidBody, body_b: &RigidBody) -> Result<Option<Collision>> {
        // Skip collision detection for static-static pairs
        if body_a.is_static() && body_b.is_static() {
            return Ok(None);
        }

        // Skip collision detection for sleeping bodies
        if body_a.is_sleeping && body_b.is_sleeping {
            return Ok(None);
        }

        // Find appropriate collision algorithm
        let algorithm_key = (body_a.collision_shape.clone(), body_b.collision_shape.clone());
        
        // For now, use a simplified collision detection
        if body_a.bounding_box.intersects(&body_b.bounding_box) {
            let mut collision = Collision::new(body_a.id, body_b.id);
            
            // Create a simple contact point
            let center_a = body_a.bounding_box.center();
            let center_b = body_b.bounding_box.center();
            let distance = (
                center_b.0 - center_a.0,
                center_b.1 - center_a.1,
                center_b.2 - center_a.2,
            );
            let distance_magnitude = (distance.0 * distance.0 + distance.1 * distance.1 + distance.2 * distance.2).sqrt();
            
            if distance_magnitude > 0.0 {
                let normal = (
                    distance.0 / distance_magnitude,
                    distance.1 / distance_magnitude,
                    distance.2 / distance_magnitude,
                );
                let penetration = 1.0; // Simplified
                let contact = ContactPoint::new(center_a, normal, penetration);
                collision.add_contact_point(contact);
            }
            
            Ok(Some(collision))
        } else {
            Ok(None)
        }
    }
}

impl SpatialGrid {
    /// Create a new spatial grid
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: std::collections::HashMap::new(),
        }
    }

    /// Insert a body into the grid
    pub fn insert(&mut self, body_id: u64, bounding_box: &BoundingBox) {
        let min_cell = self.world_to_cell(bounding_box.min);
        let max_cell = self.world_to_cell(bounding_box.max);

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                for z in min_cell.2..=max_cell.2 {
                    self.cells.entry((x, y, z)).or_default().push(body_id);
                }
            }
        }
    }

    /// Find potential collision pairs
    pub fn find_potential_pairs(&self) -> Vec<(u64, u64)> {
        let mut pairs = Vec::new();
        let mut processed = std::collections::HashSet::new();

        for cell_bodies in self.cells.values() {
            for i in 0..cell_bodies.len() {
                for j in (i + 1)..cell_bodies.len() {
                    let body_a = cell_bodies[i];
                    let body_b = cell_bodies[j];
                    let pair = if body_a < body_b { (body_a, body_b) } else { (body_b, body_a) };
                    
                    if !processed.contains(&pair) {
                        pairs.push(pair);
                        processed.insert(pair);
                    }
                }
            }
        }

        pairs
    }

    /// Clear the grid
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Convert world coordinates to grid cell coordinates
    fn world_to_cell(&self, world_pos: (f32, f32, f32)) -> (i32, i32, i32) {
        (
            (world_pos.0 / self.cell_size).floor() as i32,
            (world_pos.1 / self.cell_size).floor() as i32,
            (world_pos.2 / self.cell_size).floor() as i32,
        )
    }
}

impl CollisionAlgorithm for BoxBoxCollision {
    fn detect_collision(&self, _shape_a: &CollisionShape, _shape_b: &CollisionShape) -> Option<Collision> {
        // Simplified implementation
        None
    }
}

impl CollisionAlgorithm for SphereSphereCollision {
    fn detect_collision(&self, _shape_a: &CollisionShape, _shape_b: &CollisionShape) -> Option<Collision> {
        // Simplified implementation
        None
    }
}

impl CollisionAlgorithm for BoxSphereCollision {
    fn detect_collision(&self, _shape_a: &CollisionShape, _shape_b: &CollisionShape) -> Option<Collision> {
        // Simplified implementation
        None
    }
}
