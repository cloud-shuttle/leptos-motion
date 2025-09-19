//! Build tool integration

use crate::{Result, AnimationError};

/// Build tool integration for motion components
pub struct BuildToolIntegration {
    /// Supported build tools
    supported_tools: std::collections::HashSet<BuildToolType>,
    /// Build tool configurations
    configurations: std::collections::HashMap<BuildToolType, BuildToolConfig>,
    /// Build tool registry
    registry: BuildToolRegistry,
}

/// Build tool configuration
#[derive(Debug, Clone)]
pub struct BuildToolConfig {
    /// Build tool type
    pub tool_type: BuildToolType,
    /// Configuration options
    pub options: std::collections::HashMap<String, String>,
    /// Whether to enable optimization
    pub enable_optimization: bool,
    /// Whether to enable source maps
    pub enable_source_maps: bool,
    /// Whether to enable tree shaking
    pub enable_tree_shaking: bool,
    /// Whether to enable minification
    pub enable_minification: bool,
}

/// Build tool registry
#[derive(Debug, Clone)]
pub struct BuildToolRegistry {
    /// Registered build tools
    tools: std::collections::HashMap<BuildToolType, BuildToolInfo>,
    /// Build tool plugins
    plugins: std::collections::HashMap<String, BuildToolPlugin>,
}

/// Build tool information
#[derive(Debug, Clone)]
pub struct BuildToolInfo {
    /// Tool name
    pub name: String,
    /// Tool version
    pub version: String,
    /// Tool description
    pub description: String,
    /// Tool capabilities
    pub capabilities: Vec<BuildToolCapability>,
}

/// Build tool plugin
#[derive(Debug, Clone)]
pub struct BuildToolPlugin {
    /// Plugin name
    pub name: String,
    /// Plugin type
    pub plugin_type: BuildToolPluginType,
    /// Plugin configuration
    pub configuration: std::collections::HashMap<String, String>,
    /// Plugin dependencies
    pub dependencies: Vec<String>,
}

/// Build tool type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildToolType {
    /// Webpack build tool
    Webpack,
    /// Vite build tool
    Vite,
    /// Rollup build tool
    Rollup,
    /// Parcel build tool
    Parcel,
    /// esbuild build tool
    Esbuild,
    /// SWC build tool
    Swc,
    /// Other build tool
    Other(String),
}

/// Build tool capability
#[derive(Debug, Clone, PartialEq)]
pub enum BuildToolCapability {
    /// TypeScript support
    TypeScript,
    /// CSS preprocessing
    CssPreprocessing,
    /// Asset optimization
    AssetOptimization,
    /// Code splitting
    CodeSplitting,
    /// Hot module replacement
    HotModuleReplacement,
    /// Tree shaking
    TreeShaking,
    /// Minification
    Minification,
    /// Source maps
    SourceMaps,
    /// Other capability
    Other(String),
}

/// Build tool plugin type
#[derive(Debug, Clone, PartialEq)]
pub enum BuildToolPluginType {
    /// Loader plugin
    Loader,
    /// Plugin plugin
    Plugin,
    /// Transformer plugin
    Transformer,
    /// Optimizer plugin
    Optimizer,
    /// Other plugin type
    Other(String),
}

impl BuildToolIntegration {
    /// Create a new build tool integration
    pub fn new() -> Self {
        Self {
            supported_tools: std::collections::HashSet::new(),
            configurations: std::collections::HashMap::new(),
            registry: BuildToolRegistry::new(),
        }
    }

    /// Add build tool support
    pub fn add_build_tool(&mut self, tool_type: BuildToolType, config: BuildToolConfig) -> Result<()> {
        if self.supported_tools.contains(&tool_type) {
            return Err(AnimationError::InvalidValue(
                format!("Build tool {:?} is already supported", tool_type)
            ));
        }

        self.configurations.insert(tool_type.clone(), config);
        self.supported_tools.insert(tool_type);
        Ok(())
    }

    /// Remove build tool support
    pub fn remove_build_tool(&mut self, tool_type: &BuildToolType) -> Option<BuildToolConfig> {
        self.supported_tools.remove(tool_type);
        self.configurations.remove(tool_type)
    }

    /// Get build tool configuration
    pub fn get_config(&self, tool_type: &BuildToolType) -> Option<&BuildToolConfig> {
        self.configurations.get(tool_type)
    }

    /// Get mutable build tool configuration
    pub fn get_config_mut(&mut self, tool_type: &BuildToolType) -> Option<&mut BuildToolConfig> {
        self.configurations.get_mut(tool_type)
    }

    /// Get supported build tools
    pub fn supported_tools(&self) -> &std::collections::HashSet<BuildToolType> {
        &self.supported_tools
    }

    /// Check if build tool is supported
    pub fn is_tool_supported(&self, tool_type: &BuildToolType) -> bool {
        self.supported_tools.contains(tool_type)
    }

    /// Get build tool registry
    pub fn registry(&self) -> &BuildToolRegistry {
        &self.registry
    }

    /// Get mutable build tool registry
    pub fn registry_mut(&mut self) -> &mut BuildToolRegistry {
        &mut self.registry
    }

    /// Initialize the integration
    pub fn initialize(&mut self) -> Result<()> {
        self.registry.initialize()?;
        Ok(())
    }

    /// Get build tool count
    pub fn tool_count(&self) -> usize {
        self.supported_tools.len()
    }
}

impl BuildToolRegistry {
    /// Create a new build tool registry
    pub fn new() -> Self {
        Self {
            tools: std::collections::HashMap::new(),
            plugins: std::collections::HashMap::new(),
        }
    }

    /// Register a build tool
    pub fn register_tool(&mut self, tool_info: BuildToolInfo) {
        let tool_type = match tool_info.name.as_str() {
            "webpack" => BuildToolType::Webpack,
            "vite" => BuildToolType::Vite,
            "rollup" => BuildToolType::Rollup,
            "parcel" => BuildToolType::Parcel,
            "esbuild" => BuildToolType::Esbuild,
            "swc" => BuildToolType::Swc,
            _ => BuildToolType::Other(tool_info.name.clone()),
        };
        self.tools.insert(tool_type, tool_info);
    }

    /// Register a build tool plugin
    pub fn register_plugin(&mut self, plugin: BuildToolPlugin) {
        self.plugins.insert(plugin.name.clone(), plugin);
    }

    /// Get a build tool
    pub fn get_tool(&self, tool_type: &BuildToolType) -> Option<&BuildToolInfo> {
        self.tools.get(tool_type)
    }

    /// Get a build tool plugin
    pub fn get_plugin(&self, name: &str) -> Option<&BuildToolPlugin> {
        self.plugins.get(name)
    }

    /// Get all build tools
    pub fn tools(&self) -> &std::collections::HashMap<BuildToolType, BuildToolInfo> {
        &self.tools
    }

    /// Get all plugins
    pub fn plugins(&self) -> &std::collections::HashMap<String, BuildToolPlugin> {
        &self.plugins
    }

    /// Initialize the registry
    pub fn initialize(&mut self) -> Result<()> {
        // Register default build tools
        self.register_default_tools();
        self.register_default_plugins();
        Ok(())
    }

    /// Register default build tools
    fn register_default_tools(&mut self) {
        let default_tools = vec![
            BuildToolInfo {
                name: "webpack".to_string(),
                version: "5.0.0".to_string(),
                description: "Webpack build tool".to_string(),
                capabilities: vec![
                    BuildToolCapability::TypeScript,
                    BuildToolCapability::CssPreprocessing,
                    BuildToolCapability::AssetOptimization,
                    BuildToolCapability::CodeSplitting,
                    BuildToolCapability::HotModuleReplacement,
                    BuildToolCapability::TreeShaking,
                    BuildToolCapability::Minification,
                    BuildToolCapability::SourceMaps,
                ],
            },
            BuildToolInfo {
                name: "vite".to_string(),
                version: "4.0.0".to_string(),
                description: "Vite build tool".to_string(),
                capabilities: vec![
                    BuildToolCapability::TypeScript,
                    BuildToolCapability::CssPreprocessing,
                    BuildToolCapability::AssetOptimization,
                    BuildToolCapability::CodeSplitting,
                    BuildToolCapability::HotModuleReplacement,
                    BuildToolCapability::TreeShaking,
                    BuildToolCapability::Minification,
                    BuildToolCapability::SourceMaps,
                ],
            },
        ];

        for tool in default_tools {
            self.register_tool(tool);
        }
    }

    /// Register default plugins
    fn register_default_plugins(&mut self) {
        let default_plugins = vec![
            BuildToolPlugin {
                name: "leptos-motion-loader".to_string(),
                plugin_type: BuildToolPluginType::Loader,
                configuration: std::collections::HashMap::new(),
                dependencies: vec!["leptos".to_string()],
            },
            BuildToolPlugin {
                name: "leptos-motion-plugin".to_string(),
                plugin_type: BuildToolPluginType::Plugin,
                configuration: std::collections::HashMap::new(),
                dependencies: vec!["leptos".to_string()],
            },
        ];

        for plugin in default_plugins {
            self.register_plugin(plugin);
        }
    }

    /// Get tool count
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}
