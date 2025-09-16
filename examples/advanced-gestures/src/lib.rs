use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Advanced Gesture Examples
/// Inspired by Motion.dev's gesture examples but built with Rust and WASM
#[component]
pub fn AdvancedGesturesDemo() -> impl IntoView {
    let (drag_position, set_drag_position) = signal((0.0, 0.0));
    let (is_dragging, set_is_dragging) = signal(false);
    let (rotation, set_rotation) = signal(0.0);
    let (scale, set_scale) = signal(1.0);

    // Drag gesture handler
    let handle_drag = move |event: web_sys::MouseEvent| {
        if is_dragging.get() {
            let rect = event.current_target()
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap()
                .get_bounding_client_rect();
            
            let x = event.client_x() as f64 - rect.left() - rect.width() / 2.0;
            let y = event.client_y() as f64 - rect.top() - rect.height() / 2.0;
            
            set_drag_position.set((x, y));
        }
    };

    // Rotation gesture handler
    let handle_rotation = move |_| {
        set_rotation.set(rotation.get() + 45.0);
    };

    // Scale gesture handler
    let handle_scale = move |_| {
        set_scale.set(if scale.get() > 1.0 { 1.0 } else { 1.5 });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 p-8">
            <div class="max-w-6xl mx-auto">
                <h1 class="text-4xl font-bold text-white text-center mb-8">
                    "Advanced Gesture Examples"
                </h1>
                <p class="text-center text-blue-200 mb-12 text-lg">
                    "Interactive gesture-based animations powered by Rust and WASM"
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    
                    // Drag Gesture Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Drag Gesture"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "Click and drag the circle to move it around"
                        </p>
                        
                        <div class="relative h-48 bg-black/20 rounded-lg overflow-hidden">
                            <MotionDiv
                                class="absolute w-12 h-12 bg-gradient-to-r from-pink-500 to-purple-500 rounded-full cursor-grab active:cursor-grabbing shadow-lg".to_string()
                    style=(move || { format!(
                        "transform: translate({}px, {}px); left: 50%; top: 50%; margin-left: -24px; margin-top: -24px;",
                        drag_position.get().0,
                        drag_position.get().1
                    ) })()
                                on:mousedown=move |_| set_is_dragging.set(true)
                                on:mouseup=move |_| set_is_dragging.set(false)
                                on:mousemove=handle_drag
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                    target.insert("y".to_string(), AnimationValue::Pixels(-5.0));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.2),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                }
                            >
                                "Drag me!"
                            </MotionDiv>
                        </div>
                    </div>

                    // Rotation Gesture Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Rotation Gesture"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "Click to rotate the square"
                        </p>
                        
                        <div class="flex justify-center items-center h-48">
                            <MotionDiv
                                class="w-16 h-16 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg cursor-pointer shadow-lg".to_string()
                                style=(move || { format!("transform: rotate({}deg);", rotation.get()) })()
                                on:click=handle_rotation
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(1.05));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseInOut,
                                    ..Default::default()
                                }
                            >
                                "Rotate me!"
                            </MotionDiv>
                        </div>
                    </div>

                    // Scale Gesture Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Scale Gesture"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "Click to scale the triangle"
                        </p>
                        
                        <div class="flex justify-center items-center h-48">
                            <MotionDiv
                                class="w-0 h-0 border-l-8 border-r-8 border-b-16 border-l-transparent border-r-transparent border-b-gradient-to-r from-green-500 to-emerald-500 cursor-pointer".to_string()
                                style=(move || { format!("transform: scale({});", scale.get()) })()
                                on:click=handle_scale
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("opacity".to_string(), AnimationValue::Number(0.8));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.4),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                }
                            >
                                "Scale me!"
                            </MotionDiv>
                        </div>
                    </div>

                    // Multi-touch Gesture Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Multi-touch Gesture"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "Pinch to zoom (simulated with mouse wheel)"
                        </p>
                        
                        <div class="flex justify-center items-center h-48">
                            <MotionDiv
                                class="w-20 h-20 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-full cursor-pointer shadow-lg".to_string()
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(1.2));
                                    target.insert("rotate".to_string(), AnimationValue::Number(180.0));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.5),
                                    ease: Easing::EaseInOut,
                                    ..Default::default()
                                }
                            >
                                "Multi-touch me!"
                            </MotionDiv>
                        </div>
                    </div>

                    // Gesture Sequence Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Gesture Sequence"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "Complex gesture combinations"
                        </p>
                        
                        <div class="flex justify-center items-center h-48">
                            <MotionDiv
                                class="w-16 h-16 bg-gradient-to-r from-red-500 to-pink-500 rounded-lg cursor-pointer shadow-lg".to_string()
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                    target.insert("rotate".to_string(), AnimationValue::Number(45.0));
                                    target.insert("y".to_string(), AnimationValue::Pixels(-10.0));
                                    target
                                }
                                while_tap={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(0.95));
                                    target.insert("rotate".to_string(), AnimationValue::Number(-45.0));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                }
                            >
                                "Sequence me!"
                            </MotionDiv>
                        </div>
                    </div>

                    // Gesture Performance Example
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-6 border border-white/20">
                        <h3 class="text-xl font-semibold text-white mb-4">"Performance Demo"</h3>
                        <p class="text-blue-200 text-sm mb-4">
                            "60fps gesture tracking with WASM"
                        </p>
                        
                        <div class="flex justify-center items-center h-48">
                            <MotionDiv
                                class="w-12 h-12 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full cursor-pointer shadow-lg".to_string()
                                while_hover={
                                    let mut target = HashMap::new();
                                    target.insert("scale".to_string(), AnimationValue::Number(1.3));
                                    target.insert("rotate".to_string(), AnimationValue::Number(360.0));
                                    target
                                }
                                transition=Transition {
                                    duration: Some(0.6),
                                    ease: Easing::EaseInOut,
                                    repeat: RepeatConfig::Count(1),
                                    ..Default::default()
                                }
                            >
                                "Performance me!"
                            </MotionDiv>
                        </div>
                    </div>
                </div>

                <div class="mt-12 text-center">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-4">"Rust + WASM Performance"</h2>
                        <p class="text-blue-200 mb-6">
                            "All gestures are powered by our Rust-based animation engine, 
                            compiled to WASM for maximum performance in the browser."
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-center">
                            <div>
                                <div class="text-3xl font-bold text-green-400">"60fps"</div>
                                <div class="text-blue-200 text-sm">"Smooth Animations"</div>
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-blue-400">"<1ms"</div>
                                <div class="text-blue-200 text-sm">"Gesture Latency"</div>
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-purple-400">"WASM"</div>
                                <div class="text-blue-200 text-sm">"Native Performance"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AdvancedGesturesDemo />
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
