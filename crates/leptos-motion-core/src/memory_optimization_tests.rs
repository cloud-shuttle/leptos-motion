// TDD Tests for Memory Optimization
//
// This module contains tests to identify and fix memory issues
// to achieve the target of <10MB memory usage.

use crate::memory_optimization::*;

/// Test that memory-optimized cache works correctly
#[test]
fn test_memory_optimized_cache_basic_functionality() {
    let mut cache = MemoryOptimizedCache::new(3);

    // Insert values
    cache.insert("key1", 1.0);
    cache.insert("key2", 2.0);
    cache.insert("key3", 3.0);

    assert_eq!(cache.len(), 3);
    assert!(!cache.is_empty());

    // Get values
    assert_eq!(cache.get(&"key1"), Some(&1.0));
    assert_eq!(cache.get(&"key2"), Some(&2.0));
    assert_eq!(cache.get(&"key3"), Some(&3.0));

    // Test LRU eviction
    cache.insert("key4", 4.0);
    assert_eq!(cache.len(), 3);

    // Clear cache
    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

/// Test that memory-optimized cache handles LRU eviction correctly
#[test]
fn test_memory_optimized_cache_lru_eviction() {
    let mut cache = MemoryOptimizedCache::new(2);

    // Insert two items
    cache.insert("key1", 1.0);
    cache.insert("key2", 2.0);

    // Access key1 to make it more recent
    cache.get(&"key1");

    // Insert third item - should evict key2 (least recently used)
    cache.insert("key3", 3.0);

    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get(&"key1"), Some(&1.0)); // Should still be there
    assert_eq!(cache.get(&"key2"), None); // Should be evicted
    assert_eq!(cache.get(&"key3"), Some(&3.0)); // Should be there
}

/// Test that memory-optimized cache handles empty cache correctly
#[test]
fn test_memory_optimized_cache_empty() {
    let mut cache: MemoryOptimizedCache<&str, f64> = MemoryOptimizedCache::new(5);

    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.get(&"nonexistent"), None);
}

/// Test that memory-optimized cache handles zero size limit correctly
#[test]
fn test_memory_optimized_cache_zero_size() {
    let mut cache: MemoryOptimizedCache<&str, f64> = MemoryOptimizedCache::new(0);

    // Should not be able to insert anything
    cache.insert("key1", 1.0);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

/// Test that memory-optimized cache handles large size limit correctly
#[test]
fn test_memory_optimized_cache_large_size() {
    let mut cache = MemoryOptimizedCache::new(1000);

    // Insert many items
    for i in 0..1000 {
        cache.insert(format!("key_{}", i), i as f64);
    }

    assert_eq!(cache.len(), 1000);
    assert!(!cache.is_empty());

    // Check some values
    assert_eq!(cache.get(&"key_0".to_string()), Some(&0.0));
    assert_eq!(cache.get(&"key_999".to_string()), Some(&999.0));
    assert_eq!(cache.get(&"key_1000".to_string()), None); // Should not exist
}

/// Test that memory-optimized cache handles string keys correctly
#[test]
fn test_memory_optimized_cache_string_keys() {
    let mut cache = MemoryOptimizedCache::new(3);

    cache.insert("hello".to_string(), "world".to_string());
    cache.insert("foo".to_string(), "bar".to_string());
    cache.insert("test".to_string(), "value".to_string());

    assert_eq!(cache.get(&"hello".to_string()), Some(&"world".to_string()));
    assert_eq!(cache.get(&"foo".to_string()), Some(&"bar".to_string()));
    assert_eq!(cache.get(&"test".to_string()), Some(&"value".to_string()));
}

/// Test that memory-optimized cache handles numeric keys correctly
#[test]
fn test_memory_optimized_cache_numeric_keys() {
    let mut cache = MemoryOptimizedCache::new(3);

    cache.insert(1, "one");
    cache.insert(2, "two");
    cache.insert(3, "three");

    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&3), Some(&"three"));
    assert_eq!(cache.get(&4), None);
}

/// Test that memory-optimized cache handles complex values correctly
#[test]
fn test_memory_optimized_cache_complex_values() {
    let mut cache = MemoryOptimizedCache::new(2);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    cache.insert("vec1", vec1.clone());
    cache.insert("vec2", vec2.clone());

    assert_eq!(cache.get(&"vec1"), Some(&vec1));
    assert_eq!(cache.get(&"vec2"), Some(&vec2));
}

/// Test that memory-optimized cache handles concurrent-like access patterns
#[test]
fn test_memory_optimized_cache_access_patterns() {
    let mut cache = MemoryOptimizedCache::new(5);

    // Insert items
    for i in 0..5 {
        cache.insert(i, i * 2);
    }

    // Access items in different patterns
    cache.get(&0); // Access first
    cache.get(&4); // Access last
    cache.get(&2); // Access middle

    // Insert new item - should evict least recently used
    cache.insert(5, 10);

    // Check what's still there
    assert_eq!(cache.len(), 5);
    assert_eq!(cache.get(&0), Some(&0)); // Should still be there (accessed)
    assert_eq!(cache.get(&4), Some(&8)); // Should still be there (accessed)
    assert_eq!(cache.get(&2), Some(&4)); // Should still be there (accessed)
    assert_eq!(cache.get(&5), Some(&10)); // Should be there (newly inserted)
}
