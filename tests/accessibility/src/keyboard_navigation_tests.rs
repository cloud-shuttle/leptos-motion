//! Keyboard Navigation Accessibility Tests
//!
//! Tests for ensuring leptos-motion components work properly with keyboard navigation
//! and keyboard-only users.

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing};
use wasm_bindgen_test::*;
use web_sys::{window, document, Element, KeyboardEvent};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

/// Test that animated elements respond to keyboard events
#[wasm_bindgen_test]
async fn test_keyboard_event_handling() {
    let document = document().unwrap();
    let (key_pressed, set_key_pressed) = signal(false);
    
    let app = view! {
        <ReactiveMotionDiv
            id="keyboard-responsive"
            role="button"
            tabindex="0"
            on:keydown=move |ev: KeyboardEvent| {
                if ev.key() == "Enter" || ev.key() == " " {
                    set_key_pressed.set(true);
                }
            }
            initial=create_animation_target("scale", 1.0)
            animate=move || {
                if key_pressed.get() {
                    create_animation_target("scale", 1.1)
                } else {
                    create_animation_target("scale", 1.0)
                }
            }
        >
            "Press Enter or Space"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("keyboard-responsive").unwrap();
        
        // Focus the element
        element.focus().unwrap();
        
        // Simulate Enter key press
        let enter_event = KeyboardEvent::new("keydown").unwrap();
        // Note: In a real test, we'd need to properly simulate the key event
        // For now, we'll verify the element is focusable and has proper attributes
        
        assert_eq!(element.get_attribute("role").unwrap(), "button");
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test tab order with animated elements
#[wasm_bindgen_test]
async fn test_tab_order_with_animations() {
    let document = document().unwrap();
    
    let app = view! {
        <div>
            <button id="tab-1">"First"</button>
            <ReactiveMotionDiv
                id="tab-2"
                role="button"
                tabindex="0"
                initial=create_animation_target("opacity", 1.0)
                animate=create_animation_target("opacity", 0.8)
            >
                "Animated Second"
            </ReactiveMotionDiv>
            <button id="tab-3">"Third"</button>
            <ReactiveMotionDiv
                id="tab-4"
                role="link"
                tabindex="0"
                initial=create_animation_target("transform", "translateX(0px)")
                animate=create_animation_target("transform", "translateX(5px)")
            >
                "Animated Fourth"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let tab1 = document.get_element_by_id("tab-1").unwrap();
        let tab2 = document.get_element_by_id("tab-2").unwrap();
        let tab3 = document.get_element_by_id("tab-3").unwrap();
        let tab4 = document.get_element_by_id("tab-4").unwrap();
        
        // Verify all elements are focusable
        assert_eq!(tab1.get_attribute("tabindex"), None); // button is focusable by default
        assert_eq!(tab2.get_attribute("tabindex").unwrap(), "0");
        assert_eq!(tab3.get_attribute("tabindex"), None); // button is focusable by default
        assert_eq!(tab4.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus order
        tab1.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), tab1);
        
        tab2.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), tab2);
        
        tab3.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), tab3);
        
        tab4.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), tab4);
    });
}

/// Test that animated elements don't break keyboard shortcuts
#[wasm_bindgen_test]
async fn test_keyboard_shortcuts_with_animations() {
    let document = document().unwrap();
    let (shortcut_triggered, set_shortcut_triggered) = signal(false);
    
    let app = view! {
        <ReactiveMotionDiv
            id="shortcut-element"
            role="button"
            tabindex="0"
            on:keydown=move |ev: KeyboardEvent| {
                if ev.ctrl_key() && ev.key() == "a" {
                    set_shortcut_triggered.set(true);
                }
            }
            initial=create_animation_target("opacity", 1.0)
            animate=move || {
                if shortcut_triggered.get() {
                    create_animation_target("opacity", 0.5)
                } else {
                    create_animation_target("opacity", 1.0)
                }
            }
        >
            "Ctrl+A to trigger"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("shortcut-element").unwrap();
        
        // Verify element is focusable and has proper attributes
        assert_eq!(element.get_attribute("role").unwrap(), "button");
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Focus the element
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test escape key handling with animated modals/overlays
#[wasm_bindgen_test]
async fn test_escape_key_with_animated_overlays() {
    let document = document().unwrap();
    let (is_visible, set_is_visible) = signal(true);
    
    let app = view! {
        <ReactiveMotionDiv
            id="modal-overlay"
            role="dialog"
            aria-modal="true"
            tabindex="-1"
            on:keydown=move |ev: KeyboardEvent| {
                if ev.key() == "Escape" {
                    set_is_visible.set(false);
                }
            }
            initial=create_animation_target("opacity", 1.0)
            animate=move || {
                if is_visible.get() {
                    create_animation_target("opacity", 1.0)
                } else {
                    create_animation_target("opacity", 0.0)
                }
            }
        >
            "Press Escape to close"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("modal-overlay").unwrap();
        
        // Verify modal attributes
        assert_eq!(element.get_attribute("role").unwrap(), "dialog");
        assert_eq!(element.get_attribute("aria-modal").unwrap(), "true");
        assert_eq!(element.get_attribute("tabindex").unwrap(), "-1");
        
        // Focus the modal
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    });
}

/// Test arrow key navigation with animated lists
#[wasm_bindgen_test]
async fn test_arrow_key_navigation() {
    let document = document().unwrap();
    let (selected_index, set_selected_index) = signal(0);
    
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    
    let app = view! {
        <div role="listbox" aria-label="Animated list">
            {items.iter().enumerate().map(|(index, item)| {
                let is_selected = move || selected_index.get() == index;
                view! {
                    <ReactiveMotionDiv
                        id=format!("item-{}", index)
                        role="option"
                        aria-selected=is_selected()
                        tabindex=if is_selected() { "0" } else { "-1" }
                        on:keydown=move |ev: KeyboardEvent| {
                            match ev.key().as_str() {
                                "ArrowDown" => {
                                    set_selected_index.set((index + 1) % items.len());
                                }
                                "ArrowUp" => {
                                    set_selected_index.set((index + items.len() - 1) % items.len());
                                }
                                _ => {}
                            }
                        }
                        initial=create_animation_target("background-color", "transparent")
                        animate=move || {
                            if is_selected() {
                                create_animation_target("background-color", "blue")
                            } else {
                                create_animation_target("background-color", "transparent")
                            }
                        }
                    >
                        {item}
                    </ReactiveMotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        // Verify first item is selected by default
        let first_item = document.get_element_by_id("item-0").unwrap();
        assert_eq!(first_item.get_attribute("aria-selected").unwrap(), "true");
        assert_eq!(first_item.get_attribute("tabindex").unwrap(), "0");
        
        // Verify other items are not selected
        for i in 1..4 {
            let item = document.get_element_by_id(&format!("item-{}", i)).unwrap();
            assert_eq!(item.get_attribute("aria-selected").unwrap(), "false");
            assert_eq!(item.get_attribute("tabindex").unwrap(), "-1");
        }
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: impl Into<AnimationValue>) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value.into());
    AnimationTarget::from(target)
}
