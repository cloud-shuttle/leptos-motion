//! Engine-Integrated MotionDiv Implementation
//!
//! This module provides MotionDiv components that actually use the sophisticated
//! animation engine instead of basic style setting.

use crate::{DragConfig, DragConstraints};
use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_gestures::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Engine-integrated MotionDiv that uses the actual animation engine
#[component]
pub fn EngineIntegratedMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// NodeRef for DOM element access
    #[prop(optional)]
    node_ref: Option<NodeRef>,
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
    /// Drag animation state
    #[prop(optional)]
    while_drag: Option<AnimationTarget>,
    /// Layout animation enabled
    #[prop(optional)]
    layout: Option<bool>,
    /// Drag configuration
    #[prop(optional)]
    drag: Option<DragConfig>,
    /// Drag constraints
    #[prop(optional)]
    drag_constraints: Option<DragConstraints>,
    /// Animation start callback
    #[prop(optional)]
    on_animation_start: Option<Box<dyn Fn(AnimationHandle) + Send + Sync>>,
    /// Animation complete callback
    #[prop(optional)]
    on_animation_complete: Option<Box<dyn Fn(AnimationHandle) + Send + Sync>>,
    /// Drag callback
    #[prop(optional)]
    on_drag: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create internal node_ref if not provided
    let internal_node_ref = node_ref.unwrap_or_else(NodeRef::new);

    // Get animation engine from context or create default
    let engine = use_context::<Rc<RefCell<dyn AnimationEngine>>>()
        .unwrap_or_else(|| Rc::new(RefCell::new(DefaultAnimationEngine::new())));

    // Create gesture detector if drag is enabled
    let gesture_detector = if drag.is_some() {
        Some(Rc::new(RefCell::new(GestureDetector::new())))
    } else {
        None
    };

    // Animation state management
    let (current_animation_handle, set_current_handle) = signal(None::<AnimationHandle>);
    let (is_animating, set_animating) = signal(false);

    // Gesture state
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    let (is_dragging, set_dragging) = signal(false);

    // Handle initial animation
    if let Some(initial_target) = initial {
        let engine_clone = engine.clone();
        let node_ref_clone = internal_node_ref.clone();
        let set_handle = set_current_handle.clone();
        let set_animating = set_animating.clone();
        let on_start = on_animation_start.clone();

        create_effect(move |_| {
            if let Some(element) = node_ref_clone.get() {
                let config = AnimationConfig {
                    element: element.clone(),
                    from: HashMap::new(), // Start from current state
                    to: initial_target.clone(),
                    transition: transition.clone().unwrap_or_default(),
                    on_complete_id: None,
                    on_update_id: None,
                };

                match engine_clone.borrow_mut().animate(&config) {
                    Ok(handle) => {
                        set_handle.set(Some(handle));
                        set_animating.set(true);
                        if let Some(callback) = &on_start {
                            callback(handle);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to start initial animation: {:?}", e);
                    }
                }
            }
        });
    }

    // Handle animate prop changes
    if let Some(animate_target) = animate {
        let engine_clone = engine.clone();
        let node_ref_clone = internal_node_ref.clone();
        let set_handle = set_current_handle.clone();
        let set_animating = set_animating.clone();
        let on_start = on_animation_start.clone();
        let on_complete = on_animation_complete.clone();

        create_effect(move |_| {
            if let Some(element) = node_ref_clone.get() {
                let config = AnimationConfig {
                    element: element.clone(),
                    from: HashMap::new(), // Start from current state
                    to: animate_target.clone(),
                    transition: transition.clone().unwrap_or_default(),
                    on_complete_id: None,
                    on_update_id: None,
                };

                match engine_clone.borrow_mut().animate(&config) {
                    Ok(handle) => {
                        set_handle.set(Some(handle));
                        set_animating.set(true);
                        if let Some(callback) = &on_start {
                            callback(handle);
                        }

                        // Set up completion callback
                        if let Some(complete_callback) = &on_complete {
                            // In a real implementation, we'd set up a proper callback system
                            // For now, we'll simulate it
                            let callback_clone = complete_callback.clone();
                            let handle_clone = handle;
                            wasm_bindgen_futures::spawn_local(async move {
                                // Wait for animation duration
                                let duration = config.transition.duration.unwrap_or(0.3);
                                gloo_timers::future::TimeoutFuture::new((duration * 1000.0) as u32)
                                    .await;
                                callback_clone(handle_clone);
                            });
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to start animation: {:?}", e);
                    }
                }
            }
        });
    }

    // Handle gesture integration
    if let Some(detector) = &gesture_detector {
        let node_ref_clone = internal_node_ref.clone();
        let set_drag_pos = set_drag_position.clone();
        let set_dragging = set_dragging.clone();
        let on_drag_callback = on_drag.clone();
        let engine_clone = engine.clone();
        let while_drag_target = while_drag.clone();

        create_effect(move |_| {
            if let Some(element) = node_ref_clone.get() {
                // Set up drag event listeners
                let element_clone = element.clone();
                let set_drag_pos_clone = set_drag_pos.clone();
                let set_dragging_clone = set_dragging.clone();
                let on_drag_clone = on_drag_callback.clone();
                let engine_clone = engine_clone.clone();
                let while_drag_clone = while_drag_target.clone();

                // Mouse events for drag
                let mousedown = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    set_dragging_clone.set(true);
                    event.prevent_default();
                }) as Box<dyn FnMut(_)>);

                let mousemove = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    if set_dragging_clone.get() {
                        let x = event.client_x() as f64;
                        let y = event.client_y() as f64;
                        set_drag_pos_clone.set((x, y));

                        if let Some(callback) = &on_drag_clone {
                            callback(x, y);
                        }

                        // Trigger while_drag animation
                        if let Some(drag_target) = &while_drag_clone {
                            let config = AnimationConfig {
                                element: element_clone.clone(),
                                from: HashMap::new(),
                                to: drag_target.clone(),
                                transition: Transition {
                                    duration: Some(0.1),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                },
                                on_complete_id: None,
                                on_update_id: None,
                            };

                            let _ = engine_clone.borrow_mut().animate(&config);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let mouseup = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    set_dragging_clone.set(false);
                }) as Box<dyn FnMut(_)>);

                element
                    .add_event_listener_with_callback(
                        "mousedown",
                        mousedown.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                element
                    .add_event_listener_with_callback(
                        "mousemove",
                        mousemove.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                element
                    .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
                    .unwrap();

                // Store closures to prevent them from being dropped
                mousedown.forget();
                mousemove.forget();
                mouseup.forget();
            }
        });
    }

    // Handle hover animations
    if let Some(hover_target) = while_hover {
        let engine_clone = engine.clone();
        let node_ref_clone = internal_node_ref.clone();
        let (is_hovered, set_hovered) = signal(false);

        create_effect(move |_| {
            if let Some(element) = node_ref_clone.get() {
                let element_clone = element.clone();
                let set_hovered_clone = set_hovered.clone();
                let engine_clone = engine_clone.clone();
                let hover_target_clone = hover_target.clone();

                let mouseenter = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    set_hovered_clone.set(true);

                    let config = AnimationConfig {
                        element: element_clone.clone(),
                        from: HashMap::new(),
                        to: hover_target_clone.clone(),
                        transition: Transition {
                            duration: Some(0.2),
                            ease: Easing::EaseOut,
                            ..Default::default()
                        },
                        on_complete_id: None,
                        on_update_id: None,
                    };

                    let _ = engine_clone.borrow_mut().animate(&config);
                }) as Box<dyn FnMut(_)>);

                let mouseleave = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    set_hovered_clone.set(false);
                }) as Box<dyn FnMut(_)>);

                element
                    .add_event_listener_with_callback(
                        "mouseenter",
                        mouseenter.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                element
                    .add_event_listener_with_callback(
                        "mouseleave",
                        mouseleave.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                mouseenter.forget();
                mouseleave.forget();
            }
        });
    }

    // Cleanup on unmount
    on_cleanup(move || {
        if let Some(handle) = current_animation_handle.get() {
            let _ = engine.borrow_mut().stop(handle);
        }
    });

    view! {
        <div
            node_ref=internal_node_ref
            class=class
        >
            {children()}
        </div>
    }
}

/// Default animation engine implementation for testing
struct DefaultAnimationEngine {
    animations: HashMap<AnimationHandle, AnimationState>,
    next_handle: u64,
}

impl DefaultAnimationEngine {
    fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_handle: 1,
        }
    }
}

impl AnimationEngine for DefaultAnimationEngine {
    fn is_available(&self) -> bool {
        true
    }

    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        let handle = AnimationHandle(self.next_handle);
        self.next_handle += 1;

        // Store animation state
        self.animations.insert(handle, AnimationState::Running);

        // In a real implementation, this would start the actual animation
        // For now, we'll just store the state

        Ok(handle)
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        self.animations.remove(&handle);
        Ok(())
    }

    fn pause(&mut self, _handle: AnimationHandle) -> Result<()> {
        Ok(())
    }

    fn resume(&mut self, _handle: AnimationHandle) -> Result<()> {
        Ok(())
    }

    fn tick(&mut self, _timestamp: f64) -> Result<()> {
        Ok(())
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        match self.animations.get(&handle) {
            Some(AnimationState::Running) => Ok(PlaybackState::Running),
            Some(AnimationState::Paused) => Ok(PlaybackState::Paused),
            Some(AnimationState::Completed) => Ok(PlaybackState::Completed),
            None => Ok(PlaybackState::Completed),
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        matches!(self.animations.get(&handle), Some(AnimationState::Running))
    }

    fn get_performance_metrics(&self) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone)]
enum AnimationState {
    Running,
    Paused,
    Completed,
}
