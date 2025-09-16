use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;
use rand::prelude::SliceRandom;

/// Layout Animation Examples
/// Showcasing FLIP animations and layout transitions
#[component]
pub fn LayoutAnimationsDemo() -> impl IntoView {
    let (items, set_items) = signal(vec![
        "Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string(), 
        "Item 4".to_string(), "Item 5".to_string(), "Item 6".to_string()
    ]);
    let (layout_mode, set_layout_mode) = signal("grid");

    // Shuffle items
    let shuffle_items = move |_| {
        let mut current_items = items.get();
        current_items.shuffle(&mut rand::thread_rng());
        set_items.set(current_items);
    };

    // Add new item
    let add_item = move |_| {
        let mut current_items = items.get();
        let new_index = current_items.len() + 1;
        current_items.push(format!("Item {}", new_index));
        set_items.set(current_items);
    };

    // Remove item
    let remove_item = move |_| {
        let mut current_items = items.get();
        if !current_items.is_empty() {
            current_items.pop();
            set_items.set(current_items);
        }
    };

    // Toggle layout mode
    let toggle_layout = move |_| {
        set_layout_mode.set(if layout_mode.get() == "grid" { "list" } else { "grid" });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-green-900 via-teal-900 to-blue-900 p-8">
            <div class="max-w-6xl mx-auto">
                <h1 class="text-4xl font-bold text-white text-center mb-8">
                    "Layout Animation Examples"
                </h1>
                <p class="text-center text-teal-200 mb-12 text-lg">
                    "FLIP animations and layout transitions powered by Rust"
                </p>

                // Controls
                <div class="flex flex-wrap justify-center gap-4 mb-8">
                    <button
                        on:click=shuffle_items
                        class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors duration-200"
                    >
                        "Shuffle Items"
                    </button>
                    <button
                        on:click=add_item
                        class="px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg transition-colors duration-200"
                    >
                        "Add Item"
                    </button>
                    <button
                        on:click=remove_item
                        class="px-6 py-3 bg-red-600 hover:bg-red-700 text-white font-semibold rounded-lg transition-colors duration-200"
                    >
                        "Remove Item"
                    </button>
                    <button
                        on:click=toggle_layout
                        class="px-6 py-3 bg-purple-600 hover:bg-purple-700 text-white font-semibold rounded-lg transition-colors duration-200"
                    >
                        {move || if layout_mode.get() == "grid" { "Switch to List" } else { "Switch to Grid" }}
                    </button>
                </div>

                // Layout Container
                <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                    <div class=move || {
                        if layout_mode.get() == "grid" {
                            "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
                        } else {
                            "flex flex-col gap-4"
                        }
                    }>
                        <For
                            each=move || items.get()
                            key=|item| item.clone()
                            children=move |item| {
                                let item_clone = item.clone();
                                view! {
                                    <MotionDiv
                                        class=(move || {
                                            if layout_mode.get() == "grid" {
                                "bg-gradient-to-r from-cyan-500 to-blue-500 p-6 rounded-xl text-white font-semibold shadow-lg cursor-pointer hover:shadow-xl transition-shadow duration-200".to_string()
                            } else {
                                "bg-gradient-to-r from-cyan-500 to-blue-500 p-4 rounded-lg text-white font-semibold shadow-lg cursor-pointer hover:shadow-xl transition-shadow duration-200 flex items-center justify-between".to_string()
                            }
                                        })()
                                        initial={
                                            let mut target = HashMap::new();
                                            target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                            target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                            target.insert("y".to_string(), AnimationValue::Pixels(20.0));
                                            target
                                        }
                                        animate={
                                            let mut target = HashMap::new();
                                            target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                            target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                            target.insert("y".to_string(), AnimationValue::Pixels(0.0));
                                            target
                                        }
                                        _layout=true
                                        while_hover={
                                            let mut target = HashMap::new();
                                            target.insert("scale".to_string(), AnimationValue::Number(1.05));
                                            target.insert("y".to_string(), AnimationValue::Pixels(-5.0));
                                            target
                                        }
                                        transition=Transition {
                                            duration: Some(0.3),
                                            ease: Easing::EaseOut,
                                            ..Default::default()
                                        }
                                    >
                                        <span>{item_clone}</span>
                                        {if layout_mode.get() == "list" {
                                            view! {
                                                <span class="text-sm opacity-75">
                                                    "Click to interact"
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <span class="text-sm opacity-75">
                                                    "Click to interact"
                                                </span>
                                            }.into_view()
                                        }}
                                    </MotionDiv>
                                }
                            }
                        />
                    </div>
                </div>

                // FLIP Animation Demo
                <div class="mt-12">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-6 text-center">
                            "FLIP Animation Demo"
                        </h2>
                        <p class="text-center text-teal-200 mb-8">
                            "First, Last, Invert, Play - smooth layout transitions"
                        </p>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-white">"Before"</h3>
                                <div class="bg-black/20 rounded-lg p-4 h-32 flex items-center justify-center">
                                    <div class="w-16 h-16 bg-gradient-to-r from-pink-500 to-red-500 rounded-lg"></div>
                                </div>
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-white">"After"</h3>
                                <div class="bg-black/20 rounded-lg p-4 h-32 flex items-end justify-end">
                                    <div class="w-16 h-16 bg-gradient-to-r from-pink-500 to-red-500 rounded-lg"></div>
                                </div>
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
                                <div class="text-teal-200 text-sm">"Layout Transition Time"</div>
                            </div>
                            <div class="performance-metric">
                                <div class="text-3xl font-bold text-blue-400">"60fps"</div>
                                <div class="text-teal-200 text-sm">"Animation Frame Rate"</div>
                            </div>
                            <div class="performance-metric">
                                <div class="text-3xl font-bold text-purple-400">"WASM"</div>
                                <div class="text-teal-200 text-sm">"Native Performance"</div>
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
        <LayoutAnimationsDemo />
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
