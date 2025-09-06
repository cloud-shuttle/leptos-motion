use leptos_motion_core::{AnimationTarget, Transition, Variants};
use leptos_motion_dom::components::{DragAxis, DragConfig, DragConstraints, MotionProps};
use std::collections::HashMap;

#[test]
fn test_motion_props_creation() {
    let props = MotionProps::default();

    assert!(props.initial.is_none());
    assert!(props.animate.is_none());
    assert!(props.exit.is_none());
    assert!(props.transition.is_none());
    assert!(props.variants.is_none());
    assert!(props.layout.is_none());
    assert!(props.drag.is_none());
    assert!(props.while_hover.is_none());
    assert!(props.while_tap.is_none());
    assert!(props.while_focus.is_none());
    assert!(props.while_in_view.is_none());
}

#[test]
fn test_motion_props_with_values() {
    let mut props = MotionProps::default();

    let mut initial = AnimationTarget::new();
    initial.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );

    let mut animate = AnimationTarget::new();
    animate.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(1.0),
    );

    props.initial = Some(initial.clone());
    props.animate = Some(animate.clone());

    assert!(props.initial.is_some());
    assert!(props.animate.is_some());
    assert_eq!(props.initial.as_ref().unwrap().len(), 1);
    assert_eq!(props.animate.as_ref().unwrap().len(), 1);
}

#[test]
fn test_motion_props_clone() {
    let mut props = MotionProps::default();
    let mut initial = AnimationTarget::new();
    initial.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );
    props.initial = Some(initial);

    let cloned = props.clone();

    assert!(cloned.initial.is_some());
    assert_eq!(cloned.initial.as_ref().unwrap().len(), 1);
}

#[test]
fn test_motion_props_debug() {
    let props = MotionProps::default();
    let debug_str = format!("{:?}", props);

    assert!(debug_str.contains("MotionProps"));
}

#[test]
fn test_drag_config_creation() {
    let config = DragConfig::new();

    assert_eq!(config.axis, Some(DragAxis::Both));
    assert!(config.constraints.is_none());
    assert_eq!(config.elastic, 0.5);
    assert!(config.momentum);
}

#[test]
fn test_drag_config_with_axis() {
    let config = DragConfig::new().axis(DragAxis::X);

    assert_eq!(config.axis, Some(DragAxis::X));
    assert_eq!(config.elastic, 0.5);
    assert!(config.momentum);
}

#[test]
fn test_drag_config_clone() {
    let config = DragConfig::new().axis(DragAxis::Y);
    let cloned = config.clone();

    assert_eq!(cloned.axis, config.axis);
    assert_eq!(cloned.elastic, config.elastic);
    assert_eq!(cloned.momentum, config.momentum);
}

#[test]
fn test_drag_config_debug() {
    let config = DragConfig::new();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("DragConfig"));
}

#[test]
fn test_drag_axis_variants() {
    let axes = vec![DragAxis::X, DragAxis::Y, DragAxis::Both];

    for axis in axes {
        let debug_str = format!("{:?}", axis);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_drag_axis_clone() {
    let axis = DragAxis::X;
    let cloned = axis.clone();

    assert_eq!(axis, cloned);
}

#[test]
fn test_drag_axis_equality() {
    let axis1 = DragAxis::X;
    let axis2 = DragAxis::X;
    let axis3 = DragAxis::Y;

    assert_eq!(axis1, axis2);
    assert_ne!(axis1, axis3);
}

#[test]
fn test_drag_constraints_creation() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    assert_eq!(constraints.left, Some(-100.0));
    assert_eq!(constraints.right, Some(100.0));
    assert_eq!(constraints.top, Some(-50.0));
    assert_eq!(constraints.bottom, Some(50.0));
}

#[test]
fn test_drag_constraints_clone() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    let cloned = constraints.clone();

    assert_eq!(cloned.left, constraints.left);
    assert_eq!(cloned.right, constraints.right);
    assert_eq!(cloned.top, constraints.top);
    assert_eq!(cloned.bottom, constraints.bottom);
}

#[test]
fn test_drag_constraints_debug() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    let debug_str = format!("{:?}", constraints);

    assert!(debug_str.contains("DragConstraints"));
    assert!(debug_str.contains("-100.0"));
    assert!(debug_str.contains("100.0"));
}

#[test]
fn test_drag_config_with_constraints() {
    let constraints = DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    };

    let mut config = DragConfig::new();
    config.constraints = Some(constraints.clone());

    assert!(config.constraints.is_some());
    assert_eq!(config.constraints.as_ref().unwrap().left, Some(-100.0));
    assert_eq!(config.constraints.as_ref().unwrap().right, Some(100.0));
}

#[test]
fn test_motion_props_with_drag() {
    let mut props = MotionProps::default();
    let drag_config = DragConfig::new().axis(DragAxis::X);

    props.drag = Some(drag_config.clone());

    assert!(props.drag.is_some());
    assert_eq!(props.drag.as_ref().unwrap().axis, Some(DragAxis::X));
}

#[test]
fn test_motion_props_with_layout() {
    let mut props = MotionProps::default();
    props.layout = Some(true);

    assert_eq!(props.layout, Some(true));
}

#[test]
fn test_motion_props_with_transition() {
    let mut props = MotionProps::default();
    let mut transition = Transition::new();
    transition.duration = Some(0.5);

    props.transition = Some(transition.clone());

    assert!(props.transition.is_some());
    assert_eq!(props.transition.as_ref().unwrap().duration, Some(0.5));
}

#[test]
fn test_motion_props_with_variants() {
    let mut props = MotionProps::default();
    let variants = Variants::new();

    props.variants = Some(variants.clone());

    assert!(props.variants.is_some());
}

#[test]
fn test_motion_props_with_gesture_handlers() {
    let mut props = MotionProps::default();

    let mut hover_target = AnimationTarget::new();
    hover_target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(1.1),
    );

    let mut tap_target = AnimationTarget::new();
    tap_target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(0.9),
    );

    props.while_hover = Some(hover_target.clone());
    props.while_tap = Some(tap_target.clone());
    props.while_focus = Some(hover_target.clone());
    props.while_in_view = Some(hover_target.clone());

    assert!(props.while_hover.is_some());
    assert!(props.while_tap.is_some());
    assert!(props.while_focus.is_some());
    assert!(props.while_in_view.is_some());
}

#[test]
fn test_motion_props_with_exit() {
    let mut props = MotionProps::default();
    let mut exit_target = AnimationTarget::new();
    exit_target.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );

    props.exit = Some(exit_target.clone());

    assert!(props.exit.is_some());
    assert_eq!(props.exit.as_ref().unwrap().len(), 1);
}

#[test]
fn test_drag_config_edge_cases() {
    let config = DragConfig {
        axis: None,
        constraints: None,
        elastic: 0.0,
        momentum: false,
    };

    assert!(config.axis.is_none());
    assert!(config.constraints.is_none());
    assert_eq!(config.elastic, 0.0);
    assert!(!config.momentum);
}

#[test]
fn test_drag_constraints_edge_cases() {
    let constraints = DragConstraints {
        left: None,
        right: None,
        top: None,
        bottom: None,
    };

    assert!(constraints.left.is_none());
    assert!(constraints.right.is_none());
    assert!(constraints.top.is_none());
    assert!(constraints.bottom.is_none());
}

#[test]
fn test_motion_props_comprehensive() {
    let mut props = MotionProps::default();

    // Set all properties
    let mut initial = AnimationTarget::new();
    initial.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );

    let mut animate = AnimationTarget::new();
    animate.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(1.0),
    );

    let mut exit = AnimationTarget::new();
    exit.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );

    let mut transition = Transition::new();
    transition.duration = Some(0.3);

    let variants = Variants::new();
    let drag_config = DragConfig::new().axis(DragAxis::X);

    let mut hover_target = AnimationTarget::new();
    hover_target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(1.1),
    );

    props.initial = Some(initial);
    props.animate = Some(animate);
    props.exit = Some(exit);
    props.transition = Some(transition);
    props.variants = Some(variants);
    props.layout = Some(true);
    props.drag = Some(drag_config);
    props.while_hover = Some(hover_target.clone());
    props.while_tap = Some(hover_target.clone());
    props.while_focus = Some(hover_target.clone());
    props.while_in_view = Some(hover_target);

    // Verify all properties are set
    assert!(props.initial.is_some());
    assert!(props.animate.is_some());
    assert!(props.exit.is_some());
    assert!(props.transition.is_some());
    assert!(props.variants.is_some());
    assert_eq!(props.layout, Some(true));
    assert!(props.drag.is_some());
    assert!(props.while_hover.is_some());
    assert!(props.while_tap.is_some());
    assert!(props.while_focus.is_some());
    assert!(props.while_in_view.is_some());
}
