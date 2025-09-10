//! Simple Working Demo using proven WASM + signals patterns
//!
//! This demo follows the exact patterns from the user's guide to ensure
//! no browser hanging issues occur.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
pub fn SimpleWorkingDemo() -> impl IntoView {
    web_sys::console::log_1(&"🎯 SimpleWorkingDemo: Component created".into());
    
    // ✅ Create reactive signals for animation state
    let (is_animated, set_animated) = signal(false);
    let (x_pos, set_x_pos) = signal(0.0);
    let (y_pos, set_y_pos) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    
    // ✅ Create reactive animation target using signals
    let animate_target = move || {
        let mut target = HashMap::new();
        if is_animated.get() {
            target.insert("x".to_string(), AnimationValue::Pixels(100.0));
            target.insert("y".to_string(), AnimationValue::Pixels(50.0));
            target.insert("rotateZ".to_string(), AnimationValue::Degrees(45.0));
            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        } else {
            target.insert("x".to_string(), AnimationValue::Pixels(0.0));
            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
            target.insert("opacity".to_string(), AnimationValue::Number(0.7));
        }
        target
    };
    
    // ✅ Create initial animation target
    let initial_target = {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(0.0));
        target.insert("y".to_string(), AnimationValue::Pixels(0.0));
        target.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        target.insert("opacity".to_string(), AnimationValue::Number(0.7));
        target
    };
    
    // ✅ Animation controls
    let animate = move |_| {
        set_animated.set(true);
    };
    
    let reset = move |_| {
        set_animated.set(false);
    };
    
    let move_x = move |_| {
        set_x_pos.set(x_pos.get() + 20.0);
    };
    
    let move_y = move |_| {
        set_y_pos.set(y_pos.get() + 20.0);
    };
    
    let rotate = move |_| {
        set_rotation.set(rotation.get() + 30.0);
    };
    
    view! {
        <div style="padding: 2rem; max-width: 800px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 2rem;">
                <h1 style="font-size: 2.5rem; margin-bottom: 1rem; color: #333;">
                    "🚀 Leptos Motion - Working Demo"
                </h1>
                <p style="color: #666; margin-bottom: 1rem;">
                    "Simple, working animations using proven WASM + signals patterns"
                </p>
            </header>

            <main>
                <section style="margin-bottom: 3rem; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
                    <h2 style="margin-bottom: 1rem; color: #333;">"Basic Animation Test"</h2>
                    <p style="color: #666; margin-bottom: 1rem;">
                        "This tests basic animation functionality without complex patterns."
                    </p>
                    
                    <div style="display: flex; gap: 1rem; margin-bottom: 2rem;">
                        <button
                            on:click=animate
                            style="padding: 0.5rem 1rem; background: #4ecdc4; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Animate"
                        </button>
                        <button
                            on:click=reset
                            style="padding: 0.5rem 1rem; background: #ff6b6b; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Reset"
                        </button>
                    </div>
                    
                    <div style="display: flex; justify-content: center; align-items: center; min-height: 200px; position: relative;">
                        <MotionDiv
                            initial=initial_target.clone()
                            animate=animate_target()
                            _transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseInOut,
                                repeat: RepeatConfig::Never,
                                delay: Some(0.0),
                                stagger: None,
                            }
                            style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                        >
                            "Motion"
                        </MotionDiv>
                    </div>
                    
                    <div style="margin-top: 1rem; font-size: 0.9rem; color: #666;">
                        <p>"Current state: {move || if is_animated.get() { "Animated" } else { "Static" }}"</p>
                    </div>
                </section>
                
                <section style="margin-bottom: 3rem; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
                    <h2 style="margin-bottom: 1rem; color: #333;">"Interactive Controls"</h2>
                    <p style="color: #666; margin-bottom: 1rem;">
                        "Test individual transform controls to ensure reactivity works."
                    </p>
                    
                    <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
                        <button
                            on:click=move_x
                            style="padding: 0.5rem 1rem; background: #45b7d1; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Move X"
                        </button>
                        <button
                            on:click=move_y
                            style="padding: 0.5rem 1rem; background: #96ceb4; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Move Y"
                        </button>
                        <button
                            on:click=rotate
                            style="padding: 0.5rem 1rem; background: #feca57; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            "Rotate"
                        </button>
                    </div>
                    
                    <div style="display: flex; justify-content: center; align-items: center; min-height: 150px; position: relative;">
                        <MotionDiv
                            initial=initial_target.clone()
                            animate={
                                let mut target = HashMap::new();
                                target.insert("x".to_string(), AnimationValue::Pixels(x_pos.get()));
                                target.insert("y".to_string(), AnimationValue::Pixels(y_pos.get()));
                                target.insert("rotateZ".to_string(), AnimationValue::Degrees(rotation.get()));
                                target.insert("opacity".to_string(), AnimationValue::Number(0.8));
                                target
                            }
                            _transition=Transition {
                                duration: Some(0.3),
                                ease: Easing::EaseOut,
                                repeat: RepeatConfig::Never,
                                delay: Some(0.0),
                                stagger: None,
                            }
                            style="width: 80px; height: 80px; background: linear-gradient(45deg, #45b7d1, #96ceb4); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                        >
                            "Control"
                        </MotionDiv>
                    </div>
                    
                    <div style="margin-top: 1rem; font-size: 0.9rem; color: #666;">
                        <p>{move || format!("X: {:.1}px", x_pos.get())}</p>
                        <p>{move || format!("Y: {:.1}px", y_pos.get())}</p>
                        <p>{move || format!("Rotation: {:.1}°", rotation.get())}</p>
                    </div>
                </section>
                
                <section style="margin-bottom: 3rem; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
                    <h2 style="margin-bottom: 1rem; color: #333;">"Multiple Elements"</h2>
                    <p style="color: #666; margin-bottom: 1rem;">
                        "Test multiple animated elements to ensure no conflicts."
                    </p>
                    
                    <div style="display: flex; gap: 2rem; justify-content: center; align-items: center; min-height: 150px;">
                        {move || (0..3).map(|i| {
                            let delay = i as f64 * 0.2;
                            view! {
                                <MotionDiv
                                    initial={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(0.5));
                                        target
                                    }
                                    animate={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                        target
                                    }
                                    _transition=Transition {
                                        duration: Some(0.8),
                                        ease: Easing::EaseOut,
                                        repeat: RepeatConfig::Never,
                                        delay: Some(delay),
                                        stagger: None,
                                    }
                                    style="width: 60px; height: 60px; background: linear-gradient(45deg, #667eea, #764ba2); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string()
                                >
                                    {format!("{}", i + 1)}
                                </MotionDiv>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>
            </main>
        </div>
    }
}
