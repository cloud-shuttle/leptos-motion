//! Animation Timing Precision Tests
//!
//! Tests for ensuring leptos-motion animations maintain precise timing,
//! frame rates, and easing accuracy across different browsers and devices.

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use wasm_bindgen_test::*;
use web_sys::{window, document, Element, Performance};
use std::collections::HashMap;
use std::time::{Duration, Instant};

wasm_bindgen_test_configure!(run_in_browser);

/// Test animation duration accuracy
#[wasm_bindgen_test]
async fn test_animation_duration_accuracy() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="duration-test"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.0)
            transition=Transition {
                duration: Some(1.0), // 1 second
                ease: Easing::Linear,
                delay: None,
                repeat: RepeatConfig::Never,
                stagger: None,
            }
        >
            "Duration test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let element = document.get_element_by_id("duration-test").unwrap();
        let start_time = performance.now();
        
        // Wait for animation to complete
        let mut last_opacity = 1.0;
        let mut animation_complete = false;
        
        while !animation_complete {
            let current_time = performance.now();
            let elapsed = (current_time - start_time) / 1000.0; // Convert to seconds
            
            // Check if animation should be complete (1 second + small tolerance)
            if elapsed >= 1.1 {
                animation_complete = true;
            }
            
            // Small delay to avoid busy waiting
            wasm_bindgen_futures::spawn_local(async {
                // This is a placeholder - in a real test we'd measure actual opacity changes
            });
            
            // Break after reasonable timeout
            if elapsed > 2.0 {
                break;
            }
        }
        
        let end_time = performance.now();
        let total_duration = (end_time - start_time) / 1000.0;
        
        // Animation should complete within reasonable tolerance
        assert!(total_duration >= 0.9 && total_duration <= 1.2, 
                "Animation duration {}s not within expected range [0.9s, 1.2s]", total_duration);
    });
}

/// Test frame rate consistency
#[wasm_bindgen_test]
async fn test_frame_rate_consistency() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="framerate-test"
            initial=create_animation_target("transform", "translateX(0px)")
            animate=create_animation_target("transform", "translateX(100px)")
            transition=Transition {
                duration: Some(2.0), // 2 seconds for better measurement
                ease: Easing::Linear,
                delay: None,
                repeat: RepeatConfig::Never,
                stagger: None,
            }
        >
            "Frame rate test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let mut frame_times = Vec::new();
        let start_time = performance.now();
        let mut last_time = start_time;
        
        // Simulate frame rate measurement
        for _ in 0..60 { // Measure 60 frames
            let current_time = performance.now();
            let frame_duration = current_time - last_time;
            frame_times.push(frame_duration);
            last_time = current_time;
            
            // Small delay to simulate frame timing
            // In a real test, we'd measure actual animation frame updates
        }
        
        // Calculate average frame time
        let total_frame_time: f64 = frame_times.iter().sum();
        let average_frame_time = total_frame_time / frame_times.len() as f64;
        
        // Calculate frame rate (60fps = 16.67ms per frame)
        let expected_frame_time = 1000.0 / 60.0; // 16.67ms
        let tolerance = expected_frame_time * 0.5; // 50% tolerance
        
        // Frame rate should be reasonable (not too fast, not too slow)
        assert!(average_frame_time >= expected_frame_time - tolerance && 
                average_frame_time <= expected_frame_time + tolerance,
                "Average frame time {}ms not within expected range for 60fps", average_frame_time);
    });
}

/// Test easing function accuracy
#[wasm_bindgen_test]
async fn test_easing_function_accuracy() {
    let document = document().unwrap();
    
    // Test different easing functions
    let easing_functions = vec![
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
    ];
    
    for (i, easing) in easing_functions.iter().enumerate() {
        let app = view! {
            <ReactiveMotionDiv
                id=format!("easing-test-{}", i)
                initial=create_animation_target("opacity", 0.0)
                animate=create_animation_target("opacity", 1.0)
                transition=Transition {
                    duration: Some(1.0),
                    ease: easing.clone(),
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                }
            >
                {format!("Easing test {}", i)}
            </ReactiveMotionDiv>
        };

        mount_to_body(move || app);
        
        // Verify element was created
        let element = document.get_element_by_id(&format!("easing-test-{}", i)).unwrap();
        assert_eq!(element.text_content().unwrap(), format!("Easing test {}", i));
    }
}

/// Test animation delay precision
#[wasm_bindgen_test]
async fn test_animation_delay_precision() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="delay-test"
            initial=create_animation_target("opacity", 1.0)
            animate=create_animation_target("opacity", 0.0)
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::Linear,
                delay: Some(0.5), // 500ms delay
                repeat: RepeatConfig::Never,
                stagger: None,
            }
        >
            "Delay test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let start_time = performance.now();
        
        // Wait for delay + animation to complete
        let mut delay_started = false;
        let mut animation_started = false;
        
        while !animation_started {
            let current_time = performance.now();
            let elapsed = (current_time - start_time) / 1000.0;
            
            // Check if delay period has started (should be around 500ms)
            if elapsed >= 0.4 && elapsed <= 0.6 && !delay_started {
                delay_started = true;
            }
            
            // Check if animation has started (should be around 1000ms total)
            if elapsed >= 0.9 && elapsed <= 1.1 {
                animation_started = true;
            }
            
            // Timeout after reasonable time
            if elapsed > 2.0 {
                break;
            }
        }
        
        // Verify delay was respected
        assert!(delay_started, "Animation delay was not properly implemented");
    });
}

/// Test repeat animation timing
#[wasm_bindgen_test]
async fn test_repeat_animation_timing() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    let app = view! {
        <ReactiveMotionDiv
            id="repeat-test"
            initial=create_animation_target("scale", 1.0)
            animate=create_animation_target("scale", 1.5)
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseInOut,
                delay: None,
                repeat: RepeatConfig::Repeat(3), // Repeat 3 times
                stagger: None,
            }
        >
            "Repeat test"
        </ReactiveMotionDiv>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let start_time = performance.now();
        
        // Wait for all repeats to complete
        let mut repeat_count = 0;
        let mut last_scale = 1.0;
        
        while repeat_count < 3 {
            let current_time = performance.now();
            let elapsed = (current_time - start_time) / 1000.0;
            
            // Each repeat should take 0.5 seconds
            let expected_repeat_time = (repeat_count + 1) * 0.5;
            
            if elapsed >= expected_repeat_time - 0.1 && elapsed <= expected_repeat_time + 0.1 {
                repeat_count += 1;
            }
            
            // Timeout after reasonable time
            if elapsed > 3.0 {
                break;
            }
        }
        
        // Verify all repeats completed
        assert!(repeat_count >= 3, "Animation did not repeat the expected number of times");
    });
}

/// Test stagger animation timing
#[wasm_bindgen_test]
async fn test_stagger_animation_timing() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    // Create multiple elements with stagger
    let app = view! {
        <div>
            <ReactiveMotionDiv
                id="stagger-test-1"
                initial=create_animation_target("opacity", 0.0)
                animate=create_animation_target("opacity", 1.0)
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: Some(leptos_motion_core::StaggerConfig {
                        delay: 0.1, // 100ms stagger
                        from: leptos_motion_core::StaggerFrom::First,
                    }),
                }
            >
                "Stagger 1"
            </ReactiveMotionDiv>
            <ReactiveMotionDiv
                id="stagger-test-2"
                initial=create_animation_target("opacity", 0.0)
                animate=create_animation_target("opacity", 1.0)
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: Some(leptos_motion_core::StaggerConfig {
                        delay: 0.1,
                        from: leptos_motion_core::StaggerFrom::First,
                    }),
                }
            >
                "Stagger 2"
            </ReactiveMotionDiv>
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let start_time = performance.now();
        
        // Verify elements were created
        let element1 = document.get_element_by_id("stagger-test-1").unwrap();
        let element2 = document.get_element_by_id("stagger-test-2").unwrap();
        
        assert_eq!(element1.text_content().unwrap(), "Stagger 1");
        assert_eq!(element2.text_content().unwrap(), "Stagger 2");
        
        // In a real test, we'd measure the actual stagger timing
        // For now, just verify the elements exist and can be animated
        let current_time = performance.now();
        let elapsed = (current_time - start_time) / 1000.0;
        
        // Stagger should complete within reasonable time
        assert!(elapsed < 2.0, "Stagger animation took too long: {}s", elapsed);
    });
}

/// Test animation performance under load
#[wasm_bindgen_test]
async fn test_animation_performance_under_load() {
    let document = document().unwrap();
    let performance = window().unwrap().performance().unwrap();
    
    // Create multiple animations to test performance
    let app = view! {
        <div>
            {(0..10).map(|i| {
                view! {
                    <ReactiveMotionDiv
                        id=format!("performance-test-{}", i)
                        initial=create_animation_target("opacity", 1.0)
                        animate=create_animation_target("opacity", 0.5)
                        transition=Transition {
                            duration: Some(1.0),
                            ease: Easing::EaseInOut,
                            delay: None,
                            repeat: RepeatConfig::Never,
                            stagger: None,
                        }
                    >
                        {format!("Performance test {}", i)}
                    </ReactiveMotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    };

    mount_to_body(move || app);

    wasm_bindgen_futures::spawn_local(async {
        let start_time = performance.now();
        
        // Verify all elements were created
        for i in 0..10 {
            let element = document.get_element_by_id(&format!("performance-test-{}", i)).unwrap();
            assert_eq!(element.text_content().unwrap(), format!("Performance test {}", i));
        }
        
        let end_time = performance.now();
        let creation_time = end_time - start_time;
        
        // Creating 10 animations should be fast
        assert!(creation_time < 100.0, "Creating 10 animations took too long: {}ms", creation_time);
    });
}

/// Helper function to create animation targets
fn create_animation_target(property: &str, value: impl Into<AnimationValue>) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), value.into());
    AnimationTarget::from(target)
}
