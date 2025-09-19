//! Framework adapters for cross-framework compatibility

use crate::{Result, AnimationError};

/// Cross-framework adapter for motion components
pub struct CrossFrameworkAdapter {
    /// Supported frameworks
    supported_frameworks: std::collections::HashSet<FrameworkType>,
    /// Framework adapters
    adapters: std::collections::HashMap<FrameworkType, FrameworkAdapter>,
    /// Unified API
    unified_api: UnifiedAPI,
}

/// Framework adapter for specific frameworks
pub struct FrameworkAdapter {
    /// Framework type
    framework_type: FrameworkType,
    /// Adapter configuration
    config: FrameworkAdapterConfig,
    /// Framework components
    components: std::collections::HashMap<String, FrameworkComponent>,
}

/// Framework component wrapper
pub struct FrameworkComponent {
    /// Component name
    pub name: String,
    /// Original component
    pub original_component: String,
    /// Component props
    pub props: std::collections::HashMap<String, String>,
    /// Component children
    pub children: Option<String>,
}

/// Unified API for cross-framework compatibility
pub struct UnifiedAPI {
    /// API configuration
    config: UnifiedAPIConfig,
    /// API methods
    methods: std::collections::HashMap<String, APIMethod>,
}

/// Framework type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FrameworkType {
    /// Leptos framework
    Leptos,
    /// React framework
    React,
    /// Vue framework
    Vue,
    /// Angular framework
    Angular,
    /// Svelte framework
    Svelte,
    /// Solid framework
    Solid,
    /// Other framework
    Other(String),
}

/// Framework adapter configuration
#[derive(Debug, Clone)]
pub struct FrameworkAdapterConfig {
    /// Whether to enable automatic prop conversion
    pub auto_convert_props: bool,
    /// Whether to enable automatic event handling
    pub auto_handle_events: bool,
    /// Whether to enable automatic styling
    pub auto_handle_styling: bool,
    /// Whether to enable development mode
    pub enable_dev_mode: bool,
}

/// Unified API configuration
#[derive(Debug, Clone)]
pub struct UnifiedAPIConfig {
    /// Whether to enable automatic framework detection
    pub auto_detect_framework: bool,
    /// Whether to enable fallback to default framework
    pub enable_fallback: bool,
    /// Default framework
    pub default_framework: FrameworkType,
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

impl CrossFrameworkAdapter {
    /// Create a new cross-framework adapter
    pub fn new() -> Self {
        Self {
            supported_frameworks: std::collections::HashSet::new(),
            adapters: std::collections::HashMap::new(),
            unified_api: UnifiedAPI::new(),
        }
    }

    /// Add framework support
    pub fn add_framework_support(&mut self, framework_type: FrameworkType) -> Result<()> {
        if self.supported_frameworks.contains(&framework_type) {
            return Err(AnimationError::InvalidValue(
                format!("Framework {:?} is already supported", framework_type)
            ));
        }

        let adapter = FrameworkAdapter::new(framework_type.clone());
        self.adapters.insert(framework_type.clone(), adapter);
        self.supported_frameworks.insert(framework_type);
        Ok(())
    }

    /// Remove framework support
    pub fn remove_framework_support(&mut self, framework_type: &FrameworkType) -> Option<FrameworkAdapter> {
        self.supported_frameworks.remove(framework_type);
        self.adapters.remove(framework_type)
    }

    /// Get framework adapter
    pub fn get_adapter(&self, framework_type: &FrameworkType) -> Option<&FrameworkAdapter> {
        self.adapters.get(framework_type)
    }

    /// Get mutable framework adapter
    pub fn get_adapter_mut(&mut self, framework_type: &FrameworkType) -> Option<&mut FrameworkAdapter> {
        self.adapters.get_mut(framework_type)
    }

    /// Get supported frameworks
    pub fn supported_frameworks(&self) -> &std::collections::HashSet<FrameworkType> {
        &self.supported_frameworks
    }

    /// Check if framework is supported
    pub fn is_framework_supported(&self, framework_type: &FrameworkType) -> bool {
        self.supported_frameworks.contains(framework_type)
    }

    /// Get unified API
    pub fn unified_api(&self) -> &UnifiedAPI {
        &self.unified_api
    }

    /// Get mutable unified API
    pub fn unified_api_mut(&mut self) -> &mut UnifiedAPI {
        &mut self.unified_api
    }

    /// Initialize all adapters
    pub fn initialize(&mut self) -> Result<()> {
        for adapter in self.adapters.values_mut() {
            adapter.initialize()?;
        }
        self.unified_api.initialize()?;
        Ok(())
    }

    /// Get framework count
    pub fn framework_count(&self) -> usize {
        self.supported_frameworks.len()
    }
}

impl FrameworkAdapter {
    /// Create a new framework adapter
    pub fn new(framework_type: FrameworkType) -> Self {
        Self {
            framework_type,
            config: FrameworkAdapterConfig::default(),
            components: std::collections::HashMap::new(),
        }
    }

    /// Get framework type
    pub fn framework_type(&self) -> &FrameworkType {
        &self.framework_type
    }

    /// Get configuration
    pub fn config(&self) -> &FrameworkAdapterConfig {
        &self.config
    }

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut FrameworkAdapterConfig {
        &mut self.config
    }

    /// Register a component
    pub fn register_component(&mut self, component: FrameworkComponent) -> Result<()> {
        if self.components.contains_key(&component.name) {
            return Err(AnimationError::InvalidValue(
                format!("Component '{}' is already registered", component.name)
            ));
        }

        self.components.insert(component.name.clone(), component);
        Ok(())
    }

    /// Get a component
    pub fn get_component(&self, name: &str) -> Option<&FrameworkComponent> {
        self.components.get(name)
    }

    /// Get all components
    pub fn components(&self) -> &std::collections::HashMap<String, FrameworkComponent> {
        &self.components
    }

    /// Initialize the adapter
    pub fn initialize(&mut self) -> Result<()> {
        // Initialize framework-specific components
        self.register_default_components()?;
        Ok(())
    }

    /// Register default components
    fn register_default_components(&mut self) -> Result<()> {
        let default_components = vec![
            FrameworkComponent {
                name: "MotionDiv".to_string(),
                original_component: "div".to_string(),
                props: std::collections::HashMap::new(),
                children: None,
            },
            FrameworkComponent {
                name: "MotionSpan".to_string(),
                original_component: "span".to_string(),
                props: std::collections::HashMap::new(),
                children: None,
            },
        ];

        for component in default_components {
            self.register_component(component)?;
        }

        Ok(())
    }

    /// Get component count
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
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

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut UnifiedAPIConfig {
        &mut self.config
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

    /// Initialize the unified API
    pub fn initialize(&mut self) -> Result<()> {
        // Register default API methods
        self.register_default_methods();
        Ok(())
    }

    /// Register default methods
    fn register_default_methods(&mut self) {
        let default_methods = vec![
            APIMethod {
                name: "createMotionComponent".to_string(),
                method_type: APIMethodType::CreateComponent,
                implementation: "createMotionComponent".to_string(),
            },
            APIMethod {
                name: "animate".to_string(),
                method_type: APIMethodType::Animate,
                implementation: "animate".to_string(),
            },
            APIMethod {
                name: "handleEvent".to_string(),
                method_type: APIMethodType::HandleEvent,
                implementation: "handleEvent".to_string(),
            },
        ];

        for method in default_methods {
            self.register_method(method);
        }
    }

    /// Get method count
    pub fn method_count(&self) -> usize {
        self.methods.len()
    }
}

impl Default for FrameworkAdapterConfig {
    fn default() -> Self {
        Self {
            auto_convert_props: true,
            auto_handle_events: true,
            auto_handle_styling: true,
            enable_dev_mode: false,
        }
    }
}

impl Default for UnifiedAPIConfig {
    fn default() -> Self {
        Self {
            auto_detect_framework: true,
            enable_fallback: true,
            default_framework: FrameworkType::Leptos,
            enable_performance_monitoring: false,
        }
    }
}
