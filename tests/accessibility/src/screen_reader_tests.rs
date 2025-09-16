//! Screen Reader Accessibility Tests
//!
//! Tests for ensuring leptos-motion components work properly with screen readers
//! and assistive technologies.

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing};
use wasm_bindgen_test::*;
use web_sys::{window, document, Element};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that animated elements maintain proper ARIA attributes during animation
#[wasm_bindgen_test]
async fn test_aria_attributes_during_animation() {
    let document = document().unwrap();
    
    // Create a motion div with ARIA attributes
    let app = view! {
        <ReactiveMotionDiv
            id="animated-element"
            role="button"
            aria-label="Animated button"
            aria-expanded="false"
            tabindex="0"
            initial=create_animation_target("opacity", AnimationValue::Number(1.0))
            animate=create_animation_target("opacity", AnimationValue::Number(0.5))
        >
            "Accessible Button"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    // Wait for animation to start
    wasm_bindgen_futures::spawn_local(async {
        // Check that ARIA attributes are preserved during animation
        let element = document.get_element_by_id("animated-element").unwrap();
        
        // Verify role is maintained
        assert_eq!(element.get_attribute("role").unwrap(), "button");
        
        // Verify aria-label is maintained
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Animated button");
        
        // Verify aria-expanded is maintained
        assert_eq!(element.get_attribute("aria-expanded").unwrap(), "false");
        
        // Verify tabindex is maintained
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
    });
}

/// Test that focus management works correctly with animated elements
#[wasm_bindgen_test]
async fn test_focus_management_during_animation() {
    let document = document().unwrap();
    
    let app = view! {
        <div>
            <button id="before-button">"Before"</button>
            <ReactiveMotionDiv
                id="animated-focusable"
                role="button"
                tabindex="0"
            initial=create_animation_target("opacity", AnimationValue::Number(1.0))
            animate=create_animation_target("opacity", AnimationValue::Number(0.8))
            >
                "Animated Focusable"
            </ReactiveMotionDiv>
            <button id="after-button">"After"</button>
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let animated_element = document.get_element_by_id("animated-focusable").unwrap();
        let before_button = document.get_element_by_id("before-button").unwrap();
        let after_button = document.get_element_by_id("after-button").unwrap();
        
        // Test that focus can be set on animated element
        animated_element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), animated_element);
        
        // Test tab navigation order
        before_button.focus().unwrap();
        // Simulate Tab key press
        let tab_event = web_sys::KeyboardEvent::new("keydown").unwrap();
        // Focus should move to animated element
        assert_eq!(document.active_element().unwrap(), animated_element);
    });
}

/// Test that screen reader announcements work with animated content
#[wasm_bindgen_test]
async fn test_screen_reader_announcements() {
    let document = document().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                id="announcement-element"
                role="status"
                aria-live="polite"
            initial=create_animation_target("opacity", AnimationValue::Number(0.0))
            animate=create_animation_target("opacity", AnimationValue::Number(1.0))
            >
                "Content loaded successfully"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("announcement-element").unwrap();
        
        // Verify ARIA live region attributes
        assert_eq!(element.get_attribute("role").unwrap(), "status");
        assert_eq!(element.get_attribute("aria-live").unwrap(), "polite");
        
        // Verify content is accessible to screen readers
        assert_eq!(element.text_content().unwrap(), "Content loaded successfully");
    });
}

/// Test that animated elements don't interfere with screen reader navigation
#[wasm_bindgen_test]
async fn test_screen_reader_navigation() {
    let document = document().unwrap();
    
    let app = view! {
        <div>
            <h1 id="heading-1">"Main Heading"</h1>
            <ReactiveMotionDiv
                id="animated-content"
                role="region"
                aria-labelledby="heading-1"
            initial=create_animation_target("transform", AnimationValue::String("translateY(0px)".to_string()))
            animate=create_animation_target("transform", AnimationValue::String("translateY(10px)".to_string()))
            >
                "Animated content section"
            </ReactiveMotionDiv>
            <h2 id="heading-2">"Sub Heading"</h2>
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let heading1 = document.get_element_by_id("heading-1").unwrap();
        let animated_content = document.get_element_by_id("animated-content").unwrap();
        let heading2 = document.get_element_by_id("heading-2").unwrap();
        
        // Verify heading structure is maintained
        assert_eq!(heading1.tag_name(), "H1");
        assert_eq!(heading2.tag_name(), "H2");
        
        // Verify region is properly labeled
        assert_eq!(animated_content.get_attribute("aria-labelledby").unwrap(), "heading-1");
        
        // Verify content is accessible
        assert_eq!(animated_content.text_content().unwrap(), "Animated content section");
    });
}

/// Test that color changes don't affect accessibility
#[wasm_bindgen_test]
async fn test_color_animation_accessibility() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="color-animated"
            style="color: black; background-color: white;"
            initial=create_animation_target("color", AnimationValue::Color("black".to_string()))
            animate=create_animation_target("color", AnimationValue::Color("blue".to_string()))
        >
            "Color changing text"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("color-animated").unwrap();
        
        // Verify text content remains accessible
        assert_eq!(element.text_content().unwrap(), "Color changing text");
        
        // Verify element is still focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: AnimationValue) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value);
    AnimationTarget::from(target)
}
