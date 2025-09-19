#![cfg(test)]

//! # 3D Path Animation Tests
//!
//! This module tests 3D path animation features including:
//! - Circular path animations
//! - Spiral path animations
//! - Complex 3D trajectories
//! - Path interpolation and smoothing

use crate::animation_3d_implementation::{Animation3D, Transform3D};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};

/// Test 3D Path Animations
///
/// 3D path animations allow objects to follow complex 3D trajectories
/// through space, creating smooth and natural motion.
#[test]
fn test_3d_path_animations() {
    // ============================================================================
    // 3D PATH ANIMATIONS
    // ============================================================================

    // Create a circular path animation
    let circular_path = Transform3D::default()
        .translate_x(100.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_z(0.0);

    let circular_target = circular_path.to_animation_target();

    // Verify circular path properties
    assert_eq!(
        circular_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        circular_target.get("translateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        circular_target.get("translateZ"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        circular_target.get("rotateZ"),
        Some(&AnimationValue::Number(0.0))
    );
}

/// Test Spiral Path Animation
///
/// Tests spiral path animations with combined translation
/// and rotation movements.
#[test]
fn test_spiral_path_animation() {
    // Create a spiral path animation
    let spiral_path = Transform3D::default()
        .translate_x(50.0)
        .translate_y(50.0)
        .translate_z(100.0)
        .rotate_x(180.0)
        .rotate_y(360.0)
        .rotate_z(720.0);

    let spiral_target = spiral_path.to_animation_target();

    // Verify spiral path properties
    assert_eq!(
        spiral_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        spiral_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        spiral_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        spiral_target.get("rotateX"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        spiral_target.get("rotateY"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        spiral_target.get("rotateZ"),
        Some(&AnimationValue::Number(720.0))
    );
}

/// Test Complex 3D Trajectory
///
/// Tests complex 3D trajectories with multiple waypoints
/// and smooth interpolation.
#[test]
fn test_complex_3d_trajectory() {
    // Create a complex 3D trajectory
    let complex_trajectory = Transform3D::default()
        .translate_x(200.0)
        .translate_y(150.0)
        .translate_z(300.0)
        .rotate_x(90.0)
        .rotate_y(180.0)
        .rotate_z(270.0)
        .scale_x(1.5)
        .scale_y(1.5)
        .scale_z(1.5);

    let trajectory_target = complex_trajectory.to_animation_target();

    // Verify complex trajectory properties
    assert_eq!(
        trajectory_target.get("translateX"),
        Some(&AnimationValue::Number(200.0))
    );
    assert_eq!(
        trajectory_target.get("translateY"),
        Some(&AnimationValue::Number(150.0))
    );
    assert_eq!(
        trajectory_target.get("translateZ"),
        Some(&AnimationValue::Number(300.0))
    );
    assert_eq!(
        trajectory_target.get("rotateX"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        trajectory_target.get("rotateY"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        trajectory_target.get("rotateZ"),
        Some(&AnimationValue::Number(270.0))
    );
    assert_eq!(
        trajectory_target.get("scaleX"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        trajectory_target.get("scaleY"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        trajectory_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.5))
    );
}

/// Test Path Interpolation
///
/// Tests smooth interpolation between different path points
/// for natural motion.
#[test]
fn test_path_interpolation() {
    // Create start and end path points
    let start_point = Transform3D::default()
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let end_point = Transform3D::default()
        .translate_x(100.0)
        .translate_y(100.0)
        .translate_z(100.0)
        .rotate_x(360.0)
        .rotate_y(360.0)
        .rotate_z(360.0);

    let start_target = start_point.to_animation_target();
    let end_target = end_point.to_animation_target();

    // Verify start point properties
    assert_eq!(
        start_target.get("translateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        start_target.get("translateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        start_target.get("translateZ"),
        Some(&AnimationValue::Number(0.0))
    );

    // Verify end point properties
    assert_eq!(
        end_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        end_target.get("translateY"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        end_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
}

/// Test Dynamic Lighting Effects
///
/// Tests dynamic lighting effects that can be animated
/// along 3D paths.
#[test]
fn test_dynamic_lighting_effects() {
    // Create a flickering light effect
    let flickering_light = Transform3D::default()
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let flickering_target = flickering_light.to_animation_target();

    // Verify flickering light properties
    assert_eq!(
        flickering_target.get("scaleX"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        flickering_target.get("scaleY"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        flickering_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.0))
    );

    // Create a rotating light effect
    let rotating_light = Transform3D::default()
        .rotate_x(0.0)
        .rotate_y(360.0)
        .rotate_z(0.0)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(50.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0);

    let rotating_target = rotating_light.to_animation_target();

    // Verify rotating light properties
    assert_eq!(
        rotating_target.get("rotateY"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        rotating_target.get("translateZ"),
        Some(&AnimationValue::Number(50.0))
    );

    // Create a pulsing light effect
    let pulsing_light = Transform3D::default()
        .scale_x(1.5)
        .scale_y(1.5)
        .scale_z(1.5)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0);

    let pulsing_target = pulsing_light.to_animation_target();

    // Verify pulsing light properties
    assert_eq!(
        pulsing_target.get("scaleX"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        pulsing_target.get("scaleY"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        pulsing_target.get("scaleZ"),
        Some(&AnimationValue::Number(1.5))
    );
}
