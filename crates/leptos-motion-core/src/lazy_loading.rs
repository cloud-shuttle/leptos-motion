//! Lazy Loading and Code Splitting
//!
//! This module provides lazy loading capabilities for different parts of the
//! animation system, allowing for better bundle size optimization and
//! on-demand loading of features.

use crate::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

/// Represents a lazy-loaded module
pub trait LazyModule: Send + Sync {
    /// Get the module name
    fn name(&self) -> &str;

    /// Get the module size estimate in bytes
    fn size_estimate(&self) -> usize;

    /// Check if the module is loaded
    fn is_loaded(&self) -> bool;
}

/// Lazy loader for animation modules
pub struct AnimationLazyLoader {
    loaded_modules: Arc<Mutex<HashMap<String, Box<dyn LazyModule>>>>,
    load_times: Arc<Mutex<HashMap<String, Instant>>>,
    module_sizes: Arc<Mutex<HashMap<String, usize>>>,
}

impl AnimationLazyLoader {
    /// Create a new lazy loader
    pub fn new() -> Self {
        Self {
            loaded_modules: Arc::new(Mutex::new(HashMap::new())),
            load_times: Arc::new(Mutex::new(HashMap::new())),
            module_sizes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load a module lazily
    pub fn load_module(&self, name: &str, module: Box<dyn LazyModule>) -> Result<()> {
        let mut modules = self.loaded_modules.lock().unwrap();
        let mut times = self.load_times.lock().unwrap();
        let mut sizes = self.module_sizes.lock().unwrap();

        if modules.contains_key(name) {
            return Err(crate::AnimationError::EngineUnavailable(format!(
                "Module {} already loaded",
                name
            )));
        }

        let size = module.size_estimate();
        modules.insert(name.to_string(), module);
        times.insert(name.to_string(), Instant::now());
        sizes.insert(name.to_string(), size);

        Ok(())
    }

    /// Check if a module is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        let modules = self.loaded_modules.lock().unwrap();
        modules.contains_key(name)
    }

    /// Get a loaded module
    pub fn get_module(&self, _name: &str) -> Option<Box<dyn LazyModule>> {
        let _modules = self.loaded_modules.lock().unwrap();
        // Note: In a real implementation, we'd need to handle the trait object properly
        // For now, we'll return None as this is a simplified version
        None
    }

    /// Get the total size of loaded modules
    pub fn total_loaded_size(&self) -> usize {
        let sizes = self.module_sizes.lock().unwrap();
        sizes.values().sum()
    }

    /// Get the number of loaded modules
    pub fn loaded_count(&self) -> usize {
        let modules = self.loaded_modules.lock().unwrap();
        modules.len()
    }

    /// Get load time for a module
    pub fn get_load_time(&self, name: &str) -> Option<Instant> {
        let times = self.load_times.lock().unwrap();
        times.get(name).copied()
    }

    /// Unload a module (for cleanup)
    pub fn unload_module(&self, name: &str) -> Result<()> {
        let mut modules = self.loaded_modules.lock().unwrap();
        let mut times = self.load_times.lock().unwrap();
        let mut sizes = self.module_sizes.lock().unwrap();

        if !modules.contains_key(name) {
            return Err(crate::AnimationError::EngineUnavailable(format!(
                "Module {} not loaded",
                name
            )));
        }

        modules.remove(name);
        times.remove(name);
        sizes.remove(name);

        Ok(())
    }

    /// Clear all loaded modules
    pub fn clear_all(&self) {
        let mut modules = self.loaded_modules.lock().unwrap();
        let mut times = self.load_times.lock().unwrap();
        let mut sizes = self.module_sizes.lock().unwrap();

        modules.clear();
        times.clear();
        sizes.clear();
    }
}

/// Global lazy loader instance
static LAZY_LOADER: OnceLock<AnimationLazyLoader> = OnceLock::new();

/// Get the global lazy loader instance
pub fn get_lazy_loader() -> &'static AnimationLazyLoader {
    LAZY_LOADER.get_or_init(|| AnimationLazyLoader::new())
}

/// Lazy loading configuration
#[derive(Debug, Clone)]
pub struct LazyLoadingConfig {
    /// Maximum number of modules to keep loaded
    pub max_loaded_modules: usize,
    /// Maximum total size of loaded modules in bytes
    pub max_total_size: usize,
    /// Whether to enable automatic cleanup
    pub auto_cleanup: bool,
    /// Cleanup threshold (unload modules when exceeded)
    pub cleanup_threshold: f64,
}

impl Default for LazyLoadingConfig {
    fn default() -> Self {
        Self {
            max_loaded_modules: 10,
            max_total_size: 1024 * 1024, // 1MB
            auto_cleanup: true,
            cleanup_threshold: 0.8, // 80% of limits
        }
    }
}

/// Feature-based module loader
pub struct FeatureModuleLoader {
    config: LazyLoadingConfig,
    loader: &'static AnimationLazyLoader,
}

impl FeatureModuleLoader {
    /// Create a new feature module loader
    pub fn new(config: LazyLoadingConfig) -> Self {
        Self {
            config,
            loader: get_lazy_loader(),
        }
    }

    /// Load modules based on feature flags
    pub fn load_features(&self, features: &[&str]) -> Result<()> {
        for feature in features {
            match *feature {
                "gestures" => self.load_gesture_modules()?,
                "layout" => self.load_layout_modules()?,
                "scroll" => self.load_scroll_modules()?,
                "performance" => self.load_performance_modules()?,
                _ => {
                    // Unknown feature, skip
                    continue;
                }
            }
        }
        Ok(())
    }

    /// Load gesture-related modules
    fn load_gesture_modules(&self) -> Result<()> {
        // In a real implementation, this would load actual gesture modules
        // For now, we'll simulate it
        Ok(())
    }

    /// Load layout-related modules
    fn load_layout_modules(&self) -> Result<()> {
        // In a real implementation, this would load actual layout modules
        // For now, we'll simulate it
        Ok(())
    }

    /// Load scroll-related modules
    fn load_scroll_modules(&self) -> Result<()> {
        // In a real implementation, this would load actual scroll modules
        // For now, we'll simulate it
        Ok(())
    }

    /// Load performance-related modules
    fn load_performance_modules(&self) -> Result<()> {
        // In a real implementation, this would load actual performance modules
        // For now, we'll simulate it
        Ok(())
    }

    /// Check if cleanup is needed
    pub fn needs_cleanup(&self) -> bool {
        if !self.config.auto_cleanup {
            return false;
        }

        let loaded_count = self.loader.loaded_count();
        let total_size = self.loader.total_loaded_size();

        let count_threshold =
            (self.config.max_loaded_modules as f64 * self.config.cleanup_threshold) as usize;
        let size_threshold =
            (self.config.max_total_size as f64 * self.config.cleanup_threshold) as usize;

        loaded_count > count_threshold || total_size > size_threshold
    }

    /// Perform cleanup if needed
    pub fn cleanup_if_needed(&self) -> Result<()> {
        if self.needs_cleanup() {
            // In a real implementation, this would implement a more sophisticated
            // cleanup strategy (e.g., LRU, size-based, etc.)
            self.loader.clear_all();
        }
        Ok(())
    }
}

/// Dynamic import simulation for browser environments
#[cfg(target_arch = "wasm32")]
pub mod dynamic_import {
    use js_sys::Promise;
    use std::future::Future;
    use std::pin::Pin;
    use wasm_bindgen::prelude::*;

    /// Simulate dynamic import for WASM modules
    pub async fn import_module(module_name: &str) -> Result<JsValue, JsValue> {
        // In a real implementation, this would use the browser's dynamic import
        // For now, we'll simulate it
        let promise = Promise::resolve(&JsValue::UNDEFINED);
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        future.await
    }

    /// Load a module dynamically based on user interaction
    pub async fn load_on_interaction(interaction_type: &str) -> Result<JsValue, JsValue> {
        match interaction_type {
            "drag" => import_module("gesture_drag").await,
            "hover" => import_module("gesture_hover").await,
            "scroll" => import_module("scroll_handler").await,
            _ => Err(JsValue::from_str("Unknown interaction type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock module for testing
    struct MockModule {
        name: String,
        size: usize,
        loaded: bool,
    }

    impl LazyModule for MockModule {
        fn name(&self) -> &str {
            &self.name
        }

        fn size_estimate(&self) -> usize {
            self.size
        }

        fn is_loaded(&self) -> bool {
            self.loaded
        }
    }

    #[test]
    fn test_lazy_loader_creation() {
        let loader = AnimationLazyLoader::new();
        assert_eq!(loader.loaded_count(), 0);
        assert_eq!(loader.total_loaded_size(), 0);
    }

    #[test]
    fn test_lazy_loader_load_module() {
        let loader = AnimationLazyLoader::new();
        let module = Box::new(MockModule {
            name: "test_module".to_string(),
            size: 1024,
            loaded: true,
        });

        let result = loader.load_module("test", module);
        assert!(result.is_ok());
        assert_eq!(loader.loaded_count(), 1);
        assert_eq!(loader.total_loaded_size(), 1024);
        assert!(loader.is_loaded("test"));
    }

    #[test]
    fn test_lazy_loader_duplicate_load() {
        let loader = AnimationLazyLoader::new();
        let module1 = Box::new(MockModule {
            name: "test_module1".to_string(),
            size: 1024,
            loaded: true,
        });
        let module2 = Box::new(MockModule {
            name: "test_module2".to_string(),
            size: 2048,
            loaded: true,
        });

        loader.load_module("test", module1).unwrap();
        let result = loader.load_module("test", module2);
        assert!(result.is_err());
        assert_eq!(loader.loaded_count(), 1);
    }

    #[test]
    fn test_lazy_loader_unload() {
        let loader = AnimationLazyLoader::new();
        let module = Box::new(MockModule {
            name: "test_module".to_string(),
            size: 1024,
            loaded: true,
        });

        loader.load_module("test", module).unwrap();
        assert_eq!(loader.loaded_count(), 1);

        let result = loader.unload_module("test");
        assert!(result.is_ok());
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_loaded("test"));
    }

    #[test]
    fn test_feature_loader_config() {
        let config = LazyLoadingConfig::default();
        let loader = FeatureModuleLoader::new(config);

        // Test with no modules loaded
        assert!(!loader.needs_cleanup());

        // Test cleanup
        let result = loader.cleanup_if_needed();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_loader_load_features() {
        let config = LazyLoadingConfig::default();
        let loader = FeatureModuleLoader::new(config);

        let features = ["gestures", "layout", "scroll"];
        let result = loader.load_features(&features);
        assert!(result.is_ok());
    }
}
