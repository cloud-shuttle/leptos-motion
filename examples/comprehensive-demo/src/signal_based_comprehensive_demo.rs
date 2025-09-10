use leptos::prelude::*;
use leptos_motion_dom::signal_based_animation_controller::*;
use leptos_motion_core::types::AnimationValue;
use std::collections::HashMap;
use std::rc::Rc;

#[component]
pub fn SignalBasedComprehensiveDemo() -> impl IntoView {
    // ✅ Signal-based state management following the proven patterns
    let (counter, set_counter) = signal(0);
    let (is_visible, set_is_visible) = signal(true);
    let (is_grid_layout, set_is_grid_layout) = signal(false);
    
    // ✅ Create animation controllers for different sections
    let fade_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    }));
    
    let scale_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        map
    }));
    
    let slide_controller = Rc::new(SignalBasedAnimationController::new({
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map
    }));

    // ✅ Animation functions using signal-based patterns
    let fade_controller_clone1 = fade_controller.clone();
    let fade_in = move |_| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        fade_controller_clone1.animate_to(target);
    };
    
    let fade_controller_clone2 = fade_controller.clone();
    let fade_out = move |_| {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        fade_controller_clone2.animate_to(target);
    };
    
    let scale_controller_clone1 = scale_controller.clone();
    let scale_up = move |_| {
        let mut target = HashMap::new();
        target.insert("transform".to_string(), AnimationValue::String("scale(1.2)".to_string()));
        scale_controller_clone1.animate_to(target);
    };
    
    let scale_controller_clone2 = scale_controller.clone();
    let scale_down = move |_| {
        let mut target = HashMap::new();
        target.insert("transform".to_string(), AnimationValue::String("scale(0.8)".to_string()));
        scale_controller_clone2.animate_to(target);
    };
    
    let slide_controller_clone1 = slide_controller.clone();
    let slide_left = move |_| {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(-100.0));
        slide_controller_clone1.animate_to(target);
    };
    
    let slide_controller_clone2 = slide_controller.clone();
    let slide_right = move |_| {
        let mut target = HashMap::new();
        target.insert("x".to_string(), AnimationValue::Pixels(100.0));
        slide_controller_clone2.animate_to(target);
    };

    view! {
        <div class="app" style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <header style="text-align: center; margin-bottom: 3rem;">
                <h1 style="font-size: 3rem; margin-bottom: 1rem; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                    "🚀 Leptos Motion"
                </h1>
                <p style="font-size: 1.2rem; color: #666; margin-bottom: 0.5rem;">
                    "Signal-Based Animation Controller Demo"
                </p>
                <p style="font-size: 1rem; color: #888;">
                    "Using proven WASM + Signals patterns for reactive animations"
                </p>
            </header>

            <main>
                // ✅ Interactive Controls Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Interactive Controls"</h2>
                    
                    <div style="display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 2rem;">
                        <button 
                            on:click=move |_| set_counter.update(|c| *c += 1)
                            style="padding: 0.75rem 1.5rem; background: #4ecdc4; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600;"
                        >
                            "Increment Counter"
                        </button>
                        
                        <button 
                            on:click=move |_| set_is_visible.update(|v| *v = !*v)
                            style="padding: 0.75rem 1.5rem; background: #ff6b6b; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600;"
                        >
                            "Toggle Visibility"
                        </button>
                        
                        <button 
                            on:click=move |_| set_is_grid_layout.update(|g| *g = !*g)
                            style="padding: 0.75rem 1.5rem; background: #45b7d1; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600;"
                        >
                            "Toggle Layout"
                        </button>
                    </div>
                    
                    <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                        <button on:click=fade_in style="padding: 0.5rem 1rem; background: #96ceb4; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Fade In"
                        </button>
                        <button on:click=fade_out style="padding: 0.5rem 1rem; background: #feca57; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Fade Out"
                        </button>
                        <button on:click=scale_up style="padding: 0.5rem 1rem; background: #ff9ff3; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Scale Up"
                        </button>
                        <button on:click=scale_down style="padding: 0.5rem 1rem; background: #54a0ff; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Scale Down"
                        </button>
                        <button on:click=slide_left style="padding: 0.5rem 1rem; background: #5f27cd; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Slide Left"
                        </button>
                        <button on:click=slide_right style="padding: 0.5rem 1rem; background: #00d2d3; color: white; border: none; border-radius: 6px; cursor: pointer;">
                            "Slide Right"
                        </button>
                    </div>
                    
                    <div style="margin-top: 2rem; padding: 1rem; background: white; border-radius: 8px; border: 2px solid #e9ecef;">
                        <p style="margin: 0; font-weight: 600; color: #333;">
                            "Counter: " {counter}
                        </p>
                        <p style="margin: 0.5rem 0 0 0; color: #666;">
                            "Visibility: " {move || if is_visible.get() { "Visible" } else { "Hidden" }}
                        </p>
                        <p style="margin: 0.5rem 0 0 0; color: #666;">
                            "Layout: " {move || if is_grid_layout.get() { "Grid" } else { "List" }}
                        </p>
                    </div>
                </section>

                // ✅ Fade Animation Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Fade Animation"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "This section demonstrates fade in/out animations using the SignalBasedAnimationController."
                    </p>
                    <div 
                        style="width: 200px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600;"
                    >
                        "Fade Target"
                    </div>
                </section>

                // ✅ Scale Animation Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Scale Animation"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "This section demonstrates scale animations using signal-based patterns."
                    </p>
                    <div 
                        style="width: 150px; height: 150px; background: linear-gradient(45deg, #96ceb4, #feca57); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600;"
                    >
                        "Scale Target"
                    </div>
                </section>

                // ✅ Slide Animation Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Slide Animation"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "This section demonstrates slide animations with proper signal tracking."
                    </p>
                    <div style="position: relative; width: 100%; height: 80px; background: #e9ecef; border-radius: 8px; overflow: hidden;">
                        <div 
                            style="position: absolute; top: 20px; left: 50%; transform: translateX(-50%); width: 100px; height: 40px; background: linear-gradient(45deg, #5f27cd, #00d2d3); border-radius: 6px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 0.9rem;"
                        >
                            "Slide Target"
                        </div>
                    </div>
                </section>

                // ✅ Layout Transition Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Layout Transitions"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "This section demonstrates layout transitions between grid and list layouts."
                    </p>
                    <div 
                        style=move || {
                            if is_grid_layout.get() {
                                "display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem;"
                            } else {
                                "display: flex; flex-direction: column; gap: 1rem;"
                            }
                        }
                    >
                        {move || (0..6).map(|i| {
                            view! {
                                <div 
                                    style="padding: 1rem; background: linear-gradient(45deg, #ff9ff3, #54a0ff); border-radius: 8px; color: white; font-weight: 600; text-align: center;"
                                >
                                    {format!("Item {}", i + 1)}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>

                // ✅ Gesture Integration Section
                <section class="demo-section" style="margin-bottom: 3rem; padding: 2rem; background: #f8f9fa; border-radius: 12px;">
                    <h2 style="color: #333; margin-bottom: 1.5rem; font-size: 1.8rem;">"Gesture Integration"</h2>
                    <p style="color: #666; margin-bottom: 2rem;">
                        "This section demonstrates gesture integration with signal-based animations."
                    </p>
                    <div 
                        style="width: 200px; height: 200px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; cursor: pointer; user-select: none;"
                        on:click={
                            let scale_controller_clone3 = scale_controller.clone();
                            move |_| {
                                // Simple click gesture animation
                                let mut target = HashMap::new();
                                target.insert("transform".to_string(), AnimationValue::String("scale(0.95)".to_string()));
                                scale_controller_clone3.animate_to(target);
                            }
                        }
                    >
                        "Click Me!"
                    </div>
                </section>
            </main>
        </div>
    }
}
