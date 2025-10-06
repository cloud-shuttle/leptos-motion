use leptos::*;
use leptos_motion::*;
use advanced_features_showcase::AdvancedFeaturesShowcase;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <AdvancedFeaturesShowcase />
        }
    });
}
