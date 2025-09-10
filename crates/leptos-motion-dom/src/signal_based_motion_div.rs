//! Signal-Based MotionDiv Component
//! 
//! This component implements the proven patterns from the user's guide for
//! proper signal tracking, WASM memory management, and effect dependencies.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use crate::signal_based_controller::*;
use std::result::Result as StdResult;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;

/// Signal-based MotionDiv component that properly tracks all signal changes
#[component]
pub fn SignalBasedMotionDiv(
    /// Initial animation values
    initial: HashMap<String, AnimationValue>,
    /// Animate signal that triggers reactive updates
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    /// Transition configuration
    transition: Transition,
    /// Whether the component is visible
    is_visible: Option<ReadSignal<bool>>,
    /// Children to render
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    
    // ✅ Create signal-based animation controller
    let animation_controller = SignalBasedAnimationController::new(initial.clone());
    
    // ✅ CRITICAL: Effect that properly tracks animate signal changes
    Effect::new(move |_| {
        let animate_values = animate.get();
        
        // Update animation controller with new target values
        animation_controller.animate_to(animate_values);
    });
    
    // ✅ Effect to update DOM based on animation state
    Effect::new(move |_| {
        let state = animation_controller.animation_state.get();
        
        if let Some(div) = node_ref.get() {
            // Convert to web_sys::Element for DOM manipulation
            let element = div.into_web_sys_element();
            
            // Update element styles based on current animation values
            if let Err(e) = update_element_styles(&element, &state.current_values) {
                web_sys::console::error_1(&format!("Failed to update element styles: {:?}", e).into());
            }
        }
    });
    
    // ✅ Effect for visibility changes
    if let Some(visibility_signal) = is_visible {
        Effect::new(move |_| {
            let visible = visibility_signal.get();
            
            if let Some(div) = node_ref.get() {
                let element = div.into_web_sys_element();
                let class_name = if visible { "visible" } else { "hidden" };
                
                element.set_class_name(&class_name);
            }
        });
    }
    
    // ✅ CRITICAL: Add proper WASM memory management with cleanup
    Effect::new(move |_| {
        // This effect runs when the component is created and tracks all signals
        // When the component is destroyed, this effect will be cleaned up automatically
        
        // Track all animation-related signals to ensure proper reactivity
        let _ = animation_controller.current_values.get();
        let _ = animation_controller.target_values.get();
        let _ = animation_controller.is_playing.get();
        let _ = animation_controller.progress.get();
        let _ = animation_controller.animation_state.get();
        
        // Return cleanup function (this will be called when the effect is destroyed)
        move || {
            // Cleanup any pending timeouts or animation frames
            // The SignalBasedAnimationController handles its own cleanup
            web_sys::console::log_1(&"SignalBasedMotionDiv: Component cleanup".into());
        }
    });

    view! {
        <div
            node_ref=node_ref
            style=move || {
                // Convert initial values to CSS
                animation_values_to_css(&initial)
            }
        >
            {children()}
        </div>
    }
}

/// Reactive MotionDiv that uses proper signal tracking patterns
#[component]
pub fn ReactiveSignalBasedMotionDiv(
    /// Animate signal that triggers reactive updates
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    /// Transition configuration signal
    transition: ReadSignal<Transition>,
    /// Whether the component is visible
    is_visible: ReadSignal<bool>,
    /// Children to render
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    
    let (effect_run_count, set_effect_run_count) = signal(0);
    
    // ✅ CRITICAL: Effect with explicit dependencies
    Effect::new(move |_| {
        // This effect will re-run when ANY of these signals change:
        let animate_values = animate.get();  // Dependency 1
        let transition_config = transition.get();  // Dependency 2
        let visible = is_visible.get();  // Dependency 3
        
        set_effect_run_count.update(|count| *count += 1);
        
        if visible {
            if let Some(div) = node_ref.get() {
                let element = div.into_web_sys_element();
                
                // Apply animation to DOM element
                if let Err(e) = apply_animation_to_element(&element, &animate_values, &transition_config) {
                    web_sys::console::error_1(&format!("Failed to apply animation: {:?}", e).into());
                }
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || if is_visible.get() { "visible" } else { "hidden" }
        >
            {children()}
        </div>
    }
}

/// Simple MotionDiv that demonstrates proper signal tracking
#[component]
pub fn SimpleSignalBasedMotionDiv(
    /// Animate signal that triggers reactive updates
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    /// Children to render
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Use create_effect for reactive animations
    Effect::new(move |_| {
        let animate_values = animate.get();  // This properly tracks the signal!

        if let Some(div) = node_ref.get() {
            let element = div.into_web_sys_element();
            
            // Apply animation to DOM element
            for (property, value) in animate_values {
                let css_value = animation_value_to_css(&value);
                if let Err(e) = element
                    .dyn_ref::<web_sys::HtmlElement>()
                    .unwrap()
                    .style()
                    .set_property(&property, &css_value) {
                    web_sys::console::error_1(&format!("Failed to set CSS property {}: {:?}", property, e).into());
                }
            }
        }
    });

    view! {
        <div node_ref=node_ref>
            {children()}
        </div>
    }
}

/// WASM-safe MotionDiv wrapper
#[wasm_bindgen]
pub struct WasmMotionDivWrapper {
    component_id: String,
    animation_controller: Option<SignalBasedAnimationController>,
}

#[wasm_bindgen]
impl WasmMotionDivWrapper {
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
