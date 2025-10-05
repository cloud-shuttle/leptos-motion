use leptos::prelude::*;
use leptos_motion_dom::{MotionDiv, AnimateProp, AnimationValue};
use std::collections::HashMap;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (is_animated, set_animated) = signal(false);
    let (scale, set_scale) = signal(1.0);
    let (opacity, set_opacity) = signal(0.5);

    // Create reactive animation values
    let animate_values = move || {
        let mut values = HashMap::new();
        
        // Always use the current signal values
        values.insert("opacity".to_string(), AnimationValue::Number(opacity.get()));
        values.insert("scale".to_string(), AnimationValue::Number(scale.get()));
        
        // Add rotation when animated
        if is_animated.get() {
            values.insert("rotateZ".to_string(), AnimationValue::Degrees(360.0));
        } else {
            values.insert("rotateZ".to_string(), AnimationValue::Degrees(0.0));
        }
        
        // Debug: log the values
        web_sys::console::log_1(&format!("Animation values: {:?}", values).into());
        
        values
    };

    // Create reactive animation prop
    let animate_prop = AnimateProp::Fn(std::rc::Rc::new(animate_values));
    
    // Create node reference
    let node_ref = NodeRef::new();

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Leptos Motion Reactive Animation Demo"</h1>
            
            <div style="margin: 20px 0;">
                <button 
                    on:click=move |_| set_animated.update(|v| *v = !*v)
                    style="padding: 10px 20px; margin-right: 10px;"
                >
                    "Toggle Rotation"
                </button>
                
                <button 
                    on:click=move |_| set_scale.update(|v| *v = if *v == 1.0 { 1.5 } else { 1.0 })
                    style="padding: 10px 20px; margin-right: 10px;"
                >
                    "Toggle Scale"
                </button>
                
                <button 
                    on:click=move |_| set_opacity.update(|v| *v = if *v == 0.5 { 1.0 } else { 0.5 })
                    style="padding: 10px 20px;"
                >
                    "Toggle Opacity"
                </button>
            </div>

            <div style="margin: 20px 0;">
                <p>"Current state: " {move || if is_animated.get() { "Animated" } else { "Static" }}</p>
                <p>"Scale: " {move || scale.get()}</p>
                <p>"Opacity: " {move || opacity.get()}</p>
            </div>

            <MotionDiv
                animate=animate_prop
                node_ref=node_ref
                style="
                    width: 200px;
                    height: 200px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                    border-radius: 10px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    font-size: 18px;
                    margin: 20px auto;
                    transition: all 0.6s ease-in-out;
                    transform-origin: center center;
                ".to_string()
            >
                "Reactive Animation!"
            </MotionDiv>

            <div style="margin: 20px 0; padding: 20px; background: #f5f5f5; border-radius: 8px;">
                <h3>"Instructions:"</h3>
                <ul>
                    <li>"Click 'Toggle Rotation' to animate rotation (360° or 0°)"</li>
                    <li>"Click 'Toggle Scale' to change the scale value (1.0 or 1.5)"</li>
                    <li>"Click 'Toggle Opacity' to change the opacity value (0.5 or 1.0)"</li>
                    <li>"All animations should respond to signal changes automatically"</li>
                </ul>
            </div>
        </div>
    }
}
