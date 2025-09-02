//! Performance optimization benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_motion_core::*;
use leptos_motion_dom::performance::*;
use std::time::{Duration, Instant};

/// Benchmark animation engine performance
fn benchmark_animation_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("Animation Engine");
    
    group.bench_function("optimized_hybrid_engine_100_animations", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            
            // Create 100 animations
            for i in 0..100 {
                let config = AnimationConfig {
                    element: create_test_element(),
                    from: AnimationTarget::new(),
                    to: AnimationTarget::new(),
                    transition: Transition {
                        duration: Some(1.0),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    },
                    on_complete: None,
                    on_update: None,
                };
                
                let _handle = engine.animate(config).unwrap();
            }
            
            // Simulate 60 frames
            for frame in 0..60 {
                let timestamp = frame as f64 * 16.67; // 60fps
                engine.tick(timestamp).unwrap();
            }
            
            black_box(engine)
        });
    });
    
    group.bench_function("performance_monitor_overhead", |b| {
        b.iter(|| {
            let budget = PerformanceBudget::default();
            let mut monitor = PerformanceMonitor::new(budget).unwrap();
            
            for _ in 0..100 {
                monitor.start_frame();
                // Simulate work
                std::thread::sleep(Duration::from_micros(100));
                monitor.end_frame(10, 1024);
            }
            
            let report = monitor.get_report();
            black_box(report)
        });
    });
    
    group.finish();
}

/// Benchmark DOM optimization performance
fn benchmark_dom_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("DOM Optimizations");
    
    group.bench_function("css_optimizer_batch_100", |b| {
        b.iter(|| {
            let mut optimizer = CSSOptimizer::new();
            let element = create_test_element();
            
            let properties = vec![
                "transform".to_string(),
                "opacity".to_string(),
                "filter".to_string(),
            ];
            
            for _ in 0..100 {
                optimizer.optimize_element(&element, &properties).unwrap();
            }
            
            black_box(optimizer)
        });
    });
    
    group.bench_function("dom_batcher_1000_updates", |b| {
        b.iter(|| {
            let mut batcher = DOMBatcher::new(100, 16.0);
            let element = create_test_element();
            
            // Queue 1000 updates
            for i in 0..1000 {
                batcher.queue_update(
                    element.clone(),
                    "transform".to_string(),
                    format!("translateX({}px)", i),
                    UpdatePriority::Normal,
                );
            }
            
            // Process all updates
            let processed = batcher.flush().unwrap();
            black_box(processed)
        });
    });
    
    group.bench_function("element_cache_100_lookups", |b| {
        b.iter(|| {
            let mut cache = ElementCache::new(50);
            
            // Create test elements
            let document = web_sys::window().unwrap().document().unwrap();
            for i in 0..10 {
                let element = document.create_element("div").unwrap();
                element.set_id(&format!("test-element-{}", i));
                document.body().unwrap().append_child(&element).unwrap();
            }
            
            // Perform 100 lookups
            for _ in 0..100 {
                for i in 0..10 {
                    let selector = format!("#test-element-{}", i);
                    let _element = cache.get(&selector);
                }
            }
            
            // Clean up
            for i in 0..10 {
                let selector = format!("#test-element-{}", i);
                if let Some(element) = document.query_selector(&selector).unwrap() {
                    document.body().unwrap().remove_child(&element).unwrap();
                }
            }
            
            black_box(cache.size())
        });
    });
    
    group.finish();
}

/// Benchmark memory usage and garbage collection
fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Usage");
    
    group.bench_function("animation_pool_1000_allocations", |b| {
        b.iter(|| {
            let mut pool = AnimationPool::<String>::new();
            
            // Allocate 1000 animations
            for i in 0..1000 {
                let handle = AnimationHandle(i);
                let animation = pool.acquire(handle, || format!("animation-{}", i));
                *animation = format!("active-{}", i);
            }
            
            // Release all animations
            for i in 0..1000 {
                let handle = AnimationHandle(i);
                pool.release(handle);
            }
            
            black_box((pool.active_count(), pool.available_count()))
        });
    });
    
    group.bench_function("gpu_layer_manager_100_elements", |b| {
        b.iter(|| {
            let mut manager = GPULayerManager::new(100);
            
            // Promote 100 elements
            for i in 0..100 {
                let element_id = format!("element-{}", i);
                manager.promote_to_gpu(&element_id);
            }
            
            // Demote all elements
            for i in 0..100 {
                let element_id = format!("element-{}", i);
                manager.demote_from_gpu(&element_id);
            }
            
            black_box(manager.layer_count())
        });
    });
    
    group.finish();
}

/// Benchmark concurrent animation performance
fn benchmark_concurrent_animations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Concurrent Animations");
    
    group.bench_function("100_concurrent_spring_animations", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            let mut handles = Vec::new();
            
            // Start 100 spring animations
            for i in 0..100 {
                let config = AnimationConfig {
                    element: create_test_element(),
                    from: AnimationTarget::new(),
                    to: AnimationTarget::new(),
                    transition: Transition {
                        duration: Some(2.0),
                        ease: Easing::Spring(SpringConfig {
                            stiffness: 100.0,
                            damping: 10.0,
                            mass: 1.0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    on_complete: None,
                    on_update: None,
                };
                
                let handle = engine.animate(config).unwrap();
                handles.push(handle);
            }
            
            // Run for 2 seconds at 60fps
            for frame in 0..120 {
                let timestamp = frame as f64 * 16.67;
                engine.tick(timestamp).unwrap();
            }
            
            black_box(handles.len())
        });
    });
    
    group.bench_function("mixed_animation_types_50_each", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            let mut handles = Vec::new();
            
            // 50 linear animations
            for i in 0..50 {
                let config = AnimationConfig {
                    element: create_test_element(),
                    from: AnimationTarget::new(),
                    to: AnimationTarget::new(),
                    transition: Transition {
                        duration: Some(1.0),
                        ease: Easing::Linear,
                        ..Default::default()
                    },
                    on_complete: None,
                    on_update: None,
                };
                
                let handle = engine.animate(config).unwrap();
                handles.push(handle);
            }
            
            // 50 spring animations
            for i in 0..50 {
                let config = AnimationConfig {
                    element: create_test_element(),
                    from: AnimationTarget::new(),
                    to: AnimationTarget::new(),
                    transition: Transition {
                        duration: Some(1.0),
                        ease: Easing::Spring(SpringConfig::default()),
                        ..Default::default()
                    },
                    on_complete: None,
                    on_update: None,
                };
                
                let handle = engine.animate(config).unwrap();
                handles.push(handle);
            }
            
            // Run for 1 second at 60fps
            for frame in 0..60 {
                let timestamp = frame as f64 * 16.67;
                engine.tick(timestamp).unwrap();
            }
            
            black_box(handles.len())
        });
    });
    
    group.finish();
}

/// Benchmark frame rate consistency
fn benchmark_frame_rate_consistency(c: &mut Criterion) {
    let mut group = c.benchmark_group("Frame Rate Consistency");
    
    group.bench_function("maintain_60fps_under_load", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            let mut frame_times = Vec::new();
            
            // Create heavy load
            for i in 0..200 {
                let config = AnimationConfig {
                    element: create_test_element(),
                    from: AnimationTarget::new(),
                    to: AnimationTarget::new(),
                    transition: Transition {
                        duration: Some(3.0),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    },
                    on_complete: None,
                    on_update: None,
                };
                
                engine.animate(config).unwrap();
            }
            
            // Measure frame times
            for frame in 0..180 { // 3 seconds at 60fps
                let start = Instant::now();
                let timestamp = frame as f64 * 16.67;
                engine.tick(timestamp).unwrap();
                let frame_time = start.elapsed();
                frame_times.push(frame_time);
            }
            
            // Calculate statistics
            let avg_frame_time: Duration = frame_times.iter().sum::<Duration>() / frame_times.len() as u32;
            let max_frame_time = frame_times.iter().max().unwrap();
            
            black_box((avg_frame_time, *max_frame_time))
        });
    });
    
    group.finish();
}

/// Benchmark startup performance
fn benchmark_startup_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Startup Performance");
    
    group.bench_function("engine_initialization", |b| {
        b.iter(|| {
            let start = Instant::now();
            let engine = OptimizedHybridEngine::new();
            let init_time = start.elapsed();
            black_box((engine, init_time))
        });
    });
    
    group.bench_function("first_animation_startup", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            
            let start = Instant::now();
            let config = AnimationConfig {
                element: create_test_element(),
                from: AnimationTarget::new(),
                to: AnimationTarget::new(),
                transition: Transition::default(),
                on_complete: None,
                on_update: None,
            };
            
            let handle = engine.animate(config).unwrap();
            let startup_time = start.elapsed();
            
            black_box((handle, startup_time))
        });
    });
    
    group.finish();
}

/// Helper function to create test elements
fn create_test_element() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_element("div").unwrap()
}

criterion_group!(
    benches,
    benchmark_animation_engine,
    benchmark_dom_optimizations,
    benchmark_memory_usage,
    benchmark_concurrent_animations,
    benchmark_frame_rate_consistency,
    benchmark_startup_performance,
);

criterion_main!(benches);
