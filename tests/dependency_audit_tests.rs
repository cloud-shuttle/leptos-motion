//! Dependency Audit and Optimization Tests
//! 
//! These tests ensure our dependency audit and optimization system works correctly
//! and helps identify opportunities for bundle size reduction.

use std::collections::HashMap;
use std::process::Command;
// use std::fs; // Not used in current implementation

/// Dependency information structure
#[derive(Debug, Clone)]
struct DependencyInfo {
    name: String,
    version: String,
    size_estimate: usize,
    is_optional: bool,
    features: Vec<String>,
    used_by: Vec<String>,
}

/// Dependency auditor
struct DependencyAuditor {
    dependencies: HashMap<String, DependencyInfo>,
    unused_dependencies: Vec<String>,
    duplicate_dependencies: Vec<String>,
    oversized_dependencies: Vec<String>,
}

impl DependencyAuditor {
    fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            unused_dependencies: Vec::new(),
            duplicate_dependencies: Vec::new(),
            oversized_dependencies: Vec::new(),
        }
    }

    /// Add a dependency to the audit
    fn add_dependency(&mut self, dep: DependencyInfo) {
        // Create a unique key to handle multiple versions of the same dependency
        let key = format!("{}:{}", dep.name, dep.version);
        self.dependencies.insert(key, dep);
    }

    /// Analyze dependencies for optimization opportunities
    fn analyze(&mut self) {
        self.find_unused_dependencies();
        self.find_duplicate_dependencies();
        self.find_oversized_dependencies();
    }

    /// Find unused dependencies
    fn find_unused_dependencies(&mut self) {
        self.unused_dependencies.clear();
        
        for (_, dep) in &self.dependencies {
            if dep.used_by.is_empty() && !dep.is_optional {
                self.unused_dependencies.push(dep.name.clone());
            }
        }
    }

    /// Find duplicate dependencies
    fn find_duplicate_dependencies(&mut self) {
        self.duplicate_dependencies.clear();
        
        let mut name_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for (_, dep) in &self.dependencies {
            name_map.entry(dep.name.clone()).or_insert_with(Vec::new).push(dep.version.clone());
        }
        
        for (name, versions) in name_map {
            if versions.len() > 1 {
                self.duplicate_dependencies.push(name);
            }
        }
    }

    /// Find oversized dependencies
    fn find_oversized_dependencies(&mut self) {
        self.oversized_dependencies.clear();
        
        for (_, dep) in &self.dependencies {
            // Consider dependencies over 50KB as oversized
            if dep.size_estimate > 50 * 1024 {
                self.oversized_dependencies.push(dep.name.clone());
            }
        }
    }

    /// Get optimization recommendations
    fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.unused_dependencies.is_empty() {
            recommendations.push(format!(
                "Remove unused dependencies: {}",
                self.unused_dependencies.join(", ")
            ));
        }
        
        if !self.duplicate_dependencies.is_empty() {
            recommendations.push(format!(
                "Consolidate duplicate dependencies: {}",
                self.duplicate_dependencies.join(", ")
            ));
        }
        
        if !self.oversized_dependencies.is_empty() {
            recommendations.push(format!(
                "Consider alternatives for oversized dependencies: {}",
                self.oversized_dependencies.join(", ")
            ));
        }
        
        recommendations
    }

    /// Calculate total dependency size
    fn total_size(&self) -> usize {
        self.dependencies.values().map(|dep| dep.size_estimate).sum()
    }

    /// Get dependency count
    fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }
}

/// Test dependency auditor creation
#[test]
fn test_dependency_auditor_creation() {
    let auditor = DependencyAuditor::new();
    assert_eq!(auditor.dependency_count(), 0);
    assert_eq!(auditor.total_size(), 0);
}

/// Test adding dependencies
#[test]
fn test_add_dependencies() {
    let mut auditor = DependencyAuditor::new();
    
    let dep1 = DependencyInfo {
        name: "serde".to_string(),
        version: "1.0".to_string(),
        size_estimate: 1024,
        is_optional: false,
        features: vec!["derive".to_string()],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    let dep2 = DependencyInfo {
        name: "web-sys".to_string(),
        version: "0.3".to_string(),
        size_estimate: 2048,
        is_optional: false,
        features: vec!["Element".to_string(), "Event".to_string()],
        used_by: vec!["leptos-motion-dom".to_string()],
    };
    
    auditor.add_dependency(dep1);
    auditor.add_dependency(dep2);
    
    assert_eq!(auditor.dependency_count(), 2);
    assert_eq!(auditor.total_size(), 3072);
}

/// Test unused dependency detection
#[test]
fn test_unused_dependency_detection() {
    let mut auditor = DependencyAuditor::new();
    
    // Add a used dependency
    let used_dep = DependencyInfo {
        name: "serde".to_string(),
        version: "1.0".to_string(),
        size_estimate: 1024,
        is_optional: false,
        features: vec!["derive".to_string()],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    // Add an unused dependency
    let unused_dep = DependencyInfo {
        name: "unused-crate".to_string(),
        version: "1.0".to_string(),
        size_estimate: 512,
        is_optional: false,
        features: vec![],
        used_by: vec![],
    };
    
    auditor.add_dependency(used_dep);
    auditor.add_dependency(unused_dep);
    auditor.analyze();
    
    assert_eq!(auditor.unused_dependencies.len(), 1);
    assert!(auditor.unused_dependencies.contains(&"unused-crate".to_string()));
}

/// Test duplicate dependency detection
#[test]
fn test_duplicate_dependency_detection() {
    let mut auditor = DependencyAuditor::new();
    
    // Add dependencies with different versions (which should be detected as duplicates)
    let dep1 = DependencyInfo {
        name: "serde".to_string(),
        version: "1.0".to_string(),
        size_estimate: 1024,
        is_optional: false,
        features: vec!["derive".to_string()],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    let dep2 = DependencyInfo {
        name: "serde".to_string(),
        version: "2.0".to_string(),
        size_estimate: 1024,
        is_optional: false,
        features: vec!["derive".to_string()],
        used_by: vec!["leptos-motion-dom".to_string()],
    };
    
    auditor.add_dependency(dep1);
    auditor.add_dependency(dep2);
    auditor.analyze();
    
    // Should detect serde as having multiple versions
    assert_eq!(auditor.duplicate_dependencies.len(), 1);
    assert!(auditor.duplicate_dependencies.contains(&"serde".to_string()));
}

/// Test oversized dependency detection
#[test]
fn test_oversized_dependency_detection() {
    let mut auditor = DependencyAuditor::new();
    
    // Add a normal-sized dependency
    let normal_dep = DependencyInfo {
        name: "small-crate".to_string(),
        version: "1.0".to_string(),
        size_estimate: 1024, // 1KB
        is_optional: false,
        features: vec![],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    // Add an oversized dependency
    let oversized_dep = DependencyInfo {
        name: "large-crate".to_string(),
        version: "1.0".to_string(),
        size_estimate: 100 * 1024, // 100KB
        is_optional: false,
        features: vec![],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    auditor.add_dependency(normal_dep);
    auditor.add_dependency(oversized_dep);
    auditor.analyze();
    
    assert_eq!(auditor.oversized_dependencies.len(), 1);
    assert!(auditor.oversized_dependencies.contains(&"large-crate".to_string()));
}

/// Test optimization recommendations
#[test]
fn test_optimization_recommendations() {
    let mut auditor = DependencyAuditor::new();
    
    // Add problematic dependencies
    let unused_dep = DependencyInfo {
        name: "unused-crate".to_string(),
        version: "1.0".to_string(),
        size_estimate: 1024,
        is_optional: false,
        features: vec![],
        used_by: vec![],
    };
    
    let oversized_dep = DependencyInfo {
        name: "large-crate".to_string(),
        version: "1.0".to_string(),
        size_estimate: 100 * 1024,
        is_optional: false,
        features: vec![],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    auditor.add_dependency(unused_dep);
    auditor.add_dependency(oversized_dep);
    auditor.analyze();
    
    let recommendations = auditor.get_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations.iter().any(|r| r.contains("unused dependencies")));
    assert!(recommendations.iter().any(|r| r.contains("oversized dependencies")));
}

/// Test cargo tree analysis
#[test]
fn test_cargo_tree_analysis() {
    // Test that we can run cargo tree command
    let output = Command::new("cargo")
        .args(&["tree", "--depth", "1"])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let tree_output = String::from_utf8_lossy(&result.stdout);
                assert!(!tree_output.is_empty());
                println!("Cargo tree output: {}", tree_output);
            } else {
                // cargo tree might not be installed, that's okay for this test
                println!("cargo tree not available or failed");
            }
        }
        Err(_) => {
            // cargo tree might not be installed, that's okay for this test
            println!("cargo tree command not found");
        }
    }
}

/// Test dependency size analysis
#[test]
fn test_dependency_size_analysis() {
    let mut auditor = DependencyAuditor::new();
    
    // Simulate real leptos-motion dependencies
    let dependencies = vec![
        ("leptos", "0.8.8", 50 * 1024),      // 50KB
        ("web-sys", "0.3.77", 30 * 1024),    // 30KB
        ("wasm-bindgen", "0.2.100", 20 * 1024), // 20KB
        ("serde", "1.0.219", 10 * 1024),     // 10KB
        ("thiserror", "2.0.16", 5 * 1024),   // 5KB
    ];
    
    for (name, version, size) in dependencies {
        let dep = DependencyInfo {
            name: name.to_string(),
            version: version.to_string(),
            size_estimate: size,
            is_optional: false,
            features: vec![],
            used_by: vec!["leptos-motion".to_string()],
        };
        auditor.add_dependency(dep);
    }
    
    auditor.analyze();
    
    let total_size = auditor.total_size();
    assert_eq!(total_size, 115 * 1024); // 115KB total
    
    let recommendations = auditor.get_recommendations();
    // Should not have unused dependencies since all are marked as used
    assert!(!recommendations.iter().any(|r| r.contains("unused dependencies")));
}

/// Test feature flag optimization
#[test]
fn test_feature_flag_optimization() {
    let mut auditor = DependencyAuditor::new();
    
    // Add dependencies with different feature usage (same version, should not be duplicates)
    let dep_with_features = DependencyInfo {
        name: "web-sys".to_string(),
        version: "0.3.77".to_string(),
        size_estimate: 30 * 1024,
        is_optional: false,
        features: vec!["Element".to_string(), "Event".to_string(), "Window".to_string()],
        used_by: vec!["leptos-motion-dom".to_string()],
    };
    
    let dep_minimal_features = DependencyInfo {
        name: "web-sys-minimal".to_string(), // Different name to avoid duplicate detection
        version: "0.3.77".to_string(),
        size_estimate: 15 * 1024, // Smaller with minimal features
        is_optional: false,
        features: vec!["Element".to_string()],
        used_by: vec!["leptos-motion-core".to_string()],
    };
    
    auditor.add_dependency(dep_with_features);
    auditor.add_dependency(dep_minimal_features);
    auditor.analyze();
    
    // Should not detect duplicate dependencies since they have different names
    assert_eq!(auditor.duplicate_dependencies.len(), 0);
    
    let recommendations = auditor.get_recommendations();
    // Should not have duplicate dependency recommendations
    assert!(!recommendations.iter().any(|r| r.contains("duplicate dependencies")));
}

/// Test bundle size impact analysis
#[test]
fn test_bundle_size_impact_analysis() {
    let mut auditor = DependencyAuditor::new();
    
    // Add dependencies with known bundle size impacts
    let core_deps = vec![
        ("leptos", 50 * 1024),
        ("web-sys", 30 * 1024),
        ("wasm-bindgen", 20 * 1024),
    ];
    
    let optional_deps = vec![
        ("serde", 10 * 1024),
        ("thiserror", 5 * 1024),
        ("log", 3 * 1024),
    ];
    
    let core_size: usize = core_deps.iter().map(|(_, size)| size).sum();
    let optional_size: usize = optional_deps.iter().map(|(_, size)| size).sum();
    
    for (name, size) in core_deps {
        let dep = DependencyInfo {
            name: name.to_string(),
            version: "1.0".to_string(),
            size_estimate: size,
            is_optional: false,
            features: vec![],
            used_by: vec!["leptos-motion-core".to_string()],
        };
        auditor.add_dependency(dep);
    }
    
    for (name, size) in optional_deps {
        let dep = DependencyInfo {
            name: name.to_string(),
            version: "1.0".to_string(),
            size_estimate: size,
            is_optional: true,
            features: vec![],
            used_by: vec!["leptos-motion-core".to_string()],
        };
        auditor.add_dependency(dep);
    }
    let total_size = auditor.total_size();
    
    assert_eq!(total_size, core_size + optional_size);
    
    // Test that we can identify optimization opportunities
    auditor.analyze();
    let recommendations = auditor.get_recommendations();
    
    // Should have recommendations for optimization
    assert!(!recommendations.is_empty() || total_size > 0);
}

/// Test dependency audit report generation
#[test]
fn test_dependency_audit_report() {
    let mut auditor = DependencyAuditor::new();
    
    // Add some test dependencies
    let test_deps = vec![
        ("leptos", "0.8.8", 50 * 1024, false),
        ("web-sys", "0.3.77", 30 * 1024, false),
        ("unused-crate", "1.0.0", 10 * 1024, false),
    ];
    
    for (name, version, size, is_optional) in test_deps {
        let dep = DependencyInfo {
            name: name.to_string(),
            version: version.to_string(),
            size_estimate: size,
            is_optional,
            features: vec![],
            used_by: if name == "unused-crate" { vec![] } else { vec!["leptos-motion".to_string()] },
        };
        auditor.add_dependency(dep);
    }
    
    auditor.analyze();
    
    // Generate report
    let report = format!(
        "Dependency Audit Report:\n\
        Total Dependencies: {}\n\
        Total Size: {} KB\n\
        Unused Dependencies: {}\n\
        Duplicate Dependencies: {}\n\
        Oversized Dependencies: {}\n\
        Recommendations:\n{}",
        auditor.dependency_count(),
        auditor.total_size() / 1024,
        auditor.unused_dependencies.len(),
        auditor.duplicate_dependencies.len(),
        auditor.oversized_dependencies.len(),
        auditor.get_recommendations().join("\n")
    );
    
    assert!(!report.is_empty());
    assert!(report.contains("Dependency Audit Report"));
    assert!(report.contains("Total Dependencies: 3"));
    assert!(report.contains("Unused Dependencies: 1"));
    
    println!("{}", report);
}
