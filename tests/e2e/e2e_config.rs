//! E2E Test Configuration and Utilities
//!
//! This module provides configuration, setup, and utilities for end-to-end testing
//! of the Leptos Motion library in real browser environments.

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::*;

/// E2E Test Configuration
#[derive(Debug, Clone)]
pub struct E2EConfig {
    /// Test timeout in milliseconds
    pub timeout_ms: u64,
    /// Animation wait time in milliseconds
    pub animation_wait_ms: u64,
    /// Whether to enable debug logging
    pub debug_logging: bool,
    /// Browser viewport settings
    pub viewport: ViewportConfig,
    /// Performance monitoring settings
    pub performance_monitoring: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct ViewportConfig {
    pub width: u32,
    pub height: u32,
    pub device_scale_factor: f64,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub monitor_fps: bool,
    pub monitor_memory: bool,
    pub performance_threshold_ms: u64,
}

impl Default for E2EConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 10000,
            animation_wait_ms: 500,
            debug_logging: true,
            viewport: ViewportConfig {
                width: 1280,
                height: 720,
                device_scale_factor: 1.0,
            },
            performance_monitoring: PerformanceConfig {
                monitor_fps: true,
                monitor_memory: true,
                performance_threshold_ms: 16, // 60fps threshold
            },
        }
    }
}

/// E2E Test Environment
pub struct E2ETestEnvironment {
    config: E2EConfig,
    window: Window,
    document: Document,
    performance_monitor: Option<PerformanceMonitor>,
    test_results: Vec<TestResult>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub memory_usage_mb: f64,
    pub animation_frame_count: u32,
    pub total_animation_time_ms: u64,
}

impl E2ETestEnvironment {
    pub fn new(config: E2EConfig) -> Self {
        let window = window().expect("Window should be available");
        let document = window.document().expect("Document should be available");

        let performance_monitor = if config.performance_monitoring.monitor_fps
            || config.performance_monitoring.monitor_memory
        {
            Some(PerformanceMonitor::new(&config.performance_monitoring))
        } else {
            None
        };

        Self {
            config,
            window,
            document,
            performance_monitor,
            test_results: Vec::new(),
        }
    }

    pub fn setup_test_environment(&self) -> Result<(), JsValue> {
        // Set up test environment
        self.setup_viewport()?;
        self.setup_console_logging()?;
        self.setup_performance_monitoring()?;
        Ok(())
    }

    fn setup_viewport(&self) -> Result<(), JsValue> {
        // Set viewport size
        let viewport_meta = self.document.create_element("meta")?;
        viewport_meta.set_attribute("name", "viewport")?;
        viewport_meta.set_attribute(
            "content",
            &format!(
                "width={}, height={}, initial-scale={}",
                self.config.viewport.width,
                self.config.viewport.height,
                self.config.viewport.device_scale_factor
            ),
        )?;

        if let Some(head) = self.document.head() {
            head.append_child(&viewport_meta)?;
        }

        Ok(())
    }

    fn setup_console_logging(&self) -> Result<(), JsValue> {
        if self.config.debug_logging {
            // Set up enhanced console logging for E2E tests
            let console = self.window.console();
            console.log_1(&"E2E Test Environment Initialized".into());
        }
        Ok(())
    }

    fn setup_performance_monitoring(&self) -> Result<(), JsValue> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.start_monitoring()?;
        }
        Ok(())
    }

    pub fn run_test<F>(&mut self, test_name: &str, test_fn: F) -> TestResult
    where
        F: FnOnce(&E2ETestEnvironment) -> Result<(), String>,
    {
        let start_time = js_sys::Date::now();

        if self.config.debug_logging {
            web_sys::console::log_1(&format!("Starting E2E test: {}", test_name).into());
        }

        let result = match test_fn(self) {
            Ok(()) => {
                let duration_ms = (js_sys::Date::now() - start_time) as u64;
                let performance_metrics = self
                    .performance_monitor
                    .as_ref()
                    .and_then(|m| m.get_metrics().ok());

                TestResult {
                    test_name: test_name.to_string(),
                    passed: true,
                    duration_ms,
                    error_message: None,
                    performance_metrics,
                }
            }
            Err(error) => {
                let duration_ms = (js_sys::Date::now() - start_time) as u64;

                TestResult {
                    test_name: test_name.to_string(),
                    passed: false,
                    duration_ms,
                    error_message: Some(error),
                    performance_metrics: None,
                }
            }
        };

        self.test_results.push(result.clone());

        if self.config.debug_logging {
            let status = if result.passed { "PASSED" } else { "FAILED" };
            web_sys::console::log_1(
                &format!(
                    "E2E test {}: {} ({}ms)",
                    test_name, status, result.duration_ms
                )
                .into(),
            );
        }

        result
    }

    pub fn get_test_results(&self) -> &[TestResult] {
        &self.test_results
    }

    pub fn cleanup(&self) -> Result<(), JsValue> {
        if let Some(ref monitor) = self.performance_monitor {
            monitor.stop_monitoring()?;
        }
        Ok(())
    }
}

/// Performance Monitor for E2E Tests
pub struct PerformanceMonitor {
    config: PerformanceConfig,
    start_time: f64,
    frame_count: u32,
    animation_start_time: Option<f64>,
    total_animation_time: f64,
    memory_samples: Vec<f64>,
}

impl PerformanceMonitor {
    pub fn new(config: &PerformanceConfig) -> Self {
        Self {
            config: config.clone(),
            start_time: js_sys::Date::now(),
            frame_count: 0,
            animation_start_time: None,
            total_animation_time: 0.0,
            memory_samples: Vec::new(),
        }
    }

    pub fn start_monitoring(&self) -> Result<(), JsValue> {
        if self.config.monitor_fps {
            self.setup_fps_monitoring()?;
        }

        if self.config.monitor_memory {
            self.setup_memory_monitoring()?;
        }

        Ok(())
    }

    fn setup_fps_monitoring(&self) -> Result<(), JsValue> {
        // Set up requestAnimationFrame monitoring
        let closure = Closure::wrap(Box::new(|| {
            // This would be implemented to track frame timing
        }) as Box<dyn FnMut()>);

        self.window()
            .request_animation_frame(closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    fn setup_memory_monitoring(&self) -> Result<(), JsValue> {
        // Set up memory monitoring
        let performance = self.window().performance()?;

        // Sample memory usage periodically
        let closure = Closure::wrap(Box::new(|| {
            // This would sample memory usage
        }) as Box<dyn FnMut()>);

        self.window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                1000,
            )?;
        closure.forget();

        Ok(())
    }

    pub fn start_animation_timing(&mut self) {
        self.animation_start_time = Some(js_sys::Date::now());
    }

    pub fn end_animation_timing(&mut self) {
        if let Some(start_time) = self.animation_start_time {
            let duration = js_sys::Date::now() - start_time;
            self.total_animation_time += duration;
            self.animation_start_time = None;
        }
    }

    pub fn get_metrics(&self) -> Result<PerformanceMetrics, JsValue> {
        let current_time = js_sys::Date::now();
        let total_time = current_time - self.start_time;
        let fps = if total_time > 0.0 {
            (self.frame_count as f64 * 1000.0) / total_time
        } else {
            0.0
        };

        let memory_usage_mb = if !self.memory_samples.is_empty() {
            self.memory_samples.iter().sum::<f64>() / self.memory_samples.len() as f64
        } else {
            0.0
        };

        Ok(PerformanceMetrics {
            fps,
            memory_usage_mb,
            animation_frame_count: self.frame_count,
            total_animation_time_ms: self.total_animation_time as u64,
        })
    }

    pub fn stop_monitoring(&self) -> Result<(), JsValue> {
        // Clean up monitoring
        Ok(())
    }

    fn window(&self) -> Window {
        window().expect("Window should be available")
    }
}

/// E2E Test Assertions
pub struct E2EAssertions<'a> {
    env: &'a E2ETestEnvironment,
}

impl<'a> E2EAssertions<'a> {
    pub fn new(env: &'a E2ETestEnvironment) -> Self {
        Self { env }
    }

    pub fn assert_element_visible(&self, element: &Element) -> Result<(), String> {
        let style = self
            .env
            .window
            .get_computed_style(element)
            .map_err(|_| "Failed to get computed style")?
            .ok_or("No computed style found")?;

        let display = style.get_property_value("display").unwrap_or_default();
        let visibility = style.get_property_value("visibility").unwrap_or_default();
        let opacity = style.get_property_value("opacity").unwrap_or_default();

        if display == "none" {
            return Err("Element is not visible (display: none)".to_string());
        }

        if visibility == "hidden" {
            return Err("Element is not visible (visibility: hidden)".to_string());
        }

        if opacity == "0" {
            return Err("Element is not visible (opacity: 0)".to_string());
        }

        Ok(())
    }

    pub fn assert_animation_completed(
        &self,
        element: &Element,
        property: &str,
        expected_value: &str,
    ) -> Result<(), String> {
        let start_time = js_sys::Date::now();
        let timeout = self.env.config.animation_wait_ms as f64;

        while js_sys::Date::now() - start_time < timeout {
            let current_value = self.get_computed_style_property(element, property)?;
            if current_value.contains(expected_value) {
                return Ok(());
            }

            // Small delay to prevent busy waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Err(format!(
            "Animation for property '{}' did not complete to '{}' within {}ms",
            property, expected_value, self.env.config.animation_wait_ms
        ))
    }

    pub fn assert_performance_threshold(&self, metrics: &PerformanceMetrics) -> Result<(), String> {
        if metrics.fps < 30.0 {
            return Err(format!("FPS too low: {} (expected >= 30)", metrics.fps));
        }

        if metrics.total_animation_time_ms
            > self
                .env
                .config
                .performance_monitoring
                .performance_threshold_ms
        {
            return Err(format!(
                "Animation time too long: {}ms (expected <= {}ms)",
                metrics.total_animation_time_ms,
                self.env
                    .config
                    .performance_monitoring
                    .performance_threshold_ms
            ));
        }

        Ok(())
    }

    fn get_computed_style_property(
        &self,
        element: &Element,
        property: &str,
    ) -> Result<String, String> {
        let computed_style = self
            .env
            .window
            .get_computed_style(element)
            .map_err(|_| "Failed to get computed style")?
            .ok_or("No computed style found")?;

        Ok(computed_style
            .get_property_value(property)
            .unwrap_or_default())
    }
}

/// E2E Test Utilities
pub struct E2EUtils;

impl E2EUtils {
    /// Create a test element with specified properties
    pub fn create_test_element(
        document: &Document,
        tag: &str,
        id: &str,
        styles: &str,
        text_content: Option<&str>,
    ) -> Result<Element, JsValue> {
        let element = document.create_element(tag)?;
        element.set_id(id);
        element.set_attribute("style", styles)?;

        if let Some(text) = text_content {
            element.set_text_content(Some(text));
        }

        Ok(element)
    }

    /// Simulate user interaction
    pub fn simulate_click(element: &Element) -> Result<(), JsValue> {
        let click_event = MouseEvent::new("click")?;
        element.dispatch_event(&click_event)?;
        Ok(())
    }

    pub fn simulate_hover(element: &Element) -> Result<(), JsValue> {
        let mouseenter_event = MouseEvent::new("mouseenter")?;
        element.dispatch_event(&mouseenter_event)?;
        Ok(())
    }

    pub fn simulate_unhover(element: &Element) -> Result<(), JsValue> {
        let mouseleave_event = MouseEvent::new("mouseleave")?;
        element.dispatch_event(&mouseleave_event)?;
        Ok(())
    }

    /// Wait for specified duration
    pub fn wait_for_duration(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    /// Generate test data
    pub fn generate_test_data(count: usize) -> Vec<HashMap<String, String>> {
        (0..count)
            .map(|i| {
                let mut data = HashMap::new();
                data.insert("id".to_string(), i.to_string());
                data.insert("name".to_string(), format!("Test Item {}", i + 1));
                data.insert("value".to_string(), (i * 10).to_string());
                data
            })
            .collect()
    }
}

/// E2E Test Suite Runner
pub struct E2ETestSuite {
    config: E2EConfig,
    tests: Vec<Box<dyn Fn(&mut E2ETestEnvironment) -> TestResult>>,
}

impl E2ETestSuite {
    pub fn new(config: E2EConfig) -> Self {
        Self {
            config,
            tests: Vec::new(),
        }
    }

    pub fn add_test<F>(&mut self, test_name: &str, test_fn: F)
    where
        F: Fn(&E2ETestEnvironment) -> Result<(), String> + 'static,
    {
        let test_name = test_name.to_string();
        let test = Box::new(move |env: &mut E2ETestEnvironment| env.run_test(&test_name, &test_fn));
        self.tests.push(test);
    }

    pub fn run_all_tests(&self) -> Vec<TestResult> {
        let mut env = E2ETestEnvironment::new(self.config.clone());

        if let Err(e) = env.setup_test_environment() {
            web_sys::console::error_1(&format!("Failed to setup test environment: {:?}", e).into());
            return vec![];
        }

        let mut results = Vec::new();

        for test in &self.tests {
            let result = test(&mut env);
            results.push(result);
        }

        if let Err(e) = env.cleanup() {
            web_sys::console::error_1(
                &format!("Failed to cleanup test environment: {:?}", e).into(),
            );
        }

        results
    }

    pub fn generate_report(&self, results: &[TestResult]) -> String {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;

        let mut report = format!(
            "E2E Test Report\n\
            ===============\n\
            Total Tests: {}\n\
            Passed: {}\n\
            Failed: {}\n\
            Success Rate: {:.1}%\n\n",
            total_tests,
            passed_tests,
            failed_tests,
            if total_tests > 0 {
                (passed_tests as f64 / total_tests as f64) * 100.0
            } else {
                0.0
            }
        );

        for result in results {
            let status = if result.passed { "PASS" } else { "FAIL" };
            report.push_str(&format!(
                "{} {} ({}ms)\n",
                status, result.test_name, result.duration_ms
            ));

            if let Some(ref error) = result.error_message {
                report.push_str(&format!("  Error: {}\n", error));
            }

            if let Some(ref metrics) = result.performance_metrics {
                report.push_str(&format!(
                    "  Performance: {:.1} FPS, {:.1} MB memory\n",
                    metrics.fps, metrics.memory_usage_mb
                ));
            }
        }

        report
    }
}
