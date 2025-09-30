//! WASM-compatible contract tests for Leptos Motion
//!
//! This module provides WASM-compatible versions of contract tests that require
//! DOM access and browser APIs.

use leptos_motion_contracts::{run_contract_tests, ContractTestResult};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

// This is like the `main` function, but for WASM
#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Log that WASM module has started
    console::log_1(&"🧪 WASM Contract Tests Module Loaded".into());
}

// Function that can be called from JavaScript to run the tests
#[wasm_bindgen]
pub fn run_wasm_contract_tests() -> Vec<JsContractTestResult> {
    console::log_1(&"Running WASM contract tests...".into());

    // Check if we have DOM access
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            console::log_1(&"✅ DOM access confirmed".into());

            // Create a test element to verify DOM manipulation works
            match document.create_element("div") {
                Ok(test_element) => {
                    console::log_1(&"✅ DOM element creation successful".into());
                    let _ = test_element; // Use it to avoid warning
                }
                Err(e) => {
                    console::log_1(&format!("❌ DOM element creation failed: {:?}", e).into());
                }
            }
        } else {
            console::log_1(&"❌ No document available".into());
        }
    } else {
        console::log_1(&"❌ No window available".into());
    }

    // Run the actual contract tests
    let results = run_contract_tests();

    console::log_1(&format!("Contract tests completed: {} tests", results.len()).into());

    // Convert to JS-compatible format
    results.into_iter().map(JsContractTestResult::from).collect()
}

// JavaScript-compatible version of ContractTestResult
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsContractTestResult {
    test_name: String,
    passed: bool,
    duration_ms: f64,
    error_message: Option<String>,
    metrics: Vec<(String, f64)>,
}

#[wasm_bindgen]
impl JsContractTestResult {
    #[wasm_bindgen(getter)]
    pub fn test_name(&self) -> String {
        self.test_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn passed(&self) -> bool {
        self.passed
    }

    #[wasm_bindgen(getter)]
    pub fn duration(&self) -> f64 {
        self.duration_ms
    }

    #[wasm_bindgen(getter)]
    pub fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn metrics(&self) -> Vec<JsValue> {
        self.metrics.iter().map(|(k, v)| {
            // Create a simple object with key and value
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"key".into(), &k.into()).unwrap();
            js_sys::Reflect::set(&obj, &"value".into(), &v.into()).unwrap();
            obj.into()
        }).collect()
    }
}

impl From<ContractTestResult> for JsContractTestResult {
    fn from(result: ContractTestResult) -> Self {
        let metrics: Vec<(String, f64)> = result.metrics.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        JsContractTestResult {
            test_name: result.test_name,
            passed: result.passed,
            duration_ms: result.duration.as_secs_f64() * 1000.0,
            error_message: result.error_message,
            metrics,
        }
    }
}
