//! TDD Tests for DOM Integration
//!
//! This module contains comprehensive tests for integrating TDD implementations
//! (Scroll, FLIP, Gesture) into the DOM crate with proper component wrappers.

use leptos::prelude::*;
use leptos_motion_core::*;
// Note: We'll create mock implementations to avoid circular dependencies
// In a real implementation, these would be imported from their respective crates
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_error, console_log};
use web_sys::{Element, IntersectionObserver, MouseEvent};

// Mock implementations to avoid circular dependencies
// In a real implementation, these would be imported from their respective crates

/// Mock scroll animation configuration
#[derive(Clone, Debug)]
pub struct ScrollAnimationConfig {
    pub threshold: f64,
    pub root_margin: String,
    pub trigger_point: f64,
    pub duration: f64,
    pub easing: Easing,
}

impl Default for ScrollAnimationConfig {
    fn default() -> Self {
        Self {
            threshold: 0.1,
            root_margin: "0px".to_string(),
            trigger_point: 0.5,
            duration: 0.3,
            easing: Easing::EaseOut,
        }
    }
}

/// Mock scroll animation manager
#[derive(Clone)]
pub struct ScrollAnimationManager {
    config: ScrollAnimationConfig,
    state: (
        ReadSignal<ScrollAnimationState>,
        WriteSignal<ScrollAnimationState>,
    ),
    progress: (ReadSignal<f64>, WriteSignal<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScrollAnimationState {
    Waiting,
    Animating,
    Completed,
    Reversed,
}

impl ScrollAnimationManager {
    pub fn new(config: ScrollAnimationConfig) -> Self {
        let state = signal(ScrollAnimationState::Waiting);
        let progress = signal(0.0);

        Self {
            config,
            state,
            progress,
        }
    }

    pub fn observe_element(&mut self, _element: &Element) -> std::result::Result<(), JsValue> {
        // Mock implementation
        Ok(())
    }

    pub fn create_animation_target(&self) -> HashMap<String, AnimationValue> {
        let state = self.state.0.get();
        let progress = self.progress.0.get();

        match state {
            ScrollAnimationState::Waiting => HashMap::new(),
            ScrollAnimationState::Animating => HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(progress)),
                (
                    "scale".to_string(),
                    AnimationValue::Number(0.8 + (progress * 0.2)),
                ),
            ]),
            ScrollAnimationState::Completed => HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(1.0)),
                ("scale".to_string(), AnimationValue::Number(1.0)),
            ]),
            ScrollAnimationState::Reversed => HashMap::new(),
        }
    }
}

/// Mock FLIP configuration
#[derive(Clone, Debug)]
pub struct FLIPConfig {
    pub duration: f64,
    pub ease: Easing,
    pub animate_scale: bool,
    pub animate_rotation: bool,
    pub animate_opacity: bool,
    pub z_index: i32,
}

impl Default for FLIPConfig {
    fn default() -> Self {
        Self {
            duration: 0.3,
            ease: Easing::EaseOut,
            animate_scale: true,
            animate_rotation: false,
            animate_opacity: false,
            z_index: 1000,
        }
    }
}

/// Mock FLIP state
#[derive(Clone, Debug, PartialEq)]
pub enum FLIPState {
    First,
    Last,
    Inverted,
    Playing,
    Completed,
}

/// Mock FLIP manager
#[derive(Clone)]
pub struct FLIPManager {
    config: FLIPConfig,
    state: FLIPState,
}

impl FLIPManager {
    pub fn new(config: FLIPConfig) -> Self {
        Self {
            config,
            state: FLIPState::First,
        }
    }

    pub fn record_first(&mut self, _element: &Element) -> std::result::Result<(), JsValue> {
        self.state = FLIPState::First;
        Ok(())
    }

    pub fn get_state(&self) -> &FLIPState {
        &self.state
    }
}

/// Mock gesture animation configuration
#[derive(Clone, Debug)]
pub struct GestureAnimationConfig {
    pub drag_enabled: bool,
    pub hover_enabled: bool,
    pub tap_enabled: bool,
    pub pinch_enabled: bool,
    pub transition_duration: f64,
    pub easing: Easing,
    pub spring_config: SpringConfig,
    pub drag_constraints: DragConstraints,
    pub hover_properties: HashMap<String, AnimationValue>,
    pub tap_properties: HashMap<String, AnimationValue>,
}

#[derive(Clone, Debug)]
pub struct DragConstraints {
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
    pub constrain_to_parent: bool,
}

impl Default for DragConstraints {
    fn default() -> Self {
        Self {
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            constrain_to_parent: false,
        }
    }
}

impl Default for GestureAnimationConfig {
    fn default() -> Self {
        let mut hover_props = HashMap::new();
        hover_props.insert("scale".to_string(), AnimationValue::Number(1.05));
        hover_props.insert("opacity".to_string(), AnimationValue::Number(0.9));

        let mut tap_props = HashMap::new();
        tap_props.insert("scale".to_string(), AnimationValue::Number(0.95));
        tap_props.insert("opacity".to_string(), AnimationValue::Number(0.8));

        Self {
            drag_enabled: true,
            hover_enabled: true,
            tap_enabled: true,
            pinch_enabled: false,
            transition_duration: 0.2,
            easing: Easing::EaseOut,
            spring_config: SpringConfig::default(),
            drag_constraints: DragConstraints::default(),
            hover_properties: hover_props,
            tap_properties: tap_props,
        }
    }
}

/// Mock gesture state
#[derive(Clone, Debug, PartialEq)]
pub enum GestureState {
    Idle,
    Hovering,
    Dragging,
    Tapping,
    Pinching,
    Returning,
}

/// Mock gesture manager
#[derive(Clone)]
pub struct GestureAnimationManager {
    config: GestureAnimationConfig,
    state: GestureState,
    animation_target: HashMap<String, AnimationValue>,
}

impl GestureAnimationManager {
    pub fn new(config: GestureAnimationConfig) -> Self {
        Self {
            config,
            state: GestureState::Idle,
            animation_target: HashMap::new(),
        }
    }

    pub fn set_element(&mut self, _element: &Element) {
        // Mock implementation
    }

    pub fn handle_mouse_enter(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.config.hover_enabled {
            self.state = GestureState::Hovering;
            self.animation_target = self.config.hover_properties.clone();
        }
        Ok(())
    }

    pub fn handle_mouse_leave(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.config.hover_enabled {
            self.state = GestureState::Returning;
            self.animation_target = HashMap::new();
        }
        Ok(())
    }

    pub fn handle_mouse_down(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.config.drag_enabled {
            self.state = GestureState::Dragging;
        }
        Ok(())
    }

    pub fn handle_mouse_move(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        // Mock implementation
        Ok(())
    }

    pub fn handle_mouse_up(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.config.drag_enabled {
            self.state = GestureState::Returning;
        }
        Ok(())
    }

    pub fn handle_click(&mut self, _event: &MouseEvent) -> std::result::Result<(), JsValue> {
        if self.config.tap_enabled {
            self.state = GestureState::Tapping;
            self.animation_target = self.config.tap_properties.clone();
        }
        Ok(())
    }

    pub fn get_animation_target(&self) -> &HashMap<String, AnimationValue> {
        &self.animation_target
    }
}

/// Integrated MotionDiv component with all TDD features
#[component]
pub fn IntegratedMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Animation properties
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Scroll animation configuration
    #[prop(optional)]
    scroll_config: Option<ScrollAnimationConfig>,
    /// FLIP animation configuration
    #[prop(optional)]
    flip_config: Option<FLIPConfig>,
    /// Gesture animation configuration
    #[prop(optional)]
    gesture_config: Option<GestureAnimationConfig>,
    /// Whether scroll animations are enabled
    #[prop(optional)]
    scroll_enabled: Option<bool>,
    /// Whether FLIP animations are enabled
    #[prop(optional)]
    flip_enabled: Option<bool>,
    /// Whether gesture animations are enabled
    #[prop(optional)]
    gesture_enabled: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let scroll_enabled = scroll_enabled.unwrap_or(false);
    let flip_enabled = flip_enabled.unwrap_or(false);
    let gesture_enabled = gesture_enabled.unwrap_or(false);

    // Create signals for different animation systems
    let (scroll_manager, set_scroll_manager) = signal(None::<ScrollAnimationManager>);
    let (flip_manager, set_flip_manager) = signal(None::<FLIPManager>);
    let (gesture_manager, set_gesture_manager) = signal(None::<GestureAnimationManager>);

    // Combined animation target
    let (combined_animation_target, set_combined_animation_target) = signal(HashMap::new());

    // Initialize managers when node ref is available
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let mut managers_initialized = false;

            // Initialize scroll manager
            if scroll_enabled {
                if let Some(config) = scroll_config.clone() {
                    let mut manager = ScrollAnimationManager::new(config);
                    if let Err(e) = manager.observe_element(&element) {
                        console_error!("Failed to observe element for scroll: {:?}", e);
                    } else {
                        set_scroll_manager.set(Some(manager));
                        managers_initialized = true;
                    }
                }
            }

            // Initialize FLIP manager
            if flip_enabled {
                if let Some(config) = flip_config.clone() {
                    let mut manager = FLIPManager::new(config);
                    if let Err(e) = manager.record_first(&element) {
                        console_error!("Failed to record first layout for FLIP: {:?}", e);
                    } else {
                        set_flip_manager.set(Some(manager));
                        managers_initialized = true;
                    }
                }
            }

            // Initialize gesture manager
            if gesture_enabled {
                if let Some(config) = gesture_config.clone() {
                    let mut manager = GestureAnimationManager::new(config);
                    manager.set_element(&element);
                    set_gesture_manager.set(Some(manager));
                    managers_initialized = true;
                }
            }

            if managers_initialized {
                console_log!("All animation managers initialized successfully");
            }
        }
    });

    // Combine animation targets from all systems
    Effect::new(move |_| {
        let mut combined = HashMap::new();

        // Add base animation properties
        if let Some(base_animate) = animate.clone() {
            combined.extend(base_animate);
        }

        // Add scroll animation properties
        if let Some(scroll_mgr) = scroll_manager.get() {
            let scroll_target = scroll_mgr.create_animation_target();
            combined.extend(scroll_target);
        }

        // Add FLIP animation properties
        if let Some(flip_mgr) = flip_manager.get() {
            match flip_mgr.get_state() {
                FLIPState::First | FLIPState::Inverted => {
                    combined.insert("x".to_string(), AnimationValue::Pixels(0.0));
                    combined.insert("y".to_string(), AnimationValue::Pixels(0.0));
                    combined.insert("scaleX".to_string(), AnimationValue::Number(1.0));
                    combined.insert("scaleY".to_string(), AnimationValue::Number(1.0));
                }
                FLIPState::Playing | FLIPState::Completed => {
                    combined.insert("x".to_string(), AnimationValue::Pixels(0.0));
                    combined.insert("y".to_string(), AnimationValue::Pixels(0.0));
                    combined.insert("scaleX".to_string(), AnimationValue::Number(1.0));
                    combined.insert("scaleY".to_string(), AnimationValue::Number(1.0));
                }
                _ => {}
            }
        }

        // Add gesture animation properties
        if let Some(gesture_mgr) = gesture_manager.get() {
            combined.extend(gesture_mgr.get_animation_target().clone());
        }

        set_combined_animation_target.set(combined);
    });

    // Event handlers for gesture integration
    let on_mouse_enter = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_mouse_enter(&event) {
                console_error!("Failed to handle mouse enter: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    let on_mouse_leave = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_mouse_leave(&event) {
                console_error!("Failed to handle mouse leave: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    let on_mouse_down = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_mouse_down(&event) {
                console_error!("Failed to handle mouse down: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    let on_mouse_move = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_mouse_move(&event) {
                console_error!("Failed to handle mouse move: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    let on_mouse_up = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_mouse_up(&event) {
                console_error!("Failed to handle mouse up: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    let on_click = move |event: MouseEvent| {
        if let Some(mut manager) = gesture_manager.get() {
            if let Err(e) = manager.handle_click(&event) {
                console_error!("Failed to handle click: {:?}", e);
            } else {
                set_gesture_manager.set(Some(manager));
            }
        }
    };

    // Create reactive animation target
    let animation_target = move || combined_animation_target.get();

    view! {
        <ReactiveMotionDiv
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            node_ref=node_ref
            animate=reactive_animate(animation_target)
            _transition=transition.unwrap_or_default()
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:click=on_click
        >
            {children()}
        </ReactiveMotionDiv>
    }
}

/// Scroll-specific MotionDiv component
#[component]
pub fn ScrollMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Scroll animation configuration
    #[prop(optional)]
    scroll_config: Option<ScrollAnimationConfig>,
    /// Base animation properties
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    view! {
        <IntegratedMotionDiv
            class=class
            style=style
            node_ref=node_ref
            animate=animate
            transition=transition
            scroll_config=scroll_config
            scroll_enabled=true
            flip_enabled=false
            gesture_enabled=false
        >
            {children()}
        </IntegratedMotionDiv>
    }
}

/// FLIP-specific MotionDiv component
#[component]
pub fn FLIPMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// FLIP animation configuration
    #[prop(optional)]
    flip_config: Option<FLIPConfig>,
    /// Base animation properties
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Whether the element should be in its "moved" state
    #[prop(optional)]
    moved: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let moved = moved.unwrap_or(false);

    // Handle FLIP state changes
    Effect::new(move |_| {
        if moved {
            // Trigger FLIP animation when moved state changes
            console_log!("FLIP animation triggered - element moved");
        }
    });

    view! {
        <IntegratedMotionDiv
            class=class
            style=style
            node_ref=node_ref
            animate=animate
            transition=transition
            flip_config=flip_config
            scroll_enabled=false
            flip_enabled=true
            gesture_enabled=false
        >
            {children()}
        </IntegratedMotionDiv>
    }
}

/// Gesture-specific MotionDiv component
#[component]
pub fn GestureMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Gesture animation configuration
    #[prop(optional)]
    gesture_config: Option<GestureAnimationConfig>,
    /// Base animation properties
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    view! {
        <IntegratedMotionDiv
            class=class
            style=style
            node_ref=node_ref
            animate=animate
            transition=transition
            gesture_config=gesture_config
            scroll_enabled=false
            flip_enabled=false
            gesture_enabled=true
        >
            {children()}
        </IntegratedMotionDiv>
    }
}

/// All-in-one MotionDiv with all features enabled
#[component]
pub fn FullMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Base animation properties
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Scroll animation configuration
    #[prop(optional)]
    scroll_config: Option<ScrollAnimationConfig>,
    /// FLIP animation configuration
    #[prop(optional)]
    flip_config: Option<FLIPConfig>,
    /// Gesture animation configuration
    #[prop(optional)]
    gesture_config: Option<GestureAnimationConfig>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    view! {
        <IntegratedMotionDiv
            class=class
            style=style
            node_ref=node_ref
            animate=animate
            transition=transition
            scroll_config=scroll_config
            flip_config=flip_config
            gesture_config=gesture_config
            scroll_enabled=true
            flip_enabled=true
            gesture_enabled=true
        >
            {children()}
        </IntegratedMotionDiv>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_integrated_motion_div_creation() {
        let config = ScrollAnimationConfig::default();

        mount_to_body(move || {
            view! {
                <IntegratedMotionDiv
                    class="test-integrated".to_string()
                    scroll_config=config
                    scroll_enabled=true
                    flip_enabled=false
                    gesture_enabled=false
                >
                    "Integrated MotionDiv Test"
                </IntegratedMotionDiv>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_scroll_motion_div_component() {
        let config = ScrollAnimationConfig {
            threshold: 0.1,
            root_margin: "0px".to_string(),
            trigger_point: 0.5,
            duration: 0.3,
            easing: Easing::EaseOut,
        };

        mount_to_body(move || {
            view! {
                <div style="height: 2000px; padding: 20px;">
                    <div style="height: 500px; background: #f0f0f0;"></div>
                    <ScrollMotionDiv
                        class="test-scroll-element".to_string()
                        scroll_config=config
                    >
                        "This element will animate on scroll"
                    </ScrollMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_flip_motion_div_component() {
        let config = FLIPConfig {
            duration: 0.3,
            ease: Easing::EaseOut,
            animate_scale: true,
            animate_rotation: false,
            animate_opacity: false,
            z_index: 1000,
        };

        mount_to_body(move || {
            view! {
                <div style="position: relative; width: 100%; height: 100vh;">
                    <FLIPMotionDiv
                        class="test-flip-element".to_string()
                        flip_config=config
                        moved=false
                    >
                        "This element will FLIP when moved"
                    </FLIPMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_gesture_motion_div_component() {
        let config = GestureAnimationConfig {
            drag_enabled: true,
            hover_enabled: true,
            tap_enabled: true,
            pinch_enabled: false,
            transition_duration: 0.2,
            easing: Easing::EaseOut,
            spring_config: SpringConfig::default(),
            drag_constraints: DragConstraints::default(),
            hover_properties: HashMap::new(),
            tap_properties: HashMap::new(),
        };

        mount_to_body(move || {
            view! {
                <div style="position: relative; width: 100%; height: 100vh;">
                    <GestureMotionDiv
                        class="test-gesture-element".to_string()
                        gesture_config=config
                    >
                        "This element responds to gestures"
                    </GestureMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_full_motion_div_component() {
        let scroll_config = ScrollAnimationConfig::default();
        let flip_config = FLIPConfig::default();
        let gesture_config = GestureAnimationConfig::default();

        mount_to_body(move || {
            view! {
                <div style="position: relative; width: 100%; height: 100vh;">
                    <FullMotionDiv
                        class="test-full-element".to_string()
                        scroll_config=scroll_config
                        flip_config=flip_config
                        gesture_config=gesture_config
                    >
                        "This element has all features enabled"
                    </FullMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_animation_target_combination() {
        let mut base_animate = HashMap::new();
        base_animate.insert("opacity".to_string(), AnimationValue::Number(0.8));
        base_animate.insert("scale".to_string(), AnimationValue::Number(1.1));

        let scroll_config = ScrollAnimationConfig::default();
        let gesture_config = GestureAnimationConfig::default();

        mount_to_body(move || {
            view! {
                <IntegratedMotionDiv
                    class="test-combined".to_string()
                    animate=base_animate
                    scroll_config=scroll_config
                    gesture_config=gesture_config
                    scroll_enabled=true
                    flip_enabled=false
                    gesture_enabled=true
                >
                    "Combined animation properties test"
                </IntegratedMotionDiv>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_event_handling_integration() {
        let gesture_config = GestureAnimationConfig {
            hover_enabled: true,
            tap_enabled: true,
            drag_enabled: true,
            ..Default::default()
        };

        mount_to_body(move || {
            view! {
                <GestureMotionDiv
                    class="test-events".to_string()
                    gesture_config=gesture_config
                >
                    "Test event handling integration"
                </GestureMotionDiv>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_conditional_feature_enabling() {
        mount_to_body(move || {
            view! {
                <div>
                    <IntegratedMotionDiv
                        class="scroll-only".to_string()
                        scroll_enabled=true
                        flip_enabled=false
                        gesture_enabled=false
                    >
                        "Scroll only"
                    </IntegratedMotionDiv>

                    <IntegratedMotionDiv
                        class="flip-only".to_string()
                        scroll_enabled=false
                        flip_enabled=true
                        gesture_enabled=false
                    >
                        "FLIP only"
                    </IntegratedMotionDiv>

                    <IntegratedMotionDiv
                        class="gesture-only".to_string()
                        scroll_enabled=false
                        flip_enabled=false
                        gesture_enabled=true
                    >
                        "Gesture only"
                    </IntegratedMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_transition_configuration() {
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            delay: Some(0.1),
            ..Default::default()
        };

        mount_to_body(move || {
            view! {
                <IntegratedMotionDiv
                    class="test-transition".to_string()
                    transition=transition
                    scroll_enabled=false
                    flip_enabled=false
                    gesture_enabled=false
                >
                    "Test transition configuration"
                </IntegratedMotionDiv>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_node_ref_integration() {
        let node_ref = NodeRef::new();

        mount_to_body(move || {
            view! {
                <IntegratedMotionDiv
                    class="test-node-ref".to_string()
                    node_ref=node_ref
                    scroll_enabled=false
                    flip_enabled=false
                    gesture_enabled=false
                >
                    "Test node ref integration"
                </IntegratedMotionDiv>
            }
        });
    }
}

/// Performance tests for DOM integration
#[cfg(test)]
mod performance_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_multiple_integrated_components() {
        mount_to_body(move || {
            view! {
                <div>
                    {move || (0..10).map(|i| {
                        view! {
                            <IntegratedMotionDiv
                                class=format!("test-multiple-{}", i)
                                scroll_enabled=true
                                flip_enabled=true
                                gesture_enabled=true
                            >
                                {format!("Multiple component test {}", i)}
                            </IntegratedMotionDiv>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_animation_target_performance() {
        let mut animate = HashMap::new();
        animate.insert("x".to_string(), AnimationValue::Pixels(100.0));
        animate.insert("y".to_string(), AnimationValue::Pixels(100.0));
        animate.insert("scale".to_string(), AnimationValue::Number(1.2));
        animate.insert("opacity".to_string(), AnimationValue::Number(0.8));
        animate.insert("rotate".to_string(), AnimationValue::Number(45.0));

        mount_to_body(move || {
            view! {
                <IntegratedMotionDiv
                    class="test-performance".to_string()
                    animate=animate
                    scroll_enabled=true
                    flip_enabled=true
                    gesture_enabled=true
                >
                    "Performance test with multiple properties"
                </IntegratedMotionDiv>
            }
        });
    }
}
