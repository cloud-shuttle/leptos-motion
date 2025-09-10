//! Reactive MotionDiv Component
//!
//! This module provides a MotionDiv component that supports reactive animations
//! using Rc<dyn Fn() -> AnimationTarget> for the animate prop.

use crate::{DragAxis, DragConfig, DragConstraints};
use leptos::prelude::{
    Children, ClassAttribute, Effect, ElementChild, Get, GetUntracked, Memo, NodeRef,
    NodeRefAttribute, OnAttribute, Set, StyleAttribute,
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
    /// Signal-based animation target (properly tracks reactive dependencies)
    Signal(Memo<AnimationTarget>),
}

impl AnimationTargetOrReactive {
    /// Get the current animation target
    pub fn get_target(&self) -> AnimationTarget {
        match self {
            AnimationTargetOrReactive::Static(target) => target.clone(),
            AnimationTargetOrReactive::Reactive(closure) => closure(),
            AnimationTargetOrReactive::Signal(memo) => memo.get(),
        }
    }
}

impl From<AnimationTarget> for AnimationTargetOrReactive {
    fn from(target: AnimationTarget) -> Self {
        AnimationTargetOrReactive::Static(target)
    }
}

impl From<Rc<dyn Fn() -> AnimationTarget>> for AnimationTargetOrReactive {
    fn from(closure: Rc<dyn Fn() -> AnimationTarget>) -> Self {
        AnimationTargetOrReactive::Reactive(closure)
    }
}

impl From<Memo<AnimationTarget>> for AnimationTargetOrReactive {
    fn from(memo: Memo<AnimationTarget>) -> Self {
        AnimationTargetOrReactive::Signal(memo)
    }
}

/// Convenience function to create a reactive animation target
pub fn reactive_animate<F>(closure: F) -> AnimationTargetOrReactive
where
    F: Fn() -> AnimationTarget + 'static,
{
    AnimationTargetOrReactive::Reactive(Rc::new(closure))
}

/// Convenience function to create a static animation target
pub fn static_animate(target: AnimationTarget) -> AnimationTargetOrReactive {
    AnimationTargetOrReactive::Static(target)
}

/// Convenience function to create a signal-based animation target
/// This is the recommended approach for reactive animations as it properly tracks dependencies
pub fn signal_animate<F>(closure: F) -> AnimationTargetOrReactive
where
    F: Fn() -> AnimationTarget + 'static + Send + Sync,
{
    let memo = Memo::new(move |_| closure());
    AnimationTargetOrReactive::Signal(memo)
}

/// Reactive MotionDiv component for animated div elements
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

    // Handle animate prop with reactive support
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
                // NOTE: This approach has known issues with dependency tracking
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
                // Signal-based animation - properly tracks reactive dependencies
                Effect::new(move |_| {
                    let target = animate_memo.get(); // This properly tracks the memo
                    let mut styles = current_styles.get(); // Track the current styles signal
                    for (key, value) in target {
                        styles.insert(key, value.to_string_value());
                    }
                    set_styles.set(styles);
                });
            }
        }
    }

    // Convert styles to CSS string
    let style_string = move || {
        let mut styles = current_styles.get(); // Track the current_styles signal

        // Add drag position to styles
        let (drag_x, drag_y) = drag_position.get_untracked();
        if drag_x != 0.0 || drag_y != 0.0 {
            styles.insert(
                "transform".to_string(),
                format!("translate({}px, {}px)", drag_x, drag_y),
            );
        }

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

    // Clone drag config for use in multiple closures
    let drag_config_clone = drag.clone();
    let drag_config_mousemove = drag.clone();
    let drag_config_mouseup = drag.clone();

    view! {
        <div
            node_ref=node_ref
            class=class
            style=move || style_string()
            on:mousedown=move |_event| {
                if let Some(_drag_config) = &drag_config_clone {
                    set_dragging.set(true);
                    set_animating_momentum.set(false);
                }
            }
            on:mousemove=move |event| {
                if let Some(_drag_config) = &drag_config_mousemove {
                    if is_dragging.get() {
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
                            let momentum_step: Rc<RefCell<Option<Box<dyn FnMut()>>>> = Rc::new(RefCell::new(None));

                            let momentum_step_ref = momentum_step.clone();
                            let set_drag_position_clone = set_drag_position.clone();
                            let set_drag_velocity_clone = set_drag_velocity.clone();
                            let set_animating_momentum_clone = set_animating_momentum.clone();
                            let drag_config_clone = drag_config.clone();
                            let drag_position_clone = drag_position.clone();
                            let drag_velocity_clone = drag_velocity.clone();
                            let is_animating_momentum_clone = is_animating_momentum.clone();

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

                                    if let Some(left) = constraints.left {
                                        if constrained_x < left {
                                            if elastic_factor > 0.0 {
                                                let overshoot = left - constrained_x;
                                                constrained_x = left - (overshoot * elastic_factor);
                                            } else {
                                                constrained_x = left;
                                            }
                                        }
                                    }
                                    if let Some(right) = constraints.right {
                                        if constrained_x > right {
                                            if elastic_factor > 0.0 {
                                                let overshoot = constrained_x - right;
                                                constrained_x = right + (overshoot * elastic_factor);
                                            } else {
                                                constrained_x = right;
                                            }
                                        }
                                    }
                                    if let Some(top) = constraints.top {
                                        if constrained_y < top {
                                            if elastic_factor > 0.0 {
                                                let overshoot = top - constrained_y;
                                                constrained_y = top - (overshoot * elastic_factor);
                                            } else {
                                                constrained_y = top;
                                            }
                                        }
                                    }
                                    if let Some(bottom) = constraints.bottom {
                                        if constrained_y > bottom {
                                            if elastic_factor > 0.0 {
                                                let overshoot = constrained_y - bottom;
                                                constrained_y = bottom + (overshoot * elastic_factor);
                                            } else {
                                                constrained_y = bottom;
                                            }
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
                                            &Closure::wrap(Box::new(move || {
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
        >
            {children()}
        </div>
    }
}
