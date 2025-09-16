use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

/// Interactive Showcase - A simple working demo
#[component]
pub fn InteractiveShowcase() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 p-8">
            <div class="max-w-6xl mx-auto">
                <h1 class="text-4xl font-bold text-white text-center mb-8">
                    "🎭 Interactive Motion Showcase"
                </h1>
                <p class="text-center text-white/80 mb-8 text-lg">
                    "Experience the power of leptos-motion with interactive examples"
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // Gesture Interactions
                    <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                        <h3 class="text-xl font-bold text-white mb-4">"Gesture Interactions"</h3>
                        <div class="space-y-4">
                            <div class="relative h-32 bg-black/20 rounded-lg overflow-hidden">
                                <MotionDiv
                                    class="absolute w-8 h-8 bg-gradient-to-r from-pink-500 to-purple-500 rounded-full cursor-grab active:cursor-grabbing shadow-lg".to_string()
                                    style="left: 50%; top: 50%; margin-left: -16px; margin-top: -16px;".to_string()
                                    while_hover={
                                        let mut target = HashMap::new();
                                        target.insert("scale".to_string(), AnimationValue::Number(1.2));
                                        target
                                    }
                                    while_tap={
                                        let mut target = HashMap::new();
                                        target.insert("scale".to_string(), AnimationValue::Number(0.9));
                                        target
                                    }
                                    transition=Transition {
                                        duration: Some(0.2),
                                        ease: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                >
                                    ""
                                </MotionDiv>
                            </div>
                            <p class="text-white/80 text-sm">"Drag and drop with hover effects"</p>
                        </div>
                    </div>

                    // Hover Effects
                    <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                        <h3 class="text-xl font-bold text-white mb-4">"Hover Effects"</h3>
                        <div class="space-y-4">
                            <div class="flex justify-center items-center h-32">
                                <MotionDiv
                                    class="w-16 h-16 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg cursor-pointer shadow-lg".to_string()
                                    while_hover={
                                        let mut target = HashMap::new();
                                        target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                        target.insert("rotate".to_string(), AnimationValue::Number(45.0));
                                        target.insert("y".to_string(), AnimationValue::Pixels(-10.0));
                                        target
                                    }
                                    transition=Transition {
                                        duration: Some(0.3),
                                        ease: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                >
                                    ""
                                </MotionDiv>
                            </div>
                            <p class="text-white/80 text-sm">"Hover to see scale, rotation, and movement"</p>
                        </div>
                    </div>

                    // Layout Animations
                    <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                        <h3 class="text-xl font-bold text-white mb-4">"Layout Animations"</h3>
                        <div class="space-y-4">
                            <div class="grid grid-cols-2 gap-4">
                                <MotionDiv
                                    class="bg-gradient-to-r from-green-500 to-emerald-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
                                    initial={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                        target
                                    }
                                    animate={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                        target
                                    }
                                    _layout=true
                                    transition=Transition {
                                        duration: Some(0.3),
                                        ease: Easing::EaseOut,
                                        delay: Some(0.1),
                                        ..Default::default()
                                    }
                                >
                                    "Item 1"
                                </MotionDiv>
                                <MotionDiv
                                    class="bg-gradient-to-r from-green-500 to-emerald-500 p-4 rounded-lg text-white font-semibold text-center shadow-lg".to_string()
                                    initial={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(0.8));
                                        target
                                    }
                                    animate={
                                        let mut target = HashMap::new();
                                        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
                                        target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                        target
                                    }
                                    _layout=true
                                    transition=Transition {
                                        duration: Some(0.3),
                                        ease: Easing::EaseOut,
                                        delay: Some(0.2),
                                        ..Default::default()
                                    }
                                >
                                    "Item 2"
                                </MotionDiv>
                            </div>
                            <p class="text-white/80 text-sm">"Staggered layout animations"</p>
                        </div>
                    </div>

                    // Physics Simulations
                    <div class="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
                        <h3 class="text-xl font-bold text-white mb-4">"Physics Simulations"</h3>
                        <div class="space-y-4">
                            <div class="flex justify-center items-center h-32">
                                <MotionDiv
                                    class="w-12 h-12 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-full shadow-lg".to_string()
                                    animate={
                                        let mut target = HashMap::new();
                                        target.insert("y".to_string(), AnimationValue::Pixels(100.0));
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
                                    ""
                                </MotionDiv>
                            </div>
                            <p class="text-white/80 text-sm">"Continuous physics animation"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}