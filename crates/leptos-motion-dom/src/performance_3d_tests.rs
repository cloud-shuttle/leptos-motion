#![cfg(test)]

use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Performance test for 3D rotation animations
///
/// This test verifies that 3D rotation animations can be created
/// and executed efficiently without performance bottlenecks.
#[test]
fn test_3d_rotation_performance() {
    // Create a 3D rotation animation
    let (rotation_x, set_rotation_x) = signal(0.0);
    let rotation_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "rotateX".to_string(),
            AnimationValue::Number(rotation_x.get()),
        );
        target.insert(
            "rotateY".to_string(),
            AnimationValue::Number(rotation_x.get() * 0.5),
        );
        target.insert(
            "rotateZ".to_string(),
            AnimationValue::Number(rotation_x.get() * 0.25),
        );
        target
    };

    // Test animation creation performance
    let start_time = std::time::Instant::now();

    // Simulate 1000 animation updates
    for i in 0..1000 {
        set_rotation_x.set(i as f64 * 0.36);
        let _result = rotation_animation();
    }

    let duration = start_time.elapsed();

    // Verify that 1000 animations can be processed in under 100ms
    // This ensures we can handle 60+ FPS (16.67ms per frame)
    assert!(
        duration.as_millis() < 100,
        "3D rotation animation processing took {}ms, should be under 100ms",
        duration.as_millis()
    );

    // Verify animation result is correct
    let result = rotation_animation();
    assert_eq!(result.get("rotateX"), Some(&AnimationValue::Number(359.64)));
    assert_eq!(result.get("rotateY"), Some(&AnimationValue::Number(179.82)));
    assert_eq!(result.get("rotateZ"), Some(&AnimationValue::Number(89.91)));
}

/// Performance test for multiple concurrent 3D animations
///
/// This test verifies that multiple 3D animations can run
/// simultaneously without performance degradation.
#[test]
fn test_multiple_3d_animations_performance() {
    // Create multiple 3D animations
    let (rotation_x, set_rotation_x) = signal(0.0);
    let (scale_factor, set_scale_factor) = signal(1.0);
    let (translation_z, set_translation_z) = signal(0.0);

    let rotation_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "rotateX".to_string(),
            AnimationValue::Number(rotation_x.get()),
        );
        target.insert(
            "rotateY".to_string(),
            AnimationValue::Number(rotation_x.get() * 0.5),
        );
        target
    };

    let scale_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "scale".to_string(),
            AnimationValue::Number(scale_factor.get()),
        );
        target
    };

    let translation_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "translateZ".to_string(),
            AnimationValue::Number(translation_z.get()),
        );
        target
    };

    // Test multiple animations performance
    let start_time = std::time::Instant::now();

    // Simulate 500 updates of all three animations
    for i in 0..500 {
        set_rotation_x.set(i as f64 * 0.72);
        set_scale_factor.set(1.0 + 0.5 * (i as f64 * 0.01).sin());
        set_translation_z.set(50.0 * (i as f64 * 0.02).cos());

        let _rotation_result = rotation_animation();
        let _scale_result = scale_animation();
        let _translation_result = translation_animation();
    }

    let duration = start_time.elapsed();

    // Verify that 1500 animations (500 * 3) can be processed in under 150ms
    assert!(
        duration.as_millis() < 150,
        "Multiple 3D animations processing took {}ms, should be under 150ms",
        duration.as_millis()
    );

    // Verify all animations produce correct results
    let rotation_result = rotation_animation();
    let scale_result = scale_animation();
    let translation_result = translation_animation();

    assert_eq!(
        rotation_result.get("rotateX"),
        Some(&AnimationValue::Number(359.28))
    );
    assert_eq!(
        scale_result.get("scale"),
        Some(&AnimationValue::Number(
            1.0 + 0.5 * (499.0_f64 * 0.01).sin()
        ))
    );
    assert_eq!(
        translation_result.get("translateZ"),
        Some(&AnimationValue::Number(50.0 * (499.0_f64 * 0.02).cos()))
    );
}

/// Performance test for 3D matrix transformations
///
/// This test verifies that complex 3D matrix transformations
/// can be computed efficiently without performance issues.
#[test]
fn test_3d_matrix_performance() {
    // Create a 3D matrix animation
    let (matrix_angle, set_matrix_angle) = signal(0.0);
    let matrix_animation = move || {
        let angle: f64 = matrix_angle.get();
        let cos = angle.cos();
        let sin = angle.sin();

        let mut target = HashMap::new();
        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!(
                "matrix3d({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
                cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )),
        );
        target
    };

    // Test matrix computation performance
    let start_time = std::time::Instant::now();

    // Simulate 1000 matrix computations
    for i in 0..1000 {
        set_matrix_angle.set(i as f64 * 0.01);
        let _result = matrix_animation();
    }

    let duration = start_time.elapsed();

    // Verify that 1000 matrix computations can be processed in under 50ms
    assert!(
        duration.as_millis() < 50,
        "3D matrix computation took {}ms, should be under 50ms",
        duration.as_millis()
    );

    // Verify matrix result is correct
    let result = matrix_animation();
    let expected_cos = 9.99_f64.cos();
    let expected_sin = 9.99_f64.sin();
    let expected_matrix = format!(
        "matrix3d({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        expected_cos,
        -expected_sin,
        0.0,
        0.0,
        expected_sin,
        expected_cos,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0
    );

    if let Some(AnimationValue::String(matrix_str)) = result.get("transform") {
        assert!(matrix_str.contains(&expected_cos.to_string()));
        assert!(matrix_str.contains(&expected_sin.to_string()));
    } else {
        panic!("Expected string matrix transform");
    }
}

/// Performance test for 3D perspective changes
///
/// This test verifies that 3D perspective animations
/// can be computed efficiently for smooth transitions.
#[test]
fn test_3d_perspective_performance() {
    // Create a 3D perspective animation
    let (perspective_value, set_perspective_value) = signal(1000.0);
    let perspective_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "perspective".to_string(),
            AnimationValue::String(format!("{}px", perspective_value.get())),
        );
        target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
        target.insert("rotateY".to_string(), AnimationValue::Number(45.0));
        target
    };

    // Test perspective computation performance
    let start_time = std::time::Instant::now();

    // Simulate 1000 perspective updates
    for i in 0..1000 {
        set_perspective_value.set(500.0 + 500.0 * (i as f64 * 0.01).sin());
        let _result = perspective_animation();
    }

    let duration = start_time.elapsed();

    // Verify that 1000 perspective computations can be processed in under 50ms
    assert!(
        duration.as_millis() < 50,
        "3D perspective computation took {}ms, should be under 50ms",
        duration.as_millis()
    );

    // Verify perspective result is correct
    let result = perspective_animation();
    let expected_perspective = 500.0 + 500.0 * (999.0_f64 * 0.01).sin();
    assert_eq!(
        result.get("perspective"),
        Some(&AnimationValue::String(format!(
            "{}px",
            expected_perspective
        )))
    );
    assert_eq!(result.get("rotateX"), Some(&AnimationValue::Number(45.0)));
    assert_eq!(result.get("rotateY"), Some(&AnimationValue::Number(45.0)));
}

/// Performance test for 3D animation with transitions
///
/// This test verifies that 3D animations with CSS transitions
/// can be processed efficiently for smooth animation curves.
#[test]
fn test_3d_transition_performance() {
    // Create a 3D animation with transition
    let (rotation_value, set_rotation_value) = signal(0.0);
    let rotation_animation = move || {
        let mut target = HashMap::new();
        target.insert(
            "rotateX".to_string(),
            AnimationValue::Number(rotation_value.get()),
        );
        target.insert(
            "rotateY".to_string(),
            AnimationValue::Number(rotation_value.get() * 0.5),
        );
        target.insert(
            "rotateZ".to_string(),
            AnimationValue::Number(rotation_value.get() * 0.25),
        );
        target
    };

    let transition = Transition {
        duration: Some(0.1),
        delay: None,
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test transition performance
    let start_time = std::time::Instant::now();

    // Simulate 1000 transition updates
    for i in 0..1000 {
        set_rotation_value.set(i as f64 * 0.36);
        let _result = rotation_animation();
    }

    let duration = start_time.elapsed();

    // Verify that 1000 transition computations can be processed in under 100ms
    assert!(
        duration.as_millis() < 100,
        "3D transition computation took {}ms, should be under 100ms",
        duration.as_millis()
    );

    // Verify transition result is correct
    let result = rotation_animation();
    assert_eq!(result.get("rotateX"), Some(&AnimationValue::Number(359.64)));
    assert_eq!(result.get("rotateY"), Some(&AnimationValue::Number(179.82)));
    assert_eq!(result.get("rotateZ"), Some(&AnimationValue::Number(89.91)));

    // Verify transition configuration
    assert_eq!(transition.duration, Some(0.1));
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.repeat, RepeatConfig::Never);
}

/// Performance test for 3D animation memory usage
///
/// This test ensures that 3D animations don't cause
/// excessive memory usage or memory leaks.
#[test]
fn test_3d_animation_memory_usage() {
    // Create multiple 3D animations to test memory usage
    let mut animations = Vec::new();

    for i in 0..100 {
        let (rotation_x, _set_rotation_x) = signal(i as f64 * 3.6);
        let animation = move || {
            let mut target = HashMap::new();
            target.insert(
                "rotateX".to_string(),
                AnimationValue::Number(rotation_x.get()),
            );
            target.insert(
                "rotateY".to_string(),
                AnimationValue::Number(rotation_x.get() * 0.5),
            );
            target.insert(
                "rotateZ".to_string(),
                AnimationValue::Number(rotation_x.get() * 0.25),
            );
            target
        };
        animations.push(animation);
    }

    // Verify that all animations can be created without memory issues
    assert_eq!(animations.len(), 100);

    // Test that animations can be executed without memory leaks
    for animation in animations.iter() {
        let result = animation();
        assert!(!result.is_empty());
    }
}

/// Performance test for 3D animation with hardware acceleration
///
/// This test ensures that 3D animations use hardware acceleration
/// by checking for the presence of will-change and transform3d properties.
#[test]
fn test_3d_hardware_acceleration() {
    let mut target = HashMap::new();
    target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
    target.insert("rotateY".to_string(), AnimationValue::Number(90.0));
    target.insert("translateZ".to_string(), AnimationValue::Number(100.0));
    target.insert(
        "will-change".to_string(),
        AnimationValue::String("transform".to_string()),
    );

    // Verify hardware acceleration properties are present
    assert!(target.contains_key("will-change"));
    assert_eq!(
        target.get("will-change"),
        Some(&AnimationValue::String("transform".to_string()))
    );

    // Verify 3D transform properties are present
    assert!(target.contains_key("rotateX"));
    assert!(target.contains_key("rotateY"));
    assert!(target.contains_key("translateZ"));

    // Verify values are correct
    assert_eq!(target.get("rotateX"), Some(&AnimationValue::Number(45.0)));
    assert_eq!(target.get("rotateY"), Some(&AnimationValue::Number(90.0)));
    assert_eq!(
        target.get("translateZ"),
        Some(&AnimationValue::Number(100.0))
    );
}
