//! Shader management system

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

/// Shader type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    fn to_webgl_type(&self) -> u32 {
        match self {
            ShaderType::Vertex => WebGl2RenderingContext::VERTEX_SHADER,
            ShaderType::Fragment => WebGl2RenderingContext::FRAGMENT_SHADER,
        }
    }
}

/// Shader program
#[derive(Debug)]
pub struct ShaderProgram {
    /// WebGL program
    program: WebGlProgram,
    /// Uniform locations cache
    uniform_locations: HashMap<String, Option<WebGlUniformLocation>>,
    /// Attribute locations cache
    attribute_locations: HashMap<String, u32>,
}

impl ShaderProgram {
    /// Create a new shader program
    pub fn new(context: &WebGl2RenderingContext, program: WebGlProgram) -> Self {
        Self {
            program,
            uniform_locations: HashMap::new(),
            attribute_locations: HashMap::new(),
        }
    }

    /// Get the WebGL program
    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    /// Get uniform location
    pub fn get_uniform_location(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
    ) -> Option<&WebGlUniformLocation> {
        if !self.uniform_locations.contains_key(name) {
            let location = context.get_uniform_location(&self.program, name);
            self.uniform_locations.insert(name.to_string(), location);
        }

        self.uniform_locations
            .get(name)
            .and_then(|loc| loc.as_ref())
    }

    /// Get attribute location
    pub fn get_attribute_location(&mut self, context: &WebGl2RenderingContext, name: &str) -> u32 {
        if !self.attribute_locations.contains_key(name) {
            let location = context.get_attrib_location(&self.program, name) as u32;
            self.attribute_locations.insert(name.to_string(), location);
        }

        self.attribute_locations[name]
    }

    /// Use this program
    pub fn use_program(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }
}

/// Shader manager
pub struct ShaderManager {
    /// WebGL context
    context: Rc<WebGl2RenderingContext>,
    /// Compiled shaders cache
    shaders: HashMap<String, WebGlShader>,
    /// Linked programs cache
    programs: HashMap<String, ShaderProgram>,
}

impl ShaderManager {
    /// Create a new shader manager
    pub fn new(context: &WebGl2RenderingContext) -> Result<Self> {
        Ok(Self {
            context: Rc::new(context.clone()),
            shaders: HashMap::new(),
            programs: HashMap::new(),
        })
    }

    /// Compile a shader
    pub fn compile_shader(&mut self, source: &str, shader_type: ShaderType) -> Result<WebGlShader> {
        let context = &self.context;

        let shader = context
            .create_shader(shader_type.to_webgl_type())
            .ok_or_else(|| WebGLError::shader_compilation("Failed to create shader"))?;

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if !context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let error = context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown shader compilation error".to_string());

            context.delete_shader(Some(&shader));
            return Err(WebGLError::shader_compilation(&error));
        }

        Ok(shader)
    }

    /// Create a shader program from vertex and fragment shader sources
    pub fn create_program(
        &mut self,
        vertex_source: &str,
        fragment_source: &str,
        name: &str,
    ) -> Result<&mut ShaderProgram> {
        // Compile shaders
        let vertex_shader = self.compile_shader(vertex_source, ShaderType::Vertex)?;
        let fragment_shader = self.compile_shader(fragment_source, ShaderType::Fragment)?;

        // Create program
        let program = self
            .context
            .create_program()
            .ok_or_else(|| WebGLError::program_linking("Failed to create program"))?;

        self.context.attach_shader(&program, &vertex_shader);
        self.context.attach_shader(&program, &fragment_shader);
        self.context.link_program(&program);

        if !self
            .context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let error = self
                .context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown program linking error".to_string());

            self.context.delete_program(Some(&program));
            self.context.delete_shader(Some(&vertex_shader));
            self.context.delete_shader(Some(&fragment_shader));
            return Err(WebGLError::program_linking(&error));
        }

        // Clean up shaders
        self.context.delete_shader(Some(&vertex_shader));
        self.context.delete_shader(Some(&fragment_shader));

        // Store program
        let shader_program = ShaderProgram::new(&self.context, program);
        self.programs.insert(name.to_string(), shader_program);

        Ok(self.programs.get_mut(name).unwrap())
    }

    /// Get a shader program by name
    pub fn get_program(&mut self, name: &str) -> Option<&mut ShaderProgram> {
        self.programs.get_mut(name)
    }
    
    /// Get current active program
    pub fn get_current_program(&mut self) -> Option<&mut ShaderProgram> {
        // For now, return the first program as current
        // In a real implementation, this would track the currently bound program
        self.programs.values_mut().next()
    }
    
    /// Set lighting uniforms for the current program
    pub fn set_lighting_uniforms(&mut self, context: &WebGl2RenderingContext, lighting_manager: &crate::lighting::LightingManager) -> Result<()> {
        if let Some(program) = self.get_current_program() {
            // Set ambient light uniforms
            let ambient_count = lighting_manager.ambient_lights.len() as i32;
            if let Some(location) = program.get_uniform_location(context, "uAmbientLightCount") {
                context.uniform1i(Some(location), ambient_count);
            }
            
            // Set directional light uniforms
            let directional_count = lighting_manager.directional_lights.len() as i32;
            if let Some(location) = program.get_uniform_location(context, "uDirectionalLightCount") {
                context.uniform1i(Some(location), directional_count);
            }
            
            // Set point light uniforms
            let point_count = lighting_manager.point_lights.len() as i32;
            if let Some(location) = program.get_uniform_location(context, "uPointLightCount") {
                context.uniform1i(Some(location), point_count);
            }
            
            // Set spot light uniforms
            let spot_count = lighting_manager.spot_lights.len() as i32;
            if let Some(location) = program.get_uniform_location(context, "uSpotLightCount") {
                context.uniform1i(Some(location), spot_count);
            }
            
            // Set individual light data
            for (i, (_, ambient_light)) in lighting_manager.ambient_lights.iter().enumerate() {
                let uniform_name = format!("uAmbientLights[{}].color", i);
                if let Some(location) = program.get_uniform_location(context, &uniform_name) {
                    let color = ambient_light.light.color.as_rgb_array();
                    context.uniform3fv_with_f32_array(Some(location), &color);
                }
            }
            
            for (i, (_, directional_light)) in lighting_manager.directional_lights.iter().enumerate() {
                let color_uniform = format!("uDirectionalLights[{}].color", i);
                let direction_uniform = format!("uDirectionalLights[{}].direction", i);
                
                if let Some(location) = program.get_uniform_location(context, &color_uniform) {
                    let color = directional_light.light.color.as_rgb_array();
                    context.uniform3fv_with_f32_array(Some(location), &color);
                }
                
                if let Some(location) = program.get_uniform_location(context, &direction_uniform) {
                    let direction = directional_light.direction;
                    context.uniform3fv_with_f32_array(Some(location), &direction);
                }
            }
        }
        
        Ok(())
    }

    /// Create default shader programs
    pub fn create_default_programs(&mut self) -> Result<()> {
        // Basic vertex shader
        let vertex_source = r#"
            #version 300 es
            in vec3 position;
            in vec3 normal;
            in vec2 uv;

            uniform mat4 modelMatrix;
            uniform mat4 viewMatrix;
            uniform mat4 projectionMatrix;
            uniform mat3 normalMatrix;

            out vec3 vNormal;
            out vec2 vUv;
            out vec3 vWorldPosition;

            void main() {
                vUv = uv;
                vNormal = normalize(normalMatrix * normal);
                vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;

                gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
            }
        "#;

        // Basic fragment shader
        let fragment_source = r#"
            #version 300 es
            precision highp float;

            in vec3 vNormal;
            in vec2 vUv;
            in vec3 vWorldPosition;

            uniform vec3 diffuse;
            uniform float opacity;
            uniform vec3 ambientLightColor;
            uniform vec3 directionalLightColor;
            uniform vec3 directionalLightDirection;

            out vec4 fragColor;

            void main() {
                vec3 normal = normalize(vNormal);
                vec3 lightDirection = normalize(directionalLightDirection);

                float lambert = max(dot(normal, lightDirection), 0.0);
                vec3 lightColor = ambientLightColor + directionalLightColor * lambert;

                vec3 color = diffuse * lightColor;
                fragColor = vec4(color, opacity);
            }
        "#;

        self.create_program(vertex_source, fragment_source, "basic")?;

        // Color vertex shader
        let color_vertex_source = r#"
            #version 300 es
            in vec3 position;
            in vec3 color;

            uniform mat4 modelMatrix;
            uniform mat4 viewMatrix;
            uniform mat4 projectionMatrix;

            out vec3 vColor;

            void main() {
                vColor = color;
                gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
            }
        "#;

        // Color fragment shader
        let color_fragment_source = r#"
            #version 300 es
            precision highp float;

            in vec3 vColor;

            uniform float opacity;

            out vec4 fragColor;

            void main() {
                fragColor = vec4(vColor, opacity);
            }
        "#;

        self.create_program(color_vertex_source, color_fragment_source, "color")?;

        Ok(())
    }

    /// Dispose of all shaders and programs
    pub fn dispose(&mut self) {
        let context = &self.context;

        // Delete all programs
        for (_, program) in self.programs.drain() {
            context.delete_program(Some(program.get_program()));
        }

        // Delete all shaders
        for (_, shader) in self.shaders.drain() {
            context.delete_shader(Some(&shader));
        }
    }
}

impl Drop for ShaderManager {
    fn drop(&mut self) {
        self.dispose();
    }
}
