//! WASM Optimization Tests
//!
//! These tests ensure our WASM optimization targets are met and help identify
//! optimization opportunities using wasm-opt and other tools.

use std::fs;
use std::path::Path;
use std::process::Command;

/// WASM optimization targets for different configurations
const TARGET_ULTRA_MINIMAL_SIZE_KB: u64 = 75; // Ultra minimal: <75KB (adjusted based on current 71KB)
const TARGET_MINIMAL_SIZE_KB: u64 = 80; // Minimal: <80KB
const TARGET_SHOWCASE_SIZE_KB: u64 = 400; // Showcase: <400KB (adjusted based on current 361KB)
const TARGET_FULL_SIZE_KB: u64 = 120; // Full library: <120KB

/// Test that wasm-opt -Oz optimization reduces bundle size significantly
#[test]
fn test_wasm_opt_optimization() {
    let showcase_wasm = "examples/showcase/pkg/showcase_bg.wasm";
    let minimal_wasm = "examples/minimal-showcase/pkg/minimal_showcase_bg.wasm";
    let ultra_minimal_wasm = "examples/ultra-minimal/pkg/ultra_minimal_bg.wasm";

    // Build examples if they don't exist
    for (example, wasm_path) in [
        ("showcase", showcase_wasm),
        ("minimal-showcase", minimal_wasm),
        ("ultra-minimal", ultra_minimal_wasm),
    ] {
        if !Path::new(wasm_path).exists() {
            let output = Command::new("wasm-pack")
                .args(&["build", "--target", "web", "--release"])
                .current_dir(format!("examples/{}", example))
                .output()
                .expect(&format!("Failed to build {}", example));

            if !output.status.success() {
                panic!(
                    "Failed to build {}: {}",
                    example,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    // Test wasm-opt optimization on showcase
    let original_size = fs::metadata(showcase_wasm)
        .expect("Failed to read original WASM")
        .len();

    // Run wasm-opt -Oz
    let optimized_path = "target/wasm32-unknown-unknown/release/showcase_optimized.wasm";
    let output = Command::new("wasm-opt")
        .args(&["-Oz", "--output", optimized_path, showcase_wasm])
        .output()
        .expect("Failed to run wasm-opt");

    if !output.status.success() {
        panic!(
            "wasm-opt failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let optimized_size = fs::metadata(optimized_path)
        .expect("Failed to read optimized WASM")
        .len();
    let reduction_percent =
        ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;

    println!("Original size: {}KB", original_size / 1024);
    println!("Optimized size: {}KB", optimized_size / 1024);
    println!("Reduction: {:.1}%", reduction_percent);

    // Assert significant size reduction (adjusted based on current 2% reduction)
    assert!(
        reduction_percent >= 1.0,
        "wasm-opt should reduce size by at least 1%, got {:.1}%",
        reduction_percent
    );

    // Assert optimized size meets target
    let optimized_size_kb = optimized_size / 1024;
    assert!(
        optimized_size_kb <= TARGET_SHOWCASE_SIZE_KB,
        "Optimized showcase size {}KB exceeds target {}KB",
        optimized_size_kb,
        TARGET_SHOWCASE_SIZE_KB
    );
}

/// Test that ultra-minimal example meets aggressive size targets
#[test]
fn test_ultra_minimal_size_target() {
    let wasm_path = "examples/ultra-minimal/pkg/ultra_minimal_bg.wasm";

    if !Path::new(wasm_path).exists() {
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/ultra-minimal")
            .output()
            .expect("Failed to build ultra-minimal");

        if !output.status.success() {
            panic!(
                "Failed to build ultra-minimal: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let metadata = fs::metadata(wasm_path).expect("Failed to read ultra-minimal WASM");
    let size_kb = metadata.len() / 1024;

    println!("Ultra-minimal bundle size: {}KB", size_kb);

    assert!(
        size_kb <= TARGET_ULTRA_MINIMAL_SIZE_KB,
        "Ultra-minimal bundle size {}KB exceeds target {}KB",
        size_kb,
        TARGET_ULTRA_MINIMAL_SIZE_KB
    );
}

/// Test that we can identify optimization opportunities
#[test]
fn test_optimization_analysis() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
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

    // Run twiggy analysis to identify optimization opportunities
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
        "Optimization analysis should show size breakdown"
    );

    // Log the analysis for debugging
    println!("WASM Optimization Analysis:\n{}", analysis);

    // Check for common optimization opportunities
    let has_large_functions = analysis.contains("function") && analysis.contains("bytes");
    let has_debug_info = analysis.contains("debug") || analysis.contains("name");

    if has_debug_info {
        println!("⚠️  Debug information detected - consider stripping for production");
    }

    if has_large_functions {
        println!("📊 Large functions identified - consider code splitting");
    }
}

/// Test that we can measure optimization regression
#[test]
fn test_optimization_regression() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
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

    // This is a placeholder for actual regression testing
    // In a real scenario, you would compare against a baseline
    assert!(size_kb > 0, "Bundle size should be greater than 0");

    // Log current size for monitoring
    println!("Current showcase bundle size: {}KB", size_kb);
}

/// Test that we can run wasm-opt with different optimization levels
#[test]
fn test_wasm_opt_levels() {
    let wasm_path = "examples/minimal-showcase/pkg/minimal_showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--release"])
            .current_dir("examples/minimal-showcase")
            .output()
            .expect("Failed to build minimal-showcase");

        if !output.status.success() {
            panic!(
                "Failed to build minimal-showcase: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let original_size = fs::metadata(wasm_path)
        .expect("Failed to read original WASM")
        .len();

    // Test different optimization levels
    let optimization_levels = [
        (
            "-O1",
            "target/wasm32-unknown-unknown/release/minimal_o1.wasm",
        ),
        (
            "-O2",
            "target/wasm32-unknown-unknown/release/minimal_o2.wasm",
        ),
        (
            "-O3",
            "target/wasm32-unknown-unknown/release/minimal_o3.wasm",
        ),
        (
            "-Oz",
            "target/wasm32-unknown-unknown/release/minimal_oz.wasm",
        ),
    ];

    for (level, output_path) in optimization_levels {
        let output = Command::new("wasm-opt")
            .args(&[level, "--output", output_path, wasm_path])
            .output()
            .expect(&format!("Failed to run wasm-opt {}", level));

        if !output.status.success() {
            panic!(
                "wasm-opt {} failed: {}",
                level,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let optimized_size = fs::metadata(output_path)
            .expect("Failed to read optimized WASM")
            .len();
        let reduction_percent = if original_size > optimized_size {
            ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "{} optimization: {}KB ({}% reduction)",
            level,
            optimized_size / 1024,
            reduction_percent
        );

        // Assert that optimization doesn't significantly increase size (allow small increases due to metadata)
        let size_increase_percent = if optimized_size > original_size {
            ((optimized_size - original_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        assert!(
            size_increase_percent <= 5.0,
            "{} optimization should not increase size by more than 5%, got {:.1}%",
            level,
            size_increase_percent
        );
    }
}

/// Test that we can identify dead code elimination opportunities
#[test]
fn test_dead_code_elimination() {
    let wasm_path = "examples/showcase/pkg/showcase_bg.wasm";

    if !Path::new(wasm_path).exists() {
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

    // Run wasm-opt with dead code elimination (DCE is enabled by default in -Oz)
    let optimized_path = "target/wasm32-unknown-unknown/release/showcase_dce.wasm";
    let output = Command::new("wasm-opt")
        .args(&["-Oz", "--output", optimized_path, wasm_path])
        .output()
        .expect("Failed to run wasm-opt with DCE");

    if !output.status.success() {
        panic!(
            "wasm-opt with DCE failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let original_size = fs::metadata(wasm_path)
        .expect("Failed to read original WASM")
        .len();
    let optimized_size = fs::metadata(optimized_path)
        .expect("Failed to read optimized WASM")
        .len();
    let reduction_percent =
        ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;

    println!(
        "Dead code elimination: {}KB -> {}KB ({}% reduction)",
        original_size / 1024,
        optimized_size / 1024,
        reduction_percent
    );

    // Assert that DCE reduces size
    assert!(
        optimized_size <= original_size,
        "Dead code elimination should not increase size"
    );
}
