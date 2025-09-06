use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_motion::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

#[component]
pub fn App() -> impl IntoView {
    let (count, _set_count) = signal(0);
    let (is_visible, _set_is_visible) = signal(true);
    let (layout_mode, _set_layout_mode) = signal(false);

    view! {
        <div class="app">
            <h1>"Leptos Motion - Advanced Features! 🚀"</h1>

            <div class="demo-section">
                <h2>"✅ What We Just Implemented:"</h2>
                <ul>
                    <li>"Gesture Integration Framework"</li>
                    <li>"FLIP Animation System"</li>
                    <li>"Layout Change Detection"</li>
                    <li>"Advanced Animation Engine"</li>
                    <li>"Multi-touch Support"</li>
                </ul>
            </div>

            <div class="demo-section">
                <h2>"🎬 Animation Demo:"</h2>

                <MotionDiv
                    class="animated-box".to_string()
                    initial=motion_target!(
                        "opacity" => AnimationValue::Number(0.0),
                        "scale" => AnimationValue::Number(0.5)
                    )
                    animate=motion_target!(
                        "opacity" => AnimationValue::Number(1.0),
                        "scale" => AnimationValue::Number(1.0)
                    )
                    transition=Transition {
                        duration: Some(0.8),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    "Fade In + Scale Animation"
                </MotionDiv>

                <div class="button-group">
                    <button class="button" on:click=move |_| {
                        let new_visibility = !is_visible.get();
                        _set_is_visible.set(new_visibility);
                    }>
                        {move || if is_visible.get() { "Hide" } else { "Show" }}
                    </button>

                    <button class="button" on:click=move |_| {
                        let new_count = count.get() + 1;
                        _set_count.set(new_count);
                    }>
                        {move || format!("Count: {}", count.get())}
                    </button>
                </div>

                <Show
                    when=move || is_visible.get()
                    fallback=|| view! { <div class="hidden">"Hidden Content"</div> }
                >
                    <MotionDiv
                        class="content-box".to_string()
                        initial=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "y" => AnimationValue::Pixels(50.0)
                        )
                        animate=motion_target!(
                            "opacity" => AnimationValue::Number(1.0),
                            "y" => AnimationValue::Pixels(0.0)
                        )
                        exit=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "y" => AnimationValue::Pixels(-50.0)
                        )
                        transition=Transition {
                            duration: Some(0.5),
                            ease: Easing::EaseInOut,
                            ..Default::default()
                        }
                    >
                        <h3>"Dynamic Content"</h3>
                        <p>"This content animates in and out smoothly!"</p>
                        <p>"Count: " {count}</p>
                    </MotionDiv>
                </Show>
            </div>

            <div class="demo-section">
                <h2>"🔄 FLIP Layout Animations:"</h2>

                <div class="layout-demo">
                    <button
                        class="button"
                        on:click=move |_| {
                            let new_mode = !layout_mode.get();
                            _set_layout_mode.set(new_mode);
                        }
                    >
                        {move || if layout_mode.get() { "Switch to Grid" } else { "Switch to List" }}
                    </button>

                    <div class={move || if layout_mode.get() { "grid-layout" } else { "list-layout" }}>
                        <MotionDiv
                            class="layout-item".to_string()
                            layout=true
                            transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseInOut,
                                ..Default::default()
                            }
                        >
                            "Item 1"
                        </MotionDiv>

                        <MotionDiv
                            class="layout-item".to_string()
                            layout=true
                            transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseInOut,
                                ..Default::default()
                            }
                        >
                            "Item 2"
                        </MotionDiv>

                        <MotionDiv
                            class="layout-item".to_string()
                            layout=true
                            transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseInOut,
                                ..Default::default()
                            }
                        >
                            "Item 3"
                        </MotionDiv>

                        <MotionDiv
                            class="layout-item".to_string()
                            layout=true
                            transition=Transition {
                                duration: Some(0.6),
                                ease: Easing::EaseInOut,
                                ..Default::default()
                            }
                        >
                            "Item 4"
                        </MotionDiv>
                    </div>
                </div>
            </div>

            <div class="demo-section">
                <h2>"👆 Gesture Integration:"</h2>

                <MotionDiv
                    class="gesture-box".to_string()
                    while_hover=motion_target!(
                        "scale" => AnimationValue::Number(1.1)
                    )
                    while_tap=motion_target!(
                        "scale" => AnimationValue::Number(0.95)
                    )
                >
                    <h3>"Interactive Box!"</h3>
                    <p>"Try hovering and tapping this box"</p>
                    <p>"Drag support coming soon!"</p>
                </MotionDiv>
            </div>

            <div class="demo-section">
                <h2>"📱 Multi-touch Support:"</h2>

                <MotionDiv
                    class="touch-box".to_string()
                    while_hover=motion_target!(
                        "scale" => AnimationValue::Number(1.05)
                    )
                    while_tap=motion_target!(
                        "scale" => AnimationValue::Number(0.98)
                    )
                >
                    <h3>"Touch Interactive"</h3>
                    <p>"Supports touch, mouse, and pointer events"</p>
                    <p>"Color animations coming soon!"</p>
                </MotionDiv>
            </div>

            <div class="demo-section">
                <h2>"🔧 Technical Features:"</h2>
                <ul>
                    <li>"Spring Physics Animation Engine"</li>
                    <li>"Hardware Accelerated Transforms"</li>
                    <li>"Performance Optimized RAF Loop"</li>
                    <li>"Type-safe Animation API"</li>
                    <li>"WASM-powered for Maximum Performance"</li>
                </ul>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    // Initialize console logging for debugging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    log::info!("Starting Leptos Motion Showcase app");

    // Try using mount_to_body with spawn_local - this works in Leptos v0.8.x
    wasm_bindgen_futures::spawn_local(async move {
        mount_to_body(|| view! { <App/> });
    });
}
