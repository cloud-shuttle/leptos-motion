use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use leptos_motion_studio::morphing::PathMorpher;
use leptos_motion_dom::*;
use web_sys::HtmlInputElement;
use leptos::wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    web_sys::console::log_1(&"🚀 Path Morphing Demo: Starting".into());
}

#[wasm_bindgen]
pub fn path_morphing_demo() {
    web_sys::console::log_1(&"🎨 Path Morphing Demo: Mounting".into());
    leptos::mount::mount_to_body(|| {
        web_sys::console::log_1(&"✅ Path Morphing Demo: Component created".into());
        view! {
            <div style="
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 2rem;
                padding: 2rem;
                min-height: 100vh;
                background: linear-gradient(135deg, #0b1011 0%, #1d2628 100%);
                color: #8df0cc;
                font-family: system-ui, sans-serif;
            ">
                <h1 style="
                    font-size: 2.5rem;
                    font-weight: bold;
                    text-align: center;
                    margin-bottom: 1rem;
                ">
                    "Leptos Motion Demos"
                </h1>
                <PathMorphingDemo />
                <LayoutControlsDemo />
            </div>
        }
    });
    web_sys::console::log_1(&"✅ Path Morphing Demo: Mounted successfully".into());
}

#[component]
fn PathMorphingDemo() -> impl IntoView {
    let (path_index, set_path_index) = signal(0);
    let (morph_progress, set_morph_progress) = signal(0.0);
    let (is_animating, set_is_animating) = signal(false);

    // Complex shape data - sophisticated paths like the React Motion example
    let paths = [
        "M7 2v11h3v9l7-12h-4l4-8z", // lightning
        "M23 5.5V20c0 2.2-1.8 4-4 4h-7.3c-1.08 0-2.1-.43-2.85-1.19L1 14.83s1.26-1.23 1.3-1.25c.22-.19.49-.29.79-.29.22 0 .42.06.6.16.04.01 4.31 2.46 4.31 2.46V4c0-.83.67-1.5 1.5-1.5S11 3.17 11 4v7h1V1.5c0-.83.67-1.5 1.5-1.5S15 .67 15 1.5V11h1V2.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5V11h1V5.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5z", // hand
        "M21 16v-2l-8-5V3.5c0-.83-.67-1.5-1.5-1.5S10 2.67 10 3.5V9l-8 5v2l8-2.5V19l-2 1.5V22l3.5-1 3.5 1v-1.5L13 19v-5.5l8 2.5z", // plane
        "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z", // heart
        "M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z", // note
        "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z", // star
    ];

    let colors = [
        "#fff312", // yellow - lightning
        "#ff0088", // pink - hand
        "#dd00ee", // purple - plane
        "#9911ff", // violet - heart
        "#0d63f8", // blue - note
        "#0cdcf7", // cyan - star
    ];

    let shape_names = [
        "Lightning", "Hand", "Plane", "Heart", "Note", "Star"
    ];

    // Note: We no longer need to pre-create morphers since the new API handles everything automatically

    // Animation state management
    let (animation_interval, set_animation_interval) = signal::<Option<i32>>(None);

    // Start animation function
    let start_animation = move || {
        if animation_interval.get().is_some() {
            return; // Already animating
        }

        let set_morph_progress_clone = set_morph_progress.clone();
        let set_is_animating_clone = set_is_animating.clone();
        let set_path_index_clone = set_path_index.clone();
        let set_animation_interval_clone = set_animation_interval.clone();
        let path_index_clone = path_index.clone();
        let paths_len = paths.len();

        // Animation using setInterval with true path morphing
        let closure = Closure::wrap(Box::new(move || {
            let current_progress = morph_progress.get();
            let new_progress = current_progress + 0.01; // 1% per frame

            if new_progress >= 1.0 {
                // Animation complete
                set_is_animating_clone.set(false);
                set_morph_progress_clone.set(0.0);

                // Move to next shape
                let next_index = (path_index_clone.get() + 1) % paths_len;
                set_path_index_clone.set(next_index);

                // Clear the interval
                if let Some(interval_id) = animation_interval.get() {
                    web_sys::window().unwrap().clear_interval_with_handle(interval_id);
                    set_animation_interval_clone.set(None);
                }
            } else {
                set_morph_progress_clone.set(new_progress);
            }
        }) as Box<dyn FnMut()>);

        let interval_id = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                16, // ~60fps for smooth animation
            )
            .unwrap();

        set_animation_interval.set(Some(interval_id));
        closure.forget();
    };

    // Stop animation function
    let stop_animation = move || {
        if let Some(interval_id) = animation_interval.get() {
            web_sys::window().unwrap().clear_interval_with_handle(interval_id);
            set_animation_interval.set(None);
        }
        set_is_animating.set(false);
        set_morph_progress.set(0.0);
    };

    // Effect to start animation when is_animating becomes true
    Effect::new(move |_| {
        if is_animating.get() && animation_interval.get().is_none() {
            start_animation();
        }
    });

    // Click handler to trigger morphing
    let handle_click = move |_| {
        if !is_animating.get() {
            set_is_animating.set(true);
        }
    };

    // Right-click handler to stop animation
    let handle_right_click = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        stop_animation();
    };

    // Keyboard handler for space to trigger morphing
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == " " || ev.key() == "Enter" {
            ev.prevent_default();
            if !is_animating.get() {
                set_is_animating.set(true);
            }
        } else if ev.key() == "Escape" {
            ev.prevent_default();
            stop_animation();
        }
    };


    // Get interpolated path using the simplified PathMorpher API
    let current_path = Memo::new(move |_| {
        let current_idx = path_index.get();
        let progress = morph_progress.get();
        let animating = is_animating.get();
        
        if animating && progress > 0.0 {
            let next_idx = (current_idx + 1) % paths.len();
            
            // Use the simple morph function - it handles everything automatically
            let mut morpher = PathMorpher::new();
            let interpolated_path = morpher.morph(&paths[current_idx], &paths[next_idx], progress);
            return interpolated_path.to_data();
            }
        }
        
        // Return current path when not animating
        paths[current_idx].to_string()
    });

    // Get interpolated color with smooth transition
    let current_color = Memo::new(move |_| {
        let current_idx = path_index.get();
        let next_idx = (current_idx + 1) % colors.len();
        let progress = morph_progress.get();
        
        if is_animating.get() && progress > 0.0 {
            // Simple color interpolation (you could implement more sophisticated color morphing)
            let current_color = colors[current_idx];
            let next_color = colors[next_idx];
            
            // For now, just return the current color
            // In a full implementation, you'd interpolate between hex colors
            current_color
        } else {
            colors[current_idx]
        }
    });

    view! {
        <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            font-family: system-ui, sans-serif;
            color: white;
            padding: 2rem;
        ">
            <h1 style="
                font-size: 3rem;
                margin-bottom: 1rem;
                text-align: center;
                text-shadow: 0 2px 4px rgba(0,0,0,0.3);
            ">
                "⚡ Advanced Path Morphing Demo"
            </h1>
            
            <p style="
                font-size: 1.2rem;
                margin-bottom: 1rem;
                text-align: center;
                opacity: 0.9;
            ">
                "Complex SVG shapes morphing with leptos-motion-studio PathMorpher"
            </p>
            
            <p style="
                font-size: 1rem;
                margin-bottom: 2rem;
                text-align: center;
                opacity: 0.7;
                font-style: italic;
            ">
                "Lightning → Hand → Plane → Heart → Note → Star morphing"
            </p>

            <div 
                style="
                    position: relative;
                    width: 400px;
                    height: 400px;
                    background: rgba(255, 255, 255, 0.1);
                    border-radius: 20px;
                    backdrop-filter: blur(10px);
                    border: 1px solid rgba(255, 255, 255, 0.2);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-bottom: 2rem;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                    cursor: pointer;
                    transition: transform 0.2s ease;
                    user-select: none;
                "
                on:click=handle_click
                on:contextmenu=handle_right_click
                on:keydown=handle_keydown
                tabindex="0"
            >
                <svg 
                    width="400" 
                    height="400" 
                    viewBox="0 0 400 400"
                    style="
                        transition: all 0.3s ease;
                        filter: drop-shadow(0 4px 8px rgba(0,0,0,0.3));
                    "
                >
                    <g transform="translate(10 10) scale(17 17)">
                        <path 
                            fill=current_color
                            d=current_path
                            style=move || format!("
                                transition: fill 0.3s ease;
                                transform-origin: center;
                                opacity: {};
                            ", if is_animating.get() { "0.8" } else { "1.0" })
                        />
                    </g>
                </svg>
            </div>

            <div style="
                text-align: center;
                margin-bottom: 2rem;
            ">
                <h2 style="
                    font-size: 2rem;
                    margin-bottom: 0.5rem;
                    color: #fff312;
                    text-shadow: 0 2px 4px rgba(0,0,0,0.3);
                ">
                    {move || shape_names[path_index.get() % shape_names.len()]}
                </h2>
                <p style="
                    font-size: 1rem;
                    opacity: 0.8;
                ">
                    "Shape: " {move || format!("{}", path_index.get() + 1)} "/" {paths.len()}
                </p>
                <p style="
                    font-size: 0.9rem;
                    opacity: 0.6;
                    margin-top: 0.5rem;
                ">
                    "Morph Progress: " {move || format!("{:.1}%", morph_progress.get() * 100.0)}
                </p>
                <p style="
                    font-size: 0.8rem;
                    opacity: 0.5;
                    margin-top: 0.3rem;
                    font-family: monospace;
                ">
                    {move || if is_animating.get() { "🔄 PathMorpher Active" } else { "⏸️ Ready" }}
                </p>
            </div>

            <div style="
                display: flex;
                gap: 1rem;
                flex-wrap: wrap;
                justify-content: center;
            ">
                {(0..paths.len()).map(|i| {
                    let is_active = move || path_index.get() == i;
                    
                    view! {
                        <div
                            style=move || format!("
                                width: 12px;
                                height: 12px;
                                border-radius: 50%;
                                background: {};
                                transition: all 0.3s ease;
                                transform: scale({});
                                box-shadow: 0 2px 4px rgba(0,0,0,0.3);
                            ", 
                            colors[i],
                            if is_active() { "1.2" } else { "1.0" }
                            )
                        ></div>
                    }
                }).collect_view()}
            </div>

            <div style="
                margin-top: 2rem;
                text-align: center;
                opacity: 0.7;
            ">
                <p style="font-size: 0.9rem; margin-bottom: 0.5rem;">
                    "🖱️ Click or Space to morph • 🖱️ Right-click or Esc to stop • 🎨 Color transitions • 📐 True path interpolation"
                </p>
                <p style="font-size: 0.8rem; opacity: 0.6;">
                    "⚡ 60fps • 🧮 Simple API • 🐌 Slower for visibility • 🚀 PathMorpher powered"
                </p>
            </div>
        </div>
    }
}

#[component]
fn LayoutControlsDemo() -> impl IntoView {
    let (aspect_ratio, set_aspect_ratio) = signal(1.0);
    let (width, set_width) = signal(100.0);

    view! {
        <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 2rem;
            padding: 2rem;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 20px;
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            max-width: 600px;
            width: 100%;
        ">
            <h2 style="
                font-size: 1.8rem;
                font-weight: bold;
                text-align: center;
                margin-bottom: 1rem;
            ">
                "Layout Controls Demo"
            </h2>
            
            <div class="container" style="
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 300px;
                height: 300px;
                gap: 20px;
                margin-bottom: 2rem;
            ">
                <MotionDiv
                    class="box".to_string()
                    style=(move || format!("
                        background-color: #8df0cc;
                        position: relative;
                        z-index: 1;
                        aspect-ratio: {};
                        width: {}px;
                        border-radius: 20px;
                    ", aspect_ratio.get(), width.get()))()
                    _layout=true
                    _transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                >
                    <div></div>
                </MotionDiv>
            </div>
            
            <div class="inputContainer" style="
                display: flex;
                flex-direction: row;
                gap: 20px;
                background-color: rgba(0, 0, 0, 0.2);
                padding: 20px 40px;
                border-radius: 10px;
                position: relative;
                z-index: 2;
                border: 1px solid rgba(255, 255, 255, 0.1);
            ">
                <div class="inputs" style="
                    display: flex;
                    flex-direction: column;
                    padding-left: 50px;
                ">
                    <Input
                        value=aspect_ratio
                        set=set_aspect_ratio
                        min=0.1
                        max=5.0
                        step=0.1
                    >
                        "Aspect ratio"
                    </Input>
                    <Input
                        value=width
                        set=set_width
                        min=10.0
                        max=1000.0
                        step=5.0
                    >
                        "Width"
                    </Input>
                </div>
            </div>
            
            <div style="
                margin-top: 1rem;
                text-align: center;
                opacity: 0.7;
            ">
                <p style="font-size: 0.9rem; margin-bottom: 0.5rem;">
                    "🎛️ Adjust sliders to see smooth layout animations • 📐 Aspect ratio changes shape • 📏 Width changes size"
                </p>
                <p style="font-size: 0.8rem; opacity: 0.6;">
                    "⚡ FLIP animations • 🎨 Real-time updates • 🚀 Leptos Motion powered"
                </p>
            </div>
        </div>
    }
}

#[component]
fn Input(
    value: ReadSignal<f64>,
    set: WriteSignal<f64>,
    min: f64,
    max: f64,
    step: f64,
    children: Children,
) -> impl IntoView {
    let children = children();

    let handle_range_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        let new_value = input.value().parse::<f64>().unwrap_or(0.0);
        set.set(new_value);
    };

    let handle_number_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        let new_value = input.value().parse::<f64>().unwrap_or(0.0);
        set.set(new_value);
    };

    view! {
        <label style="
            display: flex;
            align-items: center;
            margin: 10px 0;
            font-size: 12px;
        ">
            <code style="width: 100px;">{children}</code>
            <input
                type="range"
                min=min
                max=max
                step=step
                value=move || value.get()
                on:input=handle_range_change
                style="
                    accent-color: #8df0cc;
                    font-family: 'Azeret Mono', monospace;
                    font-size: 12px;
                    margin-right: 10px;
                "
            />
            <input
                type="number"
                min=min
                max=max
                value=move || value.get()
                on:input=handle_number_change
                style="
                    border: 0;
                    border-bottom: 1px dotted #8df0cc;
                    color: #8df0cc;
                    margin-left: 10px;
                    background: transparent;
                    font-family: 'Azeret Mono', monospace;
                    font-size: 12px;
                    width: 80px;
                "
            />
        </label>
    }
}