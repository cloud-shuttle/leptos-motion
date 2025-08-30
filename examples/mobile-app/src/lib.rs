use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal("home".to_string());
    let (is_refreshing, set_refreshing) = signal(false);

    view! {
        <div class="mobile-app">
            <header class="app-header">
                <h1>"Mobile App"</h1>
                <div class="header-actions">
                    <button class="menu-btn">"☰"</button>
                </div>
            </header>

            <main class="app-content">
                <AnimatePresence mode=Some(PresenceMode::Wait)>
                    {move || {
                        match current_page.get().as_str() {
                            "home" => view! { <HomePage /> },
                            "profile" => view! { <ProfilePage /> },
                            "settings" => view! { <SettingsPage /> },
                            _ => view! { <HomePage /> },
                        }
                    }}
                </AnimatePresence>
            </main>

            <nav class="bottom-nav">
                <button
                    class:active=move || current_page.get() == "home"
                    on:click=move |_| set_current_page.set("home".to_string())
                >
                    "🏠"
                </button>
                <button
                    class:active=move || current_page.get() == "profile"
                    on:click=move |_| set_current_page.set("profile".to_string())
                >
                    "👤"
                </button>
                <button
                    class:active=move || current_page.get() == "settings"
                    on:click=move |_| set_current_page.set("settings".to_string())
                >
                    "⚙️"
                </button>
            </nav>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (is_refreshing, set_refreshing) = signal(false);
    let (posts, set_posts) = signal(vec![
        "First post content here...",
        "Second post with some text...",
        "Another interesting post...",
        "Fourth post in the feed...",
    ]);

    let refresh = move |_| {
        set_refreshing.set(true);
        set_timeout_with_handle(
            move || {
                set_posts.update(|posts| {
                    posts.push("New refreshed post!".to_string());
                });
                set_refreshing.set(false);
            },
            std::time::Duration::from_millis(2000)
        ).expect("Failed to set timeout");
    };

    view! {
        <div class="page home-page">
            <div class="pull-to-refresh" class:refreshing=is_refreshing>
                <MotionDiv
                    class="refresh-indicator"
                    animate=motion_target!(
                        "rotate" => AnimationValue::Degrees(if is_refreshing.get() { 360.0 } else { 0.0 })
                    )
                    transition=Transition {
                        duration: Some(1.0),
                        ease: Easing::Linear,
                        repeat: Some(RepeatConfig {
                            count: None,
                            reverse: false,
                            delay: Some(0.0),
                        }),
                        ..Default::default()
                    }
                >
                    "🔄"
                </MotionDiv>
                <p>"Pull to refresh"</p>
            </div>

            <div class="feed">
                <h2>"Feed"</h2>
                {posts.get().into_iter().enumerate().map(|(i, post)| {
                    view! {
                        <MotionDiv
                            class="post-card"
                            key=i
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
                            <div class="post-header">
                                <div class="avatar">"👤"</div>
                                <div class="post-info">
                                    <h3>"User {i + 1}"</h3>
                                    <span class="timestamp">"2 hours ago"</span>
                                </div>
                            </div>
                            <p class="post-content">{post}</p>
                            <div class="post-actions">
                                <button class="action-btn">"❤️"</button>
                                <button class="action-btn">"💬"</button>
                                <button class="action-btn">"📤"</button>
                            </div>
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn ProfilePage() -> impl IntoView {
    let (is_editing, set_editing) = signal(false);
    let (profile_data, set_profile_data) = signal(("John Doe", "john@example.com", "Software Developer"));

    view! {
        <div class="page profile-page">
            <div class="profile-header">
                <MotionDiv
                    class="profile-avatar"
                    while_hover=Some(motion_target!(
                        "scale" => AnimationValue::Number(1.1)
                    ))
                    on:click=move |_| set_editing.set(!is_editing.get())
                >
                    "👤"
                </MotionDiv>
                <h2>{move || profile_data.get().0}</h2>
                <p class="bio">{move || profile_data.get().2}</p>
            </div>

            <div class="profile-stats">
                <div class="stat">
                    <span class="stat-number">"1,234"</span>
                    <span class="stat-label">"Posts"</span>
                </div>
                <div class="stat">
                    <span class="stat-number">"567"</span>
                    <span class="stat-label">"Followers"</span>
                </div>
                <div class="stat">
                    <span class="stat-number">"89"</span>
                    <span class="stat-label">"Following"</span>
                </div>
            </div>

            <div class="profile-actions">
                <MotionDiv
                    class="action-button"
                    while_tap=Some(motion_target!(
                        "scale" => AnimationValue::Number(0.95)
                    ))
                >
                    "Edit Profile"
                </MotionDiv>
                <MotionDiv
                    class="action-button secondary"
                    while_tap=Some(motion_target!(
                        "scale" => AnimationValue::Number(0.95)
                    ))
                >
                    "Share Profile"
                </MotionDiv>
            </div>
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    let (notifications, set_notifications) = signal(true);
    let (dark_mode, set_dark_mode) = signal(false);
    let (auto_save, set_auto_save) = signal(true);

    view! {
        <div class="page settings-page">
            <h2>"Settings"</h2>
            
            <div class="settings-section">
                <h3>"Preferences"</h3>
                
                <div class="setting-item">
                    <div class="setting-info">
                        <span class="setting-label">"Notifications"</span>
                        <span class="setting-description">"Receive push notifications"</span>
                    </div>
                    <MotionDiv
                        class="toggle"
                        class:active=notifications
                        on:click=move |_| set_notifications.set(!notifications.get())
                        while_tap=Some(motion_target!(
                            "scale" => AnimationValue::Number(0.9)
                        ))
                    >
                        <div class="toggle-slider"></div>
                    </MotionDiv>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <span class="setting-label">"Dark Mode"</span>
                        <span class="setting-description">"Use dark theme"</span>
                    </div>
                    <MotionDiv
                        class="toggle"
                        class:active=dark_mode
                        on:click=move |_| set_dark_mode.set(!dark_mode.get())
                        while_tap=Some(motion_target!(
                            "scale" => AnimationValue::Number(0.9)
                        ))
                    >
                        <div class="toggle-slider"></div>
                    </MotionDiv>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <span class="setting-label">"Auto Save"</span>
                        <span class="setting-description">"Automatically save changes"</span>
                    </div>
                    <MotionDiv
                        class="toggle"
                        class:active=auto_save
                        on:click=move |_| set_auto_save.set(!auto_save.get())
                        while_tap=Some(motion_target!(
                            "scale" => AnimationValue::Number(0.9)
                        ))
                    >
                        <div class="toggle-slider"></div>
                    </MotionDiv>
                </div>
            </div>

            <div class="settings-section">
                <h3>"Account"</h3>
                <button class="settings-btn">"Change Password"</button>
                <button class="settings-btn">"Privacy Settings"</button>
                <button class="settings-btn danger">"Delete Account"</button>
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
