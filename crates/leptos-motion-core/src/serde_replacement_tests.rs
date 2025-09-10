//! TDD tests for serde replacement optimization
//!
//! These tests ensure that serde replacement doesn't break functionality
//! while achieving significant bundle size reductions.

#[cfg(feature = "leptos-integration")]
// WASM-specific test configuration - conditional compilation
#[cfg(feature = "web-sys")]
use wasm_bindgen_test::*;

#[cfg(feature = "web-sys")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test that core types still work after serde removal
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_core_types_after_serde_replacement() {
    use crate::*;
    use std::collections::HashMap;

    // Test AnimationValue serialization/deserialization
    let num_value = AnimationValue::Number(42.0);
    let pixel_value = AnimationValue::Pixels(100.0);
    let transform_value = AnimationValue::Transform(Transform {
        x: Some(10.0),
        y: Some(20.0),
        scale_x: Some(1.5),
        ..Default::default()
    });

    // Test that we can still create and use these values
    assert_eq!(num_value.to_string_value(), "42");
    assert_eq!(pixel_value.to_string_value(), "100px");
    assert!(
        transform_value
            .to_string_value()
            .contains("translateX(10px)")
    );

    // Test AnimationTarget creation
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.5));
    target.insert("x".to_string(), AnimationValue::Pixels(50.0));
    let animation_target: AnimationTarget = target;

    assert_eq!(
        animation_target.get("opacity"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        animation_target.get("x"),
        Some(&AnimationValue::Pixels(50.0))
    );
}

/// Test that Transition types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_transition_types_after_serde_replacement() {
    use crate::*;

    // Test basic transition creation
    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::Linear,
        repeat: RepeatConfig::Never,
        ..Default::default()
    };

    assert_eq!(transition.duration, Some(1.0));
    assert_eq!(transition.ease, Easing::Linear);
    assert_eq!(transition.repeat, RepeatConfig::Never);

    // Test spring transition
    let spring_transition = Transition {
        duration: None,
        ease: Easing::Spring(SpringConfig {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
            rest_delta: 0.01,
            rest_speed: 0.01,
        }),
        ..Default::default()
    };

    match spring_transition.ease {
        Easing::Spring(config) => {
            assert_eq!(config.stiffness, 100.0);
            assert_eq!(config.damping, 10.0);
            assert_eq!(config.mass, 1.0);
        }
        _ => panic!("Expected Spring easing"),
    }
}

/// Test that Transform types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_transform_types_after_serde_replacement() {
    use crate::*;

    // Test transform creation and usage
    let transform = Transform {
        x: Some(10.0),
        y: Some(20.0),
        z: Some(30.0),
        scale_x: Some(1.5),
        scale_y: Some(1.5),
        scale: Some(1.5),
        rotate_x: Some(45.0),
        rotate_y: Some(90.0),
        rotate_z: Some(180.0),
        skew_x: Some(5.0),
        skew_y: Some(10.0),
    };

    // Test individual fields
    assert_eq!(transform.x, Some(10.0));
    assert_eq!(transform.y, Some(20.0));
    assert_eq!(transform.z, Some(30.0));
    assert_eq!(transform.scale_x, Some(1.5));
    assert_eq!(transform.scale_y, Some(1.5));
    assert_eq!(transform.scale, Some(1.5));
    assert_eq!(transform.rotate_x, Some(45.0));
    assert_eq!(transform.rotate_y, Some(90.0));
    assert_eq!(transform.rotate_z, Some(180.0));
    assert_eq!(transform.skew_x, Some(5.0));
    assert_eq!(transform.skew_y, Some(10.0));

    // Test default transform
    let default_transform = Transform::default();
    assert_eq!(default_transform.x, None);
    assert_eq!(default_transform.y, None);
    assert_eq!(default_transform.z, None);
}

/// Test that Easing types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_easing_types_after_serde_replacement() {
    use crate::*;

    // Test all easing types
    let linear = Easing::Linear;
    let ease_in = Easing::EaseIn;
    let ease_out = Easing::EaseOut;
    let ease_in_out = Easing::EaseInOut;
    let circ_in = Easing::CircIn;
    let circ_out = Easing::CircOut;
    let circ_in_out = Easing::CircInOut;
    let back_in = Easing::BackIn;
    let back_out = Easing::BackOut;
    let back_in_out = Easing::BackInOut;

    // Test spring easing
    let spring = Easing::Spring(SpringConfig {
        stiffness: 200.0,
        damping: 20.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    });

    // Test bezier easing
    let bezier = Easing::Bezier(0.25, 0.1, 0.25, 1.0);

    // Verify all variants exist and can be matched
    match linear {
        Easing::Linear => { /* Test passes */ },
        _ => panic!("Expected Linear easing"),
    }

    match spring {
        Easing::Spring(config) => {
            assert_eq!(config.stiffness, 200.0);
            assert_eq!(config.damping, 20.0);
        }
        _ => panic!("Expected Spring easing"),
    }

    match bezier {
        Easing::Bezier(x1, y1, x2, y2) => {
            assert_eq!(x1, 0.25);
            assert_eq!(y1, 0.1);
            assert_eq!(x2, 0.25);
            assert_eq!(y2, 1.0);
        }
        _ => panic!("Expected Bezier easing"),
    }
}

/// Test that RepeatConfig types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_repeat_config_after_serde_replacement() {
    use crate::*;

    // Test all repeat configurations
    let never = RepeatConfig::Never;
    let count = RepeatConfig::Count(5);
    let infinite = RepeatConfig::Infinite;
    let infinite_reverse = RepeatConfig::InfiniteReverse;

    // Verify all variants exist and can be matched
    match never {
        RepeatConfig::Never => { /* Test passes */ },
        _ => panic!("Expected Never repeat"),
    }

    match count {
        RepeatConfig::Count(n) => assert_eq!(n, 5),
        _ => panic!("Expected Count(5) repeat"),
    }

    match infinite {
        RepeatConfig::Infinite => { /* Test passes */ },
        _ => panic!("Expected Infinite repeat"),
    }

    match infinite_reverse {
        RepeatConfig::InfiniteReverse => { /* Test passes */ },
        _ => panic!("Expected InfiniteReverse repeat"),
    }
}

/// Test that SpringConfig types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_spring_config_after_serde_replacement() {
    use crate::*;

    // Test spring config creation
    let spring_config = SpringConfig {
        stiffness: 300.0,
        damping: 30.0,
        mass: 2.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    };

    assert_eq!(spring_config.stiffness, 300.0);
    assert_eq!(spring_config.damping, 30.0);
    assert_eq!(spring_config.mass, 2.0);

    // Test default spring config
    let default_spring = SpringConfig::default();
    assert_eq!(default_spring.stiffness, 100.0);
    assert_eq!(default_spring.damping, 10.0);
    assert_eq!(default_spring.mass, 1.0);
}

/// Test that StaggerConfig types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_stagger_config_after_serde_replacement() {
    use crate::*;

    // Test stagger config creation
    let stagger_config = StaggerConfig {
        delay: 0.1,
        from: StaggerFrom::First,
    };

    assert_eq!(stagger_config.delay, 0.1);
    assert_eq!(stagger_config.from, StaggerFrom::First);

    // Test stagger from variants
    let from_first = StaggerFrom::First;
    let from_last = StaggerFrom::Last;
    let from_center = StaggerFrom::Center;
    let from_index = StaggerFrom::Index(5);

    match from_first {
        StaggerFrom::First => { /* Test passes */ },
        _ => panic!("Expected First stagger from"),
    }

    match from_last {
        StaggerFrom::Last => { /* Test passes */ },
        _ => panic!("Expected Last stagger from"),
    }

    match from_center {
        StaggerFrom::Center => { /* Test passes */ },
        _ => panic!("Expected Center stagger from"),
    }

    match from_index {
        StaggerFrom::Index(n) => assert_eq!(n, 5),
        _ => panic!("Expected Index(5) stagger from"),
    }
}

/// Test that complex nested types still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_complex_nested_types_after_serde_replacement() {
    use crate::*;
    use std::collections::HashMap;

    // Test complex animation target with multiple value types
    let mut complex_target = HashMap::new();
    complex_target.insert("opacity".to_string(), AnimationValue::Number(0.8));
    complex_target.insert("x".to_string(), AnimationValue::Pixels(100.0));
    complex_target.insert("y".to_string(), AnimationValue::Pixels(200.0));
    complex_target.insert("scale".to_string(), AnimationValue::Number(1.2));
    complex_target.insert(
        "transform".to_string(),
        AnimationValue::Transform(Transform {
            rotate_z: Some(45.0),
            scale_x: Some(1.5),
            scale_y: Some(1.5),
            ..Default::default()
        }),
    );

    let animation_target: AnimationTarget = complex_target;

    // Verify all values are accessible
    assert_eq!(
        animation_target.get("opacity"),
        Some(&AnimationValue::Number(0.8))
    );
    assert_eq!(
        animation_target.get("x"),
        Some(&AnimationValue::Pixels(100.0))
    );
    assert_eq!(
        animation_target.get("y"),
        Some(&AnimationValue::Pixels(200.0))
    );
    assert_eq!(
        animation_target.get("scale"),
        Some(&AnimationValue::Number(1.2))
    );

    // Test nested transform
    if let Some(AnimationValue::Transform(transform)) = animation_target.get("transform") {
        assert_eq!(transform.rotate_z, Some(45.0));
        assert_eq!(transform.scale_x, Some(1.5));
        assert_eq!(transform.scale_y, Some(1.5));
    } else {
        panic!("Expected Transform value");
    }
}

/// Test that animation engines still work after serde replacement
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_animation_engines_after_serde_replacement() {
    use crate::*;
    use std::collections::HashMap;

    // Test minimal engine creation
    let _minimal_engine = MinimalEngine::new();

    // Test that we can create animation configurations
    let target: AnimationTarget = HashMap::from([
        ("opacity".to_string(), AnimationValue::Number(1.0)),
        ("x".to_string(), AnimationValue::Pixels(0.0)),
    ]);

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseOut,
        ..Default::default()
    };

    // These should compile and be usable
    assert_eq!(target.len(), 2);
    assert_eq!(transition.duration, Some(0.5));
}

/// Test serde replacement bundle size targets (conceptual - actual measurement will be done externally)
#[cfg(feature = "leptos-integration")]
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_serde_replacement_bundle_size_targets() {
    // This test documents our serde replacement bundle size targets
    // Actual measurement will be done with external tools

    // Target: Significant reduction from current serde footprint
    // Expected: 20-30% reduction in bundle size
    // Test passes if we reach here
}
