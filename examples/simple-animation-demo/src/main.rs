use wasm_bindgen::prelude::*;
use web_sys::window;

// Simple test function without Leptos
#[wasm_bindgen]
pub fn test_rust() {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"🚀 RUST FUNCTION CALLED!".into());
    
    // Simple test - just set innerHTML directly
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(app_element) = document.get_element_by_id("app") {
                web_sys::console::log_1(&"✅ Found #app element!".into());
                app_element.set_inner_html("<h1>🎉 RUST IS WORKING!</h1><p>This text was set by Rust!</p>");
                web_sys::console::log_1(&"✅ Content set successfully!".into());
                return;
            }
        }
    }
    
    web_sys::console::log_1(&"❌ Failed to find #app element!".into());
}

// Export a simple function that just logs
#[wasm_bindgen]
pub fn hello_world() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}

// Main function for non-WASM targets
fn main() {
    println!("Simple Animation Demo - This is a WASM-only example");
}
