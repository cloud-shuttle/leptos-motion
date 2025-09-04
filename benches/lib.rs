// Benchmark library for Leptos Motion
// 
// This library provides benchmark utilities and shared code
// for performance testing across the Leptos Motion ecosystem.

pub mod utils {
    // Utility functions for benchmarks
    pub fn create_test_data(size: usize) -> Vec<f64> {
        (0..size).map(|i| i as f64).collect()
    }
    
    pub fn create_test_strings(size: usize) -> Vec<String> {
        (0..size).map(|i| format!("test_key_{}", i)).collect()
    }
}
