//! Enhanced MotionDiv Component with Animation Engine Integration
//!
//! This module provides an enhanced MotionDiv component that integrates with the
//! animation engine for smooth, performant animations.

use crate::{
    DragConfig, DragConstraints,
    animation_engine::{AnimationEngine},
    // easing_functions::*, // Unused
    repeat_config::{StaggerConfig},
    transform_animations::{TransformAnimationManager},
    // animation_engine::{AnimationEngineBuilder}, // Unused
    // repeat_config::{AnimationCycleManager, RepeatState}, // Unused
    // transform_animations::{TransformAnimationBuilder}, // Unused
};
use leptos::prelude::*;
use leptos_motion_core::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Enhanced MotionDiv component with full animation engine integration
#[component]
pub fn EnhancedMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for animation engine integration
    #[prop(optional)]
    _node_ref: Option<NodeRef<leptos::html::Div>>,
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

    // Create animation engine
    let animation_engine = Rc::new(RefCell::new(AnimationEngine::new()));
    let transform_manager = Rc::new(RefCell::new(TransformAnimationManager::new()));

    // Set up animation engine callbacks
    {
        let set_styles = set_styles.clone();
        let transform_manager = transform_manager.clone();

        animation_engine.borrow_mut().on_update(move |values| {
            let mut styles = current_styles.get();

            // Update transform styles
            let transform_css = transform_manager.borrow().get_css_transform();
            if !transform_css.is_empty() {
                styles.insert("transform".to_string(), transform_css);
            }

            // Update other animation values
            for (property, value) in values {
                match property.as_str() {
                    "opacity" => {
                        styles.insert("opacity".to_string(), value.to_string());
                    }
                    "scale" => {
                        styles.insert("transform".to_string(), format!("scale({})", value));
                    }
                    "rotate" => {
                        styles.insert("transform".to_string(), format!("rotate({}deg)", value));
                    }
                    "translateX" => {
                        styles.insert("transform".to_string(), format!("translateX({}px)", value));
                    }
                    "translateY" => {
                        styles.insert("transform".to_string(), format!("translateY({}px)", value));
                    }
                    _ => {
                        // Handle custom properties
                        styles.insert(property.clone(), value.to_string());
                    }
                }
            }

            set_styles.set(styles);
        });
    }

    // Handle initial styles
    if let Some(initial_target) = initial {
        let mut styles = current_styles.get();
        for (key, value) in initial_target.iter() {
            styles.insert(key.clone(), value.to_string_value());
        }
        set_styles.set(styles);
    }

    // Handle animate prop with animation engine
    if let Some(animate_target) = animate {
        let transition = transition.clone().unwrap_or_default();

        // Convert AnimationTarget to engine format
        let mut properties = HashMap::new();
        for (key, value) in animate_target.iter() {
            if let Some(num_value) = value.as_number() {
                properties.insert(key.clone(), (0.0, num_value, transition.clone()));
            }
        }

        animation_engine.borrow_mut().animate_properties(properties);
    }

    // Handle while_hover prop with animation engine
    if let Some(hover_target) = while_hover {
        let hover_target_clone = hover_target.clone();
        let animation_engine = animation_engine.clone();
        let transition = transition.clone().unwrap_or_default();

        Effect::new(move |_| {
            if _is_hovered.get() {
                let mut properties = HashMap::new();
                for (key, value) in hover_target_clone.iter() {
                    if let Some(num_value) = value.as_number() {
                        properties.insert(key.clone(), (0.0, num_value, transition.clone()));
                    }
                }
                animation_engine.borrow_mut().animate_properties(properties);
            }
        });
    }

    // Handle while_tap prop with animation engine
    if let Some(tap_target) = while_tap {
        let tap_target_clone = tap_target.clone();
        let animation_engine = animation_engine.clone();
        let transition = transition.clone().unwrap_or_default();

        Effect::new(move |_| {
            if _is_tapped.get() {
                let mut properties = HashMap::new();
                for (key, value) in tap_target_clone.iter() {
                    if let Some(num_value) = value.as_number() {
                        properties.insert(key.clone(), (0.0, num_value, transition.clone()));
                    }
                }
                animation_engine.borrow_mut().animate_properties(properties);
            }
        });
    }

    // Create style string
    let style_string = move || {
        let mut style_parts = Vec::new();

        // Add custom styles
        if let Some(custom_style) = style {
            style_parts.push(custom_style);
        }

        // Add animation styles
        let current_styles = current_styles.get();
        for (key, value) in current_styles.iter() {
            style_parts.push(format!("{}: {}", key, value));
        }

        style_parts.join("; ")
    };

    // Handle drag functionality (simplified for now)
    let drag_handlers = if drag.is_some() {
        Some((
            move |_event| {
                // Start drag
            },
            move |_event| {
                // Handle drag
            },
            move |_event| {
                // End drag
            },
        ))
    } else {
        None
    };

    view! {
        <div
            class=class
            style=style_string()
            on:mousedown=move |_event| {
                if let Some((start_handler, _, _)) = &drag_handlers {
                    start_handler(_event);
                }
            }
            on:mousemove=move |_event| {
                if let Some((_, move_handler, _)) = &drag_handlers {
                    move_handler(_event);
                }
            }
            on:mouseup=move |_event| {
                if let Some((_, _, end_handler)) = &drag_handlers {
                    end_handler(_event);
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
                let set_tapped_clone = _set_tapped.clone();
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &Closure::wrap(Box::new(move || {
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

/// Animation sequence builder for complex animations
pub struct AnimationSequenceBuilder {
    animations: Vec<AnimationStep>,
    current_delay: f64,
}

/// Single animation step in a sequence
#[derive(Debug, Clone)]
pub struct AnimationStep {
    /// The target values for this animation step
    pub target: AnimationTarget,
    /// The transition configuration for this step
    pub transition: Transition,
    /// Delay before this animation step begins (in seconds)
    pub delay: f64,
}

impl AnimationSequenceBuilder {
    /// Create a new animation sequence builder
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            current_delay: 0.0,
        }
    }

    /// Add an animation step
    pub fn add_step(mut self, target: AnimationTarget, transition: Transition) -> Self {
        self.animations.push(AnimationStep {
            target,
            transition,
            delay: self.current_delay,
        });
        self
    }

    /// Add a delay before the next step
    pub fn delay(mut self, delay: f64) -> Self {
        self.current_delay += delay;
        self
    }

    /// Build the animation sequence
    pub fn build(self) -> AnimationSequence {
        AnimationSequence {
            steps: self.animations,
            current_step: 0,
            is_complete: false,
        }
    }
}

/// Animation sequence for complex multi-step animations
#[derive(Debug, Clone)]
pub struct AnimationSequence {
    steps: Vec<AnimationStep>,
    current_step: usize,
    is_complete: bool,
}

impl AnimationSequence {
    /// Create a new animation sequence
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            is_complete: false,
        }
    }

    /// Get the current step
    pub fn current_step(&self) -> Option<&AnimationStep> {
        self.steps.get(self.current_step)
    }

    /// Advance to the next step
    pub fn advance(&mut self) {
        self.current_step += 1;
        if self.current_step >= self.steps.len() {
            self.is_complete = true;
        }
    }

    /// Check if the sequence is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.is_complete = false;
    }
}

/// Stagger animation manager for coordinating multiple elements
pub struct StaggerAnimationManager {
    elements: Vec<Rc<RefCell<AnimationEngine>>>,
    stagger_config: StaggerConfig,
    is_running: bool,
}

impl StaggerAnimationManager {
    /// Create a new stagger animation manager
    pub fn new(stagger_config: StaggerConfig) -> Self {
        Self {
            elements: Vec::new(),
            stagger_config,
            is_running: false,
        }
    }

    /// Add an element to the stagger
    pub fn add_element(&mut self, element: Rc<RefCell<AnimationEngine>>) {
        self.elements.push(element);
    }

    /// Start the stagger animation
    pub fn start(&mut self, target: AnimationTarget, transition: Transition) {
        self.is_running = true;

        for (index, element) in self.elements.iter().enumerate() {
            let delay = self.stagger_config.get_delay(index);

            // Schedule animation with delay
            let element_clone = element.clone();
            let target_clone = target.clone();
            let transition_clone = transition.clone();

            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Closure::wrap(Box::new(move || {
                        let mut properties = HashMap::new();
                        for (key, value) in target_clone.iter() {
                            if let Some(num_value) = value.as_number() {
                                properties.insert(
                                    key.clone(),
                                    (0.0, num_value, transition_clone.clone()),
                                );
                            }
                        }
                        element_clone.borrow_mut().animate_properties(properties);
                    }) as Box<dyn FnMut()>)
                    .as_ref()
                    .unchecked_ref(),
                    (delay * 1000.0) as i32, // Convert to milliseconds
                )
                .unwrap();
        }
    }

    /// Stop all stagger animations
    pub fn stop(&mut self) {
        self.is_running = false;
        for element in &self.elements {
            element.borrow_mut().stop_all();
        }
    }
}

impl Default for AnimationSequenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AnimationSequence {
    fn default() -> Self {
        Self::new()
    }
}
