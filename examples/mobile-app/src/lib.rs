//! Mobile app example with smooth transitions and responsive design
//!
//! This example demonstrates mobile-friendly animations and interactions

use leptos::*;
use leptos::prelude::{ElementChild, StyleAttribute, IntoAny, OnAttribute, Get, Set, Update, signal, ReadSignal};

/// Mobile app component
#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(0);
    let (menu_open, set_menu_open) = signal(false);

    let pages = vec!["Home", "Profile", "Messages", "Settings"];

    view! {
        <div style="min-height: 100vh; background: linear-gradient(to bottom, #1e3a8a, #3730a3); color: white; font-family: system-ui; position: relative; overflow-x: hidden;">

            // Header
            <header style="padding: 1rem; background: rgba(0,0,0,0.2); backdrop-filter: blur(10px); display: flex; align-items: center; justify-content: space-between;">
                <h1 style="margin: 0; font-size: 1.5rem; font-weight: bold;">"Mobile App"</h1>
                <button
                    style=move || format!(
                        "background: none; border: none; color: white; font-size: 1.5rem; cursor: pointer; padding: 0.5rem; border-radius: 0.5rem; transition: all 0.3s ease; transform: rotate({}deg); background: {};",
                        if menu_open.get() { 90 } else { 0 },
                        if menu_open.get() { "rgba(255,255,255,0.1)" } else { "transparent" }
                    )
                    on:click=move |_| set_menu_open.update(|open| *open = !*open)
                >
                    "☰"
                </button>
            </header>

            // Slide-out Menu
            <div
                style=move || format!(
                    "position: fixed; top: 0; right: 0; height: 100vh; width: 280px; background: rgba(0,0,0,0.9); backdrop-filter: blur(20px); transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1); transform: translateX({}px); z-index: 50; padding: 1rem;",
                    if menu_open.get() { 0 } else { 280 }
                )
            >
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
                    <h2 style="margin: 0; color: white;">"Menu"</h2>
                    <button
                        style="background: none; border: none; color: white; font-size: 1.5rem; cursor: pointer; padding: 0.5rem;"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "×"
                    </button>
                </div>

                <nav style="display: flex; flex-direction: column; gap: 0.5rem;">
                    {pages.iter().enumerate().map(|(index, page)| {
                        let is_current = move || current_page.get() == index;
                        view! {
                            <button
                                style=move || format!(
                                    "text-align: left; padding: 1rem; background: {}; border: none; color: white; border-radius: 0.5rem; cursor: pointer; transition: all 0.3s ease; transform: scale({}); font-size: 1rem;",
                                    if is_current() { "rgba(255,255,255,0.2)" } else { "transparent" },
                                    if is_current() { 1.02 } else { 1.0 }
                                )
                                on:click=move |_| {
                                    set_current_page.set(index);
                                    set_menu_open.set(false);
                                }
                            >
                                {*page}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </nav>
            </div>

            // Overlay
            {move || if menu_open.get() {
                view! {
                    <div
                        style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 40; transition: opacity 0.3s ease;"
                        on:click=move |_| set_menu_open.set(false)
                    >
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            // Main Content
            <main style="padding: 2rem 1rem; min-height: calc(100vh - 140px);">
                {move || match current_page.get() {
                    0 => view! { <HomePage /> }.into_any(),
                    1 => view! { <ProfilePage /> }.into_any(),
                    2 => view! { <MessagesPage /> }.into_any(),
                    3 => view! { <SettingsPage /> }.into_any(),
                    _ => view! { <div>"Unknown page"</div> }.into_any(),
                }}
            </main>

            // Bottom Navigation
            <nav style="position: fixed; bottom: 0; left: 0; right: 0; background: rgba(0,0,0,0.3); backdrop-filter: blur(10px); padding: 1rem; display: flex; justify-content: space-around;">
                {pages.iter().enumerate().map(|(index, page)| {
                    let is_current = move || current_page.get() == index;
                    let icon = match index {
                        0 => "🏠",
                        1 => "👤",
                        2 => "💬",
                        3 => "⚙️",
                        _ => "📱"
                    };

                    view! {
                        <button
                            style=move || format!(
                                "display: flex; flex-direction: column; align-items: center; gap: 0.25rem; background: none; border: none; color: {}; cursor: pointer; padding: 0.5rem; border-radius: 0.5rem; transition: all 0.3s ease; transform: scale({});",
                                if is_current() { "#60a5fa" } else { "rgba(255,255,255,0.7)" },
                                if is_current() { 1.1 } else { 1.0 }
                            )
                            on:click=move |_| set_current_page.set(index)
                        >
                            <span style="font-size: 1.25rem;">{icon}</span>
                            <span style="font-size: 0.75rem;">{*page}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </nav>

        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (counter, set_counter) = signal(0);

    view! {
        <div style="text-align: center;">
            <h2 style="margin: 0 0 2rem 0; font-size: 2rem;">"Welcome Home"</h2>

            <div style="background: rgba(255,255,255,0.1); border-radius: 1rem; padding: 2rem; margin-bottom: 2rem; backdrop-filter: blur(10px);">
                <h3 style="margin: 0 0 1rem 0;">"Quick Stats"</h3>
                <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;">
                    <StatCard title="Counter".to_string() value=move || counter.get().to_string() />
                    <StatCard title="Status".to_string() value=move || "Active".to_string() />
                </div>
            </div>

            <button
                style="background: #3b82f6; color: white; border: none; padding: 1rem 2rem; border-radius: 2rem; cursor: pointer; font-size: 1.1rem; font-weight: bold; transition: all 0.3s ease; transform: scale(1); box-shadow: 0 4px 15px rgba(59, 130, 246, 0.4);"
                on:click=move |_| set_counter.update(|c| *c += 1)
            >
                "Tap to Count"
            </button>
        </div>
    }
}

#[component]
fn ProfilePage() -> impl IntoView {
    view! {
        <div>
            <h2 style="margin: 0 0 2rem 0; text-align: center;">"Profile"</h2>

            <div style="text-align: center; margin-bottom: 2rem;">
                <div style="width: 120px; height: 120px; background: linear-gradient(45deg, #f59e0b, #ef4444); border-radius: 50%; margin: 0 auto 1rem auto; display: flex; align-items: center; justify-content: center; font-size: 3rem; color: white;">
                    "👤"
                </div>
                <h3 style="margin: 0 0 0.5rem 0;">"John Doe"</h3>
                <p style="margin: 0; opacity: 0.8;">"Mobile App User"</p>
            </div>

            <div style="background: rgba(255,255,255,0.1); border-radius: 1rem; padding: 1.5rem; backdrop-filter: blur(10px);">
                <ProfileStat label="Joined".to_string() value="January 2024".to_string() />
                <ProfileStat label="Posts".to_string() value="42".to_string() />
                <ProfileStat label="Following".to_string() value="128".to_string() />
                <ProfileStat label="Followers".to_string() value="256".to_string() />
            </div>
        </div>
    }
}

#[component]
fn MessagesPage() -> impl IntoView {
    let messages = vec![
        ("Alice", "Hey! How are you doing?", "2 min ago"),
        ("Bob", "Meeting at 3pm today", "1 hour ago"),
        ("Team", "New project update", "3 hours ago"),
    ];

    view! {
        <div>
            <h2 style="margin: 0 0 2rem 0; text-align: center;">"Messages"</h2>

            <div style="display: flex; flex-direction: column; gap: 1rem;">
                {messages.into_iter().map(|(sender, message, time)| {
                    view! {
                        <MessageCard
                            sender=sender.to_string()
                            message=message.to_string()
                            time=time.to_string()
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    let (notifications, set_notifications) = signal(true);
    let (location, set_location) = signal(false);

    view! {
        <div>
            <h2 style="margin: 0 0 2rem 0; text-align: center;">"Settings"</h2>

            <div style="display: flex; flex-direction: column; gap: 1rem;">
                <ToggleSetting
                    label="Push Notifications".to_string()
                    checked=notifications
                    on_change=move |value| set_notifications.set(value)
                />
                <ToggleSetting
                    label="Location Services".to_string()
                    checked=location
                    on_change=move |value| set_location.set(value)
                />
            </div>

            <div style="margin-top: 2rem; padding: 1.5rem; background: rgba(239, 68, 68, 0.1); border-radius: 1rem; border: 1px solid rgba(239, 68, 68, 0.3);">
                <h3 style="margin: 0 0 0.5rem 0; color: #fca5a5;">"Danger Zone"</h3>
                <p style="margin: 0 0 1rem 0; opacity: 0.8; font-size: 0.9rem;">"These actions cannot be undone."</p>
                <button style="background: #ef4444; color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 0.5rem; cursor: pointer; font-weight: 500;">
                    "Delete Account"
                </button>
            </div>
        </div>
    }
}

#[component]
fn StatCard(title: String, value: impl Fn() -> String + 'static + Send) -> impl IntoView {
    view! {
        <div style="background: rgba(255,255,255,0.05); border-radius: 0.5rem; padding: 1rem;">
            <div style="font-size: 1.5rem; font-weight: bold; margin-bottom: 0.25rem;">
                {move || value()}
            </div>
            <div style="opacity: 0.8; font-size: 0.875rem;">
                {title}
            </div>
        </div>
    }
}

#[component]
fn ProfileStat(label: String, value: String) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 0; border-bottom: 1px solid rgba(255,255,255,0.1);">
            <span style="opacity: 0.8;">{label}</span>
            <span style="font-weight: 500;">{value}</span>
        </div>
    }
}

#[component]
fn MessageCard(sender: String, message: String, time: String) -> impl IntoView {
    let (hovered, set_hovered) = signal(false);

    view! {
        <div
            style=move || format!(
                "background: rgba(255,255,255,0.1); border-radius: 1rem; padding: 1rem; backdrop-filter: blur(10px); cursor: pointer; transition: all 0.3s ease; transform: scale({}); border: 1px solid rgba(255,255,255,{});",
                if hovered.get() { 1.02 } else { 1.0 },
                if hovered.get() { 0.2 } else { 0.1 }
            )
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                <h4 style="margin: 0; font-weight: 600;">{sender}</h4>
                <span style="opacity: 0.6; font-size: 0.875rem;">{time}</span>
            </div>
            <p style="margin: 0; opacity: 0.9;">{message}</p>
        </div>
    }
}

#[component]
fn ToggleSetting<F>(label: String, checked: ReadSignal<bool>, on_change: F) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    view! {
        <div style="display: flex; justify-content: space-between; align-items: center; background: rgba(255,255,255,0.1); border-radius: 1rem; padding: 1rem; backdrop-filter: blur(10px);">
            <span style="font-weight: 500;">{label}</span>
            <button
                style=move || format!(
                    "width: 3rem; height: 1.5rem; border-radius: 0.75rem; border: none; cursor: pointer; transition: all 0.3s ease; background: {}; position: relative;",
                    if checked.get() { "#10b981" } else { "rgba(255,255,255,0.3)" }
                )
                on:click=move |_| on_change(!checked.get())
            >
                <div
                    style=move || format!(
                        "width: 1.25rem; height: 1.25rem; background: white; border-radius: 50%; transition: all 0.3s ease; transform: translateX({}rem); position: absolute; top: 0.125rem; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        if checked.get() { 1.5 } else { 0.125 }
                    )
                >
                </div>
            </button>
        </div>
    }
}
