# Getting Started with Leptos Motion

Welcome to Leptos Motion! This guide will help you get up and running with the animation library for Leptos.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Concepts](#basic-concepts)
- [Your First Animation](#your-first-animation)
- [Animation Types](#animation-types)
- [Gestures](#gestures)
- [Advanced Features](#advanced-features)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Installation

Add Leptos Motion to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.7"
leptos_motion = "0.1.0-alpha"
```

For a complete setup with all features:

```toml
[dependencies]
leptos = { version = "0.7", features = ["csr", "hydrate", "ssr"] }
leptos_motion = { version = "0.1.0-alpha", features = ["csr"] }
```

## Quick Start

Here's a simple example to get you started:

```rust
use leptos::prelude::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Welcome to Leptos Motion!"</h1>

            <MotionDiv
                class="animated-box"
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "x" => AnimationValue::Pixels(0.0)
                )
                initial=Some(motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "x" => AnimationValue::Pixels(-100.0)
                ))
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                "Hello, Animated World!"
            </MotionDiv>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
```

Add some CSS to make it look good:

```css
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  font-family: Arial, sans-serif;
}

.animated-box {
  padding: 20px;
  background: linear-gradient(45deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 8px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  transition: transform 0.2s ease;
}

.animated-box:hover {
  transform: translateY(-2px);
}
```

## Basic Concepts

### Animation Values

Leptos Motion uses typed animation values to ensure type safety:

```rust
// Different types of animation values
AnimationValue::Number(1.0)      // Plain numbers
AnimationValue::Pixels(100.0)    // Pixel values
AnimationValue::Degrees(45.0)    // Degree values
AnimationValue::Color(Rgba::new(1.0, 0.0, 0.0, 1.0))  // Colors
```

### Animation Targets

An animation target is a collection of properties to animate:

```rust
let target = motion_target!(
    "opacity" => AnimationValue::Number(1.0),
    "x" => AnimationValue::Pixels(100.0),
    "scale" => AnimationValue::Number(1.5),
    "rotate" => AnimationValue::Degrees(45.0)
);
```

### Transitions

Transitions control how animations behave:

```rust
let transition = Transition {
    duration: Some(0.5),           // Duration in seconds
    delay: Some(0.1),              // Delay before starting
    ease: Easing::EaseInOut,       // Easing function
    repeat: Some(RepeatConfig {     // Repeat configuration
        count: Some(3),
        reverse: true,
        delay: Some(0.2),
    }),
    stagger: Some(StaggerConfig {   // Stagger configuration
        delay: 0.1,
        from: StaggerFrom::Start,
        ease: Easing::EaseOut,
    }),
};
```

## Your First Animation

Let's create a simple fade-in animation:

```rust
#[component]
fn FadeInBox() -> impl IntoView {
    view! {
        <MotionDiv
            class="fade-box"
            initial=Some(motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            ))
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(1.0),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "This box fades in!"
        </MotionDiv>
    }
}
```

## Animation Types

### Tween Animations

Simple animations between two values:

```rust
#[component]
fn TweenAnimation() -> impl IntoView {
    let (is_expanded, set_expanded) = signal(false);

    view! {
        <div>
            <button on:click=move |_| set_expanded.set(!is_expanded.get())>
                "Toggle Size"
            </button>

            <MotionDiv
                class="tween-box"
                animate=motion_target!(
                    "scale" => AnimationValue::Number(if is_expanded.get() { 2.0 } else { 1.0 })
                )
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::BackOut,
                    ..Default::default()
                }
            >
                "Resizable Box"
            </MotionDiv>
        </div>
    }
}
```

### Spring Animations

Physics-based animations that feel natural:

```rust
#[component]
fn SpringAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            class="spring-box"
            animate=motion_target!(
                "x" => AnimationValue::Pixels(200.0),
                "y" => AnimationValue::Pixels(100.0)
            )
            transition=Transition {
                ease: Easing::Spring(SpringConfig {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }
        >
            "Spring Animation"
        </MotionDiv>
    }
}
```

### Keyframe Animations

Complex animations with multiple steps:

```rust
#[component]
fn KeyframeAnimation() -> impl IntoView {
    let keyframes = vec![
        (0.0, motion_target!("x" => AnimationValue::Pixels(0.0))),
        (0.5, motion_target!("x" => AnimationValue::Pixels(100.0), "y" => AnimationValue::Pixels(-50.0))),
        (1.0, motion_target!("x" => AnimationValue::Pixels(200.0), "y" => AnimationValue::Pixels(0.0))),
    ];

    view! {
        <MotionDiv
            class="keyframe-box"
            animate=motion_target!(
                "x" => AnimationValue::Pixels(200.0),
                "y" => AnimationValue::Pixels(0.0)
            )
            transition=Transition {
                duration: Some(2.0),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "Keyframe Animation"
        </MotionDiv>
    }
}
```

## Gestures

### Hover Animations

```rust
#[component]
fn HoverAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            class="hover-box"
            while_hover=Some(motion_target!(
                "scale" => AnimationValue::Number(1.1),
                "rotate" => AnimationValue::Degrees(5.0),
                "boxShadow" => AnimationValue::String("0 10px 30px rgba(0,0,0,0.3)".to_string())
            ))
            transition=Transition {
                duration: Some(0.2),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Hover over me!"
        </MotionDiv>
    }
}
```

### Tap Animations

```rust
#[component]
fn TapAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            class="tap-box"
            while_tap=Some(motion_target!(
                "scale" => AnimationValue::Number(0.95)
            ))
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Tap me!"
        </MotionDiv>
    }
}
```

### Drag Interactions

```rust
#[component]
fn DragAnimation() -> impl IntoView {
    let drag_config = DragConfig::new()
        .axis(DragAxis::Both)
        .constraints(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-100.0),
            bottom: Some(100.0),
        });

    view! {
        <MotionDiv
            class="drag-box"
            drag=Some(drag_config)
            while_drag=Some(motion_target!(
                "scale" => AnimationValue::Number(1.1),
                "zIndex" => AnimationValue::Number(1000.0)
            ))
        >
            "Drag me around!"
        </MotionDiv>
    }
}
```

## Advanced Features

### Variants

Define reusable animation states:

```rust
#[component]
fn VariantAnimation() -> impl IntoView {
    let (is_visible, set_visible) = signal(false);

    let variants = Variants::new()
        .variant("hidden", motion_target!(
            "opacity" => AnimationValue::Number(0.0),
            "x" => AnimationValue::Pixels(-100.0),
            "scale" => AnimationValue::Number(0.8)
        ))
        .variant("visible", motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "x" => AnimationValue::Pixels(0.0),
            "scale" => AnimationValue::Number(1.0)
        ));

    view! {
        <div>
            <button on:click=move |_| set_visible.set(!is_visible.get())>
                "Toggle Visibility"
            </button>

            <MotionDiv
                class="variant-box"
                variants=Some(variants)
                initial=Some("hidden".to_string())
                animate=Some(if is_visible.get() { "visible".to_string() } else { "hidden".to_string() })
                transition=Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                "Variant Animation"
            </MotionDiv>
        </div>
    }
}
```

### Layout Animations

Animate layout changes automatically:

```rust
#[component]
fn LayoutAnimation() -> impl IntoView {
    let (is_expanded, set_expanded) = signal(false);

    view! {
        <div class="layout-container">
            <button on:click=move |_| set_expanded.set(!is_expanded.get())>
                "Toggle Layout"
            </button>

            <MotionDiv
                class="layout-box"
                layout=Some(true)
                animate=motion_target!(
                    "width" => AnimationValue::Pixels(if is_expanded.get() { 300.0 } else { 150.0 }),
                    "height" => AnimationValue::Pixels(if is_expanded.get() { 200.0 } else { 100.0 })
                )
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                "Layout Animation"
            </MotionDiv>
        </div>
    }
}
```

### Staggered Animations

Animate multiple elements with delays:

```rust
#[component]
fn StaggeredAnimation() -> impl IntoView {
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

    view! {
        <div class="stagger-container">
            {items.into_iter().enumerate().map(|(i, item)| {
                view! {
                    <MotionDiv
                        class="stagger-item"
                        key=item
                        initial=Some(motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "y" => AnimationValue::Pixels(50.0)
                        ))
                        animate=motion_target!(
                            "opacity" => AnimationValue::Number(1.0),
                            "y" => AnimationValue::Pixels(0.0)
                        )
                        transition=Transition {
                            duration: Some(0.5),
                            delay: Some(i as f64 * 0.1),
                            ease: Easing::EaseOut,
                            ..Default::default()
                        }
                    >
                        {item}
                    </MotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

## Best Practices

### 1. Performance Optimization

- Use `transform` properties (x, y, scale, rotate) instead of layout properties when possible
- Avoid animating `width`, `height`, `top`, `left` for better performance
- Use `will-change` CSS property for elements that will animate frequently

```rust
// Good - animates transform properties
animate=motion_target!(
    "x" => AnimationValue::Pixels(100.0),
    "scale" => AnimationValue::Number(1.5)
)

// Avoid - animates layout properties
animate=motion_target!(
    "width" => AnimationValue::Pixels(200.0),
    "height" => AnimationValue::Pixels(200.0)
)
```

### 2. Easing Functions

Choose appropriate easing functions for different use cases:

```rust
// UI interactions - quick and responsive
ease: Easing::EaseOut

// Page transitions - smooth and elegant
ease: Easing::EaseInOut

// Loading states - bouncy and playful
ease: Easing::Spring(SpringConfig::default())

// Micro-interactions - snappy and precise
ease: Easing::BackOut
```

### 3. Animation Duration

Follow these guidelines for animation duration:

```rust
// Micro-interactions (hover, tap)
duration: Some(0.1..0.2)

// UI state changes
duration: Some(0.2..0.3)

// Page transitions
duration: Some(0.3..0.5)

// Complex animations
duration: Some(0.5..1.0)
```

### 4. Accessibility

Consider users with motion sensitivity:

```rust
// Check if user prefers reduced motion
let prefers_reduced_motion = window()
    .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok())
    .and_then(|m| m.matches().ok())
    .unwrap_or(false);

let duration = if prefers_reduced_motion {
    Some(0.1) // Faster animations for accessibility
} else {
    Some(0.5) // Normal duration
};
```

## Troubleshooting

### Common Issues

1. **Animation not working**
   - Check that the component is properly imported
   - Ensure the animation target is correctly formatted
   - Verify that the transition configuration is valid

2. **Performance issues**
   - Use transform properties instead of layout properties
   - Reduce the number of simultaneous animations
   - Consider using `will-change` CSS property

3. **Type errors**
   - Ensure animation values match the expected types
   - Check that all required properties are provided
   - Verify that the motion_target! macro is used correctly

### Debug Mode

Enable debug logging to troubleshoot issues:

```rust
// In your main function
console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
```

### Getting Help

- Check the [API Reference](api_reference.md) for detailed documentation
- Look at the [Examples](../examples/) for working code samples
- Open an issue on GitHub for bugs or feature requests

## Next Steps

Now that you have the basics, explore:

- [Advanced Animation Patterns](advanced_patterns.md)
- [Performance Optimization](performance.md)
- [Integration with Leptos Router](router_integration.md)
- [Custom Easing Functions](custom_easing.md)

Happy animating! 🎉
