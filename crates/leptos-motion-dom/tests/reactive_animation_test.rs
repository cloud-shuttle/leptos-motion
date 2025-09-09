use leptos::prelude::*;
use leptos_motion_dom::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn test_reactive_animation_with_closure_fails() {
    // This test demonstrates the current bug: closures don't track dependencies properly
    let (is_active, set_is_active) = create_signal(false);
    
    // Create a closure-based animation (this should fail to track dependencies)
    let animate_closure = move || {
        if is_active.get() {
            let mut target = HashMap::new();
            target.insert("transform".to_string(), AnimationValue::String("translateX(100px)".to_string()));
            target.insert("background-color".to_string(), AnimationValue::String("red".to_string()));
            target
        } else {
            let mut target = HashMap::new();
            target.insert("transform".to_string(), AnimationValue::String("translateX(0px)".to_string()));
            target.insert("background-color".to_string(), AnimationValue::String("blue".to_string()));
            target
        }
    };
    
    // Mount the component with the old closure-based API
    let app = view! {
        <div>
            <ReactiveMotionDiv animate=Some(reactive_animate(animate_closure)) />
            <button on:click=move |_| set_is_active.update(|v| *v = !*v)>"Toggle"</button>
        </div>
    };
    
    mount_to_body(app);
    
    // Initial state should be inactive (blue, translateX(0px))
    let motion_div = document().get_element_by_id("motion-div").unwrap();
    let initial_style = motion_div.get_attribute("style").unwrap();
    assert!(initial_style.contains("background-color: blue"));
    assert!(initial_style.contains("translateX(0px)"));
    
    // Click the button to activate
    let button = document().query_selector("button").unwrap().unwrap();
    let html_button = button.dyn_into::<web_sys::HtmlElement>().unwrap();
    html_button.click();
    
    // Wait for the animation to apply
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // This should fail with the current implementation because the closure
    // doesn't track the is_active signal properly
    let final_style = motion_div.get_attribute("style").unwrap();
    // The style should NOT change because the effect doesn't re-run
    assert!(final_style.contains("background-color: blue"));
    assert!(final_style.contains("translateX(0px)"));
}
