# Leptos Motion API Reference

## Overview

Leptos Motion is a high-performance animation library for Rust web applications using the Leptos framework. It provides smooth, WASM-powered animations with a reactive API.

## Core Components

### ReactiveMotionDivV2

The main animation component that provides reactive animations based on Leptos signals.

```rust
use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
fn AnimatedComponent() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    // Update animation signal
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!("scale({})", scale.get())));
        set_animate_signal.set(animations);
    });

    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        initial
    };

    let transition = Transition {
        duration: Some(0.3),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <ReactiveMotionDivV2
            initial=initial_values
            animate=animate_signal
            transition=transition
        >
            <div>"Animated Content"</div>
        </ReactiveMotionDivV2>
    }
}
```

#### Props

- `initial: Option<HashMap<String, AnimationValue>>` - Initial animation values
- `animate: ReadSignal<HashMap<String, AnimationValue>>` - Reactive animation values
- `transition: Option<Transition>` - Transition configuration

### DragMotionDiv

A component that provides drag functionality with constraints and momentum.

```rust
use leptos_motion_dom::drag_motion_div::DragMotionDiv;
use leptos_motion_dom::*;

#[component]
fn DraggableComponent() -> impl IntoView {
    let drag_config = DragConfig {
        enabled: true,
        constraints: Some(DragConstraints {
            min_x: Some(-100.0),
            max_x: Some(100.0),
            min_y: Some(-100.0),
            max_y: Some(100.0),
        }),
        momentum: Some(DragMomentum {
            enabled: true,
            damping: 0.8,
            stiffness: 0.1,
        }),
    };

    view! {
        <DragMotionDiv drag=drag_config>
            <div>"Draggable Content"</div>
        </DragMotionDiv>
    }
}
```

#### Props

- `drag: Option<DragConfig>` - Drag configuration

## Animation Values

### AnimationValue

Represents different types of animation values:

```rust
pub enum AnimationValue {
    String(String),        // CSS string values
    Number(f64),          // Numeric values
    Pixels(f64),          // Pixel values
    Degrees(f64),         // Degree values
    Percentage(f64),      // Percentage values
    Radians(f64),         // Radian values
    Color(String),        // Color values
    Transform(Transform), // Transform matrix
    Complex(ComplexValue), // Complex values
}
```

### Usage Examples

```rust
// String values for CSS properties
animations.insert("transform".to_string(), AnimationValue::String("translate(10px, 20px) scale(1.5)".to_string()));

// Numeric values
animations.insert("opacity".to_string(), AnimationValue::Number(0.8));

// Pixel values
animations.insert("width".to_string(), AnimationValue::Pixels(200.0));

// Degree values
animations.insert("rotate".to_string(), AnimationValue::Degrees(45.0));

// Percentage values
animations.insert("height".to_string(), AnimationValue::Percentage(100.0));
```

## Transition Configuration

### Transition

Controls how animations are performed:

```rust
pub struct Transition {
    pub duration: Option<f64>,        // Duration in seconds
    pub delay: Option<f64>,           // Delay in seconds
    pub ease: Easing,                 // Easing function
    pub repeat: RepeatConfig,         // Repeat configuration
    pub stagger: Option<f64>,         // Stagger delay in seconds
}
```

### Easing

Available easing functions:

```rust
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}
```

### RepeatConfig

Controls animation repetition:

```rust
pub enum RepeatConfig {
    Never,
    Once,
    Infinite,
    Count(u32),
}
```

## Drag Configuration

### DragConfig

Configuration for drag behavior:

```rust
pub struct DragConfig {
    pub enabled: bool,
    pub constraints: Option<DragConstraints>,
    pub momentum: Option<DragMomentum>,
}
```

### DragConstraints

Defines drag boundaries:

```rust
pub struct DragConstraints {
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
}
```

### DragMomentum

Controls drag momentum physics:

```rust
pub struct DragMomentum {
    pub enabled: bool,
    pub damping: f64,     // 0.0 to 1.0
    pub stiffness: f64,   // 0.0 to 1.0
}
```

## Animation Engine

### AnimationEngine

The core animation engine that manages all animations:

```rust
use leptos_motion_dom::animation_engine::AnimationEngine;

// Create animation engine
let mut engine = AnimationEngine::new();

// Start the animation loop
engine.start_animation_loop();

// Animate a property
engine.animate_property(
    "transform".to_string(),
    0.0,  // start value
    100.0, // end value
    transition,
);

// Set up update callback
engine.on_update(|values| {
    // Apply values to DOM
    for (property, value) in values {
        // Update CSS property
    }
});
```

## Performance Considerations

### Best Practices

1. **Use ReactiveMotionDivV2** for most animations
2. **Batch DOM updates** by using the animation engine
3. **Limit concurrent animations** to maintain 60fps
4. **Use appropriate easing functions** for smooth animations
5. **Avoid animating layout properties** when possible

### Performance Monitoring

```rust
// Monitor animation performance
let start_time = js_sys::Date::now();
let mut frame_count = 0u32;

fn monitor_performance() {
    frame_count += 1;
    let current_time = js_sys::Date::now();
    let elapsed = current_time - start_time;
    
    if elapsed >= 1000.0 {
        let fps = (frame_count as f64 * 1000.0) / elapsed;
        console::log_1(&format!("FPS: {:.1}", fps).into());
        frame_count = 0;
    }
    
    request_animation_frame(Closure::wrap(Box::new(move |_| {
        monitor_performance();
    }) as Box<dyn FnMut(f64)>).into_js_value().unchecked_into());
}
```

## Common Patterns

### Reactive Animations

```rust
#[component]
fn ReactiveAnimation() -> impl IntoView {
    let (is_hovered, set_hovered) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_hovered.get() {
            animations.insert("scale".to_string(), AnimationValue::Number(1.2));
            animations.insert("opacity".to_string(), AnimationValue::Number(0.8));
        } else {
            animations.insert("scale".to_string(), AnimationValue::Number(1.0));
            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <div>"Hover me!"</div>
        </ReactiveMotionDivV2>
    }
}
```

### Staggered Animations

```rust
#[component]
fn StaggeredList() -> impl IntoView {
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    
    view! {
        <div>
            {items.into_iter().enumerate().map(|(index, item)| {
                let delay = index as f64 * 0.1; // 100ms stagger
                let transition = Transition {
                    duration: Some(0.5),
                    delay: Some(delay),
                    ease: Easing::EaseOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                };
                
                view! {
                    <ReactiveMotionDivV2
                        initial=HashMap::new()
                        animate=HashMap::new()
                        transition=transition
                    >
                        <div>{item}</div>
                    </ReactiveMotionDivV2>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

### Complex Transform Animations

```rust
#[component]
fn ComplexTransform() -> impl IntoView {
    let (rotation, set_rotation) = create_signal(0.0);
    let (scale, set_scale) = create_signal(1.0);
    let (translate_x, set_translate_x) = create_signal(0.0);
    let (translate_y, set_translate_y) = create_signal(0.0);
    
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!(
            "translate({}px, {}px) scale({}) rotate({}deg)",
            translate_x.get(),
            translate_y.get(),
            scale.get(),
            rotation.get()
        )));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Complex Transform"</div>
        </ReactiveMotionDivV2>
    }
}
```

## Error Handling

### Common Issues

1. **Animation Engine Not Started**
   ```rust
   // Make sure to start the animation engine
   engine.start_animation_loop();
   ```

2. **Reactive Signal Disposal**
   ```rust
   // Use mount_to_body instead of mounting to specific containers
   mount_to_body(|| view! { <App/> })
   ```

3. **Transform Property Handling**
   ```rust
   // Use AnimationValue::String for complex transforms
   animations.insert("transform".to_string(), AnimationValue::String("translate(10px, 20px) scale(1.5)".to_string()));
   ```

## Migration Guide

### From v0.6 to v0.9

1. **Update Component Names**
   - `MotionDiv` → `ReactiveMotionDivV2`
   - Update prop names to match new API

2. **Update Signal Usage**
   - Use `ReadSignal` for animate prop
   - Use `RwSignal::new()` instead of `create_rw_signal()`

3. **Update Transition Configuration**
   - Use `ease` instead of `easing`
   - Use `RepeatConfig` enum instead of `Option<u32>`

## Examples

See the following examples for complete implementations:

- `simple-working-demo/` - Basic reactive animations
- `performance-demo/` - Performance benchmarking
- `phase2-reactive-demo/` - Advanced reactive features

## Support

For issues and questions:
- GitHub Issues: [leptos-motion repository]
- Documentation: [API Reference]
- Examples: [Example Gallery]
