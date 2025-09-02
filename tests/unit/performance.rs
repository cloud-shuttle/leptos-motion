//! Unit tests for performance optimization system

use leptos_motion_core::performance::*;
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
fn test_performance_monitor_creation() {
    let budget = PerformanceBudget::default();
    let monitor = PerformanceMonitor::new(budget);
    
    assert!(monitor.is_some());
}

#[test]
fn test_performance_monitor_frame_tracking() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    monitor.start_frame();
    std::thread::sleep(Duration::from_millis(16));
    let metrics = monitor.end_frame(10, 1024);
    
    assert!(metrics.duration > 0.0);
    assert_eq!(metrics.animations_updated, 10);
    assert_eq!(metrics.memory_usage, 1024);
    assert!(!metrics.dropped); // Should not drop at 16ms
}

#[test]
fn test_performance_monitor_fps_calculation() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Simulate 60fps for 1 second
    for _ in 0..60 {
        monitor.start_frame();
        std::thread::sleep(Duration::from_millis(16));
        monitor.end_frame(1, 100);
    }
    
    let fps = monitor.get_fps();
    assert!(fps > 50.0 && fps < 70.0); // Should be around 60fps
}

#[test]
fn test_performance_monitor_frame_drop_detection() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Simulate frame drops
    for i in 0..10 {
        monitor.start_frame();
        if i % 3 == 0 {
            // Simulate slow frame
            std::thread::sleep(Duration::from_millis(50));
        } else {
            std::thread::sleep(Duration::from_millis(16));
        }
        monitor.end_frame(1, 100);
    }
    
    let drop_rate = monitor.get_frame_drop_rate();
    assert!(drop_rate > 0.0); // Should detect some frame drops
}

#[test]
fn test_performance_monitor_budget_compliance() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Simulate good performance
    for _ in 0..30 {
        monitor.start_frame();
        std::thread::sleep(Duration::from_millis(16));
        monitor.end_frame(1, 100);
    }
    
    assert!(monitor.is_within_budget());
}

#[test]
fn test_animation_scheduler_creation() {
    let frame_budget = Duration::from_millis(16);
    let scheduler = AnimationScheduler::new(frame_budget);
    
    assert_eq!(scheduler.pending_animations.len(), 0);
    assert_eq!(scheduler.active_animations.len(), 0);
}

#[test]
fn test_animation_scheduler_priority_ordering() {
    let frame_budget = Duration::from_millis(16);
    let mut scheduler = AnimationScheduler::new(frame_budget);
    
    // Create test config
    let config = AnimationConfig {
        element: web_sys::Element::new("div").unwrap(),
        from: AnimationTarget::new(),
        to: AnimationTarget::new(),
        transition: Transition::default(),
        on_complete_id: None,
        on_update_id: None,
    };
    
    // Schedule animations with different priorities
    let handle1 = scheduler.schedule(config.clone(), AnimationPriority::Low);
    let handle2 = scheduler.schedule(config.clone(), AnimationPriority::High);
    let handle3 = scheduler.schedule(config.clone(), AnimationPriority::Normal);
    
    assert!(handle1.0 > 0);
    assert!(handle2.0 > 0);
    assert!(handle3.0 > 0);
    assert_eq!(scheduler.pending_animations.len(), 3);
}

#[test]
fn test_gpu_layer_manager() {
    let mut manager = GPULayerManager::new(10);
    
    // Test promotion
    assert!(manager.promote_to_gpu("element1"));
    assert!(manager.is_promoted("element1"));
    assert_eq!(manager.layer_count(), 1);
    
    // Test duplicate promotion
    assert!(!manager.promote_to_gpu("element1"));
    assert_eq!(manager.layer_count(), 1);
    
    // Test demotion
    assert!(manager.demote_from_gpu("element1"));
    assert!(!manager.is_promoted("element1"));
    assert_eq!(manager.layer_count(), 0);
    
    // Test demotion of non-promoted element
    assert!(!manager.demote_from_gpu("element2"));
}

#[test]
fn test_gpu_layer_manager_capacity() {
    let mut manager = GPULayerManager::new(2);
    
    // Fill to capacity
    assert!(manager.promote_to_gpu("element1"));
    assert!(manager.promote_to_gpu("element2"));
    
    // Try to exceed capacity
    assert!(!manager.promote_to_gpu("element3"));
    assert_eq!(manager.layer_count(), 2);
}

#[test]
fn test_animation_pool() {
    let mut pool = AnimationPool::<String>::new();
    
    // Test acquisition
    let handle = AnimationHandle(1);
    let animation = pool.acquire(handle, || "new_animation".to_string());
    assert_eq!(animation, "new_animation");
    assert_eq!(pool.active_count(), 1);
    assert_eq!(pool.available_count(), 0);
    
    // Test release
    pool.release(handle);
    assert_eq!(pool.active_count(), 0);
    assert_eq!(pool.available_count(), 1);
    
    // Test reuse
    let handle2 = AnimationHandle(2);
    let animation2 = pool.acquire(handle2, || "should_not_create".to_string());
    assert_eq!(animation2, "new_animation"); // Should reuse from pool
    assert_eq!(pool.active_count(), 1);
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_animation_pool_multiple_allocations() {
    let mut pool = AnimationPool::<String>::new();
    
    // Allocate multiple animations
    for i in 0..5 {
        let handle = AnimationHandle(i);
        let animation = pool.acquire(handle, || format!("animation_{}", i));
        assert_eq!(animation, format!("animation_{}", i));
    }
    
    assert_eq!(pool.active_count(), 5);
    
    // Release all
    for i in 0..5 {
        let handle = AnimationHandle(i);
        pool.release(handle);
    }
    
    assert_eq!(pool.active_count(), 0);
    assert_eq!(pool.available_count(), 5);
}

#[test]
fn test_performance_report_generation() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Simulate some frames
    for _ in 0..10 {
        monitor.start_frame();
        std::thread::sleep(Duration::from_millis(16));
        monitor.end_frame(5, 512);
    }
    
    let report = monitor.get_report();
    
    assert!(report.fps > 0.0);
    assert!(report.avg_frame_time > 0.0);
    assert!(report.frame_drop_rate >= 0.0);
    assert_eq!(report.total_frames, 10);
    assert!(report.dropped_frames >= 0);
    assert!(report.within_budget);
}

#[test]
fn test_performance_monitor_metrics_history() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Add more frames than the history capacity
    for _ in 0..100 {
        monitor.start_frame();
        std::thread::sleep(Duration::from_millis(16));
        monitor.end_frame(1, 100);
    }
    
    // Should only keep the last 60 frames (max_samples)
    let fps = monitor.get_fps();
    assert!(fps > 0.0); // Should still calculate FPS correctly
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
fn test_performance_monitor_edge_cases() {
    let budget = PerformanceBudget::default();
    let mut monitor = PerformanceMonitor::new(budget).unwrap();
    
    // Test with no frames
    assert_eq!(monitor.get_fps(), 0.0);
    assert_eq!(monitor.get_avg_frame_time(), 0.0);
    assert_eq!(monitor.get_frame_drop_rate(), 0.0);
    
    // Test with single frame
    monitor.start_frame();
    std::thread::sleep(Duration::from_millis(16));
    monitor.end_frame(1, 100);
    
    assert_eq!(monitor.get_fps(), 0.0); // Need at least 2 frames for FPS
    assert!(monitor.get_avg_frame_time() > 0.0);
    assert_eq!(monitor.get_frame_drop_rate(), 0.0);
}
