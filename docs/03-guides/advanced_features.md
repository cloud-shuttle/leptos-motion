# Advanced Features Guide

This guide covers the advanced features of Leptos Motion, including animation presets, advanced gestures, interactive tools, and performance optimizations.

**Version**: 0.4.0  
**Bundle Size**: 30KB-85KB (92% reduction from 378KB)  
**Features**: Minimal serialization, conditional compilation, feature flags, and comprehensive optimization

## Table of Contents

- [v0.4.0 Optimization Features](#v040-optimization-features)
- [Animation Presets](#animation-presets)
- [Advanced Gestures](#advanced-gestures)
- [Interactive Playground](#interactive-playground)
- [Timeline Editor](#timeline-editor)
- [Performance Optimizations](#performance-optimizations)
- [Best Practices](#best-practices)

## v0.4.0 Optimization Features

### Minimal Serialization System

Leptos Motion v0.4.0 includes a custom minimal serialization system that replaces serde for significant bundle size savings:

```rust
use leptos_motion_core::minimal_serialization::*;

// Lightweight JSON serialization
let serializer = MinimalJsonSerializer::new();
let data = serializer.serialize(&animation_data)?;

// Binary serialization for performance-critical applications
let binary_serializer = MinimalBinarySerializer::new();
let binary_data = binary_serializer.serialize(&animation_data)?;

// Compact string serialization for simple data
let string_serializer = CompactStringSerializer::new();
let string_data = string_serializer.serialize(&simple_data)?;
```

### Conditional Web-Sys Features

Optimized web-sys usage with minimal feature sets:

```rust
#[cfg(feature = "conditional-web-sys")]
use leptos_motion_core::web_sys_optimized::*;

// Only load web-sys features you actually need
#[cfg(feature = "web-sys-performance")]
let performance_api = get_performance_api()?;

#[cfg(feature = "web-sys-resize-observer")]
let resize_observer = create_resize_observer()?;
```

### Feature-Based Compilation

Granular control over functionality with feature flags:

```rust
// Core animations only (30KB)
#[cfg(feature = "core-animations")]
use leptos_motion_core::animations::*;

// Gesture support (additional 20KB)
#[cfg(feature = "gesture-support")]
use leptos_motion_core::gestures::*;

// Layout animations (additional 25KB)
#[cfg(feature = "layout-animations")]
use leptos_motion_core::layout::*;

// Performance monitoring (additional 10KB)
#[cfg(feature = "performance-metrics")]
use leptos_motion_core::performance::*;
```

## Animation Presets

Leptos Motion provides a comprehensive library of pre-built animations for common use cases. These presets are optimized for performance and follow design best practices.

### Available Presets

#### Entrance Animations

```rust
use leptos_motion_core::AnimationPresets;

// Fade in animation
let fade_in = AnimationPresets::fade_in();

// Slide up animation
let slide_up = AnimationPresets::slide_up(50.0);

// Scale in animation
let scale_in = AnimationPresets::scale_in();

// Pop in animation
let pop_in = AnimationPresets::pop_in();

// Rotate in animation
let rotate_in = AnimationPresets::rotate_in();

// Flip in animation
let flip_in = AnimationPresets::flip_in();
```

#### Exit Animations

```rust
// Fade out animation
let fade_out = AnimationPresets::fade_out();

// Slide out up animation
let slide_out_up = AnimationPresets::slide_out_up(50.0);

// Scale out animation
let scale_out = AnimationPresets::scale_out();

// Pop out animation
let pop_out = AnimationPresets::pop_out();
```

#### Interaction Animations

```rust
// Hover lift animation
let hover_lift = AnimationPresets::hover_lift();

// Hover scale animation
let hover_scale = AnimationPresets::hover_scale(1.1);

// Tap press animation
let tap_press = AnimationPresets::tap_press();

// Focus glow animation
let focus_glow = AnimationPresets::focus_glow();
```

#### Attention Animations

```rust
// Pulse animation
let pulse = AnimationPresets::pulse();

// Bounce animation
let bounce = AnimationPresets::bounce();

// Shake animation
let shake = AnimationPresets::shake();

// Wiggle animation
let wiggle = AnimationPresets::wiggle();

// Tada animation
let tada = AnimationPresets::tada();
```

#### Loading Animations

```rust
// Spin animation
let spin = AnimationPresets::spin();

// Ping animation
let ping = AnimationPresets::ping();

// Bounce loading animation
let bounce_loading = AnimationPresets::bounce_loading();
```

#### Page Transitions

```rust
// Page slide transition
let page_slide = AnimationPresets::page_slide(SlideDirection::Left);

// Page fade transition
let page_fade = AnimationPresets::page_fade();

// Page scale transition
let page_scale = AnimationPresets::page_scale();
```

#### Modal Animations

```rust
// Modal slide up
let modal_slide_up = AnimationPresets::modal_slide_up();

// Modal scale
let modal_scale = AnimationPresets::modal_scale();
```

#### List Animations

```rust
// Stagger children animation
let stagger_children = AnimationPresets::stagger_children(0.1);

// List item entrance
let list_item_entrance = AnimationPresets::list_item_entrance();

// List item exit
let list_item_exit = AnimationPresets::list_item_exit();
```

### Using Presets in Components

```rust
use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_core::AnimationPresets;

#[component]
fn AnimatedCard() -> impl IntoView {
    let animation = AnimationPresets::slide_up(30.0);

    view! {
        <MotionDiv
            initial=animation.initial.clone()
            animate=animation.animate.clone()
            transition=animation.transition.clone()
            class="card"
        >
            "Animated Content"
        </MotionDiv>
    }
}
```

### Spring Configuration Presets

```rust
use leptos_motion_core::springs;

// Gentle spring (smooth, minimal overshoot)
let gentle = springs::GENTLE;

// Bouncy spring (more oscillation)
let bouncy = springs::BOUNCY;

// Snappy spring (fast response)
let snappy = springs::SNAPPY;

// Wobbly spring (very bouncy)
let wobbly = springs::WOBBLY;

// Slow spring (smooth and slow)
let slow = springs::SLOW;
```

### Easing Presets

```rust
use leptos_motion_core::easings;

// Material Design ease
let ease = easings::EASE;

// Material Design ease in
let ease_in = easings::EASE_IN;

// Material Design ease out
let ease_out = easings::EASE_OUT;

// Material Design ease in out
let ease_in_out = easings::EASE_IN_OUT;

// Smooth spring
let spring_smooth = easings::SPRING_SMOOTH;

// Bouncy spring
let spring_bouncy = easings::SPRING_BOUNCY;

// Gentle spring
let spring_gentle = easings::SPRING_GENTLE;
```

## Advanced Gestures

Leptos Motion provides advanced gesture recognition for complex interactions including multi-touch, pinch-to-zoom, and gesture composition.

### Gesture Types

```rust
use leptos_motion_gestures::{
    AdvancedGestureRecognizer, AdvancedGestureType, SwipeDirection
};

// Pinch to zoom gesture
let pinch_zoom = AdvancedGestureType::PinchZoom;

// Rotate gesture
let rotate = AdvancedGestureType::Rotate;

// Swipe gesture with direction
let swipe_up = AdvancedGestureType::Swipe(SwipeDirection::Up);
let swipe_down = AdvancedGestureType::Swipe(SwipeDirection::Down);
let swipe_left = AdvancedGestureType::Swipe(SwipeDirection::Left);
let swipe_right = AdvancedGestureType::Swipe(SwipeDirection::Right);

// Long press gesture
let long_press = AdvancedGestureType::LongPress;

// Double tap gesture
let double_tap = AdvancedGestureType::DoubleTap;

// Pan gesture with momentum
let pan_with_momentum = AdvancedGestureType::PanWithMomentum;

// Multi-finger tap
let two_finger_tap = AdvancedGestureType::MultiFingerTap(2);
let three_finger_tap = AdvancedGestureType::MultiFingerTap(3);

// Gesture composition
let composite = AdvancedGestureType::Composite(vec![
    AdvancedGestureType::PinchZoom,
    AdvancedGestureType::Rotate,
]);
```

### Setting Up Gesture Recognition

```rust
use leptos_motion_gestures::AdvancedGestureRecognizer;
use web_sys::TouchEvent;

#[component]
fn GestureComponent() -> impl IntoView {
    let (gesture_recognizer, set_gesture_recognizer) = create_signal(AdvancedGestureRecognizer::new());

    let handle_touch_start = move |event: TouchEvent| {
        let mut recognizer = gesture_recognizer.get();
        let events = recognizer.handle_touch_start(&event);

        for event in events {
            match event.gesture_type {
                GestureType::PinchZoom => {
                    // Handle pinch zoom
                    log::info!("Pinch zoom: scale = {}", event.scale);
                }
                GestureType::Rotate => {
                    // Handle rotation
                    log::info!("Rotate: angle = {}", event.rotation);
                }
                GestureType::Swipe => {
                    // Handle swipe
                    log::info!("Swipe: velocity = {}", event.velocity);
                }
                _ => {}
            }
        }

        set_gesture_recognizer.set(recognizer);
    };

    view! {
        <div
            class="gesture-area"
            on:touchstart=handle_touch_start
            style="width: 300px; height: 300px; background: #f0f0f0;"
        >
            "Gesture Area"
        </div>
    }
}
```

### Gesture Configuration

```rust
use leptos_motion_gestures::AdvancedGestureConfig;

let config = AdvancedGestureConfig {
    min_swipe_distance: 50.0,      // Minimum distance for swipe
    min_swipe_velocity: 200.0,     // Minimum velocity for swipe
    long_press_duration: 500.0,    // Long press duration in milliseconds
    double_tap_interval: 300.0,    // Double tap interval in milliseconds
    pinch_sensitivity: 1.0,        // Pinch zoom sensitivity
    rotation_sensitivity: 1.0,     // Rotation sensitivity
    momentum_decay: 0.95,          // Momentum decay factor
};
```

### Advanced Gesture States

```rust
use leptos_motion_gestures::{
    PinchZoomState, RotateState, SwipeState, LongPressState
};

// Pinch zoom state
let pinch_state = PinchZoomState {
    scale: 1.5,
    previous_scale: 1.0,
    center_x: 150.0,
    center_y: 150.0,
    distance: 200.0,
    previous_distance: 150.0,
};

// Rotate state
let rotate_state = RotateState {
    angle: 45.0,
    previous_angle: 0.0,
    center_x: 150.0,
    center_y: 150.0,
};

// Swipe state
let swipe_state = SwipeState {
    direction: SwipeDirection::Up,
    velocity: 500.0,
    distance: 100.0,
    start_x: 150.0,
    start_y: 200.0,
    end_x: 150.0,
    end_y: 100.0,
};

// Long press state
let long_press_state = LongPressState {
    duration: 800.0,
    x: 150.0,
    y: 150.0,
    is_active: true,
};
```

## Interactive Playground

The Animation Playground is an interactive tool for building and testing animations in real-time.

### Using the Playground

```rust
use leptos_motion_dom::AnimationPlayground;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Animation Builder"</h1>
            <AnimationPlayground />
        </div>
    }
}
```

### Playground Features

1. **Preset Library**: Browse and test pre-built animations
2. **Custom Animation Builder**: Create animations with visual controls
3. **Real-time Preview**: See animations as you build them
4. **Code Generation**: Get the generated code for your animations
5. **Property Controls**: Adjust animation properties with sliders and inputs
6. **Transition Editor**: Configure easing, duration, and delays
7. **Export Options**: Copy code or export animations

### Playground Components

```rust
use leptos_motion_dom::playground::{AnimationPlayground, TimelineEditor};

// Animation playground with preset library
<AnimationPlayground />

// Timeline editor for keyframe animations
<TimelineEditor />
```

## Timeline Editor

The Timeline Editor provides a visual interface for creating complex keyframe animations.

### Timeline Features

1. **Visual Timeline**: Drag and drop keyframes on a timeline
2. **Property Tracks**: Separate tracks for different properties
3. **Real-time Preview**: See animations as you edit
4. **Easing Curves**: Visual easing curve editor
5. **Keyframe Interpolation**: Smooth interpolation between keyframes
6. **Export/Import**: Save and load animation timelines

### Using the Timeline Editor

```rust
use leptos_motion_dom::TimelineEditor;

#[component]
fn AnimationStudio() -> impl IntoView {
    view! {
        <div class="animation-studio">
            <h2>"Animation Timeline Editor"</h2>
            <TimelineEditor />
        </div>
    }
}
```

## Performance Optimizations

Leptos Motion includes advanced performance optimizations for smooth animations.

### GPU Acceleration

Transform properties are automatically GPU-accelerated:

```rust
// These properties are GPU-accelerated
let gpu_properties = motion_target!(
    "x" => AnimationValue::Pixels(100.0),      // ✅ GPU-accelerated
    "y" => AnimationValue::Pixels(100.0),      // ✅ GPU-accelerated
    "scale" => AnimationValue::Number(1.5),    // ✅ GPU-accelerated
    "rotate" => AnimationValue::Degrees(45.0), // ✅ GPU-accelerated
    "opacity" => AnimationValue::Number(0.8),  // ✅ GPU-accelerated
);

// These properties trigger layout reflows
let layout_properties = motion_target!(
    "width" => AnimationValue::Pixels(200.0),  // ❌ Layout-triggering
    "height" => AnimationValue::Pixels(200.0), // ❌ Layout-triggering
    "padding" => AnimationValue::Pixels(20.0), // ❌ Layout-triggering
);
```

### Performance Monitoring

```rust
use leptos_motion_core::performance::{PerformanceMonitor, PerformanceBudget};

// Create performance budget
let budget = PerformanceBudget {
    max_frame_time: 16.67,           // 60 FPS target
    max_concurrent_animations: 100,   // Max concurrent animations
    max_memory_usage: 10 * 1024 * 1024, // 10MB memory limit
    max_animation_duration: 5000.0,  // 5 second max duration
};

// Create performance monitor
let monitor = PerformanceMonitor::new(budget).unwrap();

// Monitor frame rate
let fps = monitor.get_fps();
let frame_drop_rate = monitor.get_frame_drop_rate();

// Check if within budget
let is_within_budget = monitor.is_within_budget();
```

### Memory Management

```rust
use leptos_motion_core::performance::AnimationPool;

// Animation object pooling
let mut pool = AnimationPool::new(100); // Pool size of 100

// Get animation from pool
let animation = pool.acquire().unwrap();

// Return animation to pool when done
pool.release(animation);
```

### Optimized Animation Engine

```rust
use leptos_motion_core::OptimizedHybridEngine;

// Create optimized engine
let mut engine = OptimizedHybridEngine::new();

// Engine automatically chooses best animation method
let handle = engine.animate(&config)?;

// Get performance metrics
if let Some(metrics) = engine.get_performance_metrics() {
    log::info!("FPS: {}, Memory: {} MB", metrics.fps, metrics.memory_usage);
}
```

## Best Practices

### Animation Performance

1. **Use GPU-accelerated properties**: Prefer `transform` and `opacity` over layout-triggering properties
2. **Limit concurrent animations**: Don't animate too many elements simultaneously
3. **Use appropriate easing**: Avoid complex easing functions for performance-critical animations
4. **Monitor performance**: Use performance monitoring in development
5. **Optimize for mobile**: Test animations on mobile devices

### Gesture Design

1. **Provide visual feedback**: Always give users feedback for their gestures
2. **Use appropriate thresholds**: Set reasonable thresholds for gesture recognition
3. **Handle edge cases**: Consider what happens when gestures are interrupted
4. **Test on different devices**: Ensure gestures work on various screen sizes and input methods
5. **Follow platform conventions**: Respect platform-specific gesture patterns

### Animation Design

1. **Keep it subtle**: Animations should enhance, not distract from the content
2. **Use consistent timing**: Maintain consistent animation durations throughout your app
3. **Consider accessibility**: Respect user preferences for reduced motion
4. **Test on low-end devices**: Ensure animations perform well on slower devices
5. **Provide fallbacks**: Have fallback animations for unsupported features

### Code Organization

1. **Use presets**: Leverage the built-in animation presets for common patterns
2. **Create reusable animations**: Build custom animations that can be reused
3. **Separate concerns**: Keep animation logic separate from business logic
4. **Document complex animations**: Add comments for complex animation sequences
5. **Test animations**: Write tests for critical animation behaviors

## Examples

### Complete Animation Example

```rust
use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_core::{AnimationPresets, springs};

#[component]
fn AnimatedCard() -> impl IntoView {
    let (is_hovered, set_is_hovered) = create_signal(false);

    let animation = AnimationPresets::slide_up(30.0);
    let hover_variants = AnimationPresets::hover_lift();

    view! {
        <MotionDiv
            initial=animation.initial.clone()
            animate=animation.animate.clone()
            transition=Transition {
                ease: springs::GENTLE,
                ..animation.transition.clone()
            }
            variants=Some(hover_variants)
            while_hover="hover"
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
            class="card"
            style="width: 200px; height: 150px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 12px; cursor: pointer;"
        >
            <div class="card-content">
                <h3>"Animated Card"</h3>
                <p>"Hover me to see the animation!"</p>
                {move || if is_hovered.get() {
                    view! { <span class="hover-indicator">"✨ Hovered!"</span> }
                } else {
                    view! { <span></span> }
                }}
            </div>
        </MotionDiv>
    }
}
```

### Advanced Gesture Example

```rust
use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion_gestures::AdvancedGestureRecognizer;

#[component]
fn GestureDemo() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (rotation, set_rotation) = create_signal(0.0);
    let (position, set_position) = create_signal((0.0, 0.0));

    let (gesture_recognizer, set_gesture_recognizer) = create_signal(AdvancedGestureRecognizer::new());

    let handle_gesture = move |event: GestureEvent| {
        match event.gesture_type {
            GestureType::PinchZoom => {
                set_scale.set(event.scale);
            }
            GestureType::Rotate => {
                set_rotation.set(event.rotation);
            }
            GestureType::Swipe => {
                set_position.set((event.x, event.y));
            }
            _ => {}
        }
    };

    view! {
        <div class="gesture-demo">
            <MotionDiv
                class="gesture-target"
                drag=true
                drag_constraints=DragConstraints::Bounded {
                    left: -200.0,
                    right: 200.0,
                    top: -200.0,
                    bottom: 200.0,
                }
                animate=motion_target!(
                    "scale" => AnimationValue::Number(scale.get()),
                    "rotate" => AnimationValue::Degrees(rotation.get()),
                    "x" => AnimationValue::Pixels(position.get().0),
                    "y" => AnimationValue::Pixels(position.get().1),
                )
                transition=Transition {
                    ease: springs::GENTLE,
                    ..Default::default()
                }
                style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; cursor: grab;"
            >
                "Interactive"
            </MotionDiv>
        </div>
    }
}
```

This comprehensive guide covers all the advanced features of Leptos Motion. For more specific examples and use cases, refer to the individual feature documentation and the example applications.
