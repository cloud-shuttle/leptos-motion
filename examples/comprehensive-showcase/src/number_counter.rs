//! Number Counter - Animated number counting

use leptos::prelude::*;
use leptos_motion::*;

#[component]
pub fn NumberCounter() -> impl IntoView {
    let (count, set_count) = signal(0);

    let increment = move |_| {
        set_count.update(|c| *c += 1);
    };

    let decrement = move |_| {
        set_count.update(|c| *c = (*c).saturating_sub(1i32));
    };

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Number Counter"</h2>
                <p class="text-gray-300 mb-6">
                    "Animated number counting with smooth transitions"
                </p>
            </div>

            <div class="flex flex-col items-center space-y-8">
                <div class="text-6xl font-bold text-white">
                    <MotionDiv
                        key=move || count.get()
                        initial=MotionValue::new(0.0, 0.0)
                        animate=MotionValue::new(1.0, 1.0)
                        transition=Transition {
                            duration: 0.3,
                            easing: Easing::Spring { stiffness: 300.0, damping: 30.0 },
                            ..Default::default()
                        }
                    >
                        {move || count.get()}
                    </MotionDiv>
                </div>

                <div class="flex gap-4">
                    <button
                        class="px-8 py-4 bg-red-600 text-white rounded-lg font-medium hover:bg-red-700 transition-colors duration-200"
                        on:click=decrement
                    >
                        "Decrement"
                    </button>
                    <button
                        class="px-8 py-4 bg-green-600 text-white rounded-lg font-medium hover:bg-green-700 transition-colors duration-200"
                        on:click=increment
                    >
                        "Increment"
                    </button>
                </div>
            </div>
        </div>
    }
}
