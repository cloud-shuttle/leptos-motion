//! Shadow mapping system for WebGL rendering

use crate::error::{Result, WebGLError};
use crate::lighting::{DirectionalLight, PointLight, SpotLight};
use gl_matrix::mat4;
use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlRenderbuffer, WebGlTexture};

/// Shadow map resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowMapResolution {
    /// 512x512
    Low,
    /// 1024x1024
    Medium,
    /// 2048x2048
    High,
    /// 4096x4096
    Ultra,
}

impl ShadowMapResolution {
    /// Get the resolution value
    pub fn get_size(&self) -> u32 {
        match self {
            ShadowMapResolution::Low => 512,
            ShadowMapResolution::Medium => 1024,
            ShadowMapResolution::High => 2048,
            ShadowMapResolution::Ultra => 4096,
        }
    }
}

/// Shadow map configuration
#[derive(Debug, Clone)]
pub struct ShadowMapConfig {
    /// Shadow map resolution
    pub resolution: ShadowMapResolution,
    /// Near plane for shadow camera
    pub near_plane: f32,
    /// Far plane for shadow camera
    pub far_plane: f32,
    /// Shadow bias to prevent shadow acne
    pub bias: f32,
    /// Normal offset to prevent shadow acne
    pub normal_offset: f32,
    /// Shadow map filtering
    pub filtering: ShadowMapFiltering,
}

impl Default for ShadowMapConfig {
    fn default() -> Self {
        Self {
            resolution: ShadowMapResolution::High,
            near_plane: 0.1,
            far_plane: 100.0,
            bias: 0.005,
            normal_offset: 0.0,
            filtering: ShadowMapFiltering::PCF,
        }
    }
}

/// Shadow map filtering types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowMapFiltering {
    /// No filtering (hard shadows)
    None,
    /// Percentage Closer Filtering (soft shadows)
    PCF,
    /// Variance Shadow Maps (very soft shadows)
    VSM,
    /// Exponential Shadow Maps
    ESM,
}

/// Shadow map for directional lights
#[derive(Debug)]
pub struct DirectionalShadowMap {
    /// Shadow map texture
    pub shadow_map: WebGlTexture,
    /// Shadow map framebuffer
    pub framebuffer: WebGlFramebuffer,
    /// Shadow map resolution
    pub resolution: u32,
    /// Shadow map configuration
    pub config: ShadowMapConfig,
    /// Light projection matrix
    pub projection_matrix: [f32; 16],
    /// Light view matrix
    pub view_matrix: [f32; 16],
    /// Light view-projection matrix
    pub view_projection_matrix: [f32; 16],
}

impl DirectionalShadowMap {
    /// Create a new directional shadow map
    pub fn new(context: &WebGl2RenderingContext, config: ShadowMapConfig) -> Result<Self> {
        let resolution = config.resolution.get_size();

        // Create shadow map texture
        let shadow_map = context
            .create_texture()
            .ok_or_else(|| WebGLError::texture_error("Failed to create shadow map texture"))?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&shadow_map));
        context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::DEPTH_COMPONENT24 as i32,
            resolution as i32,
            resolution as i32,
            0,
            WebGl2RenderingContext::DEPTH_COMPONENT,
            WebGl2RenderingContext::UNSIGNED_INT,
            None,
        )?;

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        // Create framebuffer
        let framebuffer = context.create_framebuffer().ok_or_else(|| {
            WebGLError::framebuffer_error("Failed to create shadow map framebuffer")
        })?;

        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&shadow_map),
            0,
        );

        // Check framebuffer status
        let status = context.check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER);
        if status != WebGl2RenderingContext::FRAMEBUFFER_COMPLETE {
            return Err(WebGLError::framebuffer_error(
                "Shadow map framebuffer is not complete",
            ));
        }

        // Unbind
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        // Initialize matrices
        let mut projection_matrix = [0.0; 16];
        let mut view_matrix = [0.0; 16];
        let mut view_projection_matrix = [0.0; 16];

        mat4::identity(&mut projection_matrix);
        mat4::identity(&mut view_matrix);
        mat4::identity(&mut view_projection_matrix);

        Ok(Self {
            shadow_map,
            framebuffer,
            resolution,
            config,
            projection_matrix,
            view_matrix,
            view_projection_matrix,
        })
    }

    /// Update shadow map matrices for a directional light
    pub fn update_matrices(&mut self, light: &DirectionalLight, scene_bounds: &SceneBounds) {
        // Calculate light position (opposite to direction)
        let light_pos = [
            -light.direction[0] * 50.0,
            -light.direction[1] * 50.0,
            -light.direction[2] * 50.0,
        ];

        // Calculate light target (center of scene)
        let target = [
            (scene_bounds.min[0] + scene_bounds.max[0]) * 0.5,
            (scene_bounds.min[1] + scene_bounds.max[1]) * 0.5,
            (scene_bounds.min[2] + scene_bounds.max[2]) * 0.5,
        ];

        // Create view matrix
        let up = [0.0, 1.0, 0.0];
        mat4::look_at(&mut self.view_matrix, &light_pos, &target, &up);

        // Create orthographic projection matrix
        let size = self.calculate_orthographic_size(scene_bounds);
        mat4::ortho(
            &mut self.projection_matrix,
            -size,
            size,
            -size,
            size,
            self.config.near_plane,
            self.config.far_plane,
        );

        // Calculate view-projection matrix
        mat4::multiply(
            &mut self.view_projection_matrix,
            &self.projection_matrix,
            &self.view_matrix,
        );
    }

    /// Calculate orthographic size for the shadow map
    fn calculate_orthographic_size(&self, scene_bounds: &SceneBounds) -> f32 {
        let size_x = scene_bounds.max[0] - scene_bounds.min[0];
        let size_z = scene_bounds.max[2] - scene_bounds.min[2];
        size_x.max(size_z) * 0.5
    }

    /// Bind shadow map for rendering
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        context.viewport(0, 0, self.resolution as i32, self.resolution as i32);
        context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    /// Unbind shadow map
    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    }

    /// Get shadow map texture
    pub fn get_shadow_map(&self) -> &WebGlTexture {
        &self.shadow_map
    }

    /// Get view-projection matrix
    pub fn get_view_projection_matrix(&self) -> &[f32; 16] {
        &self.view_projection_matrix
    }
}

/// Shadow map for point lights
#[derive(Debug)]
pub struct PointShadowMap {
    /// Shadow map cubemap texture
    pub shadow_map: WebGlTexture,
    /// Shadow map framebuffer
    pub framebuffer: WebGlFramebuffer,
    /// Shadow map resolution
    pub resolution: u32,
    /// Shadow map configuration
    pub config: ShadowMapConfig,
    /// Light projection matrix (perspective)
    pub projection_matrix: [f32; 16],
    /// Light view matrices for each face
    pub view_matrices: [[f32; 16]; 6],
    /// Light view-projection matrices for each face
    pub view_projection_matrices: [[f32; 16]; 6],
}

impl PointShadowMap {
    /// Create a new point shadow map
    pub fn new(context: &WebGl2RenderingContext, config: ShadowMapConfig) -> Result<Self> {
        let resolution = config.resolution.get_size();

        // Create shadow map cubemap texture
        let shadow_map = context.create_texture().ok_or_else(|| {
            WebGLError::texture_error("Failed to create point shadow map texture")
        })?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, Some(&shadow_map));

        // Create each face of the cubemap
        for i in 0..6 {
            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                0,
                WebGl2RenderingContext::DEPTH_COMPONENT24 as i32,
                resolution as i32,
                resolution as i32,
                0,
                WebGl2RenderingContext::DEPTH_COMPONENT,
                WebGl2RenderingContext::UNSIGNED_INT,
                None,
            )?;
        }

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_R,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        // Create framebuffer
        let framebuffer = context.create_framebuffer().ok_or_else(|| {
            WebGLError::framebuffer_error("Failed to create point shadow map framebuffer")
        })?;

        // Initialize matrices
        let mut projection_matrix = [0.0; 16];
        let mut view_matrices = [[0.0; 16]; 6];
        let mut view_projection_matrices = [[0.0; 16]; 6];

        mat4::identity(&mut projection_matrix);
        for i in 0..6 {
            mat4::identity(&mut view_matrices[i]);
            mat4::identity(&mut view_projection_matrices[i]);
        }

        Ok(Self {
            shadow_map,
            framebuffer,
            resolution,
            config,
            projection_matrix,
            view_matrices,
            view_projection_matrices,
        })
    }

    /// Update shadow map matrices for a point light
    pub fn update_matrices(&mut self, light: &PointLight) {
        // Create perspective projection matrix
        mat4::perspective(
            &mut self.projection_matrix,
            std::f32::consts::PI / 2.0, // 90 degrees
            1.0,                        // Aspect ratio 1:1 for cubemap faces
            self.config.near_plane,
            Some(self.config.far_plane),
        );

        // Create view matrices for each cubemap face
        let light_pos = light.position;
        let up = [0.0, 1.0, 0.0];

        // Positive X
        let target = [light_pos[0] + 1.0, light_pos[1], light_pos[2]];
        mat4::look_at(&mut self.view_matrices[0], &light_pos, &target, &up);

        // Negative X
        let target = [light_pos[0] - 1.0, light_pos[1], light_pos[2]];
        mat4::look_at(&mut self.view_matrices[1], &light_pos, &target, &up);

        // Positive Y
        let target = [light_pos[0], light_pos[1] + 1.0, light_pos[2]];
        mat4::look_at(
            &mut self.view_matrices[2],
            &light_pos,
            &target,
            &[1.0, 0.0, 0.0],
        );

        // Negative Y
        let target = [light_pos[0], light_pos[1] - 1.0, light_pos[2]];
        mat4::look_at(
            &mut self.view_matrices[3],
            &light_pos,
            &target,
            &[1.0, 0.0, 0.0],
        );

        // Positive Z
        let target = [light_pos[0], light_pos[1], light_pos[2] + 1.0];
        mat4::look_at(&mut self.view_matrices[4], &light_pos, &target, &up);

        // Negative Z
        let target = [light_pos[0], light_pos[1], light_pos[2] - 1.0];
        mat4::look_at(&mut self.view_matrices[5], &light_pos, &target, &up);

        // Calculate view-projection matrices
        for i in 0..6 {
            mat4::multiply(
                &mut self.view_projection_matrices[i],
                &self.projection_matrix,
                &self.view_matrices[i],
            );
        }
    }

    /// Bind shadow map for rendering a specific face
    pub fn bind_face(&self, context: &WebGl2RenderingContext, face: u32) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X + face,
            Some(&self.shadow_map),
            0,
        );
        context.viewport(0, 0, self.resolution as i32, self.resolution as i32);
        context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    /// Unbind shadow map
    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    }

    /// Get shadow map texture
    pub fn get_shadow_map(&self) -> &WebGlTexture {
        &self.shadow_map
    }

    /// Get view-projection matrix for a specific face
    pub fn get_view_projection_matrix(&self, face: usize) -> &[f32; 16] {
        &self.view_projection_matrices[face]
    }
}

/// Scene bounds for shadow map calculation
#[derive(Debug, Clone)]
pub struct SceneBounds {
    /// Minimum bounds
    pub min: [f32; 3],
    /// Maximum bounds
    pub max: [f32; 3],
}

impl SceneBounds {
    /// Create new scene bounds
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max }
    }

    /// Expand bounds to include a point
    pub fn expand(&mut self, point: [f32; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);

        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
    }

    /// Get center of bounds
    pub fn get_center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Get size of bounds
    pub fn get_size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }
}

/// Shadow mapping manager
pub struct ShadowMappingManager {
    /// WebGL context
    context: WebGl2RenderingContext,
    /// Directional shadow maps
    directional_shadow_maps: HashMap<String, DirectionalShadowMap>,
    /// Point shadow maps
    point_shadow_maps: HashMap<String, PointShadowMap>,
    /// Maximum number of directional shadow maps
    max_directional_shadow_maps: usize,
    /// Maximum number of point shadow maps
    max_point_shadow_maps: usize,
}

impl ShadowMappingManager {
    /// Create a new shadow mapping manager
    pub fn new(context: WebGl2RenderingContext) -> Self {
        Self {
            context,
            directional_shadow_maps: HashMap::new(),
            point_shadow_maps: HashMap::new(),
            max_directional_shadow_maps: 4,
            max_point_shadow_maps: 2,
        }
    }

    /// Add a directional shadow map
    pub fn add_directional_shadow_map(
        &mut self,
        name: &str,
        config: ShadowMapConfig,
    ) -> Result<()> {
        if self.directional_shadow_maps.len() >= self.max_directional_shadow_maps {
            return Err(WebGLError::lighting_error(
                "Maximum number of directional shadow maps reached",
            ));
        }

        let shadow_map = DirectionalShadowMap::new(&self.context, config)?;
        self.directional_shadow_maps
            .insert(name.to_string(), shadow_map);
        Ok(())
    }

    /// Add a point shadow map
    pub fn add_point_shadow_map(&mut self, name: &str, config: ShadowMapConfig) -> Result<()> {
        if self.point_shadow_maps.len() >= self.max_point_shadow_maps {
            return Err(WebGLError::lighting_error(
                "Maximum number of point shadow maps reached",
            ));
        }

        let shadow_map = PointShadowMap::new(&self.context, config)?;
        self.point_shadow_maps.insert(name.to_string(), shadow_map);
        Ok(())
    }

    /// Get a directional shadow map
    pub fn get_directional_shadow_map(&self, name: &str) -> Option<&DirectionalShadowMap> {
        self.directional_shadow_maps.get(name)
    }

    /// Get a mutable directional shadow map
    pub fn get_directional_shadow_map_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut DirectionalShadowMap> {
        self.directional_shadow_maps.get_mut(name)
    }

    /// Get a point shadow map
    pub fn get_point_shadow_map(&self, name: &str) -> Option<&PointShadowMap> {
        self.point_shadow_maps.get(name)
    }

    /// Get a mutable point shadow map
    pub fn get_point_shadow_map_mut(&mut self, name: &str) -> Option<&mut PointShadowMap> {
        self.point_shadow_maps.get_mut(name)
    }

    /// Remove a directional shadow map
    pub fn remove_directional_shadow_map(&mut self, name: &str) -> Option<DirectionalShadowMap> {
        self.directional_shadow_maps.remove(name)
    }

    /// Remove a point shadow map
    pub fn remove_point_shadow_map(&mut self, name: &str) -> Option<PointShadowMap> {
        self.point_shadow_maps.remove(name)
    }

    /// Get all directional shadow maps
    pub fn get_directional_shadow_maps(&self) -> &HashMap<String, DirectionalShadowMap> {
        &self.directional_shadow_maps
    }

    /// Get all point shadow maps
    pub fn get_point_shadow_maps(&self) -> &HashMap<String, PointShadowMap> {
        &self.point_shadow_maps
    }

    /// Get total shadow map count
    pub fn get_total_shadow_map_count(&self) -> usize {
        self.directional_shadow_maps.len() + self.point_shadow_maps.len()
    }

    /// Clear all shadow maps
    pub fn clear(&mut self) {
        self.directional_shadow_maps.clear();
        self.point_shadow_maps.clear();
    }

    /// Set maximum number of shadow maps
    pub fn set_max_shadow_maps(&mut self, directional: usize, point: usize) {
        self.max_directional_shadow_maps = directional;
        self.max_point_shadow_maps = point;
    }

    /// Get maximum number of shadow maps
    pub fn get_max_shadow_maps(&self) -> (usize, usize) {
        (self.max_directional_shadow_maps, self.max_point_shadow_maps)
    }

    /// Update all shadow maps
    pub fn update_shadow_maps(&mut self, lights: &LightCollection, scene_bounds: &SceneBounds) {
        // Update directional shadow maps
        for (name, shadow_map) in &mut self.directional_shadow_maps {
            if let Some(light) = lights.get_directional_light(name) {
                shadow_map.update_matrices(light, scene_bounds);
            }
        }

        // Update point shadow maps
        for (name, shadow_map) in &mut self.point_shadow_maps {
            if let Some(light) = lights.get_point_light(name) {
                shadow_map.update_matrices(light);
            }
        }
    }
}

/// Collection of lights for shadow map updates
pub struct LightCollection {
    /// Directional lights
    pub directional_lights: HashMap<String, DirectionalLight>,
    /// Point lights
    pub point_lights: HashMap<String, PointLight>,
}

impl LightCollection {
    /// Create a new light collection
    pub fn new() -> Self {
        Self {
            directional_lights: HashMap::new(),
            point_lights: HashMap::new(),
        }
    }

    /// Add a directional light
    pub fn add_directional_light(&mut self, name: String, light: DirectionalLight) {
        self.directional_lights.insert(name, light);
    }

    /// Add a point light
    pub fn add_point_light(&mut self, name: String, light: PointLight) {
        self.point_lights.insert(name, light);
    }

    /// Get a directional light
    pub fn get_directional_light(&self, name: &str) -> Option<&DirectionalLight> {
        self.directional_lights.get(name)
    }

    /// Get a point light
    pub fn get_point_light(&self, name: &str) -> Option<&PointLight> {
        self.point_lights.get(name)
    }
}
