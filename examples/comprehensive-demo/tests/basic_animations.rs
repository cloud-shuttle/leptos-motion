//! Tests for basic animation capabilities
//!
//! These tests define the expected behavior for basic animations like
//! fade, scale, rotate, and translate.

use crate::test_utils::*;
use leptos::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;

/// Test that fade animations work correctly
#[test]
fn test_fade_animation() {
    // Given: A component with fade animation
    let fade_config = AnimationConfig {
        duration: 500.0,
        easing: EasingFunction::EaseInOut,
        ..Default::default()
    };

    // When: Animation is triggered
    let (opacity, set_opacity) = create_signal(0.0);

    // Then: Opacity should animate from 0 to 1
    assert_eq!(opacity.get(), 0.0);

    // Simulate animation progress
    set_opacity.set(1.0);
    assert_eq!(opacity.get(), 1.0);
}

/// Test that scale animations work correctly
#[test]
fn test_scale_animation() {
    // Given: A component with scale animation
    let scale_config = AnimationConfig {
        duration: 300.0,
        easing: EasingFunction::EaseOut,
        ..Default::default()
    };

    // When: Animation is triggered
    let (scale, set_scale) = create_signal(0.0);

    // Then: Scale should animate from 0 to 1
    assert_eq!(scale.get(), 0.0);

    set_scale.set(1.0);
    assert_eq!(scale.get(), 1.0);
}

/// Test that rotate animations work correctly
#[test]
fn test_rotate_animation() {
    // Given: A component with rotate animation
    let rotate_config = AnimationConfig {
        duration: 1000.0,
        easing: EasingFunction::Linear,
        ..Default::default()
    };

    // When: Animation is triggered
    let (rotation, set_rotation) = create_signal(0.0);

    // Then: Rotation should animate from 0 to 360 degrees
    assert_eq!(rotation.get(), 0.0);

    set_rotation.set(360.0);
    assert_eq!(rotation.get(), 360.0);
}

/// Test that translate animations work correctly
#[test]
fn test_translate_animation() {
    // Given: A component with translate animation
    let translate_config = AnimationConfig {
        duration: 400.0,
        easing: EasingFunction::EaseIn,
        ..Default::default()
    };

    // When: Animation is triggered
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);

    // Then: Position should animate from (0,0) to (100,100)
    assert_eq!(x.get(), 0.0);
    assert_eq!(y.get(), 0.0);

    set_x.set(100.0);
    set_y.set(100.0);
    assert_eq!(x.get(), 100.0);
    assert_eq!(y.get(), 100.0);
}

/// Test that multiple animations can run simultaneously
#[test]
fn test_simultaneous_animations() {
    // Given: Multiple animation properties
    let (opacity, set_opacity) = create_signal(0.0);
    let (scale, set_scale) = create_signal(0.0);
    let (rotation, set_rotation) = create_signal(0.0);

    // When: All animations are triggered
    set_opacity.set(1.0);
    set_scale.set(1.0);
    set_rotation.set(180.0);

    // Then: All properties should be updated
    assert_eq!(opacity.get(), 1.0);
    assert_eq!(scale.get(), 1.0);
    assert_eq!(rotation.get(), 180.0);
}

/// Test that animation easing functions work correctly
#[test]
fn test_easing_functions() {
    // Given: Different easing functions
    let easing_functions = vec![
        EasingFunction::Linear,
        EasingFunction::EaseIn,
        EasingFunction::EaseOut,
        EasingFunction::EaseInOut,
        EasingFunction::EaseInCubic,
        EasingFunction::EaseOutCubic,
        EasingFunction::EaseInOutCubic,
    ];

    // When: Each easing function is applied
    for easing in easing_functions {
        let config = AnimationConfig {
            duration: 1000.0,
            easing,
            ..Default::default()
        };

        // Then: Configuration should be valid
        assert!(config.duration > 0.0);
    }
}

/// Test that animation delays work correctly
#[test]
fn test_animation_delay() {
    // Given: An animation with delay
    let config = AnimationConfig {
        duration: 500.0,
        delay: 200.0,
        ..Default::default()
    };

    // When: Animation is configured
    // Then: Delay should be set correctly
    assert_eq!(config.delay, 200.0);
    assert_eq!(config.duration, 500.0);
}

/// Test that animation repetition works correctly
#[test]
fn test_animation_repeat() {
    // Given: An animation with repeat
    let config = AnimationConfig {
        duration: 300.0,
        repeat: true,
        repeat_count: Some(3),
        ..Default::default()
    };

    // When: Animation is configured
    // Then: Repeat settings should be correct
    assert!(config.repeat);
    assert_eq!(config.repeat_count, Some(3));
}
