//! WebGL capabilities detection

use crate::{Result, StudioError};

/// WebGL capabilities
#[derive(Debug, Clone)]
pub struct WebGLCapabilities {
    /// WebGL version
    pub webgl_version: String,
    /// Vendor information
    pub vendor: String,
    /// Renderer information
    pub renderer: String,
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum cube map texture size
    pub max_cube_map_texture_size: u32,
    /// Maximum render buffer size
    pub max_render_buffer_size: u32,
    /// Maximum vertex attributes
    pub max_vertex_attributes: u32,
    /// Maximum vertex uniform vectors
    pub max_vertex_uniform_vectors: u32,
    /// Maximum varying vectors
    pub max_varying_vectors: u32,
    /// Maximum fragment uniform vectors
    pub max_fragment_uniform_vectors: u32,
    /// Maximum vertex texture image units
    pub max_vertex_texture_image_units: u32,
    /// Maximum texture image units
    pub max_texture_image_units: u32,
    /// Maximum combined texture image units
    pub max_combined_texture_image_units: u32,
    /// Maximum viewport dimensions
    pub max_viewport_dims: (u32, u32),
    /// Maximum texture size
    pub max_texture_size_2d: u32,
    /// Whether anisotropic filtering is supported
    pub anisotropic_filtering: bool,
    /// Whether depth textures are supported
    pub depth_textures: bool,
    /// Whether vertex array objects are supported
    pub vertex_array_objects: bool,
    /// Whether instanced arrays are supported
    pub instanced_arrays: bool,
    /// Whether multiple render targets are supported
    pub multiple_render_targets: bool,
}

impl WebGLCapabilities {
    /// Detect WebGL capabilities
    pub fn detect(context: &web_sys::WebGlRenderingContext) -> Result<Self> {
        let webgl_version = "1.0".to_string(); // Simplified
        let vendor = context.get_parameter(web_sys::WebGlRenderingContext::VENDOR)
            .unwrap_or_else(|| "Unknown".to_string());
        let renderer = context.get_parameter(web_sys::WebGlRenderingContext::RENDERER)
            .unwrap_or_else(|| "Unknown".to_string());

        let max_texture_size = context.get_parameter(web_sys::WebGlRenderingContext::MAX_TEXTURE_SIZE)
            .unwrap_or_else(|| 1024.into());
        let max_cube_map_texture_size = context.get_parameter(web_sys::WebGlRenderingContext::MAX_CUBE_MAP_TEXTURE_SIZE)
            .unwrap_or_else(|| 1024.into());
        let max_render_buffer_size = context.get_parameter(web_sys::WebGlRenderingContext::MAX_RENDERBUFFER_SIZE)
            .unwrap_or_else(|| 1024.into());
        let max_vertex_attributes = context.get_parameter(web_sys::WebGlRenderingContext::MAX_VERTEX_ATTRIBS)
            .unwrap_or_else(|| 16.into());
        let max_vertex_uniform_vectors = context.get_parameter(web_sys::WebGlRenderingContext::MAX_VERTEX_UNIFORM_VECTORS)
            .unwrap_or_else(|| 128.into());
        let max_varying_vectors = context.get_parameter(web_sys::WebGlRenderingContext::MAX_VARYING_VECTORS)
            .unwrap_or_else(|| 8.into());
        let max_fragment_uniform_vectors = context.get_parameter(web_sys::WebGlRenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS)
            .unwrap_or_else(|| 16.into());
        let max_vertex_texture_image_units = context.get_parameter(web_sys::WebGlRenderingContext::MAX_VERTEX_TEXTURE_IMAGE_UNITS)
            .unwrap_or_else(|| 0.into());
        let max_texture_image_units = context.get_parameter(web_sys::WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS)
            .unwrap_or_else(|| 8.into());
        let max_combined_texture_image_units = context.get_parameter(web_sys::WebGlRenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS)
            .unwrap_or_else(|| 8.into());

        let max_viewport_dims = (4096, 4096); // Simplified
        let max_texture_size_2d = max_texture_size;

        // Check for extensions
        let anisotropic_filtering = context.get_extension("WEBKIT_EXT_texture_filter_anisotropic").is_ok() ||
                                   context.get_extension("EXT_texture_filter_anisotropic").is_ok();
        let depth_textures = context.get_extension("WEBGL_depth_texture").is_ok();
        let vertex_array_objects = context.get_extension("OES_vertex_array_object").is_ok();
        let instanced_arrays = context.get_extension("ANGLE_instanced_arrays").is_ok();
        let multiple_render_targets = context.get_extension("WEBGL_draw_buffers").is_ok();

        Ok(Self {
            webgl_version,
            vendor,
            renderer,
            max_texture_size,
            max_cube_map_texture_size,
            max_render_buffer_size,
            max_vertex_attributes,
            max_vertex_uniform_vectors,
            max_varying_vectors,
            max_fragment_uniform_vectors,
            max_vertex_texture_image_units,
            max_texture_image_units,
            max_combined_texture_image_units,
            max_viewport_dims,
            max_texture_size_2d,
            anisotropic_filtering,
            depth_textures,
            vertex_array_objects,
            instanced_arrays,
            multiple_render_targets,
        })
    }

    /// Check if a specific capability is supported
    pub fn supports(&self, capability: &str) -> bool {
        match capability {
            "anisotropic_filtering" => self.anisotropic_filtering,
            "depth_textures" => self.depth_textures,
            "vertex_array_objects" => self.vertex_array_objects,
            "instanced_arrays" => self.instanced_arrays,
            "multiple_render_targets" => self.multiple_render_targets,
            _ => false,
        }
    }

    /// Get capability information as a string
    pub fn get_capability_info(&self) -> String {
        format!(
            "WebGL Version: {}\nVendor: {}\nRenderer: {}\nMax Texture Size: {}\nMax Vertex Attributes: {}\nAnisotropic Filtering: {}\nDepth Textures: {}\nVertex Array Objects: {}\nInstanced Arrays: {}\nMultiple Render Targets: {}",
            self.webgl_version,
            self.vendor,
            self.renderer,
            self.max_texture_size,
            self.max_vertex_attributes,
            self.anisotropic_filtering,
            self.depth_textures,
            self.vertex_array_objects,
            self.instanced_arrays,
            self.multiple_render_targets
        )
    }

    /// Check if the capabilities are sufficient for basic rendering
    pub fn is_sufficient_for_basic_rendering(&self) -> bool {
        self.max_texture_size >= 512 &&
        self.max_vertex_attributes >= 8 &&
        self.max_vertex_uniform_vectors >= 64 &&
        self.max_fragment_uniform_vectors >= 16
    }

    /// Check if the capabilities are sufficient for advanced rendering
    pub fn is_sufficient_for_advanced_rendering(&self) -> bool {
        self.is_sufficient_for_basic_rendering() &&
        self.max_texture_size >= 2048 &&
        self.max_vertex_attributes >= 16 &&
        self.anisotropic_filtering &&
        self.vertex_array_objects
    }

    /// Get performance score (0.0 to 1.0)
    pub fn get_performance_score(&self) -> f64 {
        let mut score = 0.0;

        // Texture size score
        if self.max_texture_size >= 4096 {
            score += 0.2;
        } else if self.max_texture_size >= 2048 {
            score += 0.15;
        } else if self.max_texture_size >= 1024 {
            score += 0.1;
        }

        // Vertex attributes score
        if self.max_vertex_attributes >= 16 {
            score += 0.2;
        } else if self.max_vertex_attributes >= 8 {
            score += 0.15;
        } else if self.max_vertex_attributes >= 4 {
            score += 0.1;
        }

        // Uniform vectors score
        if self.max_vertex_uniform_vectors >= 256 {
            score += 0.2;
        } else if self.max_vertex_uniform_vectors >= 128 {
            score += 0.15;
        } else if self.max_vertex_uniform_vectors >= 64 {
            score += 0.1;
        }

        // Extensions score
        if self.anisotropic_filtering { score += 0.1; }
        if self.depth_textures { score += 0.1; }
        if self.vertex_array_objects { score += 0.1; }
        if self.instanced_arrays { score += 0.1; }
        if self.multiple_render_targets { score += 0.1; }

        score.min(1.0)
    }
}
