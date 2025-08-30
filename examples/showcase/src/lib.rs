use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="showcase-container">
            <header class="showcase-header">
                <h1>"Leptos Motion Showcase"</h1>
                <p>"A comprehensive demonstration of animation capabilities"</p>
            </header>

            <main class="showcase-content">
                <section class="showcase-section">
                    <h2>"Basic Animations"</h2>
                    <BasicAnimations />
                </section>

                <section class="showcase-section">
                    <h2>"Gesture Interactions"</h2>
                    <GestureInteractions />
                </section>

                <section class="showcase-section">
                    <h2>"Advanced Patterns"</h2>
                    <AdvancedPatterns />
                </section>

                <section class="showcase-section">
                    <h2>"Performance Demo"</h2>
                    <PerformanceDemo />
                </section>
            </main>
        </div>
    }
}

#[component]
fn BasicAnimations() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let (scale, set_scale) = signal(1.0);
    let (rotation, set_rotation) = signal(0.0);

    view! {
        <div class="basic-animations">
            <div class="controls">
                <button on:click=move |_| set_visible.set(!visible.get())>
                    "Toggle Visibility"
                </button>
                <button on:click=move |_| set_scale.set(if scale.get() == 1.0 { 1.5 } else { 1.0 })>
                    "Toggle Scale"
                </button>
                <button on:click=move |_| set_rotation.set(rotation.get() + 90.0)>
                    "Rotate 90°"
                </button>
            </div>

            <div class="animation-grid">
                // Fade animation
                <MotionDiv
                    class="animation-box fade-box"
                    animate=motion_target!(
                        "opacity" => AnimationValue::Number(if visible.get() { 1.0 } else { 0.0 })
                    )
                    transition=Transition {
                        duration: Some(0.5),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    <h3>"Fade Animation"</h3>
                    <p>"Smooth opacity transitions"</p>
                </MotionDiv>

                // Scale animation
                <MotionDiv
                    class="animation-box scale-box"
                    animate=motion_target!(
                        "scale" => AnimationValue::Number(scale.get())
                    )
                    transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::BackOut,
                        ..Default::default()
                    }
                >
                    <h3>"Scale Animation"</h3>
                    <p>"Dynamic size changes"</p>
                </MotionDiv>

                // Rotation animation
                <MotionDiv
                    class="animation-box rotate-box"
                    animate=motion_target!(
                        "rotate" => AnimationValue::Degrees(rotation.get())
                    )
                    transition=Transition {
                        duration: Some(0.5),
                        ease: Easing::Spring(SpringConfig {
                            stiffness: 100.0,
                            damping: 15.0,
                            mass: 1.0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                >
                    <h3>"Rotation Animation"</h3>
                    <p>"Smooth rotation with spring physics"</p>
                </MotionDiv>

                // Combined animation
                <MotionDiv
                    class="animation-box combined-box"
                    animate=motion_target!(
                        "x" => AnimationValue::Pixels(if visible.get() { 0.0 } else { 100.0 }),
                        "y" => AnimationValue::Pixels(if visible.get() { 0.0 } else { -50.0 }),
                        "rotate" => AnimationValue::Degrees(if visible.get() { 0.0 } else { 180.0 }),
                        "scale" => AnimationValue::Number(if visible.get() { 1.0 } else { 0.8 })
                    )
                    transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::Spring(SpringConfig::default()),
                        ..Default::default()
                    }
                >
                    <h3>"Combined Animation"</h3>
                    <p>"Multiple properties animated together"</p>
                </MotionDiv>
            </div>
        </div>
    }
}

#[component]
fn GestureInteractions() -> impl IntoView {
    view! {
        <div class="gesture-interactions">
            <div class="gesture-grid">
                // Hover animation
                <MotionDiv
                    class="gesture-box hover-box"
                    while_hover=Some(motion_target!(
                        "scale" => AnimationValue::Number(1.1),
                        "rotate" => AnimationValue::Degrees(5.0),
                        "boxShadow" => AnimationValue::String("0 10px 30px rgba(0,0,0,0.3)".to_string())
                    ))
                    transition=Transition {
                        duration: Some(0.2),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    <h3>"Hover Effect"</h3>
                    <p>"Hover over me!"</p>
                </MotionDiv>

                // Tap animation
                <MotionDiv
                    class="gesture-box tap-box"
                    while_tap=Some(motion_target!(
                        "scale" => AnimationValue::Number(0.95)
                    ))
                    transition=Transition {
                        duration: Some(0.1),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    <h3>"Tap Effect"</h3>
                    <p>"Tap me!"</p>
                </MotionDiv>

                // Drag interaction
                <MotionDiv
                    class="gesture-box drag-box"
                    drag=Some(DragConfig::new()
                        .axis(DragAxis::Both)
                        .constraints(DragConstraints {
                            left: Some(-100.0),
                            right: Some(100.0),
                            top: Some(-100.0),
                            bottom: Some(100.0),
                        }))
                    while_drag=Some(motion_target!(
                        "scale" => AnimationValue::Number(1.1),
                        "zIndex" => AnimationValue::Number(1000.0)
                    ))
                >
                    <h3>"Drag Interaction"</h3>
                    <p>"Drag me around!"</p>
                </MotionDiv>

                // Focus animation
                <MotionDiv
                    class="gesture-box focus-box"
                    while_focus=Some(motion_target!(
                        "scale" => AnimationValue::Number(1.05),
                        "borderColor" => AnimationValue::String("#667eea".to_string())
                    ))
                    transition=Transition {
                        duration: Some(0.2),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    <h3>"Focus Effect"</h3>
                    <p>"Click to focus!"</p>
                </MotionDiv>
            </div>
        </div>
    }
}

#[component]
fn AdvancedPatterns() -> impl IntoView {
    let (is_visible, set_visible) = signal(false);
    let (is_expanded, set_expanded) = signal(false);

    // Variants for complex animations
    let variants = Variants::new()
        .variant("hidden", motion_target!(
            "opacity" => AnimationValue::Number(0.0),
            "x" => AnimationValue::Pixels(-100.0),
            "scale" => AnimationValue::Number(0.8)
        ))
        .variant("visible", motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "x" => AnimationValue::Pixels(0.0),
            "scale" => AnimationValue::Number(1.0)
        ));

    // Staggered items
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

    view! {
        <div class="advanced-patterns">
            <div class="controls">
                <button on:click=move |_| set_visible.set(!is_visible.get())>
                    "Toggle Variants"
                </button>
                <button on:click=move |_| set_expanded.set(!is_expanded.get())>
                    "Toggle Layout"
                </button>
            </div>

            <div class="patterns-grid">
                // Variant animation
                <MotionDiv
                    class="pattern-box variant-box"
                    variants=Some(variants)
                    initial=Some("hidden".to_string())
                    animate=Some(if is_visible.get() { "visible".to_string() } else { "hidden".to_string() })
                    transition=Transition {
                        duration: Some(0.5),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    <h3>"Variant Animation"</h3>
                    <p>"Reusable animation states"</p>
                </MotionDiv>

                // Layout animation
                <MotionDiv
                    class="pattern-box layout-box"
                    layout=Some(true)
                    animate=motion_target!(
                        "width" => AnimationValue::Pixels(if is_expanded.get() { 300.0 } else { 150.0 }),
                        "height" => AnimationValue::Pixels(if is_expanded.get() { 200.0 } else { 100.0 })
                    )
                    transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    <h3>"Layout Animation"</h3>
                    <p>"Automatic layout transitions"</p>
                </MotionDiv>

                // Staggered animation
                <div class="stagger-container">
                    <h3>"Staggered Animation"</h3>
                    <div class="stagger-items">
                        {items.into_iter().enumerate().map(|(i, item)| {
                            view! {
                                <MotionDiv
                                    class="stagger-item"
                                    key=item
                                    initial=Some(motion_target!(
                                        "opacity" => AnimationValue::Number(0.0),
                                        "y" => AnimationValue::Pixels(50.0)
                                    ))
                                    animate=motion_target!(
                                        "opacity" => AnimationValue::Number(1.0),
                                        "y" => AnimationValue::Pixels(0.0)
                                    )
                                    transition=Transition {
                                        duration: Some(0.5),
                                        delay: Some(i as f64 * 0.1),
                                        ease: Easing::EaseOut,
                                        ..Default::default()
                                    }
                                >
                                    {item}
                                </MotionDiv>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Keyframe animation
                <MotionDiv
                    class="pattern-box keyframe-box"
                    animate=motion_target!(
                        "x" => AnimationValue::Pixels(200.0),
                        "y" => AnimationValue::Pixels(0.0),
                        "rotate" => AnimationValue::Degrees(360.0)
                    )
                    transition=Transition {
                        duration: Some(2.0),
                        ease: Easing::EaseInOut,
                        repeat: Some(RepeatConfig {
                            count: None,
                            reverse: true,
                            delay: Some(0.5),
                        }),
                        ..Default::default()
                    }
                >
                    <h3>"Keyframe Animation"</h3>
                    <p>"Complex multi-step animation"</p>
                </MotionDiv>
            </div>
        </div>
    }
}

#[component]
fn PerformanceDemo() -> impl IntoView {
    let (particle_count, set_particle_count) = signal(50);
    let particles = (0..particle_count.get()).collect::<Vec<_>>();

    view! {
        <div class="performance-demo">
            <div class="controls">
                <label>
                    "Particle Count: "
                    <input
                        type="range"
                        min="10"
                        max="200"
                        value=particle_count.get()
                        on:input=move |ev| {
                            if let Some(value) = event_target_value(&ev).parse::<usize>().ok() {
                                set_particle_count.set(value);
                            }
                        }
                    />
                    {particle_count}
                </label>
            </div>

            <div class="particles-container">
                {particles.into_iter().map(|i| {
                    let delay = (i as f64 * 0.1) % 2.0;
                    view! {
                        <MotionDiv
                            class="particle"
                            key=i
                            animate=motion_target!(
                                "x" => AnimationValue::Pixels(300.0 * (i as f64 % 3.0)),
                                "y" => AnimationValue::Pixels(200.0 * (i as f64 % 2.0)),
                                "scale" => AnimationValue::Number(0.5 + (i as f64 % 5) * 0.1),
                                "rotate" => AnimationValue::Degrees(360.0)
                            )
                            transition=Transition {
                                duration: Some(3.0),
                                delay: Some(delay),
                                ease: Easing::EaseInOut,
                                repeat: Some(RepeatConfig {
                                    count: None,
                                    reverse: true,
                                    delay: Some(0.0),
                                }),
                                ..Default::default()
                            }
                        >
                            "●"
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="performance-info">
                <p>"This demo shows {particle_count} particles animating simultaneously."</p>
                <p>"Each particle has multiple animated properties running at 60fps."</p>
            </div>
        </div>
    }
}

/// Mount the application
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    mount_to_body(App);
}
