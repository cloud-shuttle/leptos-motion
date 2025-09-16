//! Card Stack - Stacked card interactions

use leptos::prelude::*;
use leptos_motion::*;

#[component]
pub fn CardStack() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Card Stack"</h2>
                <p class="text-gray-300 mb-6">
                    "Stacked card interactions with smooth animations"
                </p>
            </div>

            <div class="h-96 bg-gradient-to-br from-emerald-500 to-teal-600 rounded-2xl flex items-center justify-center">
                <p class="text-white text-xl">"Card stack coming soon..."</p>
            </div>
        </div>
    }
}
