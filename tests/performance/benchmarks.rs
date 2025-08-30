use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_motion_core::*;
use std::time::Instant;

fn benchmark_spring_physics(c: &mut Criterion) {
    c.bench_function("spring_physics_100_steps", |b| {
        b.iter(|| {
            let config = SpringConfig {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                ..Default::default()
            };
            
            let mut simulator = SpringSimulator::new(config);
            let mut state = SpringState {
                position: 0.0,
                velocity: 0.0,
            };
            
            for _ in 0..100 {
                state = simulator.step(state, 100.0, 1.0 / 60.0);
            }
            
            black_box(state)
        });
    });
}

fn benchmark_interpolation(c: &mut Criterion) {
    c.bench_function("interpolation_1000_values", |b| {
        b.iter(|| {
            let from = AnimationValue::Number(0.0);
            let to = AnimationValue::Number(100.0);
            
            for i in 0..1000 {
                let progress = i as f64 / 1000.0;
                let _result = from.interpolate(&to, progress);
            }
        });
    });
}

fn benchmark_motion_value_operations(c: &mut Criterion) {
    c.bench_function("motion_value_set_get_1000", |b| {
        b.iter(|| {
            let motion_value = MotionValue::new(0.0);
            
            for i in 0..1000 {
                motion_value.set(i as f64);
                let _value = motion_value.get();
            }
        });
    });
}

fn benchmark_motion_value_subscriptions(c: &mut Criterion) {
    c.bench_function("motion_value_subscriptions_100", |b| {
        b.iter(|| {
            let motion_value = MotionValue::new(0.0);
            let mut callbacks = Vec::new();
            
            // Add 100 subscribers
            for _ in 0..100 {
                let callback = Box::new(|_value: &f64| {
                    // Simulate callback work
                    black_box(1);
                });
                motion_value.subscribe(callback);
            }
            
            // Trigger updates
            for i in 0..10 {
                motion_value.set(i as f64);
            }
        });
    });
}

fn benchmark_easing_functions(c: &mut Criterion) {
    let easing_functions = vec![
        ("linear", Easing::Linear),
        ("ease_in", Easing::EaseIn),
        ("ease_out", Easing::EaseOut),
        ("ease_in_out", Easing::EaseInOut),
        ("back_in", Easing::BackIn),
        ("back_out", Easing::BackOut),
        ("back_in_out", Easing::BackInOut),
    ];
    
    for (name, easing) in easing_functions {
        c.bench_function(&format!("easing_{}_1000", name), |b| {
            b.iter(|| {
                for i in 0..1000 {
                    let progress = i as f64 / 1000.0;
                    let _result = easing.evaluate(progress);
                }
            });
        });
    }
}

fn benchmark_animation_target_operations(c: &mut Criterion) {
    c.bench_function("animation_target_insert_get_100", |b| {
        b.iter(|| {
            let mut target = AnimationTarget::new();
            
            for i in 0..100 {
                let key = format!("property_{}", i);
                let value = AnimationValue::Number(i as f64);
                target.insert(key.clone(), value);
                
                let _retrieved = target.get(&key);
            }
        });
    });
}

fn benchmark_concurrent_animations(c: &mut Criterion) {
    c.bench_function("concurrent_animations_50", |b| {
        b.iter(|| {
            let mut motion_values = Vec::new();
            
            // Create 50 motion values
            for i in 0..50 {
                let motion_value = MotionValue::new(0.0);
                motion_values.push(motion_value);
            }
            
            // Update all values
            for (i, motion_value) in motion_values.iter().enumerate() {
                motion_value.set(i as f64);
            }
            
            // Read all values
            for motion_value in &motion_values {
                let _value = motion_value.get();
            }
        });
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_usage_1000_motion_values", |b| {
        b.iter(|| {
            let mut motion_values = Vec::new();
            
            // Create 1000 motion values
            for i in 0..1000 {
                let motion_value = MotionValue::new(i as f64);
                motion_values.push(motion_value);
            }
            
            // Perform operations
            for motion_value in &motion_values {
                motion_value.set(motion_value.get() + 1.0);
            }
            
            // Drop all motion values (memory cleanup)
            drop(motion_values);
        });
    });
}

fn benchmark_frame_rate_simulation(c: &mut Criterion) {
    c.bench_function("frame_rate_60fps_simulation", |b| {
        b.iter(|| {
            let frame_time = 1.0 / 60.0; // 60 FPS
            let duration = 1.0; // 1 second
            let frames = (duration / frame_time) as usize;
            
            let mut motion_value = MotionValue::new(0.0);
            let target = 100.0;
            
            for _ in 0..frames {
                // Simulate animation frame
                let current = motion_value.get();
                let velocity = motion_value.get_velocity();
                
                // Simple physics update
                let new_position = current + velocity * frame_time;
                motion_value.set(new_position);
                
                // Update velocity towards target
                let error = target - new_position;
                let new_velocity = velocity + error * frame_time * 10.0; // Spring-like
                motion_value.set_velocity(new_velocity);
            }
        });
    });
}

fn benchmark_transform_operations(c: &mut Criterion) {
    c.bench_function("transform_compose_1000", |b| {
        b.iter(|| {
            let base_transform = Transform {
                x: 0.0,
                y: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                rotation: 0.0,
                skew_x: 0.0,
                skew_y: 0.0,
            };
            
            for i in 0..1000 {
                let delta_transform = Transform {
                    x: i as f64 * 0.1,
                    y: i as f64 * 0.05,
                    scale_x: 1.0 + (i as f64 * 0.001),
                    scale_y: 1.0 + (i as f64 * 0.001),
                    rotation: i as f64 * 0.1,
                    skew_x: 0.0,
                    skew_y: 0.0,
                };
                
                let _composed = base_transform.compose(&delta_transform);
            }
        });
    });
}

fn benchmark_motion_values_collection(c: &mut Criterion) {
    c.bench_function("motion_values_collection_100", |b| {
        b.iter(|| {
            let mut motion_values = MotionValues::new();
            
            // Add 100 motion values
            for i in 0..100 {
                let key = format!("property_{}", i);
                let motion_value = MotionValue::new(i as f64);
                motion_values.add(&key, motion_value);
            }
            
            // Update all values
            for i in 0..100 {
                let key = format!("property_{}", i);
                if let Some(motion_value) = motion_values.get(&key) {
                    motion_value.set(i as f64 * 2.0);
                }
            }
            
            // Iterate over all values
            for key in motion_values.keys() {
                if let Some(motion_value) = motion_values.get(key) {
                    let _value = motion_value.get();
                }
            }
        });
    });
}

criterion_group!(
    benches,
    benchmark_spring_physics,
    benchmark_interpolation,
    benchmark_motion_value_operations,
    benchmark_motion_value_subscriptions,
    benchmark_easing_functions,
    benchmark_animation_target_operations,
    benchmark_concurrent_animations,
    benchmark_memory_usage,
    benchmark_frame_rate_simulation,
    benchmark_transform_operations,
    benchmark_motion_values_collection,
);

criterion_main!(benches);
