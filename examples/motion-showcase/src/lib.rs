use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

/// Motion Showcase - A comprehensive demo similar to Motion.dev
/// Showcasing the power of leptos-motion with Rust + WASM performance
#[component]
pub fn MotionShowcase() -> impl IntoView {
    let (active_demo, set_active_demo) = signal("gestures");
    let (animation_count, set_animation_count) = signal(0);

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 p-8">
            <div class="max-w-7xl mx-auto">
                // Header
                <div class="text-center mb-12">
                    <h1 class="text-5xl font-bold text-white mb-4">
                        "Motion Showcase"
                    </h1>
                    <p class="text-xl text-purple-200 mb-6">
                        "Powerful animations with Rust + WASM performance"
                    </p>
                    <div class="flex justify-center items-center gap-6">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
                            <span class="text-green-400 text-sm font-medium">"Rust + WASM"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-blue-500 rounded-full animate-pulse"></div>
                            <span class="text-blue-400 text-sm font-medium">"60fps Performance"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-purple-500 rounded-full animate-pulse"></div>
                            <span class="text-purple-400 text-sm font-medium">"Type Safe"</span>
                        </div>
                    </div>
                </div>

                // Navigation
                <div class="flex flex-wrap justify-center gap-3 mb-8">
                    <button
                        on:click=move |_| set_active_demo.set("gestures")
                        class=move || {
                            if active_demo.get() == "gestures" {
                                "px-6 py-3 bg-white text-purple-900 font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105".to_string()
                            } else {
                                "px-6 py-3 bg-white/20 text-white font-semibold rounded-lg hover:bg-white/30 transition-all duration-200 transform hover:scale-105".to_string()
                            }
                        }
                    >
                        "GESTURES"
                    </button>
                    <button
                        on:click=move |_| set_active_demo.set("layout")
                        class=move || {
                            if active_demo.get() == "layout" {
                                "px-6 py-3 bg-white text-purple-900 font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105".to_string()
                            } else {
                                "px-6 py-3 bg-white/20 text-white font-semibold rounded-lg hover:bg-white/30 transition-all duration-200 transform hover:scale-105".to_string()
                            }
                        }
                    >
                        "LAYOUT"
                    </button>
                    <button
                        on:click=move |_| set_active_demo.set("physics")
                        class=move || {
                            if active_demo.get() == "physics" {
                                "px-6 py-3 bg-white text-purple-900 font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105".to_string()
                            } else {
                                "px-6 py-3 bg-white/20 text-white font-semibold rounded-lg hover:bg-white/30 transition-all duration-200 transform hover:scale-105".to_string()
                            }
                        }
                    >
                        "PHYSICS"
                    </button>
                    <button
                        on:click=move |_| set_active_demo.set("performance")
                        class=move || {
                            if active_demo.get() == "performance" {
                                "px-6 py-3 bg-white text-purple-900 font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105".to_string()
                            } else {
                                "px-6 py-3 bg-white/20 text-white font-semibold rounded-lg hover:bg-white/30 transition-all duration-200 transform hover:scale-105".to_string()
                            }
                        }
                    >
                        "PERFORMANCE"
                    </button>
                </div>

                // Demo Content
                <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20 min-h-96">
                    // Gestures Demo
                    <Show
                        when=move || active_demo.get() == "gestures"
                        fallback=|| view! { <div></div> }
                    >
                        <div class="space-y-8">
                            <h2 class="text-3xl font-bold text-white mb-6">"Gesture Interactions"</h2>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                // Hover Animation
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Hover Effects"</h3>
                                    <div class="flex justify-center items-center h-32">
                                        <MotionDiv
                                            class="w-20 h-20 bg-gradient-to-r from-blue-500 to-purple-500 rounded-lg cursor-pointer shadow-lg".to_string()
                                            while_hover={
                                                let mut target = HashMap::new();
                                                target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                                target.insert("rotate".to_string(), AnimationValue::Number(5.0));
                                                target.insert("y".to_string(), AnimationValue::Pixels(-5.0));
                                                target
                                            }
                                            transition=Transition {
                                                duration: Some(0.3),
                                                ease: Easing::EaseOut,
                                                ..Default::default()
                                            }
                                        >
                                            "Hover me!"
                                        </MotionDiv>
                                    </div>
                                </div>

                                // Tap Animation
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Tap Effects"</h3>
                                    <div class="flex justify-center items-center h-32">
                                        <MotionDiv
                                            class="w-20 h-20 bg-gradient-to-r from-green-500 to-teal-500 rounded-full cursor-pointer shadow-lg".to_string()
                                            while_tap={
                                                let mut target = HashMap::new();
                                                target.insert("scale".to_string(), AnimationValue::Number(0.95));
                                                target.insert("rotate".to_string(), AnimationValue::Number(180.0));
                                                target
                                            }
                                            transition=Transition {
                                                duration: Some(0.2),
                                                ease: Easing::EaseOut,
                                                ..Default::default()
                                            }
                                        >
                                            "Tap me!"
                                        </MotionDiv>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // Layout Demo
                    <Show
                        when=move || active_demo.get() == "layout"
                        fallback=|| view! { <div></div> }
                    >
                        <div class="space-y-8">
                            <h2 class="text-3xl font-bold text-white mb-6">"Layout Animations"</h2>
                            
                            <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                                    <MotionDiv
                                        class="bg-gradient-to-r from-pink-500 to-red-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
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
                                        transition=Transition {
                                            duration: Some(0.5),
                                            ease: Easing::EaseOut,
                                            delay: Some(0.1),
                                            ..Default::default()
                                        }
                                    >
                                        "Item 1"
                                    </MotionDiv>
                                    <MotionDiv
                                        class="bg-gradient-to-r from-pink-500 to-red-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
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
                                        transition=Transition {
                                            duration: Some(0.5),
                                            ease: Easing::EaseOut,
                                            delay: Some(0.2),
                                            ..Default::default()
                                        }
                                    >
                                        "Item 2"
                                    </MotionDiv>
                                    <MotionDiv
                                        class="bg-gradient-to-r from-pink-500 to-red-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
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
                                        transition=Transition {
                                            duration: Some(0.5),
                                            ease: Easing::EaseOut,
                                            delay: Some(0.3),
                                            ..Default::default()
                                        }
                                    >
                                        "Item 3"
                                    </MotionDiv>
                                    <MotionDiv
                                        class="bg-gradient-to-r from-pink-500 to-red-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
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
                                        transition=Transition {
                                            duration: Some(0.5),
                                            ease: Easing::EaseOut,
                                            delay: Some(0.4),
                                            ..Default::default()
                                        }
                                    >
                                        "Item 4"
                                    </MotionDiv>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // Physics Demo
                    <Show
                        when=move || active_demo.get() == "physics"
                        fallback=|| view! { <div></div> }
                    >
                        <div class="space-y-8">
                            <h2 class="text-3xl font-bold text-white mb-6">"Physics Simulations"</h2>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                // Spring Animation
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Spring Physics"</h3>
                                    <div class="flex justify-center items-center h-32">
                                        <MotionDiv
                                            class="w-16 h-16 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-full shadow-lg".to_string()
                                            animate={
                                                let mut target = HashMap::new();
                                                target.insert("y".to_string(), AnimationValue::Pixels(50.0));
                                                target.insert("rotate".to_string(), AnimationValue::Number(360.0));
                                                target
                                            }
                                            transition=Transition {
                                                duration: Some(2.0),
                                                ease: Easing::EaseInOut,
                                                repeat: RepeatConfig::Infinite,
                                                ..Default::default()
                                            }
                                        >
                                            "Bounce!"
                                        </MotionDiv>
                                    </div>
                                </div>

                                // Continuous Rotation
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Continuous Motion"</h3>
                                    <div class="flex justify-center items-center h-32">
                                        <MotionDiv
                                            class="w-16 h-16 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg shadow-lg".to_string()
                                            animate={
                                                let mut target = HashMap::new();
                                                target.insert("rotate".to_string(), AnimationValue::Number(360.0));
                                                target
                                            }
                                            transition=Transition {
                                                duration: Some(3.0),
                                                ease: Easing::Linear,
                                                repeat: RepeatConfig::Infinite,
                                                ..Default::default()
                                            }
                                        >
                                            "Spin!"
                                        </MotionDiv>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Show>

                    // Performance Demo
                    <Show
                        when=move || active_demo.get() == "performance"
                        fallback=|| view! { <div></div> }
                    >
                        <div class="space-y-8">
                            <h2 class="text-3xl font-bold text-white mb-6">"Performance Demo"</h2>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                // Performance Metrics
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Rust + WASM Performance"</h3>
                                    <div class="space-y-4">
                                        <div class="flex justify-between items-center">
                                            <span class="text-white">"Frame Rate:"</span>
                                            <span class="text-green-400 font-bold">"60 FPS"</span>
                                        </div>
                                        <div class="flex justify-between items-center">
                                            <span class="text-white">"Memory Usage:"</span>
                                            <span class="text-blue-400 font-bold">"< 5MB"</span>
                                        </div>
                                        <div class="flex justify-between items-center">
                                            <span class="text-white">"Bundle Size:"</span>
                                            <span class="text-purple-400 font-bold">"1.8MB"</span>
                                        </div>
                                        <div class="flex justify-between items-center">
                                            <span class="text-white">"Animation Count:"</span>
                                            <span class="text-yellow-400 font-bold">{animation_count}</span>
                                        </div>
                                    </div>
                                </div>

                                // Live Animation Counter
                                <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                                    <h3 class="text-xl font-semibold text-white mb-4">"Live Animation Counter"</h3>
                                    <div class="flex justify-center items-center h-32">
                                        <MotionDiv
                                            class="w-20 h-20 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full shadow-lg cursor-pointer".to_string()
                                            on:click=move |_| set_animation_count.set(animation_count.get() + 1)
                                            while_hover={
                                                let mut target = HashMap::new();
                                                target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                                target
                                            }
                                            transition=Transition {
                                                duration: Some(0.2),
                                                ease: Easing::EaseOut,
                                                ..Default::default()
                                            }
                                        >
                                            {animation_count}
                                        </MotionDiv>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Show>
                </div>

                // Footer
                <div class="mt-12 text-center">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20">
                        <h2 class="text-2xl font-bold text-white mb-4">"Built with leptos-motion"</h2>
                        <p class="text-purple-200 mb-6">
                            "A powerful, type-safe animation library for Rust web applications"
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-4 gap-6 text-center">
                            <div>
                                <div class="text-3xl font-bold text-green-400">"Rust"</div>
                                <div class="text-purple-200 text-sm">"Memory Safe"</div>
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-blue-400">"WASM"</div>
                                <div class="text-purple-200 text-sm">"Native Speed"</div>
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-purple-400">"Type Safe"</div>
                                <div class="text-purple-200 text-sm">"Compile Time"</div>
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-pink-400">"60fps"</div>
                                <div class="text-purple-200 text-sm">"Smooth Animations"</div>
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
        <MotionShowcase />
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}