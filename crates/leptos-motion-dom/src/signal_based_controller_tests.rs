//! TDD Tests for Signal-Based Animation Controller
//! 
//! These tests implement the proven patterns from the user's guide to ensure
//! proper signal tracking, WASM memory management, and effect dependencies.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;
// Note: console_error_panic_hook is not available in this test environment
// use console_error_panic_hook;

// Test 1: Signal-Based Animation State Management
#[wasm_bindgen_test]
fn test_signal_based_animation_state_management() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    // ✅ Test that animation state is properly managed with signals
    let (animation_state, set_animation_state) = signal(AnimationState {
        is_playing: false,
        current_values: HashMap::new(),
        target_values: HashMap::new(),
        progress: 0.0,
    });
    
    // Test initial state
    assert!(!animation_state.get().is_playing);
    assert_eq!(animation_state.get().progress, 0.0);
    
    // Test state updates
    set_animation_state.update(|state| {
        state.is_playing = true;
        state.progress = 0.5;
    });
    
    assert!(animation_state.get().is_playing);
    assert_eq!(animation_state.get().progress, 0.5);
}

// Test 2: Proper Effect Dependencies
#[wasm_bindgen_test]
fn test_proper_effect_dependencies() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    let (animate_signal, set_animate_signal) = signal(HashMap::new());
    let (transition_signal, set_transition_signal) = signal(Transition::default());
    let (is_visible_signal, set_is_visible_signal) = signal(true);
    
    let (effect_run_count, set_effect_run_count) = signal(0);
    
    // ✅ Test that effect properly tracks all signal dependencies
    Effect::new(move |_| {
        // This effect should re-run when ANY of these signals change
        let _animate_values = animate_signal.get();  // Dependency 1
        let _transition_config = transition_signal.get();  // Dependency 2
        let _visible = is_visible_signal.get();  // Dependency 3
        
        set_effect_run_count.update(|count| *count += 1);
    });
    
    // Initial effect run
    assert_eq!(effect_run_count.get(), 1);
    
    // Change animate signal - should trigger effect
    set_animate_signal.set({
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        target
    });
    assert_eq!(effect_run_count.get(), 2);
    
    // Change transition signal - should trigger effect
    set_transition_signal.set(Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.1),
        repeat: RepeatConfig::Never,
        stagger: None,
    });
    assert_eq!(effect_run_count.get(), 3);
    
    // Change visibility signal - should trigger effect
    set_is_visible_signal.set(false);
    assert_eq!(effect_run_count.get(), 4);
}

// Test 3: WASM-Optimized Signal Handling
#[wasm_bindgen_test]
fn test_wasm_optimized_signal_handling() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    // ✅ Test WASM-optimized signal management
    let mut controller = WasmMotionController::new();
    
    // Test signal creation
    let signal_id = controller.create_animation_signal("x", 0.0);
    assert_eq!(signal_id, 0);
    
    // Test signal update
    let result = controller.update_animation_value("x", 100.0);
    assert!(result.is_ok());
    
    // Test signal retrieval
    let value = controller.get_animation_value("x");
    assert_eq!(value, Some(100.0));
    
    // Test cleanup
    controller.cleanup();
    // After cleanup, signal should not exist
    let value_after_cleanup = controller.get_animation_value("x");
    assert_eq!(value_after_cleanup, None);
}

// Test 4: Signal-Based Animation Controller
#[wasm_bindgen_test]
fn test_signal_based_animation_controller() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    // ✅ Test complete animation controller using signals
    let initial_values = {
        let mut values = HashMap::new();
        values.insert("x".to_string(), AnimationValue::Pixels(0.0));
        values.insert("y".to_string(), AnimationValue::Pixels(0.0));
        values
    };
    
    let controller = AnimationController::new(initial_values.clone());
    
    // Test initial state
    assert_eq!(controller.current_values.get(), initial_values);
    assert!(!controller.is_playing.get());
    assert_eq!(controller.progress.get(), 0.0);
    
    // Test animation trigger
    let target_values = {
        let mut values = HashMap::new();
        values.insert("x".to_string(), AnimationValue::Pixels(100.0));
        values.insert("y".to_string(), AnimationValue::Pixels(200.0));
        values
    };
    
    controller.animate_to(target_values.clone());
    
    // After animation trigger, should be playing
    assert!(controller.is_playing.get());
    assert_eq!(controller.target_values.get(), target_values);
}

// Test 5: Proper Signal Tracking with create_effect
#[wasm_bindgen_test]
fn test_proper_signal_tracking_with_create_effect() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    let (animate_signal, set_animate_signal) = signal(HashMap::new());
    let (transition_signal, set_transition_signal) = signal(Transition::default());
    
    let (effect_run_count, set_effect_run_count) = signal(0);
    let (last_animate_values, set_last_animate_values) = signal(HashMap::new());
    let (last_transition_config, set_last_transition_config) = signal(Transition::default());
    
    // ✅ Test proper signal tracking with create_effect
    Effect::new(move |_| {
        let animate_values = animate_signal.get();
        let transition_config = transition_signal.get();
        
        set_last_animate_values.set(animate_values.clone());
        set_last_transition_config.set(transition_config.clone());
        set_effect_run_count.update(|count| *count += 1);
    });
    
    // Initial effect run
    assert_eq!(effect_run_count.get(), 1);
    
    // Update animate signal
    set_animate_signal.set({
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(50.0));
        target
    });
    
    // Effect should have run again
    assert_eq!(effect_run_count.get(), 2);
    assert_eq!(last_animate_values.get().get("x"), Some(&AnimationValue::Pixels(50.0)));
    
    // Update transition signal
    set_transition_signal.set(Transition {
        duration: Some(1.0),
        ease: Easing::EaseInOut,
        delay: Some(0.0),
        repeat: RepeatConfig::Never,
        stagger: None,
    });
    
    // Effect should have run again
    assert_eq!(effect_run_count.get(), 3);
    assert_eq!(last_transition_config.get().duration, Some(1.0));
}

// Test 6: WASM Memory Management
#[wasm_bindgen_test]
fn test_wasm_memory_management() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    // ✅ Test proper WASM memory management
    let mut controller = WasmMotionController::new();
    
    // Create multiple signals
    let _signal1 = controller.create_animation_signal("x", 0.0);
    let _signal2 = controller.create_animation_signal("y", 0.0);
    let _signal3 = controller.create_animation_signal("rotation", 0.0);
    
    // Verify signals exist
    assert!(controller.get_animation_value("x").is_some());
    assert!(controller.get_animation_value("y").is_some());
    assert!(controller.get_animation_value("rotation").is_some());
    
    // Test cleanup
    controller.cleanup();
    
    // After cleanup, all signals should be gone
    assert!(controller.get_animation_value("x").is_none());
    assert!(controller.get_animation_value("y").is_none());
    assert!(controller.get_animation_value("rotation").is_none());
}

// Test 7: Signal-Based Animation State Management with Complex State
#[wasm_bindgen_test]
fn test_complex_animation_state_management() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    // ✅ Test complex animation state management
    let (animation_state, set_animation_state) = signal(AnimationState {
        is_playing: false,
        current_values: {
            let mut values = HashMap::new();
            values.insert("x".to_string(), AnimationValue::Pixels(0.0));
            values.insert("y".to_string(), AnimationValue::Pixels(0.0));
            values.insert("opacity".to_string(), AnimationValue::Number(1.0));
            values
        },
        target_values: HashMap::new(),
        progress: 0.0,
    });
    
    // Test complex state updates
    set_animation_state.update(|state| {
        state.is_playing = true;
        state.progress = 0.25;
        state.current_values.insert("x".to_string(), AnimationValue::Pixels(25.0));
        state.current_values.insert("y".to_string(), AnimationValue::Pixels(50.0));
        state.current_values.insert("opacity".to_string(), AnimationValue::Number(0.75));
    });
    
    let state = animation_state.get();
    assert!(state.is_playing);
    assert_eq!(state.progress, 0.25);
    assert_eq!(state.current_values.get("x"), Some(&AnimationValue::Pixels(25.0)));
    assert_eq!(state.current_values.get("y"), Some(&AnimationValue::Pixels(50.0)));
    assert_eq!(state.current_values.get("opacity"), Some(&AnimationValue::Number(0.75)));
}

// Test 8: Effect Dependencies with Multiple Signals
#[wasm_bindgen_test]
fn test_effect_dependencies_with_multiple_signals() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    let (signal_a, set_signal_a) = signal(0.0);
    let (signal_b, set_signal_b) = signal(0.0);
    let (signal_c, set_signal_c) = signal(0.0);
    
    let (effect_run_count, set_effect_run_count) = signal(0);
    let (last_values, set_last_values) = signal((0.0, 0.0, 0.0));
    
    // ✅ Test effect with multiple signal dependencies
    Effect::new(move |_| {
        let a = signal_a.get();
        let b = signal_b.get();
        let c = signal_c.get();
        
        set_last_values.set((a, b, c));
        set_effect_run_count.update(|count| *count += 1);
    });
    
    // Initial run
    assert_eq!(effect_run_count.get(), 1);
    assert_eq!(last_values.get(), (0.0, 0.0, 0.0));
    
    // Change signal A
    set_signal_a.set(10.0);
    assert_eq!(effect_run_count.get(), 2);
    assert_eq!(last_values.get(), (10.0, 0.0, 0.0));
    
    // Change signal B
    set_signal_b.set(20.0);
    assert_eq!(effect_run_count.get(), 3);
    assert_eq!(last_values.get(), (10.0, 20.0, 0.0));
    
    // Change signal C
    set_signal_c.set(30.0);
    assert_eq!(effect_run_count.get(), 4);
    assert_eq!(last_values.get(), (10.0, 20.0, 30.0));
}

// Test 9: WASM Controller with Error Handling
#[wasm_bindgen_test]
fn test_wasm_controller_error_handling() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    let mut controller = WasmMotionController::new();
    
    // Test updating non-existent signal
    let result = controller.update_animation_value("nonexistent", 100.0);
    assert!(result.is_err());
    
    // Test getting non-existent signal
    let value = controller.get_animation_value("nonexistent");
    assert_eq!(value, None);
    
    // Test creating signal with same name twice
    let _signal1 = controller.create_animation_signal("x", 0.0);
    let _signal2 = controller.create_animation_signal("x", 100.0);
    
    // Second signal should overwrite first
    let value = controller.get_animation_value("x");
    assert_eq!(value, Some(100.0));
}

// Test 10: Animation Controller with Complex Animations
#[wasm_bindgen_test]
fn test_animation_controller_complex_animations() {
    // console_error_panic_hook::set_once(); // Not available in test environment
    
    let initial_values = {
        let mut values = HashMap::new();
        values.insert("x".to_string(), AnimationValue::Pixels(0.0));
        values.insert("y".to_string(), AnimationValue::Pixels(0.0));
        values.insert("scale".to_string(), AnimationValue::Number(1.0));
        values.insert("rotation".to_string(), AnimationValue::Degrees(0.0));
        values.insert("opacity".to_string(), AnimationValue::Number(1.0));
        values
    };
    
    let controller = AnimationController::new(initial_values.clone());
    
    // Test complex animation sequence
    let target_values = {
        let mut values = HashMap::new();
        values.insert("x".to_string(), AnimationValue::Pixels(200.0));
        values.insert("y".to_string(), AnimationValue::Pixels(150.0));
        values.insert("scale".to_string(), AnimationValue::Number(1.5));
        values.insert("rotation".to_string(), AnimationValue::Degrees(45.0));
        values.insert("opacity".to_string(), AnimationValue::Number(0.8));
        values
    };
    
    controller.animate_to(target_values.clone());
    
    // Verify animation state
    assert!(controller.is_playing.get());
    assert_eq!(controller.target_values.get(), target_values);
    assert_eq!(controller.current_values.get(), initial_values);
    
    // Test animation progress update
    controller.update_progress(0.5);
    assert_eq!(controller.progress.get(), 0.5);
    
    // Test animation completion
    controller.complete_animation();
    assert!(!controller.is_playing.get());
    assert_eq!(controller.progress.get(), 1.0);
    assert_eq!(controller.current_values.get(), target_values);
}

// Supporting types and implementations for tests

#[derive(Clone, PartialEq, Debug)]
pub struct AnimationState {
    pub is_playing: bool,
    pub current_values: HashMap<String, AnimationValue>,
    pub target_values: HashMap<String, AnimationValue>,
    pub progress: f32,
}

pub struct AnimationController {
    pub current_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub target_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub is_playing: ReadSignal<bool>,
    pub progress: ReadSignal<f32>,
    // ✅ Store write signals for updates
    set_current_values: WriteSignal<HashMap<String, AnimationValue>>,
    set_target_values: WriteSignal<HashMap<String, AnimationValue>>,
    set_is_playing: WriteSignal<bool>,
    set_progress: WriteSignal<f32>,
}

impl AnimationController {
    pub fn new(initial_values: HashMap<String, AnimationValue>) -> Self {
        let (current_values, set_current_values) = signal(initial_values.clone());
        let (target_values, set_target_values) = signal(initial_values);
        let (is_playing, set_is_playing) = signal(false);
        let (progress, set_progress) = signal(0.0);

        Self {
            current_values,
            target_values,
            is_playing,
            progress,
            set_current_values,
            set_target_values,
            set_is_playing,
            set_progress,
        }
    }

    pub fn animate_to(&self, target: HashMap<String, AnimationValue>) {
        // ✅ Use write signals for updates
        self.set_target_values.set(target);
        self.set_is_playing.set(true);
    }
    
    pub fn update_progress(&self, progress: f32) {
        self.set_progress.set(progress);
    }
    
    pub fn complete_animation(&self) {
        self.set_is_playing.set(false);
        self.set_progress.set(1.0);
        // Update current values to match target values
        self.set_current_values.set(self.target_values.get());
    }
}

pub struct WasmMotionController {
    signals: std::collections::HashMap<String, (ReadSignal<f32>, WriteSignal<f32>)>,
}

impl WasmMotionController {
    pub fn new() -> Self {
        Self {
            signals: std::collections::HashMap::new(),
        }
    }

    pub fn create_animation_signal(&mut self, name: &str, initial: f32) -> u32 {
        let (read_signal, write_signal) = signal(initial);
        let signal_id = self.signals.len() as u32;
        self.signals.insert(name.to_string(), (read_signal, write_signal));
        signal_id
    }

    pub fn update_animation_value(&mut self, name: &str, value: f32) -> std::result::Result<(), AnimationError> {
        if let Some((_, write_signal)) = self.signals.get(name) {
            write_signal.set(value);
            Ok(())
        } else {
            Err(AnimationError::InvalidProperty { property: format!("Signal '{}' not found", name) })
        }
    }
    
    pub fn get_animation_value(&self, name: &str) -> Option<f32> {
        self.signals.get(name).map(|(read_signal, _)| read_signal.get())
    }

    pub fn cleanup(&mut self) {
        self.signals.clear();
    }
}
