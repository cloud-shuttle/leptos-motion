//! Simple Animated Box Component
//!
//! A basic animated div component that demonstrates core animation functionality
//! without the complexity of the full MotionDiv system.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Simple animated box component using CSS transitions
#[component]
pub fn AnimatedBox(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for DOM access
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Initial animation state
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    /// Target animation state
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    /// Hover animation state
    #[prop(optional)]
    while_hover: Option<AnimationTarget>,
    /// Tap animation state
    #[prop(optional)]
    while_tap: Option<AnimationTarget>,
    /// Animation duration in seconds
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Animation easing
    #[prop(optional, default = Easing::EaseOut)]
    easing: Easing,
    /// Children elements
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);

    // Apply initial styles
    if let Some(initial_target) = initial {
        Effect::new(move |_| {
            if let Some(element) = node_ref.get() {
                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                    for (property, value) in initial_target.iter() {
                        let _ = html_element.style().set_property(property, &value.to_string_value());
                    }
                }
            }
        });
    }

    // Apply animate styles
    if let Some(animate_target) = animate {
        Effect::new(move |_| {
            if let Some(element) = node_ref.get() {
                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                    // Set up CSS transition
                    let easing_css = match easing {
                        Easing::Linear => "linear",
                        Easing::EaseIn => "ease-in",
                        Easing::EaseOut => "ease-out",
                        Easing::EaseInOut => "ease-in-out",
                        Easing::BackIn => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
                        Easing::BackOut => "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
                        Easing::BackInOut => "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
                        Easing::CircIn => "cubic-bezier(0.55, 0.085, 0.68, 0.53)",
                        Easing::CircOut => "cubic-bezier(0.075, 0.82, 0.165, 1)",
                        Easing::CircInOut => "cubic-bezier(0.785, 0.135, 0.15, 0.86)",
                        Easing::Spring(_) => "cubic-bezier(0.175, 0.885, 0.32, 1.275)", // Fallback to back-out
                        Easing::Bezier(a, b, c, d) => &format!("cubic-bezier({}, {}, {}, {})", a, b, c, d),
                        Easing::CubicBezier(_) => "cubic-bezier(0.25, 0.46, 0.45, 0.94)", // Fallback to ease-out
                    };
                    let _ = html_element.style().set_property("transition", &format!("all {}s {}", duration, easing_css));

                    // Apply target properties
                    for (property, value) in animate_target.iter() {
                        let _ = html_element.style().set_property(property, &value.to_string_value());
                    }
                }
            }
        });
    }

    // Handle hover animations
    if let Some(hover_target) = while_hover {
        Effect::new(move |_| {
            let is_hovered_val = is_hovered.get();
            if let Some(element) = node_ref.get() {
                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                    if is_hovered_val {
                        for (property, value) in hover_target.iter() {
                            let _ = html_element.style().set_property(property, &value.to_string_value());
                        }
                    } else {
                        // Revert hover styles
                        for (property, _value) in hover_target.iter() {
                            let _ = html_element.style().remove_property(property);
                        }
                    }
                }
            }
        });
    }

    // Handle tap animations
    if let Some(tap_target) = while_tap {
        Effect::new(move |_| {
            let is_tapped_val = is_tapped.get();
            if let Some(element) = node_ref.get() {
                if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                    if is_tapped_val {
                        for (property, value) in tap_target.iter() {
                            let _ = html_element.style().set_property(property, &value.to_string_value());
                        }
                    } else {
                        // Revert tap styles
                        for (property, _value) in tap_target.iter() {
                            let _ = html_element.style().remove_property(property);
                        }
                    }
                }
            }
        });
    }

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| {
                set_hovered.set(false);
                set_tapped.set(false);
            }
            on:mousedown=move |_| set_tapped.set(true)
            on:mouseup=move |_| set_tapped.set(false)
        >
            {if let Some(children_fn) = children {
                children_fn().into_any()
            } else {
                ().into_view().into_any()
            }}
        </div>
    }
}
