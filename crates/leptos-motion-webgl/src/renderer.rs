//! WebGL Renderer implementation

use crate::error::{Result, WebGLError};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::shader::ShaderManager;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGl2RenderingContext, HtmlCanvasElement, Performance,
};
use std::rc::Rc;
use std::cell::RefCell;

/// WebGL renderer configuration
#[derive(Debug, Clone)]
pub struct RendererConfig {
    /// Enable antialiasing
    pub antialias: bool,
    /// Enable depth testing
    pub depth_test: bool,
    /// Enable alpha blending
    pub alpha: bool,
    /// Enable stencil testing
    pub stencil: bool,
    /// Enable premultiplied alpha
    pub premultiplied_alpha: bool,
    /// Preserve drawing buffer
    pub preserve_drawing_buffer: bool,
    /// Power preference for GPU
    pub power_preference: PowerPreference,
    /// Fail if major performance caveat
    pub fail_if_major_performance_caveat: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            antialias: true,
            depth_test: true,
            alpha: true,
            stencil: false,
            premultiplied_alpha: false,
            preserve_drawing_buffer: false,
            power_preference: PowerPreference::Default,
            fail_if_major_performance_caveat: false,
        }
    }
}

/// GPU power preference
#[derive(Debug, Clone, Copy)]
pub enum PowerPreference {
    Default,
    HighPerformance,
    LowPower,
}

impl PowerPreference {
    fn to_webgl_string(&self) -> &'static str {
        match self {
            PowerPreference::Default => "default",
            PowerPreference::HighPerformance => "high-performance",
            PowerPreference::LowPower => "low-power",
        }
    }
}

/// WebGL renderer statistics
#[derive(Debug, Default)]
pub struct RendererStats {
    /// Number of draw calls
    pub draw_calls: u32,
    /// Number of triangles rendered
    pub triangles: u32,
    /// Number of vertices processed
    pub vertices: u32,
    /// Number of textures used
    pub textures: u32,
    /// Frame time in milliseconds
    pub frame_time: f64,
    /// FPS
    pub fps: f64,
}

/// Main WebGL renderer
pub struct WebGLRenderer {
    /// WebGL2 rendering context
    context: Rc<WebGl2RenderingContext>,
    /// Canvas element
    canvas: Rc<HtmlCanvasElement>,
    /// Shader manager
    shader_manager: Rc<RefCell<ShaderManager>>,
    /// Renderer configuration
    config: RendererConfig,
    /// Renderer statistics
    stats: Rc<RefCell<RendererStats>>,
    /// Animation frame ID
    animation_frame_id: Option<i32>,
    /// Performance monitoring
    performance: Performance,
    /// Last frame time
    last_frame_time: f64,
    /// Frame count for FPS calculation
    frame_count: u32,
    /// FPS calculation start time
    fps_start_time: f64,
}

impl WebGLRenderer {
    /// Create a new WebGL renderer
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self> {
        Self::new_with_config(canvas, RendererConfig::default())
    }

    /// Create a new WebGL renderer with custom configuration
    pub fn new_with_config(canvas: HtmlCanvasElement, config: RendererConfig) -> Result<Self> {
        let context = Self::create_webgl_context(&canvas, &config)?;
        let shader_manager = Rc::new(RefCell::new(ShaderManager::new(&context)?));
        let stats = Rc::new(RefCell::new(RendererStats::default()));
        
        let performance = web_sys::window()
            .ok_or_else(|| WebGLError::wasm_error("Failed to get window"))?
            .performance()
            .ok_or_else(|| WebGLError::wasm_error("Failed to get performance"))?;

        let mut renderer = Self {
            context: Rc::new(context),
            canvas: Rc::new(canvas),
            shader_manager,
            config,
            stats,
            animation_frame_id: None,
            performance,
            last_frame_time: 0.0,
            frame_count: 0,
            fps_start_time: 0.0,
        };

        renderer.initialize()?;
        Ok(renderer)
    }

    /// Create WebGL context
    fn create_webgl_context(
        canvas: &HtmlCanvasElement,
        config: &RendererConfig,
    ) -> Result<WebGl2RenderingContext> {
        let context_options = js_sys::Object::new();
        
        js_sys::Reflect::set(&context_options, &"antialias".into(), &config.antialias.into())?;
        js_sys::Reflect::set(&context_options, &"depth".into(), &config.depth_test.into())?;
        js_sys::Reflect::set(&context_options, &"alpha".into(), &config.alpha.into())?;
        js_sys::Reflect::set(&context_options, &"stencil".into(), &config.stencil.into())?;
        js_sys::Reflect::set(&context_options, &"premultipliedAlpha".into(), &config.premultiplied_alpha.into())?;
        js_sys::Reflect::set(&context_options, &"preserveDrawingBuffer".into(), &config.preserve_drawing_buffer.into())?;
        js_sys::Reflect::set(&context_options, &"powerPreference".into(), &config.power_preference.to_webgl_string().into())?;
        js_sys::Reflect::set(&context_options, &"failIfMajorPerformanceCaveat".into(), &config.fail_if_major_performance_caveat.into())?;

        let context = canvas
            .get_context_with_context_options("webgl2", &context_options)
            .map_err(|_| WebGLError::context_creation("Failed to get WebGL2 context"))?
            .ok_or_else(|| WebGLError::context_creation("WebGL2 context is null"))?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| WebGLError::context_creation("Failed to cast to WebGL2RenderingContext"))?;

        Ok(context)
    }

    /// Initialize the renderer
    fn initialize(&mut self) -> Result<()> {
        let context = &self.context;
        
        // Set viewport
        let width = self.canvas.width() as i32;
        let height = self.canvas.height() as i32;
        context.viewport(0, 0, width, height);

        // Enable depth testing
        if self.config.depth_test {
            context.enable(WebGl2RenderingContext::DEPTH_TEST);
            context.depth_func(WebGl2RenderingContext::LEQUAL);
        }

        // Enable alpha blending
        if self.config.alpha {
            context.enable(WebGl2RenderingContext::BLEND);
            context.blend_func(
                WebGl2RenderingContext::SRC_ALPHA,
                WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            );
        }

        // Enable culling
        context.enable(WebGl2RenderingContext::CULL_FACE);
        context.cull_face(WebGl2RenderingContext::BACK);
        context.front_face(WebGl2RenderingContext::CCW);

        // Set clear color
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear_depth(1.0);

        // Initialize performance monitoring
        self.fps_start_time = self.performance.now();

        Ok(())
    }

    /// Render a scene with a camera
    pub fn render(&mut self, scene: &Scene, camera: &mut dyn Camera) -> Result<()> {
        let start_time = self.performance.now();
        
        // Clear the canvas
        self.clear();

        // Update camera matrices
        camera.update_matrix();

        // Render scene
        self.render_scene(scene, camera)?;

        // Update statistics
        self.update_stats(start_time);

        Ok(())
    }

    /// Clear the canvas
    pub fn clear(&self) {
        let context = &self.context;
        let mut clear_bits = WebGl2RenderingContext::COLOR_BUFFER_BIT;
        
        if self.config.depth_test {
            clear_bits |= WebGl2RenderingContext::DEPTH_BUFFER_BIT;
        }
        
        if self.config.stencil {
            clear_bits |= WebGl2RenderingContext::STENCIL_BUFFER_BIT;
        }
        
        context.clear(clear_bits);
    }

    /// Render a scene
    fn render_scene(&mut self, _scene: &Scene, _camera: &dyn Camera) -> Result<()> {
        // TODO: Implement scene rendering
        // This will be implemented when we have the scene graph system
        Ok(())
    }

    /// Update renderer statistics
    fn update_stats(&mut self, start_time: f64) {
        let frame_time = self.performance.now() - start_time;
        self.last_frame_time = frame_time;
        self.frame_count += 1;

        // Update FPS every 60 frames
        if self.frame_count % 60 == 0 {
            let current_time = self.performance.now();
            let elapsed = current_time - self.fps_start_time;
            let fps = (self.frame_count as f64 * 1000.0) / elapsed;
            
            let mut stats = self.stats.borrow_mut();
            stats.fps = fps;
            stats.frame_time = frame_time;
            
            self.fps_start_time = current_time;
            self.frame_count = 0;
        }
    }

    /// Set the size of the renderer
    pub fn set_size(&mut self, width: u32, height: u32) -> Result<()> {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        
        let width = width as i32;
        let height = height as i32;
        self.context.viewport(0, 0, width, height);
        
        Ok(())
    }

    /// Get the current size
    pub fn get_size(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    /// Set the clear color
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
    }

    /// Get the WebGL context
    pub fn get_context(&self) -> &WebGl2RenderingContext {
        &self.context
    }

    /// Get the canvas element
    pub fn get_canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    /// Get renderer statistics
    pub fn get_stats(&self) -> RendererStats {
        RendererStats {
            draw_calls: self.stats.borrow().draw_calls,
            triangles: self.stats.borrow().triangles,
            vertices: self.stats.borrow().vertices,
            textures: self.stats.borrow().textures,
            frame_time: self.stats.borrow().frame_time,
            fps: self.stats.borrow().fps,
        }
    }

    /// Get the shader manager
    pub fn get_shader_manager(&self) -> Rc<RefCell<ShaderManager>> {
        self.shader_manager.clone()
    }

    /// Dispose of the renderer
    pub fn dispose(&mut self) {
        if let Some(frame_id) = self.animation_frame_id {
            web_sys::window()
                .unwrap()
                .cancel_animation_frame(frame_id)
                .unwrap();
            self.animation_frame_id = None;
        }
    }
}

impl Drop for WebGLRenderer {
    fn drop(&mut self) {
        self.dispose();
    }
}
