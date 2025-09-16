//! Interactive Game Demo - Working version with body mounting

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
}

#[wasm_bindgen]
pub fn PuzzleGameDemo() {
    // Mount to body - this works!
    let _ = leptos::mount::mount_to_body(|| view! { <GameDemoComponent /> });
}

#[component]
fn GameDemoComponent() -> impl IntoView {
    let (score, set_score) = signal(0);
    let (selected_item, set_selected_item) = signal::<Option<usize>>(None);

    // Handle item click
    let handle_item_click = move |index: usize| {
        set_score.update(|s| *s += 10);
        set_selected_item.set(Some(index));
        
        // Reset selection after animation
        setTimeout(move || set_selected_item.set(None), 1000);
    };

    // Shuffle items (visual effect)
    let shuffle_items = move |_| {
        set_score.set(0);
        set_selected_item.set(None);
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 text-white">
            <div class="container mx-auto px-4 py-8">
                <header class="text-center mb-8">
                    <h1 class="text-5xl font-bold mb-4 bg-gradient-to-r from-yellow-400 via-pink-400 to-purple-400 bg-clip-text text-transparent">
                        "🎮 Interactive Game Demo"
                    </h1>
                    <p class="text-xl text-gray-300 mb-6">
                        "Click items to score points and see smooth animations!"
                    </p>
                    
                    // Game stats
                    <div class="flex justify-center gap-8 mb-6">
                        <div class="bg-white/10 backdrop-blur-sm rounded-lg px-4 py-2">
                            <span class="text-sm text-gray-300">"Score: "</span>
                            <span class="text-lg font-bold text-yellow-400">{score}</span>
                        </div>
                        <div class="bg-white/10 backdrop-blur-sm rounded-lg px-4 py-2">
                            <span class="text-sm text-gray-300">"Status: "</span>
                            <span class="text-lg font-bold text-green-400">"Playing"</span>
                        </div>
                    </div>

                    // Control buttons
                    <div class="flex justify-center gap-4 mb-8">
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-green-500 to-emerald-600 text-white rounded-lg font-medium hover:from-green-600 hover:to-emerald-700 transition-all duration-200 transform hover:scale-105"
                            on:click=shuffle_items
                        >
                            "🎲 Reset Score"
                        </button>
                    </div>
                </header>

                // Game board
                <div class="flex justify-center mb-8">
                    <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
                        <div class="grid grid-cols-2 gap-6 w-80 h-80">
                            // Game item 1
                            <div
                                class="w-24 h-24 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                                on:click=move |_| handle_item_click(0)
                            >
                                "🎯"
                            </div>

                            // Game item 2
                            <div
                                class="w-24 h-24 bg-gradient-to-br from-green-400 to-blue-500 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:-rotate-3 active:scale-95"
                                on:click=move |_| handle_item_click(1)
                            >
                                "🚀"
                            </div>

                            // Game item 3
                            <div
                                class="w-24 h-24 bg-gradient-to-br from-purple-400 to-pink-500 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-6 active:scale-95"
                                on:click=move |_| handle_item_click(2)
                            >
                                "⭐"
                            </div>

                            // Game item 4
                            <div
                                class="w-24 h-24 bg-gradient-to-br from-red-400 to-orange-500 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:-rotate-6 active:scale-95"
                                on:click=move |_| handle_item_click(3)
                            >
                                "🎮"
                            </div>
                        </div>
                    </div>
                </div>

                // Instructions
                <div class="max-w-2xl mx-auto text-center">
                    <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20">
                        <h3 class="text-xl font-bold mb-4 text-yellow-400">"How to Play"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-gray-300">
                            <div class="flex items-center gap-2">
                                <span class="text-2xl">"🖱️"</span>
                                <span>"Click items to score points"</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <span class="text-2xl">"🎯"</span>
                                <span>"Watch smooth CSS animations"</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <span class="text-2xl">"⚡"</span>
                                <span>"Hover for preview effects"</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <span class="text-2xl">"🎲"</span>
                                <span>"Each click adds 10 points"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn setTimeout<F>(callback: F, delay_ms: u32)
where
    F: FnOnce() + 'static,
{
    let callback = std::rc::Rc::new(std::cell::RefCell::new(Some(callback)));
    let callback_clone = callback.clone();
    
    let closure = Closure::wrap(Box::new(move || {
        if let Some(cb) = callback_clone.borrow_mut().take() {
            cb();
        }
    }) as Box<dyn FnMut()>);
    
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms as i32,
        )
        .unwrap();
    
    closure.forget();
}