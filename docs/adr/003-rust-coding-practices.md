# ADR-003: Rust Coding Practices and Version Strategy

## Status
Accepted

## Context
The leptos-motion project is built in Rust and needs to establish clear coding standards, practices, and version management strategies to ensure code quality, maintainability, and consistency across the project.

Key considerations:
- Rust ecosystem evolves rapidly with new features and best practices
- Code quality and performance are critical for a motion library
- Team consistency and maintainability
- Integration with modern Rust tooling
- Performance optimization opportunities
- Security and safety best practices

## Decision
We will adopt **modern Rust coding practices** and maintain support for the **latest stable Rust version**.

### Rust Version Strategy

1. **Target Version**: Latest stable Rust (currently 1.75+)
2. **Update Policy**: Update to new stable versions within 2-4 weeks of release
3. **Toolchain Management**: Use `rust-toolchain.toml` for consistent toolchain
4. **Feature Flags**: Use stable features only, avoid nightly-only features in production

### Coding Standards

#### Code Style
- Follow Rust's official style guidelines
- Use `rustfmt` with default configuration
- Enforce clippy lints with strict settings
- Use `cargo fmt` and `cargo clippy` in CI/CD

#### Performance Practices
- Use `#[inline]` for hot paths and small functions
- Prefer `Vec::with_capacity()` when size is known
- Use `Box<dyn Trait>` sparingly, prefer generics when possible
- Profile and optimize critical paths
- Use `cargo bench` for performance testing

#### Safety and Error Handling
- Use `Result<T, E>` for fallible operations
- Implement custom error types with `thiserror`
- Use `Option<T>` for nullable values
- Avoid `unwrap()` in production code, use proper error handling
- Use `expect()` with descriptive messages when panicking is acceptable

#### Memory Management
- Prefer owned types over references when appropriate
- Use `Cow<'_, str>` for string flexibility
- Implement `Clone` and `Copy` traits judiciously
- Use `Arc<T>` and `Rc<T>` for shared ownership
- Avoid unnecessary allocations in hot paths

#### API Design
- Use builder patterns for complex configurations
- Implement `Default` trait for configuration structs
- Use `Into<T>` and `From<T>` for conversions
- Provide both owned and borrowed variants when appropriate
- Use `PhantomData` for type-level programming

## Consequences

### Positive
- **Code Quality**: Consistent, readable, and maintainable code
- **Performance**: Optimized code that takes advantage of Rust's zero-cost abstractions
- **Safety**: Memory-safe code with proper error handling
- **Developer Experience**: Clear patterns and practices for team members
- **Tooling**: Excellent IDE support and tooling integration
- **Ecosystem**: Access to latest Rust ecosystem improvements

### Negative
- **Learning Curve**: Team members need to stay updated with Rust best practices
- **Migration Effort**: Regular updates may require code changes
- **Complexity**: Some patterns may be more complex than alternatives
- **Compilation Time**: Strict linting and formatting may increase CI time

### Neutral
- **Toolchain Updates**: Regular Rust toolchain updates
- **Dependency Updates**: Regular updates to Rust dependencies

## Implementation Notes

### Toolchain Configuration
```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

### Cargo Configuration
```toml
# .cargo/config.toml
[build]
rustflags = ["-D", "warnings"]

[target.wasm32-unknown-unknown]
rustflags = ["-C", "opt-level=s"]

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### Clippy Configuration
```toml
# clippy.toml
# Allow some lints that are too strict for our use case
allow = [
    "clippy::too_many_arguments",
    "clippy::needless_pass_by_value",
]

# Deny important lints
deny = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::nursery",
    "clippy::cargo",
]
```

### Code Examples

#### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MotionError {
    #[error("Invalid animation duration: {0}")]
    InvalidDuration(f64),
    #[error("Animation not found: {0}")]
    AnimationNotFound(String),
    #[error("WebGL context error: {0}")]
    WebGLError(String),
}

pub fn create_animation(duration: f64) -> Result<Animation, MotionError> {
    if duration <= 0.0 {
        return Err(MotionError::InvalidDuration(duration));
    }
    Ok(Animation::new(duration))
}
```

#### Performance Optimization
```rust
pub struct AnimationPool {
    // Pre-allocate with known capacity
    animations: Vec<Animation>,
    // Use SmallVec for small collections
    active_animations: SmallVec<[AnimationId; 8]>,
}

impl AnimationPool {
    pub fn new(expected_capacity: usize) -> Self {
        Self {
            animations: Vec::with_capacity(expected_capacity),
            active_animations: SmallVec::new(),
        }
    }
    
    #[inline]
    pub fn add_animation(&mut self, animation: Animation) -> AnimationId {
        let id = AnimationId::new();
        self.animations.push(animation);
        id
    }
}
```

#### API Design
```rust
#[derive(Debug, Clone, Default)]
pub struct TransitionBuilder {
    duration: Option<f64>,
    ease: Option<Easing>,
    delay: Option<f64>,
}

impl TransitionBuilder {
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }
    
    pub fn ease(mut self, ease: Easing) -> Self {
        self.ease = Some(ease);
        self
    }
    
    pub fn build(self) -> Transition {
        Transition {
            duration: self.duration.unwrap_or(0.3),
            ease: self.ease.unwrap_or_default(),
            delay: self.delay.unwrap_or(0.0),
        }
    }
}
```

### CI/CD Integration
```yaml
# GitHub Actions
- name: Check Rust formatting
  run: cargo fmt --all -- --check

- name: Run Clippy
  run: cargo clippy --all-targets --all-features -- -D warnings

- name: Run tests
  run: cargo test --all-features

- name: Run benchmarks
  run: cargo bench --all-features
```

## References
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Documentation](https://doc.rust-lang.org/clippy/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [thiserror crate](https://docs.rs/thiserror/)
