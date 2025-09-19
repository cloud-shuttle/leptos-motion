//! Demo Validation Tests using TDD approach
//! 
//! These tests define the expected behavior of our demos and ensure they work correctly.

use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

/// Test that comprehensive-showcase serves correctly
#[tokio::test]
async fn test_comprehensive_showcase_serves_correctly() {
    // Get the project root directory
    let project_root = std::env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();
    
    // Arrange: Start the server from the dist directory
    let mut server = Command::new("python3")
        .args(&["-m", "http.server", "8083"])
        .current_dir(project_root.join("examples/comprehensive-showcase/dist"))
        .spawn()
        .expect("Failed to start server");

    // Give server time to start
    sleep(Duration::from_secs(2)).await;

    // Act: Test that the server responds
    let response = reqwest::get("http://localhost:8083/")
        .await
        .expect("Failed to make request");

    // Assert: Server should respond with 200 OK
    assert_eq!(response.status(), 200, "Server should respond with 200 OK");

    // Act: Test that WASM files are accessible
    let wasm_response = reqwest::get("http://localhost:8083/comprehensive_showcase.js")
        .await
        .expect("Failed to make WASM request");

    // Assert: WASM files should be accessible
    assert_eq!(wasm_response.status(), 200, "WASM JavaScript file should be accessible");

    // Act: Test that WASM binary is accessible
    let wasm_binary_response = reqwest::get("http://localhost:8083/comprehensive_showcase_bg.wasm")
        .await
        .expect("Failed to make WASM binary request");

    // Assert: WASM binary should be accessible
    assert_eq!(wasm_binary_response.status(), 200, "WASM binary file should be accessible");

    // Cleanup: Kill the server
    let _ = server.kill();
}

/// Test that puzzle-game-demo serves correctly
#[tokio::test]
async fn test_puzzle_game_demo_serves_correctly() {
    // Get the project root directory
    let project_root = std::env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();
    
    // Arrange: Start the server from the dist directory
    let mut server = Command::new("python3")
        .args(&["-m", "http.server", "8082"])
        .current_dir(project_root.join("examples/puzzle-game-demo/dist"))
        .spawn()
        .expect("Failed to start server");

    // Give server time to start
    sleep(Duration::from_secs(2)).await;

    // Act: Test that the server responds
    let response = reqwest::get("http://localhost:8082/")
        .await
        .expect("Failed to make request");

    // Assert: Server should respond with 200 OK
    assert_eq!(response.status(), 200, "Puzzle game server should respond with 200 OK");

    // Act: Test that WASM files are accessible
    let wasm_response = reqwest::get("http://localhost:8082/puzzle_game_demo.js")
        .await
        .expect("Failed to make WASM request");

    // Assert: WASM files should be accessible
    assert_eq!(wasm_response.status(), 200, "Puzzle game WASM JavaScript file should be accessible");

    // Cleanup: Kill the server
    let _ = server.kill();
}

/// Test that demos have required files
#[test]
fn test_comprehensive_showcase_has_required_files() {
    // Get the project root directory
    let project_root = std::env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();
    
    // Assert: Required files should exist
    let index_path = project_root.join("examples/comprehensive-showcase/dist/index.html");
    assert!(index_path.exists(), 
            "comprehensive-showcase should have index.html at {:?}", index_path);
    
    let js_path = project_root.join("examples/comprehensive-showcase/dist/comprehensive_showcase.js");
    assert!(js_path.exists(), 
            "comprehensive-showcase should have JavaScript file at {:?}", js_path);
    
    let wasm_path = project_root.join("examples/comprehensive-showcase/dist/comprehensive_showcase_bg.wasm");
    assert!(wasm_path.exists(), 
            "comprehensive-showcase should have WASM binary at {:?}", wasm_path);
}

#[test]
fn test_puzzle_game_demo_has_required_files() {
    // Get the project root directory
    let project_root = std::env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();
    
    // Assert: Required files should exist
    let index_path = project_root.join("examples/puzzle-game-demo/dist/index.html");
    assert!(index_path.exists(), 
            "puzzle-game-demo should have index.html at {:?}", index_path);
    
    let js_path = project_root.join("examples/puzzle-game-demo/dist/puzzle_game_demo.js");
    assert!(js_path.exists(), 
            "puzzle-game-demo should have JavaScript file at {:?}", js_path);
    
    let wasm_path = project_root.join("examples/puzzle-game-demo/dist/puzzle_game_demo_bg.wasm");
    assert!(wasm_path.exists(), 
            "puzzle-game-demo should have WASM binary at {:?}", wasm_path);
}

/// Test that broken demos are identified
#[test]
fn test_identify_broken_demos() {
    let broken_demos = vec![
        "examples/motion-showcase-working",
        "examples/motion-showcase", 
        "examples/showcase",
        "examples/phase2-reactive-demo",
    ];

    for demo in broken_demos {
        let dist_path = format!("{}/dist", demo);
        if std::path::Path::new(&dist_path).exists() {
            // Check if it has WASM files
            let wasm_files = std::fs::read_dir(&dist_path)
                .unwrap_or_else(|_| panic!("Cannot read dist directory for {}", demo))
                .filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().ends_with(".wasm"));
            
            assert!(!wasm_files, "Demo {} should be identified as broken (has dist but no WASM)", demo);
        }
    }
}

/// Test that duplicate demos have been successfully deleted
#[test]
fn test_duplicate_demos_deleted() {
    let duplicate_demos = vec![
        "examples/motion-showcase-working",
        "examples/motion-showcase",
        "examples/showcase", 
        "examples/simple-comprehensive-demo",
        "examples/simple-wasm-demo",
        "examples/simple-working-demo",
        "examples/ultra-minimal",
    ];

    for demo in duplicate_demos {
        assert!(!std::path::Path::new(demo).exists(), "Duplicate demo {} should have been deleted", demo);
    }
}
