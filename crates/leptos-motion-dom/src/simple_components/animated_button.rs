//! Animated Button Component
//!
//! A simple button component with built-in hover and tap animations.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;

/// Animated button component with hover and tap effects
#[component]
pub fn AnimatedButton(
    /// Button text
    #[prop(optional)]
    text: Option<String>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
    /// Button variant/style
    #[prop(optional, default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Animation duration
    #[prop(optional, default = 0.2)]
    duration: f64,
    /// Children elements (alternative to text)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    // Define hover and tap animations based on variant
    let (hover_scale, tap_scale, bg_color, hover_bg_color) = match variant {
        ButtonVariant::Primary => (1.05, 0.95, "#007bff", "#0056b3"),
        ButtonVariant::Secondary => (1.05, 0.95, "#6c757d", "#545b62"),
        ButtonVariant::Success => (1.05, 0.95, "#28a745", "#1e7e34"),
        ButtonVariant::Danger => (1.05, 0.95, "#dc3545", "#c82333"),
    };

    let hover_animate = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(hover_scale)),
        ("background-color".to_string(), AnimationValue::String(hover_bg_color.to_string())),
    ]);

    let tap_animate = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(tap_scale)),
    ]);

    let base_style = format!(
        "padding: 12px 24px; border: none; border-radius: 8px; cursor: pointer; font-weight: bold; transition: all {}s ease; background-color: {}; color: white;",
        duration, bg_color
    );

    view! {
        <div
            style=base_style
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {if let Some(children_fn) = children {
                children_fn().into_any()
            } else if let Some(button_text) = text {
                button_text.into_view().into_any()
            } else {
                "Button".to_string().into_view().into_any()
            }}
        </div>
    }
}

/// Button style variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
}
