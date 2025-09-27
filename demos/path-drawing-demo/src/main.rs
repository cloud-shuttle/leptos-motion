use leptos::prelude::*;

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
                "SVG path drawing with Leptos Motion + WASM (Rust equivalent of Framer Motion)"
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
            // Row 1
            <AnimatedCircle 
                cx="100" cy="100" r="80" stroke="#ff0088" delay=0.5 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="30" x2="360" y2="170" stroke="#8df0cc" delay=1.0 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="170" x2="360" y2="30" stroke="#8df0cc" delay=1.25 is_playing=is_playing
            />
            <AnimatedRect 
                x="410" y="30" width="140" height="140" rx="20" stroke="#0d63f8" delay=1.5 is_playing=is_playing
            />
            
            // Row 2
            <AnimatedCircle 
                cx="100" cy="300" r="80" stroke="#0d63f8" delay=1.0 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="230" x2="360" y2="370" stroke="#ff0088" delay=1.5 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="370" x2="360" y2="230" stroke="#ff0088" delay=1.75 is_playing=is_playing
            />
            <AnimatedRect 
                x="410" y="230" width="140" height="140" rx="20" stroke="#8df0cc" delay=2.0 is_playing=is_playing
            />
            
            // Row 3
            <AnimatedCircle 
                cx="100" cy="500" r="80" stroke="#8df0cc" delay=1.5 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="430" x2="360" y2="570" stroke="#0d63f8" delay=2.0 is_playing=is_playing
            />
            <AnimatedLine 
                x1="220" y1="570" x2="360" y2="430" stroke="#0d63f8" delay=2.25 is_playing=is_playing
            />
            <AnimatedRect 
                x="410" y="430" width="140" height="140" rx="20" stroke="#ff0088" delay=2.5 is_playing=is_playing
            />
        </svg>
    }
}

#[component]
fn AnimatedCircle(
    cx: &'static str,
    cy: &'static str, 
    r: &'static str,
    stroke: &'static str,
    delay: f64,
    is_playing: ReadSignal<bool>
) -> impl IntoView {
    
    view! {
        <circle
            cx=cx
            cy=cy
            r=r
            stroke=stroke
            stroke-width="10"
            stroke-linecap="round"
            fill="transparent"
            style=move || {
                let playing = is_playing.get_untracked();
                if playing {
                    let radius = r.parse::<f64>().unwrap_or(80.0);
                    let path_length = 2.0 * std::f64::consts::PI * radius;
                    let progress = ((js_sys::Date::now() / 1000.0 - delay).max(0.0) * 0.5).min(1.0);
                    let spring_progress = 1.0 - (1.0 - progress).powi(3);
                    let offset = path_length * (1.0 - spring_progress);
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 1;", path_length, offset)
                } else {
                    let radius = r.parse::<f64>().unwrap_or(80.0);
                    let path_length = 2.0 * std::f64::consts::PI * radius;
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 0;", path_length, path_length)
                }
            }
        />
    }
}

#[component]
fn AnimatedLine(
    x1: &'static str,
    y1: &'static str,
    x2: &'static str,
    y2: &'static str,
    stroke: &'static str,
    delay: f64,
    is_playing: ReadSignal<bool>
) -> impl IntoView {
    
    view! {
        <line
            x1=x1
            y1=y1
            x2=x2
            y2=y2
            stroke=stroke
            stroke-width="10"
            stroke-linecap="round"
            style=move || {
                let playing = is_playing.get_untracked();
                if playing {
                    let x1_val = x1.parse::<f64>().unwrap_or(0.0);
                    let y1_val = y1.parse::<f64>().unwrap_or(0.0);
                    let x2_val = x2.parse::<f64>().unwrap_or(0.0);
                    let y2_val = y2.parse::<f64>().unwrap_or(0.0);
                    let path_length = ((x2_val - x1_val).powi(2) + (y2_val - y1_val).powi(2)).sqrt();
                    let progress = ((js_sys::Date::now() / 1000.0 - delay).max(0.0) * 0.5).min(1.0);
                    let spring_progress = 1.0 - (1.0 - progress).powi(3);
                    let offset = path_length * (1.0 - spring_progress);
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 1;", path_length, offset)
                } else {
                    let x1_val = x1.parse::<f64>().unwrap_or(0.0);
                    let y1_val = y1.parse::<f64>().unwrap_or(0.0);
                    let x2_val = x2.parse::<f64>().unwrap_or(0.0);
                    let y2_val = y2.parse::<f64>().unwrap_or(0.0);
                    let path_length = ((x2_val - x1_val).powi(2) + (y2_val - y1_val).powi(2)).sqrt();
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 0;", path_length, path_length)
                }
            }
        />
    }
}

#[component]
fn AnimatedRect(
    x: &'static str,
    y: &'static str,
    width: &'static str,
    height: &'static str,
    rx: &'static str,
    stroke: &'static str,
    delay: f64,
    is_playing: ReadSignal<bool>
) -> impl IntoView {
    
    view! {
        <rect
            x=x
            y=y
            width=width
            height=height
            rx=rx
            stroke=stroke
            stroke-width="10"
            stroke-linecap="round"
            fill="transparent"
            style=move || {
                let playing = is_playing.get_untracked();
                if playing {
                    let w = width.parse::<f64>().unwrap_or(140.0);
                    let h = height.parse::<f64>().unwrap_or(140.0);
                    let path_length = 2.0 * (w + h);
                    let progress = ((js_sys::Date::now() / 1000.0 - delay).max(0.0) * 0.5).min(1.0);
                    let spring_progress = 1.0 - (1.0 - progress).powi(3);
                    let offset = path_length * (1.0 - spring_progress);
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 1;", path_length, offset)
                } else {
                    let w = width.parse::<f64>().unwrap_or(140.0);
                    let h = height.parse::<f64>().unwrap_or(140.0);
                    let path_length = 2.0 * (w + h);
                    format!("stroke-dasharray: {}; stroke-dashoffset: {}; opacity: 0;", path_length, path_length)
                }
            }
        />
    }
}