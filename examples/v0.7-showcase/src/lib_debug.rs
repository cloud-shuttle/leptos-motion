use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Minimal WASM test starting".into());
    
    // Test basic WASM functionality
    let result = add(5, 3);
    web_sys::console::log_1(&format!("5 + 3 = {}", result).into());
    
    // Test DOM manipulation
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(app_div) = document.get_element_by_id("app") {
                app_div.set_inner_html("<h1>Minimal WASM Test</h1><p>This is a basic WASM test without Leptos.</p>");
                web_sys::console::log_1(&"DOM manipulation successful".into());
            } else {
                web_sys::console::log_1(&"App div not found".into());
            }
        } else {
            web_sys::console::log_1(&"Document not found".into());
        }
    } else {
        web_sys::console::log_1(&"Window not found".into());
    }
    
    web_sys::console::log_1(&"Minimal WASM test completed".into());
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn test_responsiveness() -> String {
    "WASM is responsive".to_string()
}
