//! Post-processing effects system for WebGL rendering

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlRenderbuffer, WebGlTexture};

/// Post-processing effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostProcessingEffect {
    /// Bloom effect for bright areas
    Bloom,
    /// Screen Space Ambient Occlusion
    SSAO,
    /// Tone mapping for HDR to LDR conversion
    ToneMapping,
    /// Gaussian blur
    GaussianBlur,
    /// Edge detection
    EdgeDetection,
    /// Color grading
    ColorGrading,
    /// Vignette effect
    Vignette,
    /// Chromatic aberration
    ChromaticAberration,
}

/// Post-processing configuration
#[derive(Debug, Clone)]
pub struct PostProcessingConfig {
    /// Effect type
    pub effect: PostProcessingEffect,
    /// Effect intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Effect parameters
    pub parameters: HashMap<String, f32>,
    /// Enabled flag
    pub enabled: bool,
}

impl PostProcessingConfig {
    /// Create a new post-processing configuration
    pub fn new(effect: PostProcessingEffect, intensity: f32) -> Self {
        Self {
            effect,
            intensity: intensity.max(0.0).min(1.0),
            parameters: HashMap::new(),
            enabled: true,
        }
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, name: &str, value: f32) {
        self.parameters.insert(name.to_string(), value);
    }

    /// Get a parameter value
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }

    /// Set intensity
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.max(0.0).min(1.0);
    }

    /// Enable the effect
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the effect
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Framebuffer for post-processing
#[derive(Debug)]
pub struct PostProcessingFramebuffer {
    /// Framebuffer object
    pub framebuffer: WebGlFramebuffer,
    /// Color texture
    pub color_texture: WebGlTexture,
    /// Depth renderbuffer
    pub depth_renderbuffer: WebGlRenderbuffer,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
}

impl PostProcessingFramebuffer {
    /// Create a new post-processing framebuffer
    pub fn new(context: &WebGl2RenderingContext, width: u32, height: u32) -> Result<Self> {
        // Create framebuffer
        let framebuffer = context
            .create_framebuffer()
            .ok_or_else(|| WebGLError::framebuffer_error("Failed to create framebuffer"))?;

        // Create color texture
        let color_texture = context
            .create_texture()
            .ok_or_else(|| WebGLError::texture_error("Failed to create color texture"))?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&color_texture));
        context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        )?;

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

        // Create depth renderbuffer
        let depth_renderbuffer = context
            .create_renderbuffer()
            .ok_or_else(|| WebGLError::framebuffer_error("Failed to create depth renderbuffer"))?;

        context.bind_renderbuffer(
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&depth_renderbuffer),
        );
        context.renderbuffer_storage(
            WebGl2RenderingContext::RENDERBUFFER,
            WebGl2RenderingContext::DEPTH_COMPONENT24,
            width as i32,
            height as i32,
        );

        // Attach to framebuffer
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&color_texture),
            0,
        );
        context.framebuffer_renderbuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&depth_renderbuffer),
        );

        // Check framebuffer status
        let status = context.check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER);
        if status != WebGl2RenderingContext::FRAMEBUFFER_COMPLETE {
            return Err(WebGLError::framebuffer_error("Framebuffer is not complete"));
        }

        // Unbind
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, None);

        Ok(Self {
            framebuffer,
            color_texture,
            depth_renderbuffer,
            width,
            height,
        })
    }

    /// Bind the framebuffer for rendering
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        context.viewport(0, 0, self.width as i32, self.height as i32);
    }

    /// Unbind the framebuffer
    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    }

    /// Get the color texture
    pub fn get_color_texture(&self) -> &WebGlTexture {
        &self.color_texture
    }

    /// Resize the framebuffer
    pub fn resize(
        &mut self,
        context: &WebGl2RenderingContext,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.width = width;
        self.height = height;

        // Update color texture
        context.bind_texture(
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&self.color_texture),
        );
        context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        )?;

        // Update depth renderbuffer
        context.bind_renderbuffer(
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&self.depth_renderbuffer),
        );
        context.renderbuffer_storage(
            WebGl2RenderingContext::RENDERBUFFER,
            WebGl2RenderingContext::DEPTH_COMPONENT24,
            width as i32,
            height as i32,
        );

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, None);

        Ok(())
    }
}

/// Bloom effect configuration
#[derive(Debug, Clone)]
pub struct BloomConfig {
    /// Bloom threshold
    pub threshold: f32,
    /// Bloom intensity
    pub intensity: f32,
    /// Number of blur passes
    pub blur_passes: u32,
    /// Blur radius
    pub blur_radius: f32,
}

impl Default for BloomConfig {
    fn default() -> Self {
        Self {
            threshold: 1.0,
            intensity: 0.5,
            blur_passes: 5,
            blur_radius: 1.0,
        }
    }
}

/// SSAO configuration
#[derive(Debug, Clone)]
pub struct SSAOConfig {
    /// Sample radius
    pub sample_radius: f32,
    /// Number of samples
    pub sample_count: u32,
    /// Bias to prevent self-occlusion
    pub bias: f32,
    /// Intensity
    pub intensity: f32,
}

impl Default for SSAOConfig {
    fn default() -> Self {
        Self {
            sample_radius: 0.5,
            sample_count: 16,
            bias: 0.025,
            intensity: 1.0,
        }
    }
}

/// Tone mapping configuration
#[derive(Debug, Clone)]
pub struct ToneMappingConfig {
    /// Exposure value
    pub exposure: f32,
    /// White point
    pub white_point: f32,
    /// Tone mapping operator
    pub operator: ToneMappingOperator,
}

/// Tone mapping operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToneMappingOperator {
    /// Linear tone mapping
    Linear,
    /// Reinhard tone mapping
    Reinhard,
    /// ACES tone mapping
    ACES,
    /// Uncharted 2 tone mapping
    Uncharted2,
}

impl Default for ToneMappingConfig {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            white_point: 11.2,
            operator: ToneMappingOperator::ACES,
        }
    }
}

/// Post-processing pipeline
pub struct PostProcessingPipeline {
    /// WebGL context
    context: WebGl2RenderingContext,
    /// Framebuffers for ping-pong rendering
    framebuffers: Vec<PostProcessingFramebuffer>,
    /// Effect configurations
    effects: Vec<PostProcessingConfig>,
    /// Current framebuffer index
    current_framebuffer: usize,
    /// Screen quad geometry
    screen_quad: Option<ScreenQuad>,
}

/// Screen quad for post-processing
#[derive(Debug)]
struct ScreenQuad {
    /// Vertex buffer
    vertex_buffer: web_sys::WebGlBuffer,
    /// Vertex array object
    vao: web_sys::WebGlVertexArrayObject,
}

impl ScreenQuad {
    /// Create a new screen quad
    fn new(context: &WebGl2RenderingContext) -> Result<Self> {
        // Create vertex data for a full-screen quad
        let vertices = vec![
            -1.0, -1.0, 0.0, 0.0, // Bottom-left
            1.0, -1.0, 1.0, 0.0, // Bottom-right
            1.0, 1.0, 1.0, 1.0, // Top-right
            -1.0, 1.0, 0.0, 1.0, // Top-left
        ];

        // Create vertex buffer
        let vertex_buffer = context
            .create_buffer()
            .ok_or_else(|| WebGLError::buffer_error("Failed to create vertex buffer"))?;

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Create vertex array object
        let vao = context
            .create_vertex_array()
            .ok_or_else(|| WebGLError::buffer_error("Failed to create VAO"))?;

        context.bind_vertex_array(Some(&vao));
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        // Set up vertex attributes
        context.enable_vertex_attrib_array(0);
        context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 16, 0);

        context.enable_vertex_attrib_array(1);
        context.vertex_attrib_pointer_with_i32(1, 2, WebGl2RenderingContext::FLOAT, false, 16, 8);

        context.bind_vertex_array(None);
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(Self { vertex_buffer, vao })
    }

    /// Draw the screen quad
    fn draw(&self, context: &WebGl2RenderingContext) {
        context.bind_vertex_array(Some(&self.vao));
        context.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, 4);
        context.bind_vertex_array(None);
    }
}

impl PostProcessingPipeline {
    /// Create a new post-processing pipeline
    pub fn new(context: WebGl2RenderingContext, width: u32, height: u32) -> Result<Self> {
        // Create framebuffers for ping-pong rendering
        let mut framebuffers = Vec::new();
        for _ in 0..2 {
            let framebuffer = PostProcessingFramebuffer::new(&context, width, height)?;
            framebuffers.push(framebuffer);
        }

        // Create screen quad
        let screen_quad = Some(ScreenQuad::new(&context)?);

        Ok(Self {
            context,
            framebuffers,
            effects: Vec::new(),
            current_framebuffer: 0,
            screen_quad,
        })
    }

    /// Add an effect to the pipeline
    pub fn add_effect(&mut self, config: PostProcessingConfig) {
        self.effects.push(config);
    }

    /// Remove an effect from the pipeline
    pub fn remove_effect(&mut self, index: usize) -> Option<PostProcessingConfig> {
        if index < self.effects.len() {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    /// Get an effect configuration
    pub fn get_effect(&self, index: usize) -> Option<&PostProcessingConfig> {
        self.effects.get(index)
    }

    /// Get a mutable effect configuration
    pub fn get_effect_mut(&mut self, index: usize) -> Option<&mut PostProcessingConfig> {
        self.effects.get_mut(index)
    }

    /// Get all effects
    pub fn get_effects(&self) -> &[PostProcessingConfig] {
        &self.effects
    }

    /// Clear all effects
    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }

    /// Resize the pipeline
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        for framebuffer in &mut self.framebuffers {
            framebuffer.resize(&self.context, width, height)?;
        }
        Ok(())
    }

    /// Begin rendering to the pipeline
    pub fn begin(&mut self) {
        self.current_framebuffer = 0;
        self.framebuffers[self.current_framebuffer].bind(&self.context);
    }

    /// End rendering and apply effects
    pub fn end(&mut self) -> Result<()> {
        if self.effects.is_empty() {
            return Ok(());
        }

        // Apply effects in sequence
        for (i, effect) in self.effects.iter().enumerate() {
            if !effect.enabled {
                continue;
            }

            // Ping-pong between framebuffers
            let input_framebuffer = &self.framebuffers[self.current_framebuffer];
            self.current_framebuffer = 1 - self.current_framebuffer;
            let output_framebuffer = &self.framebuffers[self.current_framebuffer];

            // Bind output framebuffer
            output_framebuffer.bind(&self.context);

            // Apply effect (this would use shaders in a real implementation)
            self.apply_effect(effect, input_framebuffer.get_color_texture())?;
        }

        // Unbind framebuffer
        self.framebuffers[self.current_framebuffer].unbind(&self.context);

        Ok(())
    }

    /// Apply a specific effect
    fn apply_effect(
        &self,
        effect: &PostProcessingConfig,
        input_texture: &WebGlTexture,
    ) -> Result<()> {
        // This is a simplified implementation
        // In a real implementation, this would:
        // 1. Bind the appropriate shader program
        // 2. Set uniforms based on effect parameters
        // 3. Bind the input texture
        // 4. Draw the screen quad

        match effect.effect {
            PostProcessingEffect::Bloom => {
                // Apply bloom effect
                self.apply_bloom_effect(effect, input_texture)?;
            }
            PostProcessingEffect::SSAO => {
                // Apply SSAO effect
                self.apply_ssao_effect(effect, input_texture)?;
            }
            PostProcessingEffect::ToneMapping => {
                // Apply tone mapping
                self.apply_tone_mapping_effect(effect, input_texture)?;
            }
            PostProcessingEffect::GaussianBlur => {
                // Apply Gaussian blur
                self.apply_gaussian_blur_effect(effect, input_texture)?;
            }
            PostProcessingEffect::EdgeDetection => {
                // Apply edge detection
                self.apply_edge_detection_effect(effect, input_texture)?;
            }
            PostProcessingEffect::ColorGrading => {
                // Apply color grading
                self.apply_color_grading_effect(effect, input_texture)?;
            }
            PostProcessingEffect::Vignette => {
                // Apply vignette effect
                self.apply_vignette_effect(effect, input_texture)?;
            }
            PostProcessingEffect::ChromaticAberration => {
                // Apply chromatic aberration
                self.apply_chromatic_aberration_effect(effect, input_texture)?;
            }
        }

        Ok(())
    }

    /// Apply bloom effect
    fn apply_bloom_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified bloom implementation
        // In a real implementation, this would:
        // 1. Extract bright areas above threshold
        // 2. Apply Gaussian blur multiple times
        // 3. Combine with original image

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply SSAO effect
    fn apply_ssao_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified SSAO implementation
        // In a real implementation, this would:
        // 1. Sample depth buffer
        // 2. Generate random samples in hemisphere
        // 3. Calculate occlusion for each sample
        // 4. Apply blur to reduce noise

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply tone mapping effect
    fn apply_tone_mapping_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified tone mapping implementation
        // In a real implementation, this would:
        // 1. Apply exposure adjustment
        // 2. Apply tone mapping operator
        // 3. Apply white point adjustment

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply Gaussian blur effect
    fn apply_gaussian_blur_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified Gaussian blur implementation
        // In a real implementation, this would:
        // 1. Apply horizontal blur pass
        // 2. Apply vertical blur pass
        // 3. Repeat for multiple passes

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply edge detection effect
    fn apply_edge_detection_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified edge detection implementation
        // In a real implementation, this would:
        // 1. Apply Sobel or similar edge detection kernel
        // 2. Apply threshold to create binary edge map
        // 3. Optionally apply edge enhancement

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply color grading effect
    fn apply_color_grading_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified color grading implementation
        // In a real implementation, this would:
        // 1. Apply color curves
        // 2. Apply color balance
        // 3. Apply saturation/contrast adjustments

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply vignette effect
    fn apply_vignette_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified vignette implementation
        // In a real implementation, this would:
        // 1. Calculate distance from center
        // 2. Apply vignette curve
        // 3. Blend with original image

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Apply chromatic aberration effect
    fn apply_chromatic_aberration_effect(
        &self,
        effect: &PostProcessingConfig,
        _input_texture: &WebGlTexture,
    ) -> Result<()> {
        // Simplified chromatic aberration implementation
        // In a real implementation, this would:
        // 1. Offset red and blue channels
        // 2. Keep green channel centered
        // 3. Blend channels together

        if let Some(screen_quad) = &self.screen_quad {
            screen_quad.draw(&self.context);
        }

        Ok(())
    }

    /// Get the final output texture
    pub fn get_output_texture(&self) -> &WebGlTexture {
        &self.framebuffers[self.current_framebuffer].color_texture
    }

    /// Get the number of effects
    pub fn get_effect_count(&self) -> usize {
        self.effects.len()
    }

    /// Check if pipeline is empty
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}
