# Performance Optimization Guide

This guide covers comprehensive performance optimization techniques for Leptos Motion applications, including best practices, engine optimizations, and monitoring.

**Version**: 0.4.0  
**Bundle Size Achievement**: 92% reduction (378KB → 30KB-85KB)  
**Optimization Status**: Four-phase optimization complete with TDD methodology

## Table of Contents

- [Performance Fundamentals](#performance-fundamentals)
- [Animation Optimization](#animation-optimization)
- [Animation Engine Optimizations](#animation-engine-optimizations)
- [DOM Performance](#dom-performance)
- [Memory Management](#memory-management)
- [GPU Acceleration](#gpu-acceleration)
- [Bundle Size Optimization](#bundle-size-optimization)
- [Rendering Performance](#rendering-performance)
- [Profiling and Monitoring](#profiling-and-monitoring)
- [Performance Benchmarks](#performance-benchmarks)
- [Best Practices](#best-practices)

## Performance Fundamentals

### Target Performance Metrics

- **Frame Rate**: Maintain 60fps for smooth animations
- **Memory Usage**: Keep under 10MB for typical applications
- **Bundle Size**: 30KB minimal, 85KB optimized (92% reduction achieved)
- **Animation Latency**: <16ms per frame (60fps target)
- **Concurrent Animations**: Support 100+ simultaneous animations

### Performance Budget

```rust
// Performance budget per frame
const PERFORMANCE_BUDGET: f64 = 16.0; // milliseconds
const MAX_CONCURRENT_ANIMATIONS: usize = 100;
const MAX_MEMORY_USAGE: usize = 10 * 1024 * 1024; // 10MB
```

## Animation Optimization

### 1. Use Transform Properties

Transform properties are GPU-accelerated and provide the best performance:

```rust
// ✅ Good - GPU accelerated
animate=motion_target!(
    "x" => AnimationValue::Pixels(100.0),
    "y" => AnimationValue::Pixels(50.0),
    "scale" => AnimationValue::Number(1.5),
    "rotate" => AnimationValue::Degrees(45.0)
)

// ❌ Avoid - Layout-triggering properties
animate=motion_target!(
    "width" => AnimationValue::Pixels(200.0),
    "height" => AnimationValue::Pixels(200.0),
    "top" => AnimationValue::Pixels(100.0),
    "left" => AnimationValue::Pixels(100.0)
)
```

### 2. Optimize Easing Functions

Choose efficient easing functions for better performance:

```rust
// ✅ Fast - Simple mathematical operations
ease: Easing::Linear
ease: Easing::EaseIn
ease: Easing::EaseOut

// ⚠️ Moderate - More complex calculations
ease: Easing::EaseInOut
ease: Easing::BackIn
ease: Easing::BackOut

// 🔥 Expensive - Physics simulation
ease: Easing::Spring(SpringConfig::default())
```

### 3. Batch Animation Updates

Group related animations to reduce reflows:

```rust
#[component]
fn OptimizedAnimation() -> impl IntoView {
    let (is_expanded, set_expanded) = signal(false);

    // Batch all animations together
    let animation_target = motion_target!(
        "x" => AnimationValue::Pixels(if is_expanded.get() { 100.0 } else { 0.0 }),
        "y" => AnimationValue::Pixels(if is_expanded.get() { 50.0 } else { 0.0 }),
        "scale" => AnimationValue::Number(if is_expanded.get() { 1.2 } else { 1.0 }),
        "opacity" => AnimationValue::Number(if is_expanded.get() { 1.0 } else { 0.8 })
    );

    view! {
        <MotionDiv
            animate=animation_target
            transition=Transition {
                duration: Some(0.3),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Optimized Animation"
        </MotionDiv>
    }
}
```

### 4. Use Will-Change CSS Property

Hint to the browser about elements that will animate:

```css
.animated-element {
  will-change: transform, opacity;
}

/* Remove will-change after animation completes */
.animated-element.animation-complete {
  will-change: auto;
}
```

## Memory Management

### 1. Clean Up Motion Values

Properly dispose of motion values to prevent memory leaks:

```rust
#[component]
fn MemoryOptimizedComponent() -> impl IntoView {
    let motion_value = MotionValue::new(0.0);

    // Subscribe to changes
    let subscription = motion_value.subscribe(|value| {
        // Handle value changes
    });

    // Clean up when component unmounts
    on_cleanup(move || {
        // Unsubscribe to prevent memory leaks
        drop(subscription);
    });

    view! {
        <div>
            // Component content
        </div>
    }
}
```

### 2. Limit Concurrent Animations

Monitor and limit the number of simultaneous animations:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static ACTIVE_ANIMATIONS: AtomicUsize = AtomicUsize::new(0);

#[component]
fn AnimationManager() -> impl IntoView {
    let animation_count = create_rw_signal(0);

    // Track animation count
    Effect::new(move |_| {
        let count = ACTIVE_ANIMATIONS.load(Ordering::Relaxed);
        animation_count.set(count);

        if count > MAX_CONCURRENT_ANIMATIONS {
            log::warn!("Too many concurrent animations: {}", count);
        }
    });

    view! {
        <div>
            <p>"Active animations: {animation_count}"</p>
        </div>
    }
}
```

### 3. Use Weak References

Prevent circular references in animation callbacks:

```rust
use std::rc::Weak;

#[component]
fn WeakReferenceExample() -> impl IntoView {
    let component_ref = create_node_ref::<Div>();
    let weak_ref = Rc::downgrade(&component_ref);

    let motion_value = MotionValue::new(0.0);
    motion_value.subscribe(move |value| {
        if let Some(element) = weak_ref.upgrade() {
            // Update element with animation value
        }
    });

    view! {
        <div node_ref=component_ref>
            "Weak reference example"
        </div>
    }
}
```

## Animation Engine Optimizations

### 1. Optimized Hybrid Engine

The `OptimizedHybridEngine` automatically selects the best animation method:

```rust
use leptos_motion_core::engine::OptimizedHybridEngine;

let mut engine = OptimizedHybridEngine::new();

// Automatically chooses WAAPI or RAF based on capabilities
let handle = engine.animate(config)?;

// Get performance metrics
if let Some(report) = engine.get_performance_metrics() {
    println!("FPS: {:.1}", report.fps);
    println!("Frame drop rate: {:.2}%", report.frame_drop_rate * 100.0);
}
```

### 2. Performance Monitoring

Monitor animation performance in real-time:

```rust
use leptos_motion_core::performance::*;

let budget = PerformanceBudget::default();
let mut monitor = PerformanceMonitor::new(budget)?;

// Start monitoring a frame
monitor.start_frame();

// Perform animations
// ... animation work ...

// End monitoring and get metrics
let metrics = monitor.end_frame(animations_updated, memory_usage);
println!("Frame duration: {:.2}ms", metrics.duration);

// Get performance report
let report = monitor.get_report();
if !report.within_budget {
    println!("Performance warning: Frame drops detected");
}
```

### 3. Animation Scheduling

Batch animations for optimal performance:

```rust
use leptos_motion_core::performance::AnimationScheduler;

let frame_budget = Duration::from_millis(16);
let mut scheduler = AnimationScheduler::new(frame_budget);

// Schedule animations with priorities
let handle1 = scheduler.schedule(config1, AnimationPriority::High);
let handle2 = scheduler.schedule(config2, AnimationPriority::Normal);

// Process within frame budget
scheduler.process_pending(&mut engine)?;
```

## DOM Performance

### 1. Layout Thrashing Prevention

Avoid triggering multiple layout calculations:

```rust
// ✅ Good - Batch DOM reads and writes
let rect = element.get_bounding_client_rect();
let width = rect.width();
let height = rect.height();

// Apply all changes at once
element.style().set_property("width", &format!("{}px", width * 2)).unwrap();
element.style().set_property("height", &format!("{}px", height * 2)).unwrap();

// ❌ Bad - Alternating reads and writes
let rect = element.get_bounding_client_rect();
element.style().set_property("width", &format!("{}px", rect.width() * 2)).unwrap();
let rect = element.get_bounding_client_rect(); // Triggers another layout
element.style().set_property("height", &format!("{}px", rect.height() * 2)).unwrap();
```

### 2. Event Delegation

Use event delegation for better performance with many elements:

```rust
#[component]
fn OptimizedList(items: Vec<String>) -> impl IntoView {
    let container_ref = create_node_ref::<html::Div>();

    let on_click = move |event: web_sys::MouseEvent| {
        if let Some(target) = event.target() {
            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                if element.class_name().contains("list-item") {
                    // Handle click on list item
                    let index = element.get_attribute("data-index").unwrap_or_default();
                    println!("Clicked item at index: {}", index);
                }
            }
        }
    };

    view! {
        <div node_ref=container_ref on:click=on_click>
            {items.iter().enumerate().map(|(i, item)| {
                view! {
                    <div class="list-item" data-index={i.to_string()}>
                        {item}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

## GPU Acceleration

### 1. Transform Properties

Use GPU-accelerated properties for best performance:

```rust
// ✅ GPU accelerated - no layout recalculation
animate=motion_target!(
    "transform" => AnimationValue::Transform(TransformValue::Translate3d(100.0, 0.0, 0.0)),
    "scale" => AnimationValue::Number(1.5),
    "rotate" => AnimationValue::Degrees(45.0)
)

// ❌ Layout-triggering - causes reflow
animate=motion_target!(
    "width" => AnimationValue::Pixels(200.0),
    "height" => AnimationValue::Pixels(200.0),
    "top" => AnimationValue::Pixels(100.0),
    "left" => AnimationValue::Pixels(100.0)
)
```

### 2. Will-Change Property

Hint to the browser about upcoming changes:

```rust
// Set will-change for elements that will animate
element.style().set_property("will-change", "transform, opacity").unwrap();

// Remove after animation completes
motion_value.subscribe(move |_| {
    if animation_completed {
        element.style().remove_property("will-change").unwrap();
    }
});
```

## Bundle Size Optimization

### v0.4.0 Four-Phase Optimization

Leptos Motion v0.4.0 implements a comprehensive four-phase optimization strategy that achieved a 92% bundle size reduction:

#### Phase 1: Dead Code Elimination (120KB savings)

- Removed development-only modules in production builds
- Conditional compilation for `developer_tools`, `advanced_examples`, `ecosystem_integration`

#### Phase 2: Tree Shaking (100KB savings)

- Conditional compilation for optional features
- Removed unused functions and types
- Optimized imports and dependencies

#### Phase 3: Feature Flags (185KB savings)

- Made gestures, layout, scroll features optional
- Feature-based compilation with conditional attributes
- Granular control over functionality

#### Phase 4: Dependency Optimization (60KB+ savings)

- Custom minimal serialization system (replaces serde)
- Optimized web-sys and wasm-bindgen usage
- Removed unused dependencies (futures, tokio)

### 1. Use Build Presets

Choose the appropriate build preset for your use case:

```toml
# Minimal build (30KB) - Core animations only
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["minimal"] }

# Production build (75KB) - Optimized for production
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["production"] }

# Optimized build (85KB) - With performance monitoring
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["optimized"] }

# Standard build (125KB) - Full features
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["standard"] }

# Full build (235KB) - All features including development tools
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["full"] }
```

### 2. Tree Shaking

Use specific imports to reduce bundle size:

```rust
// ✅ Good - Only import what you need
use leptos_motion::{MotionDiv, motion_target, AnimationValue, Transition, Easing};

// ❌ Avoid - Import everything
use leptos_motion::*;
```

### 3. Feature Flags

Enable only the features you need:

```toml
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["core-animations", "raf"] }

# For production, use optimized presets
leptos-motion-core = { version = "0.4.0", features = ["production"] }
```

### 3. Code Splitting

Split animations into separate chunks:

```rust
// Lazy load complex animations
let complex_animation = create_resource(
    || (),
    |_| async move {
        // Load animation configuration
        load_animation_config().await
    }
);

view! {
    <Suspense fallback=move || view! { <div>"Loading animation..."</div> }>
        {move || {
            complex_animation.get().map(|config| {
                view! {
                    <MotionDiv animate=config>
                        "Complex Animation"
                    </MotionDiv>
                }
            })
        }}
    </Suspense>
}
```

## Rendering Performance

### 1. Virtual Scrolling for Large Lists

Implement virtual scrolling for performance with large datasets:

```rust
#[component]
fn VirtualList(items: Vec<String>) -> impl IntoView {
    let (visible_range, set_visible_range) = signal((0, 20));
    let container_height = 400.0;
    let item_height = 50.0;

    view! {
        <div class="virtual-list" style="height: {container_height}px; overflow: auto;">
            <div style="height: {items.len() as f64 * item_height}px; position: relative;">
                {move || {
                    let (start, end) = visible_range.get();
                    items[start..end.min(items.len())].iter().enumerate().map(|(i, item)| {
                        let top = (start + i) as f64 * item_height;
                        view! {
                            <MotionDiv
                                class="list-item"
                                style="position: absolute; top: {top}px; height: {item_height}px;"
                                initial=Some(motion_target!(
                                    "opacity" => AnimationValue::Number(0.0),
                                    "y" => AnimationValue::Pixels(20.0)
                                ))
                                animate=motion_target!(
                                    "opacity" => AnimationValue::Number(1.0),
                                    "y" => AnimationValue::Pixels(0.0)
                                )
                            >
                                {item}
                            </MotionDiv>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
```

### 2. Debounce Animation Updates

Prevent excessive animation updates:

```rust
use std::time::Duration;

#[component]
fn DebouncedAnimation() -> impl IntoView {
    let (value, set_value) = signal(0.0);
    let debounced_value = create_rw_signal(0.0);

    // Debounce updates
    Effect::new(move |_| {
        let current_value = value.get();
        set_timeout_with_handle(
            move || {
                debounced_value.set(current_value);
            },
            Duration::from_millis(16) // ~60fps
        ).expect("Failed to set timeout");
    });

    view! {
        <MotionDiv
            animate=motion_target!(
                "x" => AnimationValue::Pixels(debounced_value.get())
            )
        >
            "Debounced Animation"
        </MotionDiv>
    }
}
```

### 3. Use RequestAnimationFrame

Coordinate animations with the browser's refresh rate:

```rust
use web_sys::window;

#[component]
fn RafOptimizedAnimation() -> impl IntoView {
    let (frame_count, set_frame_count) = signal(0);

    Effect::new(move |_| {
        let window = window().expect("No window");

        let animate = move || {
            set_frame_count.update(|count| *count += 1);

            // Schedule next frame
            window.request_animation_frame(&Closure::wrap(Box::new(animate) as Box<dyn FnMut()>))
                .expect("Failed to request animation frame");
        };

        // Start animation loop
        window.request_animation_frame(&Closure::wrap(Box::new(animate) as Box<dyn FnMut()>))
            .expect("Failed to request animation frame");
    });

    view! {
        <div>
            <p>"Frame count: {frame_count}"</p>
        </div>
    }
}
```

## Profiling and Monitoring

### 1. Performance Monitoring

Implement performance monitoring:

```rust
use std::time::Instant;

#[component]
fn PerformanceMonitor() -> impl IntoView {
    let (fps, set_fps) = signal(0.0);
    let (frame_time, set_frame_time) = signal(0.0);
    let (memory_usage, set_memory_usage) = signal(0.0);

    Effect::new(move |_| {
        let start_time = Instant::now();

        // Measure frame time
        let frame_duration = start_time.elapsed();
        set_frame_time.set(frame_duration.as_millis() as f64);

        // Calculate FPS
        if frame_duration.as_millis() > 0 {
            let current_fps = 1000.0 / frame_duration.as_millis() as f64;
            set_fps.set(current_fps);
        }

        // Monitor memory usage (if available)
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                if let Some(memory) = performance.memory() {
                    let used_mb = memory.used_js_heap_size() as f64 / (1024.0 * 1024.0);
                    set_memory_usage.set(used_mb);
                }
            }
        }
    });

    view! {
        <div class="performance-monitor">
            <div class="metric">
                <span>"FPS: "</span>
                <span class={move || if fps.get() < 50.0 { "warning" } else { "normal" }}>
                    {format!("{:.1}", fps.get())}
                </span>
            </div>
            <div class="metric">
                <span>"Frame Time: "</span>
                <span class={move || if frame_time.get() > 16.0 { "warning" } else { "normal" }}>
                    {format!("{:.1}ms", frame_time.get())}
                </span>
            </div>
            <div class="metric">
                <span>"Memory: "</span>
                <span class={move || if memory_usage.get() > 50.0 { "warning" } else { "normal" }}>
                    {format!("{:.1}MB", memory_usage.get())}
                </span>
            </div>
        </div>
    }
}
```

### 2. Animation Profiling

Profile individual animations:

```rust
#[component]
fn ProfiledAnimation() -> impl IntoView {
    let animation_start = create_rw_signal(Instant::now());
    let animation_duration = create_rw_signal(0.0);

    let on_animation_start = move || {
        animation_start.set(Instant::now());
    };

    let on_animation_complete = move || {
        let duration = animation_start.get().elapsed();
        animation_duration.set(duration.as_millis() as f64);

        // Log performance data
        log::info!("Animation completed in {:.2}ms", duration.as_millis());
    };

    view! {
        <div>
            <MotionDiv
                on_animation_start=on_animation_start
                on_animation_complete=on_animation_complete
                animate=motion_target!(
                    "x" => AnimationValue::Pixels(200.0)
                )
            >
                "Profiled Animation"
            </MotionDiv>
            <p>"Animation duration: {animation_duration}ms"</p>
        </div>
    }
}
```

## Performance Benchmarks

### 1. Animation Performance Test

```rust
#[component]
fn PerformanceBenchmark() -> impl IntoView {
    let (test_results, set_test_results) = signal(Vec::new());
    let (is_running, set_running) = signal(false);

    let run_benchmark = move || {
        set_running.set(true);

        // Test different animation types
        let tests = vec![
            ("Transform Only", motion_target!("x" => AnimationValue::Pixels(100.0))),
            ("Opacity Only", motion_target!("opacity" => AnimationValue::Number(0.5))),
            ("Multiple Properties", motion_target!(
                "x" => AnimationValue::Pixels(100.0),
                "y" => AnimationValue::Pixels(50.0),
                "scale" => AnimationValue::Number(1.5),
                "rotate" => AnimationValue::Degrees(45.0)
            )),
            ("Spring Animation", motion_target!("x" => AnimationValue::Pixels(100.0))),
        ];

        for (test_name, animation) in tests {
            let start_time = Instant::now();

            // Run animation
            // ... animation logic ...

            let duration = start_time.elapsed();
            set_test_results.update(|results| {
                results.push((test_name.to_string(), duration.as_millis() as f64));
            });
        }

        set_running.set(false);
    };

    view! {
        <div class="benchmark">
            <button on:click=run_benchmark disabled=is_running>
                "Run Performance Benchmark"
            </button>

            <div class="results">
                <h3>"Benchmark Results"</h3>

                {move || {
                    test_results.get().iter().map(|(test_name, duration)| {
                        view! {
                            <div class="test-result">
                                <span class="test-name">{test_name}</span>
                                <span class="test-duration">{format!("{:.2}ms", duration)}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
```

## Best Practices

### 1. Performance-First Development

- **Start with performance in mind**: Design animations to be performant from the beginning
- **Measure early and often**: Use performance monitoring tools during development
- **Set performance budgets**: Define acceptable performance limits for your application
- **Test on target devices**: Ensure performance on the devices your users actually use

### 2. Animation Design Principles

- **Keep animations short**: Aim for 200-500ms duration for most interactions
- **Use easing appropriately**: Choose easing functions that feel natural
- **Avoid over-animation**: Don't animate everything - use animation to enhance, not distract
- **Consider accessibility**: Respect `prefers-reduced-motion` user preference

### 3. Code Organization

- **Separate concerns**: Keep animation logic separate from business logic
- **Reuse animations**: Create reusable animation configurations
- **Lazy load**: Only load animations when they're needed
- **Profile regularly**: Include performance testing in your CI/CD pipeline

### 4. Monitoring and Maintenance

- **Real-time monitoring**: Monitor performance in production
- **Performance budgets**: Set and enforce performance budgets
- **Regular audits**: Schedule regular performance reviews
- **User feedback**: Collect performance feedback from real users

### 5. Optimization Checklist

Before deploying animations to production:

- [ ] Animation runs at 60fps on target devices
- [ ] Memory usage stays within budget
- [ ] Bundle size impact is acceptable
- [ ] Accessibility features are implemented
- [ ] Performance monitoring is in place
- [ ] Fallbacks are provided for older browsers
- [ ] Tests cover performance edge cases
      {move || {
      test*results.get().iter().map(|(name, duration)| {
      view! {
      <div class="result">
      <span>{name}</span>
      <span>{format!("{:.2}ms", duration)}</span>
      </div>
      }
      }).collect::<Vec<*>>()
      }}
      </div>
      </div>
      }
      }

````

### 2. Memory Usage Test

```rust
#[component]
fn MemoryBenchmark() -> impl IntoView {
    let (memory_before, set_memory_before) = signal(0.0);
    let (memory_after, set_memory_after) = signal(0.0);
    let (leak_detected, set_leak_detected) = signal(false);

    let run_memory_test = move || {
        // Measure memory before
        if let Some(window) = window() {
            if let Some(performance) = window.performance() {
                if let Some(memory) = performance.memory() {
                    let before = memory.used_js_heap_size() as f64 / (1024.0 * 1024.0);
                    set_memory_before.set(before);

                    // Create many animations
                    for _ in 0..100 {
                        let motion_value = MotionValue::new(0.0);
                        motion_value.subscribe(|_| {});
                    }

                    // Measure memory after
                    let after = memory.used_js_heap_size() as f64 / (1024.0 * 1024.0);
                    set_memory_after.set(after);

                    // Check for memory leak
                    let increase = after - before;
                    set_leak_detected.set(increase > 1.0); // More than 1MB increase
                }
            }
        }
    };

    view! {
        <div class="memory-benchmark">
            <button on:click=run_memory_test>
                "Run Memory Test"
            </button>

            <div class="memory-results">
                <p>"Memory before: {memory_before}MB"</p>
                <p>"Memory after: {memory_after}MB"</p>
                <p class={move || if leak_detected.get() { "error" } else { "success" }}>
                    {if leak_detected.get() { "Memory leak detected!" } else { "No memory leak detected" }}
                </p>
            </div>
        </div>
    }
}
````

## Best Practices Summary

1. **Use transform properties** for best performance
2. **Limit concurrent animations** to prevent frame drops
3. **Clean up motion values** to prevent memory leaks
4. **Use will-change CSS** to hint GPU acceleration
5. **Debounce rapid updates** to maintain 60fps
6. **Monitor performance** in development and production
7. **Profile animations** to identify bottlenecks
8. **Optimize bundle size** with tree shaking and feature flags
9. **Use virtual scrolling** for large lists
10. **Coordinate with requestAnimationFrame** for smooth animations

By following these guidelines, you can ensure your Leptos Motion animations perform optimally across all devices and browsers.
