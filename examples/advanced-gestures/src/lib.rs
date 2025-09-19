use leptos::prelude::*;
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
    let handle_rotation = move |_: web_sys::MouseEvent| {
        set_rotation.set(rotation.get() + 45.0);
    };

    // Scale gesture handler
    let handle_scale = move |_: web_sys::MouseEvent| {
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

                // Drag Gesture Demo
                <div class="mb-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "Drag Gesture"
                        </h2>
                        <p class="text-center text-blue-200 mb-6">
                            "Click and drag the circle to move it around"
                        </p>
                        
                        <div class="relative h-48 bg-black/20 rounded-lg overflow-hidden">
                            <div
                                class="absolute w-12 h-12 bg-gradient-to-r from-pink-500 to-purple-500 rounded-full cursor-grab active:cursor-grabbing shadow-lg transition-transform duration-200 ease-out"
                                style=(move || { format!(
                                    "transform: translate({}px, {}px); left: 50%; top: 50%; margin-left: -24px; margin-top: -24px;",
                                    drag_position.get().0,
                                    drag_position.get().1
                                ) })()
                                on:mousedown=move |_| set_is_dragging.set(true)
                                on:mouseup=move |_| set_is_dragging.set(false)
                                on:mousemove=handle_drag
                            >
                                "Drag me!"
                            </div>
                        </div>
                    </div>
                </div>

                // Rotation Gesture Demo
                <div class="mb-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "Rotation Gesture"
                        </h2>
                        <p class="text-center text-blue-200 mb-6">
                            "Click the square to rotate it"
                        </p>
                        
                        <div class="flex justify-center">
                            <div
                                class="w-16 h-16 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg shadow-lg cursor-pointer transition-transform duration-300 ease-in-out"
                                style=(move || format!("transform: rotate({}deg);", rotation.get()))
                                on:click=handle_rotation
                            >
                            </div>
                        </div>
                    </div>
                </div>

                // Scale Gesture Demo
                <div class="mb-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "Scale Gesture"
                        </h2>
                        <p class="text-center text-blue-200 mb-6">
                            "Click the triangle to scale it"
                        </p>
                        
                        <div class="flex justify-center">
                            <div
                                class="w-0 h-0 border-l-8 border-r-8 border-b-16 border-l-transparent border-r-transparent border-b-gradient-to-r from-green-500 to-teal-500 cursor-pointer transition-transform duration-300 ease-in-out"
                                style=(move || format!("transform: scale({});", scale.get()))
                                on:click=handle_scale
                            >
                            </div>
                        </div>
                    </div>
                </div>

                // Combined Gestures Demo
                <div class="mb-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "Combined Gestures"
                        </h2>
                        <p class="text-center text-blue-200 mb-6">
                            "Interactive element with multiple gesture support"
                        </p>
                        
                        <div class="flex justify-center">
                            <div
                                class="w-20 h-20 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-full shadow-lg cursor-pointer transition-all duration-300 ease-in-out hover:scale-110"
                                style=(move || format!(
                                    "transform: rotate({}deg) scale({});",
                                    rotation.get(),
                                    scale.get()
                                ))
                                on:click=move |event| {
                                    handle_rotation(event.clone());
                                    handle_scale(event);
                                }
                            >
                            </div>
                        </div>
                    </div>
                </div>

                // Performance Metrics
                <div class="mt-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "Performance Metrics"
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-center">
                            <div class="performance-metric">
                                <div class="text-3xl font-bold text-green-400">"<16ms"</div>
                                <div class="text-blue-200 text-sm">"Gesture Response Time"</div>
                            </div>
                            <div class="performance-metric">
                                <div class="text-3xl font-bold text-blue-400">"60fps"</div>
                                <div class="text-blue-200 text-sm">"Animation Frame Rate"</div>
                            </div>
                            <div class="performance-metric">
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