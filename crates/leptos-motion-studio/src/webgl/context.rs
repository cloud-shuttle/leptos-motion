//! WebGL context management

use crate::{Result, StudioError};

/// WebGL context wrapper
pub struct WebGLContext {
    /// WebGL context
    context: web_sys::WebGlRenderingContext,
    /// Canvas element
    canvas: web_sys::HtmlCanvasElement,
    /// Context capabilities
    capabilities: WebGLCapabilities,
    /// Context state
    state: WebGLState,
}

/// WebGL context state
#[derive(Debug, Clone)]
struct WebGLState {
    /// Current viewport
    viewport: (i32, i32, i32, i32),
    /// Current clear color
    clear_color: (f32, f32, f32, f32),
    /// Current clear depth
    clear_depth: f32,
    /// Whether depth testing is enabled
    depth_test_enabled: bool,
    /// Whether blending is enabled
    blending_enabled: bool,
}

impl WebGLContext {
    /// Create a new WebGL context
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self> {
        let context = canvas
            .get_context("webgl")
            .map_err(|_| StudioError::InvalidState("Failed to get WebGL context".to_string()))?
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .map_err(|_| StudioError::InvalidState("Invalid WebGL context".to_string()))?;

        let capabilities = WebGLCapabilities::detect(&context)?;
        let state = WebGLState {
            viewport: (0, 0, canvas.width() as i32, canvas.height() as i32),
            clear_color: (0.0, 0.0, 0.0, 1.0),
            clear_depth: 1.0,
            depth_test_enabled: false,
            blending_enabled: false,
        };

        Ok(Self {
            context,
            canvas,
            capabilities,
            state,
        })
    }

    /// Get the WebGL context
    pub fn context(&self) -> &web_sys::WebGlRenderingContext {
        &self.context
    }

    /// Get the canvas element
    pub fn canvas(&self) -> &web_sys::HtmlCanvasElement {
        &self.canvas
    }

    /// Get context capabilities
    pub fn capabilities(&self) -> &WebGLCapabilities {
        &self.capabilities
    }

    /// Set viewport
    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.context.viewport(x, y, width, height);
        self.state.viewport = (x, y, width, height);
    }

    /// Set clear color
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.state.clear_color = (r, g, b, a);
    }

    /// Set clear depth
    pub fn set_clear_depth(&mut self, depth: f32) {
        self.context.clear_depth(depth);
        self.state.clear_depth = depth;
    }

    /// Enable depth testing
    pub fn enable_depth_test(&mut self) {
        self.context.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        self.state.depth_test_enabled = true;
    }

    /// Disable depth testing
    pub fn disable_depth_test(&mut self) {
        self.context.disable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        self.state.depth_test_enabled = false;
    }

    /// Enable blending
    pub fn enable_blending(&mut self) {
        self.context.enable(web_sys::WebGlRenderingContext::BLEND);
        self.state.blending_enabled = true;
    }

    /// Disable blending
    pub fn disable_blending(&mut self) {
        self.context.disable(web_sys::WebGlRenderingContext::BLEND);
        self.state.blending_enabled = false;
    }

    /// Clear the context
    pub fn clear(&self, color: bool, depth: bool, stencil: bool) {
        let mut clear_bits = 0;
        if color {
            clear_bits |= web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT;
        }
        if depth {
            clear_bits |= web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT;
        }
        if stencil {
            clear_bits |= web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT;
        }
        self.context.clear(clear_bits);
    }

    /// Get current state
    pub fn state(&self) -> &WebGLState {
        &self.state
    }
}
