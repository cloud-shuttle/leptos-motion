//! Error types for WebGL operations

use thiserror::Error;

/// Result type for WebGL operations
pub type Result<T> = std::result::Result<T, WebGLError>;

/// WebGL-specific error types
#[derive(Error, Debug)]
pub enum WebGLError {
    /// WebGL context creation failed
    #[error("Failed to create WebGL context: {0}")]
    ContextCreationFailed(String),

    /// WebGL context is lost
    #[error("WebGL context is lost")]
    ContextLost,

    /// Shader compilation failed
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),

    /// Program linking failed
    #[error("Program linking failed: {0}")]
    ProgramLinkingFailed(String),

    /// Invalid WebGL operation
    #[error("Invalid WebGL operation: {0}")]
    InvalidOperation(String),

    /// Buffer operation failed
    #[error("Buffer operation failed: {0}")]
    BufferError(String),

    /// Texture operation failed
    #[error("Texture operation failed: {0}")]
    TextureError(String),

    /// Framebuffer operation failed
    #[error("Framebuffer operation failed: {0}")]
    FramebufferError(String),

    /// Geometry operation failed
    #[error("Geometry operation failed: {0}")]
    GeometryError(String),

    /// Material operation failed
    #[error("Material operation failed: {0}")]
    MaterialError(String),

    /// Scene operation failed
    #[error("Scene operation failed: {0}")]
    SceneError(String),

    /// Camera operation failed
    #[error("Camera operation failed: {0}")]
    CameraError(String),

    /// Renderer operation failed
    #[error("Renderer operation failed: {0}")]
    RendererError(String),

    /// Lighting operation failed
    #[error("Lighting operation failed: {0}")]
    LightingError(String),

    /// Model loading operation failed
    #[error("Model loading operation failed: {0}")]
    ModelError(String),

    /// WASM binding error
    #[error("WASM binding error: {0}")]
    WasmError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serde serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// JavaScript value error
    #[error("JavaScript value error: {0:?}")]
    JsValue(wasm_bindgen::JsValue),
}

impl WebGLError {
    /// Create a context creation error
    pub fn context_creation(msg: &str) -> Self {
        Self::ContextCreationFailed(msg.to_string())
    }

    /// Create a shader compilation error
    pub fn shader_compilation(msg: &str) -> Self {
        Self::ShaderCompilationFailed(msg.to_string())
    }

    /// Create a program linking error
    pub fn program_linking(msg: &str) -> Self {
        Self::ProgramLinkingFailed(msg.to_string())
    }

    /// Create an invalid operation error
    pub fn invalid_operation(msg: &str) -> Self {
        Self::InvalidOperation(msg.to_string())
    }

    /// Create a buffer error
    pub fn buffer_error(msg: &str) -> Self {
        Self::BufferError(msg.to_string())
    }

    /// Create a texture error
    pub fn texture_error(msg: &str) -> Self {
        Self::TextureError(msg.to_string())
    }

    /// Create a framebuffer error
    pub fn framebuffer_error(msg: &str) -> Self {
        Self::FramebufferError(msg.to_string())
    }

    /// Create a geometry error
    pub fn geometry_error(msg: &str) -> Self {
        Self::GeometryError(msg.to_string())
    }

    /// Create a material error
    pub fn material_error(msg: &str) -> Self {
        Self::MaterialError(msg.to_string())
    }

    /// Create a scene error
    pub fn scene_error(msg: &str) -> Self {
        Self::SceneError(msg.to_string())
    }

    /// Create a camera error
    pub fn camera_error(msg: &str) -> Self {
        Self::CameraError(msg.to_string())
    }

    /// Create a renderer error
    pub fn renderer_error(msg: &str) -> Self {
        Self::RendererError(msg.to_string())
    }

    /// Create a lighting error
    pub fn lighting_error(msg: &str) -> Self {
        Self::LightingError(msg.to_string())
    }

    /// Create a model error
    pub fn model_error(msg: &str) -> Self {
        Self::ModelError(msg.to_string())
    }

    /// Create a WASM error
    pub fn wasm_error(msg: &str) -> Self {
        Self::WasmError(msg.to_string())
    }

    /// Create a JsValue error
    pub fn js_value_error(value: wasm_bindgen::JsValue) -> Self {
        Self::JsValue(value)
    }
}

impl From<wasm_bindgen::JsValue> for WebGLError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::JsValue(value)
    }
}
