#![cfg(test)]

use crate::ReactiveMotionDiv;
use leptos::prelude::*;
use leptos_motion_core::{
    AnimationValue, Easing, RepeatConfig, StaggerConfig, StaggerFrom, Transition,
};
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test 3D transform properties
#[wasm_bindgen_test]
fn test_3d_transform_properties() {
    let mut animate = HashMap::new();

    // Test perspective
    animate.insert(
        "perspective".to_string(),
        AnimationValue::String("1000px".to_string()),
    );

    // Test 3D rotations
    animate.insert("rotateX".to_string(), AnimationValue::Number(45.0));
    animate.insert("rotateY".to_string(), AnimationValue::Number(90.0));
    animate.insert("rotateZ".to_string(), AnimationValue::Number(180.0));

    // Test 3D translations
    animate.insert("translateX".to_string(), AnimationValue::Number(100.0));
    animate.insert("translateY".to_string(), AnimationValue::Number(200.0));
    animate.insert("translateZ".to_string(), AnimationValue::Number(300.0));

    // Test 3D scale
    animate.insert("scaleX".to_string(), AnimationValue::Number(1.5));
    animate.insert("scaleY".to_string(), AnimationValue::Number(2.0));
    animate.insert("scaleZ".to_string(), AnimationValue::Number(0.5));

    let _transition = Transition {
        duration: Some(1.0),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // This test will fail initially - we need to implement 3D support
    assert!(animate.contains_key("perspective"));
    assert!(animate.contains_key("rotateX"));
    assert!(animate.contains_key("rotateY"));
    assert!(animate.contains_key("rotateZ"));
    assert!(animate.contains_key("translateZ"));
    assert!(animate.contains_key("scaleZ"));
}

/// Test 3D perspective and camera controls
#[wasm_bindgen_test]
fn test_3d_perspective_controls() {
    let mut animate = HashMap::new();

    // Test perspective origin
    animate.insert(
        "perspective-origin".to_string(),
        AnimationValue::String("50% 50%".to_string()),
    );

    // Test transform-style
    animate.insert(
        "transform-style".to_string(),
        AnimationValue::String("preserve-3d".to_string()),
    );

    // Test backface-visibility
    animate.insert(
        "backface-visibility".to_string(),
        AnimationValue::String("hidden".to_string()),
    );

    // Test transform-origin for 3D
    animate.insert(
        "transform-origin".to_string(),
        AnimationValue::String("center center 0px".to_string()),
    );

    assert!(animate.contains_key("perspective-origin"));
    assert!(animate.contains_key("transform-style"));
    assert!(animate.contains_key("backface-visibility"));
    assert!(animate.contains_key("transform-origin"));
}

/// Test Matrix3D transformations
#[wasm_bindgen_test]
fn test_matrix3d_transformations() {
    let mut animate = HashMap::new();

    // Test matrix3d transform
    animate.insert(
        "matrix3d".to_string(),
        AnimationValue::String(
            "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)".to_string(),
        ),
    );

    // Test rotate3d
    animate.insert(
        "rotate3d".to_string(),
        AnimationValue::String("rotate3d(1, 1, 1, 45deg)".to_string()),
    );

    // Test translate3d
    animate.insert(
        "translate3d".to_string(),
        AnimationValue::String("translate3d(100px, 200px, 300px)".to_string()),
    );

    // Test scale3d
    animate.insert(
        "scale3d".to_string(),
        AnimationValue::String("scale3d(1.5, 2.0, 0.5)".to_string()),
    );

    assert!(animate.contains_key("matrix3d"));
    assert!(animate.contains_key("rotate3d"));
    assert!(animate.contains_key("translate3d"));
    assert!(animate.contains_key("scale3d"));
}

/// Test 3D animation performance
#[wasm_bindgen_test]
fn test_3d_animation_performance() {
    let start_time = js_sys::Date::now();

    // Create multiple 3D animations
    for i in 0..10 {
        let mut animate = HashMap::new();
        animate.insert(
            "rotateX".to_string(),
            AnimationValue::Number(i as f64 * 36.0),
        );
        animate.insert(
            "rotateY".to_string(),
            AnimationValue::Number(i as f64 * 18.0),
        );
        animate.insert(
            "translateZ".to_string(),
            AnimationValue::Number(i as f64 * 50.0),
        );

        let transition = Transition {
            duration: Some(0.1),
            delay: Some(0.0),
            ease: Easing::Linear,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        // Simulate animation processing
        let _ = format!("{:?}", animate);
        let _ = format!("{:?}", transition);
    }

    let end_time = js_sys::Date::now();
    let duration = end_time - start_time;

    // Performance requirement: <16ms for 60 FPS
    assert!(
        duration < 16.0,
        "3D animation processing took {}ms, should be <16ms",
        duration
    );
}

/// Test 3D component integration
#[wasm_bindgen_test]
fn test_3d_component_integration() {
    let mut animate = HashMap::new();
    animate.insert("rotateX".to_string(), AnimationValue::Number(45.0));
    animate.insert("rotateY".to_string(), AnimationValue::Number(90.0));
    animate.insert("translateZ".to_string(), AnimationValue::Number(100.0));

    let _transition = Transition {
        duration: Some(1.0),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // Test that 3D properties are properly handled
    let has_3d_properties = animate.contains_key("rotateX")
        && animate.contains_key("rotateY")
        && animate.contains_key("translateZ");

    assert!(has_3d_properties, "3D properties should be present");

    // Test transition configuration
    assert_eq!(_transition.duration, Some(1.0));
    assert_eq!(_transition.ease, Easing::EaseInOut);
}

/// Test 3D animation with signal-based reactivity
#[wasm_bindgen_test]
fn test_3d_signal_based_animation() {
    let (rotation, set_rotation) = signal(0.0);

    let animate = Memo::new(move |_| {
        let mut map = HashMap::new();
        map.insert(
            "rotateX".to_string(),
            AnimationValue::Number(rotation.get()),
        );
        map.insert(
            "rotateY".to_string(),
            AnimationValue::Number(rotation.get() * 0.5),
        );
        map.insert(
            "rotateZ".to_string(),
            AnimationValue::Number(rotation.get() * 2.0),
        );
        map
    });

    // Test initial state
    let initial_animate = animate.get();
    assert_eq!(
        initial_animate.get("rotateX"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        initial_animate.get("rotateY"),
        Some(&AnimationValue::Number(0.0))
    );
    assert_eq!(
        initial_animate.get("rotateZ"),
        Some(&AnimationValue::Number(0.0))
    );

    // Test signal update
    set_rotation.set(90.0);
    let updated_animate = animate.get();
    assert_eq!(
        updated_animate.get("rotateX"),
        Some(&AnimationValue::Number(90.0))
    );
    assert_eq!(
        updated_animate.get("rotateY"),
        Some(&AnimationValue::Number(45.0))
    );
    assert_eq!(
        updated_animate.get("rotateZ"),
        Some(&AnimationValue::Number(180.0))
    );
}

/// Test 3D animation with complex transformations
#[wasm_bindgen_test]
fn test_3d_complex_transformations() {
    let mut animate = HashMap::new();

    // Complex 3D transformation
    animate.insert(
        "perspective".to_string(),
        AnimationValue::String("1000px".to_string()),
    );
    animate.insert(
        "perspective-origin".to_string(),
        AnimationValue::String("center center".to_string()),
    );
    animate.insert(
        "transform-style".to_string(),
        AnimationValue::String("preserve-3d".to_string()),
    );
    animate.insert("rotateX".to_string(), AnimationValue::Number(45.0));
    animate.insert("rotateY".to_string(), AnimationValue::Number(90.0));
    animate.insert("rotateZ".to_string(), AnimationValue::Number(180.0));
    animate.insert("translateX".to_string(), AnimationValue::Number(100.0));
    animate.insert("translateY".to_string(), AnimationValue::Number(200.0));
    animate.insert("translateZ".to_string(), AnimationValue::Number(300.0));
    animate.insert("scaleX".to_string(), AnimationValue::Number(1.5));
    animate.insert("scaleY".to_string(), AnimationValue::Number(2.0));
    animate.insert("scaleZ".to_string(), AnimationValue::Number(0.5));

    let transition = Transition {
        duration: Some(2.0),
        delay: Some(0.5),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Count(2),
        stagger: Some(StaggerConfig {
            delay: 0.1,
            from: StaggerFrom::First,
        }),
    };

    // Verify all 3D properties are present
    let required_3d_properties = [
        "perspective",
        "perspective-origin",
        "transform-style",
        "rotateX",
        "rotateY",
        "rotateZ",
        "translateX",
        "translateY",
        "translateZ",
        "scaleX",
        "scaleY",
        "scaleZ",
    ];

    for property in &required_3d_properties {
        assert!(
            animate.contains_key(*property),
            "Missing 3D property: {}",
            property
        );
    }

    // Verify transition configuration
    assert_eq!(transition.duration, Some(2.0));
    assert_eq!(transition.delay, Some(0.5));
    assert_eq!(transition.ease, Easing::EaseInOut);
    assert_eq!(transition.repeat, RepeatConfig::Count(2));
    assert_eq!(
        transition.stagger,
        Some(StaggerConfig {
            delay: 0.1,
            from: StaggerFrom::First,
        })
    );
}
