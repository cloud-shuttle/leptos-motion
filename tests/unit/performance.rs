use leptos_motion_core::performance::{
    PerformanceBudget, PerformanceMonitor, FrameMetrics, AnimationPriority
};
use std::time::Duration;

#[test]
fn test_performance_budget_default() {
    let budget = PerformanceBudget::default();
    
    assert_eq!(budget.max_frame_time, 16.67); // 60fps target
    assert_eq!(budget.max_concurrent_animations, 100);
    assert_eq!(budget.max_memory_usage, 10 * 1024 * 1024); // 10MB
    assert_eq!(budget.max_animation_duration, 5000.0); // 5 seconds
}

#[test]
fn test_performance_budget_custom() {
    let budget = PerformanceBudget {
        max_frame_time: 33.33, // 30fps
        max_concurrent_animations: 50,
        max_memory_usage: 5 * 1024 * 1024, // 5MB
        max_animation_duration: 3000.0, // 3 seconds
    };
    
    assert_eq!(budget.max_frame_time, 33.33);
    assert_eq!(budget.max_concurrent_animations, 50);
    assert_eq!(budget.max_memory_usage, 5 * 1024 * 1024);
    assert_eq!(budget.max_animation_duration, 3000.0);
}

#[test]
fn test_performance_budget_clone() {
    let budget = PerformanceBudget::default();
    let cloned = budget.clone();
    
    assert_eq!(cloned.max_frame_time, budget.max_frame_time);
    assert_eq!(cloned.max_concurrent_animations, budget.max_concurrent_animations);
    assert_eq!(cloned.max_memory_usage, budget.max_memory_usage);
    assert_eq!(cloned.max_animation_duration, budget.max_animation_duration);
}

#[test]
fn test_performance_budget_debug() {
    let budget = PerformanceBudget::default();
    let debug_str = format!("{:?}", budget);
    
    assert!(debug_str.contains("PerformanceBudget"));
    assert!(debug_str.contains("16.67"));
    assert!(debug_str.contains("100"));
}

#[test]
fn test_frame_metrics_creation() {
    let metrics = FrameMetrics {
        timestamp: 1000.0,
        duration: 16.67,
        animations_updated: 5,
        memory_usage: 1024,
        dropped: false,
    };
    
    assert_eq!(metrics.timestamp, 1000.0);
    assert_eq!(metrics.duration, 16.67);
    assert_eq!(metrics.animations_updated, 5);
    assert_eq!(metrics.memory_usage, 1024);
    assert_eq!(metrics.dropped, false);
}

#[test]
fn test_frame_metrics_clone() {
    let metrics = FrameMetrics {
        timestamp: 1000.0,
        duration: 16.67,
        animations_updated: 5,
        memory_usage: 1024,
        dropped: false,
    };
    
    let cloned = metrics.clone();
    assert_eq!(metrics.timestamp, cloned.timestamp);
    assert_eq!(metrics.duration, cloned.duration);
    assert_eq!(metrics.animations_updated, cloned.animations_updated);
    assert_eq!(metrics.memory_usage, cloned.memory_usage);
    assert_eq!(metrics.dropped, cloned.dropped);
}

#[test]
fn test_frame_metrics_debug() {
    let metrics = FrameMetrics {
        timestamp: 1000.0,
        duration: 16.67,
        animations_updated: 5,
        memory_usage: 1024,
        dropped: false,
    };
    
    let debug_str = format!("{:?}", metrics);
    
    assert!(debug_str.contains("FrameMetrics"));
    assert!(debug_str.contains("1000.0"));
    assert!(debug_str.contains("16.67"));
    assert!(debug_str.contains("5"));
    assert!(debug_str.contains("1024"));
    assert!(debug_str.contains("false"));
}

#[test]
fn test_animation_priority_ordering() {
    assert!(AnimationPriority::Critical > AnimationPriority::High);
    assert!(AnimationPriority::High > AnimationPriority::Normal);
    assert!(AnimationPriority::Normal > AnimationPriority::Low);
    
    let priorities = vec![
        AnimationPriority::Low,
        AnimationPriority::Critical,
        AnimationPriority::Normal,
        AnimationPriority::High,
    ];
    
    let mut sorted = priorities.clone();
    sorted.sort();
    
    assert_eq!(sorted, vec![
        AnimationPriority::Low,
        AnimationPriority::Normal,
        AnimationPriority::High,
        AnimationPriority::Critical,
    ]);
}

#[test]
fn test_animation_priority_variants() {
    let priorities = vec![
        AnimationPriority::Low,
        AnimationPriority::Normal,
        AnimationPriority::High,
        AnimationPriority::Critical,
    ];
    
    for priority in priorities {
        let debug_str = format!("{:?}", priority);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_animation_priority_clone() {
    let priority = AnimationPriority::High;
    let cloned = priority.clone();
    
    assert_eq!(priority, cloned);
}

#[test]
fn test_animation_priority_copy() {
    let priority = AnimationPriority::Critical;
    let copied = priority;
    
    assert_eq!(priority, copied);
}

#[test]
fn test_animation_priority_equality() {
    let priority1 = AnimationPriority::High;
    let priority2 = AnimationPriority::High;
    let priority3 = AnimationPriority::Low;
    
    assert_eq!(priority1, priority2);
    assert_ne!(priority1, priority3);
}

#[test]
fn test_animation_priority_debug() {
    let priorities = vec![
        AnimationPriority::Low,
        AnimationPriority::Normal,
        AnimationPriority::High,
        AnimationPriority::Critical,
    ];
    
    for priority in priorities {
        let debug_str = format!("{:?}", priority);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_performance_budget_edge_cases() {
    let budget = PerformanceBudget {
        max_frame_time: 0.0,
        max_concurrent_animations: 0,
        max_memory_usage: 0,
        max_animation_duration: 0.0,
    };
    
    assert_eq!(budget.max_frame_time, 0.0);
    assert_eq!(budget.max_concurrent_animations, 0);
    assert_eq!(budget.max_memory_usage, 0);
    assert_eq!(budget.max_animation_duration, 0.0);
}

#[test]
fn test_frame_metrics_edge_cases() {
    let metrics = FrameMetrics {
        timestamp: 0.0,
        duration: 0.0,
        animations_updated: 0,
        memory_usage: 0,
        dropped: true,
    };
    
    assert_eq!(metrics.timestamp, 0.0);
    assert_eq!(metrics.duration, 0.0);
    assert_eq!(metrics.animations_updated, 0);
    assert_eq!(metrics.memory_usage, 0);
    assert_eq!(metrics.dropped, true);
}

#[test]
fn test_animation_priority_edge_cases() {
    // Test that we can create and compare all priority levels
    let low = AnimationPriority::Low;
    let normal = AnimationPriority::Normal;
    let high = AnimationPriority::High;
    let critical = AnimationPriority::Critical;
    
    assert!(low < normal);
    assert!(normal < high);
    assert!(high < critical);
    
    assert_eq!(low as u8, 0);
    assert_eq!(normal as u8, 1);
    assert_eq!(high as u8, 2);
    assert_eq!(critical as u8, 3);
}

#[test]
fn test_performance_budget_memory_calculation() {
    let budget = PerformanceBudget::default();
    
    // Test that memory values are reasonable
    assert!(budget.max_memory_usage > 0);
    assert!(budget.max_memory_usage < 100 * 1024 * 1024); // Should be less than 100MB
    
    // Test that frame time is reasonable
    assert!(budget.max_frame_time > 0.0);
    assert!(budget.max_frame_time < 100.0); // Should be less than 100ms
    
    // Test that animation duration is reasonable
    assert!(budget.max_animation_duration > 0.0);
    assert!(budget.max_animation_duration < 60000.0); // Should be less than 1 minute
}

#[test]
fn test_frame_metrics_calculation() {
    let metrics = FrameMetrics {
        timestamp: 1000.0,
        duration: 16.67,
        animations_updated: 5,
        memory_usage: 1024,
        dropped: false,
    };
    
    // Test that values are reasonable
    assert!(metrics.timestamp >= 0.0);
    assert!(metrics.duration >= 0.0);
    assert!(metrics.animations_updated >= 0);
    assert!(metrics.memory_usage >= 0);
    
    // Test that timestamp is reasonable (should be in milliseconds or seconds)
    assert!(metrics.timestamp < 1000000000.0); // Should be less than 1 billion
    
    // Test that duration is reasonable (should be in milliseconds)
    assert!(metrics.duration < 1000.0); // Should be less than 1 second
}

#[test]
fn test_animation_priority_serialization() {
    // Test that priority values can be converted to integers
    let priorities = vec![
        AnimationPriority::Low,
        AnimationPriority::Normal,
        AnimationPriority::High,
        AnimationPriority::Critical,
    ];
    
    for (i, priority) in priorities.iter().enumerate() {
        let value = *priority as u8;
        assert_eq!(value, i as u8);
    }
}
