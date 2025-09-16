//! Scroll Animations - Scroll-linked animations and parallax effects

use leptos::prelude::*;
use leptos_motion::*;

#[component]
pub fn ScrollAnimations() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="text-center">
                <h2 class="text-3xl font-bold mb-4">"Scroll Animations"</h2>
                <p class="text-gray-300 mb-6">
                    "Scroll-linked animations and parallax effects"
                </p>
            </div>

            <div class="h-96 bg-gradient-to-br from-orange-500 to-red-600 rounded-2xl flex items-center justify-center">
                <p class="text-white text-xl">"Scroll animations coming soon..."</p>
            </div>
        </div>
    }
}
