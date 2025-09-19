//! Apple Watch Home Screen Demo
//! 
//! This module contains the Apple Watch-style home screen demo with
//! animated app icons and selection states.

use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

#[component]
pub fn AppleWatchDemo() -> impl IntoView {
    let (selected_app, set_selected_app) = signal(0);

    view! {
        <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/20">
            <h3 class="text-3xl font-bold mb-6 text-center text-green-400">"⌚ Apple Watch Home Screen"</h3>
            
            <div class="flex justify-center">
                <div class="bg-black rounded-3xl p-6 shadow-2xl">
                    <div class="grid grid-cols-4 gap-3">
                        <MotionDiv
                            class=(move || if selected_app.get() == 0 { "w-16 h-16 bg-blue-500 rounded-2xl flex flex-col items-center justify-center cursor-pointer".to_string() } else { "w-16 h-16 bg-gray-800 rounded-2xl flex flex-col items-center justify-center cursor-pointer".to_string() })()
                            initial={
                                let mut target = HashMap::new();
                                target.insert("scale".to_string(), AnimationValue::Number(0.0));
                                target
                            }
                            animate={
                                let mut target = HashMap::new();
                                target.insert("scale".to_string(), AnimationValue::Number(1.0));
                                target
                            }
                            transition=Transition {
                                duration: Some(0.3),
                                ease: Easing::EaseOut,
                                delay: Some(0.1),
                                repeat: RepeatConfig::Never,
                                stagger: None,
                            }
                            while_hover={
                                let mut target = HashMap::new();
                                target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                target
                            }
                            while_tap={
                                let mut target = HashMap::new();
                                target.insert("scale".to_string(), AnimationValue::Number(0.9));
                                target
                            }
                            on:click=move |_| set_selected_app.set(0)
                        >
                            <span class="text-2xl">"📱"</span>
                            <span class="text-xs text-center mt-1">"Phone"</span>
                        </MotionDiv>
                        
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
