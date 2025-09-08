//! TDD Test Suite for Leptos Motion Comprehensive Demo
//! 
//! This module contains all tests that drive the development of the demo.
//! Tests are organized by feature area and written before implementation.

pub mod basic_animations;
pub mod gesture_interactions;
pub mod layout_animations;
pub mod scroll_effects;
pub mod component_showcase;
pub mod integration_tests;

/// Test utilities and helpers
pub mod test_utils {
    use leptos::*;
    use leptos_motion_core::*;
    
    /// Create a test component with motion capabilities
    pub fn create_test_component() -> impl Fn() -> impl IntoView {
        move || {
            view! {
                <div class="test-component">
                    "Test Component"
                </div>
            }
        }
    }
    
    /// Assert that an animation value is within expected range
    pub fn assert_animation_value(value: f64, expected: f64, tolerance: f64) {
        assert!(
            (value - expected).abs() < tolerance,
            "Animation value {} not within tolerance {} of expected {}",
            value,
            tolerance,
            expected
        );
    }
    
    /// Create a test animation configuration
    pub fn create_test_animation_config() -> AnimationConfig {
        AnimationConfig {
            duration: 1000.0,
            easing: EasingFunction::EaseInOut,
            delay: 0.0,
            repeat: false,
            ..Default::default()
        }
    }
}
