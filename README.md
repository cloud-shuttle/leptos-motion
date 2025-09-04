# Leptos Motion 🎬

[![Crates.io](https://img.shields.io/crates/v/leptos-motion)](https://crates.io/crates/leptos-motion)
[![Documentation](https://img.shields.io/docsrs/leptos-motion)](https://docs.rs/leptos-motion)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**High-performance animation library for [Leptos](https://github.com/leptos-rs/leptos) with Framer Motion-inspired API**

Leptos Motion brings smooth, performant animations to your Leptos applications with a familiar API that feels like home for React developers. Built with Rust and WebAssembly for maximum performance.

> **🚀 Now in Beta!** Version 0.2.0-beta.1 is ready for testing and feedback.

## 🚀 Beta Release Status

**Version 0.2.0-beta.1** is now available for testing and feedback!

- ✅ **All Core Features Working**: Animation engine, gestures, layout animations
- ✅ **Comprehensive Testing**: 70+ tests passing with full coverage
- ✅ **Performance Optimized**: 60fps animations with memory management
- ✅ **Ready for Feedback**: Stable enough for development and testing

> **Note**: This is a beta release. Some APIs may change before the 1.0 stable release.

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
leptos-motion = "0.2.0-beta.1"
```

## 🚀 Quick Start

### Basic Animation

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn AnimatedButton() -> impl IntoView {
    let (is_hovered, set_is_hovered) = create_signal(false);
    
    let button_style = Motion::new(
        is_hovered,
        |hovered| if hovered {
            "scale: 1.1; background-color: #3b82f6;"
        } else {
            "scale: 1.0; background-color: #1f2937;"
        }
    );

    view! {
        <motion_button
            style=button_style
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
            class="px-4 py-2 rounded-lg text-white font-medium transition-colors"
        >
            "Hover me!"
        </motion_button>
    }
}
```

### Spring Animation

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn SpringBox() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    
    let box_style = Motion::spring(
        is_open,
        |open| if open {
            "width: 300px; height: 200px;"
        } else {
            "width: 100px; height: 100px;"
        },
        SpringConfig::default()
            .stiffness(100.0)
            .damping(10.0)
    );

    view! {
        <div class="space-y-4">
            <motion_div
                style=box_style
                class="bg-blue-500 rounded-lg shadow-lg"
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
    let (position, set_position) = create_signal((0.0, 0.0));
    
    let card_style = Motion::spring(
        position,
        |(x, y)| format!("transform: translate({}px, {}px);", x, y),
        SpringConfig::default().stiffness(200.0)
    );

    view! {
        <motion_div
            style=card_style
            class="w-64 h-40 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl shadow-xl cursor-grab active:cursor-grabbing"
            on:drag=move |event| {
                if let Some((x, y)) = event.detail {
                    set_position.set((x, y));
                }
            }}
        >
            <div class="p-6 text-white">
                <h3 class="text-xl font-bold">"Draggable Card"</h3>
                <p class="text-purple-100">"Drag me around!"</p>
            </div>
        </motion_div>
    }
}
```

### Layout Animations

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn AnimatedList() -> impl IntoView {
    let (items, set_items) = create_signal(vec![1, 2, 3, 4, 5]);
    
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
            
            <motion_ul class="space-y-2">
                <For
                    each=items
                    key=|item| *item
                    children=move |item| {
                        let id = item;
                        view! {
                            <motion_li
                                initial="hidden"
                                animate="visible"
                                exit="hidden"
                                variants=LayoutVariants::new()
                                    .hidden("opacity: 0; transform: translateX(-20px);")
                                    .visible("opacity: 1; transform: translateX(0);")
                                class="p-3 bg-gray-100 rounded-lg flex justify-between items-center"
                            >
                                <span>"Item {id}"</span>
                                <button
                                    on:click=move |_| remove_item(id)
                                    class="px-2 py-1 bg-red-500 text-white rounded text-sm"
                                >
                                    "Remove"
                                </button>
                            </motion_li>
                        }
                    }
                />
            </motion_ul>
        </div>
    }
}
```

## 🎨 Animation Types

### Spring Animations
Natural, physics-based animations that feel organic and responsive:

```rust
let spring_style = Motion::spring(
    signal,
    |value| format!("transform: scale({})", value),
    SpringConfig::default()
        .stiffness(100.0)    // Higher = faster
        .damping(10.0)        // Lower = more bouncy
        .mass(1.0)            // Higher = more inertia
);
```

### Tween Animations
Smooth, controlled animations with custom easing:

```rust
let tween_style = Motion::tween(
    signal,
    |value| format!("opacity: {}", value),
    TweenConfig::default()
        .duration(Duration::from_millis(500))
        .easing(Easing::ease_in_out())
);
```

### Custom Easing
Create your own easing functions:

```rust
let custom_style = Motion::tween(
    signal,
    |value| format!("transform: translateY({}px)", value),
    TweenConfig::default()
        .duration(Duration::from_millis(800))
        .easing(Easing::custom(|t| t * t * (3.0 - 2.0 * t)))
);
```

## 🖱️ Gesture Support

### Drag Gestures
```rust
<motion_div
    on:drag=move |event| {
        // Handle drag events
        if let Some((x, y)) = event.detail {
            // Update position
        }
    }}
    drag_constraints=DragConstraints::parent()
    drag_elastic=0.7
>
    "Draggable content"
</motion_div>
```

### Hover Gestures
```rust
<motion_div
    on:hover_start=move |_| set_is_hovered.set(true)
    on:hover_end=move |_| set_is_hovered.set(false)
    while_hover="scale: 1.05;"
>
    "Hover me!"
</motion_div>
```

### Tap Gestures
```rust
<motion_div
    on:tap=move |_| handle_tap()
    while_tap="scale: 0.95;"
>
    "Tap me!"
</motion_div>
```

## 📱 Layout Animations

Automatically animate layout changes with the `layout` prop:

```rust
<motion_div
    layout
    class="grid grid-cols-3 gap-4"
>
    <For
        each=items
        key=|item| item.id
        children=move |item| {
            view! {
                <motion_div
                    layout
                    class="p-4 bg-white rounded-lg shadow"
                >
                    {item.content}
                </motion_div>
            }
        }
    />
</motion_div>
```

## 🎭 Presence Animations

Handle enter/exit animations automatically:

```rust
<AnimatePresence>
    {move || if show_modal() {
        Some(view! {
            <motion_div
                initial="hidden"
                animate="visible"
                exit="hidden"
                variants=ModalVariants::new()
                    .hidden("opacity: 0; scale: 0.8;")
                    .visible("opacity: 1; scale: 1;")
                class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
            >
                <div class="bg-white p-6 rounded-lg">
                    "Modal content"
                </div>
            </motion_div>
        })
    } else {
        None
    }}
</AnimatePresence>
```

## 🔧 Advanced Usage

### Custom Animation Hooks
```rust
use leptos_motion::hooks::use_motion;

#[component]
pub fn CustomAnimation() -> impl IntoView {
    let (value, set_value) = create_signal(0.0);
    
    let motion_value = use_motion(
        value,
        |v| format!("transform: rotate({}deg)", v * 360.0),
        SpringConfig::default()
    );

    view! {
        <div class="space-y-4">
            <motion_div
                style=motion_value
                class="w-20 h-20 bg-blue-500 rounded-lg"
            />
            <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                on:input=move |event| {
                    if let Ok(value) = event.target().unwrap().value().parse::<f64>() {
                        set_value.set(value);
                    }
                }}
            />
        </div>
    }
}
```

### Performance Optimization
```rust
// Use `should_render` to prevent unnecessary re-renders
let expensive_style = Motion::spring(
    expensive_signal,
    |value| expensive_calculation(value),
    SpringConfig::default()
).should_render(move |prev, curr| (prev - curr).abs() > 0.01);

// Use `throttle` for high-frequency updates
let throttled_style = Motion::spring(
    high_freq_signal,
    |value| format!("transform: translateX({}px)", value),
    SpringConfig::default()
).throttle(Duration::from_millis(16)); // 60fps
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