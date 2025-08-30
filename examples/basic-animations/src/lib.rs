//! Basic animations example for Leptos Motion

use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let (scale, set_scale) = signal(1.0);
    let (rotation, set_rotation) = signal(0.0);

    view! {
        <div class="container">
            <h1>"Leptos Motion - Basic Animations"</h1>

            <div class="controls">
                <button
                    on:click=move |_| set_visible.set(!visible.get())
                >
                    "Toggle Visibility"
                </button>

                <button
                    on:click=move |_| set_scale.set(if scale.get() == 1.0 { 1.5 } else { 1.0 })
                >
                    "Toggle Scale"
                </button>

                <button
                    on:click=move |_| set_rotation.set(rotation.get() + 90.0)
                >
                    "Rotate 90°"
                </button>
            </div>

            <div class="examples">
                // Fade animation
                <MotionDiv
                    class="box fade-box".to_string()
                    animate=(move || motion_target!(
                        "opacity" => AnimationValue::Number(if visible.get() { 1.0 } else { 0.0 })
                    ))()
                    transition=Transition {
                        duration: Some(0.5),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    "Fade Animation"
                </MotionDiv>

                // Scale animation
                <MotionDiv
                    class="box scale-box".to_string()
                    animate=(move || motion_target!(
                        "scale" => AnimationValue::Number(scale.get())
                    ))()
                    transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::BackOut,
                        ..Default::default()
                    }
                >
                    "Scale Animation"
                </MotionDiv>

                // Rotation animation
                <MotionDiv
                    class="box rotate-box".to_string()
                    animate=(move || motion_target!(
                        "rotate" => AnimationValue::Degrees(rotation.get())
                    ))()
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
                    "Rotation Animation"
                </MotionDiv>

                // Hover animation
                <MotionDiv
                    class="box hover-box".to_string()
                    while_hover=motion_target!(
                        "scale" => AnimationValue::Number(1.1),
                        "rotate" => AnimationValue::Degrees(5.0)
                    )
                    transition=Transition {
                        duration: Some(0.2),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    "Hover Me"
                </MotionDiv>

                // Combined animation
                <MotionDiv
                    class="box combined-box".to_string()
                    animate=(move || motion_target!(
                        "x" => AnimationValue::Pixels(if visible.get() { 0.0 } else { 100.0 }),
                        "y" => AnimationValue::Pixels(if visible.get() { 0.0 } else { -50.0 }),
                        "rotate" => AnimationValue::Degrees(if visible.get() { 0.0 } else { 180.0 }),
                        "scale" => AnimationValue::Number(if visible.get() { 1.0 } else { 0.8 })
                    ))()
                    transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::Spring(SpringConfig::default()),
                        ..Default::default()
                    }
                >
                    "Combined Animation"
                </MotionDiv>
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