//! Material Design Ripple - Touch ripple effect with proper timing

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

#[component]
pub fn MaterialDesignRipple() -> impl IntoView {
    let (ripples, set_ripples) = signal(Vec::<(f64, f64, usize)>::new());
    let ripple_counter = RwSignal::new(0usize);

    let handle_click = move |event: MouseEvent| {
        let rect = event.current_target()
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap()
            .get_bounding_client_rect();
        
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();
        
        let id = ripple_counter.get();
        ripple_counter.set(id + 1);
        
        set_ripples.update(|ripples| {
            ripples.push((x, y, id));
        });

        // Remove ripple after animation
        set_timeout(move || {
            set_ripples.update(|ripples| {
                ripples.retain(|(_, _, ripple_id)| *ripple_id != id);
            });
        }, std::time::Duration::from_millis(600));
    };

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Material Design Ripple"</h2>
                <p class="text-gray-300 mb-6">
                    "Touch ripple effect with proper timing and physics"
                </p>
            </div>

            <div class="flex flex-wrap gap-6 justify-center">
                // Primary button
                <button
                    class="relative px-8 py-4 bg-blue-600 text-white rounded-lg font-medium overflow-hidden transition-colors duration-200 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                    on:click=handle_click
                >
                    "Primary Button"
                    
                    // Ripples
                    <For
                        each=move || ripples.get()
                        key=|(_, _, id)| *id
                        children=move |(x, y, id)| {
                            view! {
                                <MotionDiv
                                    class="absolute bg-white rounded-full pointer-events-none"
                                    style=move || {
                                        let progress = 0.0; // This would be calculated based on animation progress
                                        let opacity = 1.0 - progress;
                                        format!(
                                            "left: {}px; top: {}px; transform: translate(-50%, -50%); opacity: {};",
                                            x, y, opacity
                                        )
                                    }
                                    initial=MotionValue::new(0.0, 0.0)
                                    animate=MotionValue::new(200.0, 200.0)
                                    transition=Transition {
                                        duration: 0.6,
                                        easing: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                />
                            }
                        }
                    />
                </button>

                // Secondary button
                <button
                    class="relative px-8 py-4 bg-gray-600 text-white rounded-lg font-medium overflow-hidden transition-colors duration-200 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                    on:click=handle_click
                >
                    "Secondary Button"
                    
                    // Ripples
                    <For
                        each=move || ripples.get()
                        key=|(_, _, id)| *id
                        children=move |(x, y, id)| {
                            view! {
                                <MotionDiv
                                    class="absolute bg-white rounded-full pointer-events-none"
                                    style=move || {
                                        let progress = 0.0;
                                        let opacity = 1.0 - progress;
                                        format!(
                                            "left: {}px; top: {}px; transform: translate(-50%, -50%); opacity: {};",
                                            x, y, opacity
                                        )
                                    }
                                    initial=MotionValue::new(0.0, 0.0)
                                    animate=MotionValue::new(200.0, 200.0)
                                    transition=Transition {
                                        duration: 0.6,
                                        easing: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                />
                            }
                        }
                    />
                </button>

                // Card with ripple
                <div
                    class="relative w-64 h-32 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl overflow-hidden cursor-pointer transition-transform duration-200 hover:scale-105"
                    on:click=handle_click
                >
                    <div class="p-6 text-white">
                        <h3 class="font-semibold mb-2">"Ripple Card"</h3>
                        <p class="text-sm opacity-90">"Click anywhere on this card"</p>
                    </div>
                    
                    // Ripples
                    <For
                        each=move || ripples.get()
                        key=|(_, _, id)| *id
                        children=move |(x, y, id)| {
                            view! {
                                <MotionDiv
                                    class="absolute bg-white rounded-full pointer-events-none"
                                    style=move || {
                                        let progress = 0.0;
                                        let opacity = 1.0 - progress;
                                        format!(
                                            "left: {}px; top: {}px; transform: translate(-50%, -50%); opacity: {};",
                                            x, y, opacity
                                        )
                                    }
                                    initial=MotionValue::new(0.0, 0.0)
                                    animate=MotionValue::new(200.0, 200.0)
                                    transition=Transition {
                                        duration: 0.6,
                                        easing: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                />
                            }
                        }
                    />
                </div>
            </div>

            // Code example
            <div class="bg-gray-900 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4">"Code Example"</h3>
                <pre class="text-sm text-gray-300 overflow-x-auto">
<code>"// Material Design ripple effect
let (ripples, set_ripples) = create_signal(Vec::new());

let handle_click = move |event: MouseEvent| {
    let (x, y) = get_click_position(event);
    set_ripples.update(|ripples| ripples.push((x, y, id)));
    
    // Remove after animation
    set_timeout(move || {
        set_ripples.update(|ripples| ripples.retain(|r| r.id != id));
    }, 600);
};

MotionDiv
    initial=MotionValue::new(0.0, 0.0)
    animate=MotionValue::new(200.0, 200.0)
    transition=Transition {
        duration: 0.6,
        easing: Easing::EaseOut,
        ..Default::default()
    }"</code>
                </pre>
            </div>
        </div>
    }
}
