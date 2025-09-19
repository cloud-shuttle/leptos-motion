//! Navigation module
//! 
//! This module handles navigation between different examples in the showcase.

use leptos::prelude::*;

#[component]
pub fn Navigation(
    current_example: ReadSignal<i32>,
    set_current_example: WriteSignal<i32>,
) -> impl IntoView {
    let next_example = move |_| {
        set_current_example.update(|i| *i = (*i + 1) % 9);
    };

    let prev_example = move |_| {
        set_current_example.update(|i| *i = if *i == 0 { 8 } else { *i - 1 });
    };

    let example_names = vec![
        "React Examples",
        "Apple Watch Demo", 
        "Source Unlock",
        "Motion Gallery",
        "Interactive Demo",
        "CSS Generation",
        "Path Drawing",
        "Conic Gradient",
        "Drag Transform"
    ];

    view! {
        <div class="flex justify-between items-center mb-8">
            <button
                class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-200 font-semibold"
                on:click=prev_example
            >
                "← Previous"
            </button>
            
            <div class="text-center">
                <h2 class="text-2xl font-bold text-white mb-2">
                    {move || example_names[current_example.get() as usize].to_string()}
                </h2>
                <p class="text-gray-400">
                    "Example " {move || current_example.get() + 1} " of 9"
                </p>
            </div>
            
            <button
                class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-200 font-semibold"
                on:click=next_example
            >
                "Next →"
            </button>
        </div>
    }
}
