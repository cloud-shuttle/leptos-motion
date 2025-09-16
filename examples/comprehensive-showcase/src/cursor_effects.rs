//! Cursor Effects - Dynamic cursor interactions

use leptos::prelude::*;
use leptos_motion::*;

#[component]
pub fn CursorEffects() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Cursor Effects"</h2>
                <p class="text-gray-300 mb-6">
                    "Dynamic cursor interactions and magnetic effects"
                </p>
            </div>

            <div class="h-96 bg-gradient-to-br from-teal-500 to-blue-600 rounded-2xl flex items-center justify-center">
                <p class="text-white text-xl">"Cursor effects coming soon..."</p>
            </div>
        </div>
    }
}
