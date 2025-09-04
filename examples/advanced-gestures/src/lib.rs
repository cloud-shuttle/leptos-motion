use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn App() -> impl IntoView {
    let (scale, set_scale) = signal(1.0);
    let (rotation, set_rotation) = signal(0.0);
    let (position, set_position) = signal((0.0, 0.0));
    let (gesture_info, set_gesture_info) = signal(String::new());
    let (tap_count, set_tap_count) = signal(0);
    let (drag_info, set_drag_info) = signal(String::new());

    // Simple gesture handling without complex dependencies
    let handle_tap = move |_| {
        set_tap_count.update(|count| *count += 1);
        set_gesture_info.set(format!("Tap detected! Count: {}", tap_count.get_untracked() + 1));
    };

    let handle_drag = move |event: web_sys::MouseEvent| {
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;
        set_position.set((x, y));
        set_drag_info.set(format!("Drag: ({:.0}, {:.0})", x, y));
    };

    let handle_wheel = move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let delta = event.delta_y();
        let scale_change = if delta > 0.0 { 0.95 } else { 1.05 };
        set_scale.update(|s| *s *= scale_change);
        set_gesture_info.set(format!("Wheel zoom: Scale {:.2}", scale_change));
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        match event.key().as_str() {
            "r" => {
                set_rotation.update(|r| *r += 15.0);
                set_gesture_info.set("Key 'R': Rotate 15° clockwise".to_string());
            }
            "l" => {
                set_rotation.update(|r| *r -= 15.0);
                set_gesture_info.set("Key 'L': Rotate 15° counter-clockwise".to_string());
            }
            "z" => {
                set_scale.update(|s| *s *= 1.1);
                set_gesture_info.set(format!("Key 'Z': Zoom in, Scale {:.2}", scale.get()));
            }
            "x" => {
                set_scale.update(|s| *s *= 0.9);
                set_gesture_info.set(format!("Key 'X': Zoom out, Scale {:.2}", scale.get()));
            }
            " " => {
                set_scale.set(1.0);
                set_rotation.set(0.0);
                set_position.set((0.0, 0.0));
                set_gesture_info.set("Spacebar: Reset all transformations".to_string());
            }
            _ => {}
        }
    };

    view! {
        <div class="advanced-gestures-container">
            <header class="gestures-header">
                <h1>"Advanced Gesture Recognition Demo"</h1>
                <p>"Try keyboard shortcuts, mouse wheel, and click interactions!"</p>
            </header>

            <main class="gestures-content">
                <section class="gesture-demo">
                    <h2>"Interactive Gesture Area"</h2>
                    
                    <div 
                        class="gesture-area"
                        on:click=handle_tap
                        on:mousemove=handle_drag
                        on:wheel=handle_wheel
                        on:keydown=handle_keydown
                        tabindex="0"
                        style="transform: scale({scale}) rotate({rotation}deg) translate({position.0}px, {position.1}px);"
                    >
                        <div class="gesture-content">
                            <h3>"Gesture Target"</h3>
                            <p>"Click to count • Mouse wheel to zoom • Keys: R/L rotate, Z/X zoom, Space reset"</p>
                            <div class="gesture-stats">
                                <div class="stat">
                                    <span class="label">"Scale:"</span>
                                    <span class="value">{move || format!("{:.2}", scale.get())}</span>
                                </div>
                                <div class="stat">
                                    <span class="label">"Rotation:"</span>
                                    <span class="value">{move || format!("{:.1}°", rotation.get())}</span>
                                </div>
                                <div class="stat">
                                    <span class="label">"Position:"</span>
                                    <span class="value">{move || format!("({:.0}, {:.0})", position.get().0, position.get().1)}</span>
                                </div>
                                <div class="stat">
                                    <span class="label">"Taps:"</span>
                                    <span class="value">{tap_count}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="gesture-info">
                    <h2>"Gesture Information"</h2>
                    <div class="info-panel">
                        <div class="info-item">
                            <h3>"Current Gesture"</h3>
                            <p class="gesture-status">{gesture_info}</p>
                        </div>
                        <div class="info-item">
                            <h3>"Drag Info"</h3>
                            <p class="drag-status">{drag_info}</p>
                        </div>
                    </div>
                </section>

                <section class="gesture-instructions">
                    <h2>"How to Use"</h2>
                    <div class="instructions-grid">
                        <div class="instruction">
                            <h3>"Mouse Wheel Zoom"</h3>
                            <p>"Scroll up/down to zoom in/out of the gesture area"</p>
                        </div>
                        <div class="instruction">
                            <h3>"Keyboard Rotation"</h3>
                            <p>"Press 'R' to rotate clockwise, 'L' to rotate counter-clockwise"</p>
                        </div>
                        <div class="instruction">
                            <h3>"Keyboard Zoom"</h3>
                            <p>"Press 'Z' to zoom in, 'X' to zoom out"</p>
                        </div>
                        <div class="instruction">
                            <h3>"Click Counter"</h3>
                            <p>"Click the gesture area to increment the tap counter"</p>
                        </div>
                        <div class="instruction">
                            <h3>"Mouse Tracking"</h3>
                            <p>"Move your mouse over the area to see position coordinates"</p>
                        </div>
                        <div class="instruction">
                            <h3>"Reset"</h3>
                            <p>"Press Spacebar to reset all transformations to default values"</p>
                        </div>
                    </div>
                </section>
            </main>
        </div>
    }
}

#[component]
pub fn AdvancedGesturesApp() -> impl IntoView {
    view! {
        <div class="app-container">
            <App />
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    
    mount_to_body(|| view! { <AdvancedGesturesApp /> });
}
