//! Reactive MotionDiv Component - FIXED VERSION
//!
//! This module provides a clean, minimal ReactiveMotionDiv component that
//! doesn't interfere with browser right-click functionality.

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;

/// Reactive MotionDiv component - FIXED to not block right-click
#[component]
pub fn ReactiveMotionDiv(
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
    /// Target animation state (reactive)
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    /// Function-based target animation state
    #[prop(optional)]
    animate_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Hover animation state
    #[prop(optional)]
    _while_hover: Option<AnimationTarget>,
    /// Function-based hover animation state
    #[prop(optional)]
    _while_hover_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Tap animation state
    #[prop(optional)]
    _while_tap: Option<AnimationTarget>,
    /// Function-based tap animation state
    #[prop(optional)]
    _while_tap_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // ✅ FIXED: Use a single, simple signal for styles
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());

    // Create node reference if not provided
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // ✅ FIXED: Build styles in a single, non-reactive computation
    let initial_styles = {
        let mut styles = HashMap::new();

        // Apply initial styles
        if let Some(initial_target) = initial {
            for (key, value) in initial_target {
                styles.insert(key, value.to_string_value());
            }
        }

        // Apply animate styles
        if let Some(animate_target) = animate {
            for (key, value) in animate_target.iter() {
                styles.insert(key.clone(), value.to_string_value());
            }
        }

        // Apply function-based animate styles
        if let Some(animate_function) = animate_fn {
            let animate_values = animate_function();
            for (key, value) in animate_values.iter() {
                styles.insert(key.clone(), value.to_string_value());
            }
        }

        // Apply transition configuration
        if let Some(transition_config) = transition {
            if let Some(duration) = transition_config.duration {
                styles.insert("transition-duration".to_string(), format!("{}s", duration));
            }
            if let Some(delay) = transition_config.delay {
                styles.insert("transition-delay".to_string(), format!("{}s", delay));
            }

            let easing_value = match transition_config.ease {
                leptos_motion_core::Easing::Linear => "linear".to_string(),
                leptos_motion_core::Easing::EaseIn => "ease-in".to_string(),
                leptos_motion_core::Easing::EaseOut => "ease-out".to_string(),
                leptos_motion_core::Easing::EaseInOut => "ease-in-out".to_string(),
                leptos_motion_core::Easing::CircIn => "cubic-bezier(0.55, 0, 1, 0.45)".to_string(),
                leptos_motion_core::Easing::CircOut => "cubic-bezier(0, 0.55, 0.45, 1)".to_string(),
                leptos_motion_core::Easing::CircInOut => {
                    "cubic-bezier(0.85, 0, 0.15, 1)".to_string()
                }
                leptos_motion_core::Easing::BackIn => {
                    "cubic-bezier(0.36, 0, 0.66, -0.56)".to_string()
                }
                leptos_motion_core::Easing::BackOut => {
                    "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string()
                }
                leptos_motion_core::Easing::BackInOut => {
                    "cubic-bezier(0.68, -0.6, 0.32, 1.6)".to_string()
                }
                leptos_motion_core::Easing::Bezier(x1, y1, x2, y2) => {
                    format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
                }
                leptos_motion_core::Easing::Spring(_) => "ease-in-out".to_string(),
            };
            styles.insert("transition-timing-function".to_string(), easing_value);
            styles.insert("transition-property".to_string(), "all".to_string());
        }

        styles
    };

    // Set the initial styles
    set_styles.set(initial_styles);

    // ✅ FIXED: Convert styles to CSS string - simple and non-reactive
    let style_string = {
        let styles = current_styles.get();
        let mut style_parts = styles
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>();

        // Add the style prop if provided
        if let Some(style_prop) = &style {
            style_parts.push(style_prop.clone());
        }

        style_parts.join("; ")
    };

    // ✅ FIXED: Simple view without any event handlers or complex effects
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style_string
        >
            {children()}
        </div>
    }
}
