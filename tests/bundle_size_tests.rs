//! Bundle Size Optimization Tests
//!
//! These tests ensure our bundle size targets are met and help identify
//! what's contributing to bundle bloat.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Bundle size targets for different configurations
const TARGET_CORE_SIZE_KB: u64 = 30; // Core only: <30KB
const TARGET_FULL_SIZE_KB: u64 = 50; // Full library: <50KB
const TARGET_SHOWCASE_SIZE_KB: u64 = 100; // Showcase example: <100KB
const TARGET_MINIMAL_SIZE_KB: u64 = 100; // Minimal showcase: <100KB

/// Test that core bundle size is within target
#[test]
fn test_core_bundle_size() {
    let wasm_path = "target/wasm32-unknown-unknown/release/leptos_motion_core.wasm";

    if !Path::new(wasm_path).exists() {
        // Build core crate for WASM
        let output = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--target",
                "wasm32-unknown-unknown",
                "-p",
                "leptos-motion-core",
            ])
            .output()
            .expect("Failed to build core crate");

        if !output.status.success() {
            panic!(
                "Failed to build core crate: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let metadata = fs::metadata(wasm_path).expect("Failed to read WASM file");
    let size_kb = metadata.len() / 1024;

    assert!(
        size_kb <= TARGET_CORE_SIZE_KB,
        "Core bundle size {}KB exceeds target {}KB",
        size_kb,
        TARGET_CORE_SIZE_KB
    );
}

/// Test that showcase example bundle size is within target
#[test]
fn test_showcase_bundle_size() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        // Build showcase example
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/showcase")
            .output()
            .expect("Failed to build showcase");

        if !output.status.success() {
            panic!(
                "Failed to build showcase: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let metadata = fs::metadata(wasm_path).expect("Failed to read WASM file");
    let size_kb = metadata.len() / 1024;

    assert!(
        size_kb <= TARGET_SHOWCASE_SIZE_KB,
        "Showcase bundle size {}KB exceeds target {}KB",
        size_kb,
        TARGET_SHOWCASE_SIZE_KB
    );
}

/// Test that we can identify the largest contributors to bundle size
#[test]
fn test_bundle_analysis() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        // Build showcase example first
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/showcase")
            .output()
            .expect("Failed to build showcase");

        if !output.status.success() {
            panic!(
                "Failed to build showcase: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Run twiggy analysis
    let output = Command::new("twiggy")
        .args(&["top", wasm_path])
        .output()
        .expect("Failed to run twiggy");

    if !output.status.success() {
        panic!(
            "Failed to run twiggy: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let analysis = String::from_utf8_lossy(&output.stdout);

    // Check that we can identify large contributors
    assert!(
        analysis.contains("Shallow Bytes"),
        "Bundle analysis should show size breakdown"
    );

    // Log the analysis for debugging
    println!("Bundle Analysis:\n{}", analysis);
}

/// Test that minimal showcase bundle size is within target
#[test]
fn test_minimal_showcase_bundle_size() {
    let wasm_path = "examples/minimal-showcase/pkg/minimal_showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        // Build minimal showcase example for WASM
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/minimal-showcase")
            .output()
            .expect("Failed to build minimal showcase example");

        if !output.status.success() {
            panic!(
                "Failed to build minimal showcase example: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let metadata = fs::metadata(wasm_path).expect("Failed to read minimal showcase WASM file");
    let size_kb = metadata.len() / 1024;

    println!("Minimal showcase bundle size: {}KB", size_kb);

    assert!(
        size_kb <= TARGET_MINIMAL_SIZE_KB,
        "Minimal showcase bundle size {}KB exceeds target {}KB",
        size_kb,
        TARGET_MINIMAL_SIZE_KB
    );
}

/// Test that feature flags reduce bundle size
#[test]
fn test_feature_flags_bundle_size() {
    // This test will be implemented once we add feature flags
    // For now, it's a placeholder to ensure the test structure is in place

    // TODO: Build with minimal features and verify size reduction
    // TODO: Build with full features and compare sizes
    // TODO: Verify that optional components are not included when disabled

    assert!(true, "Feature flags test placeholder");
}

/// Test that tree shaking is working properly
#[test]
fn test_tree_shaking() {
    // This test will be implemented once we add proper tree shaking
    // For now, it's a placeholder to ensure the test structure is in place

    // TODO: Build with unused code and verify it's not included
    // TODO: Verify that dead code elimination is working
    // TODO: Check that unused dependencies are not included

    assert!(true, "Tree shaking test placeholder");
}

/// Test that we can measure bundle size regression
#[test]
fn test_bundle_size_regression() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        // Build showcase example
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/showcase")
            .output()
            .expect("Failed to build showcase");

        if !output.status.success() {
            panic!(
                "Failed to build showcase: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let metadata = fs::metadata(wasm_path).expect("Failed to read WASM file");
    let size_kb = metadata.len() / 1024;

    // Log current size for regression tracking
    println!("Current bundle size: {}KB", size_kb);

    // For now, just ensure we can measure the size
    // In the future, we can compare against a baseline
    assert!(size_kb > 0, "Bundle size should be measurable");
}

/// Test that we can identify specific bloat sources
#[test]
fn test_identify_bundle_bloat() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        // Build showcase example
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/showcase")
            .output()
            .expect("Failed to build showcase");

        if !output.status.success() {
            panic!(
                "Failed to build showcase: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Run twiggy dominators analysis
    let output = Command::new("twiggy")
        .args(&["dominators", wasm_path])
        .output()
        .expect("Failed to run twiggy dominators");

    if !output.status.success() {
        panic!(
            "Failed to run twiggy dominators: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let analysis = String::from_utf8_lossy(&output.stdout);

    // Check for common bloat sources
    let bloat_indicators = [
        "table[0]",  // Function table (often large)
        "elem[3]",   // Element table (often large)
        "code[325]", // Large code sections
        "data[9]",   // Large data sections
    ];

    for indicator in &bloat_indicators {
        if analysis.contains(indicator) {
            println!("Found potential bloat source: {}", indicator);
        }
    }

    // Log the analysis for debugging
    println!("Bundle Bloat Analysis:\n{}", analysis);

    // For now, just ensure we can run the analysis
    assert!(analysis.len() > 0, "Bundle analysis should produce output");
}
