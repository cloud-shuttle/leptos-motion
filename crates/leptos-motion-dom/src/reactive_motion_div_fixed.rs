//! Fixed Reactive MotionDiv Component
//!
//! This module provides a simplified, robust MotionDiv component that avoids
//! the unresponsiveness issues in the original implementation.

use crate::{DragConfig, DragConstraints};
use leptos::prelude::{
    Children, ClassAttribute, Effect, ElementChild, Get, GetUntracked, Memo, NodeRef,
    NodeRefAttribute, Set, StyleAttribute, signal,
};
use leptos::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use std::rc::Rc;

// Import the types from the original module to avoid duplication
use super::reactive_motion_div::{AnimationTargetOrReactive, signal_animate};

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

    // Handle animate prop with reactive support - SIMPLIFIED VERSION
    if let Some(animate_target) = animate {
        match animate_target {
            AnimationTargetOrReactive::Static(target) => {
                // Static animation - apply once
                let mut styles = current_styles.get();
                for (key, value) in target {
                    styles.insert(key, value.to_string_value());
                }
                set_styles.set(styles);
            }
            AnimationTargetOrReactive::Reactive(closure) => {
                // Reactive animation - create effect to watch for changes
                // SIMPLIFIED: Use get_untracked to avoid dependency tracking issues
                Effect::new(move |_| {
                    let target = closure();
                    let mut styles = current_styles.get_untracked();
                    for (key, value) in target {
                        styles.insert(key, value.to_string_value());
                    }
                    set_styles.set(styles);
                });
            }
            AnimationTargetOrReactive::Signal(animate_memo) => {
                // Signal-based animation - use untracked to avoid circular dependencies
                Effect::new(move |_| {
                    let target = animate_memo.get(); // This properly tracks the memo
                    let mut styles = current_styles.get_untracked(); // Use get_untracked to avoid circular dependencies
                    for (key, value) in target {
                        styles.insert(key, value.to_string_value());
                    }
                    set_styles.set(styles);
                });
            }
        }
    }

    // Create a reactive style signal that properly tracks changes
    let reactive_style = move || {
        let styles = current_styles.get(); // Track the styles signal

        // Add CSS transitions first to ensure they're not overridden
        let mut style_parts = vec!["transition: all 0.5s ease-in-out".to_string()];

        // Add animation styles
        style_parts.extend(
            styles
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value)),
        );

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
