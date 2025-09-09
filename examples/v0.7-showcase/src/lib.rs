use leptos::prelude::*;
use leptos_motion_dom::{ReactiveMotionDivFixed, signal_animate, AnimationValue};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use leptos::web_sys;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Comprehensive Showcase starting".into());
    
    let _handle = mount_to_body(|| {
        web_sys::console::log_1(&"Comprehensive Showcase component rendering".into());
        view! {
            <div id="comprehensive-showcase">
                <ComprehensiveShowcase />
            </div>
        }
    });
    
    web_sys::console::log_1(&"Comprehensive Showcase mount completed".into());
}

#[component]
fn ComprehensiveShowcase() -> impl IntoView {
    web_sys::console::log_1(&"ComprehensiveShowcase component starting to render".into());
    
    let result = view! {
        <div class="showcase-container">
            <HeroSection />
            <FeaturesGrid />
            <CTASection />
        </div>
    };
    
    web_sys::console::log_1(&"ComprehensiveShowcase component finished rendering".into());
    result
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="hero-gradient text-white py-20">
            <div class="container mx-auto px-6 text-center">
                <h1 class="text-6xl font-bold mb-6">
                    "Animations that move"
                </h1>
                <p class="text-xl mb-8 max-w-3xl mx-auto">
                    "Create high-performance web animations with Leptos Motion's easy-to-use API — from simple transforms to advanced interactive gestures."
                </p>
                <div class="flex justify-center gap-4">
                    <button class="bg-white text-purple-600 px-8 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors">
                        "Get Started"
                    </button>
                    <button class="border-2 border-white text-white px-8 py-3 rounded-lg font-semibold hover:bg-white hover:text-purple-600 transition-colors">
                        "View Examples"
                    </button>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturesGrid() -> impl IntoView {
    view! {
        <section class="py-20">
            <div class="container mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <SimpleAPIDemo />
                    <IndependentTransformsDemo />
                    <ScrollAnimationDemo />
                    <SpringPhysicsDemo />
                    <ExitAnimationDemo />
                    <GesturesDemo />
                    <LayoutAnimationDemo />
                    <TimelineSequencesDemo />
                    <PerformanceDemo />
                </div>
            </div>
        </section>
    }
}

#[component]
fn SimpleAPIDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(&format!("Simple API triggered, is_active: {}", is_active.get()).into());
    };
    
    let animation_target = signal_animate(move || {
        let is_active_val = is_active.get();
        
        if is_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1.2) rotate(360deg)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1) rotate(0deg)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"🎯"</div>
            <h3 class="text-2xl font-bold mb-4">"Simple API"</h3>
            <p class="text-gray-600 mb-6">"Motion's pick-up-and-play API is easy to start and fun to master."</p>
            <div class="demo-container">
                <div class="flex justify-center items-center h-full">
                    <ReactiveMotionDivFixed
                        animate=animation_target
                        class="w-20 h-20 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer".to_string()
                        on:click=toggle
                    >
                        <div>"🎯"</div>
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn IndependentTransformsDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(&format!("Independent transforms triggered, is_active: {}", is_active.get()).into());
    };
    
    let animation_target = signal_animate(move || {
        let is_active_val = is_active.get();
        
        if is_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateX(50px) translateY(-30px) rotateZ(180deg) scale(1.3)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateX(0px) translateY(0px) rotateZ(0deg) scale(1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"🔄"</div>
            <h3 class="text-2xl font-bold mb-4">"Independent transforms"</h3>
            <p class="text-gray-600 mb-6">"Animate x, y, rotateZ etc independently, without wrapper elements."</p>
            <div class="demo-container">
                <div class="flex justify-center items-center h-full">
                    <ReactiveMotionDivFixed
                        animate=animation_target
                        class="w-20 h-20 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer".to_string()
                        on:click=toggle
                    >
                        <div>"🔄"</div>
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn ScrollAnimationDemo() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    
    let toggle = move |_| {
        set_is_visible.update(|visible| *visible = !*visible);
        web_sys::console::log_1(&format!("Scroll animation triggered, is_visible: {}", is_visible.get()).into());
    };
    
    let animation_target = signal_animate(move || {
        let is_visible_val = is_visible.get();
        
        if is_visible_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateY(0px) scale(1)".to_string())),
                ("opacity".to_string(), AnimationValue::String("1".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateY(50px) scale(0.8)".to_string())),
                ("opacity".to_string(), AnimationValue::String("0.5".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"📜"</div>
            <h3 class="text-2xl font-bold mb-4">"Scroll animation"</h3>
            <p class="text-gray-600 mb-6">"Smooth, hardware-accelerated scroll animations."</p>
            <div class="demo-container">
                <div class="flex justify-center items-center h-full">
                    <ReactiveMotionDivFixed
                        animate=animation_target
                        class="w-20 h-20 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer".to_string()
                        on:click=toggle
                    >
                        <div>"📜"</div>
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn SpringPhysicsDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(&format!("Spring physics triggered, is_active: {}", is_active.get()).into());
    };
    
    let spring_items = vec!["Spring 1", "Spring 2", "Spring 3", "Spring 4"];
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"🌊"</div>
            <h3 class="text-2xl font-bold mb-4">"Spring physics"</h3>
            <p class="text-gray-600 mb-6">"Real spring physics for great-feeling animations."</p>
            <div class="demo-container">
                <div class="spring-demo">
                    {spring_items.into_iter().enumerate().map(|(index, item)| {
                        let (is_spring_active, set_spring_active) = signal(false);
                        let spring_toggle = move |_| {
                            set_spring_active.update(|active| *active = !*active);
                        };
                        
                        let spring_animation = signal_animate(move || {
                            let is_spring_active_val = is_spring_active.get();
                            
                            if is_spring_active_val {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("translateY(-20px) scale(1.1)".to_string())),
                                    ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
                                    ("color".to_string(), AnimationValue::String("white".to_string())),
                                ])
                            } else {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("translateY(0px) scale(1)".to_string())),
                                    ("background".to_string(), AnimationValue::String("white".to_string())),
                                    ("color".to_string(), AnimationValue::String("black".to_string())),
                                ])
                            }
                        });
                        
                        view! {
                            <ReactiveMotionDivFixed
                                animate=spring_animation
                                class="spring-item cursor-pointer".to_string()
                                on:click=spring_toggle
                            >
                                {item}
                            </ReactiveMotionDivFixed>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn ExitAnimationDemo() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);
    
    let toggle = move |_| {
        set_is_visible.update(|visible| *visible = !*visible);
        web_sys::console::log_1(&format!("Exit animation triggered, is_visible: {}", is_visible.get()).into());
        
        // Auto-restore after animation
        if !is_visible.get() {
            let set_visible = set_is_visible.clone();
            let timeout = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Closure::wrap(Box::new(move || {
                        set_visible.set(true);
                    }) as Box<dyn Fn()>).as_ref().unchecked_ref(),
                    1000
                );
        }
    };
    
    let animation_target = signal_animate(move || {
        let is_visible_val = is_visible.get();
        
        if is_visible_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1) rotate(0deg)".to_string())),
                ("opacity".to_string(), AnimationValue::String("1".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(0) rotate(180deg)".to_string())),
                ("opacity".to_string(), AnimationValue::String("0".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"👋"</div>
            <h3 class="text-2xl font-bold mb-4">"Exit animation"</h3>
            <p class="text-gray-600 mb-6">"AnimatePresence makes it easy to animate elements as they exit."</p>
            <div class="demo-container">
                <div class="flex justify-center items-center h-full">
                    <ReactiveMotionDivFixed
                        animate=animation_target
                        class="w-20 h-20 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer".to_string()
                        on:click=toggle
                    >
                        <div>"👋"</div>
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn GesturesDemo() -> impl IntoView {
    let (hover_active, set_hover_active) = signal(false);
    let (press_active, set_press_active) = signal(false);
    let (drag_active, set_drag_active) = signal(false);
    
    let hover_toggle = move |_| {
        set_hover_active.update(|active| *active = !*active);
    };
    
    let press_toggle = move |_| {
        set_press_active.update(|active| *active = !*active);
    };
    
    let drag_toggle = move |_| {
        set_drag_active.update(|active| *active = !*active);
    };
    
    let hover_animation = signal_animate(move || {
        let hover_active_val = hover_active.get();
        
        if hover_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1.2)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string())),
            ])
        }
    });
    
    let press_animation = signal_animate(move || {
        let press_active_val = press_active.get();
        
        if press_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(0.9)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("scale(1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string())),
            ])
        }
    });
    
    let drag_animation = signal_animate(move || {
        let drag_active_val = drag_active.get();
        
        if drag_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateX(50px) translateY(-30px) scale(1.1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("translateX(0px) translateY(0px) scale(1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"✋"</div>
            <h3 class="text-2xl font-bold mb-4">"Gestures"</h3>
            <p class="text-gray-600 mb-6">"Hover, press and drag gestures that feel native, not \"webby\"."</p>
            <div class="demo-container">
                <div class="gesture-demo">
                    <ReactiveMotionDivFixed
                        animate=hover_animation
                        class="gesture-item".to_string()
                        on:click=hover_toggle
                    >
                        "Hover"
                    </ReactiveMotionDivFixed>
                    <ReactiveMotionDivFixed
                        animate=press_animation
                        class="gesture-item".to_string()
                        on:click=press_toggle
                    >
                        "Press"
                    </ReactiveMotionDivFixed>
                    <ReactiveMotionDivFixed
                        animate=drag_animation
                        class="gesture-item".to_string()
                        on:click=drag_toggle
                    >
                        "Drag"
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline">
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn LayoutAnimationDemo() -> impl IntoView {
    let (layout_mode, set_layout_mode) = signal(false);
    
    let toggle = move |_| {
        set_layout_mode.update(|mode| *mode = !*mode);
        web_sys::console::log_1(&format!("Layout animation triggered, layout_mode: {}", layout_mode.get()).into());
    };
    
    let layout_items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"📐"</div>
            <h3 class="text-2xl font-bold mb-4">"Layout animation"</h3>
            <p class="text-gray-600 mb-6">"Animate between different layouts with Motion's industry-leading layout animation engine."</p>
            <div class="demo-container">
                <div class="layout-demo">
                    {layout_items.into_iter().map(|item| {
                        let (is_layout_active, set_layout_active) = signal(false);
                        let layout_toggle = move |_| {
                            set_layout_active.update(|active| *active = !*active);
                        };
                        
                        let layout_animation = signal_animate(move || {
                            let is_layout_active_val = is_layout_active.get();
                            
                            if is_layout_active_val {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("scale(1.1)".to_string())),
                                    ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
                                ])
                            } else {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("scale(1)".to_string())),
                                    ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)".to_string())),
                                ])
                            }
                        });
                        
                        view! {
                            <ReactiveMotionDivFixed
                                animate=layout_animation
                                class="layout-item cursor-pointer".to_string()
                                on:click=layout_toggle
                            >
                                {item}
                            </ReactiveMotionDivFixed>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn TimelineSequencesDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(&format!("Timeline sequences triggered, is_active: {}", is_active.get()).into());
    };
    
    let timeline_items = vec!["1", "2", "3", "4", "5"];
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"⏱️"</div>
            <h3 class="text-2xl font-bold mb-4">"Timeline sequences"</h3>
            <p class="text-gray-600 mb-6">"Variants, stagger and timelines make it easy to precisely orchestrate animations."</p>
            <div class="demo-container">
                <div class="timeline-demo">
                    {timeline_items.into_iter().enumerate().map(|(index, item)| {
                        let (is_timeline_active, set_timeline_active) = signal(false);
                        let timeline_toggle = move |_| {
                            set_timeline_active.update(|active| *active = !*active);
                            web_sys::console::log_1(&format!("Timeline item {} clicked, active: {}", item, is_timeline_active.get()).into());
                        };
                        
                        let timeline_animation = signal_animate(move || {
                            let is_timeline_active_val = is_timeline_active.get();
                            
                            if is_timeline_active_val {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("scale(1.2) rotate(360deg) translateY(-10px)".to_string())),
                                    ("opacity".to_string(), AnimationValue::String("1".to_string())),
                                    ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string())),
                                    ("box-shadow".to_string(), AnimationValue::String("0 10px 20px rgba(250, 112, 154, 0.3)".to_string())),
                                ])
                            } else {
                                HashMap::from([
                                    ("transform".to_string(), AnimationValue::String("scale(1) rotate(0deg) translateY(0px)".to_string())),
                                    ("opacity".to_string(), AnimationValue::String("0.8".to_string())),
                                    ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
                                    ("box-shadow".to_string(), AnimationValue::String("0 4px 8px rgba(102, 126, 234, 0.2)".to_string())),
                                ])
                            }
                        });
                        
                        view! {
                            <ReactiveMotionDivFixed
                                animate=timeline_animation
                                class="timeline-item cursor-pointer".to_string()
                                on:click=timeline_toggle
                            >
                                {item}
                            </ReactiveMotionDivFixed>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn PerformanceDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    let toggle = move |_| {
        set_is_active.update(|active| *active = !*active);
        web_sys::console::log_1(&format!("Performance demo triggered, is_active: {}", is_active.get()).into());
    };
    
    let animation_target = signal_animate(move || {
        let is_active_val = is_active.get();
        
        if is_active_val {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("rotate(360deg) scale(1.3)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #fa709a 0%, #fee140 100%)".to_string())),
            ])
        } else {
            HashMap::from([
                ("transform".to_string(), AnimationValue::String("rotate(0deg) scale(1)".to_string())),
                ("background".to_string(), AnimationValue::String("linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string())),
            ])
        }
    });
    
    view! {
        <div class="feature-card bg-white rounded-xl p-8">
            <div class="text-4xl mb-4">"⚡"</div>
            <h3 class="text-2xl font-bold mb-4">"Performance"</h3>
            <p class="text-gray-600 mb-6">"Hardware-accelerated animations with 60fps performance."</p>
            <div class="demo-container">
                <div class="flex justify-center items-center h-full">
                    <ReactiveMotionDivFixed
                        animate=animation_target
                        class="w-20 h-20 rounded-xl flex items-center justify-center text-white font-bold text-2xl cursor-pointer".to_string()
                        on:click=toggle
                    >
                        <div>"⚡"</div>
                    </ReactiveMotionDivFixed>
                </div>
            </div>
            <button class="text-purple-600 font-semibold hover:underline" on:click=toggle>
                "Try it →"
            </button>
        </div>
    }
}

#[component]
fn CTASection() -> impl IntoView {
    view! {
        <section class="hero-gradient text-white py-20">
            <div class="container mx-auto px-6 text-center">
                <h2 class="text-4xl font-bold mb-6">
                    "Ready to build amazing animations?"
                </h2>
                <p class="text-xl mb-8 max-w-2xl mx-auto">
                    "Join thousands of developers using Leptos Motion to create beautiful, performant web animations."
                </p>
                <div class="flex justify-center gap-4">
                    <button class="bg-white text-purple-600 px-8 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors">
                        "Get Started Now"
                    </button>
                    <button class="border-2 border-white text-white px-8 py-3 rounded-lg font-semibold hover:bg-white hover:text-purple-600 transition-colors">
                        "View Documentation"
                    </button>
                </div>
            </div>
        </section>
    }
}
