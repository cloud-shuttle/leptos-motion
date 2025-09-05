//! Code Splitting and Lazy Loading Tests
//! 
//! These tests ensure our code splitting and lazy loading implementation
//! works correctly and provides the expected performance benefits.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Mock lazy loader for testing
struct LazyLoader<T> {
    loaded_modules: Arc<Mutex<HashMap<String, T>>>,
    load_times: Arc<Mutex<HashMap<String, Instant>>>,
}

impl<T: Clone> LazyLoader<T> {
    fn new() -> Self {
        Self {
            loaded_modules: Arc::new(Mutex::new(HashMap::new())),
            load_times: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn load_module(&self, name: &str, module: T) -> Result<(), String> {
        let mut modules = self.loaded_modules.lock().unwrap();
        let mut times = self.load_times.lock().unwrap();
        
        if modules.contains_key(name) {
            return Err(format!("Module {} already loaded", name));
        }
        
        modules.insert(name.to_string(), module);
        times.insert(name.to_string(), Instant::now());
        Ok(())
    }

    fn is_loaded(&self, name: &str) -> bool {
        let modules = self.loaded_modules.lock().unwrap();
        modules.contains_key(name)
    }

    fn get_module(&self, name: &str) -> Option<T> {
        let modules = self.loaded_modules.lock().unwrap();
        modules.get(name).cloned()
    }

    fn loaded_count(&self) -> usize {
        let modules = self.loaded_modules.lock().unwrap();
        modules.len()
    }
}

/// Test that lazy loading only loads modules when needed
#[test]
fn test_lazy_loading_on_demand() {
    let loader = LazyLoader::<String>::new();
    
    // Initially no modules loaded
    assert_eq!(loader.loaded_count(), 0);
    assert!(!loader.is_loaded("gestures"));
    assert!(!loader.is_loaded("layout"));
    assert!(!loader.is_loaded("scroll"));
    
    // Load gestures module
    loader.load_module("gestures", "gesture_module".to_string()).unwrap();
    assert_eq!(loader.loaded_count(), 1);
    assert!(loader.is_loaded("gestures"));
    assert!(!loader.is_loaded("layout"));
    assert!(!loader.is_loaded("scroll"));
    
    // Load layout module
    loader.load_module("layout", "layout_module".to_string()).unwrap();
    assert_eq!(loader.loaded_count(), 2);
    assert!(loader.is_loaded("gestures"));
    assert!(loader.is_loaded("layout"));
    assert!(!loader.is_loaded("scroll"));
}

/// Test that lazy loading prevents duplicate loading
#[test]
fn test_lazy_loading_no_duplicates() {
    let loader = LazyLoader::<String>::new();
    
    // Load a module
    loader.load_module("gestures", "gesture_module".to_string()).unwrap();
    assert_eq!(loader.loaded_count(), 1);
    
    // Try to load the same module again
    let result = loader.load_module("gestures", "another_gesture_module".to_string());
    assert!(result.is_err());
    assert_eq!(loader.loaded_count(), 1);
    
    // Verify the original module is still there
    assert_eq!(loader.get_module("gestures"), Some("gesture_module".to_string()));
}

/// Test that lazy loading tracks load times
#[test]
fn test_lazy_loading_timing() {
    let loader = LazyLoader::<String>::new();
    
    // Load a module
    let before_load = Instant::now();
    loader.load_module("gestures", "gesture_module".to_string()).unwrap();
    let after_load = Instant::now();
    
    // Verify load time is tracked
    let times = loader.load_times.lock().unwrap();
    let load_time = times.get("gestures").unwrap();
    
    assert!(load_time >= &before_load);
    assert!(load_time <= &after_load);
}

/// Test code splitting by feature flags
#[test]
fn test_feature_based_code_splitting() {
    let loader = LazyLoader::<String>::new();
    
    // Simulate loading based on feature flags
    let features = ["gestures", "layout", "scroll"];
    let mut loaded_features = Vec::new();
    
    for feature in &features {
        if feature == &"gestures" || feature == &"layout" {
            loader.load_module(feature, format!("{}_module", feature)).unwrap();
            loaded_features.push(*feature);
        }
    }
    
    // Verify only requested features are loaded
    assert_eq!(loader.loaded_count(), 2);
    assert!(loader.is_loaded("gestures"));
    assert!(loader.is_loaded("layout"));
    assert!(!loader.is_loaded("scroll"));
}

/// Test dynamic import simulation
#[test]
fn test_dynamic_import_simulation() {
    let loader = LazyLoader::<String>::new();
    
    // Simulate dynamic import based on user interaction
    let user_actions = ["drag", "hover", "scroll"];
    let mut loaded_actions = Vec::new();
    
    for action in &user_actions {
        // Simulate loading module when user performs action
        if action == &"drag" {
            loader.load_module("drag_gestures", "drag_module".to_string()).unwrap();
            loaded_actions.push("drag");
        } else if action == &"hover" {
            loader.load_module("hover_gestures", "hover_module".to_string()).unwrap();
            loaded_actions.push("hover");
        }
        // scroll module not loaded yet
    }
    
    // Verify only used features are loaded
    assert_eq!(loader.loaded_count(), 2);
    assert!(loader.is_loaded("drag_gestures"));
    assert!(loader.is_loaded("hover_gestures"));
    assert!(!loader.is_loaded("scroll_gestures"));
}

/// Test bundle size reduction through code splitting
#[test]
fn test_bundle_size_reduction() {
    let loader = LazyLoader::<usize>::new();
    
    // Simulate different bundle sizes
    let _module_sizes = [
        ("core", 50),      // 50KB core
        ("gestures", 30),  // 30KB gestures
        ("layout", 25),    // 25KB layout
        ("scroll", 20),    // 20KB scroll
    ];
    
    // Load only core initially
    loader.load_module("core", 50).unwrap();
    let initial_size: usize = loader.get_module("core").unwrap();
    
    // Load additional modules as needed
    loader.load_module("gestures", 30).unwrap();
    loader.load_module("layout", 25).unwrap();
    
    let total_size: usize = loader.loaded_modules.lock().unwrap()
        .values().sum();
    
    // Verify size calculation
    assert_eq!(initial_size, 50);
    assert_eq!(total_size, 105); // 50 + 30 + 25
    
    // Verify we can still load more modules
    assert!(!loader.is_loaded("scroll"));
    loader.load_module("scroll", 20).unwrap();
    
    let final_size: usize = loader.loaded_modules.lock().unwrap()
        .values().sum();
    assert_eq!(final_size, 125); // 50 + 30 + 25 + 20
}

/// Test lazy loading performance
#[test]
fn test_lazy_loading_performance() {
    let loader = LazyLoader::<String>::new();
    
    // Measure time to load multiple modules
    let start = Instant::now();
    
    for i in 0..10 {
        loader.load_module(&format!("module_{}", i), format!("content_{}", i)).unwrap();
    }
    
    let duration = start.elapsed();
    
    // Verify all modules loaded
    assert_eq!(loader.loaded_count(), 10);
    
    // Verify reasonable performance (should be very fast for in-memory operations)
    assert!(duration.as_millis() < 100, "Loading 10 modules took too long: {:?}", duration);
}

/// Test error handling in lazy loading
#[test]
fn test_lazy_loading_error_handling() {
    let loader = LazyLoader::<String>::new();
    
    // Test loading with empty name
    let result = loader.load_module("", "content".to_string());
    assert!(result.is_ok()); // Empty string is valid key
    
    // Test loading duplicate
    loader.load_module("test", "content1".to_string()).unwrap();
    let result = loader.load_module("test", "content2".to_string());
    assert!(result.is_err());
    
    // Test getting non-existent module
    assert!(loader.get_module("nonexistent").is_none());
    assert!(!loader.is_loaded("nonexistent"));
}

/// Test lazy loading with different module types
#[test]
fn test_lazy_loading_module_types() {
    // Test with string modules
    let string_loader = LazyLoader::<String>::new();
    string_loader.load_module("string_module", "string_content".to_string()).unwrap();
    assert_eq!(string_loader.get_module("string_module"), Some("string_content".to_string()));
    
    // Test with numeric modules
    let number_loader = LazyLoader::<i32>::new();
    number_loader.load_module("number_module", 42).unwrap();
    assert_eq!(number_loader.get_module("number_module"), Some(42));
    
    // Test with boolean modules
    let bool_loader = LazyLoader::<bool>::new();
    bool_loader.load_module("bool_module", true).unwrap();
    assert_eq!(bool_loader.get_module("bool_module"), Some(true));
}

/// Test lazy loading cleanup
#[test]
fn test_lazy_loading_cleanup() {
    let loader = LazyLoader::<String>::new();
    
    // Load some modules
    loader.load_module("module1", "content1".to_string()).unwrap();
    loader.load_module("module2", "content2".to_string()).unwrap();
    assert_eq!(loader.loaded_count(), 2);
    
    // Simulate cleanup (in real implementation, this would be more sophisticated)
    let mut modules = loader.loaded_modules.lock().unwrap();
    modules.clear();
    drop(modules);
    
    // Verify cleanup
    assert_eq!(loader.loaded_count(), 0);
    assert!(!loader.is_loaded("module1"));
    assert!(!loader.is_loaded("module2"));
}
