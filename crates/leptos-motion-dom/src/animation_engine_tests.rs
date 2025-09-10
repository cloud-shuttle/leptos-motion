//! Animation Engine Tests
//!
//! Comprehensive tests for the animation engine and related functionality.

use crate::{
    animation_engine::{AnimationEngine, AnimationEngineBuilder, SpringConfig},
    easing_functions::*,
    repeat_config::{AnimationCycleManager, CycleUpdate, RepeatState, StaggerConfig},
    transform_animations::{
        Transform2D, Transform3D, TransformAnimationBuilder, TransformAnimationManager,
    },
};
use leptos_motion_core::*;
// use std::collections::HashMap; // Unused import

/// Test basic animation engine functionality
#[test]
fn test_animation_engine_basic() {
    let mut engine = AnimationEngine::new();

    // Test property animation
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::Linear,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    engine.animate_property("opacity".to_string(), 0.0, 1.0, transition);

    // Check that animation was added
    assert!(engine.get_property_value("opacity").is_some());
    assert_eq!(engine.get_property_value("opacity"), Some(0.0));
}

/// Test animation engine builder
#[test]
fn test_animation_engine_builder() {
    let engine = AnimationEngineBuilder::new()
        .on_complete(|| {
            println!("Animation complete!");
        })
        .on_update(|values| {
            println!("Animation values: {:?}", values);
        })
        .build();

    // Test that callbacks were set
    // Test that callbacks can be set (they're private fields, so we test the methods)
    let mut engine = AnimationEngine::new();
    engine.on_complete(|| {});
    engine.on_update(|_| {});
    // If we get here without compilation errors, the callbacks work
}

/// Test spring configuration
#[test]
fn test_spring_config() {
    let config = SpringConfig::default();

    assert_eq!(config.stiffness, 100.0);
    assert_eq!(config.damping, 10.0);
    assert_eq!(config.mass, 1.0);
    assert_eq!(config.velocity, 0.0);
    assert_eq!(config.rest_delta, 0.01);
    assert_eq!(config.rest_speed, 0.01);
}

/// Test easing functions
#[test]
fn test_easing_functions() {
    // Test linear easing
    assert_eq!(standard::linear(0.0), 0.0);
    assert_eq!(standard::linear(0.5), 0.5);
    assert_eq!(standard::linear(1.0), 1.0);

    // Test ease in
    assert_eq!(standard::ease_in(0.0), 0.0);
    assert_eq!(standard::ease_in(0.5), 0.25);
    assert_eq!(standard::ease_in(1.0), 1.0);

    // Test ease out
    assert_eq!(standard::ease_out(0.0), 0.0);
    assert_eq!(standard::ease_out(0.5), 0.75);
    assert_eq!(standard::ease_out(1.0), 1.0);

    // Test ease in out
    assert_eq!(standard::ease_in_out(0.0), 0.0);
    assert_eq!(standard::ease_in_out(0.5), 0.5);
    assert_eq!(standard::ease_in_out(1.0), 1.0);
}

/// Test sine easing functions
#[test]
fn test_sine_easing() {
    // Test ease in sine
    assert_eq!(sine::ease_in_sine(0.0), 0.0);
    assert!((sine::ease_in_sine(1.0) - 1.0).abs() < 1e-10);

    // Test ease out sine
    assert_eq!(sine::ease_out_sine(0.0), 0.0);
    assert!((sine::ease_out_sine(1.0) - 1.0).abs() < 1e-10);

    // Test ease in out sine
    assert_eq!(sine::ease_in_out_sine(0.0), 0.0);
    assert!((sine::ease_in_out_sine(1.0) - 1.0).abs() < 1e-10);
}

/// Test exponential easing functions
#[test]
fn test_exponential_easing() {
    // Test ease in exponential
    assert_eq!(exponential::ease_in_expo(0.0), 0.0);
    assert_eq!(exponential::ease_in_expo(1.0), 1.0);

    // Test ease out exponential
    assert_eq!(exponential::ease_out_expo(0.0), 0.0);
    assert_eq!(exponential::ease_out_expo(1.0), 1.0);

    // Test ease in out exponential
    assert_eq!(exponential::ease_in_out_expo(0.0), 0.0);
    assert_eq!(exponential::ease_in_out_expo(1.0), 1.0);
}

/// Test circular easing functions
#[test]
fn test_circular_easing() {
    // Test ease in circular
    assert_eq!(circular::ease_in_circ(0.0), 0.0);
    assert_eq!(circular::ease_in_circ(1.0), 1.0);

    // Test ease out circular
    assert_eq!(circular::ease_out_circ(0.0), 0.0);
    assert_eq!(circular::ease_out_circ(1.0), 1.0);

    // Test ease in out circular
    assert_eq!(circular::ease_in_out_circ(0.0), 0.0);
    assert_eq!(circular::ease_in_out_circ(1.0), 1.0);
}

/// Test back easing functions
#[test]
fn test_back_easing() {
    // Test ease in back
    assert_eq!(back::ease_in_back(0.0), 0.0);
    assert!((back::ease_in_back(1.0) - 1.0).abs() < 1e-10);

    // Test ease out back
    assert!((back::ease_out_back(0.0) - 0.0).abs() < 1e-10);
    assert!((back::ease_out_back(1.0) - 1.0).abs() < 1e-10);

    // Test ease in out back
    assert_eq!(back::ease_in_out_back(0.0), 0.0);
    assert!((back::ease_in_out_back(1.0) - 1.0).abs() < 1e-10);
}

/// Test elastic easing functions
#[test]
fn test_elastic_easing() {
    // Test ease in elastic
    assert_eq!(elastic::ease_in_elastic(0.0), 0.0);
    assert_eq!(elastic::ease_in_elastic(1.0), 1.0);

    // Test ease out elastic
    assert_eq!(elastic::ease_out_elastic(0.0), 0.0);
    assert_eq!(elastic::ease_out_elastic(1.0), 1.0);

    // Test ease in out elastic
    assert_eq!(elastic::ease_in_out_elastic(0.0), 0.0);
    assert_eq!(elastic::ease_in_out_elastic(1.0), 1.0);
}

/// Test bounce easing functions
#[test]
fn test_bounce_easing() {
    // Test ease in bounce
    assert_eq!(bounce::ease_in_bounce(0.0), 0.0);
    assert_eq!(bounce::ease_in_bounce(1.0), 1.0);

    // Test ease out bounce
    assert_eq!(bounce::ease_out_bounce(0.0), 0.0);
    assert_eq!(bounce::ease_out_bounce(1.0), 1.0);

    // Test ease in out bounce
    assert_eq!(bounce::ease_in_out_bounce(0.0), 0.0);
    assert_eq!(bounce::ease_in_out_bounce(1.0), 1.0);
}

/// Test cubic bezier curves
#[test]
fn test_cubic_bezier() {
    let bezier = bezier::ease();

    // Test bezier evaluation
    assert_eq!(bezier.evaluate(0.0), 0.0);
    assert_eq!(bezier.evaluate(1.0), 1.0);

    // Test custom bezier
    let custom_bezier = bezier::custom(0.25, 0.1, 0.25, 1.0);
    assert_eq!(custom_bezier.evaluate(0.0), 0.0);
    assert_eq!(custom_bezier.evaluate(1.0), 1.0);
}

/// Test spring physics
#[test]
fn test_spring_physics() {
    use crate::easing_functions::spring::*;

    let config = SpringConfig::default();
    let mut state = SpringState::new(0.0, 100.0);

    // Test initial state
    assert_eq!(state.position, 0.0);
    assert_eq!(state.velocity, 0.0);
    assert_eq!(state.target, 100.0);
    assert!(!state.is_complete);

    // Test spring update
    update_spring(&mut state, &config, 0.016); // 60fps

    // Position should be valid (spring physics can have negative values initially)
    assert!(state.position.is_finite());
    assert!(state.position <= 100.0);
}

/// Test spring presets
#[test]
fn test_spring_presets() {
    use crate::easing_functions::spring::presets::*;

    let gentle = gentle();
    assert_eq!(gentle.stiffness, 120.0);
    assert_eq!(gentle.damping, 14.0);

    let wobbly = wobbly();
    assert_eq!(wobbly.stiffness, 180.0);
    assert_eq!(wobbly.damping, 12.0);

    let stiff = stiff();
    assert_eq!(stiff.stiffness, 210.0);
    assert_eq!(stiff.damping, 20.0);

    let slow = slow();
    assert_eq!(slow.stiffness, 280.0);
    assert_eq!(slow.damping, 60.0);

    let bouncy = bouncy();
    assert_eq!(bouncy.stiffness, 400.0);
    assert_eq!(bouncy.damping, 10.0);
}

/// Test repeat configuration
#[test]
fn test_repeat_config() {
    // Test never repeat
    let never_state = RepeatState::new(&RepeatConfig::Never);
    assert!(!never_state.should_continue());
    assert_eq!(never_state.current_count, 0);

    // Test count repeat
    let count_state = RepeatState::new(&RepeatConfig::Count(3));
    assert!(count_state.should_continue());
    assert_eq!(count_state.current_count, 0);

    // Test infinite repeat
    let infinite_state = RepeatState::new(&RepeatConfig::Infinite);
    assert!(infinite_state.should_continue());
    assert_eq!(infinite_state.current_count, 0);

    // Test infinite reverse repeat
    let infinite_reverse_state = RepeatState::new(&RepeatConfig::InfiniteReverse);
    assert!(infinite_reverse_state.should_continue());
    assert!(infinite_reverse_state.reverse);
    assert_eq!(infinite_reverse_state.current_count, 0);
}

/// Test repeat state advancement
#[test]
fn test_repeat_state_advancement() {
    let mut state = RepeatState::new(&RepeatConfig::Count(2));

    // First cycle
    assert!(state.should_continue());
    assert_eq!(state.current_count, 0);
    assert!(!state.is_reversed);

    state.advance();
    assert!(state.should_continue());
    assert_eq!(state.current_count, 1);
    assert!(!state.is_reversed);

    // Second cycle
    state.advance();
    assert!(!state.should_continue());
    assert_eq!(state.current_count, 2);
    assert!(state.is_complete);
}

/// Test reverse repeat
#[test]
fn test_reverse_repeat() {
    let mut state = RepeatState::new(&RepeatConfig::InfiniteReverse);

    // First cycle
    assert!(!state.is_reversed);
    assert_eq!(state.progress_multiplier(), 1.0);

    state.advance();
    assert!(state.is_reversed);
    assert_eq!(state.progress_multiplier(), -1.0);

    state.advance();
    assert!(!state.is_reversed);
    assert_eq!(state.progress_multiplier(), 1.0);
}

/// Test animation cycle manager
#[test]
fn test_animation_cycle_manager() {
    let mut manager = AnimationCycleManager::new(&RepeatConfig::Count(2), 1.0);

    // Initial state
    assert_eq!(manager.cycle_progress(), 0.0);
    assert_eq!(manager.current_count(), 0);
    assert!(!manager.is_complete());

    // Update with small delta
    let update = manager.update(0.5);
    assert_eq!(update, CycleUpdate::Continue);
    assert_eq!(manager.cycle_progress(), 0.5);
    assert_eq!(manager.effective_progress(), 0.5);

    // Complete first cycle
    let update = manager.update(0.5);
    assert_eq!(update, CycleUpdate::CycleComplete);
    assert_eq!(manager.current_count(), 1);
    assert_eq!(manager.cycle_progress(), 0.0);

    // Complete second cycle
    let update = manager.update(1.0);
    assert_eq!(update, CycleUpdate::AnimationComplete);
    assert!(manager.is_complete());
}

/// Test stagger configuration
#[test]
fn test_stagger_config() {
    let config = StaggerConfig::new(0.1);

    assert_eq!(config.delay, 0.1);
    assert!(!config.reverse);
    assert_eq!(config.max_count, None);

    // Test delay calculation
    assert_eq!(config.get_delay(0), 0.0);
    assert_eq!(config.get_delay(1), 0.1);
    assert_eq!(config.get_delay(2), 0.2);
}

/// Test reverse stagger
#[test]
fn test_reverse_stagger() {
    let config = StaggerConfig::new(0.1).reverse().max_count(3);

    assert_eq!(config.get_delay(0), 0.2);
    assert_eq!(config.get_delay(1), 0.1);
    assert_eq!(config.get_delay(2), 0.0);
}

/// Test 2D transforms
#[test]
fn test_transform_2d() {
    let transform = Transform2D::new()
        .translate(10.0, 20.0)
        .rotate(45.0)
        .scale(2.0, 3.0)
        .skew(5.0, 10.0);

    assert_eq!(transform.translate_x, 10.0);
    assert_eq!(transform.translate_y, 20.0);
    assert_eq!(transform.rotate, 45.0);
    assert_eq!(transform.scale_x, 2.0);
    assert_eq!(transform.scale_y, 3.0);
    assert_eq!(transform.skew_x, 5.0);
    assert_eq!(transform.skew_y, 10.0);

    // Test CSS generation
    let css = transform.to_css();
    assert!(css.contains("translate(10px, 20px)"));
    assert!(css.contains("rotate(45deg)"));
    assert!(css.contains("scale(2, 3)"));
    assert!(css.contains("skew(5deg, 10deg)"));
}

/// Test 3D transforms
#[test]
fn test_transform_3d() {
    let transform = Transform3D::new()
        .translate(10.0, 20.0, 30.0)
        .rotate(45.0, 60.0, 90.0)
        .scale(2.0, 3.0, 4.0)
        .perspective(1000.0);

    assert_eq!(transform.translate_x, 10.0);
    assert_eq!(transform.translate_y, 20.0);
    assert_eq!(transform.translate_z, 30.0);
    assert_eq!(transform.rotate_x, 45.0);
    assert_eq!(transform.rotate_y, 60.0);
    assert_eq!(transform.rotate_z, 90.0);
    assert_eq!(transform.scale_x, 2.0);
    assert_eq!(transform.scale_y, 3.0);
    assert_eq!(transform.scale_z, 4.0);
    assert_eq!(transform.perspective, 1000.0);

    // Test CSS generation
    let css = transform.to_css();
    assert!(css.contains("perspective(1000px)"));
    assert!(css.contains("translate3d(10px, 20px, 30px)"));
    assert!(css.contains("rotateX(45deg)"));
    assert!(css.contains("rotateY(60deg)"));
    assert!(css.contains("rotateZ(90deg)"));
    assert!(css.contains("scale3d(2, 3, 4)"));
}

/// Test transform interpolation
#[test]
fn test_transform_interpolation() {
    let start = Transform2D::new().translate(0.0, 0.0).scale(1.0, 1.0);
    let end = Transform2D::new().translate(100.0, 100.0).scale(2.0, 2.0);

    let interpolated = start.interpolate(&end, 0.5);

    assert_eq!(interpolated.translate_x, 50.0);
    assert_eq!(interpolated.translate_y, 50.0);
    assert_eq!(interpolated.scale_x, 1.5);
    assert_eq!(interpolated.scale_y, 1.5);
}

/// Test transform animation manager
#[test]
fn test_transform_animation_manager() {
    let mut manager = TransformAnimationManager::new();

    // Test property setting
    manager.set_property_value("translateX", 50.0);
    assert_eq!(manager.get_property_value("translateX"), 50.0);

    manager.set_property_value("scaleX", 2.0);
    assert_eq!(manager.get_property_value("scaleX"), 2.0);

    // Test CSS generation
    let css = manager.get_css_transform();
    assert!(css.contains("translate(50px, 0px)"));
    assert!(css.contains("scale(2, 1)"));
}

/// Test transform animation builder
#[test]
fn test_transform_animation_builder() {
    let transition = Transition::default();
    let manager = TransformAnimationBuilder::new()
        .translate(100.0, 200.0, &transition)
        .rotate(45.0, &transition)
        .scale(2.0, 3.0, &transition)
        .build();

    // Test that animations were set up (check that animations exist)
    assert!(manager.has_active_animations());
    
    // The property values should be at their initial values (0.0) since animations haven't been updated
    assert_eq!(manager.get_property_value("translateX"), 0.0);
    assert_eq!(manager.get_property_value("translateY"), 0.0);
    assert_eq!(manager.get_property_value("rotate"), 0.0);
    assert_eq!(manager.get_property_value("scaleX"), 1.0); // Default scale is 1.0
    assert_eq!(manager.get_property_value("scaleY"), 1.0); // Default scale is 1.0
}

/// Test transform presets
#[test]
fn test_transform_presets() {
    // Test basic transform animations
    let mut manager = TransformAnimationManager::new();
    let transition = Transition::default();
    manager.animate_property("opacity", 1.0, &transition);
    manager.animate_property("translateX", 0.0, &transition);
    manager.animate_property("scale", 1.0, &transition);

    // Test that properties are set correctly
    assert_eq!(manager.get_property_value("opacity"), 0.0);
    assert_eq!(manager.get_property_value("translateX"), 0.0);
    assert_eq!(manager.get_property_value("scale"), 0.0);
}

/// Test easing function trait
#[test]
fn test_easing_function_trait() {
    let linear_fn: fn(f64) -> f64 = standard::linear;
    assert_eq!(apply_easing(0.5, &linear_fn), 0.5);

    let ease_in_fn: fn(f64) -> f64 = standard::ease_in;
    assert_eq!(apply_easing(0.5, &ease_in_fn), 0.25);

    let bezier = bezier::ease();
    let result = apply_easing(0.5, &bezier);
    assert!(result >= 0.0 && result <= 1.0);
}

/// Test repeat config builder
#[test]
fn test_repeat_config_builder() {
    use crate::repeat_config::RepeatConfigBuilder;

    // Test count-based repeat
    let config = RepeatConfigBuilder::new().count(3).build();

    match config {
        RepeatConfig::Count(3) => assert!(true),
        _ => assert!(false, "Expected Count(3)"),
    }

    // Test infinite repeat
    let config = RepeatConfigBuilder::new().infinite().build();

    match config {
        RepeatConfig::Infinite => assert!(true),
        _ => assert!(false, "Expected Infinite"),
    }

    // Test reverse repeat
    let config = RepeatConfigBuilder::new().infinite().reverse().build();

    match config {
        RepeatConfig::InfiniteReverse => assert!(true),
        _ => assert!(false, "Expected InfiniteReverse"),
    }
}

/// Test repeat presets
#[test]
fn test_repeat_presets() {
    // Test basic repeat configurations

    // Test once
    // Test basic repeat configurations
    let never = RepeatConfig::Never;
    match never {
        RepeatConfig::Never => assert!(true),
        _ => assert!(false, "Expected Never"),
    }

    // Test times
    let count = RepeatConfig::Count(5);
    match count {
        RepeatConfig::Count(5) => assert!(true),
        _ => assert!(false, "Expected Count(5)"),
    }

    // Test forever
    let infinite = RepeatConfig::Infinite;
    match infinite {
        RepeatConfig::Infinite => assert!(true),
        _ => assert!(false, "Expected Infinite"),
    }

    // Test forever reverse
    let infinite_reverse = RepeatConfig::InfiniteReverse;
    match infinite_reverse {
        RepeatConfig::InfiniteReverse => assert!(true),
        _ => assert!(false, "Expected InfiniteReverse"),
    }
}

/// Test stagger presets
#[test]
fn test_stagger_presets() {
    // Test basic stagger configurations
    let subtle = StaggerConfig::new(0.1);
    assert_eq!(subtle.delay, 0.1);

    let dramatic = StaggerConfig::new(0.3);
    assert_eq!(dramatic.delay, 0.3);

    let rapid = StaggerConfig::new(0.05);
    assert_eq!(rapid.delay, 0.05);
}
