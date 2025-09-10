//! Signal-Based Animation Controller
//!
//! This module implements the proven patterns from the user's guide for
//! proper signal tracking, WASM memory management, and effect dependencies.

use leptos::prelude::*;
use leptos_motion_core::*;
use serde_wasm_bindgen;
use std::collections::HashMap;
use std::result::Result as StdResult;
use wasm_bindgen::prelude::*;

/// Animation state managed with signals for proper reactivity
#[derive(Clone, PartialEq, Debug)]
pub struct AnimationState {
    pub is_playing: bool,
    pub current_values: HashMap<String, AnimationValue>,
    pub target_values: HashMap<String, AnimationValue>,
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

/// Signal-based animation controller that properly tracks all state changes
pub struct SignalBasedAnimationController {
    pub current_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub target_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub is_playing: ReadSignal<bool>,
    pub progress: ReadSignal<f32>,
    pub animation_state: ReadSignal<AnimationState>,
    // Store write signals for updates
    _set_current_values: WriteSignal<HashMap<String, AnimationValue>>,
    _set_target_values: WriteSignal<HashMap<String, AnimationValue>>,
    _set_is_playing: WriteSignal<bool>,
    _set_progress: WriteSignal<f32>,
}

impl SignalBasedAnimationController {
    /// Create a new signal-based animation controller
    pub fn new(initial_values: HashMap<String, AnimationValue>) -> Self {
        let (current_values, set_current_values) = signal(initial_values.clone());
        let (target_values, set_target_values) = signal(initial_values.clone());
        let (is_playing, set_is_playing) = signal(false);
        let (progress, set_progress) = signal(0.0);

        // Create combined animation state signal
        let (animation_state, set_animation_state) = signal(AnimationState {
            is_playing: false,
            current_values: initial_values.clone(),
            target_values: initial_values,
            progress: 0.0,
        });

        // ✅ CRITICAL: Effect that properly tracks all signal changes
        Effect::new(move |_| {
            let current = current_values.get();
            let target = target_values.get();
            let playing = is_playing.get();
            let prog = progress.get();

            // Update combined state
            set_animation_state.set(AnimationState {
                is_playing: playing,
                current_values: current,
                target_values: target,
                progress: prog,
            });
        });

        Self {
            current_values,
            target_values,
            is_playing,
            progress,
            animation_state,
            _set_current_values: set_current_values,
            _set_target_values: set_target_values,
            _set_is_playing: set_is_playing,
            _set_progress: set_progress,
        }
    }

    /// Animate to target values - properly triggers reactive updates
    pub fn animate_to(&self, target: HashMap<String, AnimationValue>) {
        // ✅ Update target values (triggers effect)
        self._set_target_values.set(target);
        self._set_is_playing.set(true);
        self._set_progress.set(0.0);
    }

    /// Update animation progress
    pub fn update_progress(&self, progress: f32) {
        self._set_progress.set(progress.clamp(0.0, 1.0));
    }

    /// Complete the animation
    pub fn complete_animation(&self) {
        self._set_is_playing.set(false);
        self._set_progress.set(1.0);
        // Update current values to match target values
        self._set_current_values.set(self.target_values.get());
    }

    /// Stop the animation
    pub fn stop_animation(&self) {
        self._set_is_playing.set(false);
    }

    /// Reset to initial state
    pub fn reset(&self) {
        self._set_is_playing.set(false);
        self._set_progress.set(0.0);
        self._set_current_values.set(self.target_values.get());
    }
}

/// WASM-optimized motion controller with proper memory management
#[wasm_bindgen]
pub struct WasmMotionController {
    signals: std::collections::HashMap<String, (ReadSignal<f32>, WriteSignal<f32>)>,
    cleanup_handles: Vec<Box<dyn Fn()>>,
}

#[wasm_bindgen]
impl WasmMotionController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            signals: std::collections::HashMap::new(),
            cleanup_handles: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn create_animation_signal(&mut self, name: &str, initial_value: f32) -> u32 {
        // ✅ Create signal and store handle for cleanup
        let (read_signal, write_signal) = signal(initial_value);
        let signal_id = self.signals.len() as u32;

        self.signals
            .insert(name.to_string(), (read_signal, write_signal));

        // ✅ Store cleanup function
        let cleanup = Box::new(move || {
            // Signal cleanup logic - signals are automatically cleaned up by Leptos
        });
        self.cleanup_handles.push(cleanup);

        signal_id
    }

    #[wasm_bindgen]
    pub fn update_animation_value(&mut self, name: &str, value: f32) -> StdResult<(), JsValue> {
        if let Some((_, write_signal)) = self.signals.get(name) {
            // ✅ Update signal value
            write_signal.set(value);
            Ok(())
        } else {
            Err(JsValue::from_str(&format!("Signal '{}' not found", name)))
        }
    }

    #[wasm_bindgen]
    pub fn get_animation_value(&self, name: &str) -> Option<f32> {
        self.signals
            .get(name)
            .map(|(read_signal, _)| read_signal.get())
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Clean up all signals and effects
        for cleanup in self.cleanup_handles.drain(..) {
            cleanup();
        }
        self.signals.clear();
    }
}

/// WASM-safe component wrapper for motion components
#[wasm_bindgen]
pub struct WasmMotionComponent {
    component_id: String,
    animation_controller: Option<SignalBasedAnimationController>,
}

#[wasm_bindgen]
impl WasmMotionComponent {
    #[wasm_bindgen(constructor)]
    pub fn new(component_id: &str) -> Self {
        Self {
            component_id: component_id.to_string(),
            animation_controller: None,
        }
    }

    #[wasm_bindgen]
    pub fn initialize(&mut self, initial_values: &JsValue) -> StdResult<(), JsValue> {
        // ✅ Initialize animation controller
        let values: HashMap<String, AnimationValue> =
            serde_wasm_bindgen::from_value(initial_values.clone())?;

        self.animation_controller = Some(SignalBasedAnimationController::new(values));
        Ok(())
    }

    #[wasm_bindgen]
    pub fn animate_to(&mut self, target_values: &JsValue) -> StdResult<(), JsValue> {
        if let Some(controller) = &self.animation_controller {
            let values: HashMap<String, AnimationValue> =
                serde_wasm_bindgen::from_value(target_values.clone())?;

            controller.animate_to(values);
            Ok(())
        } else {
            Err(JsValue::from_str("Component not initialized"))
        }
    }

    #[wasm_bindgen]
    pub fn get_animation_state(&self) -> StdResult<JsValue, JsValue> {
        if let Some(controller) = &self.animation_controller {
            let state = controller.animation_state.get();
            serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Err(JsValue::from_str("Component not initialized"))
        }
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ Proper cleanup
        self.animation_controller = None;
    }
}

/// Proper signal tracking with create_effect patterns
pub struct ProperEffectMotionDiv {
    pub animate_signal: ReadSignal<HashMap<String, AnimationValue>>,
    pub transition_signal: ReadSignal<Transition>,
    pub is_visible_signal: ReadSignal<bool>,
    pub effect_run_count: ReadSignal<u32>,
}

impl ProperEffectMotionDiv {
    pub fn new(
        animate: ReadSignal<HashMap<String, AnimationValue>>,
        transition: ReadSignal<Transition>,
        is_visible: ReadSignal<bool>,
    ) -> Self {
        let (effect_run_count, set_effect_run_count) = signal(0);

        // ✅ CRITICAL: Effect with explicit dependencies
        Effect::new(move |_| {
            // This effect will re-run when ANY of these signals change:
            let _animate_values = animate.get(); // Dependency 1
            let _transition_config = transition.get(); // Dependency 2
            let _visible = is_visible.get(); // Dependency 3

            set_effect_run_count.update(|count| *count += 1);
        });

        Self {
            animate_signal: animate,
            transition_signal: transition,
            is_visible_signal: is_visible,
            effect_run_count,
        }
    }

    pub fn get_effect_run_count(&self) -> u32 {
        self.effect_run_count.get()
    }
}

/// Helper function to convert animation values to CSS
pub fn animation_value_to_css(value: &AnimationValue) -> String {
    match value {
        AnimationValue::Pixels(val) => format!("{}px", val),
        AnimationValue::Number(val) => val.to_string(),
        AnimationValue::Degrees(val) => format!("{}deg", val),
        AnimationValue::Percentage(val) => format!("{}%", val),
        AnimationValue::Radians(val) => format!("{}rad", val),
        AnimationValue::Color(val) => val.clone(),
        AnimationValue::String(val) => val.clone(),
        AnimationValue::Transform(_) => "matrix(1,0,0,1,0,0)".to_string(),
        AnimationValue::Complex(_) => "0".to_string(),
    }
}

/// Helper function to convert animation values to CSS for a HashMap
pub fn animation_values_to_css(values: &HashMap<String, AnimationValue>) -> String {
    values
        .iter()
        .map(|(key, value)| format!("{}: {}", key, animation_value_to_css(value)))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Apply animation to DOM element using proper signal tracking
pub fn apply_animation_to_element(
    element: &web_sys::Element,
    animate: &HashMap<String, AnimationValue>,
    _transition: &Transition,
) -> StdResult<(), JsValue> {
    // Use Web Animations API or CSS transitions
    for (property, value) in animate {
        let css_value = animation_value_to_css(value);
        element
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .style()
            .set_property(property, &css_value)
            .map_err(|_| JsValue::from_str("Failed to set CSS property"))?;
    }
    Ok(())
}

/// Update element styles based on animation state
pub fn update_element_styles(
    element: &web_sys::Element,
    current_values: &HashMap<String, AnimationValue>,
) -> StdResult<(), JsValue> {
    for (property, value) in current_values {
        let css_value = animation_value_to_css(value);
        element
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .style()
            .set_property(property, &css_value)
            .map_err(|_| JsValue::from_str("Failed to set CSS property"))?;
    }
    Ok(())
}
