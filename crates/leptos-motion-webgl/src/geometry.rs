//! Geometry management system

use crate::error::{Result, WebGLError};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};
use std::collections::HashMap;
use std::rc::Rc;

/// Vertex data structure
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Position
    pub position: [f32; 3],
    /// Normal
    pub normal: [f32; 3],
    /// Texture coordinates
    pub tex_coord: [f32; 2],
    /// Tangent
    pub tangent: [f32; 3],
    /// Bitangent
    pub bitangent: [f32; 3],
    /// Color
    pub color: [f32; 4],
}

impl Vertex {
    /// Create a new vertex
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coord: [0.0, 0.0],
            tangent: [1.0, 0.0, 0.0],
            bitangent: [0.0, 1.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    /// Set position
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    /// Get position
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set normal
    pub fn set_normal(&mut self, normal: [f32; 3]) {
        self.normal = normal;
    }

    /// Get normal
    pub fn get_normal(&self) -> [f32; 3] {
        self.normal
    }

    /// Set texture coordinates
    pub fn set_tex_coord(&mut self, tex_coord: [f32; 2]) {
        self.tex_coord = tex_coord;
    }

    /// Get texture coordinates
    pub fn get_tex_coord(&self) -> [f32; 2] {
        self.tex_coord
    }

    /// Set tangent
    pub fn set_tangent(&mut self, tangent: [f32; 3]) {
        self.tangent = tangent;
    }

    /// Get tangent
    pub fn get_tangent(&self) -> [f32; 3] {
        self.tangent
    }

    /// Set bitangent
    pub fn set_bitangent(&mut self, bitangent: [f32; 3]) {
        self.bitangent = bitangent;
    }

    /// Get bitangent
    pub fn get_bitangent(&self) -> [f32; 3] {
        self.bitangent
    }

    /// Set color
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    /// Get color
    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

/// Vertex attribute
#[derive(Debug, Clone)]
pub struct VertexAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute data
    pub data: Vec<f32>,
    /// Number of components per vertex
    pub size: i32,
    /// Data type
    pub data_type: u32,
    /// Normalized flag
    pub normalized: bool,
    /// Stride
    pub stride: i32,
    /// Offset
    pub offset: i32,
}

impl VertexAttribute {
    /// Create a new vertex attribute
    pub fn new(name: &str, data: Vec<f32>, size: i32) -> Self {
        Self {
            name: name.to_string(),
            data,
            size,
            data_type: WebGl2RenderingContext::FLOAT,
            normalized: false,
            stride: 0,
            offset: 0,
        }
    }
}

/// Geometry data
#[derive(Debug, Clone)]
pub struct Geometry {
    /// Geometry ID
    pub id: String,
    /// Geometry name
    pub name: String,
    /// Vertices (for model loading compatibility)
    pub vertices: Vec<Vertex>,
    /// Vertex attributes
    pub attributes: HashMap<String, VertexAttribute>,
    /// Index data
    pub indices: Option<Vec<u32>>,
    /// Bounding box
    pub bounding_box: BoundingBox,
    /// Bounding sphere
    pub bounding_sphere: BoundingSphere,
    /// Vertex count
    pub vertex_count: u32,
    /// Index count
    pub index_count: u32,
    /// WebGL buffers
    pub buffers: HashMap<String, WebGlBuffer>,
    /// Vertex array object
    pub vao: Option<WebGlVertexArrayObject>,
    /// Dirty flag
    pub needs_update: bool,
}

/// Bounding box
#[derive(Debug, Clone)]
pub struct BoundingBox {
    /// Minimum point
    pub min: [f32; 3],
    /// Maximum point
    pub max: [f32; 3],
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new() -> Self {
        Self {
            min: [f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
        }
    }

    /// Expand the bounding box to include a point
    pub fn expand_by_point(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);
        
        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
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

/// Bounding sphere
#[derive(Debug, Clone)]
pub struct BoundingSphere {
    /// Center point
    pub center: [f32; 3],
    /// Radius
    pub radius: f32,
}

impl BoundingSphere {
    /// Create a new bounding sphere
    pub fn new() -> Self {
        Self {
            center: [0.0, 0.0, 0.0],
            radius: 0.0,
        }
    }

    /// Set the bounding sphere from a bounding box
    pub fn set_from_bounding_box(&mut self, bounding_box: &BoundingBox) {
        self.center = bounding_box.get_center();
        let size = bounding_box.get_size();
        self.radius = (size[0] * size[0] + size[1] * size[1] + size[2] * size[2]).sqrt() * 0.5;
    }
}

impl Geometry {
    /// Create a new geometry
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            vertices: Vec::new(),
            attributes: HashMap::new(),
            indices: None,
            bounding_box: BoundingBox::new(),
            bounding_sphere: BoundingSphere::new(),
            vertex_count: 0,
            index_count: 0,
            buffers: HashMap::new(),
            vao: None,
            needs_update: true,
        }
    }

    /// Add a vertex attribute
    pub fn add_attribute(&mut self, name: &str, attribute: VertexAttribute) {
        self.attributes.insert(name.to_string(), attribute);
        self.needs_update = true;
    }

    /// Set indices
    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.index_count = indices.len() as u32;
        self.indices = Some(indices);
        self.needs_update = true;
    }

    /// Get indices length
    pub fn get_indices_len(&self) -> usize {
        self.indices.as_ref().map_or(0, |indices| indices.len())
    }

    /// Compute bounding box and sphere
    pub fn compute_bounding_volumes(&mut self) {
        if let Some(position_attr) = self.attributes.get("position") {
            let mut bounding_box = BoundingBox::new();
            
            for i in (0..position_attr.data.len()).step_by(position_attr.size as usize) {
                if i + 2 < position_attr.data.len() {
                    let point = [
                        position_attr.data[i],
                        position_attr.data[i + 1],
                        position_attr.data[i + 2],
                    ];
                    bounding_box.expand_by_point(point);
                }
            }
            
            self.bounding_box = bounding_box;
            self.bounding_sphere.set_from_bounding_box(&self.bounding_box);
        }
    }

    /// Update vertex count
    pub fn update_vertex_count(&mut self) {
        if let Some(position_attr) = self.attributes.get("position") {
            self.vertex_count = (position_attr.data.len() / position_attr.size as usize) as u32;
        }
    }

    /// Update index count
    pub fn update_index_count(&mut self) {
        if let Some(indices) = &self.indices {
            self.index_count = indices.len() as u32;
        }
    }

    /// Mark geometry as needing update
    pub fn mark_needs_update(&mut self) {
        self.needs_update = true;
    }

    /// Create a box geometry
    pub fn create_box(width: f32, height: f32, depth: f32) -> Self {
        let mut geometry = Self::new("BoxGeometry");
        
        // Box vertices
        let w = width * 0.5;
        let h = height * 0.5;
        let d = depth * 0.5;
        
        let positions = vec![
            // Front face
            -w, -h,  d,  w, -h,  d,  w,  h,  d, -w,  h,  d,
            // Back face
            -w, -h, -d, -w,  h, -d,  w,  h, -d,  w, -h, -d,
            // Top face
            -w,  h, -d, -w,  h,  d,  w,  h,  d,  w,  h, -d,
            // Bottom face
            -w, -h, -d,  w, -h, -d,  w, -h,  d, -w, -h,  d,
            // Right face
             w, -h, -d,  w,  h, -d,  w,  h,  d,  w, -h,  d,
            // Left face
            -w, -h, -d, -w, -h,  d, -w,  h,  d, -w,  h, -d,
        ];
        
        let normals = vec![
            // Front face
             0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,
            // Back face
             0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,
            // Top face
             0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,
            // Bottom face
             0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,
            // Right face
             1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,
            // Left face
            -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0,
        ];
        
        let uvs = vec![
            // Front face
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
            // Back face
            1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            // Top face
            0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            // Bottom face
            1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            // Right face
            1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            // Left face
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        ];
        
        let indices = vec![
            0,  1,  2,    0,  2,  3,    // front
            4,  5,  6,    4,  6,  7,    // back
            8,  9, 10,    8, 10, 11,    // top
           12, 13, 14,   12, 14, 15,    // bottom
           16, 17, 18,   16, 18, 19,    // right
           20, 21, 22,   20, 22, 23,    // left
        ];
        
        geometry.add_attribute("position", VertexAttribute::new("position", positions, 3));
        geometry.add_attribute("normal", VertexAttribute::new("normal", normals, 3));
        geometry.add_attribute("uv", VertexAttribute::new("uv", uvs, 2));
        geometry.set_indices(indices);
        
        geometry.compute_bounding_volumes();
        geometry.update_vertex_count();
        geometry.update_index_count();
        
        geometry
    }

    /// Create a sphere geometry
    pub fn create_sphere(radius: f32, width_segments: u32, height_segments: u32) -> Self {
        let mut geometry = Self::new("SphereGeometry");
        
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        
        let phi_start = 0.0;
        let phi_length = 2.0 * std::f32::consts::PI;
        let theta_start = 0.0;
        let theta_length = std::f32::consts::PI;
        
        for y in 0..=height_segments {
            let v = y as f32 / height_segments as f32;
            
            for x in 0..=width_segments {
                let u = x as f32 / width_segments as f32;
                
                let phi = phi_start + u * phi_length;
                let theta = theta_start + v * theta_length;
                
                let x_pos = radius * theta.sin() * phi.cos();
                let y_pos = radius * theta.cos();
                let z_pos = radius * theta.sin() * phi.sin();
                
                positions.extend_from_slice(&[x_pos, y_pos, z_pos]);
                normals.extend_from_slice(&[x_pos / radius, y_pos / radius, z_pos / radius]);
                uvs.extend_from_slice(&[u, v]);
            }
        }
        
        for y in 0..height_segments {
            for x in 0..width_segments {
                let a = (y * (width_segments + 1) + x) as u32;
                let b = (y * (width_segments + 1) + x + 1) as u32;
                let c = ((y + 1) * (width_segments + 1) + x) as u32;
                let d = ((y + 1) * (width_segments + 1) + x + 1) as u32;
                
                indices.extend_from_slice(&[a, b, c]);
                indices.extend_from_slice(&[b, d, c]);
            }
        }
        
        geometry.add_attribute("position", VertexAttribute::new("position", positions, 3));
        geometry.add_attribute("normal", VertexAttribute::new("normal", normals, 3));
        geometry.add_attribute("uv", VertexAttribute::new("uv", uvs, 2));
        geometry.set_indices(indices);
        
        geometry.compute_bounding_volumes();
        geometry.update_vertex_count();
        geometry.update_index_count();
        
        geometry
    }

    /// Create a plane geometry
    pub fn create_plane(width: f32, height: f32, width_segments: u32, height_segments: u32) -> Self {
        let mut geometry = Self::new("PlaneGeometry");
        
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        
        let _half_width = width * 0.5;
        let _half_height = height * 0.5;
        
        for y in 0..=height_segments {
            let v = y as f32 / height_segments as f32;
            
            for x in 0..=width_segments {
                let u = x as f32 / width_segments as f32;
                
                let x_pos = (u - 0.5) * width;
                let y_pos = 0.0;
                let z_pos = (v - 0.5) * height;
                
                positions.extend_from_slice(&[x_pos, y_pos, z_pos]);
                normals.extend_from_slice(&[0.0, 1.0, 0.0]);
                uvs.extend_from_slice(&[u, v]);
            }
        }
        
        for y in 0..height_segments {
            for x in 0..width_segments {
                let a = (y * (width_segments + 1) + x) as u32;
                let b = (y * (width_segments + 1) + x + 1) as u32;
                let c = ((y + 1) * (width_segments + 1) + x) as u32;
                let d = ((y + 1) * (width_segments + 1) + x + 1) as u32;
                
                indices.extend_from_slice(&[a, b, c]);
                indices.extend_from_slice(&[b, d, c]);
            }
        }
        
        geometry.add_attribute("position", VertexAttribute::new("position", positions, 3));
        geometry.add_attribute("normal", VertexAttribute::new("normal", normals, 3));
        geometry.add_attribute("uv", VertexAttribute::new("uv", uvs, 2));
        geometry.set_indices(indices);
        
        geometry.compute_bounding_volumes();
        geometry.update_vertex_count();
        geometry.update_index_count();
        
        geometry
    }
}
