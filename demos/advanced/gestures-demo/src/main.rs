use advanced_gestures::AdvancedGesturesApp;
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <AdvancedGesturesApp /> });
}
