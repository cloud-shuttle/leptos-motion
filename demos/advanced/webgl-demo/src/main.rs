use leptos::*;

mod simple_demo;
use simple_demo::*;

/// WebGL Advanced Features Demo
///
/// This demo showcases the advanced WebGL capabilities implemented in Phase 3:
/// - Post-processing effects (bloom, SSAO, tone mapping)
/// - Shadow mapping with directional and point lights
/// - Physics simulation with collision detection
/// - Interactive controls for real-time parameter adjustment

#[component]
pub fn WebGLAdvancedDemo() -> impl IntoView {
    view! {
        <SimpleWebGLDemo />
    }
}

fn main() {
    mount_to_body(WebGLAdvancedDemo)
}
