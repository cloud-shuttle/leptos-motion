use leptos_motion_dom::performance::{CSSOptimizer, UpdatePriority, UpdateQueue};
use std::collections::HashMap;

#[test]
fn test_update_priority_ordering() {
    assert!(UpdatePriority::Critical > UpdatePriority::High);
    assert!(UpdatePriority::High > UpdatePriority::Normal);
    assert!(UpdatePriority::Normal > UpdatePriority::Low);
}

#[test]
fn test_update_priority_debug() {
    let priorities = vec![
        UpdatePriority::Low,
        UpdatePriority::Normal,
        UpdatePriority::High,
        UpdatePriority::Critical,
    ];
    
    for priority in priorities {
        let debug_str = format!("{:?}", priority);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_update_priority_clone() {
    let priority = UpdatePriority::High;
    let cloned = priority.clone();
    
    assert_eq!(priority, cloned);
}

#[test]
fn test_update_priority_copy() {
    let priority = UpdatePriority::Critical;
    let copied = priority;
    
    assert_eq!(priority, copied);
}

#[test]
fn test_update_priority_equality() {
    let priority1 = UpdatePriority::High;
    let priority2 = UpdatePriority::High;
    let priority3 = UpdatePriority::Low;
    
    assert_eq!(priority1, priority2);
    assert_ne!(priority1, priority3);
}

#[test]
fn test_css_optimizer_creation() {
    let optimizer = CSSOptimizer::new();
    
    assert_eq!(optimizer.transform_cache.len(), 0);
    assert_eq!(optimizer.opacity_cache.len(), 0);
    assert_eq!(optimizer.scale_cache.len(), 0);
    assert_eq!(optimizer.rotation_cache.len(), 0);
}

#[test]
fn test_css_optimizer_cache_hit() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache a transform
    let key = "transform_key".to_string();
    let value = "translate3d(100px, 200px, 0px)".to_string();
    
    optimizer.cache_transform(key.clone(), value.clone());
    
    // Should find cached value
    let cached = optimizer.get_cached_transform(&key);
    assert_eq!(cached, Some(&value));
}

#[test]
fn test_css_optimizer_cache_miss() {
    let optimizer = CSSOptimizer::new();
    
    // Try to get non-cached transform
    let key = "non_existent_key".to_string();
    let cached = optimizer.get_cached_transform(&key);
    
    assert!(cached.is_none());
}

#[test]
fn test_css_optimizer_cache_multiple_transforms() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache multiple transforms
    let transforms = vec![
        ("key1".to_string(), "translateX(100px)".to_string()),
        ("key2".to_string(), "translateY(200px)".to_string()),
        ("key3".to_string(), "scale(1.5)".to_string()),
    ];
    
    for (key, value) in transforms.iter() {
        optimizer.cache_transform(key.clone(), value.clone());
    }
    
    // Verify all are cached
    for (key, value) in transforms.iter() {
        let cached = optimizer.get_cached_transform(key);
        assert_eq!(cached, Some(value));
    }
}

#[test]
fn test_css_optimizer_cache_opacity() {
    let mut optimizer = CSSOptimizer::new();
    
    let key = "opacity_key".to_string();
    let value = "0.8".to_string();
    
    optimizer.cache_opacity(key.clone(), value.clone());
    
    let cached = optimizer.get_cached_opacity(&key);
    assert_eq!(cached, Some(&value));
}

#[test]
fn test_css_optimizer_cache_scale() {
    let mut optimizer = CSSOptimizer::new();
    
    let key = "scale_key".to_string();
    let value = "1.2".to_string();
    
    optimizer.cache_scale(key.clone(), value.clone());
    
    let cached = optimizer.get_cached_scale(&key);
    assert_eq!(cached, Some(&value));
}

#[test]
fn test_css_optimizer_cache_rotation() {
    let mut optimizer = CSSOptimizer::new();
    
    let key = "rotation_key".to_string();
    let value = "45deg".to_string();
    
    optimizer.cache_rotation(key.clone(), value.clone());
    
    let cached = optimizer.get_cached_rotation(&key);
    assert_eq!(cached, Some(&value));
}

#[test]
fn test_css_optimizer_cache_overflow() {
    let mut optimizer = CSSOptimizer::new();
    
    // Fill cache beyond reasonable limits
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        optimizer.cache_transform(key, value);
    }
    
    // Should still be able to cache new values
    let new_key = "new_key".to_string();
    let new_value = "new_value".to_string();
    optimizer.cache_transform(new_key.clone(), new_value.clone());
    
    let cached = optimizer.get_cached_transform(&new_key);
    assert_eq!(cached, Some(&new_value));
}

#[test]
fn test_css_optimizer_clear_cache() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache some values
    optimizer.cache_transform("key1".to_string(), "value1".to_string());
    optimizer.cache_opacity("key2".to_string(), "0.5".to_string());
    optimizer.cache_scale("key3".to_string(), "1.0".to_string());
    optimizer.cache_rotation("key4".to_string(), "0deg".to_string());
    
    // Clear all caches
    optimizer.clear_cache();
    
    // All caches should be empty
    assert_eq!(optimizer.transform_cache.len(), 0);
    assert_eq!(optimizer.opacity_cache.len(), 0);
    assert_eq!(optimizer.scale_cache.len(), 0);
    assert_eq!(optimizer.rotation_cache.len(), 0);
}

#[test]
fn test_css_optimizer_clear_specific_cache() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache values in different caches
    optimizer.cache_transform("key1".to_string(), "value1".to_string());
    optimizer.cache_opacity("key2".to_string(), "0.5".to_string());
    optimizer.cache_scale("key3".to_string(), "1.0".to_string());
    
    // Clear only transform cache
    optimizer.clear_transform_cache();
    
    // Transform cache should be empty, others should remain
    assert_eq!(optimizer.transform_cache.len(), 0);
    assert_eq!(optimizer.opacity_cache.len(), 1);
    assert_eq!(optimizer.scale_cache.len(), 1);
}

#[test]
fn test_css_optimizer_cache_stats() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache some values
    optimizer.cache_transform("key1".to_string(), "value1".to_string());
    optimizer.cache_opacity("key2".to_string(), "0.5".to_string());
    optimizer.cache_scale("key3".to_string(), "1.0".to_string());
    
    // Get cache statistics
    let transform_count = optimizer.transform_cache.len();
    let opacity_count = optimizer.opacity_cache.len();
    let scale_count = optimizer.scale_cache.len();
    let rotation_count = optimizer.rotation_cache.len();
    
    assert_eq!(transform_count, 1);
    assert_eq!(opacity_count, 1);
    assert_eq!(scale_count, 1);
    assert_eq!(rotation_count, 0);
}

#[test]
fn test_css_optimizer_cache_key_collision() {
    let mut optimizer = CSSOptimizer::new();
    
    let key = "same_key".to_string();
    let value1 = "first_value".to_string();
    let value2 = "second_value".to_string();
    
    // Cache first value
    optimizer.cache_transform(key.clone(), value1.clone());
    assert_eq!(optimizer.get_cached_transform(&key), Some(&value1));
    
    // Cache second value with same key (should overwrite)
    optimizer.cache_transform(key.clone(), value2.clone());
    assert_eq!(optimizer.get_cached_transform(&key), Some(&value2));
    
    // First value should be gone
    assert_ne!(optimizer.get_cached_transform(&key), Some(&value1));
}

#[test]
fn test_css_optimizer_cache_different_types() {
    let mut optimizer = CSSOptimizer::new();
    
    let key = "same_key".to_string();
    
    // Cache in different caches with same key
    optimizer.cache_transform(key.clone(), "transform_value".to_string());
    optimizer.cache_opacity(key.clone(), "0.8".to_string());
    optimizer.cache_scale(key.clone(), "1.5".to_string());
    optimizer.cache_rotation(key.clone(), "90deg".to_string());
    
    // All should be cached separately
    assert_eq!(optimizer.get_cached_transform(&key), Some(&"transform_value".to_string()));
    assert_eq!(optimizer.get_cached_opacity(&key), Some(&"0.8".to_string()));
    assert_eq!(optimizer.get_cached_scale(&key), Some(&"1.5".to_string()));
    assert_eq!(optimizer.get_cached_rotation(&key), Some(&"90deg".to_string()));
}

#[test]
fn test_css_optimizer_cache_performance() {
    let mut optimizer = CSSOptimizer::new();
    
    // Measure cache performance with many entries
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        optimizer.cache_transform(key, value);
    }
    
    let cache_time = start.elapsed();
    
    // Caching should be fast
    assert!(cache_time.as_millis() < 100); // Should complete in under 100ms
    
    // Measure retrieval performance
    let start = std::time::Instant::now();
    
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let _cached = optimizer.get_cached_transform(&key);
    }
    
    let retrieval_time = start.elapsed();
    
    // Retrieval should be very fast
    assert!(retrieval_time.as_millis() < 50); // Should complete in under 50ms
}

#[test]
fn test_css_optimizer_memory_usage() {
    let mut optimizer = CSSOptimizer::new();
    
    // Cache many values
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        optimizer.cache_transform(key, value);
    }
    
    // Memory usage should be reasonable
    let transform_memory = optimizer.transform_cache.len() * std::mem::size_of::<(String, String)>();
    let opacity_memory = optimizer.opacity_cache.len() * std::mem::size_of::<(String, String)>();
    let scale_memory = optimizer.scale_cache.len() * std::mem::size_of::<(String, String)>();
    let rotation_memory = optimizer.rotation_cache.len() * std::mem::size_of::<(String, String)>();
    
    let total_memory = transform_memory + opacity_memory + scale_memory + rotation_memory;
    
    // Should be reasonable (under 1MB for 1000 entries)
    assert!(total_memory < 1024 * 1024);
}

#[test]
fn test_css_optimizer_cache_eviction() {
    let mut optimizer = CSSOptimizer::new();
    
    // Fill cache with many entries
    for i in 0..10000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        optimizer.cache_transform(key, value);
    }
    
    // Cache should still be functional
    let test_key = "test_key".to_string();
    let test_value = "test_value".to_string();
    
    optimizer.cache_transform(test_key.clone(), test_value.clone());
    let cached = optimizer.get_cached_transform(&test_key);
    
    assert_eq!(cached, Some(&test_value));
}

#[test]
fn test_css_optimizer_debug() {
    let optimizer = CSSOptimizer::new();
    let debug_str = format!("{:?}", optimizer);
    
    assert!(debug_str.contains("CSSOptimizer"));
    assert!(debug_str.contains("transform_cache"));
    assert!(debug_str.contains("opacity_cache"));
    assert!(debug_str.contains("scale_cache"));
    assert!(debug_str.contains("rotation_cache"));
}

#[test]
fn test_css_optimizer_clone() {
    let mut optimizer = CSSOptimizer::new();
    
    // Add some cached values
    optimizer.cache_transform("key1".to_string(), "value1".to_string());
    optimizer.cache_opacity("key2".to_string(), "0.5".to_string());
    
    let cloned = optimizer.clone();
    
    // Cloned optimizer should have the same cached values
    assert_eq!(cloned.get_cached_transform(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(cloned.get_cached_opacity(&"key2".to_string()), Some(&"0.5".to_string()));
}

#[test]
fn test_update_queue_creation() {
    let queue = UpdateQueue::new();
    
    assert_eq!(queue.high_priority.len(), 0);
    assert_eq!(queue.normal_priority.len(), 0);
    assert_eq!(queue.low_priority.len(), 0);
}

#[test]
fn test_update_queue_add_update() {
    let mut queue = UpdateQueue::new();
    
    let update = "test_update".to_string();
    queue.add_update(update.clone(), UpdatePriority::High);
    
    assert_eq!(queue.high_priority.len(), 1);
    assert_eq!(queue.normal_priority.len(), 0);
    assert_eq!(queue.low_priority.len(), 0);
}

#[test]
fn test_update_queue_add_multiple_priorities() {
    let mut queue = UpdateQueue::new();
    
    queue.add_update("high1".to_string(), UpdatePriority::High);
    queue.add_update("high2".to_string(), UpdatePriority::High);
    queue.add_update("normal1".to_string(), UpdatePriority::Normal);
    queue.add_update("low1".to_string(), UpdatePriority::Low);
    
    assert_eq!(queue.high_priority.len(), 2);
    assert_eq!(queue.normal_priority.len(), 1);
    assert_eq!(queue.low_priority.len(), 1);
}

#[test]
fn test_update_queue_process_updates() {
    let mut queue = UpdateQueue::new();
    
    // Add updates with different priorities
    queue.add_update("high1".to_string(), UpdatePriority::High);
    queue.add_update("normal1".to_string(), UpdatePriority::Normal);
    queue.add_update("low1".to_string(), UpdatePriority::Low);
    
    // Process updates
    let high_updates = queue.process_high_priority();
    let normal_updates = queue.process_normal_priority();
    let low_updates = queue.process_low_priority();
    
    assert_eq!(high_updates.len(), 1);
    assert_eq!(normal_updates.len(), 1);
    assert_eq!(low_updates.len(), 1);
    
    // Queues should be empty after processing
    assert_eq!(queue.high_priority.len(), 0);
    assert_eq!(queue.normal_priority.len(), 0);
    assert_eq!(queue.low_priority.len(), 0);
}

#[test]
fn test_update_queue_is_empty() {
    let mut queue = UpdateQueue::new();
    
    assert!(queue.is_empty());
    
    queue.add_update("test".to_string(), UpdatePriority::Normal);
    
    assert!(!queue.is_empty());
    
    queue.process_normal_priority();
    
    assert!(queue.is_empty());
}

#[test]
fn test_update_queue_clear() {
    let mut queue = UpdateQueue::new();
    
    // Add updates
    queue.add_update("high1".to_string(), UpdatePriority::High);
    queue.add_update("normal1".to_string(), UpdatePriority::Normal);
    queue.add_update("low1".to_string(), UpdatePriority::Low);
    
    // Clear all
    queue.clear();
    
    assert!(queue.is_empty());
}

#[test]
fn test_update_queue_debug() {
    let queue = UpdateQueue::new();
    let debug_str = format!("{:?}", queue);
    
    assert!(debug_str.contains("UpdateQueue"));
    assert!(debug_str.contains("high_priority"));
    assert!(debug_str.contains("normal_priority"));
    assert!(debug_str.contains("low_priority"));
}

#[test]
fn test_update_queue_clone() {
    let mut queue = UpdateQueue::new();
    
    // Add some updates
    queue.add_update("high1".to_string(), UpdatePriority::High);
    queue.add_update("normal1".to_string(), UpdatePriority::Normal);
    
    let cloned = queue.clone();
    
    // Cloned queue should have the same updates
    assert_eq!(cloned.high_priority.len(), 1);
    assert_eq!(cloned.normal_priority.len(), 1);
    assert_eq!(cloned.low_priority.len(), 0);
}

