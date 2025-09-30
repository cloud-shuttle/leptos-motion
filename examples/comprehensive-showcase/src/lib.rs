//! Comprehensive Showcase - Professional motion library examples
//!
//! This demo showcases comprehensive examples like those you'd find 
//! in professional motion libraries (similar to Framer Motion's examples)

use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_dom::AnimateProp;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    // Skip logger initialization to prevent panics in browser
    // The demo will work without logging
}

#[wasm_bindgen]
pub fn run_app() {
    // Mount to the app div specifically
    web_sys::console::log_1(&"Starting Leptos app mount...".into());

    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    let app_element = document.get_element_by_id("app");
    match app_element {
        Some(element) => {
            web_sys::console::log_1(&"Found #app element, mounting...".into());
            let _ = leptos::mount::mount_to(
                element.dyn_into::<web_sys::HtmlElement>().unwrap(),
                || view! { <ShowcaseComponent /> }
            );
            web_sys::console::log_1(&"Mount completed".into());
        }
        None => {
            web_sys::console::log_1(&"ERROR: #app element not found!".into());
        }
    }
}

#[component]
fn ShowcaseComponent() -> impl IntoView {
    web_sys::console::log_1(&"ShowcaseComponent called!".into());
    let (button_scale, set_button_scale) = signal(1.0);
    let (card_x, set_card_x) = signal(0.0);
    let (loading_rotation, set_loading_rotation) = signal(0.0);
    let (message, set_message) = signal("Click the buttons to see animations!".to_string());

    // Create reactive animation states
    let button_animation = Memo::new(move |_| {
        HashMap::from([
            ("scale".to_string(), AnimationValue::Number(button_scale.get()))
        ])
    });

    let card_animation = Memo::new(move |_| {
        HashMap::from([
            ("x".to_string(), AnimationValue::Pixels(card_x.get()))
        ])
    });

    let loading_animation = Memo::new(move |_| {
        HashMap::from([
            ("rotate".to_string(), AnimationValue::Number(loading_rotation.get()))
        ])
    });

    let handle_button_click = move |_| {
        set_button_scale.update(|scale| *scale = if *scale == 1.0 { 1.5 } else { 1.0 });
        set_message.set("Button Animation: Scale effect!".to_string());
    };

    let handle_card_click = move |_| {
        set_card_x.update(|x| *x = if *x == 0.0 { 100.0 } else { 0.0 });
        set_message.set("Card Animation: Slide effect!".to_string());
    };

    let handle_loading_click = move |_| {
        set_loading_rotation.update(|rot| *rot += 360.0);
        set_message.set("Loading Animation: Rotation effect!".to_string());
    };

    view! {
        <div id="app" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; min-height: 100vh; font-family: system-ui;">
            <h1 style="font-size: 48px; text-align: center; margin-bottom: 20px;">
                "🎨 Leptos Motion"
            </h1>
            <p style="font-size: 24px; text-align: center; margin-bottom: 40px;">
                "Professional Motion Library Showcase"
            </p>
            <div style="display: flex; justify-content: center; gap: 20px; flex-wrap: wrap; min-height: 200px;">
                <MotionDiv
                    node_ref=NodeRef::new()
                    class="button".to_string()
                    style="background: #4CAF50; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer;".to_string()
                    initial=HashMap::from([
                        ("scale".to_string(), AnimationValue::Number(1.0))
                    ])
                    animate=AnimateProp::Derived(button_animation)
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                    on:click=handle_button_click
                >
                    "Button Animation"
                </MotionDiv>

                <MotionDiv
                    node_ref=NodeRef::new()
                    class="card".to_string()
                    style="background: #2196F3; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer; position: relative;".to_string()
                    initial=HashMap::from([
                        ("x".to_string(), AnimationValue::Pixels(0.0))
                    ])
                    animate=AnimateProp::Derived(card_animation)
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                    on:click=handle_card_click
                >
                    "Card Animation"
                </MotionDiv>

                <MotionDiv
                    node_ref=NodeRef::new()
                    class="loading".to_string()
                    style="background: #FF9800; color: white; border: none; padding: 15px 30px; font-size: 18px; border-radius: 8px; cursor: pointer;".to_string()
                    initial=HashMap::from([
                        ("rotate".to_string(), AnimationValue::Number(0.0))
                    ])
                    animate=AnimateProp::Derived(loading_animation)
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                    on:click=handle_loading_click
                >
                    "Loading Animation"
                </MotionDiv>
            </div>
            <div style="margin-top: 40px; text-align: center;">
                <p style="font-size: 18px; opacity: 0.8;">
                    {message}
                </p>
            </div>
        </div>
    }
}