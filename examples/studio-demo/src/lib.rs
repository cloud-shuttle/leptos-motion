//! Motion Studio Demo
//!
//! A comprehensive demo showcasing the Motion Studio capabilities:
//! - Visual timeline editor
//! - 3D transform manipulation
//! - SVG path morphing
//! - Real-time preview
//! - Export functionality

use leptos::*;
use leptos_motion_studio::export::{ExportFormat, ExportResult};
use leptos_motion_studio::morphing::SvgMorphingEditor;
use leptos_motion_studio::timeline::{AnimationProperty, AnimationValue, TimelineEditor};
use leptos_motion_studio::*;
use wasm_bindgen::prelude::*;

/// Main studio demo component
#[component]
pub fn StudioDemo() -> impl IntoView {
    let (current_project, set_current_project) = create_signal(StudioProject::new("Demo Project"));
    let (demo_mode, set_demo_mode) = create_signal(DemoMode::Timeline);

    view! {
        <div class="studio-demo">
            <StudioDemoHeader
                demo_mode=demo_mode
                set_demo_mode=set_demo_mode
            />

            <div class="studio-demo__content">
                {move || match demo_mode.get() {
                    DemoMode::Timeline => view! { <TimelineDemo /> },
                    DemoMode::Transform3D => view! { <Transform3DDemo /> },
                    DemoMode::Morphing => view! { <MorphingDemo /> },
                    DemoMode::Export => view! { <ExportDemo /> },
                }}
            </div>
        </div>
    }
}

/// Demo modes
#[derive(Clone, Copy, Debug, PartialEq)]
enum DemoMode {
    Timeline,
    Transform3D,
    Morphing,
    Export,
}

/// Studio demo header
#[component]
fn StudioDemoHeader(
    demo_mode: ReadSignal<DemoMode>,
    set_demo_mode: WriteSignal<DemoMode>,
) -> impl IntoView {
    view! {
        <header class="studio-demo__header">
            <h1>"🎬 Motion Studio Demo"</h1>
            <nav class="studio-demo__nav">
                <button
                    class="studio-demo__nav-item"
                    class:active=move || demo_mode.get() == DemoMode::Timeline
                    on:click=move |_| set_demo_mode.set(DemoMode::Timeline)
                >
                    "Timeline"
                </button>
                <button
                    class="studio-demo__nav-item"
                    class:active=move || demo_mode.get() == DemoMode::Transform3D
                    on:click=move |_| set_demo_mode.set(DemoMode::Transform3D)
                >
                    "3D Transforms"
                </button>
                <button
                    class="studio-demo__nav-item"
                    class:active=move || demo_mode.get() == DemoMode::Morphing
                    on:click=move |_| set_demo_mode.set(DemoMode::Morphing)
                >
                    "SVG Morphing"
                </button>
                <button
                    class="studio-demo__nav-item"
                    class:active=move || demo_mode.get() == DemoMode::Export
                    on:click=move |_| set_demo_mode.set(DemoMode::Export)
                >
                    "Export"
                </button>
            </nav>
        </header>
    }
}

/// Timeline demo component
#[component]
fn TimelineDemo() -> impl IntoView {
    let (timeline, set_timeline) =
        create_signal(Timeline3D::new("Demo Timeline".to_string(), 10.0));
    let (selected_property, set_selected_property) = create_signal(AnimationProperty::Translation);

    view! {
        <div class="timeline-demo">
            <div class="timeline-demo__controls">
                <h3>"Timeline Controls"</h3>
                <div class="property-selector">
                    <label>"Property:"</label>
                    <select
                        prop:value=move || format!("{:?}", selected_property.get())
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                    // Temporarily disabled until FromStr is implemented
                    // if let Ok(prop) = value.parse::<AnimationProperty>() {
                    //     set_selected_property.set(prop);
                    // }
                        }
                    >
                        <option value="TranslateX">"Translate X"</option>
                        <option value="TranslateY">"Translate Y"</option>
                        <option value="RotateZ">"Rotate Z"</option>
                        <option value="ScaleX">"Scale X"</option>
                        <option value="Opacity">"Opacity"</option>
                    </select>
                </div>

                <div class="timeline-actions">
                    <button on:click=move |_| {
                        let mut tl = timeline.get();
                        let _ = tl.add_keyframe(
                            selected_property.get(),
                            tl.current_time,
                            AnimationValue::Float(100.0),
                        );
                        set_timeline.set(tl);
                    }>
                        "Add Keyframe"
                    </button>

                    <button on:click=move |_| {
                        let mut tl = timeline.get();
                        tl.play();
                        set_timeline.set(tl);
                    }>
                        "Play"
                    </button>

                    <button on:click=move |_| {
                        let mut tl = timeline.get();
                        tl.pause();
                        set_timeline.set(tl);
                    }>
                        "Pause"
                    </button>
                </div>
            </div>

            <div class="timeline-demo__editor">
                <TimelineEditor
                    timeline=Some(timeline.get())
                    on_timeline_change=move |t| set_timeline.set(t)
                />
            </div>
        </div>
    }
}

/// 3D Transform demo component
#[component]
fn Transform3DDemo() -> impl IntoView {
    let (transform, set_transform) = create_signal(Transform3D::new());
    let (perspective, set_perspective) = create_signal(Perspective::new(45.0, 1.0, 0.1, 100.0));

    view! {
        <div class="transform-3d-demo">
            <div class="transform-3d-demo__controls">
                <h3>"3D Transform Controls"</h3>

                <div class="transform-controls">
                    <div class="control-group">
                        <label>"Translation X:"</label>
                        <input
                            type="range"
                            min="-100"
                            max="100"
                            prop:value=move || transform.get().translation.x
                            on:input=move |ev| {
                                let value: f32 = event_target_value(&ev).parse().unwrap_or(0.0);
                                let mut t = transform.get();
                                t.translation.x = value;
                                set_transform.set(t);
                            }
                        />
                    </div>

                    <div class="control-group">
                        <label>"Translation Y:"</label>
                        <input
                            type="range"
                            min="-100"
                            max="100"
                            prop:value=move || transform.get().translation.y
                            on:input=move |_| {
                                // Similar implementation for Y
                            }
                        />
                    </div>

                    <div class="control-group">
                        <label>"Rotation Z:"</label>
                        <input
                            type="range"
                            min="0"
                            max="360"
                            prop:value=move || transform.get().rotation.to_euler(glam::EulerRot::XYZ).2.to_degrees()
                            on:input=move |_| {
                                // Similar implementation for rotation
                            }
                        />
                    </div>
                </div>
            </div>

            <div class="transform-3d-demo__preview">
                <div
                    class="transform-preview"
                    style:transform=move || {
                        let t = transform.get();
                        let p = perspective.get();
                        format!("perspective({}px) translate3d({}px, {}px, {}px) rotateZ({}deg) scale({}, {}, {})",
                            p.fov,
                            t.translation.x,
                            t.translation.y,
                            t.translation.z,
                            t.rotation.to_euler(glam::EulerRot::XYZ).2.to_degrees(),
                            t.scale.x,
                            t.scale.y,
                            t.scale.z
                        )
                    }
                >
                    <div class="transform-cube">
                        <div class="cube-face front">"Front"</div>
                        <div class="cube-face back">"Back"</div>
                        <div class="cube-face right">"Right"</div>
                        <div class="cube-face left">"Left"</div>
                        <div class="cube-face top">"Top"</div>
                        <div class="cube-face bottom">"Bottom"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// SVG Morphing demo component
#[component]
fn MorphingDemo() -> impl IntoView {
    let (source_path, set_source_path) = create_signal("M 50 50 L 150 50 L 100 150 Z".to_string());
    let (target_path, set_target_path) = create_signal("M 50 100 L 150 100 L 100 50 Z".to_string());
    let (morph_progress, set_morph_progress) = create_signal(0.0);

    view! {
        <div class="morphing-demo">
            <div class="morphing-demo__controls">
                <h3>"SVG Path Morphing"</h3>

                <div class="path-inputs">
                    <div class="path-input">
                        <label>"Source Path:"</label>
                        <textarea
                            prop:value=source_path
                            on:input=move |ev| set_source_path.set(event_target_value(&ev))
                        ></textarea>
                    </div>

                    <div class="path-input">
                        <label>"Target Path:"</label>
                        <textarea
                            prop:value=target_path
                            on:input=move |ev| set_target_path.set(event_target_value(&ev))
                        ></textarea>
                    </div>
                </div>

                <div class="morph-controls">
                    <label>"Morph Progress:"</label>
                    <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        prop:value=morph_progress
                        on:input=move |ev| {
                            let value: f32 = event_target_value(&ev).parse().unwrap_or(0.0);
                            set_morph_progress.set(value);
                        }
                    />
                    <span>{move || format!("{:.2}", morph_progress.get())}</span>
                </div>
            </div>

            <div class="morphing-demo__preview">
                <SvgMorphingEditor
                    from_path=Some(source_path.get())
                    to_path=Some(target_path.get())
                    on_change=move |(src, tgt)| {
                        set_source_path.set(src);
                        set_target_path.set(tgt);
                    }
                />
            </div>
        </div>
    }
}

/// Export demo component
#[component]
fn ExportDemo() -> impl IntoView {
    let (project, set_project) = create_signal(StudioProject::new("Export Demo"));
    let (export_format, set_export_format) = create_signal(ExportFormat::CSS);
    let (export_result, set_export_result) = create_signal(None::<String>);

    // Add some demo animations
    create_effect(move |_| {
        let mut p = project.get();
        p.add_animation("Demo Animation 1");
        p.add_animation("Demo Animation 2");
        set_project.set(p);
    });

    view! {
        <div class="export-demo">
            <div class="export-demo__controls">
                <h3>"Export Animation"</h3>

                <div class="format-selector">
                    <label>"Export Format:"</label>
                    <select
                        prop:value=move || format!("{:?}", export_format.get())
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                    // Temporarily disabled until FromStr is implemented
                    // if let Ok(format) = value.parse::<ExportFormat>() {
                    //     set_export_format.set(format);
                    // }
                        }
                    >
                        <option value="CSS">"CSS"</option>
                        <option value="WAAPI">"Web Animations API"</option>
                        <option value="LeptosMotion">"Leptos Motion"</option>
                        <option value="FramerMotion">"Framer Motion"</option>
                    </select>
                </div>

                <button on:click=move |_| {
                    let exporter = AnimationExporter::new(&project.get());
                    let result = match export_format.get() {
                        ExportFormat::CSS => exporter.export(),
                        ExportFormat::WAAPI => exporter.export(),
                        ExportFormat::LeptosMotion => exporter.export(),
                        ExportFormat::FramerMotion => exporter.export(),
                        _ => Ok(ExportResult {
                            content: "Format not implemented".to_string(),
                            mime_type: "text/plain".to_string(),
                            file_extension: "txt".to_string(),
                            metadata: std::collections::HashMap::new(),
                        }),
                    };

                    if let Ok(export_result) = result {
                        set_export_result.set(Some(export_result.content));
                    }
                }>
                    "Export"
                </button>
            </div>

            <div class="export-demo__result">
                <h4>"Exported Code:"</h4>
                <pre class="export-code">
                    {move || export_result.get().unwrap_or_else(|| "Click Export to generate code".to_string())}
                </pre>
            </div>
        </div>
    }
}

/// Main entry point
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(StudioDemo);
}
