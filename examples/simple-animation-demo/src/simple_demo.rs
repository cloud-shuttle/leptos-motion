//! Simple CSS Transition Demo
//!
//! This demo shows the SimpleMotionDiv with CSS transitions working

use leptos::prelude::*;
use leptos_motion_dom::{SimpleMotionDiv, AnimationValue, Transition, Easing};
use std::collections::HashMap;

#[component]
pub fn SimpleCssTransitionDemo() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);
    
    // Create animation targets
    let initial = HashMap::from([
        ("x".to_string(), AnimationValue::Pixels(0.0)),
        ("y".to_string(), AnimationValue::Pixels(0.0)),
        ("opacity".to_string(), AnimationValue::Number(1.0)),
    ]);
    
    let animate = HashMap::from([
        ("x".to_string(), AnimationValue::Pixels(100.0)),
        ("y".to_string(), AnimationValue::Pixels(100.0)),
        ("opacity".to_string(), AnimationValue::Number(0.5)),
    ]);
    
    let hover_animate = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(1.2)),
        ("rotate".to_string(), AnimationValue::Degrees(45.0)),
        ("background-color".to_string(), AnimationValue::Color("red".to_string())),
    ]);

    let tap_animate = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(0.9)),
        ("border-radius".to_string(), AnimationValue::Pixels(20.0)),
    ]);

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        ..Default::default()
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"🎬 SimpleMotionDiv - CSS Transition Demo"</h1>
            <p>"This demo shows CSS transition-based animations working!"</p>
            
            <div style="margin: 20px 0;">
                <button 
                    on:click=move |_| set_is_animated.set(!is_animated.get())
                    style="padding: 10px 20px; font-size: 16px; cursor: pointer; background: #007bff; color: white; border: none; border-radius: 5px;"
                >
                    {move || if is_animated.get() { "Reset Animation" } else { "Start Animation" }}
                </button>
            </div>
            
            <div style="display: flex; gap: 20px; flex-wrap: wrap;">
                // Basic animated box with CSS transitions
                <SimpleMotionDiv
                    class="css-animated-box".to_string()
                    style="background: linear-gradient(45deg, #ff6b6b, #4ecdc4); width: 100px; height: 100px; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer;".to_string()
                    initial=initial.clone()
                    animate=if is_animated.get() { animate.clone() } else { HashMap::new() }
                    while_hover=hover_animate.clone()
                    while_tap=tap_animate.clone()
                    transition=transition.clone()
                >
                    "CSS Box 1"
                </SimpleMotionDiv>
                
                // Second animated box with different transition
                <SimpleMotionDiv
                    class="css-animated-box-2".to_string()
                    style="background: linear-gradient(45deg, #a8e6cf, #ffd3a5); width: 100px; height: 100px; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: #333; font-weight: bold; cursor: pointer;".to_string()
                    initial=HashMap::from([
                        ("x".to_string(), AnimationValue::Pixels(0.0)),
                        ("y".to_string(), AnimationValue::Pixels(0.0)),
                        ("opacity".to_string(), AnimationValue::Number(1.0)),
                    ])
                    animate=if is_animated.get() { 
                        HashMap::from([
                            ("x".to_string(), AnimationValue::Pixels(-50.0)),
                            ("y".to_string(), AnimationValue::Pixels(50.0)),
                            ("opacity".to_string(), AnimationValue::Number(0.8)),
                        ])
                    } else { HashMap::new() }
                    while_hover=HashMap::from([
                        ("scale".to_string(), AnimationValue::Number(0.8)),
                        ("rotate".to_string(), AnimationValue::Degrees(-30.0)),
                        ("background-color".to_string(), AnimationValue::Color("blue".to_string())),
                    ])
                    while_tap=HashMap::from([
                        ("scale".to_string(), AnimationValue::Number(1.1)),
                        ("border-radius".to_string(), AnimationValue::Pixels(50.0)),
                    ])
                    transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    "CSS Box 2"
                </SimpleMotionDiv>
                
                // Third box with spring-like transition
                <SimpleMotionDiv
                    class="css-animated-box-3".to_string()
                    style="background: linear-gradient(45deg, #ff9a9e, #fecfef); width: 100px; height: 100px; border-radius: 10px; display: flex; align-items: center; justify-content: center; color: #333; font-weight: bold; cursor: pointer;".to_string()
                    initial=HashMap::from([
                        ("x".to_string(), AnimationValue::Pixels(0.0)),
                        ("y".to_string(), AnimationValue::Pixels(0.0)),
                        ("opacity".to_string(), AnimationValue::Number(1.0)),
                    ])
                    animate=if is_animated.get() { 
                        HashMap::from([
                            ("x".to_string(), AnimationValue::Pixels(75.0)),
                            ("y".to_string(), AnimationValue::Pixels(-75.0)),
                            ("opacity".to_string(), AnimationValue::Number(0.9)),
                        ])
                    } else { HashMap::new() }
                    while_hover=HashMap::from([
                        ("scale".to_string(), AnimationValue::Number(1.1)),
                        ("rotate".to_string(), AnimationValue::Degrees(180.0)),
                        ("background-color".to_string(), AnimationValue::Color("green".to_string())),
                    ])
                    while_tap=HashMap::from([
                        ("scale".to_string(), AnimationValue::Number(0.95)),
                        ("border-radius".to_string(), AnimationValue::Pixels(0.0)),
                    ])
                    transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::BackOut,
                        ..Default::default()
                    }
                >
                    "CSS Box 3"
                </SimpleMotionDiv>
            </div>
            
            <div style="margin-top: 30px; padding: 20px; background: #f5f5f5; border-radius: 10px;">
                <h3>"🎯 What's Working with SimpleMotionDiv:"</h3>
                <ul>
                    <li>"✅ CSS transition-based animations (no RAF loops)"</li>
                    <li>"✅ Basic property animations (x, y, opacity)"</li>
                    <li>"✅ Hover animations (scale, rotate, background-color)"</li>
                    <li>"✅ Tap animations (scale, border-radius)"</li>
                    <li>"✅ Multiple animated elements with different transitions"</li>
                    <li>"✅ Smooth CSS transitions with easing functions"</li>
                </ul>
                
                <h3>"🔧 Technical Details:"</h3>
                <ul>
                    <li>"Uses native CSS transitions for optimal performance"</li>
                    <li>"No JavaScript animation loops - pure CSS"</li>
                    <li>"Easing functions: Linear, EaseIn, EaseOut, EaseInOut, BackOut"</li>
                    <li>"Memory efficient - no complex animation engine"</li>
                    <li>"GPU accelerated by the browser"</li>
                </ul>
            </div>
        </div>
    }
}
