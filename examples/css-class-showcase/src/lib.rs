use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (animation_type, set_animation_type) = signal("fadeIn".to_string());
    let (is_animating, set_is_animating) = signal(false);

    let start_animation = move |_| {
        set_is_animating.set(true);
        // Simulate animation completion
        set_timeout(
            move || {
                set_is_animating.set(false);
            },
            std::time::Duration::from_millis(1000),
        );
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold text-center mb-8 text-gray-800">
                    "Leptos Motion CSS Class Showcase"
                </h1>
                
                <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
                    <h2 class="text-2xl font-semibold mb-4 text-gray-700">
                        "CSS Class-Based Animations"
                    </h2>
                    
                    <div class="mb-6">
                        <label class="block text-sm font-medium text-gray-600 mb-2">
                            "Select Animation Type:"
                        </label>
                        <select 
                            class="w-full p-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_animation_type.set(value);
                            }
                        >
                            <option value="fadeIn">"Fade In"</option>
                            <option value="slideIn">"Slide In"</option>
                            <option value="bounceIn">"Bounce In"</option>
                            <option value="scaleIn">"Scale In"</option>
                            <option value="rotateIn">"Rotate In"</option>
                        </select>
                    </div>

                    <button
                        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-md transition-colors duration-200"
                        on:click=start_animation
                        disabled=is_animating
                    >
                        {move || if is_animating.get() { "Animating..." } else { "Start Animation" }}
                    </button>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <AnimationCard 
                        title="Fade In"
                        description="Smooth opacity transition"
                        animation_class="fade-in"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="fadeIn"
                    />
                    
                    <AnimationCard 
                        title="Slide In"
                        description="Slide from left with bounce"
                        animation_class="slide-in"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="slideIn"
                    />
                    
                    <AnimationCard 
                        title="Bounce In"
                        description="Bouncy scale animation"
                        animation_class="bounce-in"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="bounceIn"
                    />
                    
                    <AnimationCard 
                        title="Scale In"
                        description="Scale from center"
                        animation_class="scale-in"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="scaleIn"
                    />
                    
                    <AnimationCard 
                        title="Rotate In"
                        description="Rotate with scale"
                        animation_class="rotate-in"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="rotateIn"
                    />
                    
                    <AnimationCard 
                        title="Custom"
                        description="Tailwind CSS classes"
                        animation_class="animate-pulse"
                        animation_type=animation_type
                        is_animating=is_animating
                        target_type="custom"
                    />
                </div>

                <div class="mt-8 bg-white rounded-lg shadow-lg p-6">
                    <h3 class="text-xl font-semibold mb-4 text-gray-700">
                        "How It Works"
                    </h3>
                    <div class="prose max-w-none text-gray-600">
                        <p class="mb-4">
                            "This showcase demonstrates the new CSS class-based animation system in Leptos Motion. 
                            Instead of complex JavaScript animations, we use CSS classes for better performance and simplicity."
                        </p>
                        <ul class="list-disc list-inside space-y-2">
                            <li>"CSS classes are dynamically injected into the document"</li>
                            <li>"Animations are triggered by applying classes to elements"</li>
                            <li>"Tailwind CSS integration for consistent styling"</li>
                            <li>"Better performance than JavaScript-based animations"</li>
                            <li>"Easier to customize and maintain"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AnimationCard(
    title: &'static str,
    description: &'static str,
    animation_class: &'static str,
    animation_type: ReadSignal<String>,
    is_animating: ReadSignal<bool>,
    target_type: &'static str,
) -> impl IntoView {
    let is_active = move || animation_type.get() == target_type && is_animating.get();
    
    view! {
        <div class=move || {
            let base = "bg-white rounded-lg shadow-md p-6 border-2 transition-all duration-300";
            if is_active() {
                format!("{} border-blue-500", base)
            } else {
                format!("{} border-gray-200", base)
            }
        }>
            <h3 class="text-lg font-semibold mb-2 text-gray-800">{title}</h3>
            <p class="text-gray-600 mb-4">{description}</p>
            
            <div class=move || {
                let base = "w-full h-20 bg-gradient-to-r from-blue-400 to-purple-500 rounded-md flex items-center justify-center text-white font-semibold";
                if is_active() {
                    format!("{} opacity-100 animate-pulse", base)
                } else {
                    format!("{} opacity-50", base)
                }
            }>
                {move || if is_active() { "Animating!" } else { "Ready" }}
            </div>
            
            <div class="mt-4 text-sm text-gray-500">
                <code class="bg-gray-100 px-2 py-1 rounded">{animation_class}</code>
            </div>
        </div>
    }
}

#[component]
pub fn CSSAnimationShowcase() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <App />
        </div>
    }
}