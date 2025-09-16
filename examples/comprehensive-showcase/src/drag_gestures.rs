//! Drag Gestures - Advanced drag interactions with constraints and spring physics

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::JsCast;

#[component]
pub fn DragGestures() -> impl IntoView {
    let (drag_pos, set_drag_pos) = signal((0.0, 0.0));
    let (is_dragging, set_is_dragging) = signal(false);
    let (drag_constraint, set_drag_constraint) = signal("none");

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Drag Gestures"</h2>
                <p class="text-gray-300 mb-6">
                    "Advanced drag interactions with constraints and spring physics"
                </p>
            </div>

            // Constraint selector
            <div class="flex justify-center gap-4 mb-8">
                <button
                    class=move || {
                        if drag_constraint.get() == "none" {
                            "px-4 py-2 bg-purple-600 text-white rounded-lg font-medium"
                        } else {
                            "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                        }
                    }
                    on:click=move |_| set_drag_constraint.set("none")
                >
                    "No Constraints"
                </button>
                <button
                    class=move || {
                        if drag_constraint.get() == "horizontal" {
                            "px-4 py-2 bg-purple-600 text-white rounded-lg font-medium"
                        } else {
                            "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                        }
                    }
                    on:click=move |_| set_drag_constraint.set("horizontal")
                >
                    "Horizontal Only"
                </button>
                <button
                    class=move || {
                        if drag_constraint.get() == "vertical" {
                            "px-4 py-2 bg-purple-600 text-white rounded-lg font-medium"
                        } else {
                            "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                        }
                    }
                    on:click=move |_| set_drag_constraint.set("vertical")
                >
                    "Vertical Only"
                </button>
                <button
                    class=move || {
                        if drag_constraint.get() == "bounds" {
                            "px-4 py-2 bg-purple-600 text-white rounded-lg font-medium"
                        } else {
                            "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                        }
                    }
                    on:click=move |_| set_drag_constraint.set("bounds")
                >
                    "Bounded"
                </button>
            </div>

            // Drag area
            <div class="relative w-full h-96 bg-gradient-to-br from-green-500 to-blue-600 rounded-2xl overflow-hidden">
                // Grid background
                <div class="absolute inset-0 opacity-20">
                    {move || (0..10).map(|i| {
                        view! {
                            <div 
                                class="absolute border border-white"
                                style=move || {
                                    format!(
                                        "left: {}px; top: 0px; width: 1px; height: 100%;",
                                        i * 50
                                    )
                                }
                            />
                            <div 
                                class="absolute border border-white"
                                style=move || {
                                    format!(
                                        "left: 0px; top: {}px; width: 100%; height: 1px;",
                                        i * 50
                                    )
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Draggable element
                <MotionDiv
                    class=move || {
                        if is_dragging.get() {
                            "absolute w-16 h-16 bg-white rounded-full shadow-2xl cursor-grabbing transition-shadow duration-200"
                        } else {
                            "absolute w-16 h-16 bg-white rounded-full shadow-lg cursor-grab transition-shadow duration-200 hover:shadow-xl"
                        }
                    }
                    style=move || {
                        let (x, y) = drag_pos.get();
                        let constraint = drag_constraint.get();
                        
                        let (final_x, final_y) = match constraint {
                            "horizontal" => (x, 150.0), // Keep Y centered
                            "vertical" => (150.0, y),   // Keep X centered
                            "bounds" => (
                                x.max(8.0_f64).min(392.0_f64), // Constrain to bounds
                                y.max(8.0_f64).min(392.0_f64)
                            ),
                            _ => (x, y), // No constraints
                        };
                        
                        format!(
                            "left: {}px; top: {}px; transform: translate(-50%, -50%);",
                            final_x, final_y
                        )
                    }
                    initial=MotionValue::new(150.0, 150.0)
                    animate=move || {
                        let (x, y) = drag_pos.get();
                        let constraint = drag_constraint.get();
                        
                        let (final_x, final_y) = match constraint {
                            "horizontal" => (x, 150.0),
                            "vertical" => (150.0, y),
                            "bounds" => (
                                x.max(8.0).min(392.0),
                                y.max(8.0).min(392.0)
                            ),
                            _ => (x, y),
                        };
                        
                        MotionValue::new(final_x, final_y)
                    }
                    transition=Transition {
                        duration: 0.3,
                        easing: Easing::Spring { stiffness: 300.0, damping: 30.0 },
                        ..Default::default()
                    }
                    on:mousedown=move |_| set_is_dragging.set(true)
                    on:mouseup=move |_| set_is_dragging.set(false)
                    on:mousemove=move |event: web_sys::MouseEvent| {
                        if is_dragging.get() {
                            let rect = event.current_target()
                                .unwrap()
                                .dyn_into::<web_sys::Element>()
                                .unwrap()
                                .get_bounding_client_rect();
                            
                            let x = event.client_x() as f64 - rect.left();
                            let y = event.client_y() as f64 - rect.top();
                            set_drag_pos.set((x, y));
                        }
                    }
                />

                // Spring follow element
                <MotionDiv
                    class="absolute w-8 h-8 bg-yellow-400 rounded-full shadow-lg pointer-events-none"
                    style=move || {
                        let (x, y) = drag_pos.get();
                        format!(
                            "left: {}px; top: {}px; transform: translate(-50%, -50%);",
                            x, y
                        )
                    }
                    initial=MotionValue::new(150.0, 150.0)
                    animate=move || {
                        let (x, y) = drag_pos.get();
                        MotionValue::new(x, y)
                    }
                    transition=Transition {
                        duration: 0.8,
                        easing: Easing::Spring { stiffness: 200.0, damping: 25.0 },
                        ..Default::default()
                    }
                />

                // Instructions
                <div class="absolute bottom-4 left-4 right-4 text-center">
                    <p class="text-white/80 text-sm">
                        "Drag the white circle around. Try different constraint modes!"
                    </p>
                </div>
            </div>

            // Code example
            <div class="bg-gray-900 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4">"Code Example"</h3>
                <pre class="text-sm text-gray-300 overflow-x-auto">
<code>"// Drag gestures with constraints
let (drag_pos, set_drag_pos) = create_signal((0.0, 0.0));
let (is_dragging, set_is_dragging) = create_signal(false);

MotionDiv
    animate=move || {
        let (x, y) = drag_pos.get();
        MotionValue::new(x, y)
    }
    transition=Transition {
        duration: 0.3,
        easing: Easing::Spring { 
            stiffness: 300.0, 
            damping: 30.0 
        },
        ..Default::default()
    }
    on:mousedown=move |_| set_is_dragging.set(true)
    on:mousemove=move |event| {
        if is_dragging.get() {
            let (x, y) = get_mouse_position(event);
            set_drag_pos.set((x, y));
        }
    }"</code>
                </pre>
            </div>
        </div>
    }
}
