//! Unified API for cross-framework compatibility

use crate::{Result, AnimationError};

/// Unified API for motion components
pub struct UnifiedAPI {
    /// API configuration
    config: UnifiedAPIConfig,
    /// API methods
    methods: std::collections::HashMap<String, APIMethod>,
}

/// Unified API configuration
#[derive(Debug, Clone)]
pub struct UnifiedAPIConfig {
    /// Whether to enable automatic framework detection
    pub auto_detect_framework: bool,
    /// Whether to enable fallback to default framework
    pub enable_fallback: bool,
    /// Default framework
    pub default_framework: String,
    /// Whether to enable performance monitoring
    pub enable_performance_monitoring: bool,
}

/// API method
#[derive(Debug, Clone)]
pub struct APIMethod {
    /// Method name
    pub name: String,
    /// Method type
    pub method_type: APIMethodType,
    /// Method implementation
    pub implementation: String,
}

/// API method type
#[derive(Debug, Clone, PartialEq)]
pub enum APIMethodType {
    /// Component creation method
    CreateComponent,
    /// Animation method
    Animate,
    /// Event handling method
    HandleEvent,
    /// Styling method
    Style,
    /// Other method
    Other(String),
}

impl UnifiedAPI {
    /// Create a new unified API
    pub fn new() -> Self {
        Self {
            config: UnifiedAPIConfig::default(),
            methods: std::collections::HashMap::new(),
        }
    }

    /// Get configuration
    pub fn config(&self) -> &UnifiedAPIConfig {
        &self.config
    }

    /// Register an API method
    pub fn register_method(&mut self, method: APIMethod) {
        self.methods.insert(method.name.clone(), method);
    }

    /// Get an API method
    pub fn get_method(&self, name: &str) -> Option<&APIMethod> {
        self.methods.get(name)
    }

    /// Get all methods
    pub fn methods(&self) -> &std::collections::HashMap<String, APIMethod> {
        &self.methods
    }
}

impl Default for UnifiedAPIConfig {
    fn default() -> Self {
        Self {
            auto_detect_framework: true,
            enable_fallback: true,
            default_framework: "leptos".to_string(),
            enable_performance_monitoring: false,
        }
    }
}
