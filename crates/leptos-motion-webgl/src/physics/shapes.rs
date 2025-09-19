//! Collision shapes

use crate::{Result, WebGLError};

/// Collision shape types
#[derive(Debug, Clone, PartialEq)]
pub enum CollisionShape {
    /// Box shape
    Box {
        /// Width
        width: f32,
        /// Height
        height: f32,
        /// Depth
        depth: f32,
    },
    /// Sphere shape
    Sphere {
        /// Radius
        radius: f32,
    },
    /// Capsule shape
    Capsule {
        /// Radius
        radius: f32,
        /// Height
        height: f32,
    },
    /// Cylinder shape
    Cylinder {
        /// Half extents (x, y, z)
        half_extents: [f32; 3],
    },
    /// Plane shape
    Plane {
        /// Normal vector (x, y, z)
        normal: (f32, f32, f32),
        /// Distance from origin
        distance: f32,
    },
}

impl CollisionShape {
    /// Create a box shape
    pub fn box_shape(width: f32, height: f32, depth: f32) -> Self {
        Self::Box { width, height, depth }
    }

    /// Create a sphere shape
    pub fn sphere(radius: f32) -> Self {
        Self::Sphere { radius }
    }

    /// Create a capsule shape
    pub fn capsule(radius: f32, height: f32) -> Self {
        Self::Capsule { radius, height }
    }

    /// Create a cylinder shape
    pub fn cylinder(half_extents: [f32; 3]) -> Self {
        Self::Cylinder { half_extents }
    }

    /// Create a plane shape
    pub fn plane(normal: (f32, f32, f32), distance: f32) -> Self {
        Self::Plane { normal, distance }
    }

    /// Get the volume of the shape
    pub fn volume(&self) -> f32 {
        match self {
            CollisionShape::Box { width, height, depth } => width * height * depth,
            CollisionShape::Sphere { radius } => (4.0 / 3.0) * std::f32::consts::PI * radius * radius * radius,
            CollisionShape::Capsule { radius, height } => {
                let sphere_volume = (4.0 / 3.0) * std::f32::consts::PI * radius * radius * radius;
                let cylinder_volume = std::f32::consts::PI * radius * radius * height;
                sphere_volume + cylinder_volume
            }
            CollisionShape::Cylinder { half_extents } => {
                let radius = half_extents[0].max(half_extents[2]); // Use max of x and z as radius
                let height = half_extents[1] * 2.0; // Full height
                std::f32::consts::PI * radius * radius * height
            }
            CollisionShape::Plane { .. } => 0.0, // Planes have no volume
        }
    }

    /// Get the volume of the shape (alias for volume)
    pub fn get_volume(&self) -> f32 {
        self.volume()
    }

    /// Get the surface area of the shape
    pub fn surface_area(&self) -> f32 {
        match self {
            CollisionShape::Box { width, height, depth } => {
                2.0 * (width * height + width * depth + height * depth)
            }
            CollisionShape::Sphere { radius } => 4.0 * std::f32::consts::PI * radius * radius,
            CollisionShape::Capsule { radius, height } => {
                let sphere_area = 4.0 * std::f32::consts::PI * radius * radius;
                let cylinder_area = 2.0 * std::f32::consts::PI * radius * height;
                sphere_area + cylinder_area
            }
            CollisionShape::Cylinder { half_extents } => {
                let radius = half_extents[0].max(half_extents[2]); // Use max of x and z as radius
                let height = half_extents[1] * 2.0; // Full height
                let side_area = 2.0 * std::f32::consts::PI * radius * height;
                let top_bottom_area = 2.0 * std::f32::consts::PI * radius * radius;
                side_area + top_bottom_area
            }
            CollisionShape::Plane { .. } => f32::INFINITY, // Planes have infinite area
        }
    }

    /// Get the bounding box of the shape
    pub fn bounding_box(&self) -> (f32, f32, f32, f32, f32, f32) {
        match self {
            CollisionShape::Box { width, height, depth } => {
                (-width / 2.0, -height / 2.0, -depth / 2.0, width / 2.0, height / 2.0, depth / 2.0)
            }
            CollisionShape::Sphere { radius } => {
                (-radius, -radius, -radius, *radius, *radius, *radius)
            }
            CollisionShape::Capsule { radius, height } => {
                (-radius, -height / 2.0, -radius, *radius, height / 2.0, *radius)
            }
            CollisionShape::Cylinder { half_extents } => {
                (-half_extents[0], -half_extents[1], -half_extents[2], 
                 half_extents[0], half_extents[1], half_extents[2])
            }
            CollisionShape::Plane { .. } => {
                (-f32::INFINITY, -f32::INFINITY, -f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY)
            }
        }
    }

    /// Get the bounding box of the shape (alias for bounding_box)
    pub fn get_bounding_box(&self) -> (f32, f32, f32, f32, f32, f32) {
        self.bounding_box()
    }

    /// Check if the shape is valid
    pub fn is_valid(&self) -> bool {
        match self {
            CollisionShape::Box { width, height, depth } => {
                *width > 0.0 && *height > 0.0 && *depth > 0.0
            }
            CollisionShape::Sphere { radius } => *radius > 0.0,
            CollisionShape::Capsule { radius, height } => *radius > 0.0 && *height > 0.0,
            CollisionShape::Cylinder { half_extents } => {
                half_extents[0] > 0.0 && half_extents[1] > 0.0 && half_extents[2] > 0.0
            }
            CollisionShape::Plane { normal, .. } => {
                let (x, y, z) = *normal;
                let length = (x * x + y * y + z * z).sqrt();
                length > 0.0
            }
        }
    }

    /// Get the inertia tensor for the shape
    pub fn inertia_tensor(&self, mass: f32) -> (f32, f32, f32) {
        if mass <= 0.0 {
            return (0.0, 0.0, 0.0);
        }

        match self {
            CollisionShape::Box { width, height, depth } => {
                let w2 = width * width;
                let h2 = height * height;
                let d2 = depth * depth;
                (
                    mass * (h2 + d2) / 12.0,
                    mass * (w2 + d2) / 12.0,
                    mass * (w2 + h2) / 12.0,
                )
            }
            CollisionShape::Sphere { radius } => {
                let r2 = radius * radius;
                let inertia = 2.0 * mass * r2 / 5.0;
                (inertia, inertia, inertia)
            }
            CollisionShape::Capsule { radius, height } => {
                let r2 = radius * radius;
                let h2 = height * height;
                let sphere_mass = mass * 0.5; // Approximate
                let cylinder_mass = mass * 0.5; // Approximate
                
                let sphere_inertia = 2.0 * sphere_mass * r2 / 5.0;
                let cylinder_inertia_x = cylinder_mass * (3.0 * r2 + h2) / 12.0;
                let cylinder_inertia_y = cylinder_mass * r2 / 2.0;
                let cylinder_inertia_z = cylinder_mass * (3.0 * r2 + h2) / 12.0;
                
                (
                    sphere_inertia + cylinder_inertia_x,
                    sphere_inertia + cylinder_inertia_y,
                    sphere_inertia + cylinder_inertia_z,
                )
            }
            CollisionShape::Cylinder { half_extents } => {
                let radius = half_extents[0].max(half_extents[2]); // Use max of x and z as radius
                let height = half_extents[1] * 2.0; // Full height
                let r2 = radius * radius;
                let h2 = height * height;
                
                (
                    mass * (3.0 * r2 + h2) / 12.0, // Ixx
                    mass * r2 / 2.0,                // Iyy
                    mass * (3.0 * r2 + h2) / 12.0, // Izz
                )
            }
            CollisionShape::Plane { .. } => (0.0, 0.0, 0.0), // Planes have no inertia
        }
    }

    /// Scale the shape by a factor
    pub fn scale(&self, factor: f32) -> Self {
        match self {
            CollisionShape::Box { width, height, depth } => {
                Self::Box {
                    width: width * factor,
                    height: height * factor,
                    depth: depth * factor,
                }
            }
            CollisionShape::Sphere { radius } => {
                Self::Sphere { radius: radius * factor }
            }
            CollisionShape::Capsule { radius, height } => {
                Self::Capsule {
                    radius: radius * factor,
                    height: height * factor,
                }
            }
            CollisionShape::Cylinder { half_extents } => {
                Self::Cylinder {
                    half_extents: [
                        half_extents[0] * factor,
                        half_extents[1] * factor,
                        half_extents[2] * factor,
                    ],
                }
            }
            CollisionShape::Plane { normal, distance } => {
                Self::Plane {
                    normal: *normal,
                    distance: distance * factor,
                }
            }
        }
    }

    /// Get the center of mass (always at origin for these shapes)
    pub fn center_of_mass(&self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }

    /// Check if two shapes can collide
    pub fn can_collide_with(&self, other: &CollisionShape) -> bool {
        match (self, other) {
            (CollisionShape::Plane { .. }, CollisionShape::Plane { .. }) => false,
            _ => true,
        }
    }

    /// Get the type name of the shape
    pub fn type_name(&self) -> &'static str {
        match self {
            CollisionShape::Box { .. } => "Box",
            CollisionShape::Sphere { .. } => "Sphere",
            CollisionShape::Capsule { .. } => "Capsule",
            CollisionShape::Cylinder { .. } => "Cylinder",
            CollisionShape::Plane { .. } => "Plane",
        }
    }
}
