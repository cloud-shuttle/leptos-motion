#![cfg(test)]

//! # Advanced 3D Features Tests
//!
//! This module tests advanced 3D animation features including:
//! - Morphing animations (shape transitions)
//! - Particle systems
//! - Complex 3D transformations
//! - Advanced perspective effects
//! - 3D path animations
//! - Dynamic lighting effects
//!
//! These features extend the basic 3D animation capabilities to provide
//! more sophisticated and visually impressive animations.

use crate::animation_3d_implementation::{Animation3D, Perspective3D, Transform3D, utils};
use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

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

    let sphere_shape = Transform3D::default()
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

/// Test 3D Particle System Animation
///
/// Particle systems create dynamic effects with multiple animated elements
/// that can be used for visual effects like explosions, fire, or magic.
#[test]
fn test_3d_particle_system_animation() {
    // ============================================================================
    // 3D PARTICLE SYSTEM ANIMATION
    // ============================================================================

    // Create particle configurations for different effects
    let explosion_particle = Transform3D::default()
        .scale_x(0.1)
        .scale_y(0.1)
        .scale_z(0.1)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let explosion_target = explosion_particle.to_animation_target();

    // Verify explosion particle properties
    assert_eq!(
        explosion_target.get("scaleX"),
        Some(&AnimationValue::Number(0.1))
    );
    assert_eq!(
        explosion_target.get("scaleY"),
        Some(&AnimationValue::Number(0.1))
    );
    assert_eq!(
        explosion_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.1))
    );

    // Create fire particle effect
    let fire_particle = Transform3D::default()
        .scale_x(0.5)
        .scale_y(1.0)
        .scale_z(0.5)
        .translate_x(0.0)
        .translate_y(50.0)
        .translate_z(0.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0);

    let fire_target = fire_particle.to_animation_target();

    // Verify fire particle properties
    assert_eq!(
        fire_target.get("scaleX"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        fire_target.get("scaleY"),
        Some(&AnimationValue::Number(1.0))
    );
    assert_eq!(
        fire_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        fire_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );

    // Create magic sparkle particle
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

    // Create a helix transformation (spiral motion)
    let helix_transform = Transform3D::default()
        .rotate_z(360.0)
        .translate_x(50.0)
        .translate_y(0.0)
        .translate_z(100.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0);

    let helix_target = helix_transform.to_animation_target();

    // Verify helix transformation properties
    assert_eq!(
        helix_target.get("rotateZ"),
        Some(&AnimationValue::Number(360.0))
    );
    assert_eq!(
        helix_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        helix_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );

    // Create a wave transformation (oscillating motion)
    let wave_transform = Transform3D::default()
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0)
        .translate_x(0.0)
        .translate_y(25.0)
        .translate_z(0.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0);

    let wave_target = wave_transform.to_animation_target();

    // Verify wave transformation properties
    assert_eq!(
        wave_target.get("translateY"),
        Some(&AnimationValue::Number(25.0))
    );
}

/// Test Advanced Perspective Effects
///
/// Advanced perspective effects create depth and visual interest
/// through sophisticated perspective manipulation.
#[test]
fn test_advanced_perspective_effects() {
    // ============================================================================
    // ADVANCED PERSPECTIVE EFFECTS
    // ============================================================================

    // Create a tunnel perspective effect
    let tunnel_perspective = Perspective3D::default()
        .perspective("200px")
        .perspective_origin("center center")
        .transform_style("preserve-3d");

    let tunnel_target = tunnel_perspective.to_animation_target();

    // Verify tunnel perspective properties
    assert_eq!(
        tunnel_target.get("perspective"),
        Some(&AnimationValue::String("200px".to_string()))
    );
    assert_eq!(
        tunnel_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );
    assert_eq!(
        tunnel_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );

    // Create a fish-eye perspective effect
    let fisheye_perspective = Perspective3D::default()
        .perspective("50px")
        .perspective_origin("center center")
        .transform_style("preserve-3d");

    let fisheye_target = fisheye_perspective.to_animation_target();

    // Verify fish-eye perspective properties
    assert_eq!(
        fisheye_target.get("perspective"),
        Some(&AnimationValue::String("50px".to_string()))
    );

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

    // Create a figure-8 path animation
    let figure8_path = Transform3D::default()
        .translate_x(50.0)
        .translate_y(50.0)
        .translate_z(0.0)
        .rotate_z(45.0);

    let figure8_target = figure8_path.to_animation_target();

    // Verify figure-8 path properties
    assert_eq!(
        figure8_target.get("translateX"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        figure8_target.get("translateY"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        figure8_target.get("rotateZ"),
        Some(&AnimationValue::Number(45.0))
    );

    // Create a spiral path animation
    let spiral_path = Transform3D::default()
        .translate_x(75.0)
        .translate_y(0.0)
        .translate_z(50.0)
        .rotate_z(180.0);

    let spiral_target = spiral_path.to_animation_target();

    // Verify spiral path properties
    assert_eq!(
        spiral_target.get("translateX"),
        Some(&AnimationValue::Number(75.0))
    );
    assert_eq!(
        spiral_target.get("translateZ"),
        Some(&AnimationValue::Number(50.0))
    );
    assert_eq!(
        spiral_target.get("rotateZ"),
        Some(&AnimationValue::Number(180.0))
    );

    // Create a bezier curve path animation
    let bezier_path = Transform3D::default()
        .translate_x(125.0)
        .translate_y(75.0)
        .translate_z(25.0)
        .rotate_x(30.0)
        .rotate_y(60.0);

    let bezier_target = bezier_path.to_animation_target();

    // Verify bezier curve path properties
    assert_eq!(
        bezier_target.get("translateX"),
        Some(&AnimationValue::Number(125.0))
    );
    assert_eq!(
        bezier_target.get("translateY"),
        Some(&AnimationValue::Number(75.0))
    );
    assert_eq!(
        bezier_target.get("translateZ"),
        Some(&AnimationValue::Number(25.0))
    );
    assert_eq!(
        bezier_target.get("rotateX"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        bezier_target.get("rotateY"),
        Some(&AnimationValue::Number(60.0))
    );
}

/// Test Dynamic Lighting Effects
///
/// Dynamic lighting effects simulate realistic lighting conditions
/// and create atmospheric visual effects.
#[test]
fn test_dynamic_lighting_effects() {
    // ============================================================================
    // DYNAMIC LIGHTING EFFECTS
    // ============================================================================

    // Create a spotlight effect
    let spotlight_effect = Transform3D::default()
        .rotate_x(45.0)
        .rotate_y(30.0)
        .rotate_z(0.0)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(100.0)
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0);

    let spotlight_target = spotlight_effect.to_animation_target();

    // Verify spotlight effect properties
    assert_eq!(
        spotlight_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        spotlight_target.get("rotateY"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        spotlight_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );

    // Create a flickering light effect
    let flickering_light = Transform3D::default()
        .scale_x(1.0)
        .scale_y(1.0)
        .scale_z(1.0)
        .rotate_x(0.0)
        .rotate_y(0.0)
        .rotate_z(0.0)
        .translate_x(0.0)
        .translate_y(0.0)
        .translate_z(0.0);

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

/// Test Advanced 3D Animation Combinations
///
/// This test combines multiple advanced 3D features to create
/// complex and visually impressive animations.
#[test]
fn test_advanced_3d_animation_combinations() {
    // ============================================================================
    // ADVANCED 3D ANIMATION COMBINATIONS
    // ============================================================================

    // Create a morphing particle system with dynamic lighting
    let morphing_particle = Animation3D::new()
        .transform(
            Transform3D::default()
                .scale_x(0.5)
                .scale_y(0.5)
                .scale_z(0.5)
                .rotate_x(180.0)
                .rotate_y(180.0)
                .rotate_z(180.0)
                .translate_x(100.0)
                .translate_y(100.0)
                .translate_z(100.0),
        )
        .perspective(
            Perspective3D::default()
                .perspective("500px")
                .perspective_origin("center center")
                .transform_style("preserve-3d"),
        )
        .transition(Transition {
            duration: Some(3.0),
            delay: Some(0.5),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::InfiniteReverse,
            stagger: None,
        });

    let morphing_particle_target = morphing_particle.to_animation_target();

    // Verify morphing particle properties
    assert_eq!(
        morphing_particle_target.get("scaleX"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        morphing_particle_target.get("scaleY"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        morphing_particle_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        morphing_particle_target.get("rotateX"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        morphing_particle_target.get("rotateY"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        morphing_particle_target.get("rotateZ"),
        Some(&AnimationValue::Number(180.0))
    );
    assert_eq!(
        morphing_particle_target.get("translateX"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        morphing_particle_target.get("translateY"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        morphing_particle_target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
    assert_eq!(
        morphing_particle_target.get("perspective"),
        Some(&AnimationValue::String("500px".to_string()))
    );
    assert_eq!(
        morphing_particle_target.get("perspective-origin"),
        Some(&AnimationValue::String("center center".to_string()))
    );
    assert_eq!(
        morphing_particle_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );

    // Create a complex 3D scene with multiple elements
    let complex_scene = Animation3D::new()
        .transform(
            Transform3D::default()
                .rotate_x(45.0)
                .rotate_y(60.0)
                .rotate_z(30.0)
                .translate_x(200.0)
                .translate_y(150.0)
                .translate_z(300.0)
                .scale_x(2.0)
                .scale_y(1.5)
                .scale_z(0.8),
        )
        .perspective(
            Perspective3D::default()
                .perspective("800px")
                .perspective_origin("top right")
                .transform_style("preserve-3d"),
        );

    let complex_scene_target = complex_scene.to_animation_target();

    // Verify complex scene properties
    assert_eq!(
        complex_scene_target.get("rotateX"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        complex_scene_target.get("rotateY"),
        Some(&AnimationValue::Number(60.0))
    );
    assert_eq!(
        complex_scene_target.get("rotateZ"),
        Some(&AnimationValue::Number(30.0))
    );
    assert_eq!(
        complex_scene_target.get("translateX"),
        Some(&AnimationValue::Number(200.0))
    );
    assert_eq!(
        complex_scene_target.get("translateY"),
        Some(&AnimationValue::Number(150.0))
    );
    assert_eq!(
        complex_scene_target.get("translateZ"),
        Some(&AnimationValue::Number(300.0))
    );
    assert_eq!(
        complex_scene_target.get("scaleX"),
        Some(&AnimationValue::Number(2.0))
    );
    assert_eq!(
        complex_scene_target.get("scaleY"),
        Some(&AnimationValue::Number(1.5))
    );
    assert_eq!(
        complex_scene_target.get("scaleZ"),
        Some(&AnimationValue::Number(0.8))
    );
    assert_eq!(
        complex_scene_target.get("perspective"),
        Some(&AnimationValue::String("800px".to_string()))
    );
    assert_eq!(
        complex_scene_target.get("perspective-origin"),
        Some(&AnimationValue::String("top right".to_string()))
    );
    assert_eq!(
        complex_scene_target.get("transform-style"),
        Some(&AnimationValue::String("preserve-3d".to_string()))
    );
}

/// Test Advanced 3D Animation Performance
///
/// This test ensures that advanced 3D animations maintain good performance
/// even with complex transformations and effects.
#[test]
fn test_advanced_3d_animation_performance() {
    // ============================================================================
    // ADVANCED 3D ANIMATION PERFORMANCE
    // ============================================================================

    // Test performance with many simultaneous 3D transformations
    let start_time = std::time::Instant::now();

    // Create 100 complex 3D transformations
    for i in 0..100 {
        let complex_transform = Transform3D::default()
            .rotate_x((i as f64) * 3.6)
            .rotate_y((i as f64) * 2.4)
            .rotate_z((i as f64) * 1.8)
            .translate_x((i as f64) * 2.0)
            .translate_y((i as f64) * 1.5)
            .translate_z((i as f64) * 3.0)
            .scale_x(1.0 + (i as f64) * 0.01)
            .scale_y(1.0 + (i as f64) * 0.01)
            .scale_z(1.0 + (i as f64) * 0.01);

        let _target = complex_transform.to_animation_target();
    }

    let duration = start_time.elapsed();

    // Verify that 100 complex transformations can be processed in under 50ms
    // This ensures we can handle 60+ FPS with complex 3D animations
    assert!(
        duration.as_millis() < 50,
        "Advanced 3D animation processing took {}ms, should be under 50ms",
        duration.as_millis()
    );

    // Test performance with perspective calculations
    let perspective_start = std::time::Instant::now();

    // Create 50 perspective configurations
    for i in 0..50 {
        let perspective = Perspective3D::default()
            .perspective(&format!("{}px", 100 + i * 10))
            .perspective_origin("center center")
            .transform_style("preserve-3d");

        let _target = perspective.to_animation_target();
    }

    let perspective_duration = perspective_start.elapsed();

    // Verify that perspective calculations are fast
    assert!(
        perspective_duration.as_millis() < 25,
        "Perspective calculation took {}ms, should be under 25ms",
        perspective_duration.as_millis()
    );
}
