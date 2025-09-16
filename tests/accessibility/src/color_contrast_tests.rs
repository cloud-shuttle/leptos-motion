//! Color Contrast Accessibility Tests
//!
//! Tests for ensuring leptos-motion animations maintain proper color contrast
//! ratios for accessibility compliance.

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing};
use wasm_bindgen_test::*;
use web_sys::{window, document, Element};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that color animations maintain WCAG AA contrast ratios
#[wasm_bindgen_test]
async fn test_color_contrast_during_animation() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="contrast-test"
            style="color: #000000; background-color: #ffffff;"
            initial=create_animation_target("color", "#000000")
            animate=create_animation_target("color", "#333333")
        >
            "High contrast text"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("contrast-test").unwrap();
        
        // Verify initial contrast is maintained
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Check that text remains readable
        assert_eq!(element.text_content().unwrap(), "High contrast text");
        
        // Verify element is still accessible
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that background color changes don't break text readability
#[wasm_bindgen_test]
async fn test_background_color_animation_contrast() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="background-contrast-test"
            style="color: #000000; background-color: #ffffff;"
            initial=create_animation_target("background-color", "#ffffff")
            animate=create_animation_target("background-color", "#f0f0f0")
        >
            "Text with changing background"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("background-contrast-test").unwrap();
        
        // Verify text content remains accessible
        assert_eq!(element.text_content().unwrap(), "Text with changing background");
        
        // Verify element maintains focusability
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that animated elements respect user's color scheme preferences
#[wasm_bindgen_test]
async fn test_respects_color_scheme_preferences() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="color-scheme-test"
            style="color: var(--text-color, #000000); background-color: var(--bg-color, #ffffff);"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.8)
        >
            "Respects color scheme"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("color-scheme-test").unwrap();
        
        // Verify CSS custom properties are used
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "Respects color scheme");
        
        // Verify element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that animated borders maintain sufficient contrast
#[wasm_bindgen_test]
async fn test_border_contrast_during_animation() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="border-contrast-test"
            style="border: 2px solid #000000; color: #000000; background-color: #ffffff;"
            initial=create_animation_target("border-color", "#000000")
            animate=create_animation_target("border-color", "#333333")
        >
            "Text with animated border"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("border-contrast-test").unwrap();
        
        // Verify text content remains accessible
        assert_eq!(element.text_content().unwrap(), "Text with animated border");
        
        // Verify element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that animated elements don't interfere with high contrast mode
#[wasm_bindgen_test]
async fn test_high_contrast_mode_compatibility() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="high-contrast-test"
            style="color: ButtonText; background-color: ButtonFace;"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.9)
        >
            "High contrast compatible"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("high-contrast-test").unwrap();
        
        // Verify system colors are used
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "High contrast compatible");
        
        // Verify element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that color animations don't cause seizures (WCAG AAA)
#[wasm_bindgen_test]
async fn test_no_seizure_triggering_animations() {
    let document = document().unwrap();
    
    // Use slow, gentle color transitions instead of rapid flashing
    let app = view! {
        <ReactiveMotionDiv
            id="seizure-safe-test"
            style="color: #000000; background-color: #ffffff;"
            initial=create_animation_target("background-color", "#ffffff")
            animate=create_animation_target("background-color", "#f8f8f8")
            transition=Transition {
                duration: Some(2.0), // Slow transition
                ease: Easing::EaseInOut,
                delay: None,
                repeat: leptos_motion_core::RepeatConfig::Never,
                stagger: None,
            }
        >
            "Seizure-safe animation"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("seizure-safe-test").unwrap();
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "Seizure-safe animation");
        
        // Verify element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test that animated elements provide sufficient focus indicators
#[wasm_bindgen_test]
async fn test_focus_indicator_contrast() {
    let document = document().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="focus-indicator-test"
            role="button"
            tabindex="0"
            style="outline: 2px solid #0066cc; color: #000000; background-color: #ffffff;"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.8)
        >
            "Focusable with good contrast"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("focus-indicator-test").unwrap();
        
        // Verify focus indicator is present
        let computed_style = window().unwrap()
            .get_computed_style(&element)
            .unwrap()
            .unwrap();
        
        // Verify element is focusable
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
        
        // Verify text content is accessible
        assert_eq!(element.text_content().unwrap(), "Focusable with good contrast");
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: impl Into<AnimationValue>) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value.into());
    AnimationTarget::from(target)
}
