//! WebGL renderer implementation

use super::*;
use crate::{Result, StudioError};

/// WebGL renderer
pub struct WebGLRenderer {
    /// WebGL context
    context: WebGLContext,
    /// Shader programs
    shader_programs: std::collections::HashMap<String, ShaderProgram>,
    /// GPU animations
    gpu_animations: std::collections::HashMap<u64, GPUAnimation>,
    /// Render state
    render_state: RenderState,
    /// Performance metrics
    performance_metrics: RendererPerformanceMetrics,
}

/// Render state
#[derive(Debug, Clone)]
struct RenderState {
    /// Current viewport
    viewport: (i32, i32, i32, i32),
    /// Current clear color
    clear_color: (f32, f32, f32, f32),
    /// Whether depth testing is enabled
    depth_test_enabled: bool,
    /// Whether blending is enabled
    blending_enabled: bool,
    /// Current shader program
    current_shader_program: Option<String>,
}

/// Renderer performance metrics
#[derive(Debug, Clone, Default)]
struct RendererPerformanceMetrics {
    /// Total frames rendered
    total_frames: u64,
    /// Average frame time in milliseconds
    avg_frame_time_ms: f64,
    /// Total render time in milliseconds
    total_render_time_ms: f64,
    /// Number of draw calls
    draw_calls: u64,
    /// Number of triangles rendered
    triangles_rendered: u64,
}

impl WebGLRenderer {
    /// Create a new WebGL renderer
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self> {
        let context = WebGLContext::new(canvas)?;
        let render_state = RenderState {
            viewport: (0, 0, context.canvas().width() as i32, context.canvas().height() as i32),
            clear_color: (0.0, 0.0, 0.0, 1.0),
            depth_test_enabled: false,
            blending_enabled: false,
            current_shader_program: None,
        };

        Ok(Self {
            context,
            shader_programs: std::collections::HashMap::new(),
            gpu_animations: std::collections::HashMap::new(),
            render_state,
            performance_metrics: RendererPerformanceMetrics::default(),
        })
    }

    /// Add a shader program
    pub fn add_shader_program(
        &mut self,
        name: String,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<()> {
        let program = ShaderProgram::new(self.context.context(), vertex_source, fragment_source)?;
        self.shader_programs.insert(name, program);
        Ok(())
    }

    /// Use a shader program
    pub fn use_shader_program(&mut self, name: &str) -> Result<()> {
        if let Some(program) = self.shader_programs.get(name) {
            program.use_program(self.context.context());
            self.render_state.current_shader_program = Some(name.to_string());
            Ok(())
        } else {
            Err(StudioError::NotFound)
        }
    }

    /// Add a GPU animation
    pub fn add_gpu_animation(&mut self, animation: GPUAnimation) {
        self.gpu_animations.insert(animation.id, animation);
    }

    /// Remove a GPU animation
    pub fn remove_gpu_animation(&mut self, id: u64) -> Option<GPUAnimation> {
        self.gpu_animations.remove(&id)
    }

    /// Update all GPU animations
    pub fn update_animations(&mut self, current_time: f64) -> Result<()> {
        for animation in self.gpu_animations.values_mut() {
            animation.update(current_time)?;
        }
        Ok(())
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();

        // Clear the screen
        self.context.clear(true, true, false);

        // Render all active animations
        for animation in self.gpu_animations.values() {
            if animation.is_running() {
                self.render_animation(animation)?;
            }
        }

        // Update performance metrics
        let frame_time = start_time.elapsed().as_secs_f64() * 1000.0;
        self.update_performance_metrics(frame_time);

        Ok(())
    }

    /// Render a single animation
    fn render_animation(&mut self, animation: &GPUAnimation) -> Result<()> {
        // Set up shader program if available
        if let Some(program) = animation.shader_program() {
            self.context.context().use_program(Some(program));
        }

        // Set up vertex buffer
        if let Some(vertex_buffer) = &animation.buffers.vertex_buffer {
            self.context.context().bind_buffer(
                web_sys::WebGlRenderingContext::ARRAY_BUFFER,
                Some(vertex_buffer),
            );
        }

        // Set up index buffer
        if let Some(index_buffer) = &animation.buffers.index_buffer {
            self.context.context().bind_buffer(
                web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(index_buffer),
            );
        }

        // Draw the animation
        self.context.context().draw_arrays(
            web_sys::WebGlRenderingContext::TRIANGLES,
            0,
            3, // Simplified - in reality this would be based on vertex count
        );

        Ok(())
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self, frame_time: f64) {
        self.performance_metrics.total_frames += 1;
        self.performance_metrics.total_render_time_ms += frame_time;
        self.performance_metrics.avg_frame_time_ms = 
            self.performance_metrics.total_render_time_ms / self.performance_metrics.total_frames as f64;
        self.performance_metrics.draw_calls += 1;
        self.performance_metrics.triangles_rendered += 3; // Simplified
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &RendererPerformanceMetrics {
        &self.performance_metrics
    }

    /// Get WebGL context
    pub fn context(&self) -> &WebGLContext {
        &self.context
    }

    /// Get mutable WebGL context
    pub fn context_mut(&mut self) -> &mut WebGLContext {
        &mut self.context
    }

    /// Get render state
    pub fn render_state(&self) -> &RenderState {
        &self.render_state
    }

    /// Set viewport
    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.context.set_viewport(x, y, width, height);
        self.render_state.viewport = (x, y, width, height);
    }

    /// Set clear color
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.context.set_clear_color(r, g, b, a);
        self.render_state.clear_color = (r, g, b, a);
    }

    /// Enable depth testing
    pub fn enable_depth_test(&mut self) {
        self.context.enable_depth_test();
        self.render_state.depth_test_enabled = true;
    }

    /// Disable depth testing
    pub fn disable_depth_test(&mut self) {
        self.context.disable_depth_test();
        self.render_state.depth_test_enabled = false;
    }

    /// Enable blending
    pub fn enable_blending(&mut self) {
        self.context.enable_blending();
        self.render_state.blending_enabled = true;
    }

    /// Disable blending
    pub fn disable_blending(&mut self) {
        self.context.disable_blending();
        self.render_state.blending_enabled = false;
    }
}
