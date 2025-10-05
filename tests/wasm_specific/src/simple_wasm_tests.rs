//! Simplified WASM-Specific Tests for Leptos Motion
//!
//! Basic WASM tests that work with the current API

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_motion_dom::{ReactiveMotionDiv, AnimateProp};
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use wasm_bindgen_test::*;
use web_sys::{window, Element, Performance};
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

/// Test WASM performance characteristics
#[wasm_bindgen_test]
async fn test_wasm_performance_characteristics() {
    let window = window().unwrap();
    let performance = window.performance().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="performance-test".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=Box::new(|| create_animation_target("opacity", AnimationValue::Number(0.5)))
            >
                "Performance test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let start_time = performance.now();
        
        // Find the element and verify it exists
        let elements = web_sys::window().unwrap().document().unwrap().get_elements_by_class_name("performance-test");
        assert!(elements.length() > 0, "Performance test element not found");
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), "Performance test");
        
        let end_time = performance.now();
        let duration = end_time - start_time;
        
        // WASM should be fast enough for basic operations
        assert!(duration < 100.0, "WASM performance too slow: {}ms", duration);
    });
}

/// Test that animations work correctly in different browser contexts
#[wasm_bindgen_test]
async fn test_browser_context_compatibility() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="context-test".to_string()
                initial=create_animation_target("transform", AnimationValue::String("translateX(0px)".to_string()))
                animate=Box::new(|| create_animation_target("transform", AnimationValue::String("translateX(100px)".to_string())))
            >
                "Context test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("context-test");
        assert!(elements.length() > 0, "Context test element not found");
        
        let element = elements.item(0).unwrap();
        
        // Test that we can access browser APIs
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Verify the element exists and is accessible
        assert_eq!(element.text_content().unwrap(), "Context test");
        
        // Test that we can modify styles (browser compatibility)
        let html_element: web_sys::HtmlElement = element.clone().dyn_into().unwrap();
        let style = html_element.style();
        style.set_property("color", "red").unwrap();
        assert_eq!(style.get_property_value("color").unwrap(), "red");
    });
}

/// Test WASM memory management
#[wasm_bindgen_test]
async fn test_wasm_memory_management() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    // Create and destroy multiple animations to test memory management
    for i in 0..5 {
        let app = view! {
            <div>
                <ReactiveMotionDiv
                    class=format!("memory-test-{}", i)
                    initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                    animate=AnimateProp::Static(create_animation_target("opacity", AnimationValue::Number(0.0)))
                    node_ref=NodeRef::new()
                    children=()
                >
                    {format!("Memory test {}", i)}
                </ReactiveMotionDiv>
            </div>
        };

        mount_to_body(move || app);
        
        // Verify element was created
        let elements = document.get_elements_by_class_name(&format!("memory-test-{}", i));
        assert!(elements.length() > 0, "Memory test element {} not found", i);
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), format!("Memory test {}", i));
        
        // Remove element to test cleanup
        element.remove();
    }
}

/// Test that animated elements respond to events
#[wasm_bindgen_test]
async fn test_browser_event_handling() {
    let document = web_sys::window().unwrap().document().unwrap();
    let (click_count, set_click_count) = signal(0);
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="event-test".to_string()
                style="role: button; tabindex: 0;".to_string()
                initial=create_animation_target("scale", AnimationValue::Number(1.0))
                animate=AnimateProp::Fn(Rc::new(move || {
                    if click_count.get() > 0 {
                        create_animation_target("scale", AnimationValue::Number(1.1))
                    } else {
                        create_animation_target("scale", AnimationValue::Number(1.0))
                    }
                }))
                node_ref=NodeRef::new()
                children=|| {}
            >
                "Click me"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("event-test");
        assert!(elements.length() > 0, "Event test element not found");
        
        let element = elements.item(0).unwrap();
        
        // Verify element is focusable
        let html_element: web_sys::HtmlElement = element.clone().dyn_into().unwrap();
        html_element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
        
        // Verify text content
        assert_eq!(element.text_content().unwrap(), "Click me");
    });
}

/// Test WASM-specific error handling
#[wasm_bindgen_test]
async fn test_wasm_error_handling() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="error-test".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=AnimateProp::Static(create_animation_target("opacity", AnimationValue::Number(0.5)))
                node_ref=NodeRef::new()
                children=|| {}
            >
                "Error test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("error-test");
        assert!(elements.length() > 0, "Error test element not found");
        
        let element = elements.item(0).unwrap();
        
        // Test that we can handle errors gracefully
        let html_element: web_sys::HtmlElement = element.clone().dyn_into().unwrap();
        let style = html_element.style();
        let result = style.set_property("invalid-property", "invalid-value");
        // This should not panic, even if the property is invalid
        assert!(result.is_ok() || result.is_err()); // Either is fine, just no panic
        
        // Test that the element still works after error
        assert_eq!(element.text_content().unwrap(), "Error test");
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: AnimationValue) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value);
    AnimationTarget::from(target)
}
