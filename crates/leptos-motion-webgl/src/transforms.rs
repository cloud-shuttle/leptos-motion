//! 3D Transform utilities for WebGL rendering


/// 3D transformation matrix and utilities
#[derive(Debug, Clone, PartialEq)]
pub struct Transform3D {
    /// Position in 3D space
    pub position: [f32; 3],
    /// Rotation in 3D space (Euler angles in radians)
    pub rotation: [f32; 3],
    /// Scale in 3D space
    pub scale: [f32; 3],
    /// Transformation matrix (4x4)
    pub matrix: [f32; 16],
}

impl Transform3D {
    /// Create a new identity transform
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            matrix: Self::identity_matrix(),
        }
    }

    /// Create a transform with position
    pub fn with_position(x: f32, y: f32, z: f32) -> Self {
        let mut transform = Self::new();
        transform.position = [x, y, z];
        transform.update_matrix();
        transform
    }

    /// Create a transform with rotation
    pub fn with_rotation(x: f32, y: f32, z: f32) -> Self {
        let mut transform = Self::new();
        transform.rotation = [x, y, z];
        transform.update_matrix();
        transform
    }

    /// Create a transform with scale
    pub fn with_scale(x: f32, y: f32, z: f32) -> Self {
        let mut transform = Self::new();
        transform.scale = [x, y, z];
        transform.update_matrix();
        transform
    }

    /// Set position
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
        self.update_matrix();
    }

    /// Set rotation
    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = [x, y, z];
        self.update_matrix();
    }

    /// Set scale
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = [x, y, z];
        self.update_matrix();
    }

    /// Get the transformation matrix
    pub fn get_matrix(&self) -> &[f32; 16] {
        &self.matrix
    }

    /// Update the transformation matrix based on position, rotation, and scale
    pub fn update_matrix(&mut self) {
        self.matrix = Self::create_matrix(
            self.position,
            self.rotation,
            self.scale,
        );
    }

    /// Create a 4x4 transformation matrix
    fn create_matrix(position: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) -> [f32; 16] {
        let mut matrix = Self::identity_matrix();

        // Apply translation
        matrix[12] = position[0];
        matrix[13] = position[1];
        matrix[14] = position[2];

        // Apply scale
        matrix[0] *= scale[0];
        matrix[5] *= scale[1];
        matrix[10] *= scale[2];

        // Apply rotation (simplified - would need proper rotation matrix in real implementation)
        // For now, just apply basic rotation
        let cos_x = rotation[0].cos();
        let sin_x = rotation[0].sin();
        let cos_y = rotation[1].cos();
        let sin_y = rotation[1].sin();
        let cos_z = rotation[2].cos();
        let sin_z = rotation[2].sin();

        // This is a simplified rotation matrix - in a real implementation,
        // you'd want to use proper matrix multiplication for all three rotations
        matrix[0] *= cos_z * cos_y;
        matrix[1] *= cos_z * sin_y * sin_x - sin_z * cos_x;
        matrix[2] *= cos_z * sin_y * cos_x + sin_z * sin_x;
        matrix[4] *= sin_z * cos_y;
        matrix[5] *= sin_z * sin_y * sin_x + cos_z * cos_x;
        matrix[6] *= sin_z * sin_y * cos_x - cos_z * sin_x;
        matrix[8] *= -sin_y;
        matrix[9] *= cos_y * sin_x;
        matrix[10] *= cos_y * cos_x;

        matrix
    }

    /// Get identity matrix
    fn identity_matrix() -> [f32; 16] {
        [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]
    }

    /// Check if this is an identity transform
    pub fn is_identity(&self) -> bool {
        self.position == [0.0, 0.0, 0.0] &&
        self.rotation == [0.0, 0.0, 0.0] &&
        self.scale == [1.0, 1.0, 1.0]
    }

    /// Combine with another transform
    pub fn combine(&self, other: &Transform3D) -> Transform3D {
        let mut result = Self::new();
        
        // Combine positions
        result.position[0] = self.position[0] + other.position[0];
        result.position[1] = self.position[1] + other.position[1];
        result.position[2] = self.position[2] + other.position[2];

        // Combine rotations
        result.rotation[0] = self.rotation[0] + other.rotation[0];
        result.rotation[1] = self.rotation[1] + other.rotation[1];
        result.rotation[2] = self.rotation[2] + other.rotation[2];

        // Combine scales
        result.scale[0] = self.scale[0] * other.scale[0];
        result.scale[1] = self.scale[1] * other.scale[1];
        result.scale[2] = self.scale[2] * other.scale[2];

        result.update_matrix();
        result
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for matrix operations
pub mod matrix_utils {
    

    /// Multiply two 4x4 matrices
    pub fn multiply_matrices(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
        let mut result = [0.0; 16];
        
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
                }
            }
        }
        
        result
    }

    /// Create a perspective projection matrix
    pub fn perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
        let f = 1.0 / (fov / 2.0).tan();
        let range_inv = 1.0 / (near - far);

        [
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (near + far) * range_inv, -1.0,
            0.0, 0.0, near * far * range_inv * 2.0, 0.0,
        ]
    }

    /// Create an orthographic projection matrix
    pub fn orthographic_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
        let lr = 1.0 / (left - right);
        let bt = 1.0 / (bottom - top);
        let nf = 1.0 / (near - far);

        [
            2.0 * lr, 0.0, 0.0, 0.0,
            0.0, 2.0 * bt, 0.0, 0.0,
            0.0, 0.0, 2.0 * nf, 0.0,
            (left + right) * lr, (top + bottom) * bt, (far + near) * nf, 1.0,
        ]
    }

    /// Create a look-at view matrix
    pub fn look_at_matrix(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [f32; 16] {
        let f = normalize([center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]);
        let s = normalize(cross(f, up));
        let u = cross(s, f);

        [
            s[0], u[0], -f[0], 0.0,
            s[1], u[1], -f[1], 0.0,
            s[2], u[2], -f[2], 0.0,
            -dot(s, eye), -dot(u, eye), dot(f, eye), 1.0,
        ]
    }

    /// Normalize a 3D vector
    fn normalize(v: [f32; 3]) -> [f32; 3] {
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if length > 0.0 {
            [v[0] / length, v[1] / length, v[2] / length]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    /// Cross product of two 3D vectors
    fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    /// Dot product of two 3D vectors
    fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform3d_creation() {
        let transform = Transform3D::new();
        assert_eq!(transform.position, [0.0, 0.0, 0.0]);
        assert_eq!(transform.rotation, [0.0, 0.0, 0.0]);
        assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
        assert!(transform.is_identity());
    }

    #[test]
    fn test_transform3d_with_position() {
        let transform = Transform3D::with_position(1.0, 2.0, 3.0);
        assert_eq!(transform.position, [1.0, 2.0, 3.0]);
        assert!(!transform.is_identity());
    }

    #[test]
    fn test_transform3d_with_rotation() {
        let transform = Transform3D::with_rotation(0.5, 1.0, 1.5);
        assert_eq!(transform.rotation, [0.5, 1.0, 1.5]);
        assert!(!transform.is_identity());
    }

    #[test]
    fn test_transform3d_with_scale() {
        let transform = Transform3D::with_scale(2.0, 3.0, 4.0);
        assert_eq!(transform.scale, [2.0, 3.0, 4.0]);
        assert!(!transform.is_identity());
    }

    #[test]
    fn test_transform3d_combine() {
        let transform1 = Transform3D::with_position(1.0, 2.0, 3.0);
        let transform2 = Transform3D::with_scale(2.0, 2.0, 2.0);
        let combined = transform1.combine(&transform2);
        
        assert_eq!(combined.position, [1.0, 2.0, 3.0]);
        assert_eq!(combined.scale, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_matrix_utils_perspective() {
        let matrix = matrix_utils::perspective_matrix(45.0, 1.0, 0.1, 100.0);
        assert_eq!(matrix.len(), 16);
        // Basic sanity check - perspective matrix should have specific structure
        assert!(matrix[15] == 0.0); // Last element should be 0 for perspective
    }

    #[test]
    fn test_matrix_utils_orthographic() {
        let matrix = matrix_utils::orthographic_matrix(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
        assert_eq!(matrix.len(), 16);
        // Basic sanity check - orthographic matrix should have specific structure
        assert!(matrix[15] == 1.0); // Last element should be 1 for orthographic
    }
}
