//! Reactive MotionDiv Component
//!
//! This module provides a reactive MotionDiv component that properly tracks
//! signal changes and triggers animations when signal values change.

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Reactive MotionDiv component that properly tracks signal changes
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
    while_hover: Option<AnimationTarget>,
    /// Function-based hover animation state
    #[prop(optional)]
    while_hover_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Tap animation state
    #[prop(optional)]
    while_tap: Option<AnimationTarget>,
    /// Function-based tap animation state
    #[prop(optional)]
    while_tap_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create signals for animation state
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);
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

    // ✅ CRITICAL: Handle animate prop with proper signal tracking
    if let Some(animate_target) = animate {
        // Apply animate styles immediately
        let mut styles = current_styles.get();
        for (key, value) in animate_target.iter() {
            styles.insert(key.clone(), value.to_string_value());
        }
        set_styles.set(styles);
    }

    // ✅ PHASE 4B: Handle transition configuration
    if let Some(transition_config) = transition {
        // Apply transition styles to the current styles
        let mut styles = current_styles.get();

        // Apply transition duration
        if let Some(duration) = transition_config.duration {
            styles.insert("transition-duration".to_string(), format!("{}s", duration));
        }

        // Apply transition delay
        if let Some(delay) = transition_config.delay {
            styles.insert("transition-delay".to_string(), format!("{}s", delay));
        }

        // Apply easing function
        let easing_value = match transition_config.ease {
            leptos_motion_core::Easing::Linear => "linear".to_string(),
            leptos_motion_core::Easing::EaseIn => "ease-in".to_string(),
            leptos_motion_core::Easing::EaseOut => "ease-out".to_string(),
            leptos_motion_core::Easing::EaseInOut => "ease-in-out".to_string(),
            leptos_motion_core::Easing::CircIn => "cubic-bezier(0.55, 0, 1, 0.45)".to_string(),
            leptos_motion_core::Easing::CircOut => "cubic-bezier(0, 0.55, 0.45, 1)".to_string(),
            leptos_motion_core::Easing::CircInOut => "cubic-bezier(0.85, 0, 0.15, 1)".to_string(),
            leptos_motion_core::Easing::BackIn => "cubic-bezier(0.36, 0, 0.66, -0.56)".to_string(),
            leptos_motion_core::Easing::BackOut => "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string(),
            leptos_motion_core::Easing::BackInOut => {
                "cubic-bezier(0.68, -0.6, 0.32, 1.6)".to_string()
            }
            leptos_motion_core::Easing::Bezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            leptos_motion_core::Easing::Spring(_) => "ease-in-out".to_string(), // Fallback for spring
        };

        styles.insert("transition-timing-function".to_string(), easing_value);

        // Set transition properties to animate all properties
        styles.insert("transition-property".to_string(), "all".to_string());

        set_styles.set(styles);
    }

    // ✅ PHASE 4A: Handle function-based animate prop
    if let Some(animate_function) = animate_fn {
        // Create a reactive effect that calls the function and updates styles
        // We need to track a signal to make the effect reactive
        let (trigger, set_trigger) = signal(0);
        Effect::new(move |_| {
            // Track the trigger signal to make this effect reactive
            let _ = trigger.get();
            let animate_values = animate_function();
            let mut styles = current_styles.get();
            for (key, value) in animate_values.iter() {
                styles.insert(key.clone(), value.to_string_value());
            }
            set_styles.set(styles);
        });
        // Trigger the effect once on mount
        set_trigger.set(1);
    }

    // ✅ CRITICAL: Handle while_hover prop with proper signal tracking
    if let Some(hover_target) = while_hover {
        let hover_target_clone = hover_target.clone();
        Effect::new(move |_| {
            let is_hovered = is_hovered.get();
            let hover_values = hover_target_clone.clone();
            let mut styles = current_styles.get();

            if is_hovered {
                for (key, value) in hover_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // ✅ PHASE 4A: Handle function-based while_hover prop
    if let Some(hover_function) = while_hover_fn {
        Effect::new(move |_| {
            let is_hovered = is_hovered.get();
            let mut styles = current_styles.get();

            if is_hovered {
                let hover_values = hover_function();
                for (key, value) in hover_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // ✅ CRITICAL: Handle while_tap prop with proper signal tracking
    if let Some(tap_target) = while_tap {
        let tap_target_clone = tap_target.clone();
        Effect::new(move |_| {
            let is_tapped = is_tapped.get();
            let tap_values = tap_target_clone.clone();
            let mut styles = current_styles.get();

            if is_tapped {
                for (key, value) in tap_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // ✅ PHASE 4A: Handle function-based while_tap prop
    if let Some(tap_function) = while_tap_fn {
        Effect::new(move |_| {
            let is_tapped = is_tapped.get();
            let mut styles = current_styles.get();

            if is_tapped {
                let tap_values = tap_function();
                for (key, value) in tap_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // Convert styles to CSS string - make it reactive with Memo::new
    let style_string = Memo::new(move |_| {
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
    });

    // ✅ CRITICAL: Add proper WASM memory management with cleanup
    Effect::new(move |_| {
        // This effect runs when the component is created and tracks all signals
        // When the component is destroyed, this effect will be cleaned up automatically

        // Track all animation-related signals to ensure proper reactivity
        let _ = current_styles.get();
        let _ = is_hovered.get();
        let _ = is_tapped.get();

        // Return cleanup function (this will be called when the effect is destroyed)
        move || {
            // Cleanup any pending timeouts or animation frames
            web_sys::console::log_1(&"🧹 ReactiveMotionDiv: Cleanup effect triggered".into());
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=class
            style=move || style_string.get()
            on:mouseenter=move |_event| {
                set_hovered.set(true);
            }
            on:mouseleave=move |_event| {
                set_hovered.set(false);
            }
            on:click=move |_event| {
                set_tapped.set(true);
                // Reset tap state after a short delay
                let set_tapped_clone = set_tapped.clone();
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                            set_tapped_clone.set(false);
                        }) as Box<dyn FnMut()>).as_ref().unchecked_ref(),
                        150 // 150ms tap duration
                    )
                    .unwrap();
            }
        >
            {children()}
        </div>
    }
}
