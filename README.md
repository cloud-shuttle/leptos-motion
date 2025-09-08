# Leptos Motion 🎬

[![Crates.io](https://img.shields.io/crates/v/leptos-motion)](https://crates.io/crates/leptos-motion)
[![Documentation](https://img.shields.io/docsrs/leptos-motion)](https://docs.rs/leptos-motion)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**High-performance animation library for [Leptos](https://github.com/leptos-rs/leptos) with Framer Motion-inspired API**

Leptos Motion brings smooth, performant animations to your Leptos applications with a familiar API that feels like home for React developers. Built with Rust and WebAssembly for maximum performance.

> **🚀 Latest Release!** Version 0.5.0 with continuous momentum animations is now available.

## 🎉 Latest Release Status

**Version 0.5.0** is now available with major momentum animation improvements!

- ✅ **Solid Foundation**: Core animation engine, gestures, layout animations
- ✅ **Comprehensive Testing**: 100+ tests passing with full coverage
- ✅ **Type Safety**: Full Rust compile-time guarantees
- ✅ **Simplified APIs**: Clean, user-friendly interfaces
- ✅ **All Examples Working**: Advanced features, mobile app, dashboard, e-commerce
- ✅ **API Compatibility**: Consistent and stable API across all components
- ✅ **Production Ready**: Optimized for real-world applications
- ✅ **Continuous Momentum**: Fully functional momentum animations with physics simulation
- ✅ **Enhanced MotionDiv**: New style prop support for flexible styling

> **Note**: This is a stable release ready for production use.

## ✨ Features

- 🚀 **High Performance**: Built with Rust and WebAssembly for 60fps animations
- 🎯 **Framer Motion Inspired**: Familiar API for React developers
- 🎨 **Rich Animation Types**: Spring, tween, and custom easing functions
- 🖱️ **Gesture Support**: Multi-touch, drag, hover, tap, and scroll animations
- 📱 **Layout Animations**: FLIP-based smooth transitions when elements change position
- 🎭 **Presence Animations**: Enter/exit animations with automatic cleanup
- 🔧 **Type Safe**: Full Rust type safety and compile-time guarantees
- 🌐 **Cross Platform**: Works in browsers and server-side rendering

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-motion = "0.3.0"
```

## 🚀 Quick Start

### Basic Animation

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn AnimatedButton() -> impl IntoView {
    view! {
        <MotionDiv
            class="px-4 py-2 rounded-lg text-white font-medium bg-blue-600".to_string()
            initial=motion_target!(
                "scale" => AnimationValue::Number(1.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            transition=Transition {
                duration: Some(0.2),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Hover me!"
        </MotionDiv>
    }
}
```

### Spring Animation

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn SpringBox() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    
    view! {
        <div class="space-y-4">
            <MotionDiv
                class="bg-blue-500 rounded-lg shadow-lg".to_string()
                animate=motion_target!(
                    "width" => AnimationValue::Pixels(if is_open.get() { 300.0 } else { 100.0 }),
                    "height" => AnimationValue::Pixels(if is_open.get() { 200.0 } else { 100.0 })
                )
                transition=Transition {
                    duration: Some(0.6),
                    ease: Easing::Spring(SpringConfig::default()
                        .stiffness(100.0)
                        .damping(10.0)
                    ),
                    ..Default::default()
                }
            />
            <button
                on:click=move |_| set_is_open.update(|x| *x = !*x)
                class="px-4 py-2 bg-gray-800 text-white rounded-lg"
            >
                "Toggle Size"
            </button>
        </div>
    }
}
```

### Gesture Animations

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn DraggableCard() -> impl IntoView {
    view! {
        <MotionDiv
            class="w-64 h-40 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl shadow-xl cursor-grab active:cursor-grabbing".to_string()
            drag=DragConfig::default()
            while_drag=motion_target!(
                "scale" => AnimationValue::Number(1.05)
            )
            drag_constraints=DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: Some(-100.0),
                bottom: Some(100.0),
            }
        >
            <div class="p-6 text-white">
                <h3 class="text-xl font-bold">"Draggable Card"</h3>
                <p class="text-purple-100">"Drag me around!"</p>
            </div>
        </MotionDiv>
    }
}
```

### Layout Animations

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn AnimatedList() -> impl IntoView {
    let (items, set_items) = signal(vec![1, 2, 3, 4, 5]);
    
    let add_item = move |_| {
        set_items.update(|items| {
            let new_id = items.len() + 1;
            items.push(new_id);
        });
    };
    
    let remove_item = move |id| {
        set_items.update(|items| {
            items.retain(|&x| x != id);
        });
    };

    view! {
        <div class="space-y-4">
            <button
                on:click=add_item
                class="px-4 py-2 bg-green-600 text-white rounded-lg"
            >
                "Add Item"
            </button>
            
            <ul class="space-y-2">
                <For
                    each=items
                    key=|item| *item
                    children=move |item| {
                        let id = item;
                        view! {
                            <MotionDiv
                                class="p-3 bg-gray-100 rounded-lg flex justify-between items-center".to_string()
                                key=id.to_string()
                                initial=motion_target!(
                                    "opacity" => AnimationValue::Number(0.0),
                                    "x" => AnimationValue::Pixels(-20.0)
                                )
                                animate=motion_target!(
                                    "opacity" => AnimationValue::Number(1.0),
                                    "x" => AnimationValue::Pixels(0.0)
                                )
                                exit=motion_target!(
                                    "opacity" => AnimationValue::Number(0.0),
                                    "x" => AnimationValue::Pixels(20.0)
                                )
                                transition=Transition {
                                    duration: Some(0.3),
                                    ease: Easing::EaseOut,
                                    ..Default::default()
                                }
                            >
                                <span>"Item {id}"</span>
                                <button
                                    on:click=move |_| remove_item(id)
                                    class="px-2 py-1 bg-red-500 text-white rounded text-sm"
                                >
                                    "Remove"
                                </button>
                            </MotionDiv>
                        }
                    }
                />
            </ul>
        </div>
    }
}
```

## 🎨 Animation Types

### Spring Animations
Natural, physics-based animations that feel organic and responsive:

```rust
<MotionDiv
    animate=motion_target!(
        "scale" => AnimationValue::Number(1.2)
    )
    transition=Transition {
        duration: Some(0.6),
        ease: Easing::Spring(SpringConfig::default()
            .stiffness(100.0)    // Higher = faster
            .damping(10.0)        // Lower = more bouncy
            .mass(1.0)            // Higher = more inertia
        ),
        ..Default::default()
    }
>
    "Spring Animation"
</MotionDiv>
```

### Tween Animations
Smooth, controlled animations with custom easing:

```rust
<MotionDiv
    animate=motion_target!(
        "opacity" => AnimationValue::Number(0.5)
    )
    transition=Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        ..Default::default()
    }
>
    "Tween Animation"
</MotionDiv>
```

### Custom Easing
Use built-in easing functions or create custom ones:

```rust
<MotionDiv
    animate=motion_target!(
        "y" => AnimationValue::Pixels(100.0)
    )
    transition=Transition {
        duration: Some(0.8),
        ease: Easing::EaseOut,
        ..Default::default()
    }
>
    "Custom Easing"
</MotionDiv>
```

## 🖱️ Gesture Support

### Drag Gestures
```rust
<MotionDiv
    drag=DragConfig::default()
    drag_constraints=DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-100.0),
        bottom: Some(100.0),
    }
    while_drag=motion_target!(
        "scale" => AnimationValue::Number(1.05)
    )
>
    "Draggable content"
</MotionDiv>
```

### Hover Gestures
```rust
<MotionDiv
    while_hover=motion_target!(
        "scale" => AnimationValue::Number(1.05)
    )
    transition=Transition {
        duration: Some(0.2),
        ease: Easing::EaseOut,
        ..Default::default()
    }
>
    "Hover me!"
</MotionDiv>
```

### Tap Gestures
```rust
<MotionDiv
    while_tap=motion_target!(
        "scale" => AnimationValue::Number(0.95)
    )
    transition=Transition {
        duration: Some(0.1),
        ease: Easing::EaseOut,
        ..Default::default()
    }
>
    "Tap me!"
</MotionDiv>
```

## 📱 Layout Animations

Automatically animate layout changes with the `layout` prop:

```rust
<MotionDiv
    layout=true
    class="grid grid-cols-3 gap-4".to_string()
>
    <For
        each=items
        key=|item| item.id
        children=move |item| {
            view! {
                <MotionDiv
                    layout=true
                    class="p-4 bg-white rounded-lg shadow".to_string()
                >
                    {item.content}
                </MotionDiv>
            }
        }
    />
</MotionDiv>
```

## 🎭 Presence Animations

Handle enter/exit animations automatically:

```rust
<AnimatePresence>
    {move || if show_modal() {
        Some(view! {
            <MotionDiv
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "scale" => AnimationValue::Number(0.8)
                )
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "scale" => AnimationValue::Number(1.0)
                )
                exit=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "scale" => AnimationValue::Number(0.8)
                )
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
                class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center".to_string()
            >
                <div class="bg-white p-6 rounded-lg">
                    "Modal content"
                </div>
            </MotionDiv>
        })
    } else {
        None
    }}
</AnimatePresence>
```

## 🔧 Advanced Usage

### Animation Presets
Use built-in animation presets for common patterns:

```rust
use leptos_motion_core::AnimationPresets;

<MotionDiv
    initial=AnimationPresets::fade_in().initial
    animate=AnimationPresets::fade_in().animate
    transition=AnimationPresets::fade_in().transition
    class="animated-element".to_string()
>
    "Fade In Animation"
</MotionDiv>
```

### Keyframe Animations
Create complex multi-step animations:

```rust
use leptos_motion_core::animation::Keyframes;

let keyframes = Keyframes::new()
    .add_keyframe(0.0, motion_target!("opacity" => AnimationValue::Number(0.0)))
    .add_keyframe(0.5, motion_target!("opacity" => AnimationValue::Number(0.5)))
    .add_keyframe(1.0, motion_target!("opacity" => AnimationValue::Number(1.0)));

<MotionDiv
    animate=keyframes.to_animation_target()
    transition=Transition {
        duration: Some(2.0),
        ease: Easing::EaseInOut,
        ..Default::default()
    }
    class="keyframe-demo".to_string()
>
    "Keyframe Animation"
</MotionDiv>
```

### Performance Optimization
```rust
// Use layout animations for smooth position changes
<MotionDiv
    layout=true
    class="performance-optimized".to_string()
>
    "Layout Animation"
</MotionDiv>

// Use appropriate easing for performance
<MotionDiv
    transition=Transition {
        duration: Some(0.3),
        ease: Easing::EaseOut, // Hardware accelerated
        ..Default::default()
    }
    class="hardware-accelerated".to_string()
>
    "Hardware Accelerated"
</MotionDiv>
```

## 🚀 Performance Features

- **WebAssembly**: Rust compiled to WASM for near-native performance
- **GPU Acceleration**: Automatic hardware acceleration when available
- **Frame Throttling**: Built-in 60fps limiting for smooth animations
- **Memory Management**: Efficient memory usage with automatic cleanup
- **Tree Shaking**: Only include the features you use

## 🌐 Browser Support

- **Modern Browsers**: Chrome 67+, Firefox 60+, Safari 11.1+, Edge 79+
- **WebAssembly**: Full WASM support required
- **CSS Grid/Flexbox**: For layout animations
- **Touch Events**: For mobile gesture support

## 📚 Documentation

- [API Reference](https://docs.rs/leptos-motion)
- [Examples](https://github.com/cloud-shuttle/leptos-motion/tree/main/examples)
- [Migration Guide](https://github.com/cloud-shuttle/leptos-motion/blob/main/docs/migration/framer-motion.md)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/cloud-shuttle/leptos-motion/blob/main/CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/cloud-shuttle/leptos-motion/blob/main/LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [Framer Motion](https://www.framer.com/motion/)
- Built with [Leptos](https://github.com/leptos-rs/leptos)
- Powered by Rust and WebAssembly

---

**Made with ❤️ by the Leptos Motion team**

[GitHub](https://github.com/cloud-shuttle/leptos-motion) • [Issues](https://github.com/cloud-shuttle/leptos-motion/issues) • [Discussions](https://github.com/cloud-shuttle/leptos-motion/discussions)