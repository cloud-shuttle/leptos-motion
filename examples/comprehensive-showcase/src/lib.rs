//! Comprehensive Showcase - Professional motion library examples
//!
//! This demo showcases comprehensive examples like those you'd find 
//! in professional motion libraries (similar to Framer Motion's examples)

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Initialize the panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
}

#[wasm_bindgen]
pub fn ComprehensiveShowcase() {
    // Mount to body - this works!
    let _ = leptos::mount::mount_to_body(|| view! { <ShowcaseComponent /> });
}

#[component]
fn ShowcaseComponent() -> impl IntoView {
    let (current_example, set_current_example) = signal(0);

    let next_example = move |_| {
        set_current_example.update(|i| *i = (*i + 1) % 9);
    };

    let prev_example = move |_| {
        set_current_example.update(|i| *i = if *i == 0 { 8 } else { *i - 1 });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white">
            <div class="container mx-auto px-4 py-8">
                // Header
                <header class="text-center mb-12">
                    <h1 class="text-6xl font-bold mb-4 bg-gradient-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
                        "🎨 Leptos Motion"
                    </h1>
                    <p class="text-2xl text-gray-300 mb-8">
                        "Professional Motion Library Showcase"
                    </p>
                    <p class="text-lg text-gray-400 mb-8">
                        "Examples & Tutorials - Unlock all examples like in Framer Motion"
                    </p>
                    
                    // Navigation
                    <div class="flex justify-center items-center gap-8 mb-8">
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-gray-600 to-gray-700 text-white rounded-lg font-medium hover:from-gray-700 hover:to-gray-800 transition-all duration-200 transform hover:scale-105"
                            on:click=prev_example
                        >
                            "← Prev"
                        </button>
                        
                        <div class="bg-white/10 backdrop-blur-sm rounded-xl px-8 py-4 border border-white/20">
                            <h2 class="text-xl font-bold text-yellow-400">
                                {move || match current_example.get() {
                                    0 => "React Components",
                                    1 => "Apple Watch",
                                    2 => "Source Unlock",
                                    3 => "Motion Gallery",
                                    4 => "Interactive Demo",
                                    5 => "CSS Generation",
                                    6 => "Path Drawing",
                                    7 => "Conic Gradient",
                                    8 => "Drag Transform",
                                    _ => "Unknown"
                                }}
                            </h2>
                            <p class="text-sm text-gray-400">
                                {move || format!("Example {} of 9", current_example.get() + 1)}
                            </p>
                        </div>
                        
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-gray-600 to-gray-700 text-white rounded-lg font-medium hover:from-gray-700 hover:to-gray-800 transition-all duration-200 transform hover:scale-105"
                            on:click=next_example
                        >
                            "Next →"
                        </button>
                    </div>
                </header>

                // Example Content
                <div class="max-w-6xl mx-auto">
                    <Show when=move || current_example.get() == 0>
                        <ReactExamples />
                    </Show>
                    <Show when=move || current_example.get() == 1>
                        <AppleWatchDemo />
                    </Show>
                    <Show when=move || current_example.get() == 2>
                        <SourceUnlockDemo />
                    </Show>
                    <Show when=move || current_example.get() == 3>
                        <MotionGallery />
                    </Show>
                    <Show when=move || current_example.get() == 4>
                        <InteractiveDemo />
                    </Show>
                    <Show when=move || current_example.get() == 5>
                        <CssGenerationExample />
                    </Show>
                    <Show when=move || current_example.get() == 6>
                        <PathDrawingExample />
                    </Show>
                    <Show when=move || current_example.get() == 7>
                        <ConicGradientExample />
                    </Show>
                    <Show when=move || current_example.get() == 8>
                        <DragTransformExample />
                    </Show>
                </div>

                // Footer
                <footer class="text-center mt-16 text-gray-400">
                    <p>"Built with Leptos Motion - Professional animations for modern web apps"</p>
                    <p class="text-sm mt-2">"🚀 Next-generation motion library for Rust and WebAssembly"</p>
                </footer>
            </div>
        </div>
    }
}

#[component]
fn ReactExamples() -> impl IntoView {
    let (active_component, set_active_component) = signal(0);

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-blue-400">"📱 React-Style Components"</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                // Component List
                <div class="space-y-4">
                    <h4 class="text-xl font-semibold mb-4">"Available Components"</h4>
                    
                    <div
                        class=move || if active_component.get() == 0 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-blue-500/20 border border-blue-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_active_component.set(0)
                    >
                        <h5 class="font-semibold">"Button"</h5>
                        <p class="text-sm text-gray-400">"A beautiful animated button component"</p>
                    </div>
                    
                    <div
                        class=move || if active_component.get() == 1 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-blue-500/20 border border-blue-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_active_component.set(1)
                    >
                        <h5 class="font-semibold">"Card"</h5>
                        <p class="text-sm text-gray-400">"Interactive card with hover effects"</p>
                    </div>
                    
                    <div
                        class=move || if active_component.get() == 2 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-blue-500/20 border border-blue-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_active_component.set(2)
                    >
                        <h5 class="font-semibold">"Modal"</h5>
                        <p class="text-sm text-gray-400">"Smooth modal with backdrop blur"</p>
                    </div>
                    
                    <div
                        class=move || if active_component.get() == 3 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-blue-500/20 border border-blue-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_active_component.set(3)
                    >
                        <h5 class="font-semibold">"Loader"</h5>
                        <p class="text-sm text-gray-400">"Animated loading spinner"</p>
                    </div>
                </div>
                
                // Component Preview
                <div class="flex items-center justify-center">
                    <div class="w-64 h-48 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-105">
                        {move || match active_component.get() {
                            0 => "Button Component",
                            1 => "Card Component",
                            2 => "Modal Component",
                            3 => "Loader Component",
                            _ => "Component"
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AppleWatchDemo() -> impl IntoView {
    let (selected_app, set_selected_app) = signal(0);

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-green-400">"⌚ Apple Watch Home Screen"</h3>
            
            <div class="flex justify-center">
                <div class="w-80 h-80 bg-black rounded-3xl p-6 border-4 border-gray-600">
                    <div class="grid grid-cols-4 gap-3 h-full">
                        
                        <div
                            class=move || if selected_app.get() == 0 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(0)
                        >
                            <span class="text-2xl">"📱"</span>
                            <span class="text-xs text-center mt-1">"Phone"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 1 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(1)
                        >
                            <span class="text-2xl">"💬"</span>
                            <span class="text-xs text-center mt-1">"Messages"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 2 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(2)
                        >
                            <span class="text-2xl">"📧"</span>
                            <span class="text-xs text-center mt-1">"Mail"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 3 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(3)
                        >
                            <span class="text-2xl">"📅"</span>
                            <span class="text-xs text-center mt-1">"Calendar"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 4 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(4)
                        >
                            <span class="text-2xl">"🎵"</span>
                            <span class="text-xs text-center mt-1">"Music"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 5 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(5)
                        >
                            <span class="text-2xl">"📷"</span>
                            <span class="text-xs text-center mt-1">"Camera"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 6 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(6)
                        >
                            <span class="text-2xl">"⚙️"</span>
                            <span class="text-xs text-center mt-1">"Settings"</span>
                        </div>
                        
                        <div
                            class=move || if selected_app.get() == 7 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer transition-all duration-200 hover:scale-110" }
                            on:click=move |_| set_selected_app.set(7)
                        >
                            <span class="text-2xl">"🏃"</span>
                            <span class="text-xs text-center mt-1">"Fitness"</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="text-center mt-6">
                <p class="text-gray-400">
                    "Selected: " {move || match selected_app.get() {
                        0 => "Phone",
                        1 => "Messages",
                        2 => "Mail",
                        3 => "Calendar",
                        4 => "Music",
                        5 => "Camera",
                        6 => "Settings",
                        7 => "Fitness",
                        _ => "Unknown"
                    }}
                </p>
            </div>
        </div>
    }
}

#[component]
fn SourceUnlockDemo() -> impl IntoView {
    let (is_unlocked, set_is_unlocked) = signal(false);

    let unlock_source = move || {
        set_is_unlocked.set(true);
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-purple-400">"🔓 Source Code Unlock"</h3>
            
            <Show when=move || is_unlocked.get()>
                <div class="text-center">
                    <div class="text-6xl mb-4">"🔓"</div>
                    <h4 class="text-2xl font-bold mb-4 text-green-400">"Source Unlocked!"</h4>
                    <div class="bg-gray-900 rounded-lg p-6 text-left font-mono text-sm">
                        <div class="text-green-400">"// Leptos Motion Component"</div>
                        <div class="text-blue-400">"use leptos::prelude::*;"</div>
                        <div class="text-blue-400">"use leptos_motion::*;"</div>
                        <br/>
                        <div class="text-yellow-400">"#[component]"</div>
                        <div class="text-white">"fn AnimatedButton() -> impl IntoView {"</div>
                        <div class="text-white ml-4">"view! {"</div>
                        <div class="text-white ml-8">"<MotionDiv"</div>
                        <div class="text-white ml-12">"class=\"btn\""</div>
                        <div class="text-white ml-12">"while_hover=|| {{"</div>
                        <div class="text-white ml-16">"scale: 1.1,"</div>
                        <div class="text-white ml-16">"rotate: 5.0"</div>
                        <div class="text-white ml-12">"}}"</div>
                        <div class="text-white ml-8">">"</div>
                        <div class="text-white ml-12">"Click me!"</div>
                        <div class="text-white ml-8">"</MotionDiv>"</div>
                        <div class="text-white ml-4">"}"</div>
                        <div class="text-white">"}"</div>
                    </div>
                </div>
            </Show>
            
            <Show when=move || !is_unlocked.get()>
                <div class="text-center">
                    <div class="text-6xl mb-4">"🔒"</div>
                    <h4 class="text-2xl font-bold mb-4">"Source Code Locked"</h4>
                    <p class="text-gray-400 mb-6">"Click the button below to unlock the source code"</p>
                    <button
                        class="px-8 py-4 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg font-medium hover:from-purple-600 hover:to-pink-600 transition-all duration-200 transform hover:scale-105"
                        on:click=move |_| unlock_source()
                    >
                        "🔓 Unlock Source"
                    </button>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn MotionGallery() -> impl IntoView {
    let (selected_animation, set_selected_animation) = signal(0);

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-pink-400">"🎨 Motion Gallery"</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                // Animation List
                <div class="space-y-4">
                    <h4 class="text-xl font-semibold mb-4">"Available Animations"</h4>
                    
                    <div
                        class=move || if selected_animation.get() == 0 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(0)
                    >
                        <h5 class="font-semibold">"Fade In"</h5>
                        <p class="text-sm text-gray-400">"opacity: 0 → 1"</p>
                    </div>
                    
                    <div
                        class=move || if selected_animation.get() == 1 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(1)
                    >
                        <h5 class="font-semibold">"Slide Up"</h5>
                        <p class="text-sm text-gray-400">"transform: translateY(100px) → 0"</p>
                    </div>
                    
                    <div
                        class=move || if selected_animation.get() == 2 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(2)
                    >
                        <h5 class="font-semibold">"Scale"</h5>
                        <p class="text-sm text-gray-400">"transform: scale(0) → 1"</p>
                    </div>
                    
                    <div
                        class=move || if selected_animation.get() == 3 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(3)
                    >
                        <h5 class="font-semibold">"Rotate"</h5>
                        <p class="text-sm text-gray-400">"transform: rotate(0deg) → 360deg"</p>
                    </div>
                    
                    <div
                        class=move || if selected_animation.get() == 4 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(4)
                    >
                        <h5 class="font-semibold">"Bounce"</h5>
                        <p class="text-sm text-gray-400">"easing: bounce"</p>
                    </div>
                    
                    <div
                        class=move || if selected_animation.get() == 5 { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-pink-500/20 border border-pink-400" } else { "p-4 rounded-lg cursor-pointer transition-all duration-200 bg-white/5 hover:bg-white/10" }
                        on:click=move |_| set_selected_animation.set(5)
                    >
                        <h5 class="font-semibold">"Elastic"</h5>
                        <p class="text-sm text-gray-400">"easing: elastic"</p>
                    </div>
                </div>
                
                // Animation Preview
                <div class="flex items-center justify-center">
                    <div class="w-32 h-32 bg-gradient-to-br from-pink-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg animate-pulse">
                        {move || match selected_animation.get() {
                            0 => "Fade In",
                            1 => "Slide Up",
                            2 => "Scale",
                            3 => "Rotate",
                            4 => "Bounce",
                            5 => "Elastic",
                            _ => "Animation"
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn InteractiveDemo() -> impl IntoView {
    let (score, set_score) = signal(0);

    let handle_click = move || {
        set_score.update(|s| *s += 10);
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-yellow-400">"🎮 Interactive Demo"</h3>
            
            <div class="text-center mb-6">
                <div class="bg-white/10 rounded-lg px-6 py-3 inline-block">
                    <span class="text-lg font-bold">"Score: " {score}</span>
                </div>
            </div>
            
            <div class="grid grid-cols-4 gap-4 mb-6">
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🎯"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🚀"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "⭐"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🎮"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🏆"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "💎"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🎪"
                </div>
                
                <div
                    class="w-16 h-16 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl cursor-pointer shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:rotate-3 active:scale-95"
                    on:click=move |_| handle_click()
                >
                    "🎨"
                </div>
            </div>
            
            <div class="text-center">
                <button
                    class="px-6 py-3 bg-gradient-to-r from-yellow-500 to-orange-600 text-white rounded-lg font-medium hover:from-yellow-600 hover:to-orange-700 transition-all duration-200 transform hover:scale-105"
                    on:click=move |_| set_score.set(0)
                >
                    "🎲 Reset Score"
                </button>
            </div>
            
            <div class="mt-8 text-center">
                <h4 class="text-lg font-semibold mb-4">"How to Play"</h4>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm text-gray-300">
                    <div class="flex items-center gap-2">
                        <span>"🖱️"</span>
                        <span>"Click items to score"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <span>"🎯"</span>
                        <span>"Watch animations"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <span>"⚡"</span>
                        <span>"Hover for effects"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <span>"🎲"</span>
                        <span>"Each click +10 points"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CssGenerationExample() -> impl IntoView {
    let (state, set_state) = signal(false);
    
    let toggle_state = move |_| {
        set_state.update(|s| *s = !*s);
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-cyan-400">"🎨 CSS Generation Example"</h3>
            
            <div class="example-container">
                <div 
                    class="box" 
                    data-state=move || if state.get() { "true" } else { "false" }
                />
                <button on:click=toggle_state>"Toggle position"</button>

                <style>
                    {r#"
                        .example-container {
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            gap: 20px;
                            padding: 20px;
                        }

                        .example-container .box {
                            width: 100px;
                            height: 100px;
                            background-color: #8df0cc;
                            border-radius: 10px;
                            transition: transform 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
                            transform: translateX(-100%);
                        }

                        .example-container .box[data-state="true"] {
                            transform: translateX(100%) rotate(180deg);
                        }

                        .example-container button {
                            background-color: #8df0cc;
                            color: #0f1115;
                            border-radius: 5px;
                            padding: 10px;
                            margin: 10px;
                            border: none;
                            cursor: pointer;
                            font-weight: bold;
                        }

                        .example-container button:hover {
                            background-color: #7ae0bc;
                            transform: scale(1.05);
                        }
                    "#}
                </style>
            </div>
            
            <div class="mt-8 text-center">
                <h4 class="text-lg font-semibold mb-4 text-cyan-300">"How it works"</h4>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-300">
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-cyan-400 font-semibold mb-2">"Spring Animation"</div>
                        <div>"CSS transition with cubic-bezier easing"</div>
                    </div>
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-cyan-400 font-semibold mb-2">"State-based Transform"</div>
                        <div>"Data attribute controls transform state"</div>
                    </div>
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-cyan-400 font-semibold mb-2">"Interactive Toggle"</div>
                        <div>"Click button to animate between states"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PathDrawingExample() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);
    
    let start_animation = move |_| {
        set_is_animated.set(true);
    };

    let reset_animation = move |_| {
        set_is_animated.set(false);
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-emerald-400">"🎨 Path Drawing Animation"</h3>
            
            <div class="flex flex-col items-center gap-6">
                // Control buttons
                <div class="flex gap-4">
                    <button 
                        on:click=start_animation
                        class="px-6 py-3 bg-gradient-to-r from-emerald-500 to-teal-600 text-white rounded-lg font-medium hover:from-emerald-600 hover:to-teal-700 transition-all duration-200 transform hover:scale-105"
                    >
                        "🎬 Start Animation"
                    </button>
                    <button 
                        on:click=reset_animation
                        class="px-6 py-3 bg-gradient-to-r from-gray-500 to-gray-600 text-white rounded-lg font-medium hover:from-gray-600 hover:to-gray-700 transition-all duration-200 transform hover:scale-105"
                    >
                        "🔄 Reset"
                    </button>
                </div>

                // SVG Animation
                <div class="bg-black/20 rounded-xl p-4">
                    <svg
                        width="600"
                        height="600"
                        viewBox="0 0 600 600"
                        class="max-w-[80vw] max-h-[60vh]"
                    >
                        // Row 1
                        <circle
                            cx="100"
                            cy="100"
                            r="80"
                            stroke="#ff0088"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-circle-1" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="30"
                            x2="360"
                            y2="170"
                            stroke="#8df0cc"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-1" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="170"
                            x2="360"
                            y2="30"
                            stroke="#8df0cc"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-2" } else { "" }
                        />
                        <rect
                            width="140"
                            height="140"
                            x="410"
                            y="30"
                            rx="20"
                            stroke="#0d63f8"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-rect-1" } else { "" }
                        />
                        
                        // Row 2
                        <circle
                            cx="100"
                            cy="300"
                            r="80"
                            stroke="#0d63f8"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-circle-2" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="230"
                            x2="360"
                            y2="370"
                            stroke="#ff0088"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-3" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="370"
                            x2="360"
                            y2="230"
                            stroke="#ff0088"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-4" } else { "" }
                        />
                        <rect
                            width="140"
                            height="140"
                            x="410"
                            y="230"
                            rx="20"
                            stroke="#8df0cc"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-rect-2" } else { "" }
                        />
                        
                        // Row 3
                        <circle
                            cx="100"
                            cy="500"
                            r="80"
                            stroke="#8df0cc"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-circle-3" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="430"
                            x2="360"
                            y2="570"
                            stroke="#0d63f8"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-5" } else { "" }
                        />
                        <line
                            x1="220"
                            y1="570"
                            x2="360"
                            y2="430"
                            stroke="#0d63f8"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-line-6" } else { "" }
                        />
                        <rect
                            width="140"
                            height="140"
                            x="410"
                            y="430"
                            rx="20"
                            stroke="#ff0088"
                            fill="transparent"
                            stroke-width="10"
                            stroke-linecap="round"
                            class=move || if is_animated.get() { "animate-draw-rect-3" } else { "" }
                        />
                    </svg>
                </div>

                // Animation styles
                <style>
                    {r#"
                        @keyframes draw-path {
                            from {
                                stroke-dasharray: 1000;
                                stroke-dashoffset: 1000;
                                opacity: 0;
                            }
                            to {
                                stroke-dasharray: 1000;
                                stroke-dashoffset: 0;
                                opacity: 1;
                            }
                        }

                        .animate-draw-circle-1 {
                            animation: draw-path 1.5s ease-out 0.5s both;
                        }

                        .animate-draw-line-1 {
                            animation: draw-path 1.5s ease-out 1.0s both;
                        }

                        .animate-draw-line-2 {
                            animation: draw-path 1.5s ease-out 1.25s both;
                        }

                        .animate-draw-rect-1 {
                            animation: draw-path 1.5s ease-out 1.5s both;
                        }

                        .animate-draw-circle-2 {
                            animation: draw-path 1.5s ease-out 1.0s both;
                        }

                        .animate-draw-line-3 {
                            animation: draw-path 1.5s ease-out 1.5s both;
                        }

                        .animate-draw-line-4 {
                            animation: draw-path 1.5s ease-out 1.75s both;
                        }

                        .animate-draw-rect-2 {
                            animation: draw-path 1.5s ease-out 2.0s both;
                        }

                        .animate-draw-circle-3 {
                            animation: draw-path 1.5s ease-out 1.5s both;
                        }

                        .animate-draw-line-5 {
                            animation: draw-path 1.5s ease-out 2.0s both;
                        }

                        .animate-draw-line-6 {
                            animation: draw-path 1.5s ease-out 2.25s both;
                        }

                        .animate-draw-rect-3 {
                            animation: draw-path 1.5s ease-out 2.5s both;
                        }
                    "#}
                </style>
            </div>
            
            <div class="mt-8 text-center">
                <h4 class="text-lg font-semibold mb-4 text-emerald-300">"How it works"</h4>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-300">
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-emerald-400 font-semibold mb-2">"SVG Path Drawing"</div>
                        <div>"Uses stroke-dasharray and stroke-dashoffset"</div>
                    </div>
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-emerald-400 font-semibold mb-2">"Staggered Animation"</div>
                        <div>"Each shape animates with different delays"</div>
                    </div>
                    <div class="bg-gray-800/50 rounded-lg p-4">
                        <div class="text-emerald-400 font-semibold mb-2">"Interactive Control"</div>
                        <div>"Start and reset the animation sequence"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ConicGradientExample() -> impl IntoView {
    let (gradient_x, set_gradient_x) = signal(0.5);
    let (gradient_y, set_gradient_y) = signal(0.5);
    let (dimensions, set_dimensions) = signal((0.0, 0.0, 0.0, 0.0)); // width, height, top, left

    let handle_pointer_move = move |event: web_sys::PointerEvent| {
        let (width, height, _top, _left) = dimensions.get();
        if width > 0.0 && height > 0.0 {
            set_gradient_x.set(event.client_x() as f64 / width);
            set_gradient_y.set(event.client_y() as f64 / height);
        }
    };

    let handle_pointer_enter = move |event: web_sys::PointerEvent| {
        if let Some(target) = event.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                let rect = element.get_bounding_client_rect();
                set_dimensions.set((
                    rect.width(),
                    rect.height(),
                    rect.top(),
                    rect.left(),
                ));
            }
        }
    };

    let background_style = move || {
        let (_width, _height, top, left) = dimensions.get();
        let x_percent = gradient_x.get() * 100.0;
        let y_percent = gradient_y.get() * 100.0;
        format!(
            "conic-gradient(from 0deg at calc({}% - {}px) calc({}% - {}px), #0cdcf7, #ff0088, #fff312, #0cdcf7)",
            x_percent, left, y_percent, top
        )
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-violet-400">"🌈 Conic Gradient Animation"</h3>
            
            <div class="flex flex-col items-center gap-6">
                // Interactive gradient container
                <div 
                    class="relative w-full h-96 flex items-center justify-center overflow-hidden rounded-xl"
                    on:pointermove=handle_pointer_move
                >
                    <div
                        class="w-96 h-96 rounded-full cursor-none shadow-2xl"
                        style:background=background_style
                        on:pointerenter=handle_pointer_enter
                    />
                </div>

                // Instructions
                <div class="text-center">
                    <h4 class="text-lg font-semibold mb-4 text-violet-300">"How it works"</h4>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-300">
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-violet-400 font-semibold mb-2">"Mouse Tracking"</div>
                            <div>"Follows your mouse movement"</div>
                        </div>
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-violet-400 font-semibold mb-2">"Conic Gradient"</div>
                            <div>"CSS conic-gradient with dynamic positioning"</div>
                        </div>
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-violet-400 font-semibold mb-2">"Real-time Updates"</div>
                            <div>"Smooth gradient center following cursor"</div>
                        </div>
                    </div>
                </div>

                // Color palette info
                <div class="text-center">
                    <h4 class="text-lg font-semibold mb-4 text-violet-300">"Color Palette"</h4>
                    <div class="flex justify-center gap-4">
                        <div class="flex items-center gap-2">
                            <div class="w-6 h-6 rounded-full bg-[#0cdcf7]"></div>
                            <span class="text-sm">"Cyan"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-6 h-6 rounded-full bg-[#ff0088]"></div>
                            <span class="text-sm">"Pink"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-6 h-6 rounded-full bg-[#fff312]"></div>
                            <span class="text-sm">"Yellow"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DragTransformExample() -> impl IntoView {
    let (x_position, set_x_position) = signal(0.0);
    let (is_dragging, set_is_dragging) = signal(false);

    let handle_drag = move |event: web_sys::PointerEvent| {
        if is_dragging.get() {
            let container_width = 500.0;
            let box_width = 140.0;
            let max_x = (container_width - box_width) / 2.0;
            let min_x = -max_x;
            
            let new_x = (event.client_x() as f64 - container_width / 2.0).clamp(min_x, max_x);
            set_x_position.set(new_x);
        }
    };

    let handle_drag_start = move |_| {
        set_is_dragging.set(true);
    };

    let handle_drag_end = move |_| {
        set_is_dragging.set(false);
    };

    // Transform functions based on x position
    let background_gradient = move || {
        let x = x_position.get();
        if x < -50.0 {
            "linear-gradient(180deg, #ff008c 0%, rgb(211, 9, 225) 100%)"
        } else if x > 50.0 {
            "linear-gradient(180deg, rgb(230, 255, 0) 0%, rgb(3, 209, 0) 100%)"
        } else {
            "linear-gradient(180deg, #7700ff 0%, rgb(68, 0, 255) 100%)"
        }
    };

    let stroke_color = move || {
        let x = x_position.get();
        if x < -50.0 {
            "rgb(211, 9, 225)"
        } else if x > 50.0 {
            "rgb(3, 209, 0)"
        } else {
            "rgb(68, 0, 255)"
        }
    };

    let tick_path_length = move || {
        let x = x_position.get();
        if x > 10.0 {
            ((x - 10.0) / 90.0).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };

    let cross_path_a_length = move || {
        let x = x_position.get();
        if x < -10.0 {
            ((-x - 10.0) / 45.0).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };

    let cross_path_b_length = move || {
        let x = x_position.get();
        if x < -50.0 {
            ((-x - 50.0) / 50.0).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-orange-400">"🎯 Drag Transform Animation"</h3>
            
            <div class="flex flex-col items-center gap-6">
                // Interactive drag container
                <div 
                    class="relative w-full max-w-lg h-80 flex items-center justify-center rounded-xl overflow-hidden"
                    style:background=background_gradient
                    on:pointermove=handle_drag
                    on:pointerup=handle_drag_end
                    on:pointerleave=handle_drag_end
                >
                    <div
                        class="absolute w-36 h-36 bg-gray-100 rounded-2xl p-5 cursor-grab active:cursor-grabbing shadow-lg"
                        style:transform=move || format!("translateX({}px)", x_position.get())
                        on:pointerdown=handle_drag_start
                    >
                        <svg class="w-full h-full" viewBox="0 0 50 50">
                            // Circle
                            <path
                                fill="none"
                                stroke-width="2"
                                stroke=stroke_color
                                d="M 0, 20 a 20, 20 0 1,0 40,0 a 20, 20 0 1,0 -40,0"
                                style:transform="translate(5px, 5px)"
                            />
                            // Tick (appears when dragged right)
                            <path
                                fill="none"
                                stroke-width="2"
                                stroke=stroke_color
                                d="M14,26 L 22,33 L 35,16"
                                stroke-dasharray="0 1"
                                style:stroke-dashoffset=move || format!("{}", 1.0 - tick_path_length())
                            />
                            // Cross A (appears when dragged left)
                            <path
                                fill="none"
                                stroke-width="2"
                                stroke=stroke_color
                                d="M17,17 L33,33"
                                stroke-dasharray="0 1"
                                style:stroke-dashoffset=move || format!("{}", 1.0 - cross_path_a_length())
                            />
                            // Cross B (appears when dragged far left)
                            <path
                                fill="none"
                                stroke-width="2"
                                stroke=stroke_color
                                d="M33,17 L17,33"
                                stroke-dasharray="0 1"
                                style:stroke-dashoffset=move || format!("{}", 1.0 - cross_path_b_length())
                            />
                        </svg>
                    </div>
                </div>

                // Instructions
                <div class="text-center">
                    <h4 class="text-lg font-semibold mb-4 text-orange-300">"How it works"</h4>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-300">
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-orange-400 font-semibold mb-2">"Drag Interaction"</div>
                            <div>"Drag the box left and right"</div>
                        </div>
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-orange-400 font-semibold mb-2">"Dynamic Gradients"</div>
                            <div>"Background changes based on position"</div>
                        </div>
                        <div class="bg-gray-800/50 rounded-lg p-4">
                            <div class="text-orange-400 font-semibold mb-2">"SVG Path Animation"</div>
                            <div>"Tick and cross paths animate"</div>
                        </div>
                    </div>
                </div>

                // Position indicator
                <div class="text-center">
                    <div class="bg-gray-800/50 rounded-lg px-4 py-2 inline-block">
                        <span class="text-orange-400 font-semibold">"Position: " {move || format!("{:.1}px", x_position.get())}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}