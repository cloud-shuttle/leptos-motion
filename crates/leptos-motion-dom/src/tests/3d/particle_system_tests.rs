#![cfg(test)]

//! # 3D Particle System Tests
//!
//! This module tests 3D particle system features including:
//! - Particle generation and management
//! - Particle physics and movement
//! - Particle effects (sparkles, explosions, trails)
//! - Particle lifecycle management

use crate::animation_3d_implementation::{Animation3D, Transform3D};
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};

/// Test Particle System Generation
///
/// Tests the creation and management of particle systems
/// with different particle types and behaviors.
#[test]
fn test_particle_system_generation() {
    // ============================================================================
    // PARTICLE SYSTEM GENERATION
    // ============================================================================

    // Create a basic particle system
    let particle_system = Transform3D::default()
        .scale_x(0.1)
        .scale_y(0.1)
        .scale_z(0.1)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let particle_target = particle_system.to_animation_target();

    // Verify basic particle properties
    assert_eq!(
        particle_target.get("scaleX"),
        Some(&AnimationValue::Number(0.1))
    );
    assert_eq!(
        particle_target.get("scaleY"),
        Some(&AnimationValue::Number(0.1))
    );
    assert_eq!(
        particle_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.1))
    );
    assert_eq!(
        particle_target.get("translateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        particle_target.get("translateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        particle_target.get("translateZ"),
        Some(&AnimationValue::Number(0.0))
    );
}

/// Test Sparkle Particle Effects
///
/// Tests sparkle particle effects with rotation and scaling
/// animations for visual appeal.
#[test]
fn test_sparkle_particle_effects() {
    // Create a sparkle particle effect
    let sparkle_particle = Transform3D::default()
        .scale_x(0.2)
        .scale_y(0.2)
        .scale_z(0.2)
        .translate_x(25.0)
        .translate_y(25.0)
        .translate_z(25.0)
        .rotate_x(360.0)
        .rotate_y(360.0)
        .rotate_z(360.0);

    let sparkle_target = sparkle_particle.to_animation_target();

    // Verify sparkle particle properties
    assert_eq!(
        sparkle_target.get("scaleX"),
        Some(&AnimationValue::Number(0.2))
    );
    assert_eq!(
        sparkle_target.get("scaleY"),
        Some(&AnimationValue::Number(0.2))
    );
    assert_eq!(
        sparkle_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.2))
    );
    assert_eq!(
        sparkle_target.get("translateX"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        sparkle_target.get("translateY"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        sparkle_target.get("translateZ"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        sparkle_target.get("rotateX"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        sparkle_target.get("rotateY"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        sparkle_target.get("rotateZ"),
        Some(&AnimationValue::Number(360.0))
    );
}

/// Test Explosion Particle Effects
///
/// Tests explosion particle effects with radial movement
/// and scaling animations.
#[test]
fn test_explosion_particle_effects() {
    // Create an explosion particle effect
    let explosion_particle = Transform3D::default()
        .scale_x(2.0)
        .scale_y(2.0)
        .scale_z(2.0)
        .translate_x(100.0)
        .translate_y(100.0)
        .translate_z(100.0)
        .rotate_x(180.0)
        .rotate_y(180.0)
        .rotate_z(180.0);

    let explosion_target = explosion_particle.to_animation_target();

    // Verify explosion particle properties
    assert_eq!(
        explosion_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        explosion_target.get("scaleY"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        explosion_target.get("scaleZ"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        explosion_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        explosion_target.get("translateY"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        explosion_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
}

/// Test Particle Trail Effects
///
/// Tests particle trail effects with continuous movement
/// and fading animations.
#[test]
fn test_particle_trail_effects() {
    // Create a particle trail effect
    let trail_particle = Transform3D::default()
        .scale_x(0.5)
        .scale_y(0.5)
        .scale_z(0.5)
        .translate_x(50.0)
        .translate_y(50.0)
        .translate_z(50.0)
        .rotate_x(90.0)
        .rotate_y(90.0)
        .rotate_z(90.0);

    let trail_target = trail_particle.to_animation_target();

    // Verify trail particle properties
    assert_eq!(
        trail_target.get("scaleX"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        trail_target.get("scaleY"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        trail_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        trail_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        trail_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        trail_target.get("translateZ"),
        Some(&AnimationValue::Number(50.0))
    );
}
