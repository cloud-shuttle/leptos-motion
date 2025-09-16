# Leptos Motion

A high-performance animation library for Rust web applications using the Leptos framework. Built with WebAssembly for native-speed animations and a reactive API for seamless integration with Leptos.

## 🚀 Features

- **WASM-Powered**: Native-speed animations compiled to WebAssembly
- **Reactive API**: Seamless integration with Leptos signals and effects
- **High Performance**: 60fps animations with optimized rendering
- **Type Safe**: Full Rust type safety with compile-time guarantees
- **Flexible**: Support for complex transforms, drag interactions, and custom animations
- **Modern**: Built for the modern web with CSS transforms and opacity

## 📁 Project Structure

This repository is organized for clarity and maintainability:

- **`crates/`** - Core library crates (leptos-motion-core, leptos-motion-dom, etc.)
- **`demos/`** - Organized demonstration applications
  - `demos/basic/` - Simple demos for getting started
  - `demos/advanced/` - Complex demos showcasing advanced features
  - `demos/showcase/` - Comprehensive feature demonstrations
  - `demos/legacy/` - Legacy demos for reference
- **`examples/`** - Simple code examples and templates
- **`tests/`** - Organized test suites (unit, integration, e2e)
- **`docs/`** - Comprehensive documentation
  - `docs/analysis/` - Technical analysis and comparisons
  - `docs/roadmaps/` - Development roadmaps and plans
  - `docs/releases/` - Release notes and summaries
  - `docs/status/` - Project status and issue tracking

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr"] }
leptos-motion-dom = { version = "0.9.0" }
wasm-bindgen = "0.2"
```

## 🎬 Quick Start

```rust
use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div::ReactiveMotionDiv;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
fn App() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    // Update animation signal when scale changes
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("scale".to_string(), AnimationValue::Number(scale.get()));
        set_animate_signal.set(animations);
    });

    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("scale".to_string(), AnimationValue::Number(1.0));
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
        <div>
            <h1>"Leptos Motion Demo"</h1>
            <ReactiveMotionDiv
                initial=initial_values
                animate=animate_signal
                transition=transition
            >
                <div
                    style="width: 100px; height: 100px; background: #4ecdc4; border-radius: 8px; cursor: pointer;"
                    on:click=move |_| {
                        set_scale.set(if scale.get() == 1.0 { 1.5 } else { 1.0 });
                    }
                >
                    "Click me!"
                </div>
            </ReactiveMotionDiv>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
```

## 🎯 Core Components

### ReactiveMotionDiv

The main animation component that provides reactive animations based on Leptos signals.

```rust
<ReactiveMotionDiv
    initial=initial_values
    animate=animate_signal
    transition=transition
>
    <div>"Your content here"</div>
</ReactiveMotionDiv>
```

### DragMotionDiv

A component that provides drag functionality with constraints and momentum.

```rust
<DragMotionDiv drag=drag_config>
    <div>"Draggable content"</div>
</DragMotionDiv>
```

## 🎨 Animation Types

### Transform Animations

```rust
// Scale
animations.insert("transform".to_string(), AnimationValue::String("scale(1.5)".to_string()));

// Rotation
animations.insert("transform".to_string(), AnimationValue::String("rotate(45deg)".to_string()));

// Translation
animations.insert("transform".to_string(), AnimationValue::String("translate(10px, 20px)".to_string()));

// Complex transforms
animations.insert("transform".to_string(), AnimationValue::String("translate(10px, 20px) scale(1.5) rotate(45deg)".to_string()));
```

### Property Animations

```rust
// Opacity
animations.insert("opacity".to_string(), AnimationValue::Number(0.8));

// Width
animations.insert("width".to_string(), AnimationValue::Pixels(200.0));

// Color
animations.insert("background-color".to_string(), AnimationValue::Color("#ff6b6b".to_string()));
```

## ⚡ Performance

Leptos Motion is built for performance:

- **WASM Compilation**: Native-speed animations
- **Optimized Rendering**: Uses CSS transforms and opacity for hardware acceleration
- **Reactive Updates**: Only updates when signals change
- **Frame Rate Control**: Built-in 60fps targeting

### Performance Benchmarking

Run the performance demo to test animation performance:

```bash
cd performance-demo
wasm-pack build --target web --out-dir pkg
python3 -m http.server 8000
# Open http://localhost:8000/performance-demo/index.html
```

## 📚 Documentation

- [API Reference](docs/API_REFERENCE.md) - Complete API documentation
- [Usage Guide](docs/USAGE_GUIDE.md) - Comprehensive usage examples
- [Examples](examples/) - Working examples and demos

## 🎮 Examples

### Basic Animation

```rust
#[component]
fn BasicAnimation() -> impl IntoView {
    let (is_hovered, set_hovered) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_hovered.get() {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1.1)".to_string()));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        }
        set_animate_signal.set(animations);
    });
    
    view! {
        <ReactiveMotionDiv
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <div>"Hover me!"</div>
        </ReactiveMotionDiv>
    }
}
```

## 🎬 Running Demos

Try out the interactive demos to see leptos-motion in action:

```bash
# Basic reactive animation demo
cd demos/basic/reactive-demo
trunk serve --open

# Comprehensive showcase
cd demos/showcase/comprehensive-demo
trunk serve --open

# WebGL 3D animations
cd demos/advanced/webgl-demo
trunk serve --open
```

### Drag and Drop

```rust
#[component]
fn DraggableItem() -> impl IntoView {
    let drag_config = DragConfig {
        enabled: true,
        constraints: Some(DragConstraints {
            min_x: Some(-200.0),
            max_x: Some(200.0),
            min_y: Some(-200.0),
            max_y: Some(200.0),
        }),
        momentum: Some(DragMomentum {
            enabled: true,
            damping: 0.8,
            stiffness: 0.1,
        }),
    };
    
    view! {
        <DragMotionDiv drag=drag_config>
            <div style="width: 100px; height: 100px; background: #ff6b6b; border-radius: 8px; cursor: grab;">
                "Drag me!"
        </div>
        </DragMotionDiv>
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
                    <ReactiveMotionDiv
                        initial=HashMap::new()
                        animate=HashMap::new()
                        transition=transition
                    >
                        <div>{item}</div>
                    </ReactiveMotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

## 🛠️ Development

### Building Examples

```bash
# Build a specific example
cd examples/phase2-reactive-demo
wasm-pack build --target web --out-dir pkg

# Serve locally
python3 -m http.server 8000
# Open http://localhost:8000/phase2-reactive-demo/index.html
```

### Running Tests

```bash
cargo test
```

### Performance Testing

```bash
cd performance-demo
wasm-pack build --target web --out-dir pkg
python3 -m http.server 8000
# Open http://localhost:8000/performance-demo/index.html
```

## 📋 Available Examples

- **`simple-working-demo/`** - Basic reactive animations
- **`performance-demo/`** - Performance benchmarking
- **`phase2-reactive-demo/`** - Advanced reactive features

## 🔧 Configuration

### Transition Configuration

```rust
let transition = Transition {
    duration: Some(0.3),        // Duration in seconds
    delay: Some(0.0),          // Delay in seconds
    ease: Easing::EaseInOut,   // Easing function
    repeat: RepeatConfig::Never, // Repeat configuration
    stagger: None,             // Stagger delay in seconds
};
```

### Easing Functions

Available easing functions include:
- `Linear`, `EaseIn`, `EaseOut`, `EaseInOut`
- `EaseInSine`, `EaseOutSine`, `EaseInOutSine`
- `EaseInQuad`, `EaseOutQuad`, `EaseInOutQuad`
- `EaseInCubic`, `EaseOutCubic`, `EaseInOutCubic`
- `EaseInQuart`, `EaseOutQuart`, `EaseInOutQuart`
- `EaseInQuint`, `EaseOutQuint`, `EaseInOutQuint`
- `EaseInExpo`, `EaseOutExpo`, `EaseInOutExpo`
- `EaseInCirc`, `EaseOutCirc`, `EaseInOutCirc`
- `EaseInBack`, `EaseOutBack`, `EaseInOutBack`
- `EaseInElastic`, `EaseOutElastic`, `EaseInOutElastic`
- `EaseInBounce`, `EaseOutBounce`, `EaseInOutBounce`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on top of the amazing [Leptos](https://github.com/leptos-rs/leptos) framework
- Inspired by [Framer Motion](https://www.framer.com/motion/) for React
- Uses [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for WebAssembly integration

## 📞 Support

- GitHub Issues: [Report bugs or request features](https://github.com/your-org/leptos-motion/issues)
- Documentation: [API Reference](docs/API_REFERENCE.md) and [Usage Guide](docs/USAGE_GUIDE.md)
- Examples: [Working examples](examples/) and [demos](demos/)

---

**Leptos Motion** - High-performance animations for the modern web, powered by Rust and WebAssembly. 🦀✨