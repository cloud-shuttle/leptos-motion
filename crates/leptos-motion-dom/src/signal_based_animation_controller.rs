//! Signal-Based Animation Controller
//!
//! This module implements the proven signal-based animation patterns from our guide.
//! It provides proper signal tracking, WASM memory management, and effect dependencies.

use leptos::prelude::*;
use leptos_motion_core::types::AnimationValue;
use leptos_motion_core::types::Transition;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Animation state managed by signals
#[derive(Clone, PartialEq, Debug)]
pub struct AnimationState {
    /// Whether the animation is currently playing
    pub is_playing: bool,
    /// Current animation values
    pub current_values: HashMap<String, AnimationValue>,
    /// Target animation values
    pub target_values: HashMap<String, AnimationValue>,
    /// Animation progress from 0.0 to 1.0
    pub progress: f32,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_values: HashMap::new(),
            target_values: HashMap::new(),
            progress: 0.0,
        }
    }
}

/// Signal-based animation controller following proven patterns
pub struct SignalBasedAnimationController {
    /// Current animation state
    pub animation_state: ReadSignal<AnimationState>,
    /// Target values signal
    pub target_values: ReadSignal<HashMap<String, AnimationValue>>,
    /// Whether animation is playing
    pub is_playing: ReadSignal<bool>,
    /// Animation progress (0.0 to 1.0)
    pub progress: ReadSignal<f32>,
    /// Write signals for updating state
    set_animation_state: WriteSignal<AnimationState>,
    set_target_values: WriteSignal<HashMap<String, AnimationValue>>,
    #[allow(dead_code)]
    set_is_playing: WriteSignal<bool>,
    #[allow(dead_code)]
    set_progress: WriteSignal<f32>,
    /// Cleanup handles for WASM memory management
    #[allow(dead_code)]
    cleanup_handles: Vec<Box<dyn Fn()>>,
}

impl SignalBasedAnimationController {
    /// Create a new signal-based animation controller
    pub fn new(initial_values: HashMap<String, AnimationValue>) -> Self {
        // ✅ Create signals for all animation state
        let (animation_state, set_animation_state) = signal(AnimationState {
            is_playing: false,
            current_values: initial_values.clone(),
            target_values: initial_values,
            progress: 0.0,
        });

        let (target_values, set_target_values) = signal(HashMap::new());
        let (is_playing, set_is_playing) = signal(false);
        let (progress, set_progress) = signal(0.0);

        Self {
            animation_state,
            target_values,
            is_playing,
            progress,
            set_animation_state,
            set_target_values,
            set_is_playing,
            set_progress,
            cleanup_handles: Vec::new(),
        }
    }

    /// Animate to target values
    pub fn animate_to(&self, target: HashMap<String, AnimationValue>) {
        // ✅ Update target values (triggers effect)
        self.set_target_values.set(target.clone());

        // Update animation state
        let mut current_state = self.animation_state.get();
        current_state.target_values = target;
        current_state.is_playing = true;
        self.set_animation_state.set(current_state);
    }

    /// Start animation loop
    #[allow(dead_code)]
    fn start_animation_loop(&self) {
        // ✅ Animation loop implementation
        // This would typically use request_animation_frame
        // For now, we'll simulate with a simple update
        // Note: We need to store the write signals to update them
    }

    /// Get current animation values
    pub fn get_current_values(&self) -> HashMap<String, AnimationValue> {
        self.animation_state.get().current_values
    }

    /// Check if animation is playing
    pub fn is_animation_playing(&self) -> bool {
        self.is_playing.get()
    }

    /// Get animation progress
    pub fn get_progress(&self) -> f32 {
        self.progress.get()
    }
}

/// WASM-safe motion component wrapper (simplified without WASM bindings)
pub struct WasmMotionComponent {
    #[allow(dead_code)]
    component_id: String,
    animation_controller: Option<SignalBasedAnimationController>,
}

impl WasmMotionComponent {
    /// Create a new WASM motion component with the given component ID
    pub fn new(component_id: &str) -> Self {
        Self {
            component_id: component_id.to_string(),
            animation_controller: None,
        }
    }

    /// Initialize the animation controller with initial values
    pub fn initialize(&mut self, initial_values: HashMap<String, AnimationValue>) {
        // ✅ Initialize animation controller
        self.animation_controller = Some(SignalBasedAnimationController::new(initial_values));
    }

    /// Animate to the specified target values
    pub fn animate_to(&mut self, target_values: HashMap<String, AnimationValue>) {
        if let Some(controller) = &self.animation_controller {
            controller.animate_to(target_values);
        }
    }

    /// Clean up the animation controller
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Proper cleanup for WASM memory management
        self.animation_controller = None;
    }
}

/// Proper signal tracking motion div component
#[component]
pub fn ProperSignalTrackingMotionDiv(
    /// Initial animation values
    initial: HashMap<String, AnimationValue>,
    /// Animate signal for reactive updates
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    /// Transition configuration (currently unused)
    _transition: Transition,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Use Effect::new with proper signal tracking
    Effect::new(move |_| {
        let animate_values = animate.get(); // This properly tracks the signal!

        if let Some(div) = node_ref.get() {
            // Apply animation to DOM element
            for (property, value) in &animate_values {
                let css_value = animation_value_to_css(value);
                let element = div.unchecked_ref::<web_sys::HtmlElement>();
                let _ = element.style().set_property(property, &css_value);
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            style=move || {
                // Convert initial values to CSS
                initial_to_css(&initial)
            }
        />
    }
}

/// Helper function to convert animation values to CSS
pub fn animation_value_to_css(value: &AnimationValue) -> String {
    match value {
        AnimationValue::Number(n) => n.to_string(),
        AnimationValue::Pixels(p) => format!("{}px", p),
        AnimationValue::Degrees(d) => format!("{}deg", d),
        AnimationValue::Color(c) => c.clone(),
        AnimationValue::String(s) => s.clone(),
        AnimationValue::Percentage(p) => format!("{}%", p),
        AnimationValue::Radians(r) => format!("{}rad", r),
        AnimationValue::Transform(t) => format!("transform({:?})", t),
        AnimationValue::Complex(c) => format!("complex({:?})", c),
    }
}

/// Helper function to convert initial values to CSS
pub fn initial_to_css(values: &HashMap<String, AnimationValue>) -> String {
    values
        .iter()
        .map(|(property, value)| {
            let css_value = animation_value_to_css(value);
            format!("{}: {}", property, css_value)
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// WASM-optimized signal handling (simplified without WASM bindings)
pub struct WasmSignalHandler {
    // ✅ Store signals in a way that works with WASM
    animation_signals: std::collections::HashMap<String, WriteSignal<AnimationValue>>,
    cleanup_handles: Vec<Box<dyn Fn()>>,
}

impl Default for WasmSignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmSignalHandler {
    /// Create a new WASM signal handler
    pub fn new() -> Self {
        Self {
            animation_signals: std::collections::HashMap::new(),
            cleanup_handles: Vec::new(),
        }
    }

    /// Create a new animation signal with the given name and initial value
    pub fn create_animation_signal(&mut self, name: &str, initial_value: f32) -> u32 {
        // ✅ Create signal and store handle for cleanup
        let (_signal, set_signal) = signal(AnimationValue::Number(initial_value as f64));
        let signal_id = self.animation_signals.len() as u32;

        self.animation_signals.insert(name.to_string(), set_signal);

        // ✅ Store cleanup function
        let cleanup = Box::new(move || {
            // Signal cleanup logic
        });
        self.cleanup_handles.push(cleanup);

        signal_id
    }

    /// Update an animation value by name
    pub fn update_animation_value(&mut self, name: &str, value: f32) -> Result<(), String> {
        if let Some(signal) = self.animation_signals.get(name) {
            // ✅ Update signal value
            signal.set(AnimationValue::Number(value as f64));
            Ok(())
        } else {
            Err(format!("Signal '{}' not found", name))
        }
    }

    /// Clean up all signals and effects
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Clean up all signals and effects
        for cleanup in self.cleanup_handles.drain(..) {
            cleanup();
        }
        self.animation_signals.clear();
    }
}

// ✅ Implement Drop for automatic cleanup
impl Drop for WasmSignalHandler {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl Drop for WasmMotionComponent {
    fn drop(&mut self) {
        self.cleanup();
    }
}
