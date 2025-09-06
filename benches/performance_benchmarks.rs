// Performance Benchmarks for Leptos Motion
//
// This module provides comprehensive performance testing using Criterion
// to ensure the animation system maintains high performance standards.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use leptos_motion_core::*;
use leptos_motion_dom::*;
use leptos_motion_gestures::*;
use std::collections::HashMap;
use std::time::Duration;

// Benchmark MotionValue operations
fn bench_motion_value_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_value_creation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("create_motion_value", |b| {
        b.iter(|| black_box(MotionValue::new(42.0)))
    });

    group.bench_function("create_motion_number", |b| {
        b.iter(|| black_box(MotionNumber::new(100.0)))
    });

    group.bench_function("create_motion_transform", |b| {
        b.iter(|| black_box(MotionTransform::identity()))
    });

    group.finish();
}

fn bench_motion_value_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_value_operations");
    group.measurement_time(Duration::from_secs(10));

    let motion_value = MotionValue::new(0.0);

    group.bench_function("get_value", |b| b.iter(|| black_box(motion_value.get())));

    group.bench_function("set_value", |b| {
        b.iter(|| motion_value.set(black_box(42.0)))
    });

    group.bench_function("get_velocity", |b| {
        b.iter(|| black_box(motion_value.get_velocity()))
    });

    group.finish();
}

fn bench_motion_values_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_values_operations");
    group.measurement_time(Duration::from_secs(10));

    let motion_values = MotionValues::new();

    group.bench_function("add_value", |b| {
        b.iter(|| motion_values.add("test_key".to_string(), MotionValue::new(black_box(42.0))))
    });

    group.bench_function("get_value", |b| {
        motion_values.add("test_key".to_string(), MotionValue::new(42.0));
        b.iter(|| black_box(motion_values.get("test_key")))
    });

    group.bench_function("set_value", |b| {
        motion_values.add("test_key".to_string(), MotionValue::new(0.0));
        b.iter(|| motion_values.set("test_key", MotionValue::new(black_box(42.0))))
    });

    group.finish();
}

// Benchmark gesture detection
fn bench_gesture_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("gesture_detection");
    group.measurement_time(Duration::from_secs(10));

    let mut detector = MultiTouchGestureDetector::default();

    let touches = vec![
        TouchPoint {
            id: 1,
            x: 100.0,
            y: 100.0,
            pressure: 1.0,
            timestamp: 1000,
        },
        TouchPoint {
            id: 2,
            x: 200.0,
            y: 200.0,
            pressure: 1.0,
            timestamp: 1000,
        },
    ];

    group.bench_function("handle_touch_start", |b| {
        b.iter(|| black_box(detector.handle_touch_start(touches.clone())))
    });

    group.bench_function("handle_touch_move", |b| {
        detector.handle_touch_start(touches.clone());
        b.iter(|| black_box(detector.handle_touch_move(touches.clone())))
    });

    group.finish();
}

// Benchmark animation target operations
fn bench_animation_target_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_target_operations");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("create_animation_target", |b| {
        b.iter(|| {
            let mut target = HashMap::new();
            target.insert("x".to_string(), AnimationValue::Pixels(100.0));
            target.insert("y".to_string(), AnimationValue::Pixels(200.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.5));
            target.insert("rotate".to_string(), AnimationValue::Degrees(45.0));
            target.insert("opacity".to_string(), AnimationValue::Number(0.8));
            black_box(target)
        })
    });

    group.bench_function("animation_value_to_string", |b| {
        let values = vec![
            AnimationValue::Pixels(100.0),
            AnimationValue::Number(1.5),
            AnimationValue::Degrees(45.0),
            AnimationValue::Color("#ff0000".to_string()),
        ];

        b.iter(|| {
            for value in &values {
                black_box(value.to_string());
            }
        })
    });

    group.finish();
}

// Benchmark transition operations
fn bench_transition_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_operations");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("create_transition", |b| {
        b.iter(|| {
            black_box(Transition {
                duration: Some(0.5),
                delay: Some(0.1),
                ease: Easing::EaseInOut,
                ..Default::default()
            })
        })
    });

    group.finish();
}

// Benchmark motion props operations
fn bench_motion_props_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_props_operations");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("create_motion_props", |b| {
        b.iter(|| {
            let mut initial = HashMap::new();
            initial.insert("x".to_string(), AnimationValue::Pixels(0.0));
            initial.insert("y".to_string(), AnimationValue::Pixels(0.0));

            let mut animate = HashMap::new();
            animate.insert("x".to_string(), AnimationValue::Pixels(100.0));
            animate.insert("y".to_string(), AnimationValue::Pixels(200.0));

            black_box(MotionProps {
                initial: Some(initial),
                animate: Some(animate),
                exit: None,
                transition: Some(Transition {
                    duration: Some(0.5),
                    delay: Some(0.1),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }),
                variants: None,
                layout: Some(true),
                drag: None,
                while_hover: None,
                while_tap: None,
                while_focus: None,
                while_in_view: None,
                event_handlers: None,
            })
        })
    });

    group.finish();
}

// Benchmark with different data sizes
fn bench_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");
    group.measurement_time(Duration::from_secs(10));

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("motion_values_operations", size),
            size,
            |b, &size| {
                let motion_values = MotionValues::new();

                // Pre-populate with data
                for i in 0..size {
                    motion_values.add(format!("key_{}", i), MotionValue::new(i as f64));
                }

                b.iter(|| {
                    // Test operations on the populated data
                    for i in 0..size {
                        let key = format!("key_{}", i);
                        let _ = motion_values.get(&key);
                    }
                })
            },
        );
    }

    group.finish();
}

// Benchmark memory usage patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("create_and_drop_motion_values", |b| {
        b.iter(|| {
            let mut values = Vec::new();
            for i in 0..1000 {
                values.push(MotionValue::new(i as f64));
            }
            black_box(values);
            // Values are dropped here
        })
    });

    group.bench_function("reuse_motion_values", |b| {
        let mut values = Vec::new();
        for i in 0..1000 {
            values.push(MotionValue::new(i as f64));
        }

        b.iter(|| {
            for value in &values {
                black_box(value.get());
            }
        })
    });

    group.finish();
}

// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("parallel_motion_value_creation", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    std::thread::spawn(move || {
                        let mut values = Vec::new();
                        for j in 0..250 {
                            values.push(MotionValue::new((i * 250 + j) as f64));
                        }
                        values
                    })
                })
                .collect();

            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            black_box(results);
        })
    });

    group.finish();
}

// Benchmark real-world scenarios
fn bench_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_scenarios");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("animation_sequence", |b| {
        let motion_value = MotionValue::new(0.0);

        b.iter(|| {
            // Simulate a complete animation sequence
            motion_value.set(0.0);
            for i in 1..=100 {
                motion_value.set(i as f64);
                black_box(motion_value.get());
            }
        })
    });

    group.bench_function("gesture_sequence", |b| {
        let mut detector = MultiTouchGestureDetector::default();

        b.iter(|| {
            // Simulate a complete gesture sequence
            let start_touches = vec![
                TouchPoint {
                    id: 1,
                    x: 100.0,
                    y: 100.0,
                    pressure: 1.0,
                    timestamp: 1000,
                },
                TouchPoint {
                    id: 2,
                    x: 200.0,
                    y: 200.0,
                    pressure: 1.0,
                    timestamp: 1000,
                },
            ];

            let _ = detector.handle_touch_start(start_touches);

            for i in 1..=10 {
                let move_touches = vec![
                    TouchPoint {
                        id: 1,
                        x: 100.0 + (i as f64 * 10.0),
                        y: 100.0 + (i as f64 * 10.0),
                        pressure: 1.0,
                        timestamp: 1000 + i,
                    },
                    TouchPoint {
                        id: 2,
                        x: 200.0 + (i as f64 * 10.0),
                        y: 200.0 + (i as f64 * 10.0),
                        pressure: 1.0,
                        timestamp: 1000 + i,
                    },
                ];

                let _ = detector.handle_touch_move(move_touches);
            }
        })
    });

    group.finish();
}

// Custom benchmark configuration
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3))
        .sample_size(100);
    targets =
        bench_motion_value_creation,
        bench_motion_value_operations,
        bench_motion_values_operations,
        bench_gesture_detection,
        bench_animation_target_operations,
        bench_transition_operations,
        bench_motion_props_operations,
        bench_scalability,
        bench_memory_patterns,
        bench_concurrent_operations,
        bench_real_world_scenarios
);

criterion_main!(benches);
