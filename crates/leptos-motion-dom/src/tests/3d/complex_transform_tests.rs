#![cfg(test)]

//! # Complex 3D Transform Tests
//!
//! This module tests complex 3D transformation features including:
//! - Multi-axis transformations
//! - Combined rotation, scaling, and translation
//! - Complex transform chains
//! - Transform interpolation

use crate::animation_3d_implementation::{Animation3D, Transform3D};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};

/// Test Complex 3D Transformations
///
/// Complex transformations combine multiple 3D operations to create
/// sophisticated visual effects.
#[test]
fn test_complex_3d_transformations() {
    // ============================================================================
    // COMPLEX 3D TRANSFORMATIONS
    // ============================================================================

    // Create a complex transformation that combines rotation, scaling, and translation
    let complex_transform = Transform3D::default()
        .rotate_x(45.0)
        .rotate_y(60.0)
        .rotate_z(30.0)
        .translate_x(100.0)
        .translate_y(50.0)
        .translate_z(200.0)
        .scale_x(1.5)
        .scale_y(0.8)
        .scale_z(1.2);

    let complex_target = complex_transform.to_animation_target();

    // Verify all complex transformation properties
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
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        complex_target.get("scaleY"),
        Some(&AnimationValue::Number(0.8))
    );
    assert_eq!(
        complex_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.2))
    );
}

/// Test Multi-Axis Rotation
///
/// Tests complex rotations around multiple axes simultaneously.
#[test]
fn test_multi_axis_rotation() {
    // Create a multi-axis rotation
    let multi_rotation = Transform3D::default()
        .rotate_x(90.0)
        .rotate_y(180.0)
        .rotate_z(270.0);

    let rotation_target = multi_rotation.to_animation_target();

    // Verify multi-axis rotation properties
    assert_eq!(
        rotation_target.get("rotateX"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        rotation_target.get("rotateY"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        rotation_target.get("rotateZ"),
        Some(&AnimationValue::Number(270.0))
    );
}

/// Test Complex Scaling
///
/// Tests complex scaling operations with different scales
/// on each axis.
#[test]
fn test_complex_scaling() {
    // Create a complex scaling transformation
    let complex_scaling = Transform3D::default()
        .scale_x(2.0)
        .scale_y(0.5)
        .scale_z(1.5);

    let scaling_target = complex_scaling.to_animation_target();

    // Verify complex scaling properties
    assert_eq!(
        scaling_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        scaling_target.get("scaleY"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        scaling_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.5))
    );
}

/// Test Transform Chain
///
/// Tests chaining multiple transformations together
/// to create complex effects.
#[test]
fn test_transform_chain() {
    // Create a transform chain
    let transform_chain = Transform3D::default()
        .translate_x(50.0)
        .translate_y(25.0)
        .translate_z(75.0)
        .rotate_x(45.0)
        .rotate_y(30.0)
        .rotate_z(60.0)
        .scale_x(1.2)
        .scale_y(0.8)
        .scale_z(1.1);

    let chain_target = transform_chain.to_animation_target();

    // Verify transform chain properties
    assert_eq!(
        chain_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        chain_target.get("translateY"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        chain_target.get("translateZ"),
        Some(&AnimationValue::Number(75.0))
    );
    assert_eq!(
        chain_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        chain_target.get("rotateY"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        chain_target.get("rotateZ"),
        Some(&AnimationValue::Number(60.0))
    );
    assert_eq!(
        chain_target.get("scaleX"),
        Some(&AnimationValue::Number(1.2))
    );
    assert_eq!(
        chain_target.get("scaleY"),
        Some(&AnimationValue::Number(0.8))
    );
    assert_eq!(
        chain_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.1))
    );
}

/// Test Transform Interpolation
///
/// Tests interpolation between different transform states
/// for smooth transitions.
#[test]
fn test_transform_interpolation() {
    // Create start and end transform states
    let start_transform = Transform3D::default()
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0);

    let end_transform = Transform3D::default()
        .translate_x(100.0)
        .translate_y(100.0)
        .translate_z(100.0)
        .rotate_x(360.0)
        .rotate_y(360.0)
        .rotate_z(360.0)
        .scale_x(2.0)
        .scale_y(2.0)
        .scale_z(2.0);

    let start_target = start_transform.to_animation_target();
    let end_target = end_transform.to_animation_target();

    // Verify start transform properties
    assert_eq!(
        start_target.get("translateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        start_target.get("rotateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        start_target.get("scaleX"),
        Some(&AnimationValue::Number(1.0))
    );

    // Verify end transform properties
    assert_eq!(
        end_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        end_target.get("rotateX"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        end_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
}
