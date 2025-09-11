//! Camera system

use crate::error::{Result, WebGLError};
use gl_matrix::mat4;
use std::f32::consts::PI;

/// Camera trait for different camera types
pub trait Camera {
    /// Update camera matrices
    fn update_matrix(&mut self);
    
    /// Get the view matrix
    fn get_view_matrix(&self) -> [f32; 16];
    
    /// Get the projection matrix
    fn get_projection_matrix(&self) -> [f32; 16];
    
    /// Get the camera position
    fn get_position(&self) -> [f32; 3];
    
    /// Get the camera target
    fn get_target(&self) -> [f32; 3];
    
    /// Get the camera up vector
    fn get_up(&self) -> [f32; 3];
}

/// Base camera implementation
#[derive(Debug, Clone)]
pub struct CameraBase {
    /// Camera position
    pub position: [f32; 3],
    /// Camera target (look-at point)
    pub target: [f32; 3],
    /// Camera up vector
    pub up: [f32; 3],
    /// View matrix
    pub matrix: [f32; 16],
    /// Matrix world
    pub matrix_world: [f32; 16],
    /// Camera ID
    pub id: String,
    /// Camera name
    pub name: String,
    /// Visible flag
    pub visible: bool,
}

impl CameraBase {
    /// Create a new camera base
    pub fn new(name: &str) -> Self {
        Self {
            position: [0.0, 0.0, 5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            matrix: {
                let mut m = [0.0; 16];
                mat4::identity(&mut m);
                m
            },
            matrix_world: {
                let mut m = [0.0; 16];
                mat4::identity(&mut m);
                m
            },
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            visible: true,
        }
    }

    /// Update the view matrix
    pub fn update_view_matrix(&mut self) {
        mat4::look_at(&mut self.matrix, &self.position, &self.target, &self.up);
    }

    /// Set position
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    /// Set target
    pub fn set_target(&mut self, x: f32, y: f32, z: f32) {
        self.target = [x, y, z];
    }

    /// Set up vector
    pub fn set_up(&mut self, x: f32, y: f32, z: f32) {
        self.up = [x, y, z];
    }

    /// Look at a target position
    pub fn look_at(&mut self, target: [f32; 3]) {
        self.target = target;
    }
}

/// Perspective camera
#[derive(Debug, Clone)]
pub struct PerspectiveCamera {
    /// Base camera
    pub base: CameraBase,
    /// Field of view in degrees
    pub fov: f32,
    /// Aspect ratio
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Projection matrix
    pub projection_matrix: [f32; 16],
}

impl PerspectiveCamera {
    /// Create a new perspective camera
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut camera = Self {
            base: CameraBase::new("PerspectiveCamera"),
            fov,
            aspect,
            near,
            far,
            projection_matrix: {
                let mut m = [0.0; 16];
                mat4::identity(&mut m);
                m
            },
        };
        
        camera.update_matrix();
        camera
    }

    /// Set field of view
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update_projection_matrix();
    }

    /// Set aspect ratio
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update_projection_matrix();
    }

    /// Set near and far planes
    pub fn set_near_far(&mut self, near: f32, far: f32) {
        self.near = near;
        self.far = far;
        self.update_projection_matrix();
    }

    /// Update projection matrix
    fn update_projection_matrix(&mut self) {
        mat4::perspective(
            &mut self.projection_matrix,
            self.fov * PI / 180.0, // Convert to radians
            self.aspect,
            self.near,
            Some(self.far),
        );
    }
}

impl Camera for PerspectiveCamera {
    fn update_matrix(&mut self) {
        self.base.update_view_matrix();
        self.update_projection_matrix();
    }

    fn get_view_matrix(&self) -> [f32; 16] {
        self.base.matrix
    }

    fn get_projection_matrix(&self) -> [f32; 16] {
        self.projection_matrix
    }

    fn get_position(&self) -> [f32; 3] {
        self.base.position
    }

    fn get_target(&self) -> [f32; 3] {
        self.base.target
    }

    fn get_up(&self) -> [f32; 3] {
        self.base.up
    }
}

/// Orthographic camera
#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    /// Base camera
    pub base: CameraBase,
    /// Left clipping plane
    pub left: f32,
    /// Right clipping plane
    pub right: f32,
    /// Top clipping plane
    pub top: f32,
    /// Bottom clipping plane
    pub bottom: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Projection matrix
    pub projection_matrix: [f32; 16],
}

impl OrthographicCamera {
    /// Create a new orthographic camera
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let mut camera = Self {
            base: CameraBase::new("OrthographicCamera"),
            left,
            right,
            top,
            bottom,
            near,
            far,
            projection_matrix: {
                let mut m = [0.0; 16];
                mat4::identity(&mut m);
                m
            },
        };
        
        camera.update_matrix();
        camera
    }

    /// Set the orthographic frustum
    pub fn set_frustum(&mut self, left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        self.near = near;
        self.far = far;
        self.update_projection_matrix();
    }

    /// Update projection matrix
    fn update_projection_matrix(&mut self) {
        mat4::ortho(
            &mut self.projection_matrix,
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        );
    }
}

impl Camera for OrthographicCamera {
    fn update_matrix(&mut self) {
        self.base.update_view_matrix();
        self.update_projection_matrix();
    }

    fn get_view_matrix(&self) -> [f32; 16] {
        self.base.matrix
    }

    fn get_projection_matrix(&self) -> [f32; 16] {
        self.projection_matrix
    }

    fn get_position(&self) -> [f32; 3] {
        self.base.position
    }

    fn get_target(&self) -> [f32; 3] {
        self.base.target
    }

    fn get_up(&self) -> [f32; 3] {
        self.base.up
    }
}

/// Camera controls for interactive cameras
#[derive(Debug, Clone)]
pub struct CameraControls {
    /// Enable rotation
    pub enable_rotate: bool,
    /// Enable zoom
    pub enable_zoom: bool,
    /// Enable pan
    pub enable_pan: bool,
    /// Auto rotate
    pub auto_rotate: bool,
    /// Auto rotate speed
    pub auto_rotate_speed: f32,
    /// Damping factor
    pub damping_factor: f32,
    /// Zoom speed
    pub zoom_speed: f32,
    /// Rotate speed
    pub rotate_speed: f32,
    /// Pan speed
    pub pan_speed: f32,
    /// Min distance
    pub min_distance: f32,
    /// Max distance
    pub max_distance: f32,
    /// Min polar angle
    pub min_polar_angle: f32,
    /// Max polar angle
    pub max_polar_angle: f32,
    /// Min azimuth angle
    pub min_azimuth_angle: f32,
    /// Max azimuth angle
    pub max_azimuth_angle: f32,
}

impl Default for CameraControls {
    fn default() -> Self {
        Self {
            enable_rotate: true,
            enable_zoom: true,
            enable_pan: true,
            auto_rotate: false,
            auto_rotate_speed: 2.0,
            damping_factor: 0.05,
            zoom_speed: 1.0,
            rotate_speed: 1.0,
            pan_speed: 1.0,
            min_distance: 0.0,
            max_distance: f32::INFINITY,
            min_polar_angle: 0.0,
            max_polar_angle: PI,
            min_azimuth_angle: f32::NEG_INFINITY,
            max_azimuth_angle: f32::INFINITY,
        }
    }
}
