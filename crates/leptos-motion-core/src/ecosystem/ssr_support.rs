//! Server-side rendering (SSR) support

use crate::{Result, AnimationError};

/// SSR support for motion components
pub struct SSRSupport {
    /// SSR configuration
    config: SSRConfig,
    /// SSR renderer
    renderer: SSRRenderer,
    /// Hydration support
    hydration: HydrationSupport,
}

/// SSR configuration
#[derive(Debug, Clone)]
pub struct SSRConfig {
    /// Whether to enable SSR
    pub enable_ssr: bool,
    /// Whether to enable hydration
    pub enable_hydration: bool,
    /// Whether to enable streaming
    pub enable_streaming: bool,
    /// Whether to enable caching
    pub enable_caching: bool,
}

/// SSR renderer
#[derive(Debug, Clone)]
pub struct SSRRenderer {
    /// Renderer type
    pub renderer_type: SSRRendererType,
    /// Renderer configuration
    pub configuration: std::collections::HashMap<String, String>,
}

/// Hydration support
#[derive(Debug, Clone)]
pub struct HydrationSupport {
    /// Whether hydration is enabled
    pub enabled: bool,
    /// Hydration configuration
    pub configuration: std::collections::HashMap<String, String>,
}

/// SSR renderer type
#[derive(Debug, Clone, PartialEq)]
pub enum SSRRendererType {
    /// Leptos SSR renderer
    Leptos,
    /// Custom SSR renderer
    Custom(String),
}

impl SSRSupport {
    /// Create a new SSR support
    pub fn new(config: SSRConfig) -> Self {
        Self {
            config,
            renderer: SSRRenderer::new(SSRRendererType::Leptos),
            hydration: HydrationSupport::new(),
        }
    }

    /// Get configuration
    pub fn config(&self) -> &SSRConfig {
        &self.config
    }

    /// Get renderer
    pub fn renderer(&self) -> &SSRRenderer {
        &self.renderer
    }

    /// Get hydration support
    pub fn hydration(&self) -> &HydrationSupport {
        &self.hydration
    }
}

impl SSRRenderer {
    /// Create a new SSR renderer
    pub fn new(renderer_type: SSRRendererType) -> Self {
        Self {
            renderer_type,
            configuration: std::collections::HashMap::new(),
        }
    }
}

impl HydrationSupport {
    /// Create a new hydration support
    pub fn new() -> Self {
        Self {
            enabled: true,
            configuration: std::collections::HashMap::new(),
        }
    }
}
