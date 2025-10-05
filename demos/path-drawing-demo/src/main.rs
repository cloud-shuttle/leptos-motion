use leptos::prelude::*;
use leptos_motion::{MotionPath, AnimationValue, Easing, Transition};
use leptos_motion_dom::AnimateProp;
use std::rc::Rc;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (is_playing, set_playing) = signal(false);
    
    // Start animation when playing
    Effect::new(move |_| {
        if is_playing.get() {
            // Auto-stop after animation completes
            let timeout = set_timeout_with_handle(move || {
                set_playing.set(false);
            }, std::time::Duration::from_millis(8000)); // 8 seconds for full animation
            
            Box::new(move || {
                if let Ok(handle) = timeout {
                    handle.clear();
                }
            }) as Box<dyn Fn()>
        } else {
            Box::new(|| {}) as Box<dyn Fn()>
        }
    });

    view! {
        <div style="
            min-height: 100vh;
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            color: white;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 2rem;
        ">
            <h1 style="
                font-size: 3rem;
                font-weight: 900;
                text-align: center;
                margin-bottom: 2rem;
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1);
                background-size: 300% 300%;
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            ">
                "🎨 Path Drawing Animation"
            </h1>

            <p style="
                text-align: center;
                font-size: 1.2rem;
                margin-bottom: 2rem;
                opacity: 0.9;
                font-weight: 300;
            ">
                "Pure Rust/WASM path drawing with automatic length calculation using web_sys::SvgPathElement.getTotalLength()"
            </p>

            // Control button
            <button
                on:click=move |_| set_playing.update(|v| *v = !*v)
                style="
                    padding: 1rem 2rem;
                    border: none;
                    border-radius: 50px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                    color: white;
                    font-size: 1.1rem;
                    font-weight: 700;
                    cursor: pointer;
                    box-shadow: 0 10px 30px rgba(255, 107, 107, 0.4);
                    transition: all 0.3s ease;
                    margin-bottom: 2rem;
                "
            >
                {move || if is_playing.get() { "⏸️ Pause Drawing" } else { "▶️ Start Drawing" }}
            </button>

            // SVG Canvas
            <div style="
                background: rgba(255,255,255,0.05);
                border-radius: 20px;
                padding: 2rem;
                backdrop-filter: blur(20px);
                border: 1px solid rgba(255,255,255,0.1);
            ">
                <PathDrawingSVG is_playing=is_playing />
            </div>
        </div>
    }
}

#[component]
fn PathDrawingSVG(is_playing: ReadSignal<bool>) -> impl IntoView {
    view! {
        <svg
            width="600"
            height="600"
            viewBox="0 0 600 600"
            style="
                max-width: 80vw;
                width: 100%;
                height: 100%;
            "
        >
            // Row 1 - Circle
            <MotionPath
                d=String::from("M 20 100 A 80 80 0 1 1 180 100 A 80 80 0 1 1 20 100")
                stroke=String::from("#ff0088")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(2.0),
                    delay: Some(0.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 1 - Diagonal Lines (X)
            <MotionPath
                d=String::from("M 240 30 L 360 150")
                stroke=String::from("#8df0cc")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(1.0),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            <MotionPath
                d=String::from("M 240 150 L 360 30")
                stroke=String::from("#8df0cc")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(1.25),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 1 - Rectangle
            <MotionPath
                d=String::from("M 430 30 L 550 30 L 550 150 L 430 150 Z")
                stroke=String::from("#0d63f8")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(2.0),
                    delay: Some(1.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 2 - Circle
            <MotionPath
                d=String::from("M 20 300 A 80 80 0 1 1 180 300 A 80 80 0 1 1 20 300")
                stroke=String::from("#0d63f8")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
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

            // Row 2 - Diagonal Lines (X)
            <MotionPath
                d=String::from("M 240 230 L 360 350")
                stroke=String::from("#ff0088")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(1.5),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            <MotionPath
                d=String::from("M 240 350 L 360 230")
                stroke=String::from("#ff0088")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(1.75),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 2 - Rectangle
            <MotionPath
                d=String::from("M 430 230 L 550 230 L 550 350 L 430 350 Z")
                stroke=String::from("#8df0cc")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(2.0),
                    delay: Some(2.0),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 3 - Circle
            <MotionPath
                d=String::from("M 20 500 A 80 80 0 1 1 180 500 A 80 80 0 1 1 20 500")
                stroke=String::from("#8df0cc")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(2.0),
                    delay: Some(1.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 3 - Diagonal Lines (X)
            <MotionPath
                d=String::from("M 240 430 L 360 550")
                stroke=String::from("#0d63f8")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(2.0),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            <MotionPath
                d=String::from("M 240 550 L 360 430")
                stroke=String::from("#0d63f8")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(1.5),
                    delay: Some(2.25),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>

            // Row 3 - Rectangle
            <MotionPath
                d=String::from("M 430 430 L 550 430 L 550 550 L 430 550 Z")
                stroke=String::from("#ff0088")
                stroke_width=String::from("10")
                stroke_linecap=String::from("round")
                fill=String::from("transparent")
                initial=std::collections::HashMap::from([
                    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(2000.0)) // Start hidden - large offset to ensure paths are hidden
                ])
                animate=AnimateProp::Derived(Memo::new(move |_| if is_playing.get() {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
                    ])
                } else {
                    std::collections::HashMap::from([
                        ("stroke-dashoffset".to_string(), AnimationValue::Pixels(500.0)) // Hide by offsetting
                    ])
                }))
                _transition=Transition {
                    duration: Some(2.0),
                    delay: Some(2.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {|| ()}
            </MotionPath>
        </svg>
    }
}
