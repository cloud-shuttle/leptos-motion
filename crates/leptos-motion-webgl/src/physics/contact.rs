//! Contact point and collision information


/// Contact point between two rigid bodies
#[derive(Debug, Clone)]
pub struct ContactPoint {
    /// Position of the contact point (x, y, z)
    pub position: (f32, f32, f32),
    /// Normal vector at the contact point (x, y, z)
    pub normal: (f32, f32, f32),
    /// Penetration depth
    pub penetration: f32,
    /// Contact point on body A (relative to body A's center)
    pub point_a: (f32, f32, f32),
    /// Contact point on body B (relative to body B's center)
    pub point_b: (f32, f32, f32),
    /// Contact impulse
    pub impulse: f32,
    /// Friction impulse
    pub friction_impulse: (f32, f32),
    /// Whether this contact is valid
    pub is_valid: bool,
}

impl ContactPoint {
    /// Create a new contact point
    pub fn new(
        position: (f32, f32, f32),
        normal: (f32, f32, f32),
        penetration: f32,
    ) -> Self {
        Self {
            position,
            normal,
            penetration,
            point_a: (0.0, 0.0, 0.0),
            point_b: (0.0, 0.0, 0.0),
            impulse: 0.0,
            friction_impulse: (0.0, 0.0),
            is_valid: true,
        }
    }

    /// Set contact points relative to bodies
    pub fn set_contact_points(&mut self, point_a: (f32, f32, f32), point_b: (f32, f32, f32)) {
        self.point_a = point_a;
        self.point_b = point_b;
    }

    /// Set contact impulse
    pub fn set_impulse(&mut self, impulse: f32) {
        self.impulse = impulse;
    }

    /// Set friction impulse
    pub fn set_friction_impulse(&mut self, friction_impulse: (f32, f32)) {
        self.friction_impulse = friction_impulse;
    }

    /// Invalidate the contact
    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    /// Check if the contact is valid
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get the contact force magnitude
    pub fn force_magnitude(&self) -> f32 {
        self.impulse.abs()
    }

    /// Get the friction force magnitude
    pub fn friction_force_magnitude(&self) -> f32 {
        (self.friction_impulse.0 * self.friction_impulse.0 + 
         self.friction_impulse.1 * self.friction_impulse.1).sqrt()
    }

    /// Get the total contact force
    pub fn total_force(&self) -> f32 {
        (self.force_magnitude() * self.force_magnitude() + 
         self.friction_force_magnitude() * self.friction_force_magnitude()).sqrt()
    }
}

/// Collision information between two rigid bodies
#[derive(Debug, Clone)]
pub struct Collision {
    /// ID of body A
    pub body_a_id: u64,
    /// ID of body B
    pub body_b_id: u64,
    /// Contact points
    pub contact_points: Vec<ContactPoint>,
    /// Whether the collision is valid
    pub is_valid: bool,
    /// Collision timestamp
    pub timestamp: f64,
}

impl Collision {
    /// Create a new collision
    pub fn new(body_a_id: u64, body_b_id: u64) -> Self {
        Self {
            body_a_id,
            body_b_id,
            contact_points: Vec::new(),
            is_valid: true,
            timestamp: 0.0,
        }
    }

    /// Add a contact point
    pub fn add_contact_point(&mut self, contact: ContactPoint) {
        self.contact_points.push(contact);
    }

    /// Get the number of contact points
    pub fn contact_count(&self) -> usize {
        self.contact_points.len()
    }

    /// Get the first contact point
    pub fn first_contact(&self) -> Option<&ContactPoint> {
        self.contact_points.first()
    }

    /// Get the deepest contact point
    pub fn deepest_contact(&self) -> Option<&ContactPoint> {
        self.contact_points.iter().max_by(|a, b| {
            a.penetration.partial_cmp(&b.penetration).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get the total penetration depth
    pub fn total_penetration(&self) -> f32 {
        self.contact_points.iter().map(|c| c.penetration).sum()
    }

    /// Get the average penetration depth
    pub fn average_penetration(&self) -> f32 {
        if self.contact_points.is_empty() {
            0.0
        } else {
            self.total_penetration() / self.contact_points.len() as f32
        }
    }

    /// Get the maximum penetration depth
    pub fn max_penetration(&self) -> f32 {
        self.contact_points.iter().map(|c| c.penetration).fold(0.0, f32::max)
    }

    /// Get the total contact impulse
    pub fn total_impulse(&self) -> f32 {
        self.contact_points.iter().map(|c| c.impulse).sum()
    }

    /// Get the total friction impulse
    pub fn total_friction_impulse(&self) -> (f32, f32) {
        self.contact_points.iter().fold((0.0, 0.0), |acc, c| {
            (acc.0 + c.friction_impulse.0, acc.1 + c.friction_impulse.1)
        })
    }

    /// Invalidate the collision
    pub fn invalidate(&mut self) {
        self.is_valid = false;
        for contact in &mut self.contact_points {
            contact.invalidate();
        }
    }

    /// Check if the collision is valid
    pub fn is_valid(&self) -> bool {
        self.is_valid && !self.contact_points.is_empty()
    }

    /// Set the collision timestamp
    pub fn set_timestamp(&mut self, timestamp: f64) {
        self.timestamp = timestamp;
    }

    /// Get the collision timestamp
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    /// Clear all contact points
    pub fn clear_contacts(&mut self) {
        self.contact_points.clear();
    }

    /// Check if this collision involves a specific body
    pub fn involves_body(&self, body_id: u64) -> bool {
        self.body_a_id == body_id || self.body_b_id == body_id
    }

    /// Get the other body ID
    pub fn other_body_id(&self, body_id: u64) -> Option<u64> {
        if self.body_a_id == body_id {
            Some(self.body_b_id)
        } else if self.body_b_id == body_id {
            Some(self.body_a_id)
        } else {
            None
        }
    }

    /// Get the collision normal (from the first contact point)
    pub fn collision_normal(&self) -> Option<(f32, f32, f32)> {
        self.first_contact().map(|c| c.normal)
    }

    /// Get the collision center (average of all contact points)
    pub fn collision_center(&self) -> (f32, f32, f32) {
        if self.contact_points.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let total_x: f32 = self.contact_points.iter().map(|c| c.position.0).sum();
        let total_y: f32 = self.contact_points.iter().map(|c| c.position.1).sum();
        let total_z: f32 = self.contact_points.iter().map(|c| c.position.2).sum();
        let count = self.contact_points.len() as f32;

        (total_x / count, total_y / count, total_z / count)
    }

    /// Check if the collision is persistent (multiple contact points)
    pub fn is_persistent(&self) -> bool {
        self.contact_points.len() > 1
    }

    /// Get the collision severity (based on penetration depth)
    pub fn severity(&self) -> f32 {
        self.max_penetration()
    }

    /// Check if the collision is severe
    pub fn is_severe(&self) -> bool {
        self.severity() > 0.1 // Threshold for severe collision
    }
}
