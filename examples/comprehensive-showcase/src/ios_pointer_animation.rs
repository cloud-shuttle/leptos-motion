//! iOS Pointer Animation - Smooth pointer following with spring physics

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

#[component]
pub fn IosPointerAnimation() -> impl IntoView {
    let (mouse_pos, set_mouse_pos) = signal((0.0, 0.0));
    let (is_hovering, set_is_hovering) = signal(false);

    let handle_mouse_move = move |event: MouseEvent| {
        let rect = event.current_target()
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap()
            .get_bounding_client_rect();
        
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();
        set_mouse_pos.set((x, y));
    };

    let handle_mouse_enter = move |_| set_is_hovering.set(true);
    let handle_mouse_leave = move |_| set_is_hovering.set(false);

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"iOS Pointer Animation"</h2>
                <p class="text-gray-300 mb-6">
                    "Smooth pointer following with spring physics, inspired by iOS interactions"
                </p>
            </div>

            <div 
                class="relative w-full h-96 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl overflow-hidden cursor-none"
                on:mousemove=handle_mouse_move
                on:mouseenter=handle_mouse_enter
                on:mouseleave=handle_mouse_leave
            >
                // Background grid
                <div class="absolute inset-0 opacity-20">
                    {move || (0..20).map(|i| {
                        view! {
                            <div 
                                class="absolute bg-white rounded-full"
                                style=move || {
                                    let (x, y) = mouse_pos.get();
                                    let distance = ((x - (i as f64 * 50.0)).powi(2) + (y - (i as f64 * 30.0)).powi(2)).sqrt();
                                    let scale = if distance < 100.0 { 1.0 - (distance / 100.0) * 0.8 } else { 0.2 };
                                    format!(
                                        "left: {}px; top: {}px; width: 4px; height: 4px; transform: scale({});",
                                        i * 50, i * 30, scale
                                    )
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Main pointer
                <MotionDiv
                    class="absolute w-8 h-8 bg-white rounded-full shadow-lg pointer-events-none"
                    style=move || {
                        let (x, y) = mouse_pos.get();
                        format!(
                            "left: {}px; top: {}px; transform: translate(-50%, -50%);",
                            x, y
                        )
                    }
                    initial=MotionValue::new(0.0, 0.0)
                    animate=move || {
                        let (x, y) = mouse_pos.get();
                        MotionValue::new(x, y)
                    }
                    transition=Transition {
                        duration: 0.3,
                        easing: Easing::Spring { stiffness: 300.0, damping: 30.0 },
                        ..Default::default()
                    }
                />

                // Ripple effect
                <MotionDiv
                    class="absolute w-16 h-16 bg-white rounded-full pointer-events-none"
                    style=move || {
                        let (x, y) = mouse_pos.get();
                        let opacity = if is_hovering.get() { 0.3 } else { 0.0 };
                        format!(
                            "left: {}px; top: {}px; transform: translate(-50%, -50%); opacity: {};",
                            x, y, opacity
                        )
                    }
                    initial=MotionValue::new(0.0, 0.0)
                    animate=move || {
                        let (x, y) = mouse_pos.get();
                        MotionValue::new(x, y)
                    }
                    transition=Transition {
                        duration: 0.6,
                        easing: Easing::Spring { stiffness: 200.0, damping: 25.0 },
                        ..Default::default()
                    }
                />

                // Instructions
                <div class="absolute bottom-4 left-4 right-4 text-center">
                    <p class="text-white/80 text-sm">
                        "Move your mouse around to see the smooth pointer following"
                    </p>
                </div>
            </div>

            // Code example
            <div class="bg-gray-900 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4">"Code Example"</h3>
                <pre class="text-sm text-gray-300 overflow-x-auto">
<code>"// Smooth pointer following with spring physics
let (mouse_pos, set_mouse_pos) = create_signal((0.0, 0.0));

MotionDiv
    animate=move || {
        let (x, y) = mouse_pos.get();
        MotionValue::new(x, y)
    }
    transition=Transition {
        duration: 0.3,
        easing: Easing::Spring { 
            stiffness: 300.0, 
            damping: 30.0 
        },
        ..Default::default()
    }"</code>
                </pre>
            </div>
        </div>
    }
}
