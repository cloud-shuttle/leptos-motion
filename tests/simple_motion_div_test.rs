//! Simple test to verify MotionDiv API works
//!
//! This test focuses on the core issues identified in our TDD analysis.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;
use std::collections::HashMap;

/// Test 1: Basic MotionDiv Usage - This should work
#[test]
fn test_basic_motion_div_compiles() {
    let _ = create_scope(|cx| {
        let initial = {
            let mut target = HashMap::new();
            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
            target.insert("scale".to_string(), AnimationValue::Number(0.5));
            target
        };

        let animate = {
            let mut target = HashMap::new();
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.0));
            target
        };

        // This should work - basic usage with Option<T> parameters
        let _component = view! { cx,
            <MotionDiv
                initial=Some(initial)
                animate=Some(animate)
            >
                "Test Content"
            </MotionDiv>
        };
    });
}

/// Test 2: MotionDiv with all optional parameters - This should work
#[test]
fn test_motion_div_all_optional_params() {
    let _ = create_scope(|cx| {
        let _component = view! { cx,
            <MotionDiv
                class=Some("test-class".to_string())
                style=Some("color: red".to_string())
                node_ref=Some(NodeRef::new())
                initial=None
                animate=None
                transition=None
                _while_hover=None
                _while_tap=None
                _layout=None
                drag=None
                drag_constraints=None
            >
                "Test Content"
            </MotionDiv>
        };
    });
}

/// Test 3: MotionDiv with transition - This should work
#[test]
fn test_motion_div_with_transition() {
    let _ = create_scope(|cx| {
        let transition = Transition {
            duration: Some(0.5),
            delay: Some(0.1),
            ease: Easing::EaseInOut,
            repeat: RepeatConfig::Never,
            stagger: None,
        };

        let _component = view! { cx,
            <MotionDiv
                transition=Some(transition)
            >
                "Test Content"
            </MotionDiv>
        };
    });
}

/// Test 4: MotionDiv with drag config - This should work
#[test]
fn test_motion_div_with_drag() {
    let _ = create_scope(|cx| {
        let drag_config = DragConfig {
            axis: Some(DragAxis::X),
            momentum: Some(true),
            elastic: Some(0.2),
            constraints: None,
        };

        let _component = view! { cx,
            <MotionDiv
                drag=Some(drag_config)
            >
                "Test Content"
            </MotionDiv>
        };
    });
}
