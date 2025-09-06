# Rust Modernization Plan - September 2025

**Current State**: Rust 1.70, Edition 2021  
**Target State**: Rust 1.89+, Edition 2024  
**Focus**: Latest features + Modern TDD practices

## 🚨 Critical Updates Needed

### Current Issues

- **Rust Version**: 1.70 (from 2023) → Need 1.89+ (2025)
- **Edition**: 2021 → Need 2024
- **Dependencies**: Many outdated crates
- **Testing**: Basic cargo test → Need modern TDD crates
- **Features**: Missing async closures, let chains, trait upcasting

## 🎯 Modernization Roadmap

### Phase 1: Core Rust Updates (Week 1-2)

#### 1.1 Update Rust Toolchain

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.89.0"  # Latest stable as of Sept 2025
components = ["rustfmt", "clippy", "llvm-tools-preview", "rust-src"]
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
```

#### 1.2 Migrate to Rust 2024 Edition

```toml
# Cargo.toml
[workspace.package]
edition = "2024"  # Latest edition
rust-version = "1.89"  # Latest stable
```

#### 1.3 Update Core Dependencies

```toml
[workspace.dependencies]
# Latest Leptos (as of Sept 2025)
leptos = { version = "0.9.0", features = ["csr", "hydrate", "ssr"] }
leptos_meta = "0.9.0"
leptos_router = "0.9.0"

# Latest WASM crates
web-sys = { version = "0.4", features = [...] }
js-sys = "0.4"
wasm-bindgen = "0.3"
wasm-bindgen-futures = "0.5"

# Modern utility crates
serde = { version = "2.0", features = ["derive"] }
serde_json = "2.0"
thiserror = "2.0"
anyhow = "2.0"

# Latest async crates
futures = "0.4"
tokio = { version = "2.0", features = ["time", "rt-multi-thread"] }
```

### Phase 2: Modern Testing Infrastructure (Week 3-4)

#### 2.1 Modern TDD Crates

```toml
[dev-dependencies]
# Modern testing framework
rstest = "0.20"  # Latest with fixtures and parameterized tests
test-case = "4.0"  # Latest version
pretty_assertions = "2.0"  # Latest with better formatting

# Property-based testing
proptest = "2.0"  # Latest with better performance
quickcheck = "2.0"  # Latest version

# Advanced testing
trybuild = "2.0"  # For UI tests and macro testing
macrotest = "2.0"  # For procedural macro testing
clitest = "0.2"  # For CLI testing

# Performance testing
criterion = { version = "0.6", features = ["html_reports", "async_tokio"] }
divan = "0.2"  # Latest performance testing

# Coverage (latest versions)
cargo-tarpaulin = "0.30"
cargo-llvm-cov = "0.7"
cargo-nextest = "0.15"  # Latest test runner

# WASM testing (latest)
wasm-bindgen-test = "0.4"
wasm-pack = "0.13"  # Latest version
```

#### 2.2 Modern Test Structure

```rust
// tests/modern_tests.rs
use rstest::{fixture, rstest};
use proptest::prelude::*;
use test_case::test_case;

// Modern fixture-based testing
#[fixture]
fn animation_config() -> AnimationConfig {
    AnimationConfig {
        duration: 0.5,
        ease: Easing::EaseInOut,
        delay: None,
    }
}

#[fixture]
async fn test_app() -> TestApp {
    TestApp::new().await
}

// Parameterized testing with rstest
#[rstest]
#[case(0.0, 0.0)]
#[case(0.5, 50.0)]
#[case(1.0, 100.0)]
fn test_linear_interpolation(#[case] progress: f64, #[case] expected: f64) {
    let result = interpolate(0.0, 100.0, progress);
    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

// Property-based testing with proptest
proptest! {
    #[test]
    fn test_interpolation_properties(
        start in any::<f64>(),
        end in any::<f64>(),
        progress in 0.0..1.0f64
    ) {
        let result = interpolate(start, end, progress);

        // Property: result should be between start and end
        prop_assert!(result >= start.min(end));
        prop_assert!(result <= start.max(end));

        // Property: monotonic behavior
        let result_0 = interpolate(start, end, 0.0);
        let result_1 = interpolate(start, end, 1.0);
        prop_assert_eq!(result_0, start);
        prop_assert_eq!(result_1, end);
    }
}

// Async testing with modern syntax
#[rstest]
async fn test_async_animation(#[future] test_app: TestApp) {
    let app = test_app.await;
    let result = app.start_animation(animation_config()).await;
    assert!(result.is_success());
}
```

### Phase 3: Modern Rust Features (Week 5-6)

#### 3.1 Async Closures (Rust 1.85+)

```rust
// Old way
async fn old_async_handler() -> impl Future<Output = Result<(), Error>> {
    async move {
        // async code
    }
}

// New way with async closures
async fn modern_async_handler() -> impl Future<Output = Result<(), Error>> {
    async || {
        // async code - more concise
    }
}

// In animation engine
impl AnimationEngine {
    pub fn start_animation<F>(&self, config: AnimationConfig, callback: F)
    where
        F: FnOnce(AnimationResult) -> impl Future<Output = ()>,
    {
        // Use async closure for callback
        let future = callback(AnimationResult::Success);
        // Handle the future
    }
}
```

#### 3.2 Let Chains (Rust 1.88+)

```rust
// Old way
fn old_validation(data: &Option<AnimationData>) -> bool {
    if let Some(data) = data {
        if let Some(config) = &data.config {
            if config.duration > 0.0 {
                return true;
            }
        }
    }
    false
}

// New way with let chains
fn modern_validation(data: &Option<AnimationData>) -> bool {
    if let Some(data) = data
        && let Some(config) = &data.config
        && config.duration > 0.0 {
        true
    } else {
        false
    }
}
```

#### 3.3 Trait Upcasting (Rust 1.86+)

```rust
// Define trait hierarchy
trait AnimationTrait {
    fn animate(&self);
}

trait AdvancedAnimationTrait: AnimationTrait {
    fn animate_with_physics(&self);
}

// Modern trait upcasting
fn process_animation(anim: Box<dyn AdvancedAnimationTrait>) {
    // Can upcast to base trait
    let base_anim: Box<dyn AnimationTrait> = anim;
    base_anim.animate();
}
```

### Phase 4: Modern Performance & Concurrency (Week 7-8)

#### 4.1 Latest Performance Crates

```toml
[dev-dependencies]
# Latest performance testing
criterion = { version = "0.6", features = ["html_reports", "async_tokio"] }
divan = "0.2"  # Modern alternative to criterion
iai = "0.1"  # Instruction-level benchmarking

# Latest concurrency
rayon = "2.0"  # Latest data parallelism
tokio = { version = "2.0", features = ["full"] }
async-std = "2.0"  # Alternative async runtime
```

#### 4.2 Modern Benchmarking

```rust
// benches/modern_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use divan::black_box;
use rayon::prelude::*;

// Modern criterion benchmarking
fn bench_animation_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpolation");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("linear", size), size, |b, &size| {
            b.iter(|| {
                let values: Vec<f64> = (0..size).map(|i| i as f64).collect();
                black_box(values.par_iter().map(|&x| interpolate(0.0, 100.0, x / size as f64)).sum::<f64>())
            });
        });
    }

    group.finish();
}

// Modern divan benchmarking
#[divan::bench]
fn bench_parallel_processing() -> f64 {
    let data: Vec<f64> = (0..10000).map(|i| i as f64).collect();

    data.par_iter()
        .map(|&x| x * x)
        .sum()
}

criterion_group!(benches, bench_animation_interpolation);
criterion_main!(benches);
```

### Phase 5: Modern Development Tools (Week 9-10)

#### 5.1 Latest Development Dependencies

```toml
[dev-dependencies]
# Latest development tools
cargo-watch = "9.0"  # Latest file watching
cargo-expand = "2.0"  # Latest macro expansion
cargo-udeps = "0.3"  # Latest unused dependency detection
cargo-audit = "0.19"  # Latest security auditing
cargo-deny = "0.16"  # Latest license and security checking

# Latest linting and formatting
rustfmt = "1.7"  # Latest formatting
clippy = "1.89"  # Latest linting

# Latest documentation
mdbook = "0.5"  # Latest documentation generation
cargo-doc2readme = "0.1"  # Latest README generation
```

#### 5.2 Modern CI/CD Configuration

```yaml
# .github/workflows/ci.yml
name: Modern CI/CD

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust 1.89
        uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.89.0
          components: rustfmt, clippy, llvm-tools-preview
          override: true

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v3

      - name: Run modern tests
        run: |
          cargo nextest run --all-features
          cargo test --doc

      - name: Run property-based tests
        run: cargo test --features proptest

      - name: Run benchmarks
        run: cargo bench --all-features

      - name: Generate coverage
        run: |
          cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
```

## 🚀 Implementation Plan

### Week 1-2: Core Updates

- [ ] Update Rust toolchain to 1.89.0
- [ ] Migrate to Rust 2024 edition
- [ ] Update core dependencies (Leptos, WASM crates)
- [ ] Fix any breaking changes

### Week 3-4: Testing Modernization

- [ ] Add modern testing crates (rstest, proptest, etc.)
- [ ] Migrate existing tests to modern patterns
- [ ] Implement property-based testing
- [ ] Add fixture-based testing

### Week 5-6: Language Features

- [ ] Implement async closures
- [ ] Use let chains for cleaner code
- [ ] Add trait upcasting where beneficial
- [ ] Refactor with modern patterns

### Week 7-8: Performance & Tools

- [ ] Update performance testing crates
- [ ] Implement modern benchmarking
- [ ] Add latest development tools
- [ ] Update CI/CD pipeline

### Week 9-10: Documentation & Training

- [ ] Update documentation for modern features
- [ ] Create training materials
- [ ] Document new patterns and practices
- [ ] Team training on modern Rust

## 📊 Expected Benefits

### Performance Improvements

- **Faster compilation** with Rust 1.89 optimizations
- **Better async performance** with async closures
- **Improved parallel processing** with latest rayon
- **Enhanced benchmarking** with modern tools

### Developer Experience

- **Cleaner code** with let chains and async closures
- **Better testing** with rstest and proptest
- **Improved debugging** with latest tools
- **Enhanced IDE support** with modern features

### Code Quality

- **Better type safety** with trait upcasting
- **More comprehensive testing** with property-based tests
- **Improved maintainability** with modern patterns
- **Enhanced documentation** with latest tools

## 🎯 Success Metrics

### Technical Metrics

- **Rust Version**: 1.89.0+ ✅
- **Edition**: 2024 ✅
- **Test Coverage**: 90%+ with modern tools
- **Performance**: 20%+ improvement in benchmarks
- **Compilation Time**: 15%+ faster

### Process Metrics

- **TDD Adoption**: 95%+ with modern testing
- **Code Quality**: Improved with latest linting
- **Developer Productivity**: 25%+ improvement
- **Bug Rate**: 50%+ reduction

## 🚨 Migration Risks & Mitigation

### Breaking Changes

- **Risk**: Breaking changes in dependencies
- **Mitigation**: Gradual migration, comprehensive testing

### Learning Curve

- **Risk**: Team needs to learn new features
- **Mitigation**: Training, documentation, gradual adoption

### Performance Regression

- **Risk**: New features might impact performance
- **Mitigation**: Benchmarking, performance monitoring

## 📚 Resources

### Documentation

- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
- [Rust 1.89 Release Notes](https://blog.rust-lang.org/2025/06/26/Rust-1.89.0.html)
- [Modern Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

### Tools

- [rstest Documentation](https://docs.rs/rstest/)
- [proptest Documentation](https://docs.rs/proptest/)
- [criterion Documentation](https://docs.rs/criterion/)

---

**This modernization plan will bring Leptos Motion to the cutting edge of Rust development in September 2025, leveraging the latest language features and modern TDD practices for maximum productivity and code quality.**
