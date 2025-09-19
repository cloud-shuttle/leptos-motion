//! WebGL shader program management

use crate::{Result, StudioError};

/// WebGL shader program
pub struct ShaderProgram {
    /// WebGL program
    program: web_sys::WebGlProgram,
    /// Vertex shader
    vertex_shader: web_sys::WebGlShader,
    /// Fragment shader
    fragment_shader: web_sys::WebGlShader,
    /// Uniform locations cache
    uniform_locations: std::collections::HashMap<String, Option<web_sys::WebGlUniformLocation>>,
    /// Attribute locations cache
    attribute_locations: std::collections::HashMap<String, u32>,
}

impl ShaderProgram {
    /// Create a new shader program
    pub fn new(
        context: &web_sys::WebGlRenderingContext,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Self> {
        let vertex_shader = Self::compile_shader(
            context,
            web_sys::WebGlRenderingContext::VERTEX_SHADER,
            vertex_source,
        )?;

        let fragment_shader = Self::compile_shader(
            context,
            web_sys::WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_source,
        )?;

        let program = Self::link_program(context, &vertex_shader, &fragment_shader)?;

        Ok(Self {
            program,
            vertex_shader,
            fragment_shader,
            uniform_locations: std::collections::HashMap::new(),
            attribute_locations: std::collections::HashMap::new(),
        })
    }

    /// Compile a shader
    fn compile_shader(
        context: &web_sys::WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<web_sys::WebGlShader> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| StudioError::InvalidState("Failed to create shader".to_string()))?;

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, web_sys::WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let error = context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown shader compilation error".to_string());
            Err(StudioError::InvalidState(format!("Shader compilation failed: {}", error)))
        }
    }

    /// Link a shader program
    fn link_program(
        context: &web_sys::WebGlRenderingContext,
        vertex_shader: &web_sys::WebGlShader,
        fragment_shader: &web_sys::WebGlShader,
    ) -> Result<web_sys::WebGlProgram> {
        let program = context
            .create_program()
            .ok_or_else(|| StudioError::InvalidState("Failed to create program".to_string()))?;

        context.attach_shader(&program, vertex_shader);
        context.attach_shader(&program, fragment_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, web_sys::WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            let error = context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown program linking error".to_string());
            Err(StudioError::InvalidState(format!("Program linking failed: {}", error)))
        }
    }

    /// Use this shader program
    pub fn use_program(&self, context: &web_sys::WebGlRenderingContext) {
        context.use_program(Some(&self.program));
    }

    /// Get uniform location
    pub fn get_uniform_location(
        &mut self,
        context: &web_sys::WebGlRenderingContext,
        name: &str,
    ) -> Option<&web_sys::WebGlUniformLocation> {
        if !self.uniform_locations.contains_key(name) {
            let location = context.get_uniform_location(&self.program, name);
            self.uniform_locations.insert(name.to_string(), location);
        }
        self.uniform_locations.get(name)?.as_ref()
    }

    /// Get attribute location
    pub fn get_attribute_location(
        &mut self,
        context: &web_sys::WebGlRenderingContext,
        name: &str,
    ) -> u32 {
        if !self.attribute_locations.contains_key(name) {
            let location = context.get_attrib_location(&self.program, name);
            self.attribute_locations.insert(name.to_string(), location as u32);
        }
        self.attribute_locations[name]
    }

    /// Set uniform float
    pub fn set_uniform_1f(
        &mut self,
        context: &web_sys::WebGlRenderingContext,
        name: &str,
        value: f32,
    ) {
        if let Some(location) = self.get_uniform_location(context, name) {
            context.uniform1f(Some(location), value);
        }
    }

    /// Set uniform vec3
    pub fn set_uniform_3f(
        &mut self,
        context: &web_sys::WebGlRenderingContext,
        name: &str,
        x: f32,
        y: f32,
        z: f32,
    ) {
        if let Some(location) = self.get_uniform_location(context, name) {
            context.uniform3f(Some(location), x, y, z);
        }
    }

    /// Set uniform mat4
    pub fn set_uniform_matrix4fv(
        &mut self,
        context: &web_sys::WebGlRenderingContext,
        name: &str,
        transpose: bool,
        matrix: &[f32],
    ) {
        if let Some(location) = self.get_uniform_location(context, name) {
            context.uniform_matrix4fv_with_f32_array(Some(location), transpose, matrix);
        }
    }

    /// Get the WebGL program
    pub fn program(&self) -> &web_sys::WebGlProgram {
        &self.program
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        // Note: In a real implementation, you'd need access to the WebGL context
        // to properly clean up shaders and programs
    }
}
