//! Minimal MotionDiv Component
//!
//! This is a completely minimal version to test if the issue is with the complex logic

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;

/// Minimal MotionDiv component with no complex logic
#[component]
pub fn MinimalMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Target animation state (reactive)
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    /// Function-based target animation state
    #[prop(optional)]
    animate_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Simple state for animation
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());

    // Handle animate prop
    if let Some(animate_target) = animate {
        let mut styles = HashMap::new();
        for (key, value) in animate_target.iter() {
            styles.insert(key.clone(), value.to_string_value());
        }
        set_styles.set(styles);
    }

    // Handle animate_fn prop
    if let Some(animate_function) = animate_fn {
        let animate_values = animate_function();
        let mut styles = HashMap::new();
        for (key, value) in animate_values.iter() {
            styles.insert(key.clone(), value.to_string_value());
        }
        set_styles.set(styles);
    }

    // Handle transition
    if let Some(transition_config) = transition {
        let mut styles = current_styles.get();

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
            _ => "ease-in-out".to_string(),
        };

        styles.insert("transition-timing-function".to_string(), easing_value);
        styles.insert("transition-property".to_string(), "all".to_string());

        set_styles.set(styles);
    }

    // Convert styles to CSS string
    let style_string = move || {
        let styles = current_styles.get();
        let mut style_parts = styles
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>();

        if let Some(style_prop) = &style {
            style_parts.push(style_prop.clone());
        }

        style_parts.join("; ")
    };

    view! {
        <div
            class=class
            style=style_string
        >
            {children()}
        </div>
    }
}
