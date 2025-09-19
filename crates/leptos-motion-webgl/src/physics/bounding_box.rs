//! Bounding box implementation

use crate::{Result, WebGLError};

/// 3D bounding box
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    /// Minimum corner (x, y, z)
    pub min: (f32, f32, f32),
    /// Maximum corner (x, y, z)
    pub max: (f32, f32, f32),
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::empty()
    }
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
        Self { min, max }
    }

    /// Create a new empty bounding box (default constructor)
    pub fn empty() -> Self {
        Self {
            min: (f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: (-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
        }
    }

    /// Create a bounding box from center and size
    pub fn from_center_size(center: (f32, f32, f32), size: (f32, f32, f32)) -> Self {
        let half_size = (size.0 / 2.0, size.1 / 2.0, size.2 / 2.0);
        Self {
            min: (center.0 - half_size.0, center.1 - half_size.1, center.2 - half_size.2),
            max: (center.0 + half_size.0, center.1 + half_size.1, center.2 + half_size.2),
        }
    }


    /// Check if the bounding box is valid
    pub fn is_valid(&self) -> bool {
        self.min.0 <= self.max.0 && self.min.1 <= self.max.1 && self.min.2 <= self.max.2
    }

    /// Check if the bounding box is empty
    pub fn is_empty(&self) -> bool {
        self.min.0 > self.max.0 || self.min.1 > self.max.1 || self.min.2 > self.max.2
    }

    /// Get the center of the bounding box
    pub fn center(&self) -> (f32, f32, f32) {
        (
            (self.min.0 + self.max.0) / 2.0,
            (self.min.1 + self.max.1) / 2.0,
            (self.min.2 + self.max.2) / 2.0,
        )
    }

    /// Get the size of the bounding box
    pub fn size(&self) -> (f32, f32, f32) {
        (
            self.max.0 - self.min.0,
            self.max.1 - self.min.1,
            self.max.2 - self.min.2,
        )
    }

    /// Get the volume of the bounding box
    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.0 * size.1 * size.2
    }

    /// Get the surface area of the bounding box
    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2.0 * (size.0 * size.1 + size.0 * size.2 + size.1 * size.2)
    }

    /// Check if a point is inside the bounding box
    pub fn contains_point(&self, point: (f32, f32, f32)) -> bool {
        point.0 >= self.min.0 && point.0 <= self.max.0 &&
        point.1 >= self.min.1 && point.1 <= self.max.1 &&
        point.2 >= self.min.2 && point.2 <= self.max.2
    }

    /// Check if another bounding box is inside this one
    pub fn contains_box(&self, other: &BoundingBox) -> bool {
        self.min.0 <= other.min.0 && self.max.0 >= other.max.0 &&
        self.min.1 <= other.min.1 && self.max.1 >= other.max.1 &&
        self.min.2 <= other.min.2 && self.max.2 >= other.max.2
    }

    /// Check if this bounding box intersects with another
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.0 <= other.max.0 && self.max.0 >= other.min.0 &&
        self.min.1 <= other.max.1 && self.max.1 >= other.min.1 &&
        self.min.2 <= other.max.2 && self.max.2 >= other.min.2
    }

    /// Expand the bounding box to include a point
    pub fn expand_to_include_point(&mut self, point: (f32, f32, f32)) {
        self.min.0 = self.min.0.min(point.0);
        self.min.1 = self.min.1.min(point.1);
        self.min.2 = self.min.2.min(point.2);
        self.max.0 = self.max.0.max(point.0);
        self.max.1 = self.max.1.max(point.1);
        self.max.2 = self.max.2.max(point.2);
    }

    /// Expand the bounding box to include another bounding box
    pub fn expand_to_include_box(&mut self, other: &BoundingBox) {
        self.min.0 = self.min.0.min(other.min.0);
        self.min.1 = self.min.1.min(other.min.1);
        self.min.2 = self.min.2.min(other.min.2);
        self.max.0 = self.max.0.max(other.max.0);
        self.max.1 = self.max.1.max(other.max.1);
        self.max.2 = self.max.2.max(other.max.2);
    }

    /// Expand the bounding box by a margin
    pub fn expand(&mut self, margin: f32) {
        self.min.0 -= margin;
        self.min.1 -= margin;
        self.min.2 -= margin;
        self.max.0 += margin;
        self.max.1 += margin;
        self.max.2 += margin;
    }

    /// Expand the bounding box by different margins for each axis
    pub fn expand_xyz(&mut self, margin_x: f32, margin_y: f32, margin_z: f32) {
        self.min.0 -= margin_x;
        self.min.1 -= margin_y;
        self.min.2 -= margin_z;
        self.max.0 += margin_x;
        self.max.1 += margin_y;
        self.max.2 += margin_z;
    }

    /// Transform the bounding box by a matrix (simplified)
    pub fn transform(&self, matrix: &[f32; 16]) -> Self {
        // Simplified transformation - in a real implementation, you'd properly transform all 8 corners
        let center = self.center();
        let size = self.size();
        
        // For now, just return the original box
        // In a real implementation, you'd transform all 8 corners and create a new bounding box
        Self::from_center_size(center, size)
    }

    /// Get the distance to a point
    pub fn distance_to_point(&self, point: (f32, f32, f32)) -> f32 {
        let dx = if point.0 < self.min.0 {
            self.min.0 - point.0
        } else if point.0 > self.max.0 {
            point.0 - self.max.0
        } else {
            0.0
        };

        let dy = if point.1 < self.min.1 {
            self.min.1 - point.1
        } else if point.1 > self.max.1 {
            point.1 - self.max.1
        } else {
            0.0
        };

        let dz = if point.2 < self.min.2 {
            self.min.2 - point.2
        } else if point.2 > self.max.2 {
            point.2 - self.max.2
        } else {
            0.0
        };

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get the distance to another bounding box
    pub fn distance_to_box(&self, other: &BoundingBox) -> f32 {
        let dx = if self.max.0 < other.min.0 {
            other.min.0 - self.max.0
        } else if other.max.0 < self.min.0 {
            self.min.0 - other.max.0
        } else {
            0.0
        };

        let dy = if self.max.1 < other.min.1 {
            other.min.1 - self.max.1
        } else if other.max.1 < self.min.1 {
            self.min.1 - other.max.1
        } else {
            0.0
        };

        let dz = if self.max.2 < other.min.2 {
            other.min.2 - self.max.2
        } else if other.max.2 < self.min.2 {
            self.min.2 - other.max.2
        } else {
            0.0
        };

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get the intersection with another bounding box
    pub fn intersection(&self, other: &BoundingBox) -> Option<BoundingBox> {
        if !self.intersects(other) {
            return None;
        }

        Some(BoundingBox::new(
            (
                self.min.0.max(other.min.0),
                self.min.1.max(other.min.1),
                self.min.2.max(other.min.2),
            ),
            (
                self.max.0.min(other.max.0),
                self.max.1.min(other.max.1),
                self.max.2.min(other.max.2),
            ),
        ))
    }

    /// Get the union with another bounding box
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            (
                self.min.0.min(other.min.0),
                self.min.1.min(other.min.1),
                self.min.2.min(other.min.2),
            ),
            (
                self.max.0.max(other.max.0),
                self.max.1.max(other.max.1),
                self.max.2.max(other.max.2),
            ),
        )
    }

    /// Get the 8 corners of the bounding box
    pub fn corners(&self) -> [(f32, f32, f32); 8] {
        [
            (self.min.0, self.min.1, self.min.2),
            (self.max.0, self.min.1, self.min.2),
            (self.min.0, self.max.1, self.min.2),
            (self.max.0, self.max.1, self.min.2),
            (self.min.0, self.min.1, self.max.2),
            (self.max.0, self.min.1, self.max.2),
            (self.min.0, self.max.1, self.max.2),
            (self.max.0, self.max.1, self.max.2),
        ]
    }
}
