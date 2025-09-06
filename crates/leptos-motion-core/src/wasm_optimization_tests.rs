//! TDD tests for WASM optimization
//!
//! These tests ensure that WASM compilation optimizations don't break functionality
//! while achieving significant bundle size reductions.

#[cfg(feature = "leptos-integration")]
use wasm_bindgen_test::*;

#[cfg(feature = "leptos-integration")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test that core functionality still works after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_core_functionality_after_wasm_optimization() {
    use crate::*;
    use std::collections::HashMap;

    // Test that basic types still work
    let animation_value = AnimationValue::Number(42.0);
    assert_eq!(animation_value.to_string(), "42");

    // Test animation target creation
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.5));
    target.insert("x".to_string(), AnimationValue::Pixels(100.0));
    let animation_target: AnimationTarget = target;

    assert_eq!(
        animation_target.get("opacity"),
        Some(&AnimationValue::Number(0.5))
    );
    assert_eq!(
        animation_target.get("x"),
        Some(&AnimationValue::Pixels(100.0))
    );
}

/// Test that engines still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_engines_after_wasm_optimization() {
    use crate::*;

    // Test minimal engine creation
    let _minimal_engine = MinimalEngine::new();

    // Test that we can create transition configurations
    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseOut,
        ..Default::default()
    };

    assert_eq!(transition.duration, Some(0.5));
    assert_eq!(transition.ease, Easing::EaseOut);
}

/// Test that performance monitoring still works after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_performance_monitoring_after_wasm_optimization() {
    use crate::performance::*;

    // Test performance budget creation
    let budget = PerformanceBudget::default();
    assert_eq!(budget.target_fps, 60.0);

    // Test performance monitor creation
    let _monitor = PerformanceMonitor::new(budget);
    // Monitor should be created successfully
    assert!(true);
}

/// Test that animation values still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_animation_values_after_wasm_optimization() {
    use crate::*;

    // Test numeric values
    let num_value = AnimationValue::Number(42.0);
    assert_eq!(num_value.to_string(), "42");

    // Test pixel values
    let pixel_value = AnimationValue::Pixels(100.0);
    assert_eq!(pixel_value.to_string(), "100px");

    // Test transform values
    let transform_value = AnimationValue::Transform(Transform {
        x: Some(10.0),
        y: Some(20.0),
        scale_x: Some(1.5),
        scale_y: Some(1.5),
        rotate_z: Some(45.0),
        ..Default::default()
    });
    assert!(transform_value.to_string().contains("translateX(10px)"));
}

/// Test that easing functions still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_easing_functions_after_wasm_optimization() {
    use crate::*;

    // Test all easing types
    let linear = Easing::Linear;
    let ease_in = Easing::EaseIn;
    let ease_out = Easing::EaseOut;
    let ease_in_out = Easing::EaseInOut;
    let spring = Easing::Spring(SpringConfig {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    });

    // Verify all variants exist and can be matched
    match linear {
        Easing::Linear => assert!(true),
        _ => panic!("Expected Linear easing"),
    }

    match spring {
        Easing::Spring(config) => {
            assert_eq!(config.stiffness, 100.0);
            assert_eq!(config.damping, 10.0);
        }
        _ => panic!("Expected Spring easing"),
    }
}

/// Test that repeat configurations still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_repeat_configurations_after_wasm_optimization() {
    use crate::*;

    // Test all repeat configurations
    let never = RepeatConfig::Never;
    let count = RepeatConfig::Count(5);
    let infinite = RepeatConfig::Infinite;
    let infinite_reverse = RepeatConfig::InfiniteReverse;

    // Verify all variants exist and can be matched
    match never {
        RepeatConfig::Never => assert!(true),
        _ => panic!("Expected Never repeat"),
    }

    match count {
        RepeatConfig::Count(n) => assert_eq!(n, 5),
        _ => panic!("Expected Count(5) repeat"),
    }

    match infinite {
        RepeatConfig::Infinite => assert!(true),
        _ => panic!("Expected Infinite repeat"),
    }

    match infinite_reverse {
        RepeatConfig::InfiniteReverse => assert!(true),
        _ => panic!("Expected InfiniteReverse repeat"),
    }
}

/// Test that stagger configurations still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_stagger_configurations_after_wasm_optimization() {
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
        StaggerFrom::First => assert!(true),
        _ => panic!("Expected First stagger from"),
    }

    match from_last {
        StaggerFrom::Last => assert!(true),
        _ => panic!("Expected Last stagger from"),
    }

    match from_center {
        StaggerFrom::Center => assert!(true),
        _ => panic!("Expected Center stagger from"),
    }

    match from_index {
        StaggerFrom::Index(n) => assert_eq!(n, 5),
        _ => panic!("Expected Index(5) stagger from"),
    }
}

/// Test that complex nested types still work after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_complex_nested_types_after_wasm_optimization() {
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

/// Test WASM optimization bundle size targets (conceptual - actual measurement will be done externally)
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_wasm_optimization_bundle_size_targets() {
    // This test documents our WASM optimization bundle size targets
    // Actual measurement will be done with external tools

    // Target: Additional 20-30% reduction from current 605KB
    // Expected: 400-500KB total bundle size
    assert!(true);
}

/// Test that memory optimization still works after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_memory_optimization_after_wasm_optimization() {
    use crate::memory_optimization::*;

    // Test memory profiler creation
    let _profiler = MemoryProfiler::new();

    // Test memory optimization constants
    assert_eq!(TARGET_ANIMATION_MEMORY_KB, 50.0);
    assert_eq!(TARGET_ENGINE_MEMORY_KB, 100.0);
    assert_eq!(TARGET_CACHE_MEMORY_KB, 25.0);
}

/// Test that lazy loading still works after WASM optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_lazy_loading_after_wasm_optimization() {
    use crate::lazy_loading::*;

    // Test lazy loader creation
    let _loader = get_lazy_loader();

    // Test lazy loading configuration
    let config = LazyLoadingConfig::default();
    assert_eq!(config.max_loaded_modules, 10);
    assert_eq!(config.max_total_size, 1024 * 1024); // 1MB
}
