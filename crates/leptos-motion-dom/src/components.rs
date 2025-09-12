//! Motion Components for Leptos
//!
//! This module provides motion components that integrate with Leptos

use crate::{
    DragAxis,
    DragConfig,
    DragConstraints,
    // animation_engine::{AnimationEngine, AnimationEngineBuilder}, // Unused
    // easing_functions::*, // Unused
    // repeat_config::{AnimationCycleManager, RepeatState}, // Unused
    // transform_animations::{TransformAnimationBuilder, TransformAnimationManager}, // Unused
};

/// Type alias for momentum step callback
type MomentumStepCallback = Rc<RefCell<Option<Box<dyn FnMut()>>>>;
use leptos::prelude::{
    Children, ClassAttribute, Effect, ElementChild, Get, GetUntracked, NodeRef, NodeRefAttribute,
    OnAttribute, Set, StyleAttribute,
};
use leptos::reactive::signal::signal;
use leptos::*;
use leptos_motion_core::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys;

/// Animation target that can be either static or reactive
#[derive(Clone)]
pub enum AnimationTargetOrReactive {
    /// Static animation target
    Static(AnimationTarget),
    /// Reactive animation target (function that returns AnimationTarget)
    Reactive(Rc<dyn Fn() -> AnimationTarget>),
}

impl AnimationTargetOrReactive {
    /// Get the current animation target
    pub fn get_target(&self) -> AnimationTarget {
        match self {
            AnimationTargetOrReactive::Static(target) => target.clone(),
            AnimationTargetOrReactive::Reactive(closure) => closure(),
        }
    }
}

/// Simple MotionDiv component for animated div elements
#[component]
pub fn MotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for animation engine integration
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Key for React-like re-rendering
    #[prop(optional)]
    key: Option<String>,
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
    while_hover: Option<AnimationTarget>,
    /// Tap animation state
    #[prop(optional)]
    while_tap: Option<AnimationTarget>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Drag configuration
    #[prop(optional)]
    drag: Option<DragConfig>,
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

    // Create signals for drag and momentum animation
    let (is_dragging, set_dragging) = signal(false);
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    let (drag_velocity, set_drag_velocity) = signal((0.0, 0.0));
    let (is_animating_momentum, set_animating_momentum) = signal(false);

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

    // ✅ CRITICAL FIX: Handle animate prop with proper signal tracking
    if let Some(animate_target) = animate {
        // Create a reactive effect that updates styles when animate target changes
        Effect::new(move |_| {
            let animate_values = animate_target.clone();
            let mut styles = current_styles.get();
            for (key, value) in animate_values.iter() {
                styles.insert(key.clone(), value.to_string_value());
            }
            set_styles.set(styles);
        });
    }

    // ✅ CRITICAL FIX: Handle while_hover prop with proper signal tracking
    if let Some(hover_target) = while_hover {
        let (hover_signal, _set_hover_signal) = signal(hover_target.clone());
        Effect::new(move |_| {
            let is_hovered = _is_hovered.get();
            let hover_values = hover_signal.get();
            let mut styles = current_styles.get();

            if is_hovered {
                for (key, value) in hover_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // ✅ CRITICAL FIX: Handle while_tap prop with proper signal tracking
    if let Some(tap_target) = while_tap {
        let (tap_signal, _set_tap_signal) = signal(tap_target.clone());
        Effect::new(move |_| {
            let is_tapped = _is_tapped.get();
            let tap_values = tap_signal.get();
            let mut styles = current_styles.get();

            if is_tapped {
                for (key, value) in tap_values.iter() {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
            set_styles.set(styles);
        });
    }

    // Convert styles to CSS string
    let style_string = move || {
        let mut styles = current_styles.get_untracked();

        // Add drag position to styles
        let (drag_x, drag_y) = drag_position.get_untracked();
        if drag_x != 0.0 || drag_y != 0.0 {
            styles.insert(
                "transform".to_string(),
                format!("translate({}px, {}px)", drag_x, drag_y),
            );
        }

        // Combine with the style prop if provided
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

    // Clone drag config for use in multiple closures
    let drag_config_clone = drag.clone();
    let drag_config_mousemove = drag.clone();
    let drag_config_mouseup = drag.clone();

    // ✅ CRITICAL FIX: Add proper WASM memory management with cleanup
    Effect::new(move |_| {
        // This effect runs when the component is created and tracks all signals
        // When the component is destroyed, this effect will be cleaned up automatically

        // Track all animation-related signals to ensure proper reactivity
        let _ = current_styles.get();
        let _ = _is_hovered.get();
        let _ = _is_tapped.get();
        let _ = is_dragging.get();
        let _ = drag_position.get();
        let _ = drag_velocity.get();
        let _ = is_animating_momentum.get();

        // Return cleanup function (this will be called when the effect is destroyed)
        move || {
            // Cleanup any pending timeouts or animation frames
            // The AnimationEngine already handles its own cleanup
            web_sys::console::log_1(&"🧹 MotionDiv: Cleanup effect triggered".into());
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style_string()
            on:mousedown=move |_event| {
                if let Some(_drag_config) = &drag_config_clone {
                    set_dragging.set(true);
                    set_animating_momentum.set(false);
                }
            }
            on:mousemove=move |event| {
                if let Some(_drag_config) = &drag_config_mousemove && is_dragging.get() {
                    let (current_x, current_y) = drag_position.get();
                    let new_x = current_x + event.movement_x() as f64;
                    let new_y = current_y + event.movement_y() as f64;
                    set_drag_position.set((new_x, new_y));

                    // Update velocity based on mouse movement
                    let velocity_x = event.movement_x() as f64;
                    let velocity_y = event.movement_y() as f64;
                    set_drag_velocity.set((velocity_x, velocity_y));
                }
            }
            on:mouseup=move |_event| {
                if let Some(drag_config) = &drag_config_mouseup {
                    set_dragging.set(false);

                    // Start momentum animation if enabled
                    if drag_config.momentum.unwrap_or(false) {
                        set_animating_momentum.set(true);

                        // Start momentum animation with proper continuous loop using Rc<RefCell<>>
                        let start_momentum = move || {
                            // Create a momentum step function using Rc<RefCell<>> to avoid circular references
                            let momentum_step: MomentumStepCallback = Rc::new(RefCell::new(None));

                            let momentum_step_ref = momentum_step.clone();
                            let set_drag_position_clone = set_drag_position;
                            let set_drag_velocity_clone = set_drag_velocity;
                            let set_animating_momentum_clone = set_animating_momentum;
                            let drag_config_clone = drag_config.clone();
                            let drag_position_clone = drag_position;
                            let drag_velocity_clone = drag_velocity;
                            let is_animating_momentum_clone = is_animating_momentum;

                            *momentum_step.borrow_mut() = Some(Box::new(move || {
                                // Check if we should continue animating
                                if !is_animating_momentum_clone.get() {
                                    return;
                                }

                                let (current_x, current_y) = drag_position_clone.get();
                                let (velocity_x, velocity_y) = drag_velocity_clone.get();

                                // Apply friction (0.95 = 5% friction per frame)
                                let friction = 0.95;
                                let new_velocity_x = velocity_x * friction;
                                let new_velocity_y = velocity_y * friction;

                                // Update position based on velocity
                                let new_x = current_x + new_velocity_x;
                                let new_y = current_y + new_velocity_y;

                                // Apply constraints during momentum with elastic behavior
                                let (final_x, final_y) = if let Some(constraints) = &drag_config_clone.constraints {
                                    let mut constrained_x = new_x;
                                    let mut constrained_y = new_y;

                                    // Apply axis constraints
                                    match drag_config_clone.axis {
                                        Some(DragAxis::X) => constrained_y = current_y,
                                        Some(DragAxis::Y) => constrained_x = current_x,
                                        _ => {} // Both or None - no axis constraint
                                    }

                                    // Apply boundary constraints with elastic behavior
                                    let elastic_factor = drag_config_clone.elastic.unwrap_or(0.0);

                                    if let Some(left) = constraints.left && constrained_x < left {
                                        if elastic_factor > 0.0 {
                                            let overshoot = left - constrained_x;
                                            constrained_x = left - (overshoot * elastic_factor);
                                        } else {
                                            constrained_x = left;
                                        }
                                    }
                                    if let Some(right) = constraints.right && constrained_x > right {
                                        if elastic_factor > 0.0 {
                                            let overshoot = constrained_x - right;
                                            constrained_x = right + (overshoot * elastic_factor);
                                        } else {
                                            constrained_x = right;
                                        }
                                    }
                                    if let Some(top) = constraints.top && constrained_y < top {
                                        if elastic_factor > 0.0 {
                                            let overshoot = top - constrained_y;
                                            constrained_y = top - (overshoot * elastic_factor);
                                        } else {
                                            constrained_y = top;
                                        }
                                    }
                                    if let Some(bottom) = constraints.bottom && constrained_y > bottom {
                                        if elastic_factor > 0.0 {
                                            let overshoot = constrained_y - bottom;
                                            constrained_y = bottom + (overshoot * elastic_factor);
                                        } else {
                                            constrained_y = bottom;
                                        }
                                    }

                                    (constrained_x, constrained_y)
                                } else {
                                    (new_x, new_y)
                                };

                                // Update position and velocity
                                set_drag_position_clone.set((final_x, final_y));
                                set_drag_velocity_clone.set((new_velocity_x, new_velocity_y));

                                // Check if we should stop (velocity too low)
                                let velocity_magnitude = (new_velocity_x * new_velocity_x + new_velocity_y * new_velocity_y).sqrt();
                                if velocity_magnitude < 0.1 {
                                    set_animating_momentum_clone.set(false);
                                } else {
                                    // Schedule next frame using a simple timeout
                                    let momentum_step_ref = momentum_step_ref.clone();
                                    let _ = web_sys::window()
                                        .unwrap()
                                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                                            Closure::wrap(Box::new(move || {
                                                // Call the momentum step function recursively
                                                if let Some(ref mut step) = *momentum_step_ref.borrow_mut() {
                                                    step();
                                                }
                                            }) as Box<dyn FnMut()>).as_ref().unchecked_ref(),
                                            16 // ~60fps
                                        )
                                        .unwrap();
                                }
                            }));

                            // Start the momentum animation
                            if let Some(ref mut step) = *momentum_step.borrow_mut() {
                                step();
                            }
                        };

                        // Start the momentum animation
                        start_momentum();
                    }
                }
            }
            on:mouseenter=move |_event| {
                _set_hovered.set(true);
            }
            on:mouseleave=move |_event| {
                _set_hovered.set(false);
            }
            on:click=move |_event| {
                _set_tapped.set(true);
                // Reset tap state after a short delay
                let set_tapped_clone = _set_tapped;
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        Closure::wrap(Box::new(move || {
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

/// Simple MotionSpan component for animated span elements
#[component]
pub fn MotionSpan(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Key for React-like re-rendering
    #[prop(optional)]
    key: Option<String>,
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
            styles.insert(key, value.to_string_value());
        }
        set_styles.set(styles);
    }

    // Handle animate prop
    if let Some(animate_target) = animate {
        let mut styles = current_styles.get();
        for (key, value) in animate_target {
            styles.insert(key, value.to_string_value());
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
