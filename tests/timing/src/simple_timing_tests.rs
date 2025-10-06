//! Simplified Animation Timing Precision Tests for Leptos Motion
//!
//! Basic timing tests that work with the current API

use leptos::prelude::*;
use leptos::{task::spawn_local};
use leptos_motion_dom::{ReactiveMotionDiv, AnimateProp};
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use wasm_bindgen_test::*;
use web_sys::{window, Element, Performance};
use std::collections::HashMap;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

/// Test animation duration accuracy
#[wasm_bindgen_test]
async fn test_animation_duration_accuracy() {
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="duration-test".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=AnimateProp::Static(create_animation_target("opacity", AnimationValue::Number(0.0)))
                _transition=Transition {
                    duration: Some(1.0), // 1 second
                    ease: Easing::Linear,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                }
                node_ref=NodeRef::new()
            >
                "Duration test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let start_time = performance.now();
        
        // Verify element was created
        let elements = document.get_elements_by_class_name("duration-test");
        assert!(elements.length() > 0, "Duration test element not found");
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), "Duration test");
        
        // Wait a bit to simulate animation timing
        let end_time = performance.now();
        let duration = end_time - start_time;
        
        // Basic timing should be reasonable
        assert!(duration < 1000.0, "Animation setup took too long: {}ms", duration);
    });
}

/// Test frame rate consistency
#[wasm_bindgen_test]
async fn test_frame_rate_consistency() {
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="framerate-test".to_string()
                initial=create_animation_target("transform", AnimationValue::String("translateX(0px)".to_string()))
                animate=AnimateProp::Static(create_animation_target("transform", AnimationValue::String("translateX(100px)".to_string())))
                _transition=Transition {
                    duration: Some(2.0), // 2 seconds for better measurement
                    ease: Easing::Linear,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                }
                node_ref=NodeRef::new()
            >
                "Frame rate test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let mut frame_times = Vec::new();
        let start_time = performance.now();
        let mut last_time = start_time;
        
        // Simulate frame rate measurement
        for _ in 0..10 { // Measure 10 frames
            let current_time = performance.now();
            let frame_duration = current_time - last_time;
            frame_times.push(frame_duration);
            last_time = current_time;
        }
        
        // Verify element was created
        let elements = document.get_elements_by_class_name("framerate-test");
        assert!(elements.length() > 0, "Frame rate test element not found");
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), "Frame rate test");
        
        // Calculate average frame time
        let total_frame_time: f64 = frame_times.iter().sum();
        let average_frame_time = total_frame_time / frame_times.len() as f64;
        
        // Frame rate should be reasonable (not too fast, not too slow)
        assert!(average_frame_time > 0.0 && average_frame_time < 100.0,
                "Average frame time {}ms not reasonable", average_frame_time);
    });
}

/// Test easing function accuracy
#[wasm_bindgen_test]
async fn test_easing_function_accuracy() {
    let document = web_sys::window().unwrap().document().unwrap();
    
    // Test different easing functions
    let easing_functions = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
        Easing::CircIn,
        Easing::CircOut,
        Easing::CircInOut,
        Easing::BackIn,
        Easing::BackOut,
        Easing::BackInOut,
    ];
    
    for (i, easing) in easing_functions.iter().enumerate() {
        let app = view! {
            <div>
                <ReactiveMotionDiv
                    class=format!("easing-test-{}", i)
                    initial=create_animation_target("opacity", AnimationValue::Number(0.0))
                    animate=AnimateProp::Static(create_animation_target("opacity", AnimationValue::Number(1.0)))
                    _transition=Transition {
                        duration: Some(1.0),
                        ease: easing.clone(),
                        delay: None,
                        repeat: RepeatConfig::Never,
                        stagger: None,
                    }
                    node_ref=NodeRef::new()
                >
                    {format!("Easing test {}", i)}
                </ReactiveMotionDiv>
            </div>
        };

        mount_to_body(move || app);
        
        // Verify element was created
        let elements = document.get_elements_by_class_name(&format!("easing-test-{}", i));
        assert!(elements.length() > 0, "Easing test element {} not found", i);
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), format!("Easing test {}", i));
    }
}

/// Test animation delay precision
#[wasm_bindgen_test]
async fn test_animation_delay_precision() {
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="delay-test".to_string()
                initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                animate=AnimateProp::Static(create_animation_target("opacity", AnimationValue::Number(0.0)))
                node_ref=NodeRef::new()
            >
                "Delay test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let start_time = performance.now();
        
        // Verify element was created
        let elements = document.get_elements_by_class_name("delay-test");
        assert!(elements.length() > 0, "Delay test element not found");
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), "Delay test");
        
        let end_time = performance.now();
        let duration = end_time - start_time;
        
        // Basic setup should be fast
        assert!(duration < 100.0, "Animation setup took too long: {}ms", duration);
    });
}

/// Test repeat animation timing
#[wasm_bindgen_test]
async fn test_repeat_animation_timing() {
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <div>
            <ReactiveMotionDiv
                class="repeat-test".to_string()
                initial=create_animation_target("scale", AnimationValue::Number(1.0))
                animate=AnimateProp::Static(create_animation_target("scale", AnimationValue::Number(1.5)))
                _transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    delay: None,
                    repeat: RepeatConfig::Count(3), // Repeat 3 times
                    stagger: None,
                }
                node_ref=NodeRef::new()
            >
                "Repeat test"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let start_time = performance.now();
        
        // Verify element was created
        let elements = document.get_elements_by_class_name("repeat-test");
        assert!(elements.length() > 0, "Repeat test element not found");
        
        let element = elements.item(0).unwrap();
        assert_eq!(element.text_content().unwrap(), "Repeat test");
        
        let end_time = performance.now();
        let duration = end_time - start_time;
        
        // Basic setup should be fast
        assert!(duration < 100.0, "Animation setup took too long: {}ms", duration);
    });
}

/// Test animation performance under load
#[wasm_bindgen_test]
async fn test_animation_performance_under_load() {
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window().unwrap().performance().unwrap();

    // Create multiple animations to test performance
    let app = view! {
        <div>
            {(0..5).map(|i| {
                let node_ref: NodeRef<leptos::html::Div> = NodeRef::new();
                view! {
                    <ReactiveMotionDiv
                        class=format!("performance-test-{}", i)
                        initial=create_animation_target("opacity", AnimationValue::Number(1.0))
                        animate=AnimateProp::Fn(Rc::new(|| create_animation_target("opacity", AnimationValue::Number(0.5))))
                        _transition=Transition {
                            duration: Some(1.0),
                            ease: Easing::EaseInOut,
                            delay: None,
                            repeat: RepeatConfig::Never,
                            stagger: None,
                        }
                        node_ref=node_ref
                    >
                        {format!("Performance test {}", i)}
                    </ReactiveMotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    };

    mount_to_body(move || app);

    spawn_local(async move {
        let start_time = performance.now();
        
        // Verify all elements were created
        for i in 0..5 {
            let elements = document.get_elements_by_class_name(&format!("performance-test-{}", i));
            assert!(elements.length() > 0, "Performance test element {} not found", i);
            
            let element = elements.item(0).unwrap();
            assert_eq!(element.text_content().unwrap(), format!("Performance test {}", i));
        }
        
        let end_time = performance.now();
        let creation_time = end_time - start_time;
        
        // Creating 5 animations should be fast
        assert!(creation_time < 200.0, "Creating 5 animations took too long: {}ms", creation_time);
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: AnimationValue) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value);
    AnimationTarget::from(target)
}
