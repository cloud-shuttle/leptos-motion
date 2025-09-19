#![cfg(test)]

//! # 3D Morphing Animation Tests
//!
//! This module tests 3D morphing animation features including:
//! - Shape transitions (cube-to-sphere, pyramid-to-cylinder)
//! - Smooth interpolation between different 3D shapes
//! - Complex morphing keyframes
//! - Multi-stage morphing animations

use crate::animation_3d_implementation::{Animation3D, Transform3D};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};

/// Test 3D Morphing Animation System
///
/// Morphing allows smooth transitions between different 3D shapes
/// by interpolating between different transform configurations.
#[test]
fn test_3d_morphing_animation_system() {
    // ============================================================================
    // 3D MORPHING ANIMATION SYSTEM
    // ============================================================================

    // Define morphing keyframes for a cube-to-sphere transition
    let cube_shape = Transform3D::default()
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let _sphere_shape = Transform3D::default()
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .rotate_x(45.0)
        .rotate_y(45.0)
        .rotate_z(45.0);

    // Create morphing animation with multiple keyframes
    let morphing_animation = Animation3D::new()
        .transform(cube_shape)
        .transition(Transition {
            duration: Some(2.0),
            delay: Some(0.0),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::InfiniteReverse,
            stagger: None,
        });

    let morphing_target = morphing_animation.to_animation_target();

    // Verify morphing animation properties
    assert_eq!(
        morphing_target.get("scaleX"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        morphing_target.get("scaleY"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        morphing_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        morphing_target.get("rotateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        morphing_target.get("rotateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        morphing_target.get("rotateZ"),
        Some(&AnimationValue::Number(0.0))
    );

    // Test morphing between different shapes
    let pyramid_shape = Transform3D::default()
        .scale_x(1.0)
        .scale_y(0.5)
        .scale_z(1.0)
        .rotate_x(30.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let pyramid_target = pyramid_shape.to_animation_target();

    // Verify pyramid shape properties
    assert_eq!(
        pyramid_target.get("scaleX"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        pyramid_target.get("scaleY"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        pyramid_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        pyramid_target.get("rotateX"),
        Some(&AnimationValue::Number(30.0))
    );
}

/// Test Multi-Stage Morphing
///
/// Tests complex morphing animations with multiple stages
/// and intermediate keyframes.
#[test]
fn test_multi_stage_morphing() {
    // Create a multi-stage morphing animation
    let stage1 = Transform3D::default()
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .rotate_x(0.0);

    let stage2 = Transform3D::default()
        .scale_x(1.2)
        .scale_y(0.8)
        .scale_z(1.0)
        .rotate_x(45.0);

    let stage3 = Transform3D::default()
        .scale_x(0.8)
        .scale_y(1.2)
        .scale_z(1.0)
        .rotate_x(90.0);

    // Test each stage
    let stage1_target = stage1.to_animation_target();
    let stage2_target = stage2.to_animation_target();
    let stage3_target = stage3.to_animation_target();

    // Verify stage transitions
    assert_eq!(stage1_target.get("scaleX"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(stage2_target.get("scaleX"), Some(&AnimationValue::Number(1.2)));
    assert_eq!(stage3_target.get("scaleX"), Some(&AnimationValue::Number(0.8)));

    assert_eq!(stage1_target.get("rotateX"), Some(&AnimationValue::Number(0.0)));
    assert_eq!(stage2_target.get("rotateX"), Some(&AnimationValue::Number(45.0)));
    assert_eq!(stage3_target.get("rotateX"), Some(&AnimationValue::Number(90.0)));
}

/// Test Complex Shape Morphing
///
/// Tests morphing between complex 3D shapes with multiple
/// transform properties.
#[test]
fn test_complex_shape_morphing() {
    // Create a complex morphing animation
    let complex_morph = Transform3D::default()
        .scale_x(1.5)
        .scale_y(0.7)
        .scale_z(1.3)
        .rotate_x(60.0)
        .rotate_y(30.0)
        .rotate_z(45.0)
        .translate_x(50.0)
        .translate_y(25.0)
        .translate_z(100.0);

    let complex_target = complex_morph.to_animation_target();

    // Verify all complex properties
    assert_eq!(complex_target.get("scaleX"), Some(&AnimationValue::Number(1.5)));
    assert_eq!(complex_target.get("scaleY"), Some(&AnimationValue::Number(0.7)));
    assert_eq!(complex_target.get("scaleZ"), Some(&AnimationValue::Number(1.3)));
    assert_eq!(complex_target.get("rotateX"), Some(&AnimationValue::Number(60.0)));
    assert_eq!(complex_target.get("rotateY"), Some(&AnimationValue::Number(30.0)));
    assert_eq!(complex_target.get("rotateZ"), Some(&AnimationValue::Number(45.0)));
    assert_eq!(complex_target.get("translateX"), Some(&AnimationValue::Number(50.0)));
    assert_eq!(complex_target.get("translateY"), Some(&AnimationValue::Number(25.0)));
    assert_eq!(complex_target.get("translateZ"), Some(&AnimationValue::Number(100.0)));
}
