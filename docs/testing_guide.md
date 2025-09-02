# Leptos Motion Testing Guide

This guide covers the comprehensive testing strategy for Leptos Motion, including unit tests, integration tests, visual regression tests, E2E tests, and performance benchmarks.

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Types](#test-types)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Test Configuration](#test-configuration)
- [Coverage](#coverage)
- [Performance Testing](#performance-testing)
- [Visual Regression Testing](#visual-regression-testing)
- [E2E Testing](#e2e-testing)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Testing Philosophy

### Core Principles

1. **Test at Multiple Levels**: Unit → Integration → E2E → Visual
2. **Performance is a Feature**: Treat performance regressions as bugs
3. **Cross-Browser Compatibility**: Test on all major browsers
4. **Automation First**: All tests must be automatable
5. **Fast Feedback**: Quick test execution for developer productivity

### Testing Goals

- **Code Coverage**: Minimum 90% for core functionality
- **Performance**: No regression beyond 5% threshold
- **Compatibility**: Support for 95% of browser market share
- **Reliability**: Zero critical bugs in production
- **Developer Experience**: Tests run in <30 seconds locally

## Test Types

### 1. Unit Tests

Unit tests verify individual functions and components in isolation.

**Location**: `tests/unit/`
**Framework**: `cargo test`
**Coverage**: Core logic, mathematical functions, type system

```rust
#[test]
fn test_interpolation_linear() {
    let interpolator = LinearInterpolator::new();
    assert_relative_eq!(interpolator.interpolate(0.0, 100.0, 0.5), 50.0);
}
```

### 2. Integration Tests

Integration tests verify how components work together.

**Location**: `tests/integration/`
**Framework**: `wasm-bindgen-test`
**Coverage**: Component interactions, animation lifecycle

```rust
#[wasm_bindgen_test]
async fn test_animation_completes() {
    let test_app = TestApp::new();
    let handle = test_app.animate(Animation { /* ... */ });
    handle.await;
    assert_eq!(test_app.get_computed_style(".test-element", "opacity"), "1");
}
```

### 3. Visual Regression Tests

Visual tests ensure animations look consistent across browsers.

**Location**: `tests/visual/`
**Framework**: Custom visual test helpers
**Coverage**: Animation appearance, transform consistency

```rust
#[wasm_bindgen_test]
async fn test_fade_in_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("fade-test", "fade-element");
    // ... animation setup
    let final_opacity = helper.get_opacity(&element);
    assert!((final_opacity - 1.0).abs() < 0.01);
}
```

### 4. E2E Tests

End-to-end tests simulate real user interactions.

**Location**: `tests/e2e/`
**Framework**: `wasm-bindgen-test` with browser automation
**Coverage**: User workflows, complex interactions

```rust
#[wasm_bindgen_test]
async fn test_button_hover_animation_e2e() {
    let helper = E2ETestHelper::new();
    let button = helper.create_button_with_animation();
    helper.simulate_hover(&button);
    helper.assert_style_contains(&button, "transform", "scale(1.1)");
}
```

### 5. Performance Tests

Performance tests measure animation performance and memory usage.

**Location**: `tests/performance/`
**Framework**: `criterion`
**Coverage**: Frame rate, memory usage, bundle size

```rust
fn benchmark_animation_engine(c: &mut Criterion) {
    c.bench_function("animation_engine", |b| {
        b.iter(|| {
            let mut engine = OptimizedHybridEngine::new();
            engine.animate(&test_config);
        });
    });
}
```

## Running Tests

### Quick Commands

```bash
# Run all tests
./scripts/test-quality.sh

# Run specific test types
./scripts/test-quality.sh unit
./scripts/test-quality.sh integration
./scripts/test-quality.sh visual
./scripts/test-quality.sh e2e
./scripts/test-quality.sh performance

# Run with cargo aliases
cargo test-all
cargo test-unit
cargo test-integration
cargo test-wasm
cargo test-performance
```

### Individual Test Suites

```bash
# Unit tests for specific crate
cargo test --package leptos-motion-core --lib
cargo test --package leptos-motion-dom --lib

# Integration tests
cargo test --test '*' --all-features

# WASM tests
wasm-pack test --headless --chrome
cargo test --target wasm32-unknown-unknown

# Performance benchmarks
cargo bench --all-features

# Coverage
cargo test-coverage
cargo tarpaulin --all-features
```

### Test Options

```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in parallel
cargo nextest run

# Run tests with specific features
cargo test --features "full"
```

## Writing Tests

### Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_condition() {
        function_that_panics();
    }
}
```

### Integration Test Structure

```rust
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_integration_scenario() {
    // Setup
    let test_app = TestApp::new();
    
    // Execute
    let result = test_app.perform_action();
    
    // Verify
    assert!(result.is_success());
}
```

### Visual Test Structure

```rust
#[wasm_bindgen_test]
async fn test_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("test", "class");
    
    // Apply animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
        />
    };
    
    // Wait and verify
    helper.wait_for_animation(150);
    let final_opacity = helper.get_opacity(&element);
    assert!((final_opacity - 1.0).abs() < 0.01);
}
```

### E2E Test Structure

```rust
#[wasm_bindgen_test]
async fn test_user_workflow() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create UI elements
    let button = helper.create_button("test-button");
    
    // Simulate user interaction
    helper.simulate_click(&button);
    helper.wait_for_animation(100);
    
    // Verify results
    helper.assert_style_equals(&button, "background-color", "rgb(255, 0, 0)");
}
```

## Test Configuration

### Cargo Configuration

The `.cargo/config.toml` file contains test-specific configurations:

```toml
[alias]
test-all = "test --workspace --all-features"
test-unit = "test --lib --all-features"
test-integration = "test --test '*' --all-features"

[tarpaulin]
out = ["Html", "Xml"]
output-dir = "coverage/tarpaulin"
fail-under = 80
```

### Test Helpers

#### VisualTestHelper

Provides utilities for visual regression testing:

```rust
let helper = VisualTestHelper::new();
let element = helper.create_test_element("id", "class");
let opacity = helper.get_opacity(&element);
let transform = helper.get_transform_matrix(&element);
```

#### E2ETestHelper

Provides utilities for end-to-end testing:

```rust
let helper = E2ETestHelper::new();
let app = helper.create_test_app();
helper.simulate_click(&element);
helper.simulate_hover(&element);
helper.simulate_drag(&element, 0.0, 0.0, 100.0, 100.0);
```

## Coverage

### Coverage Tools

1. **Tarpaulin**: Rust-specific coverage tool
2. **LLVM-Cov**: LLVM-based coverage with HTML reports
3. **Codecov**: Cloud-based coverage reporting

### Running Coverage

```bash
# Generate coverage reports
cargo test-coverage
cargo tarpaulin --all-features

# View HTML reports
open coverage/llvm-cov/index.html
open coverage/tarpaulin/tarpaulin-report.html
```

### Coverage Targets

- **Overall**: 90% minimum
- **Core**: 95% minimum
- **Components**: 85% minimum
- **Gestures**: 80% minimum

## Performance Testing

### Benchmark Types

1. **Animation Engine**: Core animation performance
2. **DOM Operations**: DOM manipulation speed
3. **Memory Usage**: Memory allocation and cleanup
4. **Bundle Size**: Final WASM bundle size

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --all-features

# Run specific benchmark
cargo bench animation_engine

# Generate benchmark reports
cargo bench -- --verbose
```

### Performance Targets

- **Frame Rate**: 60fps minimum
- **Memory**: <10MB for typical usage
- **Bundle Size**: <50KB gzipped
- **Animation Latency**: <16ms

## Visual Regression Testing

### Visual Test Categories

1. **Animation Consistency**: Ensure animations look the same
2. **Transform Accuracy**: Verify CSS transforms are correct
3. **Color Consistency**: Check color animations
4. **Layout Stability**: Ensure animations don't break layout

### Writing Visual Tests

```rust
#[wasm_bindgen_test]
async fn test_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("test", "class");
    
    // Apply animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(2.0)
            )
        />
    };
    
    // Verify visual state
    helper.wait_for_animation(150);
    assert!((helper.get_opacity(&element) - 1.0).abs() < 0.01);
    assert!(helper.get_transform_matrix(&element).contains("scale(2)"));
}
```

## E2E Testing

### E2E Test Categories

1. **User Workflows**: Complete user journeys
2. **Interactive Elements**: Buttons, forms, modals
3. **Complex Animations**: Multi-step animations
4. **Performance Under Load**: Many concurrent animations

### Writing E2E Tests

```rust
#[wasm_bindgen_test]
async fn test_complete_user_workflow() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Setup UI
    let button = helper.create_button("trigger");
    let modal = helper.create_modal("content");
    
    // Simulate user interaction
    helper.simulate_click(&button);
    helper.wait_for_animation(200);
    
    // Verify modal is visible
    helper.assert_style_equals(&modal, "opacity", "1");
    
    // Close modal
    helper.simulate_click(&button);
    helper.wait_for_animation(200);
    
    // Verify modal is hidden
    helper.assert_style_equals(&modal, "opacity", "0");
}
```

## Best Practices

### Test Organization

1. **Group Related Tests**: Use modules to organize tests
2. **Descriptive Names**: Use clear, descriptive test names
3. **Arrange-Act-Assert**: Follow AAA pattern
4. **Test Isolation**: Each test should be independent

### Test Data

1. **Use Constants**: Define test data as constants
2. **Random Data**: Use property-based testing for edge cases
3. **Realistic Data**: Use realistic test scenarios
4. **Edge Cases**: Test boundary conditions

### Performance

1. **Fast Tests**: Keep unit tests under 1ms
2. **Parallel Execution**: Use parallel test execution
3. **Minimal Setup**: Minimize test setup time
4. **Efficient Assertions**: Use efficient assertion methods

### Maintenance

1. **Update Tests**: Keep tests in sync with code changes
2. **Remove Dead Tests**: Remove obsolete tests
3. **Document Changes**: Document test changes
4. **Review Coverage**: Regularly review coverage reports

## Troubleshooting

### Common Issues

#### WASM Tests Fail

```bash
# Check WASM target installation
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli

# Run with verbose output
wasm-pack test --headless --chrome --verbose
```

#### Performance Tests Slow

```bash
# Run with fewer iterations
cargo bench -- --measurement-time 1

# Run specific benchmark
cargo bench animation_engine

# Check system resources
top
```

#### Coverage Reports Missing

```bash
# Install coverage tools
cargo install cargo-tarpaulin
cargo install cargo-llvm-cov

# Generate reports
cargo test-coverage
cargo tarpaulin --all-features
```

#### Visual Tests Inconsistent

```bash
# Check browser compatibility
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox

# Increase wait times
helper.wait_for_animation(300);

# Add tolerance for floating point comparisons
assert!((actual - expected).abs() < 0.01);
```

### Debugging Tips

1. **Use `--nocapture`**: See test output
2. **Add Logging**: Use `println!` for debugging
3. **Check Browser Console**: For WASM test issues
4. **Profile Performance**: Use browser dev tools
5. **Isolate Issues**: Run tests individually

### Getting Help

1. **Check Documentation**: Review this guide
2. **Search Issues**: Look for similar problems
3. **Ask Community**: Post on GitHub Discussions
4. **Create Minimal Example**: Reproduce issue in isolation

## Conclusion

This comprehensive testing strategy ensures Leptos Motion is reliable, performant, and maintainable. By following these guidelines, you can contribute to the project with confidence that your changes are well-tested and won't introduce regressions.

Remember:
- Write tests for new features
- Update tests when changing existing code
- Run the full test suite before submitting PRs
- Monitor coverage and performance metrics
- Keep tests fast and maintainable
