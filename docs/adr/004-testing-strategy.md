# ADR-004: Testing Pyramid and Coverage Standards

## Status
Accepted

## Context
The leptos-motion library requires comprehensive testing to ensure reliability, performance, and correctness. We need to establish a testing strategy that provides confidence in our code quality while maintaining development velocity.

Key considerations:
- Motion libraries are complex and require testing of timing, interpolation, and visual behavior
- WebAssembly (WASM) testing presents unique challenges
- Performance testing is critical for animation libraries
- Visual regression testing for UI components
- Integration testing for complex animation sequences
- Test maintainability and developer experience

## Decision
We will implement a **comprehensive testing pyramid** with **near-100% test coverage** as our goal.

### Testing Pyramid Structure

#### 1. Unit Tests (70% of tests)
- **Target**: Individual functions, methods, and small components
- **Coverage Goal**: 95%+ line coverage
- **Tools**: Rust's built-in `#[test]` framework, `cargo test`
- **Scope**: Core animation logic, easing functions, interpolation, utilities

#### 2. Integration Tests (20% of tests)
- **Target**: Component interactions, API boundaries, cross-crate functionality
- **Coverage Goal**: 90%+ integration coverage
- **Tools**: Rust integration tests, custom test harnesses
- **Scope**: Motion components, timeline interactions, export functionality

#### 3. End-to-End Tests (10% of tests)
- **Target**: Complete user workflows, visual behavior, performance
- **Coverage Goal**: 100% of critical user paths
- **Tools**: Playwright, custom WASM test runners
- **Scope**: Full animation sequences, visual regression, performance benchmarks

### Coverage Standards

#### Code Coverage Targets
- **Overall Coverage**: 95%+ line coverage
- **Critical Paths**: 100% coverage (animation logic, interpolation, timing)
- **Public APIs**: 100% coverage
- **Error Handling**: 100% coverage
- **Performance Critical**: 100% coverage

#### Coverage Tools
- `cargo tarpaulin` for Rust code coverage
- `cargo llvm-cov` for detailed coverage reports
- Custom WASM coverage tools for browser-specific code
- Playwright coverage for E2E tests

### Testing Categories

#### 1. Functional Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linear_interpolation() {
        let start = 0.0;
        let end = 100.0;
        let result = lerp(start, end, 0.5);
        assert_eq!(result, 50.0);
    }
    
    #[test]
    fn test_easing_functions() {
        let easing = Easing::EaseInOut;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(1.0), 1.0);
        assert!(easing.evaluate(0.5) > 0.0);
        assert!(easing.evaluate(0.5) < 1.0);
    }
}
```

#### 2. Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_animation_duration_always_positive(duration in 0.1f64..10.0) {
        let animation = Animation::new(duration);
        assert!(animation.duration() > 0.0);
    }
    
    #[test]
    fn test_interpolation_bounds(
        start in -1000.0f64..1000.0,
        end in -1000.0f64..1000.0,
        t in 0.0f64..1.0
    ) {
        let result = lerp(start, end, t);
        let min_val = start.min(end);
        let max_val = start.max(end);
        assert!(result >= min_val);
        assert!(result <= max_val);
    }
}
```

#### 3. Performance Tests
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_animation_update(c: &mut Criterion) {
    let mut animation = Animation::new(1.0);
    
    c.bench_function("animation_update", |b| {
        b.iter(|| {
            animation.update(black_box(0.016)); // 60fps
        })
    });
}

criterion_group!(benches, benchmark_animation_update);
criterion_main!(benches);
```

#### 4. Visual Regression Tests
```rust
#[wasm_bindgen_test]
async fn test_motion_div_visual_behavior() {
    let element = create_test_element();
    let motion_div = MotionDiv::new()
        .initial(vec![("opacity", 0.0)])
        .animate(vec![("opacity", 1.0)])
        .transition(Transition::default());
    
    // Test visual behavior
    motion_div.mount(&element).await;
    
    // Wait for animation to complete
    wait_for_animation_completion().await;
    
    // Verify final state
    assert_eq!(element.style().opacity(), "1");
}
```

## Consequences

### Positive
- **High Confidence**: Near-100% coverage provides confidence in code quality
- **Regression Prevention**: Comprehensive tests catch regressions early
- **Documentation**: Tests serve as living documentation
- **Refactoring Safety**: High coverage enables safe refactoring
- **Performance Assurance**: Performance tests ensure consistent performance
- **Visual Consistency**: Visual regression tests maintain UI consistency

### Negative
- **Development Overhead**: Writing comprehensive tests takes time
- **Maintenance Burden**: Tests need to be maintained alongside code
- **CI/CD Time**: Extensive test suites increase build times
- **Complexity**: Some tests may be complex to write and maintain
- **False Positives**: High coverage may lead to brittle tests

### Neutral
- **Test Infrastructure**: Need to maintain test infrastructure and tooling
- **Coverage Reporting**: Need to track and report coverage metrics

## Implementation Notes

### Test Organization
```
tests/
├── unit/                 # Unit tests
│   ├── animation.rs
│   ├── easing.rs
│   └── interpolation.rs
├── integration/          # Integration tests
│   ├── motion_components.rs
│   ├── timeline.rs
│   └── export.rs
├── e2e/                  # End-to-end tests
│   ├── basic_animations.spec.ts
│   ├── complex_sequences.spec.ts
│   └── performance.spec.ts
└── fixtures/             # Test fixtures and data
    ├── animations.json
    └── test_assets/
```

### Coverage Configuration
```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.5"
proptest = "1.0"
wasm-bindgen-test = "0.3"
```

### CI/CD Integration
```yaml
# GitHub Actions
- name: Run unit tests
  run: cargo test --all-features

- name: Run integration tests
  run: cargo test --test '*'

- name: Run E2E tests
  run: pnpm test:e2e

- name: Generate coverage report
  run: cargo tarpaulin --out Html

- name: Run performance benchmarks
  run: cargo bench --all-features
```

### Coverage Monitoring
```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# Check coverage thresholds
cargo tarpaulin --fail-under 95

# Generate detailed coverage
cargo llvm-cov --html --open
```

### Test Data Management
```rust
// Test fixtures
pub mod fixtures {
    pub fn create_test_animation() -> Animation {
        Animation::new(1.0)
            .with_easing(Easing::EaseInOut)
            .with_delay(0.1)
    }
    
    pub fn create_test_element() -> HtmlElement {
        // Create test DOM element
    }
}
```

## References
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion Benchmarking](https://docs.rs/criterion/)
- [Proptest Property Testing](https://docs.rs/proptest/)
- [wasm-bindgen-test](https://docs.rs/wasm-bindgen-test/)
- [Tarpaulin Coverage](https://docs.rs/cargo-tarpaulin/)
