# Leptos Motion

A comprehensive animation library for Rust and Leptos, providing Motion-inspired animation capabilities with type safety and performance.

## 🚀 Features

- **Declarative Animations**: React Motion-style API for Leptos components
- **Hardware Accelerated**: Uses Web Animations API with RAF fallback
- **Gesture System**: Built-in drag, hover, tap, and pan gestures
- **Layout Animations**: FLIP-technique layout transitions
- **Scroll Animations**: Parallax and scroll-triggered effects
- **Type Safe**: Full Rust type safety with zero runtime surprises
- **Performance First**: 60fps for 100+ concurrent animations

## 📖 Documentation

### Architecture & Design
- [**Design Document**](docs/design.md) - Comprehensive implementation design and architecture
- [**Implementation Plan**](docs/implementation_plan.md) - 16-week development roadmap
- [**Testing Strategy**](docs/testing_strategy.md) - Comprehensive test approach

### Core Concepts

#### Animation Engine
- Hybrid animation system (WAAPI + RAF)
- Spring physics simulation
- Easing functions and bezier curves
- Motion values with reactive updates

#### Component System
```rust
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MotionDiv
            animate=hashmap!{
                "x" => AnimationValue::Pixels(100.0),
                "rotate" => AnimationValue::Degrees(360.0),
            }
            transition=Transition {
                duration: Some(2.0),
                ease: Easing::Spring(SpringConfig::default()),
                ..Default::default()
            }
        >
            "Animated content"
        </MotionDiv>
    }
}
```

#### Gesture System
```rust
<MotionDiv
    while_hover=hashmap!{"scale" => AnimationValue::Number(1.1)}
    while_tap=hashmap!{"scale" => AnimationValue::Number(0.9)}
    drag=Some(DragConfig {
        axis: Some(DragAxis::Both),
        constraints: Some(DragConstraints {
            left: -100.0,
            right: 100.0,
            top: -100.0,
            bottom: 100.0,
        }),
        ..Default::default()
    })
>
    "Interactive element"
</MotionDiv>
```

#### Exit Animations
```rust
<AnimatePresence mode=PresenceMode::Sync>
    <For
        each=items
        key=|item| *item
        children=move |item| view! {
            <MotionDiv
                key=item.to_string()
                initial=hashmap!{"opacity" => AnimationValue::Number(0.0)}
                animate=hashmap!{"opacity" => AnimationValue::Number(1.0)}
                exit=hashmap!{"opacity" => AnimationValue::Number(0.0)}
            >
                {format!("Item {}", item)}
            </MotionDiv>
        }
    />
</AnimatePresence>
```

## 🏗️ Project Structure

```
leptos-motion/
├── docs/                        # Comprehensive documentation
│   ├── design.md               # Architecture and design
│   ├── implementation_plan.md  # Development roadmap
│   └── testing_strategy.md     # Test approach
├── crates/                     # Modular crate structure
│   ├── leptos-motion-core/     # Core animation engine
│   ├── leptos-motion-dom/      # DOM components
│   ├── leptos-motion-gestures/ # Gesture recognition
│   ├── leptos-motion-layout/   # Layout animations
│   ├── leptos-motion-scroll/   # Scroll animations
│   └── leptos-motion-macros/   # Procedural macros
├── examples/                   # Example applications
│   ├── basic-animations/       # Basic animation examples
│   ├── gestures/              # Gesture examples
│   ├── layout-animations/     # Layout transition examples
│   ├── scroll-effects/        # Scroll animation examples
│   └── showcase/              # Complete showcase app
└── tests/                     # Comprehensive test suite
    ├── integration/           # Integration tests
    ├── e2e/                   # End-to-end tests
    └── visual/               # Visual regression tests
```

## 🎯 Implementation Status

### Phase 1: Foundation ✅
- [x] Core animation engine architecture
- [x] Motion value system design
- [x] Basic component framework
- [x] Type system definitions

### Phase 2: Core Features 🚧
- [ ] RAF-based animation engine
- [ ] Web Animations API integration
- [ ] Spring physics implementation
- [ ] Basic motion components

### Phase 3: Gestures 📋
- [ ] Hover and tap gestures
- [ ] Drag functionality
- [ ] Pan and pinch gestures
- [ ] Gesture composition

### Phase 4: Advanced Features 📋
- [ ] AnimatePresence component
- [ ] Layout animation system
- [ ] Scroll-triggered animations
- [ ] Variants and orchestration

### Phase 5: Optimization 📋
- [ ] Performance optimizations
- [ ] Animation batching
- [ ] Memory management
- [ ] Bundle size optimization

## 🧪 Testing Approach

### Multi-Layer Testing Strategy
- **Unit Tests**: Core logic and mathematical functions (>90% coverage)
- **Integration Tests**: Component and animation integration
- **E2E Tests**: Real browser interactions and workflows
- **Performance Tests**: Frame rate and memory benchmarks
- **Visual Tests**: Screenshot regression testing

### Testing Tools
- `wasm-bindgen-test` for WASM unit tests
- `Playwright` for cross-browser E2E testing  
- `Criterion` for performance benchmarking
- `Percy`/`BackstopJS` for visual regression
- `Proptest` for property-based testing

## 📊 Performance Targets

- **Bundle Size**: <30KB core, <50KB full
- **Frame Rate**: 60fps for 100+ simultaneous animations
- **Memory**: <10MB for typical applications
- **Startup**: <100ms initialization time
- **API Coverage**: 90% parity with Motion

## 🛠️ Development

### Prerequisites
```bash
# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install testing tools
cargo install wasm-pack cargo-nextest
npm install -g playwright @percy/cli
```

### Build and Test
```bash
# Run all tests
cargo test --all-features
wasm-pack test --headless --chrome

# Run benchmarks
cargo bench

# Build examples
cargo build --examples --target wasm32-unknown-unknown

# Run visual tests
npm run test:visual
```

## 🗺️ Roadmap

### v0.1.0 Alpha (4 months)
- Core animation engine
- Basic motion components
- Essential gestures
- Documentation website

### v0.2.0 Beta (6 months)
- Layout animations
- Scroll effects
- Performance optimizations
- Extended gesture system

### v1.0.0 Stable (8 months)
- Production-ready API
- Comprehensive examples
- Full test coverage
- Migration tools

### Post-1.0 Features
- 3D transforms
- SVG animations
- Timeline editor
- WebGPU acceleration

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Code style and conventions
- Development workflow
- Testing requirements
- Pull request process

## 📄 License

Licensed under either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Inspired by [Framer Motion](https://www.framer.com/motion/) for React
- Built on the [Leptos](https://leptos.dev) framework
- Leveraging Rust's performance and safety guarantees

---

**Status**: 🚧 In Development | **Version**: Pre-alpha | **Last Updated**: 2024

For questions, feedback, or contributions, please [open an issue](https://github.com/cloud-shuttle/leptos-motion/issues) or join our community discussions.