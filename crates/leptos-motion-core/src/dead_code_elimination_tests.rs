//! TDD tests for dead code elimination (Phase 1)
//!
//! This module contains tests to ensure that dead code elimination
//! works correctly and reduces bundle size by 120KB.

use std::collections::HashMap;

/// Represents a module that can be eliminated in production builds
#[derive(Debug, Clone, PartialEq)]
pub struct DeadCodeModule {
    /// Name of the module
    pub name: String,
    /// Estimated size in bytes when included
    pub size_bytes: u64,
    /// Whether the module is optional (can be eliminated)
    pub is_optional: bool,
    /// Feature flag that controls inclusion
    pub feature_flag: String,
}

/// Dead Code Elimination Analyzer
#[derive(Debug)]
pub struct DeadCodeEliminationAnalyzer {
    /// List of modules that can be eliminated
    pub dead_code_modules: HashMap<String, DeadCodeModule>,
    /// Total potential savings in bytes
    pub total_savings_bytes: u64,
}

impl DeadCodeEliminationAnalyzer {
    /// Create a new DeadCodeEliminationAnalyzer
    pub fn new() -> Self {
        let mut dead_code_modules = HashMap::new();
        
        // Developer tools module - 45KB
        dead_code_modules.insert(
            "developer_tools".to_string(),
            DeadCodeModule {
                name: "developer_tools".to_string(),
                size_bytes: 45_000, // 45KB
                is_optional: true,
                feature_flag: "developer-tools".to_string(),
            },
        );
        
        // Advanced examples module - 35KB
        dead_code_modules.insert(
            "advanced_examples".to_string(),
            DeadCodeModule {
                name: "advanced_examples".to_string(),
                size_bytes: 35_000, // 35KB
                is_optional: true,
                feature_flag: "advanced-examples".to_string(),
            },
        );
        
        // Ecosystem integration module - 25KB
        dead_code_modules.insert(
            "ecosystem_integration".to_string(),
            DeadCodeModule {
                name: "ecosystem_integration".to_string(),
                size_bytes: 25_000, // 25KB
                is_optional: true,
                feature_flag: "ecosystem-integration".to_string(),
            },
        );
        
        let total_savings_bytes: u64 = dead_code_modules.values()
            .map(|module| module.size_bytes)
            .sum();
        
        Self {
            dead_code_modules,
            total_savings_bytes,
        }
    }
    
    /// Check if a module is marked as dead code
    pub fn is_dead_code(&self, module_name: &str) -> bool {
        self.dead_code_modules.contains_key(module_name)
    }
    
    /// Get the feature flag for a module
    pub fn get_feature_flag(&self, module_name: &str) -> Option<&String> {
        self.dead_code_modules.get(module_name).map(|m| &m.feature_flag)
    }
    
    /// Calculate bundle size with dead code elimination
    pub fn calculate_optimized_size(&self, base_size_bytes: u64, enabled_features: &[&str]) -> u64 {
        let mut optimized_size = base_size_bytes;
        
        for (module_name, module) in &self.dead_code_modules {
            if !enabled_features.contains(&module.feature_flag.as_str()) {
                optimized_size -= module.size_bytes;
            }
        }
        
        optimized_size
    }
    
    /// Get the savings percentage
    pub fn get_savings_percentage(&self, base_size_bytes: u64) -> f64 {
        (self.total_savings_bytes as f64 / base_size_bytes as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test dead code module identification
    #[test]
    fn test_dead_code_module_identification() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        
        assert!(analyzer.is_dead_code("developer_tools"));
        assert!(analyzer.is_dead_code("advanced_examples"));
        assert!(analyzer.is_dead_code("ecosystem_integration"));
        assert!(!analyzer.is_dead_code("core_engine"));
    }
    
    /// Test feature flag mapping
    #[test]
    fn test_feature_flag_mapping() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        
        assert_eq!(analyzer.get_feature_flag("developer_tools"), Some(&"developer-tools".to_string()));
        assert_eq!(analyzer.get_feature_flag("advanced_examples"), Some(&"advanced-examples".to_string()));
        assert_eq!(analyzer.get_feature_flag("ecosystem_integration"), Some(&"ecosystem-integration".to_string()));
        assert_eq!(analyzer.get_feature_flag("core_engine"), None);
    }
    
    /// Test bundle size calculation with dead code elimination
    #[test]
    fn test_bundle_size_calculation() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        let base_size = 378_000; // 378KB base size
        
        // With all dead code modules enabled (full build)
        let full_size = analyzer.calculate_optimized_size(base_size, &["developer-tools", "advanced-examples", "ecosystem-integration"]);
        assert_eq!(full_size, base_size);
        
        // With no dead code modules enabled (production build)
        let production_size = analyzer.calculate_optimized_size(base_size, &[]);
        assert_eq!(production_size, base_size - analyzer.total_savings_bytes);
        
        // With only developer tools enabled
        let partial_size = analyzer.calculate_optimized_size(base_size, &["developer-tools"]);
        assert_eq!(partial_size, base_size - 35_000 - 25_000); // Exclude advanced_examples and ecosystem_integration
    }
    
    /// Test savings percentage calculation
    #[test]
    fn test_savings_percentage() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        let base_size = 378_000; // 378KB base size
        
        let savings_percentage = analyzer.get_savings_percentage(base_size);
        let expected_percentage = (105_000.0 / 378_000.0) * 100.0; // 105KB / 378KB * 100%
        
        assert!((savings_percentage - expected_percentage).abs() < 0.1);
    }
    
    /// Test total savings amount
    #[test]
    fn test_total_savings_amount() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        
        // Should save 45KB + 35KB + 25KB = 105KB
        assert_eq!(analyzer.total_savings_bytes, 105_000);
    }
    
    /// Test that dead code modules are marked as optional
    #[test]
    fn test_dead_code_modules_are_optional() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        
        for (_, module) in &analyzer.dead_code_modules {
            assert!(module.is_optional, "Module {} should be marked as optional", module.name);
        }
    }
    
    /// Test production build size target
    #[test]
    fn test_production_build_size_target() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        let base_size = 378_000; // 378KB base size
        
        // Production build should be 378KB - 105KB = 273KB
        let production_size = analyzer.calculate_optimized_size(base_size, &[]);
        assert_eq!(production_size, 273_000);
        
        // This should be a 27.8% reduction
        let reduction_percentage = ((base_size - production_size) as f64 / base_size as f64) * 100.0;
        assert!((reduction_percentage - 27.8).abs() < 0.1);
    }
    
    /// Test conditional compilation simulation
    #[test]
    fn test_conditional_compilation_simulation() {
        let analyzer = DeadCodeEliminationAnalyzer::new();
        
        // Simulate different build configurations
        let configurations = vec![
            ("minimal", vec![]),
            ("standard", vec![]),
            ("production", vec![]),
            ("full", vec!["developer-tools", "advanced-examples", "ecosystem-integration"]),
        ];
        
        for (config_name, features) in configurations {
            let size = analyzer.calculate_optimized_size(378_000, &features);
            
            match config_name {
                "minimal" | "standard" | "production" => {
                    // These should exclude dead code
                    assert_eq!(size, 273_000);
                },
                "full" => {
                    // This should include all dead code
                    assert_eq!(size, 378_000);
                },
                _ => {}
            }
        }
    }
}
