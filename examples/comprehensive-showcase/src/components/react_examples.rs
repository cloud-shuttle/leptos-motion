//! React-style component examples
//! 
//! This module contains React-style component examples that demonstrate
//! various UI patterns with motion animations.

use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

#[component]
pub fn ReactExamples() -> impl IntoView {
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
                    <MotionDiv
                        class="w-64 h-48 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg shadow-lg".to_string()
                        initial={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(0.8));
                            target
                        }
                        animate={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(1.0));
                            target
                        }
                        transition=Transition {
                            duration: Some(0.5),
                            ease: Easing::EaseOut,
                            delay: None,
                            repeat: RepeatConfig::Never,
                            stagger: None,
                        }
                        while_hover={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(1.05));
                            target
                        }
                        while_tap={
                            let mut target = HashMap::new();
                            target.insert("scale".to_string(), AnimationValue::Number(0.95));
                            target
                        }
                    >
                        {move || match active_component.get() {
                            0 => "Button Component",
                            1 => "Card Component",
                            2 => "Modal Component",
                            3 => "Loader Component",
                            _ => "Component"
                        }}
                    </MotionDiv>
                </div>
            </div>
            
            <div class="mt-6 text-center">
                <p class="text-gray-400 text-sm">
                    "Click on different components to see them in action. Each component demonstrates different animation patterns and interaction states."
                </p>
            </div>
        </div>
    }
}
