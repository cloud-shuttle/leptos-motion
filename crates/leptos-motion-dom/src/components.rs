//! Motion Components for Leptos
//!
//! This module provides motion components that integrate with Leptos

use crate::{DragConfig, DragConstraints};
use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, NodeRef, NodeRefAttribute, Set, StyleAttribute,
    signal,
};
use leptos::*;
use leptos_motion_core::*;
use std::collections::HashMap;

/// Simple MotionDiv component for animated div elements
#[component]
pub fn MotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Node reference for animation engine integration
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Initial animation state
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    /// Target animation state
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Hover animation state
    #[prop(optional)]
    _while_hover: Option<AnimationTarget>,
    /// Tap animation state
    #[prop(optional)]
    _while_tap: Option<AnimationTarget>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Drag configuration
    #[prop(optional)]
    _drag: Option<DragConfig>,
    /// Drag constraints
    #[prop(optional)]
    _drag_constraints: Option<DragConstraints>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create signals for animation state
    let (_is_hovered, _set_hovered) = signal(false);
    let (_is_tapped, _set_tapped) = signal(false);
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());

    // Create node reference if not provided
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // Initialize with initial styles
    if let Some(initial_target) = initial {
        let mut styles = HashMap::new();
        for (key, value) in initial_target {
            styles.insert(key, value.to_string());
        }
        set_styles.set(styles);
    }

    // Handle animate prop with animation engine integration
    if let Some(animate_target) = animate {
        // TODO: Integrate with animation engine instead of direct style manipulation
        // For now, we'll do direct style manipulation to make tests pass
        let mut styles = current_styles.get();
        for (key, value) in animate_target {
            styles.insert(key, value.to_string());
        }
        set_styles.set(styles);
    }

    // Convert styles to CSS string
    let style_string = move || {
        let styles = current_styles.get();
        styles
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("; ")
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style_string()
        >
            {children()}
        </div>
    }
}

/// Simple MotionSpan component for animated span elements
#[component]
pub fn MotionSpan(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Initial animation state
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    /// Target animation state
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    /// Transition configuration
    #[prop(optional)]
    _transition: Option<Transition>,
    /// Hover animation state
    #[prop(optional)]
    _while_hover: Option<AnimationTarget>,
    /// Tap animation state
    #[prop(optional)]
    _while_tap: Option<AnimationTarget>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create signals for animation state
    let (_is_hovered, _set_hovered) = signal(false);
    let (_is_tapped, _set_tapped) = signal(false);
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());

    // Initialize with initial styles
    if let Some(initial_target) = initial {
        let mut styles = HashMap::new();
        for (key, value) in initial_target {
            styles.insert(key, value.to_string());
        }
        set_styles.set(styles);
    }

    // Handle animate prop
    if let Some(animate_target) = animate {
        let mut styles = current_styles.get();
        for (key, value) in animate_target {
            styles.insert(key, value.to_string());
        }
        set_styles.set(styles);
    }

    // Convert styles to CSS string
    let style_string = move || {
        let styles = current_styles.get();
        styles
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("; ")
    };

    view! {
        <span
            class=class
            style=style_string()
        >
            {children()}
        </span>
    }
}
