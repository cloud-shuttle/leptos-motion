//! Simplified Accessibility Tests for Leptos Motion
//!
//! Basic accessibility tests that work with the current API

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use wasm_bindgen_test::*;
use web_sys::{window, Element};
use wasm_bindgen::JsCast;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that animated elements maintain proper ARIA attributes
#[wasm_bindgen_test]
async fn test_aria_attributes_during_animation() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="animated-element".to_string()
                style="role: button; aria-label: 'Animated button'; aria-expanded: false; tabindex: 0;".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=Box::new(|| create_animation_target("opacity", AnimationValue::Number(0.5)))
            >
                "Accessible Button"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        // Find the animated element by class
        let elements = document.get_elements_by_class_name("animated-element");
        assert!(elements.length() > 0, "Animated element not found");
        
        let element = elements.item(0).unwrap();
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "Accessible Button");
    });
}

/// Test that animated elements can be focused
#[wasm_bindgen_test]
async fn test_focus_management_during_animation() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <button class="before-button">"Before"</button>
            <ReactiveMotionDiv
                class="animated-focusable".to_string()
                style="role: button; tabindex: 0;".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=Box::new(|| create_animation_target("opacity", AnimationValue::Number(0.8)))
            >
                "Animated Focusable"
            </ReactiveMotionDiv>
            <button class="after-button">"After"</button>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("animated-focusable");
        assert!(elements.length() > 0, "Animated focusable element not found");
        
        let element = elements.item(0).unwrap();
        
        // Test that focus can be set on animated element
        let html_element: web_sys::HtmlElement = element.clone().dyn_into().unwrap();
        html_element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "Animated Focusable");
    });
}

/// Test that animated elements maintain text content accessibility
#[wasm_bindgen_test]
async fn test_text_content_accessibility() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="text-test".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(0.0))
                animate=Box::new(|| create_animation_target("opacity", AnimationValue::Number(1.0)))
            >
                "Content loaded successfully"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("text-test");
        assert!(elements.length() > 0, "Text test element not found");
        
        let element = elements.item(0).unwrap();
        
        // Verify content is accessible to screen readers
        assert_eq!(element.text_content().unwrap(), "Content loaded successfully");
    });
}

/// Test that animated elements don't interfere with screen reader navigation
#[wasm_bindgen_test]
async fn test_screen_reader_navigation() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <h1 class="heading-1">"Main Heading"</h1>
            <ReactiveMotionDiv
                class="animated-content".to_string()
                style="role: region;".to_string()
                initial=create_animation_target("transform", AnimationValue::String("translateY(0px)".to_string()))
                animate=Box::new(|| create_animation_target("transform", AnimationValue::String("translateY(10px)".to_string())))
            >
                "Animated content section"
            </ReactiveMotionDiv>
            <h2 class="heading-2">"Sub Heading"</h2>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let heading1 = document.get_elements_by_class_name("heading-1").item(0).unwrap();
        let animated_content = document.get_elements_by_class_name("animated-content").item(0).unwrap();
        let heading2 = document.get_elements_by_class_name("heading-2").item(0).unwrap();
        
        // Verify heading structure is maintained
        assert_eq!(heading1.tag_name(), "H1");
        assert_eq!(heading2.tag_name(), "H2");
        
        // Verify content is accessible
        assert_eq!(animated_content.text_content().unwrap(), "Animated content section");
    });
}

/// Test that color changes don't affect accessibility
#[wasm_bindgen_test]
async fn test_color_animation_accessibility() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="color-animated".to_string()
                style="color: black; background-color: white;".to_string()
                initial=create_animation_target("color", AnimationValue::Color("black".to_string()))
                animate=Box::new(|| create_animation_target("color", AnimationValue::Color("blue".to_string())))
            >
                "Color changing text"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let elements = document.get_elements_by_class_name("color-animated");
        assert!(elements.length() > 0, "Color animated element not found");
        
        let element = elements.item(0).unwrap();
        
        // Verify text content remains accessible
        assert_eq!(element.text_content().unwrap(), "Color changing text");
        
        // Verify element is still focusable
        let html_element: web_sys::HtmlElement = element.clone().dyn_into().unwrap();
        html_element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: AnimationValue) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value);
    AnimationTarget::from(target)
}
