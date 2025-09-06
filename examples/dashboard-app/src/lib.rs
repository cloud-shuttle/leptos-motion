//! Dashboard app example with smooth transitions and animations
//!
//! This example demonstrates a dashboard interface with working animations

use leptos::*;
use leptos_motion_core::*;

/// Dashboard app component
#[component]
pub fn App() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0);
    let (loading, set_loading) = signal(false);

    let tabs = vec!["Overview", "Analytics", "Reports", "Settings"];

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
            <div style="container: mx-auto; padding: 2rem;">

                // Header
                <header style="background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); border-radius: 1rem; padding: 1.5rem; margin-bottom: 2rem; border: 1px solid rgba(255,255,255,0.2);">
                    <h1 style="color: white; font-size: 2rem; font-weight: bold; margin: 0;">"Dashboard Analytics"</h1>
                    <p style="color: rgba(255,255,255,0.8); margin: 0.5rem 0 0 0;">"Real-time data visualization and insights"</p>
                </header>

                // Tab Navigation
                <nav style="display: flex; gap: 0.5rem; margin-bottom: 2rem;">
                    {tabs.into_iter().enumerate().map(|(index, tab)| {
                        let is_active = move || active_tab.get() == index;
                        view! {
                            <button
                                style=move || format!(
                                    "padding: 0.75rem 1.5rem; border-radius: 0.5rem; border: none; cursor: pointer; font-weight: 500; transition: all 0.3s ease; background: {}; color: {}; transform: scale({});",
                                    if is_active() { "rgba(255,255,255,0.9)" } else { "rgba(255,255,255,0.1)" },
                                    if is_active() { "#374151" } else { "white" },
                                    if is_active() { 1.05 } else { 1.0 }
                                )
                                on:click=move |_| set_active_tab.set(index)
                            >
                                {tab}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </nav>

                // Content Area
                <main style="background: rgba(255,255,255,0.95); border-radius: 1rem; padding: 2rem; min-height: 400px;">
                    {move || match active_tab.get() {
                        0 => view! { <OverviewContent /> }.into_any(),
                        1 => view! { <AnalyticsContent /> }.into_any(),
                        2 => view! { <ReportsContent loading=loading set_loading=set_loading /> }.into_any(),
                        3 => view! { <SettingsContent /> }.into_any(),
                        _ => view! { <div>"Unknown tab"</div> }.into_any(),
                    }}
                </main>

            </div>
        </div>
    }
}

#[component]
fn OverviewContent() -> impl IntoView {
    view! {
        <div>
            <h2 style="color: #1f2937; margin-bottom: 1.5rem;">"Overview"</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;">
                <MetricCard title="Total Users".to_string() value="12,345".to_string() color="#3b82f6".to_string() />
                <MetricCard title="Revenue".to_string() value="$45,678".to_string() color="#10b981".to_string() />
                <MetricCard title="Growth".to_string() value="+23.5%".to_string() color="#f59e0b".to_string() />
                <MetricCard title="Conversion".to_string() value="4.2%".to_string() color="#ef4444".to_string() />
            </div>
        </div>
    }
}

#[component]
fn AnalyticsContent() -> impl IntoView {
    view! {
        <div>
            <h2 style="color: #1f2937; margin-bottom: 1.5rem;">"Analytics"</h2>
            <div style="background: #f8fafc; padding: 2rem; border-radius: 0.5rem; border: 1px solid #e2e8f0;">
                <p style="color: #64748b; text-align: center; font-size: 1.1rem;">
                    "Analytics dashboard with interactive charts and real-time data visualization."
                </p>
                <div style="margin-top: 2rem; display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;">
                    <div style="background: white; padding: 1.5rem; border-radius: 0.5rem; border: 1px solid #e2e8f0;">
                        <h4 style="margin: 0 0 1rem 0; color: #374151;">"Page Views"</h4>
                        <div style="height: 100px; background: linear-gradient(to right, #3b82f6, #1d4ed8); border-radius: 0.25rem; display: flex; align-items: end; justify-content: center; color: white; font-weight: bold;">
                            "📈 Trending Up"
                        </div>
                    </div>
                    <div style="background: white; padding: 1.5rem; border-radius: 0.5rem; border: 1px solid #e2e8f0;">
                        <h4 style="margin: 0 0 1rem 0; color: #374151;">"User Engagement"</h4>
                        <div style="height: 100px; background: linear-gradient(to right, #10b981, #047857); border-radius: 0.25rem; display: flex; align-items: end; justify-content: center; color: white; font-weight: bold;">
                            "💪 Strong"
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ReportsContent(loading: ReadSignal<bool>, set_loading: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div>
            <h2 style="color: #1f2937; margin-bottom: 1.5rem;">"Reports"</h2>
            <div style="display: flex; gap: 1rem; margin-bottom: 2rem;">
                <button
                    style=move || format!(
                        "padding: 0.75rem 1.5rem; background: {}; color: white; border: none; border-radius: 0.5rem; cursor: pointer; transition: all 0.3s ease; opacity: {};",
                        if loading.get() { "#6b7280" } else { "#3b82f6" },
                        if loading.get() { 0.6 } else { 1.0 }
                    )
                    disabled=loading
                    on:click=move |_| {
                        set_loading.set(true);
                        set_timeout(move || set_loading.set(false), std::time::Duration::from_secs(2));
                    }
                >
                    {move || if loading.get() { "Generating..." } else { "Generate Report" }}
                </button>
            </div>

            {move || if loading.get() {
                view! {
                    <div style="background: #fef3c7; padding: 2rem; border-radius: 0.5rem; border: 1px solid #f59e0b; text-align: center;">
                        <div style="display: inline-block; width: 20px; height: 20px; border: 2px solid #f59e0b; border-radius: 50%; border-top-color: transparent; animation: spin 1s linear infinite; margin-right: 0.5rem;"></div>
                        "Generating comprehensive report..."
                    </div>
                }.into_any()
            } else {
                view! {
                    <div style="background: #f0fdf4; padding: 2rem; border-radius: 0.5rem; border: 1px solid #10b981;">
                        <h3 style="color: #065f46; margin: 0 0 1rem 0;">"📊 Monthly Report"</h3>
                        <p style="color: #047857; margin: 0;">"Report ready for download. All metrics showing positive growth trends."</p>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

#[component]
fn SettingsContent() -> impl IntoView {
    let (notifications, set_notifications) = signal(true);
    let (dark_mode, set_dark_mode) = signal(false);

    view! {
        <div>
            <h2 style="color: #1f2937; margin-bottom: 1.5rem;">"Settings"</h2>
            <div style="space-y: 1rem;">
                <SettingToggle
                    label="Enable Notifications".to_string()
                    checked=notifications
                    on_change=move |value| set_notifications.set(value)
                />
                <SettingToggle
                    label="Dark Mode".to_string()
                    checked=dark_mode
                    on_change=move |value| set_dark_mode.set(value)
                />
            </div>
        </div>
    }
}

#[component]
fn MetricCard(title: String, value: String, color: String) -> impl IntoView {
    let (hovered, set_hovered) = signal(false);

    view! {
        <div
            style=move || format!(
                "background: white; padding: 1.5rem; border-radius: 0.5rem; border: 1px solid #e5e7eb; cursor: pointer; transition: all 0.3s ease; transform: translateY({}px) scale({}); box-shadow: 0 {}px {}px rgba(0,0,0,{});",
                if hovered.get() { -2 } else { 0 },
                if hovered.get() { 1.02 } else { 1.0 },
                if hovered.get() { 8 } else { 1 },
                if hovered.get() { 25 } else { 3 },
                if hovered.get() { 0.1 } else { 0.05 }
            )
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <h3 style="color: #6b7280; font-size: 0.875rem; font-weight: 500; margin: 0 0 0.5rem 0; text-transform: uppercase; letter-spacing: 0.05em;">
                {title}
            </h3>
            <p style=format!("color: {}; font-size: 2rem; font-weight: bold; margin: 0;", color)>
                {value}
            </p>
        </div>
    }
}

#[component]
fn SettingToggle<F>(label: String, checked: ReadSignal<bool>, on_change: F) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    view! {
        <div style="display: flex; align-items: center; justify-content: space-between; padding: 1rem; background: #f8fafc; border-radius: 0.5rem; border: 1px solid #e2e8f0;">
            <span style="color: #374151; font-weight: 500;">{label}</span>
            <button
                style=move || format!(
                    "width: 3rem; height: 1.5rem; border-radius: 0.75rem; border: none; cursor: pointer; transition: all 0.3s ease; background: {}; position: relative;",
                    if checked.get() { "#10b981" } else { "#d1d5db" }
                )
                on:click=move |_| on_change(!checked.get())
            >
                <div
                    style=move || format!(
                        "width: 1.25rem; height: 1.25rem; background: white; border-radius: 50%; transition: all 0.3s ease; transform: translateX({}rem); position: absolute; top: 0.125rem;",
                        if checked.get() { 1.5 } else { 0.125 }
                    )
                >
                </div>
            </button>
        </div>
    }
}
