//! Simple Demo - One Leptos Motion Capability
//!
//! Demonstrating basic animation with MotionDiv

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

mod advanced_3d_demo;
mod animate_fn_test;
mod animation_3d_demo;
mod basic_test_demo;
mod batched_updates_test;
mod fixed_reactive_demo;
mod memoization_test;
mod minimal_comprehensive_demo;
mod minimal_motion_test;
mod minimal_test_demo;
mod motion_showcase_demo;
mod phase4a_demo;
mod reactive_motion_div_test;
mod signal_based_comprehensive_demo;
mod simple_demo;
mod simple_div_test;
mod simple_phase4a_test;
mod simple_working_demo;
mod simplified_showcase;
mod tdd_reactive_demo;
mod transition_config_test;
mod ultra_simple_demo;
mod working_tdd_demo;
// mod simple_comprehensive_demo; // Commented out
// mod simplified_comprehensive_demo; // Commented out
// mod feature_showcase_demo;
// mod reactive_style_test;
// mod simple_animation_test;
// mod test_fix;
// mod demo_unit_tests;
// mod working_motion_demo;
// mod fixed_demo;
// mod simple_working_demo;
// mod signal_based_demo;
use simplified_showcase::SimplifiedShowcase;
// use simple_working_demo::SimpleWorkingDemo;
// use feature_showcase_demo::FeatureShowcaseDemo;
// use reactive_style_test::ReactiveStyleTest;
// use simple_animation_test::SimpleAnimationTest;
// use test_fix::TestFix;
// use working_motion_demo::WorkingMotionDemo;
// use fixed_demo::FixedDemo;
// use simple_working_demo::SimpleWorkingDemo;
// use signal_based_demo::SignalBasedDemo;

/// Main function to run the demo
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("Failed to initialize console log");

    web_sys::console::log_1(&"🚀 Leptos Motion: Starting main function".into());

    // Add error handling for the mount
    match std::panic::catch_unwind(|| {
        web_sys::console::log_1(&"📦 Leptos Motion: About to mount MotionShowcaseDemo".into());
        mount_to_body(|| {
            web_sys::console::log_1(&"🎨 Leptos Motion: Creating MotionShowcaseDemo view".into());
            web_sys::console::log_1(&"🎨 Leptos Motion: About to render MotionShowcaseDemo".into());
            let demo = view! { <SimplifiedShowcase/> };
            web_sys::console::log_1(
                &"🎨 Leptos Motion: MotionShowcaseDemo created successfully".into(),
            );
            demo
        });
        web_sys::console::log_1(
            &"✅ Leptos Motion: MotionShowcaseDemo mounted successfully".into(),
        );
    }) {
        Ok(_) => {
            web_sys::console::log_1(
                &"✅ Leptos Motion: Main function completed successfully".into(),
            );
        }
        Err(e) => {
            web_sys::console::error_1(
                &format!("❌ Leptos Motion: Main function panicked: {:?}", e).into(),
            );
        }
    }
}
