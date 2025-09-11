//! Material system

use crate::error::{Result, WebGLError};
use std::collections::HashMap;

/// Material type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Basic,
    Lambert,
    Phong,
    Standard,
    Physical,
}

/// Material side
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialSide {
    Front,
    Back,
    Double,
}

/// Material blending mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Additive,
    Subtractive,
    Multiply,
    Screen,
    Overlay,
}

/// Material base class
#[derive(Debug, Clone)]
pub struct Material {
    /// Material ID
    pub id: String,
    /// Material name
    pub name: String,
    /// Material type
    pub material_type: MaterialType,
    /// Visible flag
    pub visible: bool,
    /// Transparent flag
    pub transparent: bool,
    /// Opacity
    pub opacity: f32,
    /// Alpha test threshold
    pub alpha_test: f32,
    /// Side to render
    pub side: MaterialSide,
    /// Blending mode
    pub blend_mode: BlendMode,
    /// Depth test
    pub depth_test: bool,
    /// Depth write
    pub depth_write: bool,
    /// Wireframe mode
    pub wireframe: bool,
    /// Wireframe line width
    pub wireframe_line_width: f32,
    /// User data
    pub user_data: HashMap<String, String>,
}

impl Material {
    /// Create a new material
    pub fn new(name: &str, material_type: MaterialType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            material_type,
            visible: true,
            transparent: false,
            opacity: 1.0,
            alpha_test: 0.0,
            side: MaterialSide::Front,
            blend_mode: BlendMode::Normal,
            depth_test: true,
            depth_write: true,
            wireframe: false,
            wireframe_line_width: 1.0,
            user_data: HashMap::new(),
        }
    }

    /// Set opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
        self.transparent = self.opacity < 1.0;
    }

    /// Set transparent flag
    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
    }

    /// Set side
    pub fn set_side(&mut self, side: MaterialSide) {
        self.side = side;
    }

    /// Set blend mode
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.blend_mode = blend_mode;
    }

    /// Set wireframe mode
    pub fn set_wireframe(&mut self, wireframe: bool) {
        self.wireframe = wireframe;
    }
}

/// Basic material (unlit)
#[derive(Debug, Clone)]
pub struct BasicMaterial {
    /// Base material
    pub base: Material,
    /// Diffuse color
    pub color: [f32; 3],
    /// Diffuse map
    pub map: Option<String>, // Texture ID
    /// Alpha map
    pub alpha_map: Option<String>, // Texture ID
    /// Environment map
    pub env_map: Option<String>, // Texture ID
    /// Environment map intensity
    pub env_map_intensity: f32,
    /// Fog
    pub fog: bool,
}

impl BasicMaterial {
    /// Create a new basic material
    pub fn new(name: &str) -> Self {
        Self {
            base: Material::new(name, MaterialType::Basic),
            color: [1.0, 1.0, 1.0],
            map: None,
            alpha_map: None,
            env_map: None,
            env_map_intensity: 1.0,
            fog: true,
        }
    }

    /// Set color
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set diffuse map
    pub fn set_map(&mut self, map: Option<String>) {
        self.map = map;
    }

    /// Set alpha map
    pub fn set_alpha_map(&mut self, alpha_map: Option<String>) {
        self.alpha_map = alpha_map;
    }

    /// Set environment map
    pub fn set_env_map(&mut self, env_map: Option<String>) {
        self.env_map = env_map;
    }
}

/// Lambert material (diffuse lighting)
#[derive(Debug, Clone)]
pub struct LambertMaterial {
    /// Base material
    pub base: Material,
    /// Diffuse color
    pub color: [f32; 3],
    /// Diffuse map
    pub map: Option<String>, // Texture ID
    /// Alpha map
    pub alpha_map: Option<String>, // Texture ID
    /// Environment map
    pub env_map: Option<String>, // Texture ID
    /// Environment map intensity
    pub env_map_intensity: f32,
    /// Emissive color
    pub emissive: [f32; 3],
    /// Emissive map
    pub emissive_map: Option<String>, // Texture ID
    /// Emissive intensity
    pub emissive_intensity: f32,
    /// Fog
    pub fog: bool,
}

impl LambertMaterial {
    /// Create a new Lambert material
    pub fn new(name: &str) -> Self {
        Self {
            base: Material::new(name, MaterialType::Lambert),
            color: [1.0, 1.0, 1.0],
            map: None,
            alpha_map: None,
            env_map: None,
            env_map_intensity: 1.0,
            emissive: [0.0, 0.0, 0.0],
            emissive_map: None,
            emissive_intensity: 1.0,
            fog: true,
        }
    }

    /// Set color
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set emissive color
    pub fn set_emissive(&mut self, r: f32, g: f32, b: f32) {
        self.emissive = [r, g, b];
    }

    /// Set diffuse map
    pub fn set_map(&mut self, map: Option<String>) {
        self.map = map;
    }

    /// Set emissive map
    pub fn set_emissive_map(&mut self, emissive_map: Option<String>) {
        self.emissive_map = emissive_map;
    }
}

/// Phong material (specular lighting)
#[derive(Debug, Clone)]
pub struct PhongMaterial {
    /// Base material
    pub base: Material,
    /// Diffuse color
    pub color: [f32; 3],
    /// Diffuse map
    pub map: Option<String>, // Texture ID
    /// Alpha map
    pub alpha_map: Option<String>, // Texture ID
    /// Environment map
    pub env_map: Option<String>, // Texture ID
    /// Environment map intensity
    pub env_map_intensity: f32,
    /// Emissive color
    pub emissive: [f32; 3],
    /// Emissive map
    pub emissive_map: Option<String>, // Texture ID
    /// Emissive intensity
    pub emissive_intensity: f32,
    /// Specular color
    pub specular: [f32; 3],
    /// Specular map
    pub specular_map: Option<String>, // Texture ID
    /// Shininess
    pub shininess: f32,
    /// Normal map
    pub normal_map: Option<String>, // Texture ID
    /// Normal scale
    pub normal_scale: [f32; 2],
    /// Bump map
    pub bump_map: Option<String>, // Texture ID
    /// Bump scale
    pub bump_scale: f32,
    /// Displacement map
    pub displacement_map: Option<String>, // Texture ID
    /// Displacement scale
    pub displacement_scale: f32,
    /// Displacement bias
    pub displacement_bias: f32,
    /// Fog
    pub fog: bool,
}

impl PhongMaterial {
    /// Create a new Phong material
    pub fn new(name: &str) -> Self {
        Self {
            base: Material::new(name, MaterialType::Phong),
            color: [1.0, 1.0, 1.0],
            map: None,
            alpha_map: None,
            env_map: None,
            env_map_intensity: 1.0,
            emissive: [0.0, 0.0, 0.0],
            emissive_map: None,
            emissive_intensity: 1.0,
            specular: [0.111111, 0.111111, 0.111111],
            specular_map: None,
            shininess: 30.0,
            normal_map: None,
            normal_scale: [1.0, 1.0],
            bump_map: None,
            bump_scale: 1.0,
            displacement_map: None,
            displacement_scale: 1.0,
            displacement_bias: 0.0,
            fog: true,
        }
    }

    /// Set color
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set emissive color
    pub fn set_emissive(&mut self, r: f32, g: f32, b: f32) {
        self.emissive = [r, g, b];
    }

    /// Set specular color
    pub fn set_specular(&mut self, r: f32, g: f32, b: f32) {
        self.specular = [r, g, b];
    }

    /// Set shininess
    pub fn set_shininess(&mut self, shininess: f32) {
        self.shininess = shininess.max(0.0);
    }

    /// Set diffuse map
    pub fn set_map(&mut self, map: Option<String>) {
        self.map = map;
    }

    /// Set specular map
    pub fn set_specular_map(&mut self, specular_map: Option<String>) {
        self.specular_map = specular_map;
    }

    /// Set normal map
    pub fn set_normal_map(&mut self, normal_map: Option<String>) {
        self.normal_map = normal_map;
    }

    /// Set bump map
    pub fn set_bump_map(&mut self, bump_map: Option<String>) {
        self.bump_map = bump_map;
    }
}

/// Standard material (PBR)
#[derive(Debug, Clone)]
pub struct StandardMaterial {
    /// Base material
    pub base: Material,
    /// Diffuse color
    pub color: [f32; 3],
    /// Diffuse map
    pub map: Option<String>, // Texture ID
    /// Alpha map
    pub alpha_map: Option<String>, // Texture ID
    /// Environment map
    pub env_map: Option<String>, // Texture ID
    /// Environment map intensity
    pub env_map_intensity: f32,
    /// Emissive color
    pub emissive: [f32; 3],
    /// Emissive map
    pub emissive_map: Option<String>, // Texture ID
    /// Emissive intensity
    pub emissive_intensity: f32,
    /// Metalness
    pub metalness: f32,
    /// Metalness map
    pub metalness_map: Option<String>, // Texture ID
    /// Roughness
    pub roughness: f32,
    /// Roughness map
    pub roughness_map: Option<String>, // Texture ID
    /// Normal map
    pub normal_map: Option<String>, // Texture ID
    /// Normal scale
    pub normal_scale: [f32; 2],
    /// Bump map
    pub bump_map: Option<String>, // Texture ID
    /// Bump scale
    pub bump_scale: f32,
    /// Displacement map
    pub displacement_map: Option<String>, // Texture ID
    /// Displacement scale
    pub displacement_scale: f32,
    /// Displacement bias
    pub displacement_bias: f32,
    /// AO map
    pub ao_map: Option<String>, // Texture ID
    /// AO map intensity
    pub ao_map_intensity: f32,
    /// Fog
    pub fog: bool,
}

impl StandardMaterial {
    /// Create a new standard material
    pub fn new(name: &str) -> Self {
        Self {
            base: Material::new(name, MaterialType::Standard),
            color: [1.0, 1.0, 1.0],
            map: None,
            alpha_map: None,
            env_map: None,
            env_map_intensity: 1.0,
            emissive: [0.0, 0.0, 0.0],
            emissive_map: None,
            emissive_intensity: 1.0,
            metalness: 0.0,
            metalness_map: None,
            roughness: 1.0,
            roughness_map: None,
            normal_map: None,
            normal_scale: [1.0, 1.0],
            bump_map: None,
            bump_scale: 1.0,
            displacement_map: None,
            displacement_scale: 1.0,
            displacement_bias: 0.0,
            ao_map: None,
            ao_map_intensity: 1.0,
            fog: true,
        }
    }

    /// Set color
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set emissive color
    pub fn set_emissive(&mut self, r: f32, g: f32, b: f32) {
        self.emissive = [r, g, b];
    }

    /// Set metalness
    pub fn set_metalness(&mut self, metalness: f32) {
        self.metalness = metalness.clamp(0.0, 1.0);
    }

    /// Set roughness
    pub fn set_roughness(&mut self, roughness: f32) {
        self.roughness = roughness.clamp(0.0, 1.0);
    }

    /// Set diffuse map
    pub fn set_map(&mut self, map: Option<String>) {
        self.map = map;
    }

    /// Set metalness map
    pub fn set_metalness_map(&mut self, metalness_map: Option<String>) {
        self.metalness_map = metalness_map;
    }

    /// Set roughness map
    pub fn set_roughness_map(&mut self, roughness_map: Option<String>) {
        self.roughness_map = roughness_map;
    }

    /// Set normal map
    pub fn set_normal_map(&mut self, normal_map: Option<String>) {
        self.normal_map = normal_map;
    }
}
