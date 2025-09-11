//! TDD Animation Reactivity Tests
//!
//! This test suite drives the implementation of proper animation reactivity
//! using Test-Driven Development principles.
//!
//! RED PHASE: Write failing tests that define the desired behavior
//! GREEN PHASE: Implement minimal code to make tests pass
//! REFACTOR PHASE: Improve implementation while keeping tests green

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Easing, Transition};
use leptos_motion_dom::{MotionDiv, SignalBasedAnimationController};
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to create a simple animation target
fn create_animation_target(property: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), AnimationValue::Number(value));
    target
}

/// Helper function to create a complex animation target
fn create_complex_animation_target() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert(
        "transform".to_string(),
        AnimationValue::String("scale(1.0)".to_string()),
    );
    target.insert(
        "backgroundColor".to_string(),
        AnimationValue::String("red".to_string()),
    );
    target
}

// ============================================================================
// RED PHASE: Failing Tests - Define Desired Behavior
// ============================================================================

/// Test 1: Signal-based animation controller should track signal changes
/// RED: This test will fail because SignalBasedAnimationController doesn't properly track signals
#[wasm_bindgen_test]
fn test_signal_based_controller_tracks_changes() {
    let (animate_signal, set_animate_signal) = signal(create_animation_target("opacity", 0.0));

    // Create controller with initial values
    let initial_values = create_animation_target("opacity", 0.0);
    let controller = SignalBasedAnimationController::new(initial_values);

    // Initial state
    let initial_values = controller.get_current_values();
    assert_eq!(
        initial_values.get("opacity"),
        Some(&AnimationValue::Number(0.0))
    );

    // Update signal - this should trigger animation
    set_animate_signal.set(create_animation_target("opacity", 1.0));

    // RED: This will fail because the controller doesn't track the signal
    // The controller should automatically update when the signal changes
    let updated_values = controller.get_current_values();
    assert_eq!(
        updated_values.get("opacity"),
        Some(&AnimationValue::Number(1.0))
    );
}

/// Test 2: MotionDiv should react to signal changes in animate prop
/// RED: This test will fail because MotionDiv doesn't properly track signal dependencies
#[wasm_bindgen_test]
fn test_motion_div_reacts_to_signal_changes() {
    let (is_visible, set_visible) = signal(false);

    // Create reactive animation target
    let animate_target = move || {
        if is_visible.get() {
            create_animation_target("opacity", 1.0)
        } else {
            create_animation_target("opacity", 0.0)
        }
    };

    let component = view! {
        <MotionDiv
            initial=Some(create_animation_target("opacity", 0.0))
            animate=Some(animate_target())
        >
            "Test Content"
        </MotionDiv>
    };

    mount_to_body(component);

    // Initial state should be invisible
    // RED: This will fail because MotionDiv doesn't properly track the signal
    let initial_opacity = get_computed_style("opacity");
    assert_eq!(initial_opacity, "0");

    // Change signal - this should trigger animation
    set_visible.set(true);

    // RED: This will fail because the component doesn't react to signal changes
    let updated_opacity = get_computed_style("opacity");
    assert_eq!(updated_opacity, "1");
}

/// Test 3: Animation engine should integrate with signals
/// RED: This test will fail because animation engine doesn't integrate with Leptos signals
#[wasm_bindgen_test]
fn test_animation_engine_signal_integration() {
    let (animation_values, set_animation_values) = signal(HashMap::<String, f64>::new());

    // Create animation engine with signal integration
    let mut engine = leptos_motion_dom::AnimationEngine::new();

    // Set up signal-based update callback
    let set_values = set_animation_values.clone();
    engine.on_update(move |values| {
        set_values.set(values.clone());
    });

    // Start animation
    engine.animate_property(
        "opacity".to_string(),
        0.0,
        1.0,
        Transition {
            duration: Some(0.1),
            ease: Easing::Linear,
            ..Default::default()
        },
    );

    // RED: This will fail because the engine doesn't properly integrate with signals
    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(200));

    let final_values = animation_values.get();
    assert_eq!(final_values.get("opacity"), Some(&1.0));
}

/// Test 4: Complex reactive animations should work
/// RED: This test will fail because complex reactive animations aren't implemented
#[wasm_bindgen_test]
fn test_complex_reactive_animations() {
    let (step, set_step) = signal(0);
    let (is_active, set_active) = signal(false);

    // Complex animation that depends on multiple signals
    let complex_animate = move || {
        let mut target = HashMap::new();

        match step.get() {
            0 => {
                target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(0.5)".to_string()),
                );
            }
            1 => {
                target.insert("opacity".to_string(), AnimationValue::Number(0.5));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(0.8)".to_string()),
                );
            }
            2 => {
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(1.0)".to_string()),
                );
            }
            _ => {
                target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                target.insert(
                    "transform".to_string(),
                    AnimationValue::String("scale(1.0)".to_string()),
                );
            }
        }

        // Add active state modifier
        if is_active.get() {
            target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("blue".to_string()),
            );
        } else {
            target.insert(
                "backgroundColor".to_string(),
                AnimationValue::String("red".to_string()),
            );
        }

        target
    };

    let component = view! {
        <MotionDiv
            initial=Some(create_animation_target("opacity", 0.0))
            animate=Some(complex_animate())
        >
            "Complex Animation"
        </MotionDiv>
    };

    mount_to_body(component);

    // Test step 0
    let initial_opacity = get_computed_style("opacity");
    let initial_transform = get_computed_style("transform");
    assert_eq!(initial_opacity, "0");
    assert!(initial_transform.contains("scale(0.5)"));

    // Test step 1
    set_step.set(1);
    let step1_opacity = get_computed_style("opacity");
    let step1_transform = get_computed_style("transform");
    assert_eq!(step1_opacity, "0.5");
    assert!(step1_transform.contains("scale(0.8)"));

    // Test step 2
    set_step.set(2);
    let step2_opacity = get_computed_style("opacity");
    let step2_transform = get_computed_style("transform");
    assert_eq!(step2_opacity, "1");
    assert!(step2_transform.contains("scale(1.0)"));

    // Test active state
    set_active.set(true);
    let active_bg = get_computed_style("backgroundColor");
    assert_eq!(active_bg, "blue");
}

/// Test 5: Animation should trigger on signal change, not just on mount
/// RED: This test will fail because animations don't trigger on signal changes
#[wasm_bindgen_test]
fn test_animation_triggers_on_signal_change() {
    let (counter, set_counter) = signal(0);

    // Animation that changes based on counter
    let counter_animate = move || {
        let mut target = HashMap::new();
        let count = counter.get();

        target.insert(
            "transform".to_string(),
            AnimationValue::String(format!("translateX({}px)", count * 10)),
        );
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(if count % 2 == 0 { 1.0 } else { 0.5 }),
        );

        target
    };

    let component = view! {
        <MotionDiv
            initial=Some(create_animation_target("opacity", 1.0))
            animate=Some(counter_animate())
        >
            "Counter Animation"
        </MotionDiv>
    };

    mount_to_body(component);

    // Initial state
    let initial_transform = get_computed_style("transform");
    assert!(initial_transform.contains("translateX(0px)"));

    // Change counter - should trigger animation
    set_counter.set(5);
    let updated_transform = get_computed_style("transform");
    assert!(updated_transform.contains("translateX(50px)"));

    // Change counter again
    set_counter.set(10);
    let final_transform = get_computed_style("transform");
    assert!(final_transform.contains("translateX(100px)"));
}

/// Test 6: Multiple MotionDiv components should work independently
/// RED: This test will fail because signal tracking isn't properly isolated
#[wasm_bindgen_test]
fn test_multiple_motion_divs_independent() {
    let (div1_state, set_div1_state) = signal(0);
    let (div2_state, set_div2_state) = signal(0);

    let div1_animate = move || {
        let mut target = HashMap::new();
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(if div1_state.get() == 0 { 0.0 } else { 1.0 }),
        );
        target
    };

    let div2_animate = move || {
        let mut target = HashMap::new();
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(if div2_state.get() == 0 { 1.0 } else { 0.0 }),
        );
        target
    };

    let component = view! {
        <div>
            <MotionDiv
                id="div1"
                initial=Some(create_animation_target("opacity", 0.0))
                animate=Some(div1_animate())
            >
                "Div 1"
            </MotionDiv>
            <MotionDiv
                id="div2"
                initial=Some(create_animation_target("opacity", 1.0))
                animate=Some(div2_animate())
            >
                "Div 2"
            </MotionDiv>
        </div>
    };

    mount_to_body(component);

    // Initial state
    let div1_initial = get_computed_style_by_id("div1", "opacity");
    let div2_initial = get_computed_style_by_id("div2", "opacity");
    assert_eq!(div1_initial, "0");
    assert_eq!(div2_initial, "1");

    // Change div1 state - should only affect div1
    set_div1_state.set(1);
    let div1_updated = get_computed_style_by_id("div1", "opacity");
    let div2_unchanged = get_computed_style_by_id("div2", "opacity");
    assert_eq!(div1_updated, "1");
    assert_eq!(div2_unchanged, "1"); // Should remain unchanged

    // Change div2 state - should only affect div2
    set_div2_state.set(1);
    let div1_unchanged = get_computed_style_by_id("div1", "opacity");
    let div2_updated = get_computed_style_by_id("div2", "opacity");
    assert_eq!(div1_unchanged, "1"); // Should remain unchanged
    assert_eq!(div2_updated, "0");
}

// ============================================================================
// Helper Functions for Testing
// ============================================================================

/// Get computed style value for the first element with the given property
fn get_computed_style(property: &str) -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let first_div = body.query_selector("div").unwrap().unwrap();
    let computed_style = window.get_computed_style(&first_div).unwrap().unwrap();
    computed_style
        .get_property_value(property)
        .unwrap_or_default()
}

/// Get computed style value for element with specific ID
fn get_computed_style_by_id(id: &str, property: &str) -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(id).unwrap();
    let computed_style = window.get_computed_style(&element).unwrap().unwrap();
    computed_style
        .get_property_value(property)
        .unwrap_or_default()
}

/// Mount component to body for testing
fn mount_to_body(component: impl IntoView) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Clear body first
    body.set_inner_html("");

    // Mount component (simplified - in real implementation this would use Leptos mount)
    body.set_inner_html("<div>Test Component</div>");
}

// ============================================================================
// GREEN PHASE: These tests will be implemented after the RED phase
// ============================================================================

/// Test 7: Animation performance with many signal changes
/// This test will be implemented in the GREEN phase
#[wasm_bindgen_test]
#[ignore] // Ignore until GREEN phase
fn test_animation_performance_with_signal_changes() {
    // This test will verify that rapid signal changes don't cause performance issues
    // and that animations are properly batched or debounced
}

/// Test 8: Memory cleanup with signal-based animations
/// This test will be implemented in the GREEN phase
#[wasm_bindgen_test]
#[ignore] // Ignore until GREEN phase
fn test_memory_cleanup_with_signals() {
    // This test will verify that signal-based animations properly clean up
    // and don't cause memory leaks
}

/// Test 9: Error handling in signal-based animations
/// This test will be implemented in the GREEN phase
#[wasm_bindgen_test]
#[ignore] // Ignore until GREEN phase
fn test_error_handling_signal_animations() {
    // This test will verify that errors in signal-based animations are handled gracefully
}
