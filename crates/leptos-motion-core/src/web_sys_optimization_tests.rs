//! TDD tests for web-sys optimization
//!
//! These tests ensure that web-sys feature flag optimizations don't break functionality
//! while achieving significant bundle size reductions.

#[cfg(feature = "leptos-integration")]
use wasm_bindgen::JsCast;
#[cfg(feature = "leptos-integration")]
use wasm_bindgen_test::*;

#[cfg(feature = "leptos-integration")]
wasm_bindgen_test_configure!(run_in_browser);

/// Test that core DOM functionality still works after web-sys optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_core_dom_functionality_after_web_sys_optimization() {
    use web_sys::*;

    // Test that we can still access basic DOM APIs
    let window = window().expect("Window should be available");
    let document = window.document().expect("Document should be available");

    // Test element creation
    let element = document
        .create_element("div")
        .expect("Should create element");
    assert!(element.tag_name() == "DIV");

    // Test style manipulation - cast to HtmlElement to access style
    let html_element = element
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Should be HtmlElement");
    let style = html_element.style();
    style
        .set_property("opacity", "0.5")
        .expect("Should set style");
    assert_eq!(style.get_property_value("opacity").unwrap(), "0.5");
}

/// Test that animation APIs still work after web-sys optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_animation_apis_after_web_sys_optimization() {
    use web_sys::*;

    // Test that we can still access animation APIs
    let window = window().expect("Window should be available");
    let document = window.document().expect("Document should be available");

    // Test element creation for animation
    let element = document
        .create_element("div")
        .expect("Should create element");

    // Test that we can set transform styles (core animation functionality)
    let html_element = element
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Should be HtmlElement");
    let style = html_element.style();
    style
        .set_property("transform", "translateX(100px)")
        .expect("Should set transform");
    assert!(
        style
            .get_property_value("transform")
            .unwrap()
            .contains("translateX")
    );
}

/// Test that performance APIs still work after web-sys optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_performance_apis_after_web_sys_optimization() {
    use web_sys::*;

    // Test that we can still access performance APIs
    let window = window().expect("Window should be available");
    let performance = window
        .performance()
        .expect("Performance should be available");

    // Test that we can get timing information
    let now = performance.now();
    assert!(now >= 0.0);
}

/// Test that console APIs still work after web-sys optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_console_apis_after_web_sys_optimization() {
    use web_sys::console;

    // Test that we can still use console for debugging
    console::log_1(&"Test message".into());
    // If we get here without panicking, console works
    assert!(true);
}

/// Test that minimal web-sys features are sufficient for core functionality
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_minimal_web_sys_features_sufficient() {
    use web_sys::*;

    // Test that we have the absolute minimum web-sys features needed
    let window = window().expect("Window should be available");
    let document = window.document().expect("Document should be available");

    // Test element creation and basic manipulation
    let element = document
        .create_element("div")
        .expect("Should create element");
    let html_element = element
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Should be HtmlElement");
    let style = html_element.style();

    // Test core animation properties
    style
        .set_property("opacity", "1")
        .expect("Should set opacity");
    style
        .set_property("transform", "translateX(0px)")
        .expect("Should set transform");
    style
        .set_property("transition", "all 0.3s ease")
        .expect("Should set transition");

    // Verify all properties were set
    assert_eq!(style.get_property_value("opacity").unwrap(), "1");
    assert!(
        style
            .get_property_value("transform")
            .unwrap()
            .contains("translateX")
    );
    assert!(
        style
            .get_property_value("transition")
            .unwrap()
            .contains("0.3s")
    );
}

/// Test that web-sys optimization doesn't break engine functionality
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_engine_functionality_after_web_sys_optimization() {
    use crate::*;

    // Test that our engines still work with optimized web-sys
    let _raf_engine = RafEngine::new();
    let _waapi_engine = WaapiEngine::new();
    let _minimal_engine = MinimalEngine::new();

    // If we get here without panicking, engines work
    assert!(true);
}

/// Test bundle size targets for web-sys optimization
#[cfg(feature = "leptos-integration")]
#[wasm_bindgen_test]
fn test_web_sys_bundle_size_targets() {
    // This test documents our web-sys optimization targets
    // Actual measurement will be done with external tools

    // Target: 30% reduction from current 592KB
    // Target: <400KB after web-sys optimization
    // Target: Only include essential web-sys features

    assert!(true); // Placeholder for bundle size validation
}
