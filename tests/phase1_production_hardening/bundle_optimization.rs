//! Phase 1 TDD Implementation: Bundle Size Optimization
//! 
//! Target: Achieve <50KB minimal build (currently 643KB)
//! Focus: Advanced tree shaking, micro-optimizations, feature granularity

use rstest::*;
use wasm_bindgen_test::*;
use std::process::Command;

wasm_bindgen_test_configure!(run_in_browser);

/// Test: Minimal bundle size under 50KB
/// This will FAIL initially - current minimal build is 643KB
#[wasm_bindgen_test]
fn test_minimal_bundle_under_50kb() {
    // Arrange: Build with minimal features only
    let bundle_size = get_minimal_bundle_size();
    const TARGET_SIZE_KB: usize = 50;
    const TARGET_SIZE_BYTES: usize = TARGET_SIZE_KB * 1024;
    
    // Assert: Bundle should be under 50KB
    assert!(
        bundle_size < TARGET_SIZE_BYTES,
        "Minimal bundle size {}KB exceeds target {}KB by {}KB", 
        bundle_size / 1024,
        TARGET_SIZE_KB,
        (bundle_size.saturating_sub(TARGET_SIZE_BYTES)) / 1024
    );
    
    // Log progress toward goal
    web_sys::console::log_1(&format!(
        "Bundle optimization progress: {}KB / {}KB target ({}% of goal)",
        bundle_size / 1024,
        TARGET_SIZE_KB,
        (bundle_size * 100) / TARGET_SIZE_BYTES
    ).into());
}

/// Test: Tree shaking eliminates unused features
/// This will FAIL initially - need better dead code elimination
#[rstest]
#[case::spring_physics("spring-physics")]
#[case::gesture_support("gesture-support")]
#[case::layout_animations("layout-animations")]
#[case::advanced_easing("advanced-easing")]
#[wasm_bindgen_test]
fn test_tree_shaking_effectiveness(#[case] unused_feature: &str) {
    // Arrange: Build with and without specific feature
    let minimal_size = get_bundle_size_without_feature(unused_feature);
    let with_feature_size = get_bundle_size_with_feature(unused_feature);
    
    // Assert: Unused features should be completely eliminated
    assert_eq!(
        minimal_size, 
        get_minimal_bundle_size(),
        "Feature '{}' should be completely tree-shaken when not used",
        unused_feature
    );
    
    // Assert: Feature should add size when actually used
    assert!(
        with_feature_size > minimal_size,
        "Feature '{}' should add size when included: minimal={}KB, with_feature={}KB",
        unused_feature,
        minimal_size / 1024,
        with_feature_size / 1024
    );
}

/// Test: Feature granularity allows fine-grained control
/// This will FAIL initially - need more granular feature flags
#[rstest]
#[case::core_only(vec!["core-animations"])]
#[case::with_easing(vec!["core-animations", "easing"])]
#[case::with_spring(vec!["core-animations", "spring"])]
#[case::with_gestures(vec!["core-animations", "gesture-support"])]
#[wasm_bindgen_test]
fn test_feature_granularity_bundle_sizes(#[case] features: Vec<&str>) {
    // Arrange: Build with specific feature combination
    let bundle_size = get_bundle_size_with_features(&features);
    let core_only_size = get_bundle_size_with_features(&vec!["core-animations"]);
    
    // Assert: Each additional feature should add reasonable amount
    if features.len() > 1 {
        let size_increase = bundle_size.saturating_sub(core_only_size);
        const MAX_FEATURE_SIZE_KB: usize = 20; // Each feature max 20KB
        
        assert!(
            size_increase < MAX_FEATURE_SIZE_KB * 1024 * (features.len() - 1),
            "Feature combination {:?} adds too much size: {}KB (max {}KB per feature)",
            features,
            size_increase / 1024,
            MAX_FEATURE_SIZE_KB
        );
    }
    
    web_sys::console::log_1(&format!(
        "Features {:?}: {}KB",
        features,
        bundle_size / 1024
    ).into());
}

/// Test: Dead code elimination removes unused imports
/// This will FAIL initially - need better dependency analysis
#[wasm_bindgen_test]
fn test_dead_code_elimination_removes_unused_imports() {
    // Arrange: Check bundle contents
    let bundle_analysis = analyze_bundle_contents();
    
    // Assert: Known large dependencies should not be present in minimal build
    let unwanted_deps = vec![
        "futures", 
        "approx", 
        "num-traits",
        "wasm-bindgen-futures"
    ];
    
    for dep in unwanted_deps {
        assert!(
            !bundle_analysis.contains_dependency(dep),
            "Minimal bundle should not contain '{}' dependency",
            dep
        );
    }
    
    // Assert: Only essential dependencies should remain
    let essential_deps = vec!["js-sys", "wasm-bindgen"];
    for dep in essential_deps {
        assert!(
            bundle_analysis.contains_dependency(dep),
            "Minimal bundle should contain essential '{}' dependency",
            dep
        );
    }
}

/// Test: Conditional compilation removes unused code paths
/// This will FAIL initially - need better #[cfg] usage
#[rstest]
#[case::without_leptos("leptos-integration")]
#[case::without_serde("serde-support")]
#[case::without_web_sys("web-sys")]
#[wasm_bindgen_test]
fn test_conditional_compilation_effectiveness(#[case] feature_to_exclude: &str) {
    // Arrange: Build without specific integration
    let without_feature_size = get_bundle_size_without_feature(feature_to_exclude);
    let with_feature_size = get_bundle_size_with_feature(feature_to_exclude);
    
    // Assert: Excluding feature should significantly reduce size
    let size_reduction = with_feature_size.saturating_sub(without_feature_size);
    const MIN_REDUCTION_KB: usize = 10; // At least 10KB reduction expected
    
    assert!(
        size_reduction > MIN_REDUCTION_KB * 1024,
        "Excluding '{}' should reduce bundle by at least {}KB, got {}KB reduction",
        feature_to_exclude,
        MIN_REDUCTION_KB,
        size_reduction / 1024
    );
}

/// Test: Bundle size progression toward target
/// Track our progress toward the 50KB goal
#[wasm_bindgen_test]
fn test_bundle_size_regression_protection() {
    // Arrange: Get current sizes for different feature sets
    let sizes = BundleSizeMeasurement {
        minimal: get_minimal_bundle_size(),
        core_animations: get_bundle_size_with_features(&vec!["core-animations"]),
        standard: get_bundle_size_with_features(&vec!["standard"]),
    };
    
    // Assert: Current progress benchmarks
    const CURRENT_MINIMAL_KB: usize = 643; // Current baseline
    const TARGET_MINIMAL_KB: usize = 50;   // Our goal
    
    // We should not regress from current state
    assert!(
        sizes.minimal <= CURRENT_MINIMAL_KB * 1024,
        "Minimal bundle size regressed: {}KB > {}KB baseline",
        sizes.minimal / 1024,
        CURRENT_MINIMAL_KB
    );
    
    // Track progress
    let progress_percent = ((CURRENT_MINIMAL_KB * 1024).saturating_sub(sizes.minimal) * 100) 
        / ((CURRENT_MINIMAL_KB - TARGET_MINIMAL_KB) * 1024);
    
    web_sys::console::log_1(&format!(
        "Bundle optimization progress: {}% toward 50KB goal (currently {}KB)",
        progress_percent,
        sizes.minimal / 1024
    ).into());
    
    // For now, just ensure we're not regressing
    // TODO: This will become stricter as we implement optimizations
}

// ============================================================================
// Helper Functions for Bundle Size Testing
// ============================================================================

fn get_minimal_bundle_size() -> usize {
    get_bundle_size_with_features(&vec!["minimal"])
}

fn get_bundle_size_with_features(features: &Vec<&str>) -> usize {
    // Mock implementation for Red Phase
    // In Green Phase, this will actually build and measure WASM bundle
    
    // Simulate current bundle sizes (will be replaced with real measurement)
    match features.join(",").as_str() {
        "minimal" => 643 * 1024,        // 643KB - current baseline
        "core-animations" => 643 * 1024, // Same as minimal currently
        "standard" => 7 * 1024 * 1024,  // 7MB - with all features
        _ => {
            // Estimate based on feature count
            let base_size = 643 * 1024;
            let feature_overhead = features.len() * 50 * 1024; // 50KB per feature
            base_size + feature_overhead
        }
    }
}

fn get_bundle_size_without_feature(feature: &str) -> usize {
    // Mock implementation - will be real in Green Phase
    match feature {
        "spring-physics" | "gesture-support" | "layout-animations" => get_minimal_bundle_size(),
        _ => get_minimal_bundle_size() + 10 * 1024, // Assume 10KB overhead
    }
}

fn get_bundle_size_with_feature(feature: &str) -> usize {
    // Mock implementation - will be real in Green Phase
    let base = get_minimal_bundle_size();
    match feature {
        "spring-physics" => base + 30 * 1024,    // 30KB for spring physics
        "gesture-support" => base + 25 * 1024,   // 25KB for gestures
        "layout-animations" => base + 40 * 1024, // 40KB for layout
        "advanced-easing" => base + 15 * 1024,   // 15KB for easing
        "leptos-integration" => base + 100 * 1024, // 100KB for Leptos
        "serde-support" => base + 80 * 1024,     // 80KB for serde
        "web-sys" => base + 200 * 1024,          // 200KB for web-sys
        _ => base + 20 * 1024,                   // Default 20KB
    }
}

struct BundleAnalysis {
    dependencies: Vec<String>,
    size_by_module: std::collections::HashMap<String, usize>,
}

impl BundleAnalysis {
    fn contains_dependency(&self, dep: &str) -> bool {
        self.dependencies.iter().any(|d| d.contains(dep))
    }
}

fn analyze_bundle_contents() -> BundleAnalysis {
    // Mock implementation for Red Phase
    // In Green Phase, this will use wasm-pack and wee_alloc tools
    BundleAnalysis {
        dependencies: vec![
            "js-sys".to_string(),
            "wasm-bindgen".to_string(),
            // Should NOT contain these in minimal build:
            // "futures", "approx", "num-traits"
        ],
        size_by_module: std::collections::HashMap::new(),
    }
}

struct BundleSizeMeasurement {
    minimal: usize,
    core_animations: usize,
    standard: usize,
}

// ============================================================================
// Real Implementation Functions (for Green Phase)
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
fn build_and_measure_wasm_bundle(features: &[&str]) -> Result<usize, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    // Create temporary Cargo.toml with specific features
    let feature_string = features.join(",");
    
    // Build with wasm-pack
    let output = Command::new("wasm-pack")
        .args(&[
            "build", 
            "--target", "web",
            "--no-typescript",
            "--features", &feature_string
        ])
        .output()?;
        
    if !output.status.success() {
        return Err(format!("Build failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    // Measure .wasm file size
    let wasm_path = Path::new("pkg").join("leptos_motion_bg.wasm");
    let metadata = fs::metadata(wasm_path)?;
    
    Ok(metadata.len() as usize)
}

#[cfg(test)]
mod mock_implementations {
    use super::*;
    
    // These will be replaced with real implementations in Green Phase
    
    #[test]
    fn test_mock_bundle_size_functions() {
        let minimal = get_minimal_bundle_size();
        let with_spring = get_bundle_size_with_feature("spring-physics");
        
        assert!(minimal > 0);
        assert!(with_spring >= minimal);
        
        web_sys::console::log_1(&format!(
            "Mock bundle sizes - Minimal: {}KB, With Spring: {}KB",
            minimal / 1024,
            with_spring / 1024
        ).into());
    }
}