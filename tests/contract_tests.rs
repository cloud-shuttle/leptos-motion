//! Contract Tests for Leptos Motion
//!
//! This test suite runs comprehensive contract tests to ensure:
//! - API stability and backward compatibility
//! - Cross-crate contract validation
//! - Performance guarantees
//! - Error handling consistency
//! - Memory usage contracts

use leptos_motion_contracts::*;

#[test]
fn test_all_contracts() {
    let report = run_contract_tests();
    report.print_report();
    
    // Save report to file
    let _ = report.save_to_file("contract_test_report.md");
    
    // Assert that all critical contracts pass
    assert!(report.failed_tests == 0, 
        "Contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_core_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-core");
    report.print_report();
    
    // Assert that core contracts pass
    assert!(report.failed_tests == 0, 
        "Core crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_dom_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-dom");
    report.print_report();
    
    // Assert that DOM contracts pass
    assert!(report.failed_tests == 0, 
        "DOM crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_gestures_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-gestures");
    report.print_report();
    
    // Assert that gestures contracts pass
    assert!(report.failed_tests == 0, 
        "Gestures crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_layout_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-layout");
    report.print_report();
    
    // Assert that layout contracts pass
    assert!(report.failed_tests == 0, 
        "Layout crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_studio_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-studio");
    report.print_report();
    
    // Assert that studio contracts pass
    assert!(report.failed_tests == 0, 
        "Studio crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_webgl_crate_contracts() {
    let report = run_crate_contract_tests("leptos-motion-webgl");
    report.print_report();
    
    // Assert that WebGL contracts pass
    assert!(report.failed_tests == 0, 
        "WebGL crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_performance_contracts() {
    let runner = ContractTestRunner::new();
    let results = performance_contracts::run_all_performance_contract_tests();
    let report = runner.generate_report(&results);
    
    report.print_report();
    
    // Assert that performance contracts pass
    assert!(report.failed_tests == 0, 
        "Performance contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_memory_contracts() {
    let runner = ContractTestRunner::new();
    let results = memory_contracts::run_all_memory_contract_tests();
    let report = runner.generate_report(&results);
    
    report.print_report();
    
    // Assert that memory contracts pass
    assert!(report.failed_tests == 0, 
        "Memory contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_error_handling_contracts() {
    let runner = ContractTestRunner::new();
    let results = error_contracts::run_all_error_contract_tests();
    let report = runner.generate_report(&results);
    
    report.print_report();
    
    // Assert that error handling contracts pass
    assert!(report.failed_tests == 0, 
        "Error handling contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_cross_crate_contracts() {
    let runner = ContractTestRunner::new();
    let results = cross_crate_contracts::run_all_cross_crate_contract_tests();
    let report = runner.generate_report(&results);
    
    report.print_report();
    
    // Assert that cross-crate contracts pass
    assert!(report.failed_tests == 0, 
        "Cross-crate contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}

#[test]
fn test_api_contracts() {
    let runner = ContractTestRunner::new();
    let results = api_contracts::run_all_api_contract_tests();
    let report = runner.generate_report(&results);
    
    report.print_report();
    
    // Assert that API contracts pass
    assert!(report.failed_tests == 0, 
        "API contract tests failed: {}/{} tests failed", 
        report.failed_tests, 
        report.total_tests);
}
