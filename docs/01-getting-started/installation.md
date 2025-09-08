# Installation Guide

This guide covers how to install and set up Leptos Motion in your project.

## Prerequisites

Before installing Leptos Motion, ensure you have:

- **Rust 1.70+** - Latest stable version recommended
- **Cargo** - Comes with Rust
- **Leptos 0.8.8** - Compatible version
- **WebAssembly target** - For browser applications

### Installing Rust

If you don't have Rust installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

## Installation Methods

### Method 1: Cargo.toml (Recommended)

Add Leptos Motion to your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr", "hydrate", "ssr"] }
leptos-motion = "0.4.0"
```

### Method 2: Latest from Git

For the latest development version:

```toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr", "hydrate", "ssr"] }
leptos-motion = { git = "https://github.com/cloud-shuttle/leptos-motion" }
```

### Method 3: Specific Features

Install with specific features enabled:

```toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr", "hydrate", "ssr"] }
leptos-motion = { version = "0.4.0", features = ["csr", "gestures", "layout"] }
```

## Feature Flags & Build Presets

Leptos Motion v0.4.0 includes comprehensive feature flags and build presets for optimal bundle sizes:

### Build Presets

| Preset       | Bundle Size | Description                 | Use Case             |
| ------------ | ----------- | --------------------------- | -------------------- |
| `minimal`    | ~30KB       | Core animations only        | Minimal applications |
| `production` | ~75KB       | Optimized for production    | Most applications    |
| `optimized`  | ~85KB       | With performance monitoring | Performance-critical |
| `standard`   | ~125KB      | Full features               | Feature-rich apps    |
| `full`       | ~235KB      | All features + dev tools    | Development          |

### Feature Flags

| Feature                 | Description                       | Default |
| ----------------------- | --------------------------------- | ------- |
| `minimal-serialization` | Custom lightweight serialization  | ✅      |
| `conditional-web-sys`   | Optimized web-sys feature usage   | ✅      |
| `performance-metrics`   | Performance monitoring            | ❌      |
| `memory-optimization`   | Memory usage optimization         | ❌      |
| `lazy-loading`          | Lazy loading of animation modules | ❌      |
| `gesture-support`       | Gesture recognition               | ❌      |
| `layout-animations`     | Layout animations                 | ❌      |
| `scroll-animations`     | Scroll-triggered animations       | ❌      |

### Example with Build Presets

```toml
# Minimal build (30KB)
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["minimal"] }

# Production build (75KB)
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["production"] }

# Optimized build (85KB)
[dependencies]
leptos-motion-core = { version = "0.4.0", features = ["optimized"] }
```

### Example with Custom Features

```toml
[dependencies]
leptos-motion-core = {
    version = "0.4.0",
    features = [
        "core-animations",
        "raf",
        "minimal-serialization",
        "conditional-web-sys",
        "performance-metrics"
    ]
}
```

## Project Setup

### 1. Create a New Leptos Project

```bash
# Create new Leptos project
cargo leptos new my-animated-app
cd my-animated-app

# Add Leptos Motion
cargo add leptos-motion
```

### 2. Configure Leptos.toml

Ensure your `Leptos.toml` is configured correctly:

```toml
[package]
name = "my-animated-app"
version = "0.1.0"
edition = "2021"

[leptos]
output-name = "my_animated_app"
site-root = "target/site"
site-pkg-dir = "pkg"
```

### 3. Update main.rs

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MotionDiv
            class="welcome".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Welcome to Leptos Motion!"
        </MotionDiv>
    }
}

fn main() {
    mount_to_body(App)
}
```

## Verification

### 1. Build the Project

```bash
# Build for development
cargo leptos build --dev

# Build for production
cargo leptos build --release
```

### 2. Run Development Server

```bash
# Start development server
cargo leptos serve --dev
```

### 3. Test Installation

Create a simple test to verify everything works:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use leptos_motion::*;

    #[test]
    fn test_motion_div_creation() {
        let _component = view! {
            <MotionDiv
                class="test".to_string()
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0)
                )
            >
                "Test"
            </MotionDiv>
        };
        // If this compiles, installation is successful
    }
}
```

## Troubleshooting

### Common Issues

**1. Version Conflicts**

```bash
# Check for version conflicts
cargo tree | grep leptos
```

**2. Missing WebAssembly Target**

```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown
```

**3. Build Errors**

```bash
# Clean and rebuild
cargo clean
cargo leptos build --dev
```

**4. Feature Not Available**

```toml
# Ensure feature is enabled
leptos-motion-core = { version = "0.4.0", features = ["gesture-support"] }
```

### Getting Help

If you encounter issues:

1. Check the [Troubleshooting Guide](../reference/troubleshooting.md)
2. Review the [FAQ](../reference/faq.md)
3. Search [GitHub Issues](https://github.com/cloud-shuttle/leptos-motion/issues)
4. Open a new issue with details

## Next Steps

After successful installation:

1. Follow the [First Animation](first-animation.md) tutorial
2. Explore [Basic Examples](../../examples/basic.md)
3. Read the [API Reference](../api/README.md)
4. Check out [Performance Tips](../guides/performance.md)

## Platform-Specific Notes

### Windows

- Ensure you have Visual Studio Build Tools installed
- Use PowerShell or Command Prompt for best compatibility

### macOS

- Xcode Command Line Tools may be required
- Use Homebrew for additional dependencies if needed

### Linux

- Install build essentials: `sudo apt install build-essential`
- Ensure you have the latest Rust toolchain

---

**Next**: [First Animation](first-animation.md) - Create your first animated component
