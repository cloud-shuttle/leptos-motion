# Quick Start Guides

## Overview

Leptos Motion provides different levels of complexity for different use cases. Choose the guide that best fits your needs:

- **[Simple Animations](#simple-animations)** - CSS class-based animations for basic use cases
- **[Standard Animations](#standard-animations)** - JavaScript-based animations for most applications
- **[Advanced Animations](#advanced-animations)** - Enterprise features for complex applications

## Simple Animations

**Best for**: Prototypes, simple websites, basic UI interactions

**Bundle Size**: ~15KB (CSS only)
**Complexity**: Low
**Performance**: Excellent (CSS hardware acceleration)

### Quick Start

```rust
use leptos::*;
use leptos_motion_dom::*;

#[component]
fn SimpleFadeIn() -> impl IntoView {
    let (is_visible, set_is_visible) = create_signal(false);

    view! {
        <div class="fade-in" class:opacity-0=move || !is_visible()>
            <h1>"Hello World!"</h1>
            <button on:click=move |_| set_is_visible(true)>
                "Fade In"
            </button>
        </div>
    }
}
```

### CSS Classes Available

```css
.fade-in {
  animation: fadeIn 0.6s ease-out;
}
.slide-in {
  animation: slideIn 0.8s ease-out;
}
.bounce-in {
  animation: bounceIn 1s ease-out;
}
.scale-in {
  animation: scaleIn 0.5s ease-out;
}
.rotate-in {
  animation: rotateIn 0.7s ease-out;
}
```

### When to Use

- ✅ Simple hover effects
- ✅ Page transitions
- ✅ Basic UI feedback
- ✅ Prototypes and demos
- ❌ Complex gesture interactions
- ❌ Advanced animation sequences
- ❌ Performance monitoring needed

## Standard Animations

**Best for**: Most production applications, interactive UIs, complex animations

**Bundle Size**: ~75KB (production preset)
**Complexity**: Medium
**Performance**: Excellent (Rust + WASM)

### Quick Start

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn StandardAnimation() -> impl IntoView {
    let (is_animating, set_is_animating) = create_signal(false);

    view! {
        <MotionDiv
            initial=AnimationValue::Opacity(0.0)
            animate=move || if is_animating() {
                AnimationValue::Opacity(1.0)
            } else {
                AnimationValue::Opacity(0.0)
            }
            transition=Transition {
                duration: 0.6,
                easing: Easing::EaseOut,
                ..Default::default()
            }
            class="w-32 h-32 bg-blue-500 rounded-lg"
        >
            <button on:click=move |_| set_is_animating(!is_animating())>
                "Toggle Animation"
            </button>
        </MotionDiv>
    }
}
```

### Key Features

- **Motion Components**: `MotionDiv`, `MotionButton`, `MotionMain`
- **Animation Values**: Opacity, scale, rotation, color, string
- **Transitions**: Duration, easing, delay, repeat
- **Gestures**: Hover, tap, drag with constraints
- **Layout Animations**: FLIP animations for layout changes

### When to Use

- ✅ Interactive applications
- ✅ Complex animation sequences
- ✅ Gesture-based interactions
- ✅ Layout animations
- ✅ Production applications
- ❌ Simple CSS-only effects
- ❌ Extreme bundle size constraints

## Advanced Animations

**Best for**: Enterprise applications, games, data visualization, performance-critical apps

**Bundle Size**: ~125KB (standard preset)
**Complexity**: High
**Performance**: Excellent (optimized Rust + WASM)

### Quick Start

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn AdvancedAnimation() -> impl IntoView {
    let (performance_metrics, set_performance_metrics) = create_signal(None);

    // Performance monitoring
    let monitor_performance = move || {
        let metrics = get_performance_metrics();
        set_performance_metrics(Some(metrics));
    };

    view! {
        <MotionDiv
            initial=AnimationValue::Opacity(0.0)
            animate=AnimationValue::Opacity(1.0)
            transition=Transition {
                duration: 0.6,
                easing: Easing::EaseOut,
                ..Default::default()
            }
            while_hover=AnimationValue::Scale(1.1)
            while_tap=AnimationValue::Scale(0.95)
            drag=true
            drag_constraints=DragConstraints::Parent
            on_drag_start=move |_| monitor_performance()
            class="w-32 h-32 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg cursor-grab"
        >
            <div class="text-white text-center p-4">
                "Drag me!"
            </div>
        </MotionDiv>

        <Show when=move || performance_metrics().is_some()>
            <div class="mt-4 p-4 bg-gray-100 rounded">
                <h3>"Performance Metrics"</h3>
                <pre>{move || format!("{:#?}", performance_metrics())}</pre>
            </div>
        </Show>
    }
}
```

### Advanced Features

- **Performance Monitoring**: FPS tracking, memory usage, animation timing
- **Multi-touch Gestures**: Complex gesture recognition and handling
- **Feature Flags**: Granular control over functionality
- **Lazy Loading**: On-demand module loading
- **Code Splitting**: Dynamic imports for optimal bundle size
- **Advanced Layout**: FLIP animations, shared element transitions
- **Custom Easing**: Bezier curves, spring physics
- **Animation Sequences**: Staggered animations, keyframes

### When to Use

- ✅ Enterprise applications
- ✅ Performance-critical applications
- ✅ Complex gesture interactions
- ✅ Data visualization
- ✅ Games and interactive media
- ✅ Applications requiring monitoring
- ❌ Simple websites
- ❌ Basic prototypes

## Migration Guide

### From CSS-only to Standard

```rust
// Before (CSS-only)
<div class="fade-in">"Content"</div>

// After (Standard)
<MotionDiv
    initial=AnimationValue::Opacity(0.0)
    animate=AnimationValue::Opacity(1.0)
    transition=Transition { duration: 0.6, ..Default::default() }
>
    "Content"
</MotionDiv>
```

### From Standard to Advanced

```rust
// Before (Standard)
<MotionDiv animate=AnimationValue::Opacity(1.0)>
    "Content"
</MotionDiv>

// After (Advanced)
<MotionDiv
    animate=AnimationValue::Opacity(1.0)
    while_hover=AnimationValue::Scale(1.1)
    drag=true
    on_drag_start=move |_| log_performance()
>
    "Content"
</MotionDiv>
```

## Performance Comparison

| Approach     | Bundle Size | Performance | Complexity | Use Case          |
| ------------ | ----------- | ----------- | ---------- | ----------------- |
| **CSS-only** | ~15KB       | Excellent   | Low        | Simple effects    |
| **Standard** | ~75KB       | Excellent   | Medium     | Most applications |
| **Advanced** | ~125KB      | Excellent   | High       | Enterprise apps   |

## Recommendations

### Choose CSS-only when:

- Building simple websites or prototypes
- Bundle size is critical (<20KB)
- You only need basic animations
- Team has limited Rust experience

### Choose Standard when:

- Building production applications
- You need interactive animations
- Bundle size is moderate (50-100KB acceptable)
- You want the best of both worlds

### Choose Advanced when:

- Building enterprise applications
- Performance monitoring is required
- Complex gesture interactions needed
- Bundle size is not a primary concern

## Getting Help

- **Documentation**: [docs/](./) - Comprehensive guides and API reference
- **Examples**: [examples/](../examples/) - Working code samples
- **Issues**: [GitHub Issues](https://github.com/your-repo/leptos-motion/issues) - Bug reports and feature requests
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/leptos-motion/discussions) - Community support

## Next Steps

1. **Choose your approach** based on your use case
2. **Follow the quick start guide** for your chosen approach
3. **Explore examples** in the [examples/](../examples/) directory
4. **Read the full documentation** for advanced features
5. **Join the community** for support and feedback
