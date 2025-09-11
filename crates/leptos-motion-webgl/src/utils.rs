//! Utility functions for WebGL operations

use crate::error::{Result, WebGLError};
use gl_matrix::mat4;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

/// WebGL utility functions
pub struct WebGLUtils;

impl WebGLUtils {
    /// Create a vertex buffer
    pub fn create_buffer(context: &WebGl2RenderingContext) -> Result<WebGlBuffer> {
        context
            .create_buffer()
            .ok_or_else(|| WebGLError::buffer_error("Failed to create buffer"))
    }

    /// Create a vertex array object
    pub fn create_vertex_array_object(
        context: &WebGl2RenderingContext,
    ) -> Result<WebGlVertexArrayObject> {
        context
            .create_vertex_array()
            .ok_or_else(|| WebGLError::buffer_error("Failed to create vertex array object"))
    }

    /// Upload data to a buffer
    pub fn upload_buffer_data(
        context: &WebGl2RenderingContext,
        buffer: &WebGlBuffer,
        data: &[f32],
        usage: u32,
    ) -> Result<()> {
        let array = js_sys::Float32Array::new_with_length(data.len() as u32);
        for (i, &value) in data.iter().enumerate() {
            array.set_index(i as u32, value);
        }

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer));
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array,
            usage,
        );

        Ok(())
    }

    /// Upload index data to a buffer
    pub fn upload_index_buffer_data(
        context: &WebGl2RenderingContext,
        buffer: &WebGlBuffer,
        data: &[u32],
        usage: u32,
    ) -> Result<()> {
        let array = js_sys::Uint32Array::new_with_length(data.len() as u32);
        for (i, &value) in data.iter().enumerate() {
            array.set_index(i as u32, value);
        }

        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer));
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &array,
            usage,
        );

        Ok(())
    }

    /// Set up vertex attribute
    pub fn setup_vertex_attribute(
        context: &WebGl2RenderingContext,
        location: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        context.enable_vertex_attrib_array(location);
        context
            .vertex_attrib_pointer_with_i32(location, size, data_type, normalized, stride, offset);
    }

    /// Create a perspective matrix
    pub fn create_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::perspective(&mut matrix, fov, aspect, near, Some(far));
        matrix
    }

    /// Create an orthographic matrix
    pub fn create_orthographic_matrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::ortho(&mut matrix, left, right, bottom, top, near, far);
        matrix
    }

    /// Create a look-at matrix
    pub fn create_look_at_matrix(eye: &[f32; 3], center: &[f32; 3], up: &[f32; 3]) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::look_at(&mut matrix, eye, center, up);
        matrix
    }

    /// Create a translation matrix
    pub fn create_translation_matrix(translation: &[f32; 3]) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::identity(&mut matrix);
        let temp = matrix;
        mat4::translate(&mut matrix, &temp, translation);
        matrix
    }

    /// Create a rotation matrix
    pub fn create_rotation_matrix(rotation: &[f32; 3]) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::identity(&mut matrix);
        let mut temp = matrix;
        mat4::rotate_z(&mut matrix, &temp, rotation[2]);
        temp = matrix;
        mat4::rotate_y(&mut matrix, &temp, rotation[1]);
        temp = matrix;
        mat4::rotate_x(&mut matrix, &temp, rotation[0]);
        matrix
    }

    /// Create a scale matrix
    pub fn create_scale_matrix(scale: &[f32; 3]) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::identity(&mut matrix);
        let temp = matrix;
        mat4::scale(&mut matrix, &temp, scale);
        matrix
    }

    /// Create a transformation matrix
    pub fn create_transformation_matrix(
        translation: &[f32; 3],
        rotation: &[f32; 3],
        scale: &[f32; 3],
    ) -> [f32; 16] {
        let mut matrix = [0.0; 16];
        mat4::identity(&mut matrix);

        // Apply translation
        let mut temp = matrix;
        mat4::translate(&mut matrix, &temp, translation);

        // Apply rotation (ZYX order)
        temp = matrix;
        mat4::rotate_z(&mut matrix, &temp, rotation[2]);
        temp = matrix;
        mat4::rotate_y(&mut matrix, &temp, rotation[1]);
        temp = matrix;
        mat4::rotate_x(&mut matrix, &temp, rotation[0]);

        // Apply scale
        temp = matrix;
        mat4::scale(&mut matrix, &temp, scale);

        matrix
    }

    /// Convert degrees to radians
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    /// Convert radians to degrees
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * 180.0 / std::f32::consts::PI
    }

    /// Clamp a value between min and max
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.min(max).max(min)
    }

    /// Linear interpolation
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// Smooth step interpolation
    pub fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = Self::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Smoother step interpolation
    pub fn smoother_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = Self::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Get WebGL error string
    pub fn get_webgl_error_string(_context: &WebGl2RenderingContext, error: u32) -> &'static str {
        match error {
            WebGl2RenderingContext::INVALID_ENUM => "INVALID_ENUM",
            WebGl2RenderingContext::INVALID_VALUE => "INVALID_VALUE",
            WebGl2RenderingContext::INVALID_OPERATION => "INVALID_OPERATION",
            WebGl2RenderingContext::INVALID_FRAMEBUFFER_OPERATION => {
                "INVALID_FRAMEBUFFER_OPERATION"
            }
            WebGl2RenderingContext::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            WebGl2RenderingContext::CONTEXT_LOST_WEBGL => "CONTEXT_LOST_WEBGL",
            _ => "UNKNOWN_ERROR",
        }
    }

    /// Check for WebGL errors
    pub fn check_webgl_errors(context: &WebGl2RenderingContext) -> Result<()> {
        let error = context.get_error();
        if error != WebGl2RenderingContext::NO_ERROR {
            let error_string = Self::get_webgl_error_string(context, error);
            return Err(WebGLError::invalid_operation(&format!(
                "WebGL error: {}",
                error_string
            )));
        }
        Ok(())
    }

    /// Get WebGL context info
    pub fn get_context_info(context: &WebGl2RenderingContext) -> ContextInfo {
        ContextInfo {
            version: context
                .get_parameter(WebGl2RenderingContext::VERSION)
                .unwrap_or_else(|_| "Unknown".into())
                .as_string()
                .unwrap_or_else(|| "Unknown".to_string()),
            vendor: context
                .get_parameter(WebGl2RenderingContext::VENDOR)
                .unwrap_or_else(|_| "Unknown".into())
                .as_string()
                .unwrap_or_else(|| "Unknown".to_string()),
            renderer: context
                .get_parameter(WebGl2RenderingContext::RENDERER)
                .unwrap_or_else(|_| "Unknown".into())
                .as_string()
                .unwrap_or_else(|| "Unknown".to_string()),
            shading_language_version: context
                .get_parameter(WebGl2RenderingContext::SHADING_LANGUAGE_VERSION)
                .unwrap_or_else(|_| "Unknown".into())
                .as_string()
                .unwrap_or_else(|| "Unknown".to_string()),
            max_texture_size: context
                .get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE)
                .unwrap_or_else(|_| 0.into())
                .as_f64()
                .unwrap_or(0.0) as u32,
            max_vertex_attribs: context
                .get_parameter(WebGl2RenderingContext::MAX_VERTEX_ATTRIBS)
                .unwrap_or_else(|_| 0.into())
                .as_f64()
                .unwrap_or(0.0) as u32,
            max_vertex_uniform_vectors: context
                .get_parameter(WebGl2RenderingContext::MAX_VERTEX_UNIFORM_VECTORS)
                .unwrap_or_else(|_| 0.into())
                .as_f64()
                .unwrap_or(0.0) as u32,
            max_fragment_uniform_vectors: context
                .get_parameter(WebGl2RenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS)
                .unwrap_or_else(|_| 0.into())
                .as_f64()
                .unwrap_or(0.0) as u32,
            max_varying_vectors: context
                .get_parameter(WebGl2RenderingContext::MAX_VARYING_VECTORS)
                .unwrap_or_else(|_| 0.into())
                .as_f64()
                .unwrap_or(0.0) as u32,
        }
    }
}

/// WebGL context information
#[derive(Debug, Clone)]
pub struct ContextInfo {
    /// WebGL version
    pub version: String,
    /// GPU vendor
    pub vendor: String,
    /// GPU renderer
    pub renderer: String,
    /// Shading language version
    pub shading_language_version: String,
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum vertex attributes
    pub max_vertex_attribs: u32,
    /// Maximum vertex uniform vectors
    pub max_vertex_uniform_vectors: u32,
    /// Maximum fragment uniform vectors
    pub max_fragment_uniform_vectors: u32,
    /// Maximum varying vectors
    pub max_varying_vectors: u32,
}
