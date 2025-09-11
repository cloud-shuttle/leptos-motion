//! Model loading system for WebGL rendering

use crate::error::{Result, WebGLError};
use crate::geometry::{Geometry, Vertex, VertexAttribute};
use crate::material::Material;
use std::collections::HashMap;
use std::str::FromStr;

/// Model format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFormat {
    OBJ,
    GLTF,
    GLB,
    FBX,
    DAE,
}

impl FromStr for ModelFormat {
    type Err = WebGLError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "obj" => Ok(ModelFormat::OBJ),
            "gltf" => Ok(ModelFormat::GLTF),
            "glb" => Ok(ModelFormat::GLB),
            "fbx" => Ok(ModelFormat::FBX),
            "dae" => Ok(ModelFormat::DAE),
            _ => Err(WebGLError::model_error(&format!(
                "Unsupported model format: {}",
                s
            ))),
        }
    }
}

/// Model loading options
#[derive(Debug, Clone)]
pub struct ModelLoadOptions {
    /// Generate normals if missing
    pub generate_normals: bool,
    /// Generate tangents and bitangents
    pub generate_tangents: bool,
    /// Flip Y coordinates
    pub flip_y: bool,
    /// Scale factor
    pub scale: f32,
    /// Center the model
    pub center: bool,
    /// Maximum number of vertices per mesh
    pub max_vertices: Option<usize>,
    /// Maximum number of indices per mesh
    pub max_indices: Option<usize>,
}

impl Default for ModelLoadOptions {
    fn default() -> Self {
        Self {
            generate_normals: true,
            generate_tangents: false,
            flip_y: false,
            scale: 1.0,
            center: false,
            max_vertices: None,
            max_indices: None,
        }
    }
}

/// Mesh data structure
#[derive(Debug, Clone)]
pub struct Mesh {
    /// Mesh name
    pub name: String,
    /// Mesh geometry
    pub geometry: Geometry,
    /// Mesh material
    pub material: Option<Material>,
    /// Bounding box
    pub bounding_box: BoundingBox,
}

/// Bounding box structure
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    /// Minimum corner
    pub min: [f32; 3],
    /// Maximum corner
    pub max: [f32; 3],
    /// Center point
    pub center: [f32; 3],
    /// Size
    pub size: [f32; 3],
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        let center = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];
        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];

        Self {
            min,
            max,
            center,
            size,
        }
    }

    /// Create an empty bounding box
    pub fn empty() -> Self {
        Self {
            min: [f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
            center: [0.0, 0.0, 0.0],
            size: [0.0, 0.0, 0.0],
        }
    }

    /// Expand bounding box to include a point
    pub fn expand(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);

        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);

        // Update center and size
        self.center = [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ];
        self.size = [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ];
    }

    /// Expand bounding box to include another bounding box
    pub fn expand_bbox(&mut self, other: &BoundingBox) {
        self.expand(other.min);
        self.expand(other.max);
    }

    /// Check if bounding box contains a point
    pub fn contains(&self, point: [f32; 3]) -> bool {
        point[0] >= self.min[0]
            && point[0] <= self.max[0]
            && point[1] >= self.min[1]
            && point[1] <= self.max[1]
            && point[2] >= self.min[2]
            && point[2] <= self.max[2]
    }

    /// Check if bounding box intersects with another bounding box
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min[0] <= other.max[0]
            && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1]
            && self.max[1] >= other.min[1]
            && self.min[2] <= other.max[2]
            && self.max[2] >= other.min[2]
    }

    /// Get bounding box volume
    pub fn volume(&self) -> f32 {
        self.size[0] * self.size[1] * self.size[2]
    }

    /// Get bounding box diagonal length
    pub fn diagonal_length(&self) -> f32 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Model data structure
#[derive(Debug, Clone)]
pub struct Model {
    /// Model name
    pub name: String,
    /// Model meshes
    pub meshes: Vec<Mesh>,
    /// Model materials
    pub materials: HashMap<String, Material>,
    /// Model bounding box
    pub bounding_box: BoundingBox,
    /// Model format
    pub format: ModelFormat,
    /// Load options used
    pub load_options: ModelLoadOptions,
}

impl Model {
    /// Create a new model
    pub fn new(name: String, format: ModelFormat, load_options: ModelLoadOptions) -> Self {
        Self {
            name,
            meshes: Vec::new(),
            materials: HashMap::new(),
            bounding_box: BoundingBox::empty(),
            format,
            load_options,
        }
    }

    /// Add a mesh to the model
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.bounding_box.expand_bbox(&mesh.bounding_box);
        self.meshes.push(mesh);
    }

    /// Add a material to the model
    pub fn add_material(&mut self, name: String, material: Material) {
        self.materials.insert(name, material);
    }

    /// Get mesh by name
    pub fn get_mesh(&self, name: &str) -> Option<&Mesh> {
        self.meshes.iter().find(|mesh| mesh.name == name)
    }

    /// Get mesh by index
    pub fn get_mesh_by_index(&self, index: usize) -> Option<&Mesh> {
        self.meshes.get(index)
    }

    /// Get material by name
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    /// Get mesh count
    pub fn get_mesh_count(&self) -> usize {
        self.meshes.len()
    }

    /// Get material count
    pub fn get_material_count(&self) -> usize {
        self.materials.len()
    }

    /// Get total vertex count
    pub fn get_total_vertex_count(&self) -> usize {
        self.meshes
            .iter()
            .map(|mesh| mesh.geometry.vertices.len())
            .sum()
    }

    /// Get total index count
    pub fn get_total_index_count(&self) -> usize {
        self.meshes
            .iter()
            .map(|mesh| mesh.geometry.get_indices_len())
            .sum()
    }

    /// Check if model is empty
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }

    /// Get model statistics
    pub fn get_stats(&self) -> ModelStats {
        ModelStats {
            mesh_count: self.get_mesh_count(),
            material_count: self.get_material_count(),
            total_vertices: self.get_total_vertex_count(),
            total_indices: self.get_total_index_count(),
            bounding_box_volume: self.bounding_box.volume(),
            bounding_box_diagonal: self.bounding_box.diagonal_length(),
        }
    }
}

/// Model statistics
#[derive(Debug, Clone)]
pub struct ModelStats {
    /// Number of meshes
    pub mesh_count: usize,
    /// Number of materials
    pub material_count: usize,
    /// Total vertex count
    pub total_vertices: usize,
    /// Total index count
    pub total_indices: usize,
    /// Bounding box volume
    pub bounding_box_volume: f32,
    /// Bounding box diagonal length
    pub bounding_box_diagonal: f32,
}

/// OBJ model loader
pub struct ObjLoader;

impl ObjLoader {
    /// Load model from OBJ data
    pub fn load(data: &str, options: ModelLoadOptions) -> Result<Model> {
        let mut model = Model::new("obj_model".to_string(), ModelFormat::OBJ, options.clone());

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut faces = Vec::new();
        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut current_material = None;

        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    // Vertex position
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        vertices.push([x, y, z]);
                    }
                }
                "vn" => {
                    // Vertex normal
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        normals.push([x, y, z]);
                    }
                }
                "vt" => {
                    // Texture coordinate
                    if parts.len() >= 3 {
                        let u = parts[1].parse::<f32>().unwrap_or(0.0);
                        let v = parts[2].parse::<f32>().unwrap_or(0.0);
                        tex_coords.push([u, v]);
                    }
                }
                "f" => {
                    // Face
                    if parts.len() >= 4 {
                        let mut face_vertices = Vec::new();
                        for i in 1..parts.len() {
                            let vertex_data = parts[i];
                            let indices: Vec<&str> = vertex_data.split('/').collect();

                            let vertex_idx = indices[0].parse::<usize>().unwrap_or(1) - 1;
                            let tex_idx = if indices.len() > 1 && !indices[1].is_empty() {
                                indices[1].parse::<usize>().unwrap_or(1) - 1
                            } else {
                                0
                            };
                            let normal_idx = if indices.len() > 2 && !indices[2].is_empty() {
                                indices[2].parse::<usize>().unwrap_or(1) - 1
                            } else {
                                0
                            };

                            face_vertices.push((vertex_idx, tex_idx, normal_idx));
                        }
                        faces.push((face_vertices, current_material.clone()));
                    }
                }
                "usemtl" => {
                    // Use material
                    if parts.len() >= 2 {
                        current_material = Some(parts[1].to_string());
                    }
                }
                "mtllib" => {
                    // Material library (not implemented yet)
                    // TODO: Load material library
                }
                _ => {
                    // Ignore other commands
                }
            }
        }

        // Process faces and create geometry
        if !faces.is_empty() {
            let mut mesh_vertices = Vec::new();
            let mut mesh_indices = Vec::new();
            let mut vertex_map = HashMap::new();

            for (face_vertices, material_name) in faces {
                if face_vertices.len() < 3 {
                    continue; // Skip invalid faces
                }

                // Triangulate face (simple fan triangulation)
                for i in 1..face_vertices.len() - 1 {
                    let indices = [0, i, i + 1];
                    let mut triangle_vertices: Vec<usize> = Vec::new();

                    for &idx in &indices {
                        let (vertex_idx, tex_idx, normal_idx) = face_vertices[idx];

                        if vertex_idx >= vertices.len() {
                            continue;
                        }

                        let mut vertex = Vertex::new();
                        vertex.set_position(vertices[vertex_idx]);

                        if tex_idx < tex_coords.len() {
                            vertex.set_tex_coord(tex_coords[tex_idx]);
                        }

                        if normal_idx < normals.len() {
                            vertex.set_normal(normals[normal_idx]);
                        }

                        // Check if we've seen this vertex before
                        let vertex_key = (vertex_idx, tex_idx, normal_idx);
                        if let Some(&existing_index) = vertex_map.get(&vertex_key) {
                            mesh_indices.push(existing_index as u32);
                        } else {
                            let new_index = mesh_vertices.len();
                            mesh_vertices.push(vertex);
                            mesh_indices.push(new_index as u32);
                            vertex_map.insert(vertex_key, new_index);
                        }
                    }
                }
            }

            // Generate normals if requested and missing
            if options.generate_normals {
                Self::generate_normals(&mut mesh_vertices, &mesh_indices);
            }

            // Apply transformations
            if options.scale != 1.0 {
                for vertex in &mut mesh_vertices {
                    let pos = vertex.get_position();
                    vertex.set_position([
                        pos[0] * options.scale,
                        pos[1] * options.scale,
                        pos[2] * options.scale,
                    ]);
                }
            }

            if options.flip_y {
                for vertex in &mut mesh_vertices {
                    let pos = vertex.get_position();
                    vertex.set_position([pos[0], -pos[1], pos[2]]);
                }
            }

            // Calculate bounding box
            let mut bounding_box = BoundingBox::empty();
            for vertex in &mesh_vertices {
                bounding_box.expand(vertex.get_position());
            }

            if options.center {
                let center = bounding_box.center;
                for vertex in &mut mesh_vertices {
                    let pos = vertex.get_position();
                    vertex.set_position([
                        pos[0] - center[0],
                        pos[1] - center[1],
                        pos[2] - center[2],
                    ]);
                }
                bounding_box = BoundingBox::empty();
                for vertex in &mesh_vertices {
                    bounding_box.expand(vertex.get_position());
                }
            }

            // Create geometry
            let mut geometry = Geometry::new("mesh");
            geometry.vertices = mesh_vertices;
            geometry.set_indices(mesh_indices);

            // Create mesh
            let mesh = Mesh {
                name: "mesh".to_string(),
                geometry,
                material: None, // TODO: Handle materials
                bounding_box,
            };

            model.add_mesh(mesh);
        }

        Ok(model)
    }

    /// Generate normals for vertices
    fn generate_normals(vertices: &mut Vec<Vertex>, indices: &[u32]) {
        // Initialize normals to zero
        for vertex in vertices.iter_mut() {
            vertex.set_normal([0.0, 0.0, 0.0]);
        }

        // Calculate face normals and accumulate
        for i in (0..indices.len()).step_by(3) {
            if i + 2 >= indices.len() {
                break;
            }

            let i0 = indices[i] as usize;
            let i1 = indices[i + 1] as usize;
            let i2 = indices[i + 2] as usize;

            if i0 >= vertices.len() || i1 >= vertices.len() || i2 >= vertices.len() {
                continue;
            }

            let p0 = vertices[i0].get_position();
            let p1 = vertices[i1].get_position();
            let p2 = vertices[i2].get_position();

            // Calculate face normal
            let v1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];

            let mut normal = [
                v1[1] * v2[2] - v1[2] * v2[1],
                v1[2] * v2[0] - v1[0] * v2[2],
                v1[0] * v2[1] - v1[1] * v2[0],
            ];

            // Normalize
            let length =
                (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            if length > 0.0 {
                normal[0] /= length;
                normal[1] /= length;
                normal[2] /= length;
            }

            // Accumulate normals
            let n0 = vertices[i0].get_normal();
            let n1 = vertices[i1].get_normal();
            let n2 = vertices[i2].get_normal();

            vertices[i0].set_normal([n0[0] + normal[0], n0[1] + normal[1], n0[2] + normal[2]]);
            vertices[i1].set_normal([n1[0] + normal[0], n1[1] + normal[1], n1[2] + normal[2]]);
            vertices[i2].set_normal([n2[0] + normal[0], n2[1] + normal[1], n2[2] + normal[2]]);
        }

        // Normalize accumulated normals
        for vertex in vertices.iter_mut() {
            let normal = vertex.get_normal();
            let length =
                (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            if length > 0.0 {
                vertex.set_normal([normal[0] / length, normal[1] / length, normal[2] / length]);
            }
        }
    }
}

/// Model loader manager
pub struct ModelLoader {
    /// Supported formats
    supported_formats: Vec<ModelFormat>,
}

impl ModelLoader {
    /// Create a new model loader
    pub fn new() -> Self {
        Self {
            supported_formats: vec![ModelFormat::OBJ], // Only OBJ for now
        }
    }

    /// Load model from data
    pub fn load_from_data(
        &self,
        data: &str,
        format: ModelFormat,
        options: ModelLoadOptions,
    ) -> Result<Model> {
        match format {
            ModelFormat::OBJ => ObjLoader::load(data, options),
            _ => Err(WebGLError::model_error(&format!(
                "Format {:?} not yet implemented",
                format
            ))),
        }
    }

    /// Load model from URL (async - not implemented yet)
    pub fn load_from_url(
        &self,
        _url: &str,
        _format: ModelFormat,
        _options: ModelLoadOptions,
    ) -> Result<Model> {
        Err(WebGLError::model_error(
            "Async model loading not yet implemented",
        ))
    }

    /// Get supported formats
    pub fn get_supported_formats(&self) -> &[ModelFormat] {
        &self.supported_formats
    }

    /// Check if format is supported
    pub fn is_format_supported(&self, format: ModelFormat) -> bool {
        self.supported_formats.contains(&format)
    }
}
