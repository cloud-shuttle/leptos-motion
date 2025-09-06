//! TDD tests for error handling in production scenarios
//! 
//! These tests ensure that error handling works correctly in production environments

use crate::*;
use std::collections::HashMap;

// WASM-specific test configuration - conditional compilation
#[cfg(feature = "web-sys")]
use wasm_bindgen_test::*;

#[cfg(feature = "web-sys")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test that error context is properly created and contains all necessary information
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_error_context_creation() {
    let context = ErrorContext::new("test_operation")
        .with_component("MotionDiv")
        .with_info("property", "opacity")
        .with_info("value", "1.0");
    
    assert_eq!(context.operation, "test_operation");
    assert_eq!(context.component, Some("MotionDiv".to_string()));
    assert_eq!(context.additional_info.get("property"), Some(&"opacity".to_string()));
    assert_eq!(context.additional_info.get("value"), Some(&"1.0".to_string()));
}

/// Test that error recovery strategies are correctly determined for different error types
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_error_recovery_strategies() {
    let handler = DefaultErrorHandler::default();
    
    // Test engine unavailable error
    let engine_error = AnimationError::EngineUnavailable("WAAPI not supported".to_string());
    let context = ErrorContext::new("animate");
    let strategy = handler.handle_error(&engine_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test invalid property error
    let property_error = AnimationError::InvalidProperty {
        property: "invalid_prop".to_string(),
    };
    let strategy = handler.handle_error(&property_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test already running error
    let running_error = AnimationError::AlreadyRunning {
        handle: AnimationHandle(123),
    };
    let strategy = handler.handle_error(&running_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test not found error
    let not_found_error = AnimationError::NotFound {
        handle: AnimationHandle(456),
    };
    let strategy = handler.handle_error(&not_found_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);
    
    // Test DOM error
    let dom_error = AnimationError::DomError("Element not found".to_string());
    let strategy = handler.handle_error(&dom_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);
    
    // Test math error
    let math_error = AnimationError::MathError("Division by zero".to_string());
    let strategy = handler.handle_error(&math_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test performance budget exceeded
    let perf_error = AnimationError::PerformanceBudgetExceeded("Frame budget exceeded".to_string());
    let strategy = handler.handle_error(&perf_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test invalid config
    let config_error = AnimationError::InvalidConfig("Invalid duration".to_string());
    let strategy = handler.handle_error(&config_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test memory error
    let memory_error = AnimationError::MemoryError("Out of memory".to_string());
    let strategy = handler.handle_error(&memory_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);
    
    // Test timing error
    let timing_error = AnimationError::TimingError("Invalid timing".to_string());
    let strategy = handler.handle_error(&timing_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);
    
    // Test not implemented
    let not_impl_error = AnimationError::NotImplemented("Feature not available".to_string());
    let strategy = handler.handle_error(&not_impl_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);
    
    // Test invalid value
    let value_error = AnimationError::InvalidValue("NaN value".to_string());
    let strategy = handler.handle_error(&value_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
}

/// Test that user-friendly error messages are generated correctly
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_user_friendly_error_messages() {
    let mut handler = DefaultErrorHandler {
        log_errors: false,
        show_user_messages: true,
    };
    
    // Test engine unavailable message
    let engine_error = AnimationError::EngineUnavailable("WAAPI not supported".to_string());
    let message = handler.user_message(&engine_error);
    assert_eq!(message, "Animation system temporarily unavailable");
    
    // Test invalid property message
    let property_error = AnimationError::InvalidProperty {
        property: "invalid_prop".to_string(),
    };
    let message = handler.user_message(&property_error);
    assert_eq!(message, "Invalid animation property");
    
    // Test already running message
    let running_error = AnimationError::AlreadyRunning {
        handle: AnimationHandle(123),
    };
    let message = handler.user_message(&running_error);
    assert_eq!(message, "Animation already in progress");
    
    // Test not found message
    let not_found_error = AnimationError::NotFound {
        handle: AnimationHandle(456),
    };
    let message = handler.user_message(&not_found_error);
    assert_eq!(message, "Animation not found");
    
    // Test DOM error message
    let dom_error = AnimationError::DomError("Element not found".to_string());
    let message = handler.user_message(&dom_error);
    assert_eq!(message, "Animation display error");
    
    // Test math error message
    let math_error = AnimationError::MathError("Division by zero".to_string());
    let message = handler.user_message(&math_error);
    assert_eq!(message, "Animation calculation error");
    
    // Test performance budget exceeded message
    let perf_error = AnimationError::PerformanceBudgetExceeded("Frame budget exceeded".to_string());
    let message = handler.user_message(&perf_error);
    assert_eq!(message, "Animation performance limit reached");
    
    // Test invalid config message
    let config_error = AnimationError::InvalidConfig("Invalid duration".to_string());
    let message = handler.user_message(&config_error);
    assert_eq!(message, "Invalid animation configuration");
    
    // Test memory error message
    let memory_error = AnimationError::MemoryError("Out of memory".to_string());
    let message = handler.user_message(&memory_error);
    assert_eq!(message, "Animation memory error");
    
    // Test timing error message
    let timing_error = AnimationError::TimingError("Invalid timing".to_string());
    let message = handler.user_message(&timing_error);
    assert_eq!(message, "Animation timing error");
    
    // Test not implemented message
    let not_impl_error = AnimationError::NotImplemented("Feature not available".to_string());
    let message = handler.user_message(&not_impl_error);
    assert_eq!(message, "Feature not yet available");
    
    // Test invalid value message
    let value_error = AnimationError::InvalidValue("NaN value".to_string());
    let message = handler.user_message(&value_error);
    assert_eq!(message, "Invalid animation value");
}

/// Test that error handling works when user messages are disabled
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_error_handling_without_user_messages() {
    let handler = DefaultErrorHandler {
        log_errors: false,
        show_user_messages: false,
    };
    
    let engine_error = AnimationError::EngineUnavailable("WAAPI not supported".to_string());
    let message = handler.user_message(&engine_error);
    assert_eq!(message, "");
}

/// Test that error handling works with logging enabled
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_error_handling_with_logging() {
    let handler = DefaultErrorHandler {
        log_errors: true,
        show_user_messages: false,
    };
    
    let engine_error = AnimationError::EngineUnavailable("WAAPI not supported".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("property", "opacity");
    
    // This should not panic and should handle the error gracefully
    let strategy = handler.handle_error(&engine_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
}

/// Test that error handling works in production scenarios with invalid animation values
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_production_error_scenarios() {
    let handler = DefaultErrorHandler::default();
    
    // Test with NaN values
    let nan_error = AnimationError::InvalidValue("NaN value in opacity".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("property", "opacity")
        .with_info("value", "NaN");
    let strategy = handler.handle_error(&nan_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test with infinity values
    let inf_error = AnimationError::InvalidValue("Infinity value in scale".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("property", "scale")
        .with_info("value", "Infinity");
    let strategy = handler.handle_error(&inf_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test with negative duration
    let duration_error = AnimationError::InvalidConfig("Negative duration: -1.0".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("property", "duration")
        .with_info("value", "-1.0");
    let strategy = handler.handle_error(&duration_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test with missing element
    let dom_error = AnimationError::DomError("Element not found in DOM".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("element_id", "missing-element");
    let strategy = handler.handle_error(&dom_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);
}

/// Test that error handling works with complex animation scenarios
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_complex_animation_error_scenarios() {
    let handler = DefaultErrorHandler::default();
    
    // Test with multiple animations running
    let running_error = AnimationError::AlreadyRunning {
        handle: AnimationHandle(789),
    };
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("active_animations", "5")
        .with_info("max_animations", "10");
    let strategy = handler.handle_error(&running_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test with performance budget exceeded
    let perf_error = AnimationError::PerformanceBudgetExceeded("Frame budget exceeded: 20ms".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("frame_time", "20ms")
        .with_info("budget", "16ms");
    let strategy = handler.handle_error(&perf_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test with memory allocation failure
    let memory_error = AnimationError::MemoryError("Failed to allocate animation buffer".to_string());
    let context = ErrorContext::new("animate")
        .with_component("MotionDiv")
        .with_info("buffer_size", "1MB")
        .with_info("available_memory", "512KB");
    let strategy = handler.handle_error(&memory_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);
}

/// Test that error handling works with gesture system errors
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_gesture_system_error_scenarios() {
    let handler = DefaultErrorHandler::default();
    
    // Test with invalid gesture configuration
    let config_error = AnimationError::InvalidConfig("Invalid gesture threshold: -1.0".to_string());
    let context = ErrorContext::new("gesture_detect")
        .with_component("GestureSystem")
        .with_info("gesture_type", "drag")
        .with_info("threshold", "-1.0");
    let strategy = handler.handle_error(&config_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Fallback);
    
    // Test with gesture timing error
    let timing_error = AnimationError::TimingError("Gesture timeout exceeded".to_string());
    let context = ErrorContext::new("gesture_detect")
        .with_component("GestureSystem")
        .with_info("gesture_type", "pinch")
        .with_info("timeout", "5000ms");
    let strategy = handler.handle_error(&timing_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Retry);
}

/// Test that error handling works with layout animation errors
#[cfg_attr(feature = "web-sys", wasm_bindgen_test)]
#[cfg_attr(not(feature = "web-sys"), test)]
fn test_layout_animation_error_scenarios() {
    let handler = DefaultErrorHandler::default();
    
    // Test with layout calculation error
    let math_error = AnimationError::MathError("Layout calculation failed: division by zero".to_string());
    let context = ErrorContext::new("layout_animate")
        .with_component("LayoutAnimator")
        .with_info("element_id", "layout-element")
        .with_info("operation", "calculate_transform");
    let strategy = handler.handle_error(&math_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Skip);
    
    // Test with layout not implemented
    let not_impl_error = AnimationError::NotImplemented("Layout animation not supported in this browser".to_string());
    let context = ErrorContext::new("layout_animate")
        .with_component("LayoutAnimator")
        .with_info("browser", "Safari")
        .with_info("version", "14.0");
    let strategy = handler.handle_error(&not_impl_error, &context);
    assert_eq!(strategy, RecoveryStrategy::Abort);
}
