//! Fixed MotionDiv component using proven WASM + signals patterns
//!
//! This implements the solutions from the Leptos Motion WASM + Signals Guide

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;

/// ✅ FIXED: Signal-based animation controller using proven patterns
/// A fixed animation controller that manages animation state using Leptos signals
#[derive(Clone)]
pub struct FixedAnimationController {
    /// Current animation values
    pub current_values: ReadSignal<HashMap<String, AnimationValue>>,
    /// Target animation values
    pub target_values: ReadSignal<HashMap<String, AnimationValue>>,
    /// Whether the animation is currently playing
    pub is_playing: ReadSignal<bool>,
    /// Animation progress from 0.0 to 1.0
    pub progress: ReadSignal<f32>,
    // Store write signals for updates
    _set_current_values: WriteSignal<HashMap<String, AnimationValue>>,
    _set_target_values: WriteSignal<HashMap<String, AnimationValue>>,
    _set_is_playing: WriteSignal<bool>,
    _set_progress: WriteSignal<f32>,
}

impl FixedAnimationController {
    /// Create a new fixed animation controller with initial values
    pub fn new(initial_values: HashMap<String, AnimationValue>) -> Self {
        let (current_values, set_current_values) = signal(initial_values.clone());
        let (target_values, set_target_values) = signal(initial_values);
        let (is_playing, set_is_playing) = signal(false);
        let (progress, set_progress) = signal(0.0);

        // ✅ CRITICAL: Effect to handle animation updates with proper signal tracking
        Effect::new(move |_| {
            let _current = current_values.get();
            let target = target_values.get();
            let playing = is_playing.get();

            if playing {
                // Simple animation logic - just copy target to current for now
                set_current_values.set(target.clone());
                set_is_playing.set(false);
            }
        });

        Self {
            current_values,
            target_values,
            is_playing,
            progress,
            _set_current_values: set_current_values,
            _set_target_values: set_target_values,
            _set_is_playing: set_is_playing,
            _set_progress: set_progress,
        }
    }

    /// Animate to the specified target values
    pub fn animate_to(&self, target: HashMap<String, AnimationValue>) {
        // ✅ Update target values (triggers effect)
        self._set_target_values.set(target);
        self._set_is_playing.set(true);
    }
}

/// ✅ FIXED: MotionDiv component using proper signal tracking
#[component]
pub fn FixedMotionDiv(
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
    animate: Option<ReadSignal<AnimationTarget>>,
    /// Transition configuration
    #[prop(optional)]
    _transition: Option<Transition>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // ✅ Create node reference if not provided
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // ✅ Initialize animation controller with proper signal management
    let initial_values = initial.unwrap_or_default();
    let animation_controller = FixedAnimationController::new(initial_values);

    // ✅ CRITICAL FIX: Use Effect::new for proper signal tracking
    if let Some(animate_signal) = animate {
        let controller_clone = animation_controller.clone();
        Effect::new(move |_| {
            // This effect will re-run when animate_signal changes
            let animate_values = animate_signal.get();

            // Update animation controller
            controller_clone.animate_to(animate_values);
        });
    }

    // ✅ FIXED: Use Effect::new for DOM updates (simplified approach)
    // The style_string closure below will handle the reactive updates

    // ✅ Create style string from current values
    let style_string = move || {
        let current_values = animation_controller.current_values.get();
        let mut style_parts = Vec::new();

        // Add current animation styles
        for (property, value) in current_values {
            style_parts.push(format!("{}: {}", property, value.to_string_value()));
        }

        // Add any additional styles
        if let Some(additional_style) = &style {
            style_parts.push(additional_style.clone());
        }

        style_parts.join("; ")
    };

    view! {
        <div
            class=class.unwrap_or_default()
            style=style_string
            node_ref=node_ref
        >
            {children()}
        </div>
    }
}

// WASM bindings removed for now to focus on the core signal tracking fix
