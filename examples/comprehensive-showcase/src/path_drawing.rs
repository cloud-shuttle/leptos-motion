//! MotionPath Drawing Animation Demo
//!
//! Showcase of the Leptos Motion MotionPath component with
//! real-time path length calculation and stroke-dashoffset animation.

use leptos::prelude::*;
use leptos_motion::{MotionPath, AnimationValue, Easing, Transition};
use leptos_motion_dom::AnimateProp;

#[component]
pub fn PathDrawingDemo() -> impl IntoView {
    let (is_playing, set_is_playing) = signal(true);

    view! {
        <div style="background: linear-gradient(135deg, #1e1b4b 0%, #7c3aed 50%, #1e1b4b 100%); min-height: 100vh; color: white; padding: 20px;">
            <h1 style="font-size: 48px; margin-bottom: 20px; text-align: center;">"🎨 MotionPath Drawing Animations"</h1>
            <p style="font-size: 24px; margin-bottom: 40px; opacity: 0.8; text-align: center;">
                "Leptos Motion - Pure Rust/WASM Path Drawing with Real-time Length Calculation"
            </p>

            // Controls
            <div style="display: flex; justify-content: center; gap: 20px; margin-bottom: 40px;">
                <button
                    style="padding: 12px 24px; border: none; border-radius: 8px; font-size: 16px; cursor: pointer; background: rgba(255,255,255,0.2); color: white;"
                    on:click=move |_| set_is_playing.update(|playing| *playing = !*playing)
                >
                    {move || if is_playing.get() { "⏸️ Pause" } else { "▶️ Play" }}
                </button>
            </div>

            <div style="display: flex; justify-content: center;">
                <svg
                    width="800"
                    height="400"
                    viewBox="0 0 800 400"
                    style="max-width: 90vw; border-radius: 20px; background: rgba(255,255,255,0.1); padding: 20px;"
                >
                    // Circle path - should animate from invisible to fully drawn
                    <MotionPath
                        d=String::from("M 100 150 A 80 80 0 1 1 260 150 A 80 80 0 1 1 100 150")
                        stroke=String::from("#ff0088")
                        stroke_width=String::from("8")
                        fill=String::from("transparent")
                        stroke_linecap=String::from("round")
                        animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::new()
                }))
                        _transition=Transition {
                            duration: Some(2.0),
                            ease: Easing::EaseInOut,
                            ..Default::default()
                        }
                    >
                        {|| ()}
                    </MotionPath>

                    // Diagonal line
                    <MotionPath
                        d=String::from("M 350 70 L 550 230")
                        stroke=String::from("#8df0cc")
                        stroke_width=String::from("6")
                        stroke_linecap=String::from("round")
                        animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::new()
                }))
                        _transition=Transition {
                            duration: Some(1.5),
                            delay: Some(0.5),
                            ease: Easing::EaseOut,
                            ..Default::default()
                        }
                    >
                        {|| ()}
                    </MotionPath>

                    // Rectangle path
                    <MotionPath
                        d=String::from("M 620 80 L 720 80 L 720 180 L 620 180 Z")
                        stroke=String::from("#0d63f8")
                        stroke_width=String::from("6")
                        fill=String::from("transparent")
                        stroke_linecap=String::from("round")
                        animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::new()
                }))
                        _transition=Transition {
                            duration: Some(2.0),
                            delay: Some(1.0),
                            ease: Easing::EaseInOut,
                            ..Default::default()
                        }
                    >
                        {|| ()}
                    </MotionPath>
                </svg>
            </div>

            <div style="margin-top: 40px; text-align: center;">
                <p style="font-size: 18px; opacity: 0.8;">
                    "MotionPath components with automatic stroke-dashoffset animation from calculated path lengths."
                </p>
                <p style="font-size: 16px; opacity: 0.6; margin-top: 20px;">
                    "✅ Real-time path length calculation using web_sys::SvgPathElement.getTotalLength()"
                </p>
                <p style="font-size: 16px; opacity: 0.6;">
                    "✅ Hash-based caching for performance optimization"
                </p>
                <p style="font-size: 16px; opacity: 0.6;">
                    "✅ Full stroke-dashoffset animation support in the motion engine"
                </p>
            </div>
        </div>
    }
}
