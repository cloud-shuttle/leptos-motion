//! AnimatePresence component for exit animations

use leptos::prelude::*;

/// AnimatePresence component for coordinating exit animations
#[component]
pub fn AnimatePresence(
    /// Presence mode controlling how exit animations are coordinated
    #[prop(optional)] mode: Option<PresenceMode>,
    /// Child components to manage presence for
    children: Children,
) -> impl IntoView {
    let _mode = mode.unwrap_or(PresenceMode::Sync);

    // Simplified implementation for now
    view! {
        <div class="animate-presence">
            {children()}
        </div>
    }
}

/// Presence animation mode
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceMode {
    /// All animations happen simultaneously
    Sync,
    /// Wait for exit animations before entering new elements
    Wait,
    /// Preserve layout during animations
    PopLayout,
}
