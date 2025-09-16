//! Simple Comprehensive Demo - Working examples with current API

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
}

#[wasm_bindgen]
pub fn SimpleComprehensiveDemo() {
    // Mount to the #app element instead of body
    let app_element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let _ = leptos::mount::mount_to(app_element, || view! { <SimpleComprehensiveDemoComponent /> });
}

#[component]
fn SimpleComprehensiveDemoComponent() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);
    let (click_count, set_click_count) = signal(0);
    let (items, set_items) = signal(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ]);

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

    view! {
        <div 
            class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white" 
            style="user-select: text !important; -webkit-user-select: text !important; -moz-user-select: text !important; -ms-user-select: text !important;"
            on:contextmenu=move |_| {
                // Allow right-click context menu
            }
            on:selectstart=move |_| {
                // Allow text selection
            }
        >
            <div class="container mx-auto px-4 py-8">
                <header class="text-center mb-12">
                    <h1 class="text-5xl font-bold mb-4 bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                        "Leptos Motion"
                    </h1>
                    <p class="text-xl text-gray-300 mb-8">
                        "Simple Comprehensive Demo - Working Examples"
                    </p>
                </header>

                // Demo content
                <main class="max-w-6xl mx-auto space-y-12">
                    // Basic Animation Demo
                    <div class="bg-gray-800/50 backdrop-blur-sm rounded-2xl p-8 border border-gray-700">
                        <div class="text-center mb-8">
                            <h2 class="text-3xl font-bold mb-4">"Basic Animation"</h2>
                            <p class="text-gray-300 mb-6">
                                "Simple scale and rotation animation"
                            </p>
                        </div>

                        <div class="flex justify-center">
                            <MotionDiv
                                class="w-32 h-32 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl cursor-pointer flex items-center justify-center text-white font-bold text-xl".to_string()
                                initial=(move || {
                                    let mut initial = HashMap::new();
                                    initial.insert("scale".to_string(), AnimationValue::Number(1.0));
                                    initial.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
                                    initial
                                })()
                                animate=(move || {
                                    let mut animate = HashMap::new();
                                    if is_animated.get() {
                                        animate.insert("scale".to_string(), AnimationValue::Number(1.2));
                                        animate.insert("rotate".to_string(), AnimationValue::Degrees(180.0));
                                    } else {
                                        animate.insert("scale".to_string(), AnimationValue::Number(1.0));
                                        animate.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
                                    }
                                    animate
                                })()
                                transition=Transition {
                                    duration: Some(0.5),
                                    ease: Easing::EaseInOut,
                                    delay: None,
                                    repeat: RepeatConfig::Never,
                                    stagger: None,
                                }
                                on:click=move |_| set_is_animated.set(!is_animated.get())
                            >
                                "Click Me!"
                            </MotionDiv>
                        </div>

                        <div class="text-center mt-6">
                            <button
                                class="px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors duration-200"
                                on:click=move |_| set_is_animated.set(!is_animated.get())
                            >
                                "Toggle Animation"
                            </button>
                        </div>
                    </div>

                    // Hover Effects Demo
                    <div class="bg-gray-800/50 backdrop-blur-sm rounded-2xl p-8 border border-gray-700">
                        <div class="text-center mb-8">
                            <h2 class="text-3xl font-bold mb-4">"Hover Effects"</h2>
                            <p class="text-gray-300 mb-6">
                                "Smooth hover animations with different effects"
                            </p>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <MotionDiv
                                class="w-full h-32 bg-gradient-to-br from-green-500 to-teal-600 rounded-xl cursor-pointer flex items-center justify-center text-white font-bold text-lg".to_string()
                                initial=(move || {
                                    let mut initial = HashMap::new();
                                    initial.insert("scale".to_string(), AnimationValue::Number(1.0));
                                    initial
                                })()
                                while_hover=(move || {
                                    let mut hover = HashMap::new();
                                    hover.insert("scale".to_string(), AnimationValue::Number(1.05));
                                    hover
                                })()
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseOut,
                                    delay: None,
                                    repeat: RepeatConfig::Never,
                                    stagger: None,
                                }
                            >
                                "Scale on Hover"
                            </MotionDiv>

                            <MotionDiv
                                class="w-full h-32 bg-gradient-to-br from-orange-500 to-red-600 rounded-xl cursor-pointer flex items-center justify-center text-white font-bold text-lg".to_string()
                                initial=(move || {
                                    let mut initial = HashMap::new();
                                    initial.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
                                    initial
                                })()
                                while_hover=(move || {
                                    let mut hover = HashMap::new();
                                    hover.insert("rotate".to_string(), AnimationValue::Degrees(3.0));
                                    hover
                                })()
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseOut,
                                    delay: None,
                                    repeat: RepeatConfig::Never,
                                    stagger: None,
                                }
                            >
                                "Rotate on Hover"
                            </MotionDiv>

                            <MotionDiv
                                class="w-full h-32 bg-gradient-to-br from-pink-500 to-purple-600 rounded-xl cursor-pointer flex items-center justify-center text-white font-bold text-lg".to_string()
                                initial=(move || {
                                    let mut initial = HashMap::new();
                                    initial.insert("y".to_string(), AnimationValue::Pixels(0.0));
                                    initial
                                })()
                                while_hover=(move || {
                                    let mut hover = HashMap::new();
                                    hover.insert("y".to_string(), AnimationValue::Pixels(-8.0));
                                    hover
                                })()
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseOut,
                                    delay: None,
                                    repeat: RepeatConfig::Never,
                                    stagger: None,
                                }
                            >
                                "Lift on Hover"
                            </MotionDiv>
                        </div>
                    </div>

                    // Click Animation Demo
                    <div class="bg-gray-800/50 backdrop-blur-sm rounded-2xl p-8 border border-gray-700">
                        <div class="text-center mb-8">
                            <h2 class="text-3xl font-bold mb-4">"Click Animation"</h2>
                            <p class="text-gray-300 mb-6">
                                "Interactive click animations with counter"
                            </p>
                        </div>

                        <div class="flex flex-col items-center space-y-6">
                            <MotionDiv
                                class="w-40 h-40 bg-gradient-to-br from-yellow-500 to-orange-600 rounded-full cursor-pointer flex items-center justify-center text-white font-bold text-2xl".to_string()
                                initial=(move || {
                                    let mut initial = HashMap::new();
                                    initial.insert("scale".to_string(), AnimationValue::Number(1.0));
                                    initial
                                })()
                                while_tap=(move || {
                                    let mut tap = HashMap::new();
                                    tap.insert("scale".to_string(), AnimationValue::Number(0.95));
                                    tap
                                })()
                                transition=Transition {
                                    duration: Some(0.1),
                                    ease: Easing::EaseOut,
                                    delay: None,
                                    repeat: RepeatConfig::Never,
                                    stagger: None,
                                }
                                on:click=move |_| set_click_count.set(click_count.get() + 1)
                            >
                                {move || click_count.get()}
                            </MotionDiv>

                            <div class="text-center">
                                <p class="text-gray-300 mb-4">"Click the circle to increment the counter"</p>
                                <button
                                    class="px-6 py-3 bg-yellow-600 text-white rounded-lg font-medium hover:bg-yellow-700 transition-colors duration-200"
                                    on:click=move |_| set_click_count.set(0)
                                >
                                    "Reset Counter"
                                </button>
                            </div>
                        </div>
                    </div>

                    // Layout Demo
                    <div class="bg-gray-800/50 backdrop-blur-sm rounded-2xl p-8 border border-gray-700">
                        <div class="text-center mb-8">
                            <h2 class="text-3xl font-bold mb-4">"Layout Demo"</h2>
                            <p class="text-gray-300 mb-6">
                                "Dynamic list with add/remove animations"
                            </p>
                        </div>

                        <div class="flex justify-center gap-4 mb-8">
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
                        </div>

                        <div class="max-w-md mx-auto">
                            <div class="space-y-3">
                                <For
                                    each=move || items.get()
                                    key=|item| item.clone()
                                    children=move |item| {
                                        let item_clone = item.clone();
                                        view! {
                                            <MotionDiv
                                                class="bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg p-4 text-white font-medium cursor-pointer".to_string()
                                                initial=(move || {
                                                    let mut initial = HashMap::new();
                                                    initial.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                                    initial.insert("y".to_string(), AnimationValue::Pixels(-20.0));
                                                    initial
                                                })()
                                                animate=(move || {
                                                    let mut animate = HashMap::new();
                                                    animate.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                                    animate.insert("y".to_string(), AnimationValue::Pixels(0.0));
                                                    animate
                                                })()
                                                while_hover=(move || {
                                                    let mut hover = HashMap::new();
                                                    hover.insert("scale".to_string(), AnimationValue::Number(1.05));
                                                    hover
                                                })()
                                                transition=Transition {
                                                    duration: Some(0.3),
                                                    ease: Easing::EaseOut,
                                                    delay: None,
                                                    repeat: RepeatConfig::Never,
                                                    stagger: None,
                                                }
                                                on:click=move |_| {
                                                    if let Some(index) = items.get().iter().position(|i| i == &item_clone) {
                                                        remove_item(index);
                                                    }
                                                }
                                            >
                                                <div class="flex justify-between items-center">
                                                    <span>{item}</span>
                                                    <span class="text-white/70 hover:text-white transition-colors duration-200">"×"</span>
                                                </div>
                                            </MotionDiv>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    </div>
                </main>

                <footer class="text-center mt-12 text-gray-400">
                    <p>"Built with Leptos Motion - Rust-powered animations for the web"</p>
                </footer>
            </div>
        </div>
    }
}
