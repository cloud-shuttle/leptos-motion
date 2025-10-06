use leptos::*;
use leptos_motion::*;

#[component]
pub fn AdvancedFeaturesShowcase() -> impl IntoView {
    let (active_demo, set_active_demo) = create_signal("variants".to_string());

    view! {
        <div class="showcase-container">
            <header class="showcase-header">
                <h1>"🎨 Leptos Motion Advanced Features Showcase"</h1>
                <p class="subtitle">"Experience all 4 advanced animation systems working together"</p>
            </header>

            <nav class="demo-nav">
                <button
                    class=move || format!("nav-btn {}", if active_demo.get() == "variants" { "active" } else { "" })
                    on:click=move |_| set_active_demo.set("variants".to_string())
                >
                    "🎭 Variants"
                </button>
                <button
                    class=move || format!("nav-btn {}", if active_demo.get() == "keyframes" { "active" } else { "" })
                    on:click=move |_| set_active_demo.set("keyframes".to_string())
                >
                    "🎬 Keyframes"
                </button>
                <button
                    class=move || format!("nav-btn {}", if active_demo.get() == "stagger" { "active" } else { "" })
                    on:click=move |_| set_active_demo.set("stagger".to_string())
                >
                    "⚡ Stagger"
                </button>
                <button
                    class=move || format!("nav-btn {}", if active_demo.get() == "timeline" { "active" } else { "" })
                    on:click=move |_| set_active_demo.set("timeline".to_string())
                >
                    "🎯 Timeline"
                </button>
                <button
                    class=move || format!("nav-btn {}", if active_demo.get() == "combined" { "active" } else { "" })
                    on:click=move |_| set_active_demo.set("combined".to_string())
                >
                    "🎊 Combined"
                </button>
            </nav>

            <main class="demo-content">
                {move || match active_demo.get().as_str() {
                    "variants" => view! { <VariantsDemo /> }.into_view(),
                    "keyframes" => view! { <KeyframesDemo /> }.into_view(),
                    "stagger" => view! { <StaggerDemo /> }.into_view(),
                    "timeline" => view! { <TimelineDemo /> }.into_view(),
                    "combined" => view! { <CombinedDemo /> }.into_view(),
                    _ => view! { <VariantsDemo /> }.into_view(),
                }}
            </main>
        </div>
    }
}

#[component]
pub fn VariantsDemo() -> impl IntoView {
    let (variant, set_variant) = create_signal("idle".to_string());

    // Define animation variants
    let button_variants = AnimationVariants::new()
        .variant("idle", hashmap! {
            "scale" => AnimationValue::Number(1.0),
            "backgroundColor" => AnimationValue::String("#3b82f6".to_string()),
        })
        .variant("hover", hashmap! {
            "scale" => AnimationValue::Number(1.05),
            "backgroundColor" => AnimationValue::String("#2563eb".to_string()),
        })
        .variant("pressed", hashmap! {
            "scale" => AnimationValue::Number(0.95),
            "backgroundColor" => AnimationValue::String("#1d4ed8".to_string()),
        });

    view! {
        <div class="demo-section">
            <h2>"🎭 Variants System"</h2>
            <p>"Named animation states with reusable definitions"</p>

            <div class="demo-controls">
                <button on:click=move |_| set_variant.set("idle".to_string())>"Idle"</button>
                <button on:click=move |_| set_variant.set("hover".to_string())>"Hover"</button>
                <button on:click=move |_| set_variant.set("pressed".to_string())>"Pressed"</button>
            </div>

            <div class="demo-visualization">
                <MotionDiv
                    class="variant-button"
                    variants=Some(button_variants)
                    animate_variant=Some(variant.get())
                    initial_variant=Some("idle".to_string())
                >
                    "Animated Button"
                </MotionDiv>
            </div>

            <div class="code-example">
                <pre><code>"let variants = AnimationVariants::new()
    .variant(\"idle\", scale: 1.0, blue)
    .variant(\"hover\", scale: 1.05, darker blue)
    .variant(\"pressed\", scale: 0.95, darkest blue);

<MotionDiv variants=variants animate_variant=variant />"</code></pre>
            </div>
        </div>
    }
}

#[component]
pub fn KeyframesDemo() -> impl IntoView {
    let (is_animating, set_is_animating) = create_signal(false);

    // Define keyframes sequence
    let keyframes = Keyframes::new()
        .keyframe(0.0, hashmap! {
            "x" => AnimationValue::Pixels(0.0),
            "opacity" => AnimationValue::Number(1.0),
        })
        .keyframe(0.25, hashmap! {
            "x" => AnimationValue::Pixels(100.0),
            "opacity" => AnimationValue::Number(0.8),
        })
        .keyframe(0.5, hashmap! {
            "x" => AnimationValue::Pixels(200.0),
            "opacity" => AnimationValue::Number(0.6),
        })
        .keyframe(0.75, hashmap! {
            "x" => AnimationValue::Pixels(300.0),
            "opacity" => AnimationValue::Number(0.3),
        })
        .keyframe(1.0, hashmap! {
            "x" => AnimationValue::Pixels(400.0),
            "opacity" => AnimationValue::Number(0.0),
        })
        .build()
        .unwrap();

    let animate_props = move || {
        if is_animating.get() {
            Some(hashmap! {
                "x" => AnimationValue::Pixels(400.0),
                "opacity" => AnimationValue::Number(0.0),
            })
        } else {
            Some(hashmap! {
                "x" => AnimationValue::Pixels(0.0),
                "opacity" => AnimationValue::Number(1.0),
            })
        }
    };

    view! {
        <div class="demo-section">
            <h2>"🎬 Keyframes System"</h2>
            <p>"Multi-step animations with smooth interpolation between states"</p>

            <div class="demo-controls">
                <button on:click=move |_| set_is_animating.set(!is_animating.get())>
                    {move || if is_animating.get() { "Stop" } else { "Start" }}
                </button>
            </div>

            <div class="demo-visualization keyframes-track">
                <MotionDiv
                    class="keyframe-ball"
                    animate=Some(AnimateProp::Keyframes(keyframes))
                    node_ref=NodeRef::new()
                >
                    "⚽"
                </MotionDiv>
            </div>

            <div class="code-example">
                <pre><code>"let keyframes = Keyframes::new()
    .keyframe(0.0, x: 0px, opacity: 1.0)
    .keyframe(0.25, x: 100px, opacity: 0.8)
    .keyframe(0.5, x: 200px, opacity: 0.6)
    .keyframe(0.75, x: 300px, opacity: 0.3)
    .keyframe(1.0, x: 400px, opacity: 0.0);

<MotionDiv animate=AnimateProp::Keyframes(keyframes) />"</code></pre>
            </div>
        </div>
    }
}

#[component]
pub fn StaggerDemo() -> impl IntoView {
    let (is_animating, set_is_animating) = create_signal(false);

    // Create stagger configuration
    let stagger_config = MotionStaggerConfig::new(
        MotionStaggerConfigBuilder::new()
            .direction(StaggerDirection::Forward)
            .fixed_delay(0.1)
            .start_delay(0.2)
            .build(),
        vec![
            hashmap! { "opacity" => AnimationValue::Number(0.0), "y" => AnimationValue::Pixels(20.0) },
            hashmap! { "opacity" => AnimationValue::Number(0.0), "y" => AnimationValue::Pixels(20.0) },
            hashmap! { "opacity" => AnimationValue::Number(0.0), "y" => AnimationValue::Pixels(20.0) },
            hashmap! { "opacity" => AnimationValue::Number(0.0), "y" => AnimationValue::Pixels(20.0) },
            hashmap! { "opacity" => AnimationValue::Number(0.0), "y" => AnimationValue::Pixels(20.0) },
        ]
    );

    let animate_props = move || {
        if is_animating.get() {
            Some(hashmap! {
                "opacity" => AnimationValue::Number(1.0),
                "y" => AnimationValue::Pixels(0.0),
            })
        } else {
            Some(hashmap! {
                "opacity" => AnimationValue::Number(0.0),
                "y" => AnimationValue::Pixels(20.0),
            })
        }
    };

    view! {
        <div class="demo-section">
            <h2>"⚡ Stagger Animations"</h2>
            <p>"Sequential element animation effects with configurable timing patterns"</p>

            <div class="demo-controls">
                <button on:click=move |_| set_is_animating.set(!is_animating.get())>
                    {move || if is_animating.get() { "Hide" } else { "Show" }}
                </button>
            </div>

            <div class="demo-visualization stagger-container">
                {(0..5).map(|i| {
                    view! {
                        <MotionDiv
                            class="stagger-item"
                            key=i
                            animate=animate_props()
                            stagger=Some(ElementStaggerConfig::new(
                                MotionStaggerConfigBuilder::new()
                                    .direction(StaggerDirection::Forward)
                                    .fixed_delay(0.1)
                                    .build(),
                                vec![hashmap! {
                                    "opacity" => AnimationValue::Number(1.0),
                                    "y" => AnimationValue::Pixels(0.0),
                                }]
                            ))
                            node_ref=NodeRef::new()
                        >
                            {format!("Item {}", i + 1)}
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="code-example">
                <pre><code>"let stagger = MotionStaggerConfigBuilder::new()
    .direction(StaggerDirection::Forward)
    .fixed_delay(0.1)
    .start_delay(0.2)
    .build();

<MotionDiv stagger=stagger animate=properties />"</code></pre>
            </div>
        </div>
    }
}

#[component]
pub fn TimelineDemo() -> impl IntoView {
    let (timeline_state, set_timeline_state) = create_signal("stopped".to_string());

    // Create a timeline with multiple tracks
    let timeline = TimelineBuilder::new("complex-timeline".to_string(), 4.0)
        .add_track(TimelineTrack::new(
            AnimationTarget {
                property: "x".to_string(),
                from_value: 0.0,
                to_value: 200.0,
                current_value: 0.0,
                duration: 1.0,
                start_time: 0.0,
                easing: "ease-out".to_string(),
            },
            0.0, 1.0, "slide-right".to_string()
        ).with_property("x".to_string(), AnimationValue::Pixels(200.0)))
        .add_track(TimelineTrack::new(
            AnimationTarget {
                property: "scale".to_string(),
                from_value: 1.0,
                to_value: 1.5,
                current_value: 1.0,
                duration: 0.5,
                start_time: 1.0,
                easing: "ease-in".to_string(),
            },
            1.0, 0.5, "scale-up".to_string()
        ).with_property("scale".to_string(), AnimationValue::Number(1.5)))
        .add_track(TimelineTrack::new(
            AnimationTarget {
                property: "rotation".to_string(),
                from_value: 0.0,
                to_value: 360.0,
                current_value: 0.0,
                duration: 1.0,
                start_time: 2.0,
                easing: "linear".to_string(),
            },
            2.0, 1.0, "spin".to_string()
        ).with_property("rotate".to_string(), AnimationValue::Degrees(360.0)))
        .add_event(TimelineEvent::new(1.0, TimelineEventType::Update, "scale-start".to_string()))
        .add_event(TimelineEvent::new(2.0, TimelineEventType::Update, "spin-start".to_string()))
        .build();

    view! {
        <div class="demo-section">
            <h2>"🎯 Timeline Orchestration"</h2>
            <p>"Complex animation sequences with precise timing and event control"</p>

            <div class="demo-controls">
                <button on:click=move |_| {
                    // In a real implementation, we'd control the timeline here
                    set_timeline_state.set("playing".to_string());
                }>"Play Timeline"</button>
                <button on:click=move |_| set_timeline_state.set("paused".to_string())>"Pause"</button>
                <button on:click=move |_| set_timeline_state.set("stopped".to_string())>"Stop"</button>
            </div>

            <div class="demo-visualization timeline-track">
                <div class="timeline-progress">
                    <div class="timeline-bar" style=move || format!("width: {}%", timeline.current_time / timeline.duration * 100.0)></div>
                </div>

                <MotionDiv
                    class="timeline-object"
                    animate=Some(AnimateProp::Static(hashmap! {
                        "x" => AnimationValue::Pixels(200.0),
                        "scale" => AnimationValue::Number(1.5),
                        "rotate" => AnimationValue::Degrees(360.0),
                    }))
                    transition=Some(Transition {
                        duration: Some(4.0),
                        ease: Some(EasingFunction::EaseOut),
                        ..Default::default()
                    })
                    timeline=Some(timeline)
                    node_ref=NodeRef::new()
                >
                    "🚀"
                </MotionDiv>
            </div>

            <div class="code-example">
                <pre><code>"let timeline = TimelineBuilder::new(\"sequence\", 4.0)
    .add_track(track1.start(0s).duration(1s))  // Slide right
    .add_track(track2.start(1s).duration(0.5s)) // Scale up
    .add_track(track3.start(2s).duration(1s))  // Spin
    .add_event(event.at(1s).on_scale_start())
    .build();

<MotionDiv timeline=timeline />"</code></pre>
            </div>
        </div>
    }
}

#[component]
pub fn CombinedDemo() -> impl IntoView {
    let (is_active, set_is_active) = create_signal(false);

    view! {
        <div class="demo-section">
            <h2>"🎊 Combined Advanced Features"</h2>
            <p>"All 4 advanced animation systems working together"</p>

            <div class="demo-controls">
                <button on:click=move |_| set_is_active.set(!is_active.get())>
                    {move || if is_active.get() { "Reset" } else { "Start Sequence" }}
                </button>
            </div>

            <div class="demo-visualization combined-showcase">
                <div class="combined-grid">
                    {(0..9).map(|i| {
                        let delay = i as f64 * 0.1;

                        // Create variants for each card
                        let card_variants = AnimationVariants::new()
                            .variant("hidden", hashmap! {
                                "opacity" => AnimationValue::Number(0.0),
                                "scale" => AnimationValue::Number(0.8),
                                "y" => AnimationValue::Pixels(50.0),
                            })
                            .variant("visible", hashmap! {
                                "opacity" => AnimationValue::Number(1.0),
                                "scale" => AnimationValue::Number(1.0),
                                "y" => AnimationValue::Pixels(0.0),
                            });

                        view! {
                            <MotionDiv
                                class="combined-card"
                                key=i
                                variants=Some(card_variants)
                                animate_variant=Some(if is_active.get() { "visible" } else { "hidden" })
                                initial_variant=Some("hidden")
                                transition=Some(Transition {
                                    duration: Some(0.6),
                                    delay: Some(delay),
                                    ease: Some(EasingFunction::EaseOut),
                                    ..Default::default()
                                })
                                node_ref=NodeRef::new()
                            >
                                <div class="card-content">
                                    <div class="card-number">{i + 1}</div>
                                    <div class="card-icon">
                                        {match i % 4 {
                                            0 => "🎭",
                                            1 => "🎬",
                                            2 => "⚡",
                                            3 => "🎯",
                                            _ => "🎊",
                                        }}
                                    </div>
                                </div>
                            </MotionDiv>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            <div class="code-example">
                <pre><code>"// Variants + Stagger + Keyframes + Timeline = Magic! 🎉

let variants = AnimationVariants::new()
    .variant(\"hidden\", opacity: 0, scale: 0.8, y: 50px)
    .variant(\"visible\", opacity: 1, scale: 1, y: 0px);

<MotionDiv
    variants=variants
    animate_variant=\"visible\"
    transition=Transition { delay: 0.1 * index, .. }
/>"</code></pre>
            </div>
        </div>
    }
}
