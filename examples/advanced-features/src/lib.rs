//! Advanced Features Showcase
//! 
//! This example demonstrates all the advanced features of Leptos Motion:
//! - Animation Presets
//! - Advanced Gestures
//! - Interactive Playground
//! - Timeline Editor
//! - Performance Optimizations

use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_core::{
    AnimationPresets, SlideDirection, springs, easings,
    AnimationBuilder, AnimationConfig, Keyframes
};
use leptos_motion_gestures::{
    AdvancedGestureRecognizer, AdvancedGestureType, SwipeDirection
};

#[component]
pub fn AdvancedFeaturesApp() -> impl IntoView {
    let (current_section, set_current_section) = create_signal("presets".to_string());
    
    view! {
        <div class="advanced-features-app">
            <header class="app-header">
                <h1>"🚀 Leptos Motion - Advanced Features"</h1>
                <nav class="nav-tabs">
                    <button
                        class=move || if current_section.get() == "presets" { "nav-tab active" } else { "nav-tab" }
                        on:click=move |_| set_current_section.set("presets".to_string())
                    >
                        "Animation Presets"
                    </button>
                    <button
                        class=move || if current_section.get() == "gestures" { "nav-tab active" } else { "nav-tab" }
                        on:click=move |_| set_current_section.set("gestures".to_string())
                    >
                        "Advanced Gestures"
                    </button>
                    <button
                        class=move || if current_section.get() == "playground" { "nav-tab active" } else { "nav-tab" }
                        on:click=move |_| set_current_section.set("playground".to_string())
                    >
                        "Animation Playground"
                    </button>
                    <button
                        class=move || if current_section.get() == "timeline" { "nav-tab active" } else { "nav-tab" }
                        on:click=move |_| set_current_section.set("timeline".to_string())
                    >
                        "Timeline Editor"
                    </button>
                    <button
                        class=move || if current_section.get() == "performance" { "nav-tab active" } else { "nav-tab" }
                        on:click=move |_| set_current_section.set("performance".to_string())
                    >
                        "Performance"
                    </button>
                </nav>
            </header>
            
            <main class="app-content">
                {move || match current_section.get().as_str() {
                    "presets" => view! { <AnimationPresetsShowcase /> },
                    "gestures" => view! { <AdvancedGesturesShowcase /> },
                    "playground" => view! { <AnimationPlayground /> },
                    "timeline" => view! { <TimelineEditor /> },
                    "performance" => view! { <PerformanceShowcase /> },
                    _ => view! { <div>"Unknown section"</div> },
                }}
            </main>
        </div>
    }
}

/// Animation Presets Showcase
#[component]
fn AnimationPresetsShowcase() -> impl IntoView {
    let (selected_preset, set_selected_preset) = create_signal("fade_in".to_string());
    let (is_playing, set_is_playing) = create_signal(false);
    
    let presets = vec![
        ("fade_in", "Fade In", "Simple opacity animation"),
        ("slide_up", "Slide Up", "Slide from bottom with spring"),
        ("scale_in", "Scale In", "Scale with back easing"),
        ("pop_in", "Pop In", "Scale from 0 with spring"),
        ("rotate_in", "Rotate In", "Rotate with back easing"),
        ("flip_in", "Flip In", "3D flip animation"),
        ("bounce", "Bounce", "Continuous bounce"),
        ("shake", "Shake", "Horizontal shake"),
        ("pulse", "Pulse", "Scale pulse"),
        ("spin", "Spin", "Continuous rotation"),
    ];
    
    let animation_config = create_memo(move |_| {
        match selected_preset.get().as_str() {
            "fade_in" => AnimationPresets::fade_in(),
            "slide_up" => AnimationPresets::slide_up(50.0),
            "scale_in" => AnimationPresets::scale_in(),
            "pop_in" => AnimationPresets::pop_in(),
            "rotate_in" => AnimationPresets::rotate_in(),
            "flip_in" => AnimationPresets::flip_in(),
            "bounce" => {
                let keyframes = AnimationPresets::bounce();
                AnimationBuilder::new()
                    .animate(keyframes.to_animation_target())
                    .transition(Transition {
                        duration: Some(1.0),
                        ease: easings::EASE_OUT,
                        repeat: RepeatConfig::Infinite,
                        ..Default::default()
                    })
                    .build()
            }
            "shake" => {
                let keyframes = AnimationPresets::shake();
                AnimationBuilder::new()
                    .animate(keyframes.to_animation_target())
                    .transition(Transition {
                        duration: Some(0.5),
                        ease: easings::EASE_IN_OUT,
                        repeat: RepeatConfig::Infinite,
                        ..Default::default()
                    })
                    .build()
            }
            "pulse" => {
                let keyframes = AnimationPresets::pulse();
                AnimationBuilder::new()
                    .animate(keyframes.to_animation_target())
                    .transition(Transition {
                        duration: Some(2.0),
                        ease: easings::EASE_IN_OUT,
                        repeat: RepeatConfig::Infinite,
                        ..Default::default()
                    })
                    .build()
            }
            "spin" => AnimationPresets::spin(),
            _ => AnimationPresets::fade_in(),
        }
    });
    
    view! {
        <div class="presets-showcase">
            <div class="showcase-header">
                <h2>"🎨 Animation Presets"</h2>
                <p>"Pre-built animations for common use cases"</p>
            </div>
            
            <div class="showcase-content">
                <div class="preset-selector">
                    <h3>"Choose a Preset"</h3>
                    <div class="preset-grid">
                        {presets.into_iter().map(|(key, name, description)| {
                            let is_selected = create_memo(move |_| selected_preset.get() == key);
                            view! {
                                <div
                                    class=move || if is_selected.get() { "preset-card selected" } else { "preset-card" }
                                    on:click=move |_| set_selected_preset.set(key.to_string())
                                >
                                    <h4>{name}</h4>
                                    <p>{description}</p>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
                
                <div class="preset-preview">
                    <div class="preview-header">
                        <h3>"Preview"</h3>
                        <div class="preview-controls">
                            <button
                                class="preview-btn"
                                on:click=move |_| set_is_playing.set(!is_playing.get())
                            >
                                {move || if is_playing.get() { "⏸ Pause" } else { "▶ Play" }}
                            </button>
                            <button
                                class="preview-btn"
                                on:click=move |_| {
                                    set_is_playing.set(false);
                                    // Reset animation
                                }
                            >
                                "🔄 Reset"
                            </button>
                        </div>
                    </div>
                    
                    <div class="preview-container">
                        <AnimatePresence mode=PresenceMode::Wait>
                            {move || if is_playing.get() {
                                view! {
                                    <MotionDiv
                                        class="animated-element"
                                        initial=animation_config.get().initial.clone()
                                        animate=animation_config.get().animate.clone()
                                        transition=animation_config.get().transition.clone()
                                        style="width: 120px; height: 120px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; font-size: 16px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);"
                                    >
                                        "Animate!"
                                    </MotionDiv>
                                }
                            } else {
                                view! {
                                    <div
                                        class="animated-element"
                                        style="width: 120px; height: 120px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; font-size: 16px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);"
                                    >
                                        "Ready"
                                    </div>
                                }
                            }}
                        </AnimatePresence>
                    </div>
                    
                    <div class="code-preview">
                        <h4>"Generated Code"</h4>
                        <pre class="code-block">
                            {move || {
                                let config = animation_config.get();
                                format!(
                                    r#"// Using AnimationPresets
let animation = AnimationPresets::{}();

<MotionDiv
    initial={{{:?}}}
    animate={{{:?}}}
    transition={{{:?}}}
>
    "Your Content"
</MotionDiv>"#,
                                    selected_preset.get(),
                                    config.initial,
                                    config.animate,
                                    config.transition
                                )
                            }}
                        </pre>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Advanced Gestures Showcase
#[component]
fn AdvancedGesturesShowcase() -> impl IntoView {
    let (gesture_events, set_gesture_events) = create_signal(Vec::new());
    let (is_dragging, set_is_dragging) = create_signal(false);
    
    view! {
        <div class="gestures-showcase">
            <div class="showcase-header">
                <h2>"👆 Advanced Gestures"</h2>
                <p>"Multi-touch, pinch-to-zoom, and complex gesture recognition"</p>
            </div>
            
            <div class="showcase-content">
                <div class="gesture-demo">
                    <h3>"Gesture Demo Area"</h3>
                    <div class="gesture-container">
                        <MotionDiv
                            class="gesture-target"
                            drag=true
                            drag_constraints=DragConstraints::Bounded {
                                left: -100.0,
                                right: 100.0,
                                top: -100.0,
                                bottom: 100.0,
                            }
                            while_drag=motion_target!(
                                "scale" => AnimationValue::Number(1.1),
                                "boxShadow" => AnimationValue::String("0 20px 40px rgba(0,0,0,0.3)".to_string())
                            )
                            on:dragstart=move |_| set_is_dragging.set(true)
                            on:dragend=move |_| set_is_dragging.set(false)
                            style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; cursor: grab; user-select: none;"
                        >
                            "Drag Me"
                        </MotionDiv>
                        
                        <div class="gesture-instructions">
                            <p>"Try these gestures:"</p>
                            <ul>
                                <li>"🖱️ Drag the element around"</li>
                                <li>"📱 Pinch to zoom (on touch devices)"</li>
                                <li>"🔄 Rotate with two fingers"</li>
                                <li>"👆 Long press for context menu"</li>
                                <li>"↔️ Swipe in any direction"</li>
                            </ul>
                        </div>
                    </div>
                </div>
                
                <div class="gesture-events">
                    <h3>"Gesture Events"</h3>
                    <div class="events-log">
                        {move || {
                            gesture_events.get().into_iter().map(|event| {
                                view! {
                                    <div class="event-item">
                                        <span class="event-type">{format!("{:?}", event.gesture_type)}</span>
                                        <span class="event-state">{format!("{:?}", event.state)}</span>
                                        <span class="event-coords">{format!("({:.1}, {:.1})", event.x, event.y)}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
                
                <div class="gesture-code">
                    <h3>"Gesture Implementation"</h3>
                    <pre class="code-block">
                        {r#"// Advanced gesture recognition
let mut recognizer = AdvancedGestureRecognizer::new();

// Handle touch events
let events = recognizer.handle_touch_start(&touch_event);
for event in events {
    match event.gesture_type {
        GestureType::PinchZoom => {
            // Handle pinch zoom
        }
        GestureType::Rotate => {
            // Handle rotation
        }
        GestureType::Swipe => {
            // Handle swipe
        }
        _ => {}
    }
}

// In your component
<MotionDiv
    drag=true
    drag_constraints=DragConstraints::Bounded {
        left: -100.0,
        right: 100.0,
        top: -100.0,
        bottom: 100.0,
    }
    while_drag=motion_target!(
        "scale" => AnimationValue::Number(1.1)
    )
>
    "Interactive Element"
</MotionDiv>"#}
                    </pre>
                </div>
            </div>
        </div>
    }
}

/// Performance Showcase
#[component]
fn PerformanceShowcase() -> impl IntoView {
    let (element_count, set_element_count) = create_signal(10);
    let (is_running, set_is_running) = create_signal(false);
    let (fps, set_fps) = create_signal(60.0);
    let (memory_usage, set_memory_usage) = create_signal(0.0);
    
    // Generate animated elements
    let animated_elements = create_memo(move |_| {
        let count = element_count.get();
        (0..count).map(|i| {
            let delay = i as f64 * 0.1;
            view! {
                <MotionDiv
                    class="performance-element"
                    initial=motion_target!(
                        "opacity" => AnimationValue::Number(0.0),
                        "y" => AnimationValue::Pixels(50.0)
                    )
                    animate=motion_target!(
                        "opacity" => AnimationValue::Number(1.0),
                        "y" => AnimationValue::Pixels(0.0)
                    )
                    transition=Transition {
                        duration: Some(0.5),
                        delay: Some(delay),
                        ease: easings::EASE_OUT,
                        ..Default::default()
                    }
                    style=format!("background: hsl({}, 70%, 60%);", (i * 30) % 360)
                >
                    {format!("Element {}", i + 1)}
                </MotionDiv>
            }
        }).collect::<Vec<_>>()
    });
    
    view! {
        <div class="performance-showcase">
            <div class="showcase-header">
                <h2>"⚡ Performance Optimizations"</h2>
                <p>"GPU acceleration, memory management, and performance monitoring"</p>
            </div>
            
            <div class="showcase-content">
                <div class="performance-controls">
                    <div class="control-group">
                        <label>"Element Count: " {element_count}</label>
                        <input
                            type="range"
                            min="1"
                            max="100"
                            value=element_count
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse().unwrap_or(10);
                                set_element_count.set(value);
                            }
                        />
                    </div>
                    
                    <div class="control-group">
                        <button
                            class="performance-btn"
                            on:click=move |_| set_is_running.set(!is_running.get())
                        >
                            {move || if is_running.get() { "⏸ Stop Animation" } else { "▶ Start Animation" }}
                        </button>
                    </div>
                    
                    <div class="performance-metrics">
                        <div class="metric">
                            <span class="metric-label">"FPS:"</span>
                            <span class="metric-value">{fps}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Memory:"</span>
                            <span class="metric-value">{move || format!("{:.1} MB", memory_usage.get())}</span>
                        </div>
                    </div>
                </div>
                
                <div class="performance-demo">
                    <h3>"Performance Test"</h3>
                    <div class="elements-container">
                        {move || if is_running.get() {
                            animated_elements.get()
                        } else {
                            vec![view! {
                                <div class="performance-element" style="background: #ccc;">
                                    "Click 'Start Animation' to test performance"
                                </div>
                            }]
                        }}
                    </div>
                </div>
                
                <div class="performance-info">
                    <h3>"Performance Features"</h3>
                    <div class="feature-list">
                        <div class="feature-item">
                            <h4>"🎯 GPU Acceleration"</h4>
                            <p>"Transform properties (x, y, scale, rotate, opacity) are GPU-accelerated for smooth 60fps animations."</p>
                        </div>
                        
                        <div class="feature-item">
                            <h4>"🧠 Memory Management"</h4>
                            <p>"Animation objects are pooled and reused to minimize memory allocations and garbage collection."</p>
                        </div>
                        
                        <div class="feature-item">
                            <h4>"📊 Performance Monitoring"</h4>
                            <p>"Real-time monitoring of frame rate, memory usage, and animation performance with configurable budgets."</p>
                        </div>
                        
                        <div class="feature-item">
                            <h4>"⚡ Optimized Engine"</h4>
                            <p>"Hybrid animation engine that automatically chooses between Web Animations API and RequestAnimationFrame for optimal performance."</p>
                        </div>
                        
                        <div class="feature-item">
                            <h4>"🎨 DOM Batching"</h4>
                            <p>"DOM updates are batched to minimize reflows and repaints, improving overall performance."</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper function to convert Keyframes to AnimationTarget
trait KeyframesExt {
    fn to_animation_target(&self) -> AnimationTarget;
}

impl KeyframesExt for Keyframes {
    fn to_animation_target(&self) -> AnimationTarget {
        // For demonstration, return the last keyframe
        if let Some(last_keyframe) = self.values.last() {
            last_keyframe.clone()
        } else {
            AnimationTarget::new()
        }
    }
}
