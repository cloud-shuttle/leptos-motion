// Simple Performance Benchmarks for Leptos Motion
//
// This module provides basic performance testing using Criterion
// to ensure the animation system maintains high performance standards.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leptos_motion_core::*;
use leptos_motion_dom::*;
use leptos_motion_gestures::*;
use std::collections::HashMap;
use std::time::Duration;

// Benchmark MotionValue operations
fn bench_motion_value_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_value_creation");
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("create_motion_value", |b| {
        b.iter(|| black_box(MotionValue::new(42.0)))
    });

    group.bench_function("create_motion_number", |b| {
        b.iter(|| black_box(MotionNumber::new(100.0)))
    });

    group.finish();
}

fn bench_motion_value_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("motion_value_operations");
    group.measurement_time(Duration::from_secs(3));

    let motion_value = MotionValue::new(0.0);

    group.bench_function("get_value", |b| b.iter(|| black_box(motion_value.get())));

    group.bench_function("set_value", |b| {
        b.iter(|| motion_value.set(black_box(42.0)))
    });

    group.finish();
}

// Benchmark gesture detection
fn bench_gesture_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("gesture_detection");
    group.measurement_time(Duration::from_secs(3));

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

    group.finish();
}

// Benchmark animation target operations
fn bench_animation_target_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_target_operations");
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("create_animation_target", |b| {
        b.iter(|| {
            let mut target = HashMap::new();
            target.insert("x".to_string(), AnimationValue::Pixels(100.0));
            target.insert("y".to_string(), AnimationValue::Pixels(200.0));
            target.insert("scale".to_string(), AnimationValue::Number(1.5));
            black_box(target)
        })
    });

    group.finish();
}

// Benchmark transition operations
fn bench_transition_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_operations");
    group.measurement_time(Duration::from_secs(3));

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

// Custom benchmark configuration
criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(3))
        .warm_up_time(Duration::from_secs(1))
        .sample_size(20);
    targets =
        bench_motion_value_creation,
        bench_motion_value_operations,
        bench_gesture_detection,
        bench_animation_target_operations,
        bench_transition_operations
);

criterion_main!(benches);
