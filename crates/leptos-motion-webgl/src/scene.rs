//! Scene graph system

use crate::error::{Result, WebGLError};
use gl_matrix::mat4;
use std::collections::HashMap;
use std::sync::Arc;

/// 3D object in the scene
#[derive(Debug, Clone)]
pub struct Object3D {
    /// Object ID
    pub id: String,
    /// Object name
    pub name: String,
    /// Position
    pub position: [f32; 3],
    /// Rotation (Euler angles in radians)
    pub rotation: [f32; 3],
    /// Scale
    pub scale: [f32; 3],
    /// Model matrix
    pub matrix: [f32; 16],
    /// World matrix
    pub matrix_world: [f32; 16],
    /// Parent object
    pub parent: Option<Arc<Object3D>>,
    /// Children objects
    pub children: Vec<Arc<Object3D>>,
    /// Visible flag
    pub visible: bool,
    /// User data
    pub user_data: HashMap<String, String>,
}

impl Object3D {
    /// Create a new 3D object
    pub fn new(name: &str) -> Self {
        let mut object = Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
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
            parent: None,
            children: Vec::new(),
            visible: true,
            user_data: HashMap::new(),
        };
        
        object.update_matrix();
        object
    }

    /// Update the model matrix
    pub fn update_matrix(&mut self) {
        let mut matrix = [0.0; 16];
        mat4::identity(&mut matrix);
        
        // Apply translation
        let mut temp = matrix;
        mat4::translate(&mut matrix, &temp, &self.position);
        
        // Apply rotation (ZYX order)
        temp = matrix;
        mat4::rotate_z(&mut matrix, &temp, self.rotation[2]);
        temp = matrix;
        mat4::rotate_y(&mut matrix, &temp, self.rotation[1]);
        temp = matrix;
        mat4::rotate_x(&mut matrix, &temp, self.rotation[0]);
        
        // Apply scale
        temp = matrix;
        mat4::scale(&mut matrix, &temp, &self.scale);
        
        self.matrix = matrix;
    }

    /// Update the world matrix
    pub fn update_matrix_world(&mut self, _force: bool) {
        if let Some(_parent) = &self.parent {
            // TODO: Update parent matrix world first
            // For now, just use the model matrix
            self.matrix_world = self.matrix;
        } else {
            self.matrix_world = self.matrix;
        }
    }

    /// Add a child object
    pub fn add(&mut self, child: Arc<Object3D>) {
        self.children.push(child);
    }

    /// Remove a child object
    pub fn remove(&mut self, child_id: &str) -> Option<Arc<Object3D>> {
        if let Some(pos) = self.children.iter().position(|c| c.id == child_id) {
            Some(self.children.remove(pos))
        } else {
            None
        }
    }

    /// Get a child by name
    pub fn get_child_by_name(&self, name: &str) -> Option<&Arc<Object3D>> {
        self.children.iter().find(|child| child.name == name)
    }

    /// Get a child by ID
    pub fn get_child_by_id(&self, id: &str) -> Option<&Arc<Object3D>> {
        self.children.iter().find(|child| child.id == id)
    }

    /// Traverse the object hierarchy
    pub fn traverse<F>(&self, callback: &mut F)
    where
        F: FnMut(&Object3D),
    {
        callback(self);
        for child in &self.children {
            child.traverse(callback);
        }
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

    /// Look at a target position
    pub fn look_at(&mut self, _target: [f32; 3]) {
        // TODO: Implement look at functionality
        // This requires more complex matrix calculations
    }
}

/// Scene containing 3D objects
#[derive(Debug)]
pub struct Scene {
    /// Scene ID
    pub id: String,
    /// Scene name
    pub name: String,
    /// Root object
    pub root: Object3D,
    /// Background color
    pub background: Option<[f32; 4]>,
    /// Fog settings
    pub fog: Option<Fog>,
    /// Environment settings
    pub environment: Option<Environment>,
    /// User data
    pub user_data: HashMap<String, String>,
}

/// Fog settings
#[derive(Debug, Clone)]
pub struct Fog {
    /// Fog color
    pub color: [f32; 3],
    /// Fog near distance
    pub near: f32,
    /// Fog far distance
    pub far: f32,
}

/// Environment settings
#[derive(Debug, Clone)]
pub struct Environment {
    /// Environment map
    pub map: Option<String>, // Texture ID
    /// Environment intensity
    pub intensity: f32,
}

impl Scene {
    /// Create a new scene
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Scene".to_string(),
            root: Object3D::new("root"),
            background: None,
            fog: None,
            environment: None,
            user_data: HashMap::new(),
        }
    }

    /// Create a new scene with a name
    pub fn new_with_name(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            root: Object3D::new("root"),
            background: None,
            fog: None,
            environment: None,
            user_data: HashMap::new(),
        }
    }

    /// Add an object to the scene
    pub fn add(&mut self, object: Arc<Object3D>) {
        self.root.add(object);
    }

    /// Remove an object from the scene
    pub fn remove(&mut self, object_id: &str) -> Option<Arc<Object3D>> {
        self.root.remove(object_id)
    }

    /// Get an object by name
    pub fn get_object_by_name(&self, name: &str) -> Option<&Arc<Object3D>> {
        self.root.get_child_by_name(name)
    }

    /// Get an object by ID
    pub fn get_object_by_id(&self, id: &str) -> Option<&Arc<Object3D>> {
        self.root.get_child_by_id(id)
    }

    /// Traverse all objects in the scene
    pub fn traverse<F>(&self, callback: &mut F)
    where
        F: FnMut(&Object3D),
    {
        self.root.traverse(callback);
    }

    /// Update all matrices in the scene
    pub fn update_matrix_world(&mut self, force: bool) {
        self.root.update_matrix_world(force);
        for _child in &self.root.children {
            // TODO: Update child matrices recursively
        }
    }

    /// Set background color
    pub fn set_background(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.background = Some([r, g, b, a]);
    }

    /// Set fog
    pub fn set_fog(&mut self, color: [f32; 3], near: f32, far: f32) {
        self.fog = Some(Fog { color, near, far });
    }

    /// Set environment
    pub fn set_environment(&mut self, map: Option<String>, intensity: f32) {
        self.environment = Some(Environment { map, intensity });
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
