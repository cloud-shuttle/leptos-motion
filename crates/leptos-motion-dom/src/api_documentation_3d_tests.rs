#![cfg(test)]

//! # 3D Animation API Documentation Tests
//!
//! This module serves as a living documentation for the 3D animation features
//! provided by `leptos-motion-dom`. It demonstrates how to use the 3D animation
//! capabilities, including transforms, perspective, and matrix3d operations.
//!
//! The tests here are primarily for documentation and API contract verification,
//! ensuring that the public API remains consistent and well-understood.
//!
//! ## Key Components:
//!
//! - `ReactiveMotionDiv`: The core component for applying animations.
//! - `AnimationValue`: Enum for various animation property types (Number, Pixels, Degrees, String).
//! - `AnimationTarget`: A `HashMap<String, AnimationValue>` representing the target state of properties.
//! - `Transition`: Configuration for animation timing, easing, and repetition.
//! - `Transform3D`, `Perspective3D`, `Animation3D`: 3D animation utility structs.
//!
//! ---

use crate::animation_3d_implementation::{Animation3D, Perspective3D, Transform3D, utils};
use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// API Documentation Test for 3D Animation System
///
/// This test serves as living documentation for the 3D animation API.
/// It demonstrates all available features and their usage patterns.
#[test]
fn test_3d_animation_api_documentation() {
    // ============================================================================
    // 3D TRANSFORM API DOCUMENTATION
    // ============================================================================

    // Basic 3D Transform Creation
    let transform = Transform3D::default()
        .rotate_x(45.0) // Rotate around X-axis (degrees)
        .rotate_y(90.0) // Rotate around Y-axis (degrees)
        .rotate_z(180.0) // Rotate around Z-axis (degrees)
        .translate_x(100.0) // Translate along X-axis (pixels)
        .translate_y(50.0) // Translate along Y-axis (pixels)
        .translate_z(200.0) // Translate along Z-axis (pixels)
        .scale_x(2.0) // Scale along X-axis
        .scale_y(0.5) // Scale along Y-axis
        .scale_z(1.2); // Scale along Z-axis

    // Convert to AnimationTarget for use with ReactiveMotionDiv
    let animation_target = transform.to_animation_target();

    // Verify all transform properties are correctly set
    assert_eq!(
        animation_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        animation_target.get("rotateY"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        animation_target.get("rotateZ"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        animation_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        animation_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        animation_target.get("translateZ"),
        Some(&AnimationValue::Number(200.0))
    );
    assert_eq!(
        animation_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        animation_target.get("scaleY"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        animation_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.2))
    );
}

/// Test 3D Perspective Configuration
#[test]
fn test_3d_perspective_api_documentation() {
    // ============================================================================
    // 3D PERSPECTIVE API DOCUMENTATION
    // ============================================================================

    // Basic 3D Perspective Creation
    let perspective = Perspective3D::default()
        .perspective("800px") // Perspective distance (pixels)
        .perspective_origin("center center") // Perspective origin
        .transform_style("preserve-3d"); // Transform style

    // Convert to AnimationTarget for use with ReactiveMotionDiv
    let animation_target = perspective.to_animation_target();

    // Verify all perspective properties are correctly set
    assert_eq!(
        animation_target.get("perspective"),
        Some(&AnimationValue::String("800px".to_string()))
    );
    assert_eq!(
        animation_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );
    assert_eq!(
        animation_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );

    // Advanced perspective configurations
    let advanced_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("top left");

    let advanced_target = advanced_perspective.to_animation_target();
    assert_eq!(
        advanced_target.get("perspective"),
        Some(&AnimationValue::String("1000px".to_string()))
    );
    assert_eq!(
        advanced_target.get("perspective-origin"),
        Some(&AnimationValue::String("top left".to_string()))
    );

    // Dynamic perspective changes
    let dynamic_perspective = Perspective3D::default()
        .perspective("500px")
        .perspective_origin("bottom right");

    let dynamic_target = dynamic_perspective.to_animation_target();
    assert_eq!(
        dynamic_target.get("perspective"),
        Some(&AnimationValue::String("500px".to_string()))
    );
    assert_eq!(
        dynamic_target.get("perspective-origin"),
        Some(&AnimationValue::String("bottom right".to_string()))
    );
}

/// Test 3D Animation Configuration
#[test]
fn test_3d_animation_configuration_api_documentation() {
    // ============================================================================
    // 3D ANIMATION CONFIGURATION API DOCUMENTATION
    // ============================================================================

    // Create a complete 3D animation with transform and perspective
    let animation = Animation3D::new()
        .transform(Transform3D::default().rotate_x(90.0).translate_z(100.0))
        .perspective(
            Perspective3D::default()
                .perspective("1000px")
                .transform_style("preserve-3d"),
        )
        .transition(Transition {
            duration: Some(2.0),
            delay: Some(0.5),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Count(3),
            stagger: None,
        });

    // Convert to AnimationTarget for use with ReactiveMotionDiv
    let animation_target = animation.to_animation_target();

    // Verify animation properties
    assert_eq!(
        animation_target.get("rotateX"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        animation_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        animation_target.get("perspective"),
        Some(&AnimationValue::String("1000px".to_string()))
    );
    assert_eq!(
        animation_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );

    // Verify transition configuration
    assert_eq!(animation.transition.duration, Some(2.0));
    assert_eq!(animation.transition.delay, Some(0.5));
    assert_eq!(animation.transition.ease, Easing::EaseInOut);
    assert_eq!(animation.transition.repeat, RepeatConfig::Count(3));
}

/// Test 3D Animation Convenience Methods
#[test]
fn test_3d_animation_convenience_methods_api_documentation() {
    // ============================================================================
    // 3D ANIMATION CONVENIENCE METHODS API DOCUMENTATION
    // ============================================================================

    // Create 3D rotation animation using convenience method
    let rotate_animation = Animation3D::rotate_3d(45.0, 90.0, 180.0);
    let rotate_target = rotate_animation.to_animation_target();

    assert_eq!(
        rotate_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        rotate_target.get("rotateY"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        rotate_target.get("rotateZ"),
        Some(&AnimationValue::Number(180.0))
    );

    // Create 3D translation animation using convenience method
    let translate_animation = Animation3D::translate_3d(100.0, 200.0, 300.0);
    let translate_target = translate_animation.to_animation_target();

    assert_eq!(
        translate_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        translate_target.get("translateY"),
        Some(&AnimationValue::Number(200.0))
    );
    assert_eq!(
        translate_target.get("translateZ"),
        Some(&AnimationValue::Number(300.0))
    );

    // Create 3D scale animation using convenience method
    let scale_animation = Animation3D::scale_3d(1.5, 2.0, 0.5);
    let scale_target = scale_animation.to_animation_target();

    assert_eq!(
        scale_target.get("scaleX"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        scale_target.get("scaleY"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        scale_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.5))
    );

    // Create 3D perspective animation using convenience method
    let perspective_animation = Animation3D::with_perspective("1200px");
    let perspective_target = perspective_animation.to_animation_target();

    assert_eq!(
        perspective_target.get("perspective"),
        Some(&AnimationValue::String("1200px".to_string()))
    );
    assert_eq!(
        perspective_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );
}

/// Test 3D Utility Functions
#[test]
fn test_3d_utility_functions_api_documentation() {
    // ============================================================================
    // 3D UTILITY FUNCTIONS API DOCUMENTATION
    // ============================================================================

    // Matrix3D utility function
    let matrix3d_string = utils::matrix3d(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    assert_eq!(
        matrix3d_string,
        "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)"
    );

    // Rotate3D utility function
    let rotate3d_string = utils::rotate3d(1.0, 1.0, 1.0, 45.0);
    assert_eq!(rotate3d_string, "rotate3d(1, 1, 1, 45deg)");

    // Translate3D utility function
    let translate3d_string = utils::translate3d(100.0, 200.0, 300.0);
    assert_eq!(translate3d_string, "translate3d(100px, 200px, 300px)");

    // Scale3D utility function
    let scale3d_string = utils::scale3d(1.5, 2.0, 0.5);
    assert_eq!(scale3d_string, "scale3d(1.5, 2, 0.5)");

    // Matrix3D animation target utility
    let matrix3d_target = utils::matrix3d_animation_target(
        0.707, -0.707, 0.0, 0.0, 0.707, 0.707, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 50.0, 50.0, 0.0, 1.0,
    );
    assert_eq!(
        matrix3d_target.get("matrix3d"),
        Some(&AnimationValue::String(
            "matrix3d(0.707, -0.707, 0, 0, 0.707, 0.707, 0, 0, 0, 0, 1, 0, 50, 50, 0, 1)"
                .to_string()
        ))
    );

    // Rotate3D animation target utility
    let rotate3d_target = utils::rotate3d_animation_target(1.0, 1.0, 1.0, 45.0);
    assert_eq!(
        rotate3d_target.get("rotate3d"),
        Some(&AnimationValue::String(
            "rotate3d(1, 1, 1, 45deg)".to_string()
        ))
    );

    // Translate3D animation target utility
    let translate3d_target = utils::translate3d_animation_target(100.0, 200.0, 300.0);
    assert_eq!(
        translate3d_target.get("translate3d"),
        Some(&AnimationValue::String(
            "translate3d(100px, 200px, 300px)".to_string()
        ))
    );

    // Scale3D animation target utility
    let scale3d_target = utils::scale3d_animation_target(1.5, 2.0, 0.5);
    assert_eq!(
        scale3d_target.get("scale3d"),
        Some(&AnimationValue::String("scale3d(1.5, 2, 0.5)".to_string()))
    );
}

/// Test 3D Animation with ReactiveMotionDiv Integration
#[test]
fn test_3d_animation_reactive_motion_div_integration() {
    // ============================================================================
    // 3D ANIMATION WITH REACTIVEMOTIONDIV INTEGRATION API DOCUMENTATION
    // ============================================================================

    // Create a 3D animation using the builder pattern
    let cube_animation = Transform3D::default()
        .rotate_x(45.0)
        .rotate_y(45.0)
        .rotate_z(0.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .translate_z(50.0);

    let cube_target = cube_animation.to_animation_target();

    // Verify cube animation properties
    assert_eq!(
        cube_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        cube_target.get("rotateY"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        cube_target.get("rotateZ"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        cube_target.get("translateZ"),
        Some(&AnimationValue::Number(50.0))
    );

    // Create a zoom effect animation
    let zoom_effect = Transform3D::default()
        .scale_x(2.0)
        .scale_y(2.0)
        .scale_z(2.0);

    let zoom_target = zoom_effect.to_animation_target();

    // Verify zoom effect properties
    assert_eq!(
        zoom_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        zoom_target.get("scaleY"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        zoom_target.get("scaleZ"),
        Some(&AnimationValue::Number(2.0))
    );

    // Create a perspective change animation
    let perspective_change = Perspective3D::default()
        .perspective("500px")
        .perspective_origin("center center")
        .transform_style("preserve-3d");

    let perspective_target = perspective_change.to_animation_target();

    // Verify perspective change properties
    assert_eq!(
        perspective_target.get("perspective"),
        Some(&AnimationValue::String("500px".to_string()))
    );
    assert_eq!(
        perspective_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );
    assert_eq!(
        perspective_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );
}

/// Test 3D Animation Performance Considerations
#[test]
fn test_3d_animation_performance_api_documentation() {
    // ============================================================================
    // 3D ANIMATION PERFORMANCE API DOCUMENTATION
    // ============================================================================

    // Create a performance-optimized 3D animation
    let performance_animation = Transform3D::default().rotate_x(360.0).translate_z(100.0);

    let performance_target = performance_animation.to_animation_target();

    // Add will-change for hardware acceleration
    let mut optimized_target = performance_target;
    optimized_target.insert(
        "will-change".to_string(),
        AnimationValue::String("transform".to_string()),
    );

    // Verify performance optimization properties
    assert_eq!(
        optimized_target.get("rotateX"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        optimized_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        optimized_target.get("will-change"),
        Some(&AnimationValue::String("transform".to_string()))
    );

    // Create a complex 3D animation with multiple transforms
    let complex_animation = Transform3D::default()
        .rotate_x(45.0)
        .rotate_y(60.0)
        .rotate_z(30.0)
        .translate_x(100.0)
        .translate_y(50.0)
        .translate_z(200.0)
        .scale_x(1.2)
        .scale_y(1.2)
        .scale_z(1.2);

    let complex_target = complex_animation.to_animation_target();

    // Verify all complex animation properties
    assert_eq!(
        complex_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        complex_target.get("rotateY"),
        Some(&AnimationValue::Number(60.0))
    );
    assert_eq!(
        complex_target.get("rotateZ"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        complex_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        complex_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        complex_target.get("translateZ"),
        Some(&AnimationValue::Number(200.0))
    );
    assert_eq!(
        complex_target.get("scaleX"),
        Some(&AnimationValue::Number(1.2))
    );
    assert_eq!(
        complex_target.get("scaleY"),
        Some(&AnimationValue::Number(1.2))
    );
    assert_eq!(
        complex_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.2))
    );
}

/// Test 3D Animation Signal-Based Reactivity
#[test]
fn test_3d_animation_signal_reactivity_api_documentation() {
    // ============================================================================
    // 3D ANIMATION SIGNAL-BASED REACTIVITY API DOCUMENTATION
    // ============================================================================

    // Create a signal-based 3D animation
    let (rotation_angle, _set_rotation_angle) = signal(0.0);

    // Create animation target based on signal
    let signal_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "rotateY".to_string(),
            AnimationValue::Number(rotation_angle.get()),
        );
        target
    };

    // Test signal-based animation
    let animation_result = signal_animation();
    assert_eq!(
        animation_result.get("rotateY"),
        Some(&AnimationValue::Number(0.0))
    );

    // Create a more complex signal-based 3D animation
    let (is_rotating, _set_is_rotating) = signal(false);
    let (scale_factor, _set_scale_factor) = signal(1.0);

    let complex_signal_animation = move || {
        let mut target = HashMap::new();
        if is_rotating.get() {
            target.insert("rotateX".to_string(), AnimationValue::Number(360.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(180.0));
        } else {
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
        }
        target.insert(
            "scaleX".to_string(),
            AnimationValue::Number(scale_factor.get()),
        );
        target.insert(
            "scaleY".to_string(),
            AnimationValue::Number(scale_factor.get()),
        );
        target.insert(
            "scaleZ".to_string(),
            AnimationValue::Number(scale_factor.get()),
        );
        target
    };

    // Test complex signal-based animation
    let complex_result = complex_signal_animation();
    assert_eq!(
        complex_result.get("rotateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        complex_result.get("rotateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        complex_result.get("scaleX"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        complex_result.get("scaleY"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        complex_result.get("scaleZ"),
        Some(&AnimationValue::Number(1.0))
    );
}
