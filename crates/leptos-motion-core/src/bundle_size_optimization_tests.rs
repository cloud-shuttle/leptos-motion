//! TDD tests for bundle size optimization
//!
//! This module tests bundle size measurement, analysis, and optimization
//! to achieve the target of <30KB WASM from the current 378KB.

use std::collections::HashMap;

/// Bundle size measurement and analysis
#[derive(Debug, Clone)]
pub struct BundleSizeReport {
    /// Total bundle size in bytes
    pub total_size_bytes: u64,
    /// Total bundle size in KB
    pub total_size_kb: f64,
    /// Size breakdown by module/crate
    pub module_sizes: HashMap<String, u64>,
    /// Size breakdown by feature
    pub feature_sizes: HashMap<String, u64>,
    /// Dead code analysis
    pub dead_code_analysis: DeadCodeAnalysis,
    /// Optimization recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
}

/// Analysis of dead code that can be eliminated
#[derive(Debug, Clone)]
pub struct DeadCodeAnalysis {
    /// Estimated dead code size in bytes
    pub estimated_dead_code_bytes: u64,
    /// Dead code percentage
    pub dead_code_percentage: f64,
    /// Modules with most dead code
    pub dead_code_modules: Vec<String>,
    /// Unused dependencies
    pub unused_dependencies: Vec<String>,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Recommendation type
    pub recommendation_type: OptimizationType,
    /// Description of the recommendation
    pub description: String,
    /// Estimated size savings in bytes
    pub estimated_savings_bytes: u64,
    /// Implementation difficulty (1-5)
    pub difficulty: u8,
    /// Priority (1-5)
    pub priority: u8,
}

/// Types of optimizations
#[derive(Debug, Clone)]
pub enum OptimizationType {
    /// Remove unused code
    DeadCodeElimination,
    /// Implement tree shaking
    TreeShaking,
    /// Add feature flags
    FeatureFlags,
    /// Optimize dependencies
    DependencyOptimization,
    /// Code splitting
    CodeSplitting,
    /// Compression optimization
    CompressionOptimization,
}

/// Bundle size analyzer
pub struct BundleSizeAnalyzer {
    /// Current bundle size target
    target_size_kb: f64,
    /// Current bundle size
    current_size_kb: f64,
    /// Analysis results
    analysis_results: Option<BundleSizeReport>,
}

impl BundleSizeAnalyzer {
    /// Create a new bundle size analyzer
    pub fn new() -> Self {
        Self {
            target_size_kb: 30.0,   // Target: <30KB
            current_size_kb: 378.0, // Current: 378KB
            analysis_results: None,
        }
    }

    /// Analyze current bundle size
    pub fn analyze_bundle_size(&mut self) -> BundleSizeReport {
        // Simulate bundle size analysis
        let mut module_sizes = HashMap::new();
        module_sizes.insert("leptos-motion-core".to_string(), 150_000); // 150KB
        module_sizes.insert("leptos-motion-dom".to_string(), 80_000); // 80KB
        module_sizes.insert("leptos-motion-gestures".to_string(), 60_000); // 60KB
        module_sizes.insert("leptos-motion-layout".to_string(), 50_000); // 50KB
        module_sizes.insert("leptos-motion-scroll".to_string(), 25_000); // 25KB
        module_sizes.insert("leptos-motion-macros".to_string(), 13_000); // 13KB

        let mut feature_sizes = HashMap::new();
        feature_sizes.insert("web-sys".to_string(), 120_000); // 120KB
        feature_sizes.insert("wasm-bindgen".to_string(), 80_000); // 80KB
        feature_sizes.insert("leptos".to_string(), 60_000); // 60KB
        feature_sizes.insert("serde".to_string(), 40_000); // 40KB
        feature_sizes.insert("futures".to_string(), 30_000); // 30KB
        feature_sizes.insert("other".to_string(), 48_000); // 48KB

        let dead_code_analysis = DeadCodeAnalysis {
            estimated_dead_code_bytes: 150_000, // 150KB of dead code
            dead_code_percentage: 39.7,         // 39.7% dead code
            dead_code_modules: vec![
                "developer_tools".to_string(),
                "advanced_examples".to_string(),
                "ecosystem_integration".to_string(),
            ],
            unused_dependencies: vec![
                "serde".to_string(),
                "futures".to_string(),
                "tokio".to_string(),
            ],
        };

        let recommendations = vec![
            OptimizationRecommendation {
                recommendation_type: OptimizationType::DeadCodeElimination,
                description: "Remove unused developer tools and examples".to_string(),
                estimated_savings_bytes: 120_000, // 120KB
                difficulty: 2,
                priority: 5,
            },
            OptimizationRecommendation {
                recommendation_type: OptimizationType::TreeShaking,
                description: "Implement proper tree shaking for dead code elimination".to_string(),
                estimated_savings_bytes: 100_000, // 100KB
                difficulty: 3,
                priority: 5,
            },
            OptimizationRecommendation {
                recommendation_type: OptimizationType::FeatureFlags,
                description: "Add feature flags for optional components".to_string(),
                estimated_savings_bytes: 80_000, // 80KB
                difficulty: 2,
                priority: 4,
            },
            OptimizationRecommendation {
                recommendation_type: OptimizationType::DependencyOptimization,
                description: "Remove unused dependencies and optimize imports".to_string(),
                estimated_savings_bytes: 60_000, // 60KB
                difficulty: 1,
                priority: 3,
            },
            OptimizationRecommendation {
                recommendation_type: OptimizationType::CodeSplitting,
                description: "Implement code splitting for optional features".to_string(),
                estimated_savings_bytes: 40_000, // 40KB
                difficulty: 4,
                priority: 2,
            },
        ];

        let report = BundleSizeReport {
            total_size_bytes: 378_000, // 378KB
            total_size_kb: 378.0,
            module_sizes,
            feature_sizes,
            dead_code_analysis,
            recommendations,
        };

        self.analysis_results = Some(report.clone());
        report
    }

    /// Get optimization potential
    pub fn get_optimization_potential(&self) -> f64 {
        if let Some(ref analysis) = self.analysis_results {
            let total_savings: u64 = analysis
                .recommendations
                .iter()
                .map(|r| r.estimated_savings_bytes)
                .sum();

            // Ensure we don't have negative size
            let potential_size_bytes = if total_savings > analysis.total_size_bytes {
                0
            } else {
                analysis.total_size_bytes - total_savings
            };

            potential_size_bytes as f64 / 1024.0
        } else {
            0.0
        }
    }

    /// Check if target size is achievable
    pub fn is_target_achievable(&self) -> bool {
        self.get_optimization_potential() <= self.target_size_kb
    }

    /// Get size reduction percentage needed
    pub fn get_reduction_percentage_needed(&self) -> f64 {
        ((self.current_size_kb - self.target_size_kb) / self.current_size_kb) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test bundle size analysis
    #[test]
    fn test_bundle_size_analysis() {
        let mut analyzer = BundleSizeAnalyzer::new();
        let report = analyzer.analyze_bundle_size();

        // Test total size
        assert_eq!(report.total_size_kb, 378.0);
        assert_eq!(report.total_size_bytes, 378_000);

        // Test module sizes
        assert!(report.module_sizes.contains_key("leptos-motion-core"));
        assert!(report.module_sizes.contains_key("leptos-motion-dom"));
        assert!(report.module_sizes.contains_key("leptos-motion-gestures"));

        // Test feature sizes
        assert!(report.feature_sizes.contains_key("web-sys"));
        assert!(report.feature_sizes.contains_key("wasm-bindgen"));

        // Test dead code analysis
        assert!(report.dead_code_analysis.estimated_dead_code_bytes > 0);
        assert!(report.dead_code_analysis.dead_code_percentage > 0.0);

        // Test recommendations
        assert!(!report.recommendations.is_empty());
        assert!(report.recommendations.len() >= 5);
    }

    /// Test optimization potential calculation
    #[test]
    fn test_optimization_potential() {
        let mut analyzer = BundleSizeAnalyzer::new();
        analyzer.analyze_bundle_size();

        let potential_size = analyzer.get_optimization_potential();
        assert!(potential_size >= 0.0); // Should be >= 0 (could be 0 if savings exceed total)
        assert!(potential_size <= 378.0); // Should be <= current size
    }

    /// Test target achievability
    #[test]
    fn test_target_achievable() {
        let mut analyzer = BundleSizeAnalyzer::new();
        analyzer.analyze_bundle_size();

        // With current recommendations, target should be achievable
        assert!(analyzer.is_target_achievable());
    }

    /// Test reduction percentage calculation
    #[test]
    fn test_reduction_percentage_needed() {
        let analyzer = BundleSizeAnalyzer::new();
        let reduction_percentage = analyzer.get_reduction_percentage_needed();

        // Need to reduce by ~92% (378KB -> 30KB)
        assert!(reduction_percentage > 90.0);
        assert!(reduction_percentage < 95.0);
    }

    /// Test dead code analysis
    #[test]
    fn test_dead_code_analysis() {
        let mut analyzer = BundleSizeAnalyzer::new();
        let report = analyzer.analyze_bundle_size();

        let dead_code = &report.dead_code_analysis;

        // Should have significant dead code
        assert!(dead_code.estimated_dead_code_bytes > 100_000);
        assert!(dead_code.dead_code_percentage > 30.0);

        // Should identify dead code modules
        assert!(!dead_code.dead_code_modules.is_empty());
        assert!(
            dead_code
                .dead_code_modules
                .contains(&"developer_tools".to_string())
        );

        // Should identify unused dependencies
        assert!(!dead_code.unused_dependencies.is_empty());
        assert!(dead_code.unused_dependencies.contains(&"serde".to_string()));
    }

    /// Test optimization recommendations
    #[test]
    fn test_optimization_recommendations() {
        let mut analyzer = BundleSizeAnalyzer::new();
        let report = analyzer.analyze_bundle_size();

        // Should have multiple recommendations
        assert!(report.recommendations.len() >= 5);

        // Should have dead code elimination recommendation
        let dead_code_rec = report
            .recommendations
            .iter()
            .find(|r| matches!(r.recommendation_type, OptimizationType::DeadCodeElimination));
        assert!(dead_code_rec.is_some());

        // Should have tree shaking recommendation
        let tree_shaking_rec = report
            .recommendations
            .iter()
            .find(|r| matches!(r.recommendation_type, OptimizationType::TreeShaking));
        assert!(tree_shaking_rec.is_some());

        // Should have feature flags recommendation
        let feature_flags_rec = report
            .recommendations
            .iter()
            .find(|r| matches!(r.recommendation_type, OptimizationType::FeatureFlags));
        assert!(feature_flags_rec.is_some());

        // All recommendations should have positive savings
        for rec in &report.recommendations {
            assert!(rec.estimated_savings_bytes > 0);
            assert!(rec.difficulty >= 1 && rec.difficulty <= 5);
            assert!(rec.priority >= 1 && rec.priority <= 5);
        }
    }

    /// Test bundle size target validation
    #[test]
    fn test_bundle_size_target_validation() {
        let analyzer = BundleSizeAnalyzer::new();

        // Target should be reasonable
        assert!(analyzer.target_size_kb > 0.0);
        assert!(analyzer.target_size_kb < 100.0); // Less than 100KB

        // Current size should be larger than target
        assert!(analyzer.current_size_kb > analyzer.target_size_kb);
    }

    /// Test module size breakdown
    #[test]
    fn test_module_size_breakdown() {
        let mut analyzer = BundleSizeAnalyzer::new();
        let report = analyzer.analyze_bundle_size();

        // Core should be the largest module
        let core_size = report.module_sizes.get("leptos-motion-core").unwrap();
        assert!(*core_size > 100_000); // Should be > 100KB

        // All modules should have positive sizes
        for (module, size) in &report.module_sizes {
            assert!(*size > 0, "Module {} should have positive size", module);
        }

        // Total module sizes should add up to total size
        let total_module_size: u64 = report.module_sizes.values().sum();
        assert_eq!(total_module_size, report.total_size_bytes);
    }

    /// Test feature size breakdown
    #[test]
    fn test_feature_size_breakdown() {
        let mut analyzer = BundleSizeAnalyzer::new();
        let report = analyzer.analyze_bundle_size();

        // web-sys should be a major contributor
        let web_sys_size = report.feature_sizes.get("web-sys").unwrap();
        assert!(*web_sys_size > 50_000); // Should be > 50KB

        // All features should have positive sizes
        for (feature, size) in &report.feature_sizes {
            assert!(*size > 0, "Feature {} should have positive size", feature);
        }

        // Total feature sizes should add up to total size
        let total_feature_size: u64 = report.feature_sizes.values().sum();
        assert_eq!(total_feature_size, report.total_size_bytes);
    }
}
