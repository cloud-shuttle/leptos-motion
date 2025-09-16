//! Layout Animations - FLIP animations for smooth layout transitions

use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_layout::*;

#[component]
pub fn LayoutAnimations() -> impl IntoView {
    let (items, set_items) = signal(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
    ]);
    let (layout_mode, set_layout_mode) = signal("grid");

    let add_item = move |_| {
        set_items.update(|items| {
            let new_id = items.len() + 1;
            items.push(format!("Item {}", new_id));
        });
    };

    let remove_item = move |index: usize| {
        set_items.update(|items| {
            if items.len() > 1 {
                items.remove(index);
            }
        });
    };

    let shuffle_items = move |_| {
        set_items.update(|items| {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            items.shuffle(&mut rng);
        });
    };

    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Layout Animations"</h2>
                <p class="text-gray-300 mb-6">
                    "FLIP animations for smooth layout transitions"
                </p>
            </div>

            // Controls
            <div class="flex flex-wrap justify-center gap-4 mb-8">
                <button
                    class="px-6 py-3 bg-green-600 text-white rounded-lg font-medium hover:bg-green-700 transition-colors duration-200"
                    on:click=add_item
                >
                    "Add Item"
                </button>
                <button
                    class="px-6 py-3 bg-red-600 text-white rounded-lg font-medium hover:bg-red-700 transition-colors duration-200"
                    on:click=move |_| {
                        if items.get().len() > 1 {
                            remove_item(items.get().len() - 1);
                        }
                    }
                >
                    "Remove Item"
                </button>
                <button
                    class="px-6 py-3 bg-purple-600 text-white rounded-lg font-medium hover:bg-purple-700 transition-colors duration-200"
                    on:click=shuffle_items
                >
                    "Shuffle"
                </button>
                
                // Layout mode selector
                <div class="flex gap-2">
                    <button
                        class=move || {
                            if layout_mode.get() == "grid" {
                                "px-4 py-2 bg-blue-600 text-white rounded-lg font-medium"
                            } else {
                                "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                            }
                        }
                        on:click=move |_| set_layout_mode.set("grid")
                    >
                        "Grid"
                    </button>
                    <button
                        class=move || {
                            if layout_mode.get() == "list" {
                                "px-4 py-2 bg-blue-600 text-white rounded-lg font-medium"
                            } else {
                                "px-4 py-2 bg-gray-600 text-gray-300 rounded-lg font-medium hover:bg-gray-500"
                            }
                        }
                        on:click=move |_| set_layout_mode.set("list")
                    >
                        "List"
                    </button>
                </div>
            </div>

            // Layout container
            <div class="min-h-96 bg-gray-800 rounded-2xl p-8">
                <div 
                    class=move || {
                        if layout_mode.get() == "grid" {
                            "grid grid-cols-3 gap-4"
                        } else {
                            "flex flex-col gap-4"
                        }
                    }
                >
                    <For
                        each=move || items.get()
                        key=|item| item.clone()
                        children=move |item| {
                            let item_clone = item.clone();
                            view! {
                                <MotionDiv
                                    class="bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg p-6 text-white font-medium cursor-pointer transition-transform duration-200 hover:scale-105"
                                    _layout=true
                                    transition=Transition {
                                        duration: 0.5,
                                        easing: Easing::Spring { stiffness: 300.0, damping: 30.0 },
                                        ..Default::default()
                                    }
                                    on:click=move |_| {
                                        if let Some(index) = items.get().iter().position(|i| i == &item_clone) {
                                            remove_item(index);
                                        }
                                    }
                                >
                                    <div class="flex justify-between items-center">
                                        <span>{item}</span>
                                        <button 
                                            class="text-white/70 hover:text-white transition-colors duration-200"
                                            on:click=move |event: web_sys::MouseEvent| {
                                                event.stop_propagation();
                                                if let Some(index) = items.get().iter().position(|i| i == &item_clone) {
                                                    remove_item(index);
                                                }
                                            }
                                        >
                                            "×"
                                        </button>
                                    </div>
                                </MotionDiv>
                            }
                        }
                    />
                </div>
            </div>

            // Shared element demo
            <div class="space-y-4">
                <h3 class="text-xl font-semibold text-center">"Shared Element Animation"</h3>
                <div class="grid grid-cols-2 gap-8">
                    <div class="space-y-4">
                        <h4 class="font-medium">"Source"</h4>
                        <MotionDiv
                            class="w-32 h-32 bg-gradient-to-br from-pink-500 to-red-500 rounded-xl cursor-pointer"
                            _layout=true
                            transition=Transition {
                                duration: 0.6,
                                easing: Easing::Spring { stiffness: 200.0, damping: 25.0 },
                                ..Default::default()
                            }
                        />
                    </div>
                    <div class="space-y-4">
                        <h4 class="font-medium">"Target"</h4>
                        <MotionDiv
                            class="w-32 h-32 bg-gradient-to-br from-pink-500 to-red-500 rounded-xl cursor-pointer"
                            _layout=true
                            transition=Transition {
                                duration: 0.6,
                                easing: Easing::Spring { stiffness: 200.0, damping: 25.0 },
                                ..Default::default()
                            }
                        />
                    </div>
                </div>
            </div>

            // Code example
            <div class="bg-gray-900 rounded-lg p-6">
                <h3 class="text-lg font-semibold mb-4">"Code Example"</h3>
                <pre class="text-sm text-gray-300 overflow-x-auto">
<code>"// Layout animations with FLIP
let (items, set_items) = create_signal(vec![\"Item 1\".to_string()]);

MotionDiv
    _layout=true
    transition=Transition {
        duration: 0.5,
        easing: Easing::Spring { 
            stiffness: 300.0, 
            damping: 30.0 
        },
        ..Default::default()
    }
    on:click=move |_| {
        set_items.update(|items| {
            items.push(format!(\"Item {}\", items.len() + 1));
        });
    }"</code>
                </pre>
            </div>
        </div>
    }
}
