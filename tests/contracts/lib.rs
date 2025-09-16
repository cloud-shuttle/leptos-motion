//! Contract Testing Library for Leptos Motion
//!
//! This library provides comprehensive contract testing for the leptos-motion ecosystem.
//! It ensures API stability, performance guarantees, error handling consistency,
//! and memory usage contracts across all crates.

pub mod api_contracts;
pub mod cross_crate_contracts;
pub mod performance_contracts;
pub mod error_contracts;
pub mod memory_contracts;
pub mod simple_contract_tests;

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Contract test result with detailed metrics
#[derive(Debug, Clone)]
pub struct ContractTestResult {
    pub test_name: String,
    pub passed: bool,
    pub duration: Duration,
    pub metrics: HashMap<String, f64>,
    pub error_message: Option<String>,
}

/// Contract test suite that can be run across all crates
pub trait ContractTestSuite {
    fn name(&self) -> &str;
    fn run(&self) -> Vec<ContractTestResult>;
    fn required_features(&self) -> Vec<&str>;
}

/// Performance contract specification
#[derive(Debug, Clone)]
pub struct PerformanceContract {
    pub operation_name: String,
    pub max_duration_ms: f64,
    pub max_memory_mb: f64,
    pub min_throughput_ops_per_sec: f64,
}

/// API contract specification
#[derive(Debug, Clone)]
pub struct ApiContract {
    pub interface_name: String,
    pub version: String,
    pub backward_compatible: bool,
    pub required_methods: Vec<String>,
    pub required_types: Vec<String>,
}

/// Error contract specification
#[derive(Debug, Clone)]
pub struct ErrorContract {
    pub error_type: String,
    pub expected_error_codes: Vec<String>,
    pub recovery_possible: bool,
    pub error_message_format: String,
}

/// Memory contract specification
#[derive(Debug, Clone)]
pub struct MemoryContract {
    pub operation_name: String,
    pub max_memory_mb: f64,
    pub max_memory_growth_mb: f64,
    pub cleanup_required: bool,
    pub leak_tolerance_mb: f64,
}

/// Contract test runner
pub struct ContractTestRunner {
    pub performance_contracts: Vec<PerformanceContract>,
    pub api_contracts: Vec<ApiContract>,
    pub error_contracts: Vec<ErrorContract>,
    pub memory_contracts: Vec<MemoryContract>,
}

impl ContractTestRunner {
    pub fn new() -> Self {
        Self {
            performance_contracts: Vec::new(),
            api_contracts: Vec::new(),
            error_contracts: Vec::new(),
            memory_contracts: Vec::new(),
        }
    }

    pub fn add_performance_contract(&mut self, contract: PerformanceContract) {
        self.performance_contracts.push(contract);
    }

    pub fn add_api_contract(&mut self, contract: ApiContract) {
        self.api_contracts.push(contract);
    }

    pub fn add_error_contract(&mut self, contract: ErrorContract) {
        self.error_contracts.push(contract);
    }

    pub fn add_memory_contract(&mut self, contract: MemoryContract) {
        self.memory_contracts.push(contract);
    }

    /// Run all contract tests
    pub fn run_all_contracts(&self) -> Vec<ContractTestResult> {
        let mut results = Vec::new();
        
        // Run API contract tests
        results.extend(api_contracts::run_all_api_contract_tests());
        
        // Run cross-crate contract tests
        results.extend(cross_crate_contracts::run_all_cross_crate_contract_tests());
        
        // Run performance contract tests
        results.extend(performance_contracts::run_all_performance_contract_tests());
        
        // Run error handling contract tests
        results.extend(error_contracts::run_all_error_contract_tests());
        
        // Run memory contract tests
        results.extend(memory_contracts::run_all_memory_contract_tests());
        
        results
    }

    /// Run contract tests for a specific crate
    pub fn run_crate_contracts(&self, crate_name: &str) -> Vec<ContractTestResult> {
        let mut results = Vec::new();
        
        match crate_name {
            "leptos-motion-core" => {
                results.extend(api_contracts::test_leptos_motion_core_api_contracts());
            },
            "leptos-motion-dom" => {
                results.extend(api_contracts::test_leptos_motion_dom_api_contracts());
            },
            "leptos-motion-gestures" => {
                results.extend(api_contracts::test_leptos_motion_gestures_api_contracts());
            },
            "leptos-motion-layout" => {
                results.extend(api_contracts::test_leptos_motion_layout_api_contracts());
            },
            "leptos-motion-studio" => {
                results.extend(api_contracts::test_leptos_motion_studio_api_contracts());
            },
            "leptos-motion-webgl" => {
                results.extend(api_contracts::test_leptos_motion_webgl_api_contracts());
            },
            _ => {
                // Run all tests if crate name not recognized
                results.extend(self.run_all_contracts());
            }
        }
        
        results
    }

    /// Generate contract test report
    pub fn generate_report(&self, results: &[ContractTestResult]) -> ContractTestReport {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        
        let total_duration = results.iter()
            .map(|r| r.duration)
            .fold(Duration::new(0, 0), |acc, d| acc + d);
        
        let avg_duration = if total_tests > 0 {
            total_duration.as_secs_f64() / total_tests as f64
        } else {
            0.0
        };
        
        let mut test_categories = HashMap::new();
        for result in results {
            let category = result.test_name.split('_').next().unwrap_or("unknown").to_string();
            let entry = test_categories.entry(category).or_insert((0, 0));
            if result.passed {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
        
        ContractTestReport {
            total_tests,
            passed_tests,
            failed_tests,
            total_duration,
            avg_duration_ms: avg_duration * 1000.0,
            test_categories,
            results: results.to_vec(),
        }
    }
}

/// Contract test report
#[derive(Debug, Clone)]
pub struct ContractTestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub avg_duration_ms: f64,
    pub test_categories: HashMap<String, (usize, usize)>, // (passed, failed)
    pub results: Vec<ContractTestResult>,
}

impl ContractTestReport {
    /// Print the contract test report
    pub fn print_report(&self) {
        println!("=== Leptos Motion Contract Test Report ===");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {}", self.passed_tests);
        println!("Failed: {}", self.failed_tests);
        println!("Success Rate: {:.1}%", 
            if self.total_tests > 0 { 
                (self.passed_tests as f64 / self.total_tests as f64) * 100.0 
            } else { 
                0.0 
            });
        println!("Total Duration: {:?}", self.total_duration);
        println!("Average Duration: {:.2}ms", self.avg_duration_ms);
        
        println!("\n=== Test Categories ===");
        for (category, (passed, failed)) in &self.test_categories {
            let total = passed + failed;
            let success_rate = if total > 0 { 
                (*passed as f64 / total as f64) * 100.0 
            } else { 
                0.0 
            };
            println!("{}: {}/{} passed ({:.1}%)", category, passed, total, success_rate);
        }
        
        if self.failed_tests > 0 {
            println!("\n=== Failed Tests ===");
            for result in &self.results {
                if !result.passed {
                    println!("❌ {}: {}", result.test_name, 
                        result.error_message.as_deref().unwrap_or("Unknown error"));
                }
            }
        }
        
        println!("\n=== Performance Metrics ===");
        for result in &self.results {
            if !result.metrics.is_empty() {
                println!("{}:", result.test_name);
                for (metric, value) in &result.metrics {
                    println!("  {}: {:.3}", metric, value);
                }
            }
        }
    }
    
    /// Save the report to a file
    pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(filename)?;
        
        writeln!(file, "# Leptos Motion Contract Test Report")?;
        writeln!(file, "")?;
        writeln!(file, "## Summary")?;
        writeln!(file, "- Total Tests: {}", self.total_tests)?;
        writeln!(file, "- Passed: {}", self.passed_tests)?;
        writeln!(file, "- Failed: {}", self.failed_tests)?;
        writeln!(file, "- Success Rate: {:.1}%", 
            if self.total_tests > 0 { 
                (self.passed_tests as f64 / self.total_tests as f64) * 100.0 
            } else { 
                0.0 
            })?;
        writeln!(file, "- Total Duration: {:?}", self.total_duration)?;
        writeln!(file, "- Average Duration: {:.2}ms", self.avg_duration_ms)?;
        
        writeln!(file, "\n## Test Categories")?;
        for (category, (passed, failed)) in &self.test_categories {
            let total = passed + failed;
            let success_rate = if total > 0 { 
                (*passed as f64 / total as f64) * 100.0 
            } else { 
                0.0 
            };
            writeln!(file, "- {}: {}/{} passed ({:.1}%)", category, passed, total, success_rate)?;
        }
        
        if self.failed_tests > 0 {
            writeln!(file, "\n## Failed Tests")?;
            for result in &self.results {
                if !result.passed {
                    writeln!(file, "- ❌ {}: {}", result.test_name, 
                        result.error_message.as_deref().unwrap_or("Unknown error"))?;
                }
            }
        }
        
        writeln!(file, "\n## Performance Metrics")?;
        for result in &self.results {
            if !result.metrics.is_empty() {
                writeln!(file, "### {}", result.test_name)?;
                for (metric, value) in &result.metrics {
                    writeln!(file, "- {}: {:.3}", metric, value)?;
                }
            }
        }
        
        Ok(())
    }
}

/// Utility functions for contract testing
pub mod utils {
    use super::*;
    
    /// Measure memory usage of an operation
    pub fn measure_memory_usage<F, R>(operation: F) -> (R, f64)
    where
        F: FnOnce() -> R,
    {
        // In a real implementation, this would use platform-specific memory measurement
        // For now, we'll return a placeholder
        let result = operation();
        let memory_usage = 0.0; // Placeholder
        (result, memory_usage)
    }
    
    /// Measure execution time of an operation
    pub fn measure_execution_time<F, R>(operation: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Assert that a value is within acceptable range
    pub fn assert_within_range(value: f64, min: f64, max: f64, name: &str) -> bool {
        if value < min || value > max {
            eprintln!("Contract violation: {} = {} is not within range [{}, {}]", name, value, min, max);
            false
        } else {
            true
        }
    }
    
    /// Assert that an operation completes within time limit
    pub fn assert_duration_contract<F>(operation: F, max_duration: Duration, name: &str) -> bool
    where
        F: FnOnce(),
    {
        let (_, duration) = measure_execution_time(operation);
        if duration > max_duration {
            eprintln!("Duration contract violation: {} took {:?}, max allowed: {:?}", name, duration, max_duration);
            false
        } else {
            true
        }
    }
}

/// Run all contract tests and generate a report
pub fn run_contract_tests() -> ContractTestReport {
    let runner = ContractTestRunner::new();
    let results = runner.run_all_contracts();
    runner.generate_report(&results)
}

/// Run contract tests for a specific crate
pub fn run_crate_contract_tests(crate_name: &str) -> ContractTestReport {
    let runner = ContractTestRunner::new();
    let results = runner.run_crate_contracts(crate_name);
    runner.generate_report(&results)
}
