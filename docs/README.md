# Leptos Motion Documentation

Welcome to the comprehensive documentation for Leptos Motion, a high-performance animation library for Rust and Leptos applications.

## 📚 Documentation Index

### Getting Started
- [Quick Start Guide](./GETTING_STARTED.md) - Get up and running in minutes
- [Installation Guide](./INSTALLATION.md) - Detailed installation instructions
- [Basic Examples](./EXAMPLES.md) - Simple examples to get you started

### User Guides
- [Animation Concepts](./ANIMATION_CONCEPTS.md) - Understanding animation fundamentals
- [API Reference](./API_REFERENCE.md) - Complete API documentation
- [Performance Guide](./PERFORMANCE_GUIDE.md) - Optimizing your animations
- [Migration Guide](./MIGRATION_GUIDE.md) - Upgrading from previous versions

### Developer Resources
- [Architecture Overview](./ARCHITECTURE.md) - System design and components
- [Contributing Guide](./CONTRIBUTING.md) - How to contribute to the project
- [Testing Guide](./TESTING.md) - Testing strategies and best practices
- [Release Notes](./RELEASE_NOTES.md) - What's new in each version

### Standards and Guidelines
- [Documentation Standards](./DOCUMENTATION_STANDARDS.md) - How we write documentation
- [Coding Standards](./CODING_STANDARDS.md) - Code style and best practices
- [ADR Index](./ADR/) - Architecture Decision Records

### Advanced Topics
- [Custom Easing Functions](./CUSTOM_EASING.md) - Creating your own easing curves
- [Performance Optimization](./PERFORMANCE_OPTIMIZATION.md) - Advanced performance techniques
- [Browser Compatibility](./BROWSER_COMPATIBILITY.md) - Cross-browser considerations
- [Troubleshooting](./TROUBLESHOOTING.md) - Common issues and solutions

## 🚀 Quick Start

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0.0);
    
    view! {
        <div
            style="transform: translateX({move || x()}px)"
            on:click=move |_| set_x.update(|x| *x += 100.0)
        >
            "Click me to animate!"
        </div>
    }
}
```

## 📖 Key Features

- **High Performance**: Optimized for 60fps animations
- **Type Safe**: Full Rust type safety with compile-time checks
- **WASM Compatible**: Runs efficiently in the browser
- **Flexible**: Support for custom easing functions and animations
- **Well Tested**: Comprehensive test suite with 95%+ coverage

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Node.js 18+
- pnpm 8+

### Setup
```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-motion.git
cd leptos-motion

# Install dependencies
pnpm install

# Run tests
cargo test
pnpm test:e2e

# Build examples
cargo build --examples
```

### Documentation Development
```bash
# Generate API documentation
cargo doc --open

# Build user guides
cd docs && mdbook build

# Serve documentation locally
cd docs && mdbook serve
```

## 📊 Project Status

- **Version**: 0.8.3
- **Test Coverage**: 95%+
- **Documentation Coverage**: 100% of public APIs
- **CI/CD**: Fully automated with quality gates
- **Performance**: 60fps on modern browsers

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details on:

- Code style and standards
- Testing requirements
- Pull request process
- Issue reporting

## 📄 License

This project is licensed under the ISC License - see the [LICENSE](../LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/cloud-shuttle/leptos-motion)
- [Crates.io](https://crates.io/crates/leptos-motion)
- [Documentation](https://docs.rs/leptos-motion)
- [Examples](https://github.com/cloud-shuttle/leptos-motion/tree/main/examples)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/leptos-motion/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/leptos-motion/discussions)
- **Discord**: [Leptos Discord](https://discord.gg/leptos)

---

*This documentation is maintained by the Leptos Motion team. Last updated: September 2024*