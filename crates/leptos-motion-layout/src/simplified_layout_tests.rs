// TDD Tests for Simplified Layout/Scroll API
//
// This module contains tests for the new simplified layout/scroll API
// that hides complexity and provides a clean, simple interface.

use crate::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test fixture for creating a mock element
fn mock_element() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_element("div").unwrap()
}

/// Test fixture for creating a simple layout info
fn simple_layout_info() -> LayoutInfo {
    LayoutInfo::new(100.0, 200.0, 300.0, 400.0)
}

/// Test fixture for creating a simple layout config
fn simple_layout_config() -> SimplifiedLayoutConfig {
    SimplifiedLayoutConfig::new()
        .duration(0.5)
        .easing(SimplifiedEasing::EaseInOut)
        .hardware_accelerated(true)
        .enable_flip(true)
        .enable_shared_elements(true)
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_creation() {
    // Test that we can create a simplified layout manager
    let manager = SimplifiedLayoutManager::new();
    assert!(!manager.is_tracking());
    assert_eq!(manager.tracked_count(), 0);
    assert_eq!(manager.animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_with_config() {
    // Test layout manager with custom configuration
    let config = simple_layout_config();
    let manager = SimplifiedLayoutManager::with_config(config.clone());

    assert!(!manager.is_tracking());
    assert_eq!(manager.tracked_count(), 0);
    assert_eq!(manager.animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_start_tracking() {
    // Test starting layout tracking
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    let result = manager.start_tracking("test-element", &element);
    assert!(result.is_ok());
    assert!(manager.is_tracking());
    assert_eq!(manager.tracked_count(), 1);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_stop_tracking() {
    // Test stopping layout tracking
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();
    assert_eq!(manager.tracked_count(), 1);

    let result = manager.stop_tracking("test-element");
    assert!(result.is_ok());
    assert_eq!(manager.tracked_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_animate_layout_change() {
    // Test animating layout changes
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    let result = manager.animate_layout_change("test-element", &from_layout, &to_layout);
    assert!(result.is_ok());
    assert_eq!(manager.animation_count(), 1);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_flip_animation() {
    // Test FLIP animation
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    let result = manager.flip_animate("test-element", &from_layout, &to_layout);
    assert!(result.is_ok());
    assert_eq!(manager.animation_count(), 1);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_shared_element_transition() {
    // Test shared element transition
    let mut manager = SimplifiedLayoutManager::new();
    let element1 = mock_element();
    let element2 = mock_element();

    manager.start_tracking("element1", &element1).unwrap();
    manager.start_tracking("element2", &element2).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    let result =
        manager.shared_element_transition("element1", "element2", &from_layout, &to_layout);
    assert!(result.is_ok());
    assert_eq!(manager.animation_count(), 1);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_batch_operations() {
    // Test batch operations
    let mut manager = SimplifiedLayoutManager::new();
    let element1 = mock_element();
    let element2 = mock_element();

    // Batch start tracking
    let result =
        manager.batch_start_tracking(vec![("element1", &element1), ("element2", &element2)]);
    assert!(result.is_ok());
    assert_eq!(manager.tracked_count(), 2);

    // Batch animate
    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    let result = manager.batch_animate(vec![
        ("element1", &from_layout, &to_layout),
        ("element2", &from_layout, &to_layout),
    ]);
    assert!(result.is_ok());
    assert_eq!(manager.animation_count(), 2);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_get_layout_info() {
    // Test getting layout information
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let layout_info = manager.get_layout_info("test-element");
    assert!(layout_info.is_some());
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_get_animation_status() {
    // Test getting animation status
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    manager
        .animate_layout_change("test-element", &from_layout, &to_layout)
        .unwrap();

    let status = manager.get_animation_status("test-element");
    assert!(status.is_some());
    let status = status.unwrap();
    assert!(status.is_animating);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_pause_resume_animation() {
    // Test pausing and resuming animations
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    manager
        .animate_layout_change("test-element", &from_layout, &to_layout)
        .unwrap();

    // Pause animation
    let result = manager.pause_animation("test-element");
    assert!(result.is_ok());

    let status = manager.get_animation_status("test-element").unwrap();
    assert!(status.is_paused);

    // Resume animation
    let result = manager.resume_animation("test-element");
    assert!(result.is_ok());

    let status = manager.get_animation_status("test-element").unwrap();
    assert!(!status.is_paused);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_cancel_animation() {
    // Test canceling animations
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    manager
        .animate_layout_change("test-element", &from_layout, &to_layout)
        .unwrap();
    assert_eq!(manager.animation_count(), 1);

    // Cancel animation
    let result = manager.cancel_animation("test-element");
    assert!(result.is_ok());
    assert_eq!(manager.animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_clear_all() {
    // Test clearing all tracking and animations
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    manager
        .animate_layout_change("test-element", &from_layout, &to_layout)
        .unwrap();

    assert_eq!(manager.tracked_count(), 1);
    assert_eq!(manager.animation_count(), 1);

    // Clear all
    manager.clear_all();

    assert_eq!(manager.tracked_count(), 0);
    assert_eq!(manager.animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_get_performance_metrics() {
    // Test getting performance metrics
    let mut manager = SimplifiedLayoutManager::new();
    let element = mock_element();

    manager.start_tracking("test-element", &element).unwrap();

    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    manager
        .animate_layout_change("test-element", &from_layout, &to_layout)
        .unwrap();

    let metrics = manager.get_performance_metrics();
    assert!(metrics.is_some());
    let metrics = metrics.unwrap();
    assert!(metrics.total_animations >= 0);
    assert!(metrics.average_duration >= 0.0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_update_config() {
    // Test updating configuration
    let mut manager = SimplifiedLayoutManager::new();
    let config = simple_layout_config();

    manager.update_config(config.clone());

    let current_config = manager.get_config();
    assert_eq!(current_config.duration, config.duration);
    assert_eq!(current_config.easing, config.easing);
    assert_eq!(
        current_config.hardware_accelerated,
        config.hardware_accelerated
    );
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_error_handling() {
    // Test error handling for invalid operations
    let mut manager = SimplifiedLayoutManager::new();

    // Try to stop tracking non-existent element
    let result = manager.stop_tracking("non-existent");
    assert!(result.is_err());

    // Try to animate non-existent element
    let from_layout = simple_layout_info();
    let to_layout = LayoutInfo::new(200.0, 300.0, 400.0, 500.0);

    let result = manager.animate_layout_change("non-existent", &from_layout, &to_layout);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_clone() {
    // Test that simplified layout manager can be cloned
    let manager1 = SimplifiedLayoutManager::new();
    let manager2 = manager1.clone();

    assert_eq!(manager1.is_tracking(), manager2.is_tracking());
    assert_eq!(manager1.tracked_count(), manager2.tracked_count());
    assert_eq!(manager1.animation_count(), manager2.animation_count());
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_debug() {
    // Test debug formatting
    let manager = SimplifiedLayoutManager::new();
    let debug_str = format!("{:?}", manager);
    assert!(debug_str.contains("SimplifiedLayoutManager"));
    assert!(debug_str.contains("is_tracking"));
    assert!(debug_str.contains("tracked_count"));
    assert!(debug_str.contains("animation_count"));
}

#[wasm_bindgen_test]
fn test_simplified_layout_manager_default() {
    // Test default implementation
    let manager = SimplifiedLayoutManager::default();
    assert!(!manager.is_tracking());
    assert_eq!(manager.tracked_count(), 0);
    assert_eq!(manager.animation_count(), 0);
}

#[wasm_bindgen_test]
fn test_simplified_layout_config_creation() {
    // Test layout config creation
    let config = SimplifiedLayoutConfig::new();
    assert_eq!(config.duration, 0.3);
    assert_eq!(config.easing, SimplifiedEasing::EaseInOut);
    assert!(config.hardware_accelerated);
    assert!(config.enable_flip);
    assert!(config.enable_shared_elements);
}

#[wasm_bindgen_test]
fn test_simplified_layout_config_fluent_api() {
    // Test layout config fluent API
    let config = SimplifiedLayoutConfig::new()
        .duration(0.5)
        .easing(SimplifiedEasing::EaseOut)
        .hardware_accelerated(false)
        .enable_flip(false)
        .enable_shared_elements(false);

    assert_eq!(config.duration, 0.5);
    assert_eq!(config.easing, SimplifiedEasing::EaseOut);
    assert!(!config.hardware_accelerated);
    assert!(!config.enable_flip);
    assert!(!config.enable_shared_elements);
}

#[wasm_bindgen_test]
fn test_simplified_layout_config_clone() {
    // Test that simplified layout config can be cloned
    let config1 = SimplifiedLayoutConfig::new()
        .duration(0.5)
        .easing(SimplifiedEasing::EaseOut);

    let config2 = config1.clone();

    assert_eq!(config1.duration, config2.duration);
    assert_eq!(config1.easing, config2.easing);
}

#[wasm_bindgen_test]
fn test_simplified_layout_config_debug() {
    // Test debug formatting
    let config = SimplifiedLayoutConfig::new()
        .duration(0.5)
        .easing(SimplifiedEasing::EaseOut);

    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("SimplifiedLayoutConfig"));
    assert!(debug_str.contains("duration"));
    assert!(debug_str.contains("easing"));
}

#[wasm_bindgen_test]
fn test_simplified_layout_config_default() {
    // Test default implementation
    let config = SimplifiedLayoutConfig::default();
    assert_eq!(config.duration, 0.3);
    assert_eq!(config.easing, SimplifiedEasing::EaseInOut);
    assert!(config.hardware_accelerated);
    assert!(config.enable_flip);
    assert!(config.enable_shared_elements);
}
