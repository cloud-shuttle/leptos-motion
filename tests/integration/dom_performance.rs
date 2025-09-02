//! Integration tests for DOM performance optimizations

use leptos::prelude::*;
use leptos_motion_dom::performance::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_css_optimizer_property_categorization() {
    let optimizer = CSSOptimizer::new();
    
    // Test GPU-accelerated properties
    assert!(optimizer.is_gpu_accelerated("transform"));
    assert!(optimizer.is_gpu_accelerated("opacity"));
    assert!(optimizer.is_gpu_accelerated("filter"));
    
    // Test layout-triggering properties
    assert!(optimizer.is_layout_triggering("width"));
    assert!(optimizer.is_layout_triggering("height"));
    assert!(optimizer.is_layout_triggering("padding"));
    assert!(optimizer.is_layout_triggering("margin"));
    
    // Test paint-triggering properties
    assert!(optimizer.is_paint_triggering("background-color"));
    assert!(optimizer.is_paint_triggering("border-color"));
    assert!(optimizer.is_paint_triggering("color"));
}

#[wasm_bindgen_test]
async fn test_dom_batcher_batching() {
    let mut batcher = DOMBatcher::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    // Add updates with different priorities
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "opacity".to_string(),
        value: "0.5".to_string(),
        priority: UpdatePriority::High,
    });
    
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "transform".to_string(),
        value: "translateX(100px)".to_string(),
        priority: UpdatePriority::Normal,
    });
    
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "background-color".to_string(),
        value: "red".to_string(),
        priority: UpdatePriority::Low,
    });
    
    assert_eq!(batcher.pending_updates.len(), 3);
    
    // Process updates
    let processed = batcher.process_updates();
    assert_eq!(processed.len(), 3);
    assert_eq!(batcher.pending_updates.len(), 0);
}

#[wasm_bindgen_test]
async fn test_element_cache() {
    let mut cache = ElementCache::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    element.set_id("test-element");
    
    // Test caching
    assert!(cache.get("test-element").is_none());
    
    cache.set("test-element", element.clone());
    let cached = cache.get("test-element");
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().id(), "test-element");
    
    // Test cache hit
    let cached_again = cache.get("test-element");
    assert!(cached_again.is_some());
    
    // Test cache miss
    assert!(cache.get("non-existent").is_none());
}

#[wasm_bindgen_test]
async fn test_dom_performance_monitor() {
    let mut monitor = DOMPerformanceMonitor::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    // Start monitoring
    monitor.start_operation("test_operation");
    
    // Simulate some DOM operations
    element.set_attribute("data-test", "value").unwrap();
    element.set_class_name("test-class");
    
    // End monitoring
    let metrics = monitor.end_operation("test_operation");
    
    assert!(metrics.is_some());
    let metrics = metrics.unwrap();
    assert!(metrics.duration > 0.0);
    assert_eq!(metrics.operation_name, "test_operation");
}

#[wasm_bindgen_test]
async fn test_css_optimizer_batch_optimization() {
    let optimizer = CSSOptimizer::new();
    
    let properties = vec![
        ("transform", "translateX(100px)"),
        ("opacity", "0.5"),
        ("width", "200px"),
        ("background-color", "red"),
    ];
    
    let optimized = optimizer.optimize_batch(properties);
    
    // Should group by optimization type
    assert!(optimized.gpu_accelerated.len() > 0);
    assert!(optimized.layout_triggering.len() > 0);
    assert!(optimized.paint_triggering.len() > 0);
}

#[wasm_bindgen_test]
async fn test_dom_batcher_priority_ordering() {
    let mut batcher = DOMBatcher::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    // Add updates in reverse priority order
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "low".to_string(),
        value: "value".to_string(),
        priority: UpdatePriority::Low,
    });
    
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "high".to_string(),
        value: "value".to_string(),
        priority: UpdatePriority::High,
    });
    
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "normal".to_string(),
        value: "value".to_string(),
        priority: UpdatePriority::Normal,
    });
    
    // Process updates and check order
    let processed = batcher.process_updates();
    assert_eq!(processed.len(), 3);
    
    // High priority should come first
    assert_eq!(processed[0].property, "high");
    // Normal priority should come second
    assert_eq!(processed[1].property, "normal");
    // Low priority should come last
    assert_eq!(processed[2].property, "low");
}

#[wasm_bindgen_test]
async fn test_element_cache_eviction() {
    let mut cache = ElementCache::new();
    
    // Create test elements
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Fill cache beyond capacity
    for i in 0..150 {
        let element = document.create_element("div").unwrap();
        element.set_id(&format!("element-{}", i));
        cache.set(&format!("element-{}", i), element);
    }
    
    // Cache should have evicted some elements
    assert!(cache.get("element-0").is_none()); // Should be evicted
    assert!(cache.get("element-149").is_some()); // Should still be there
}

#[wasm_bindgen_test]
async fn test_dom_performance_monitor_multiple_operations() {
    let mut monitor = DOMPerformanceMonitor::new();
    
    // Start multiple operations
    monitor.start_operation("op1");
    monitor.start_operation("op2");
    
    // End operations
    let metrics1 = monitor.end_operation("op1");
    let metrics2 = monitor.end_operation("op2");
    
    assert!(metrics1.is_some());
    assert!(metrics2.is_some());
    
    // Get overall report
    let report = monitor.get_report();
    assert_eq!(report.operation_count, 2);
    assert!(report.total_duration > 0.0);
}

#[wasm_bindgen_test]
async fn test_css_optimizer_property_validation() {
    let optimizer = CSSOptimizer::new();
    
    // Test valid properties
    assert!(optimizer.is_valid_property("transform"));
    assert!(optimizer.is_valid_property("opacity"));
    assert!(optimizer.is_valid_property("background-color"));
    
    // Test invalid properties
    assert!(!optimizer.is_valid_property("invalid-property"));
    assert!(!optimizer.is_valid_property(""));
}

#[wasm_bindgen_test]
async fn test_dom_batcher_concurrent_updates() {
    let mut batcher = DOMBatcher::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    // Add multiple updates to the same property
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "opacity".to_string(),
        value: "0.5".to_string(),
        priority: UpdatePriority::Normal,
    });
    
    batcher.add_update(PendingUpdate {
        element: element.clone(),
        property: "opacity".to_string(),
        value: "1.0".to_string(),
        priority: UpdatePriority::High,
    });
    
    // Should deduplicate and keep the higher priority
    let processed = batcher.process_updates();
    assert_eq!(processed.len(), 1);
    assert_eq!(processed[0].value, "1.0");
}

#[wasm_bindgen_test]
async fn test_element_cache_performance() {
    let mut cache = ElementCache::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    element.set_id("performance-test");
    
    // Measure cache performance
    let start = web_sys::window().unwrap().performance().unwrap().now();
    
    for _ in 0..1000 {
        cache.set("performance-test", element.clone());
        let _ = cache.get("performance-test");
    }
    
    let end = web_sys::window().unwrap().performance().unwrap().now();
    let duration = end - start;
    
    // Should be fast (less than 10ms for 1000 operations)
    assert!(duration < 10.0);
}

#[wasm_bindgen_test]
async fn test_dom_performance_monitor_memory_tracking() {
    let mut monitor = DOMPerformanceMonitor::new();
    
    // Start operation
    monitor.start_operation("memory_test");
    
    // Simulate memory allocation
    let mut data = Vec::new();
    for _ in 0..1000 {
        data.push("test".to_string());
    }
    
    // End operation
    let metrics = monitor.end_operation("memory_test");
    
    assert!(metrics.is_some());
    let metrics = metrics.unwrap();
    assert!(metrics.memory_usage > 0);
}

#[wasm_bindgen_test]
async fn test_css_optimizer_complex_properties() {
    let optimizer = CSSOptimizer::new();
    
    // Test complex transform values
    assert!(optimizer.is_gpu_accelerated("transform"));
    assert!(optimizer.is_gpu_accelerated("transform-origin"));
    
    // Test complex filter values
    assert!(optimizer.is_gpu_accelerated("filter"));
    assert!(optimizer.is_gpu_accelerated("backdrop-filter"));
    
    // Test complex layout properties
    assert!(optimizer.is_layout_triggering("display"));
    assert!(optimizer.is_layout_triggering("position"));
    assert!(optimizer.is_layout_triggering("float"));
}

#[wasm_bindgen_test]
async fn test_dom_batcher_error_handling() {
    let mut batcher = DOMBatcher::new();
    
    // Test with invalid element
    let invalid_element = web_sys::Element::new("invalid").unwrap();
    
    batcher.add_update(PendingUpdate {
        element: invalid_element,
        property: "opacity".to_string(),
        value: "0.5".to_string(),
        priority: UpdatePriority::Normal,
    });
    
    // Should handle gracefully
    let processed = batcher.process_updates();
    assert_eq!(processed.len(), 1);
}

#[wasm_bindgen_test]
async fn test_element_cache_clear() {
    let mut cache = ElementCache::new();
    
    // Create test element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    element.set_id("test-clear");
    
    cache.set("test-clear", element);
    assert!(cache.get("test-clear").is_some());
    
    // Clear cache
    cache.clear();
    assert!(cache.get("test-clear").is_none());
    assert_eq!(cache.size(), 0);
}
