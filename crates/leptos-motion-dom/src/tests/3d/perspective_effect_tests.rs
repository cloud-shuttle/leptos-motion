#![cfg(test)]

//! # 3D Perspective Effect Tests
//!
//! This module tests 3D perspective effect features including:
//! - Perspective transformations
//! - Perspective origin settings
//! - Transform style configurations
//! - Dynamic perspective changes

use crate::animation_3d_implementation::{Perspective3D, Transform3D};
use leptos_motion_core::AnimationValue;

/// Test 3D Perspective Effects
///
/// Tests various perspective effects and configurations
/// for creating depth and 3D visual impact.
#[test]
fn test_3d_perspective_effects() {
    // ============================================================================
    // 3D PERSPECTIVE EFFECTS
    // ============================================================================

    // Create a basic perspective effect
    let basic_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("center center")
        .transform_style("preserve-3d");

    let basic_target = basic_perspective.to_animation_target();

    // Verify basic perspective properties
    assert_eq!(
        basic_target.get("perspective"),
        Some(&AnimationValue::String("1000px".to_string()))
    );
    assert_eq!(
        basic_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );
    assert_eq!(
        basic_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );
}

/// Test Wide-Angle Perspective
///
/// Tests wide-angle perspective effects for dramatic
/// 3D visual impact.
#[test]
fn test_wide_angle_perspective() {
    // Create a wide-angle perspective effect
    let wide_angle_perspective = Perspective3D::default()
        .perspective("2000px")
        .perspective_origin("center center")
        .transform_style("preserve-3d");

    let wide_angle_target = wide_angle_perspective.to_animation_target();

    // Verify wide-angle perspective properties
    assert_eq!(
        wide_angle_target.get("perspective"),
        Some(&AnimationValue::String("2000px".to_string()))
    );
}

/// Test Dynamic Perspective
///
/// Tests dynamic perspective changes that can be
/// animated over time.
#[test]
fn test_dynamic_perspective() {
    // Create a dynamic perspective that changes over time
    let dynamic_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("top left")
        .transform_style("preserve-3d");

    let dynamic_target = dynamic_perspective.to_animation_target();

    // Verify dynamic perspective properties
    assert_eq!(
        dynamic_target.get("perspective"),
        Some(&AnimationValue::String("1000px".to_string()))
    );
    assert_eq!(
        dynamic_target.get("perspective-origin"),
        Some(&AnimationValue::String("top left".to_string()))
    );
}

/// Test Perspective Origin Variations
///
/// Tests different perspective origin settings for
/// various visual effects.
#[test]
fn test_perspective_origin_variations() {
    // Test center origin
    let center_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("center center");

    let center_target = center_perspective.to_animation_target();
    assert_eq!(
        center_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );

    // Test top-left origin
    let top_left_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("top left");

    let top_left_target = top_left_perspective.to_animation_target();
    assert_eq!(
        top_left_target.get("perspective-origin"),
        Some(&AnimationValue::String("top left".to_string()))
    );

    // Test bottom-right origin
    let bottom_right_perspective = Perspective3D::default()
        .perspective("1000px")
        .perspective_origin("bottom right");

    let bottom_right_target = bottom_right_perspective.to_animation_target();
    assert_eq!(
        bottom_right_target.get("perspective-origin"),
        Some(&AnimationValue::String("bottom right".to_string()))
    );
}

/// Test Transform Style Variations
///
/// Tests different transform style settings for
/// 3D rendering behavior.
#[test]
fn test_transform_style_variations() {
    // Test preserve-3d style
    let preserve_3d = Perspective3D::default()
        .perspective("1000px")
        .transform_style("preserve-3d");

    let preserve_target = preserve_3d.to_animation_target();
    assert_eq!(
        preserve_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );

    // Test flat style
    let flat_style = Perspective3D::default()
        .perspective("1000px")
        .transform_style("flat");

    let flat_target = flat_style.to_animation_target();
    assert_eq!(
        flat_target.get("transform-style"),
        Some(&AnimationValue::String("flat".to_string()))
    );
}

/// Test Perspective with Transform Combinations
///
/// Tests combining perspective effects with other
/// 3D transformations.
#[test]
fn test_perspective_with_transforms() {
    // Create a perspective with transform combination
    let perspective_transform = Transform3D::default()
        .translate_x(50.0)
        .translate_y(25.0)
        .translate_z(100.0)
        .rotate_x(45.0)
        .rotate_y(30.0)
        .rotate_z(15.0)
        .scale_x(1.2)
        .scale_y(0.8)
        .scale_z(1.1);

    let transform_target = perspective_transform.to_animation_target();

    // Verify transform properties
    assert_eq!(
        transform_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        transform_target.get("translateY"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        transform_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        transform_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        transform_target.get("rotateY"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        transform_target.get("rotateZ"),
        Some(&AnimationValue::Number(15.0))
    );
    assert_eq!(
        transform_target.get("scaleX"),
        Some(&AnimationValue::Number(1.2))
    );
    assert_eq!(
        transform_target.get("scaleY"),
        Some(&AnimationValue::Number(0.8))
    );
    assert_eq!(
        transform_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.1))
    );
}
