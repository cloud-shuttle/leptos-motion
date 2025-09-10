//! Fixed Reactive MotionDiv Component
//!
//! This module provides a simplified, robust MotionDiv component that avoids
//! the unresponsiveness issues in the original implementation.

use crate::{DragConfig, DragConstraints};
use leptos::prelude::{
    Children, ClassAttribute, Effect, ElementChild, Get, Memo, NodeRef, NodeRefAttribute, Set,
    StyleAttribute,
};
use leptos::reactive::signal::signal;
use leptos::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use std::rc::Rc;

// Import the types from the original module to avoid duplication
use super::reactive_motion_div::AnimationTargetOrReactive;

/// Fixed ReactiveMotionDiv component for animated div elements
/// This version removes complex momentum animations and focuses on basic functionality
#[component]
pub fn ReactiveMotionDivFixed(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for animation engine integration
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Initial animation state
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    /// Target animation state (can be static or reactive)
    #[prop(optional)]
    animate: Option<AnimationTargetOrReactive>,
    /// Transition configuration
    #[prop(optional)]
    _transition: Option<Transition>,
    /// Hover animation state
    #[prop(optional)]
    _while_hover: Option<AnimationTarget>,
    /// Tap animation state
    #[prop(optional)]
    _while_tap: Option<AnimationTarget>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Drag configuration (simplified - no complex momentum)
    #[prop(optional)]
    _drag: Option<DragConfig>,
    /// Drag constraints
    #[prop(optional)]
    _drag_constraints: Option<DragConstraints>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create signals for animation state
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());

    // Create node reference if not provided
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // Initialize with initial styles
    if let Some(initial_target) = initial {
        let mut styles = HashMap::new();
        for (key, value) in initial_target {
            styles.insert(key, value.to_string_value());
        }
        set_styles.set(styles);
    }

    // Create a reactive style signal that properly tracks changes
    let reactive_style = move || {
        // Add CSS transitions first to ensure they're not overridden
        let mut style_parts = vec!["transition: all 0.6s ease-in-out".to_string()];

        // Handle animate prop with reactive support - COMPUTE INSIDE THE REACTIVE FUNCTION
        if let Some(animate_target) = &animate {
            let animation_styles = match animate_target {
                AnimationTargetOrReactive::Static(target) => {
                    // Static animation - convert to styles immediately
                    let mut styles = HashMap::new();
                    for (key, value) in target {
                        styles.insert(key.clone(), value.to_string_value());
                    }
                    styles
                }
                AnimationTargetOrReactive::Reactive(closure) => {
                    // Reactive animation - call the closure to get current styles
                    let target = closure(); // This will track any signals used in the closure
                    let mut styles = HashMap::new();
                    for (key, value) in target {
                        styles.insert(key.clone(), value.to_string_value());
                    }
                    styles
                }
                AnimationTargetOrReactive::Signal(animate_memo) => {
                    // Signal-based animation - get the current value
                    let target = animate_memo.get();
                    let mut styles = HashMap::new();
                    for (key, value) in target {
                        styles.insert(key.clone(), value.to_string_value());
                    }
                    styles
                }
            };

            // Add animation styles
            style_parts.extend(
                animation_styles
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value)),
            );
        }

        // Add the style prop if provided
        if let Some(style_prop) = &style {
            style_parts.push(style_prop.clone());
        }

        style_parts.join("; ")
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=reactive_style
        >
            {children()}
        </div>
    }
}
