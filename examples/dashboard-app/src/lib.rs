use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    let (current_view, set_current_view) = signal("overview".to_string());
    let (is_loading, set_loading) = signal(false);

    view! {
        <div class="dashboard-container">
            <header class="dashboard-header">
                <h1>"Analytics Dashboard"</h1>
                <nav class="dashboard-nav">
                    <button
                        class:active=move || current_view.get() == "overview"
                        on:click=move |_| set_current_view.set("overview".to_string())
                    >
                        "Overview"
                    </button>
                    <button
                        class:active=move || current_view.get() == "analytics"
                        on:click=move |_| set_current_view.set("analytics".to_string())
                    >
                        "Analytics"
                    </button>
                    <button
                        class:active=move || current_view.get() == "reports"
                        on:click=move |_| set_current_view.set("reports".to_string())
                    >
                        "Reports"
                    </button>
                </nav>
            </header>

            <main class="dashboard-main">
                <AnimatePresence mode=Some(PresenceMode::Wait)>
                    {move || {
                        match current_view.get().as_str() {
                            "overview" => view! { <OverviewView /> },
                            "analytics" => view! { <AnalyticsView /> },
                            "reports" => view! { <ReportsView /> },
                            _ => view! { <OverviewView /> },
                        }
                    }}
                </AnimatePresence>
            </main>
        </div>
    }
}

#[component]
fn OverviewView() -> impl IntoView {
    let (data, set_data) = signal(Vec::new());
    let (is_loading, set_loading) = signal(true);

    // Simulate data loading
    Effect::new(move |_| {
        set_loading.set(true);
        set_timeout_with_handle(
            move || {
                let mock_data = vec![
                    ("Sales", 12500.0, "#667eea"),
                    ("Revenue", 8900.0, "#764ba2"),
                    ("Users", 2340.0, "#f093fb"),
                    ("Orders", 567.0, "#f5576c"),
                ];
                set_data.set(mock_data);
                set_loading.set(false);
            },
            std::time::Duration::from_millis(1500)
        ).expect("Failed to set timeout");
    });

    view! {
        <div class="overview-view">
            <h2>"Overview"</h2>
            
            <div class="metrics-grid">
                {move || {
                    if is_loading.get() {
                        vec![
                            view! { <LoadingCard /> },
                            view! { <LoadingCard /> },
                            view! { <LoadingCard /> },
                            view! { <LoadingCard /> },
                        ]
                    } else {
                        data.get().into_iter().enumerate().map(|(i, (title, value, color))| {
                            view! {
                                <MotionDiv
                                    class="metric-card"
                                    key=title
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
                                    <div class="metric-icon" style=format!("background-color: {}", color)>
                                        "📊"
                                    </div>
                                    <div class="metric-content">
                                        <h3>{title}</h3>
                                        <p class="metric-value">{format!("{:.0}", value)}</p>
                                    </div>
                                </MotionDiv>
                            }
                        }).collect::<Vec<_>>()
                    }
                }}
            </div>

            <div class="chart-section">
                <h3>"Recent Activity"</h3>
                <ChartComponent />
            </div>
        </div>
    }
}

#[component]
fn LoadingCard() -> impl IntoView {
    view! {
        <MotionDiv
            class="metric-card loading"
            animate=motion_target!(
                "opacity" => AnimationValue::Number(0.6)
            )
            transition=Transition {
                duration: Some(1.0),
                ease: Easing::EaseInOut,
                repeat: Some(RepeatConfig {
                    count: None,
                    reverse: true,
                    delay: Some(0.0),
                }),
                ..Default::default()
            }
        >
            <div class="loading-placeholder">
                <div class="loading-bar"></div>
                <div class="loading-bar short"></div>
            </div>
        </MotionDiv>
    }
}

#[component]
fn ChartComponent() -> impl IntoView {
    let (chart_data, set_chart_data) = signal(vec![
        ("Mon", 45.0),
        ("Tue", 52.0),
        ("Wed", 38.0),
        ("Thu", 67.0),
        ("Fri", 73.0),
        ("Sat", 89.0),
        ("Sun", 61.0),
    ]);

    view! {
        <div class="chart-container">
            <div class="chart-bars">
                {chart_data.get().into_iter().enumerate().map(|(i, (day, value))| {
                    let height = (value / 100.0) * 200.0;
                    view! {
                        <MotionDiv
                            class="chart-bar"
                            key=day
                            initial=Some(motion_target!(
                                "height" => AnimationValue::Pixels(0.0)
                            ))
                            animate=motion_target!(
                                "height" => AnimationValue::Pixels(height)
                            )
                            transition=Transition {
                                duration: Some(0.8),
                                delay: Some(i as f64 * 0.1),
                                ease: Easing::BackOut,
                                ..Default::default()
                            }
                        >
                            <div class="bar-label">{day}</div>
                            <div class="bar-value">{format!("{:.0}", value)}</div>
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn AnalyticsView() -> impl IntoView {
    view! {
        <div class="analytics-view">
            <h2>"Analytics"</h2>
            <p>"Detailed analytics and insights coming soon..."</p>
            
            <MotionDiv
                class="coming-soon"
                animate=motion_target!(
                    "scale" => AnimationValue::Number(1.0)
                )
                initial=Some(motion_target!(
                    "scale" => AnimationValue::Number(0.8),
                    "opacity" => AnimationValue::Number(0.0)
                ))
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::BackOut,
                    ..Default::default()
                }
            >
                "🚀 Coming Soon!"
            </MotionDiv>
        </div>
    }
}

#[component]
fn ReportsView() -> impl IntoView {
    view! {
        <div class="reports-view">
            <h2>"Reports"</h2>
            <p>"Generate and view detailed reports..."</p>
            
            <MotionDiv
                class="reports-grid"
                layout=Some(true)
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                <div class="report-card">
                    <h3>"Monthly Report"</h3>
                    <p>"Comprehensive monthly analysis"</p>
                    <button class="download-btn">"Download"</button>
                </div>
                
                <div class="report-card">
                    <h3>"Quarterly Report"</h3>
                    <p>"Quarterly performance summary"</p>
                    <button class="download-btn">"Download"</button>
                </div>
                
                <div class="report-card">
                    <h3>"Annual Report"</h3>
                    <p>"Year-end comprehensive review"</p>
                    <button class="download-btn">"Download"</button>
                </div>
            </MotionDiv>
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
