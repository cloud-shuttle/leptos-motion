//! Leptos framework integration

use crate::{Result, AnimationError};

/// Leptos integration for motion components
pub struct LeptosIntegration {
    /// Integration configuration
    config: LeptosIntegrationConfig,
    /// Motion components registry
    components: std::collections::HashMap<String, MotionComponent>,
    /// Resource integration
    resource_integration: ResourceIntegration,
}

/// Leptos integration configuration
#[derive(Debug, Clone)]
pub struct LeptosIntegrationConfig {
    /// Whether to enable automatic component registration
    pub auto_register_components: bool,
    /// Whether to enable SSR support
    pub enable_ssr: bool,
    /// Whether to enable hydration support
    pub enable_hydration: bool,
    /// Whether to enable development mode features
    pub enable_dev_mode: bool,
}

/// Motion component for Leptos
pub struct MotionComponent {
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: MotionComponentType,
    /// Component props
    pub props: std::collections::HashMap<String, String>,
}

/// Motion component type
#[derive(Debug, Clone, PartialEq)]
pub enum MotionComponentType {
    /// Basic motion component
    Basic,
    /// Advanced motion component
    Advanced,
    /// Custom motion component
    Custom(String),
}

/// Resource integration for Leptos
pub struct ResourceIntegration {
    /// Resource manager
    resource_manager: ResourceManager,
    /// Asset loader
    asset_loader: AssetLoader,
}

/// Resource manager
#[derive(Debug, Clone)]
pub struct ResourceManager {
    /// Registered resources
    resources: std::collections::HashMap<String, ResourceInfo>,
}

/// Asset loader
#[derive(Debug, Clone)]
pub struct AssetLoader {
    /// Loaded assets
    assets: std::collections::HashMap<String, AssetInfo>,
}

/// Resource information
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    /// Resource name
    pub name: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource path
    pub path: String,
    /// Resource size
    pub size: usize,
}

/// Asset information
#[derive(Debug, Clone)]
pub struct AssetInfo {
    /// Asset name
    pub name: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Asset data
    pub data: Vec<u8>,
    /// Asset metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Resource type
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    /// CSS resource
    Css,
    /// JavaScript resource
    JavaScript,
    /// Image resource
    Image,
    /// Font resource
    Font,
    /// Other resource
    Other(String),
}

/// Asset type
#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    /// Image asset
    Image,
    /// Audio asset
    Audio,
    /// Video asset
    Video,
    /// Font asset
    Font,
    /// Other asset
    Other(String),
}

impl LeptosIntegration {
    /// Create a new Leptos integration
    pub fn new(config: LeptosIntegrationConfig) -> Self {
        Self {
            config,
            components: std::collections::HashMap::new(),
            resource_integration: ResourceIntegration::new(),
        }
    }

    /// Create a new Leptos integration with default configuration
    pub fn new_default() -> Self {
        Self::new(LeptosIntegrationConfig::default())
    }

    /// Register a motion component
    pub fn register_component(&mut self, component: MotionComponent) -> Result<()> {
        if self.components.contains_key(&component.name) {
            return Err(AnimationError::InvalidValue(
                format!("Component '{}' is already registered", component.name)
            ));
        }

        self.components.insert(component.name.clone(), component);
        Ok(())
    }

    /// Unregister a motion component
    pub fn unregister_component(&mut self, name: &str) -> Option<MotionComponent> {
        self.components.remove(name)
    }

    /// Get a motion component
    pub fn get_component(&self, name: &str) -> Option<&MotionComponent> {
        self.components.get(name)
    }

    /// Get all registered components
    pub fn components(&self) -> &std::collections::HashMap<String, MotionComponent> {
        &self.components
    }

    /// Get configuration
    pub fn config(&self) -> &LeptosIntegrationConfig {
        &self.config
    }

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut LeptosIntegrationConfig {
        &mut self.config
    }

    /// Get resource integration
    pub fn resource_integration(&self) -> &ResourceIntegration {
        &self.resource_integration
    }

    /// Get mutable resource integration
    pub fn resource_integration_mut(&mut self) -> &mut ResourceIntegration {
        &mut self.resource_integration
    }

    /// Initialize the integration
    pub fn initialize(&mut self) -> Result<()> {
        if self.config.auto_register_components {
            self.register_default_components()?;
        }
        Ok(())
    }

    /// Register default components
    fn register_default_components(&mut self) -> Result<()> {
        let default_components = vec![
            MotionComponent {
                name: "MotionDiv".to_string(),
                component_type: MotionComponentType::Basic,
                props: std::collections::HashMap::new(),
            },
            MotionComponent {
                name: "MotionSpan".to_string(),
                component_type: MotionComponentType::Basic,
                props: std::collections::HashMap::new(),
            },
            MotionComponent {
                name: "MotionButton".to_string(),
                component_type: MotionComponentType::Basic,
                props: std::collections::HashMap::new(),
            },
        ];

        for component in default_components {
            self.register_component(component)?;
        }

        Ok(())
    }

    /// Check if a component is registered
    pub fn is_component_registered(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// Get component count
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Clear all components
    pub fn clear_components(&mut self) {
        self.components.clear();
    }
}

impl Default for LeptosIntegrationConfig {
    fn default() -> Self {
        Self {
            auto_register_components: true,
            enable_ssr: true,
            enable_hydration: true,
            enable_dev_mode: false,
        }
    }
}

impl ResourceIntegration {
    /// Create a new resource integration
    pub fn new() -> Self {
        Self {
            resource_manager: ResourceManager::new(),
            asset_loader: AssetLoader::new(),
        }
    }

    /// Get resource manager
    pub fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    /// Get mutable resource manager
    pub fn resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    /// Get asset loader
    pub fn asset_loader(&self) -> &AssetLoader {
        &self.asset_loader
    }

    /// Get mutable asset loader
    pub fn asset_loader_mut(&mut self) -> &mut AssetLoader {
        &mut self.asset_loader
    }
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: std::collections::HashMap::new(),
        }
    }

    /// Register a resource
    pub fn register_resource(&mut self, resource: ResourceInfo) {
        self.resources.insert(resource.name.clone(), resource);
    }

    /// Get a resource
    pub fn get_resource(&self, name: &str) -> Option<&ResourceInfo> {
        self.resources.get(name)
    }

    /// Get all resources
    pub fn resources(&self) -> &std::collections::HashMap<String, ResourceInfo> {
        &self.resources
    }

    /// Get resource count
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

impl AssetLoader {
    /// Create a new asset loader
    pub fn new() -> Self {
        Self {
            assets: std::collections::HashMap::new(),
        }
    }

    /// Load an asset
    pub fn load_asset(&mut self, asset: AssetInfo) {
        self.assets.insert(asset.name.clone(), asset);
    }

    /// Get an asset
    pub fn get_asset(&self, name: &str) -> Option<&AssetInfo> {
        self.assets.get(name)
    }

    /// Get all assets
    pub fn assets(&self) -> &std::collections::HashMap<String, AssetInfo> {
        &self.assets
    }

    /// Get asset count
    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }
}
