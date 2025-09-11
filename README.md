# Leptos Motion 🎬

[![Crates.io](https://img.shields.io/crates/v/leptos-motion)](https://crates.io/crates/leptos-motion)
[![Documentation](https://img.shields.io/docsrs/leptos-motion)](https://docs.rs/leptos-motion)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.89+-blue.svg)](https://www.rust-lang.org)
[![pnpm](https://img.shields.io/badge/package%20manager-pnpm-blue.svg)](https://pnpm.io/)

**Production-ready animation library for [Leptos](https://github.com/leptos-rs/leptos) with complete Motion Studio, WebGL acceleration, and Framer Motion-inspired API**

Leptos Motion brings smooth, performant animations to your Leptos applications with a familiar API that feels like home for React developers. Built with Rust and WebAssembly for maximum performance, featuring a complete animation studio, WebGL acceleration, and enterprise-grade development practices.

> **🚀 Latest Release!** Version 0.8.3 with complete Motion Studio and WebGL acceleration is now available.

## 🎉 Latest Release Status

**Version 0.8.3** is now available with complete Motion Studio, WebGL acceleration, and enterprise-grade development practices!

- ✅ **🎬 Complete Motion Studio**: Full-featured animation studio with timeline editor, 3D transforms, and path morphing
- ✅ **🚀 WebGL Acceleration**: GPU-accelerated animations with post-processing effects and physics simulation
- ✅ **📦 pnpm Migration**: Modern package management with pnpm as primary package manager
- ✅ **📋 Architecture Decision Records**: Comprehensive ADRs documenting development practices and standards
- ✅ **🧪 Near-100% Test Coverage**: Comprehensive testing pyramid with Playwright E2E testing
- ✅ **🔧 Latest Rust Practices**: Modern Rust toolchain with strict linting and formatting
- ✅ **🌊 Enhanced CubicBezier**: Custom curve support with advanced easing functions
- ✅ **📤 Export Functionality**: Support for CSS, WAAPI, Leptos Motion, and Framer Motion formats
- ✅ **🎯 Leptos v0.8.8 Compatibility**: Always supporting the latest stable Leptos version
- ✅ **⚡ Performance Optimizations**: Memory efficiency and zero-allocation animation loops
- ✅ **🌐 Cross-browser Testing**: Playwright automation for all demos and examples
- ✅ **📚 Comprehensive Documentation**: Complete ADRs, migration guides, and compatibility analysis

> **Note**: This is a production-ready release with enterprise-grade development practices and comprehensive animation capabilities.

## ✨ Features

- 🚀 **High Performance**: Built with Rust and WebAssembly for 60fps animations
- 🎯 **Framer Motion Inspired**: Familiar API for React developers
- 🎬 **Complete Motion Studio**: Full-featured animation studio with timeline editor, 3D transforms, and path morphing
- 🚀 **WebGL Acceleration**: GPU-accelerated animations with post-processing effects and physics simulation
- 🎨 **Rich Animation Types**: Spring, tween, CubicBezier, and custom easing functions
- 🖱️ **Gesture Support**: Multi-touch, drag, hover, tap, and scroll animations
- 📱 **Layout Animations**: FLIP-based smooth transitions when elements change position
- 🎭 **Presence Animations**: Enter/exit animations with automatic cleanup
- 🌊 **Spring Physics**: Natural motion with configurable tension, friction, and mass
- 👻 **AnimatePresence**: Advanced enter/exit animations with multiple presence modes
- 🎭 **Variants System**: Named animation states with smooth transitions
- ⏰ **Timeline Sequences**: Advanced orchestration for complex animation sequences
- 📤 **Export Functionality**: Support for CSS, WAAPI, Leptos Motion, and Framer Motion formats
- ⚡ **Performance Optimizations**: Memory pools, caching, and edge case handling
- 🔧 **Type Safe**: Full Rust type safety and compile-time guarantees
- 🌐 **Cross Platform**: Works in browsers and server-side rendering
- 📦 **Modern Tooling**: pnpm package management with comprehensive ADRs
- 🧪 **Enterprise Testing**: Near-100% test coverage with Playwright E2E testing

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-motion = "0.8.3"
leptos-motion-studio = "0.8.3"  # Optional: Complete animation studio
```

### Package Manager

We use **pnpm** as our primary package manager for optimal performance and disk efficiency:

```bash
# Install pnpm (if not already installed)
npm install -g pnpm

# Install dependencies
pnpm install
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

## 🎬 Motion Studio

The complete animation studio provides professional-grade tools for creating complex animations:

### Timeline Editor
```rust
use leptos_motion_studio::timeline::{TimelineEditor, AnimationProperty, AnimationValue};

#[component]
pub fn AnimationTimeline() -> impl IntoView {
    let (timeline, set_timeline) = create_signal(Timeline3D::new("My Animation".to_string(), 10.0));
    
    view! {
        <TimelineEditor
            timeline=timeline
            on_timeline_change=move |new_timeline| set_timeline.set(new_timeline)
        />
    }
}
```

### 3D Transforms
```rust
use leptos_motion_studio::transforms::{Transform3D, Transform3DEditor};

#[component]
pub fn Transform3DDemo() -> impl IntoView {
    let (transform, set_transform) = create_signal(Transform3D::new());
    
    view! {
        <Transform3DEditor
            transform=transform
            on_change=move |new_transform| set_transform.set(new_transform)
        />
    }
}
```

### SVG Path Morphing
```rust
use leptos_motion_studio::morphing::SvgMorphingEditor;

#[component]
pub fn PathMorphingDemo() -> impl IntoView {
    view! {
        <SvgMorphingEditor
            source_path="M 50 50 L 150 50 L 100 150 Z"
            target_path="M 50 100 L 150 100 L 100 50 Z"
            progress=0.5
        />
    }
}
```

## 🚀 WebGL Acceleration

GPU-accelerated animations with advanced post-processing effects:

```rust
use leptos_motion_studio::webgl::{WebGLRenderer, GPUAnimation};

#[component]
pub fn WebGLDemo() -> impl IntoView {
    let renderer = WebGLRenderer::new();
    
    view! {
        <div class="webgl-container">
            <GPUAnimation
                renderer=renderer
                effects=vec![
                    "bloom".to_string(),
                    "ssao".to_string(),
                    "tone_mapping".to_string()
                ]
            />
        </div>
    }
}
```

## 🌟 New in v0.8.3

### Complete Motion Studio

Professional-grade animation tools with timeline editor, 3D transforms, and path morphing:

### Enhanced CubicBezier Easing

Advanced easing functions with custom curve support:

```rust
use leptos_motion_core::easing::{Easing, CubicBezier};

<MotionDiv
    animate=motion_target!(
        "x" => AnimationValue::Pixels(200.0)
    )
    transition=Transition {
        duration: Some(0.6),
        ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0)),
        ..Default::default()
    }
>
    "Custom CubicBezier Animation"
</MotionDiv>
```

### Export Functionality

Export animations to multiple formats:

```rust
use leptos_motion_studio::export::{AnimationExporter, ExportFormat};

let exporter = AnimationExporter::new(&project);
let css_result = exporter.export(ExportFormat::CSS);
let waapi_result = exporter.export(ExportFormat::WAAPI);
let framer_result = exporter.export(ExportFormat::FramerMotion);
```

### pnpm Package Management

Modern package management with pnpm for optimal performance and disk efficiency.

### Architecture Decision Records

Comprehensive ADRs documenting our development practices and standards for enterprise-grade development.

### Spring Physics Engine

Create natural, physics-based animations with configurable spring parameters:

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn SpringPhysicsDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    view! {
        <div class="space-y-4">
            <MotionDiv
                class="w-20 h-20 bg-green-500 rounded-full".to_string()
                animate=motion_target!(
                    "x" => AnimationValue::Pixels(if is_active.get() { 200.0 } else { 0.0 })
                )
                transition=Transition {
                    duration: Some(1.0),
                    ease: Easing::Spring(SpringConfig::default()
                        .tension(300.0)
                        .friction(30.0)
                        .mass(1.0)
                    ),
                    ..Default::default()
                }
            />
            <button
                on:click=move |_| set_is_active.update(|x| *x = !*x)
                class="px-4 py-2 bg-green-600 text-white rounded-lg"
            >
                "Spring Animation"
            </button>
        </div>
    }
}
```

### AnimatePresence Component

Handle enter and exit animations for conditionally rendered elements:

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn PresenceDemo() -> impl IntoView {
    let (show, set_show) = signal(false);
    
    view! {
        <div class="space-y-4">
            <AnimatePresence>
                {move || if show.get() {
                    view! {
                        <MotionDiv
                            class="w-32 h-32 bg-blue-500 rounded-lg".to_string()
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
                        />
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </AnimatePresence>
            <button
                on:click=move |_| set_show.update(|x| *x = !*x)
                class="px-4 py-2 bg-blue-600 text-white rounded-lg"
            >
                "Toggle Presence"
            </button>
        </div>
    }
}
```

### Variants System

Define named animation states for smooth transitions:

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn VariantsDemo() -> impl IntoView {
    let (current_variant, set_current_variant) = signal("idle".to_string());
    
    let variants = {
        let mut v = HashMap::new();
        v.insert("idle".to_string(), motion_target!(
            "scale" => AnimationValue::Number(1.0),
            "rotate" => AnimationValue::Number(0.0)
        ));
        v.insert("hover".to_string(), motion_target!(
            "scale" => AnimationValue::Number(1.1),
            "rotate" => AnimationValue::Number(5.0)
        ));
        v.insert("tap".to_string(), motion_target!(
            "scale" => AnimationValue::Number(0.95),
            "rotate" => AnimationValue::Number(-5.0)
        ));
        v
    };
    
    view! {
        <div class="space-y-4">
            <MotionDiv
                class="w-24 h-24 bg-purple-500 rounded-lg cursor-pointer".to_string()
                variants=variants
                animate=current_variant
                on:mouseenter=move |_| set_current_variant.set("hover".to_string())
                on:mouseleave=move |_| set_current_variant.set("idle".to_string())
                on:click=move |_| set_current_variant.set("tap".to_string())
            />
            <div class="flex gap-2">
                <button
                    on:click=move |_| set_current_variant.set("idle".to_string())
                    class="px-3 py-1 bg-gray-600 text-white rounded text-sm"
                >
                    "Idle"
                </button>
                <button
                    on:click=move |_| set_current_variant.set("hover".to_string())
                    class="px-3 py-1 bg-gray-600 text-white rounded text-sm"
                >
                    "Hover"
                </button>
                <button
                    on:click=move |_| set_current_variant.set("tap".to_string())
                    class="px-3 py-1 bg-gray-600 text-white rounded text-sm"
                >
                    "Tap"
                </button>
            </div>
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
- [Motion Studio Guide](https://github.com/cloud-shuttle/leptos-motion/tree/main/crates/leptos-motion-studio)
- [WebGL Demo](https://github.com/cloud-shuttle/leptos-motion/tree/main/examples/webgl-demo)
- [Architecture Decision Records](https://github.com/cloud-shuttle/leptos-motion/tree/main/docs/adr)
- [Migration Guide](https://github.com/cloud-shuttle/leptos-motion/blob/main/MIGRATION_GUIDE.md)
- [Compatibility Analysis](https://github.com/cloud-shuttle/leptos-motion/blob/main/COMPATIBILITY_ANALYSIS.md)
- [Release Notes v0.8.3](https://github.com/cloud-shuttle/leptos-motion/blob/main/RELEASE_NOTES_v0.8.3.md)

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