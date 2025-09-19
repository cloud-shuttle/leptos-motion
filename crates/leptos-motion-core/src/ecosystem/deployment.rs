//! Deployment platform integration

use crate::{Result, AnimationError};

/// Deployment platform integration
pub struct DeploymentIntegration {
    /// Supported platforms
    supported_platforms: std::collections::HashSet<DeploymentPlatform>,
    /// Platform configurations
    configurations: std::collections::HashMap<DeploymentPlatform, DeploymentConfig>,
}

/// Deployment configuration
#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    /// Platform type
    pub platform: DeploymentPlatform,
    /// Configuration options
    pub options: std::collections::HashMap<String, String>,
    /// Whether to enable optimization
    pub enable_optimization: bool,
    /// Whether to enable compression
    pub enable_compression: bool,
}

/// Deployment platform
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeploymentPlatform {
    /// Vercel platform
    Vercel,
    /// Netlify platform
    Netlify,
    /// GitHub Pages platform
    GitHubPages,
    /// AWS platform
    Aws,
    /// Other platform
    Other(String),
}

impl DeploymentIntegration {
    /// Create a new deployment integration
    pub fn new() -> Self {
        Self {
            supported_platforms: std::collections::HashSet::new(),
            configurations: std::collections::HashMap::new(),
        }
    }

    /// Add platform support
    pub fn add_platform(&mut self, platform: DeploymentPlatform, config: DeploymentConfig) -> Result<()> {
        if self.supported_platforms.contains(&platform) {
            return Err(AnimationError::InvalidValue(
                format!("Platform {:?} is already supported", platform)
            ));
        }

        self.configurations.insert(platform.clone(), config);
        self.supported_platforms.insert(platform);
        Ok(())
    }

    /// Get platform configuration
    pub fn get_config(&self, platform: &DeploymentPlatform) -> Option<&DeploymentConfig> {
        self.configurations.get(platform)
    }

    /// Get supported platforms
    pub fn supported_platforms(&self) -> &std::collections::HashSet<DeploymentPlatform> {
        &self.supported_platforms
    }

    /// Check if platform is supported
    pub fn is_platform_supported(&self, platform: &DeploymentPlatform) -> bool {
        self.supported_platforms.contains(platform)
    }
}
