//! Demo Showcase Component
//!
//! A component that demonstrates various animation capabilities in a clean, organized way.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use super::animated_box::AnimatedBox;
use super::animated_button::AnimatedButton;

/// Demo showcase component
#[component]
pub fn DemoShowcase() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);

    // Animation targets for the demo boxes
    let initial = HashMap::from([
        ("x".to_string(), AnimationValue::Pixels(0.0)),
        ("y".to_string(), AnimationValue::Pixels(0.0)),
        ("opacity".to_string(), AnimationValue::Number(1.0)),
        ("scale".to_string(), AnimationValue::Number(1.0)),
    ]);

    let animate = HashMap::from([
        ("x".to_string(), AnimationValue::Pixels(200.0)),
        ("y".to_string(), AnimationValue::Pixels(100.0)),
        ("opacity".to_string(), AnimationValue::Number(0.7)),
        ("scale".to_string(), AnimationValue::Number(1.2)),
    ]);

    let while_hover = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(1.1)),
        ("rotate".to_string(), AnimationValue::Degrees(5.0)),
    ]);

    let while_tap = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(0.9)),
    ]);

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh;">
            <div style="max-width: 1200px; margin: 0 auto;">
                <h1 style="color: white; text-align: center; margin-bottom: 30px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                    "🎬 Leptos Motion - Small Components Demo"
                </h1>
                
                <div style="background: white; border-radius: 15px; padding: 20px; margin-bottom: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h2 style="color: #333; margin-bottom: 20px;">"Animation Controls"</h2>
                    
                    <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                        <AnimatedButton
                            text=if is_animated.get() { Some("Reset Animation".to_string()) } else { Some("Start Animation".to_string()) }
                            on_click=Some(Box::new(move || set_is_animated.set(!is_animated.get())))
                            variant=super::animated_button::ButtonVariant::Primary
                        />
                        
                        <AnimatedButton
                            text=Some("Success Button".to_string())
                            variant=super::animated_button::ButtonVariant::Success
                        />
                        
                        <AnimatedButton
                            text=Some("Danger Button".to_string())
                            variant=super::animated_button::ButtonVariant::Danger
                        />
                    </div>
                </div>

                <div style="background: white; border-radius: 15px; padding: 20px; margin-bottom: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h3 style="color: #333; margin-bottom: 15px;">"🚀 AnimatedBox Components"</h3>
                    <p style="color: #666; margin-bottom: 20px;">"These use the simple AnimatedBox component with CSS transitions."</p>
                    
                    <div style="display: flex; gap: 20px; flex-wrap: wrap; justify-content: center;">
                        <AnimatedBox
                            style=Some("width: 120px; height: 120px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer; box-shadow: 0 4px 8px rgba(0,0,0,0.1);".to_string())
                            initial=initial.clone()
                            animate=if is_animated.get() { animate.clone() } else { HashMap::new() }
                            while_hover=while_hover.clone()
                            while_tap=while_tap.clone()
                            duration=0.5
                            easing=Easing::EaseInOut
                        >
                            "Box 1"
                        </AnimatedBox>

                        <AnimatedBox
                            style=Some("width: 120px; height: 120px; background: linear-gradient(45deg, #a8e6cf, #dcedc1); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: #333; font-weight: bold; cursor: pointer; box-shadow: 0 4px 8px rgba(0,0,0,0.1);".to_string())
                            initial=Some(HashMap::from([
                                ("x".to_string(), AnimationValue::Pixels(0.0)),
                                ("y".to_string(), AnimationValue::Pixels(0.0)),
                                ("rotate".to_string(), AnimationValue::Degrees(0.0)),
                            ]))
                            animate=if is_animated.get() {
                                HashMap::from([
                                    ("x".to_string(), AnimationValue::Pixels(-100.0)),
                                    ("y".to_string(), AnimationValue::Pixels(80.0)),
                                    ("rotate".to_string(), AnimationValue::Degrees(180.0)),
                                ])
                            } else { HashMap::new() }
                            while_hover=Some(HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(1.1)),
                                ("rotate".to_string(), AnimationValue::Degrees(-15.0)),
                            ]))
                            duration=0.8
                            easing=Easing::BackOut
                        >
                            "Box 2"
                        </AnimatedBox>

                        <AnimatedBox
                            style=Some("width: 120px; height: 120px; background: linear-gradient(45deg, #ffd89b, #19547b); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: pointer; box-shadow: 0 4px 8px rgba(0,0,0,0.1);".to_string())
                            initial=Some(HashMap::from([
                                ("border-radius".to_string(), AnimationValue::Pixels(8.0)),
                                ("background-color".to_string(), AnimationValue::String("linear-gradient(45deg, #ffd89b, #19547b)".to_string())),
                            ]))
                            animate=if is_animated.get() {
                                HashMap::from([
                                    ("border-radius".to_string(), AnimationValue::Pixels(50.0)),
                                    ("background-color".to_string(), AnimationValue::String("linear-gradient(45deg, #19547b, #ffd89b)".to_string())),
                                ])
                            } else { HashMap::new() }
                            while_hover=Some(HashMap::from([
                                ("scale".to_string(), AnimationValue::Number(1.05)),
                                ("rotate".to_string(), AnimationValue::Degrees(-10.0)),
                            ]))
                            duration=0.6
                            easing=Easing::EaseInOut
                        >
                            "Box 3"
                        </AnimatedBox>
                    </div>
                </div>

                <div style="background: white; border-radius: 15px; padding: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <h3 style="color: #333; margin-bottom: 15px;">"🎯 What's Working:"</h3>
                    <ul style="color: #666; line-height: 1.6;">
                        <li>"✅ Small, focused components (AnimatedBox, AnimatedButton)"</li>
                        <li>"✅ CSS transition-based animations"</li>
                        <li>"✅ Hover and tap interaction animations"</li>
                        <li>"✅ Configurable duration and easing functions"</li>
                        <li>"✅ Easy to test and understand"</li>
                        <li>"✅ Composable and reusable"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}