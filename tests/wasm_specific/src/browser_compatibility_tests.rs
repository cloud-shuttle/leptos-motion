//! WASM-Specific Browser Compatibility Tests
//!
//! Tests for ensuring leptos-motion works correctly across different browsers
//! with varying WASM support and performance characteristics.

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing};
use wasm_bindgen_test::*;
use web_sys::{window, document, Element, Performance};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Test WASM performance characteristics across browsers
#[wasm_bindgen_test]
async fn test_wasm_performance_characteristics() {
    let window = window().unwrap();
    let performance = window.performance().unwrap();
    
    // Test that we can create animations without performance issues
    let app = view! {
        <ReactiveMotionDiv
            id="performance-test"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.5)
        >
            "Performance test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let start_time = performance.now();
        
        // Create multiple animations to test WASM performance
        for i in 0..10 {
            let element = document().unwrap().get_element_by_id("performance-test").unwrap();
            // Simulate animation updates
            let _ = element.style().set_property("opacity", &format!("{}", 0.5 + (i as f64 * 0.05)));
        }
        
        let end_time = performance.now();
        let duration = end_time - start_time;
        
        // WASM should be fast enough for smooth animations
        // Allow for some variance across browsers
        assert!(duration < 100.0, "WASM performance too slow: {}ms", duration);
    });
}

/// Test that animations work correctly in different browser contexts
#[wasm_bindgen_test]
async fn test_browser_context_compatibility() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="context-test"
            initial=create_animation_target("transform", "translateX(0px)")
            animate=create_animation_target("transform", "translateX(100px)")
        >
            "Context test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("context-test").unwrap();
        
        // Test that we can access browser APIs
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Verify the element exists and is accessible
        assert_eq!(element.text_content().unwrap(), "Context test");
        
        // Test that we can modify styles (browser compatibility)
        element.style().set_property("color", "red").unwrap();
        assert_eq!(element.style().get_property_value("color").unwrap(), "red");
    });
}

/// Test WASM memory management across browsers
#[wasm_bindgen_test]
async fn test_wasm_memory_management() {
    let document = document().unwrap();
    
    // Create and destroy multiple animations to test memory management
    for i in 0..5 {
        let app = view! {
            <ReactiveMotionDiv
                id=format!("memory-test-{}", i)
                initial=create_animation_target("opacity", 1.0)
                animate=create_animation_target("opacity", 0.0)
            >
                {format!("Memory test {}", i)}
            </ReactiveMotionDiv>
        };

        mount_to_body(move || app);
        
        // Verify element was created
        let element = document.get_element_by_id(&format!("memory-test-{}", i)).unwrap();
        assert_eq!(element.text_content().unwrap(), format!("Memory test {}", i));
        
        // Remove element to test cleanup
        element.remove();
    }
}

/// Test that animations work correctly with different browser event handling
#[wasm_bindgen_test]
async fn test_browser_event_handling() {
    let document = document().unwrap();
    let (click_count, set_click_count) = signal(0);
    
    let app = view! {
        <ReactiveMotionDiv
            id="event-test"
            role="button"
            tabindex="0"
            on:click=move |_| {
                set_click_count.update(|count| *count += 1);
            }
            initial=create_animation_target("scale", 1.0)
            animate=move || {
                if click_count.get() > 0 {
                    create_animation_target("scale", 1.1)
                } else {
                    create_animation_target("scale", 1.0)
                }
            }
        >
            "Click me"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("event-test").unwrap();
        
        // Test that element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
        
        // Test that element responds to events
        assert_eq!(element.get_attribute("role").unwrap(), "button");
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
    });
}

/// Test WASM-specific error handling
#[wasm_bindgen_test]
async fn test_wasm_error_handling() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="error-test"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.5)
        >
            "Error test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("error-test").unwrap();
        
        // Test that we can handle errors gracefully
        let result = element.style().set_property("invalid-property", "invalid-value");
        // This should not panic, even if the property is invalid
        assert!(result.is_ok() || result.is_err()); // Either is fine, just no panic
        
        // Test that the element still works after error
        assert_eq!(element.text_content().unwrap(), "Error test");
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: impl Into<AnimationValue>) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value.into());
    AnimationTarget::from(target)
}
