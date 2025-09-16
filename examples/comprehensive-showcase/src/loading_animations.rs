//! Loading Animations - Various loading indicators

use leptos::prelude::*;
use leptos_motion::*;

#[component]
pub fn LoadingAnimations() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Loading Animations"</h2>
                <p class="text-gray-300 mb-6">
                    "Various loading indicators and progress animations"
                </p>
            </div>

            <div class="h-96 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center">
                <p class="text-white text-xl">"Loading animations coming soon..."</p>
            </div>
        </div>
    }
}
