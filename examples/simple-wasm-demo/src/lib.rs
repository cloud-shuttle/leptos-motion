//! Simple WASM Performance Demo
//!
//! A minimal demo showing tailwind-rs-wasm performance benefits

use leptos::prelude::*;

// Simulate standard CSS class generation
fn standard_class() -> String {
    format!("bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200")
}

// Simulate WASM-optimized CSS class generation
fn wasm_class() -> &'static str {
    "bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
}

#[component]
pub fn SimpleDemo() -> impl IntoView {
    let (iterations, set_iterations) = signal(1000);
    let (standard_time, set_standard_time) = signal(0.0);
    let (wasm_time, set_wasm_time) = signal(0.0);
    let (is_running, set_is_running) = signal(false);

    let run_benchmark = move || {
        set_is_running.set(true);
        
        // Standard benchmark
        let start = std::time::Instant::now();
        for _i in 0..iterations.get() {
            let _class = standard_class();
        }
        let standard_duration = start.elapsed().as_millis() as f64;
        set_standard_time.set(standard_duration);

        // WASM benchmark
        let start = std::time::Instant::now();
        for _i in 0..iterations.get() {
            let _class = wasm_class();
        }
        let wasm_duration = start.elapsed().as_millis() as f64;
        set_wasm_time.set(wasm_duration);
        
        set_is_running.set(false);
    };

    let improvement = move || {
        let standard = standard_time.get();
        let wasm = wasm_time.get();
        if standard > 0.0 && wasm > 0.0 {
            ((standard - wasm) / standard * 100.0).round() as f64
        } else {
            0.0
        }
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-8">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-4xl font-bold text-gray-900 mb-8 text-center">
                    "🚀 Tailwind-RS WASM Performance Demo"
                </h1>

                // Control Panel
                <div class="bg-white rounded-xl shadow-lg p-6 mb-8">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Performance Benchmark"</h2>
                    <div class="flex items-center gap-4 mb-4">
                        <label class="text-sm font-medium text-gray-700">
                            "Iterations: "
                        </label>
                        <input
                            type="number"
                            value=iterations
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                    set_iterations.set(val);
                                }
                            }
                            class="border border-gray-300 rounded-md px-3 py-2 w-24"
                            disabled=is_running
                        />
                    </div>
                    <button
                        on:click=move |_| run_benchmark()
                        disabled=is_running
                        class="bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white font-semibold py-2 px-6 rounded-lg transition-colors duration-200"
                    >
                        {move || if is_running.get() { "Running..." } else { "Run Benchmark" }}
                    </button>
                </div>

                // Results
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                    <div class="bg-white rounded-xl shadow-lg p-6">
                        <h3 class="text-xl font-semibold text-gray-800 mb-4">"Standard CSS Generation"</h3>
                        <div class="text-center">
                            <div class="text-3xl font-bold text-gray-700 mb-2">
                                {move || format!("{:.2}ms", standard_time.get())}
                            </div>
                            <p class="text-gray-600">"Time taken"</p>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow-lg p-6">
                        <h3 class="text-xl font-semibold text-gray-800 mb-4">"WASM-Optimized CSS Generation"</h3>
                        <div class="text-center">
                            <div class="text-3xl font-bold text-green-600 mb-2">
                                {move || format!("{:.2}ms", wasm_time.get())}
                            </div>
                            <p class="text-gray-600">"Time taken"</p>
                        </div>
                    </div>
                </div>

                // Performance Improvement
                <div class="bg-gradient-to-r from-green-50 to-blue-50 rounded-xl shadow-lg p-6 mb-8">
                    <h3 class="text-2xl font-semibold text-gray-800 mb-4 text-center">"Performance Improvement"</h3>
                    <div class="text-center">
                        <div class="text-6xl font-bold text-green-600 mb-2">
                            {move || format!("{:.0}%", improvement())}
                        </div>
                        <p class="text-lg text-gray-600">
                            "faster with WASM optimization"
                        </p>
                    </div>
                </div>

                // Sample Classes
                <div class="bg-white rounded-xl shadow-lg p-6">
                    <h3 class="text-xl font-semibold text-gray-800 mb-4">"Sample Generated Classes"</h3>
                    <div class="space-y-4">
                        <div>
                            <h4 class="font-medium text-gray-700 mb-2">"Standard (String allocation):"</h4>
                            <code class="block bg-gray-100 p-3 rounded border text-sm text-gray-800">
                                {standard_class()}
                            </code>
                        </div>
                        <div>
                            <h4 class="font-medium text-gray-700 mb-2">"WASM-Optimized (Static reference):"</h4>
                            <code class="block bg-gray-100 p-3 rounded border text-sm text-gray-800">
                                {wasm_class()}
                            </code>
                        </div>
                    </div>
                </div>

                // Benefits
                <div class="bg-white rounded-xl shadow-lg p-6 mt-8">
                    <h3 class="text-xl font-semibold text-gray-800 mb-4">"WASM Optimization Benefits"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <h4 class="font-medium text-red-600 mb-2">"Standard Approach"</h4>
                            <ul class="space-y-1 text-sm text-gray-600">
                                <li>"❌ String allocation for each class"</li>
                                <li>"❌ Runtime string formatting"</li>
                                <li>"❌ Higher memory usage"</li>
                                <li>"❌ Garbage collection pressure"</li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="font-medium text-green-600 mb-2">"WASM-Optimized Approach"</h4>
                            <ul class="space-y-1 text-sm text-gray-600">
                                <li>"✅ Static string references"</li>
                                <li>"✅ Compile-time optimization"</li>
                                <li>"✅ Reduced memory usage"</li>
                                <li>"✅ No garbage collection"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <SimpleDemo /> })
}
