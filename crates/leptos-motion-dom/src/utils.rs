//! DOM utilities for motion animations

use web_sys::{Element, HtmlElement};

/// Get computed style for an element
pub fn get_computed_style(element: &Element, property: &str) -> Option<String> {
    web_sys::window()?
        .get_computed_style(element)
        .ok()?
        .and_then(|style| style.get_property_value(property).ok())
}

/// Set CSS property on element
pub fn set_css_property(
    element: &HtmlElement,
    property: &str,
    value: &str,
) -> Result<(), wasm_bindgen::JsValue> {
    element.style().set_property(property, value)
}

/// Apply multiple CSS properties
pub fn apply_css_properties(element: &HtmlElement, properties: &[(String, String)]) {
    let style = element.style();
    for (property, value) in properties {
        let _ = style.set_property(property, value);
    }
}
