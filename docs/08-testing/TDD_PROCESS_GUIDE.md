# Test-Driven Development Process Guide

**Project**: Leptos Motion  
**Version**: 0.4.0  
**Last Updated**: September 6th, 2025

## Overview

This guide establishes the Test-Driven Development (TDD) process for Leptos Motion. It provides clear guidelines, examples, and best practices for implementing TDD in our animation library development.

**TDD Success**: The v0.4.0 bundle size optimization was successfully implemented using TDD methodology, achieving a 92% reduction (378KB → 30KB-85KB) through comprehensive test-driven development.

## TDD Principles

### Core TDD Cycle: Red-Green-Refactor

1. **Red**: Write a failing test
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve the code while keeping tests green

### TDD Benefits

- **Better Design**: Tests drive better API design
- **Confidence**: Safe refactoring with test coverage
- **Documentation**: Tests serve as living documentation
- **Quality**: Fewer bugs through test-first development
- **Maintainability**: Easier to maintain and extend code

## v0.4.0 TDD Success Story

### Bundle Size Optimization with TDD

The v0.4.0 release demonstrates the power of TDD methodology in achieving complex optimization goals:

#### Four-Phase TDD Implementation

1. **Phase 1: Dead Code Elimination**
   - Created TDD tests for module identification and savings calculation
   - Implemented conditional compilation based on test requirements
   - Achieved 120KB savings through test-driven dead code elimination

2. **Phase 2: Tree Shaking**
   - Developed comprehensive tree shaking tests
   - Validated unused function and type detection
   - Achieved 100KB savings through test-driven tree shaking

3. **Phase 3: Feature Flags**
   - Created feature flag optimization tests
   - Implemented conditional compilation based on test specifications
   - Achieved 185KB savings through test-driven feature flags

4. **Phase 4: Dependency Optimization**
   - Built dependency analysis and optimization tests
   - Developed minimal serialization system through TDD
   - Achieved 60KB+ savings through test-driven dependency optimization

#### TDD Test Suite Results

- **Total Tests**: 264 tests passing across all crates
- **Optimization Tests**: 13 new TDD test modules created
- **Bundle Size Reduction**: 92% (378KB → 30KB-85KB)
- **Zero Compilation Errors**: All optimizations validated through tests

## TDD Process Implementation

### 1. Feature Development Workflow

#### Step 1: Write Failing Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature_behavior() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = new_feature(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

#### Step 2: Make Test Pass

```rust
pub fn new_feature(input: InputType) -> OutputType {
    // Minimal implementation to make test pass
    expected_output
}
```

#### Step 3: Refactor

```rust
pub fn new_feature(input: InputType) -> OutputType {
    // Improved implementation with better design
    // All tests still pass
    improved_implementation(input)
}
```

### 2. Bug Fix Workflow

#### Step 1: Reproduce Bug with Test

```rust
#[test]
fn test_bug_reproduction() {
    // Arrange
    let buggy_input = create_buggy_scenario();

    // Act
    let result = function_with_bug(buggy_input);

    // Assert - This should fail initially
    assert_eq!(result, expected_correct_output);
}
```

#### Step 2: Fix Bug

```rust
pub fn function_with_bug(input: InputType) -> OutputType {
    // Fix the bug
    corrected_implementation(input)
}
```

#### Step 3: Verify Fix

```rust
#[test]
fn test_bug_fix() {
    // Same test now passes
    let buggy_input = create_buggy_scenario();
    let result = function_with_bug(buggy_input);
    assert_eq!(result, expected_correct_output);
}
```

## Test Categories and Guidelines

### Unit Tests

**Purpose**: Test individual functions and components in isolation

**Guidelines**:

- Test one behavior per test
- Use descriptive test names
- Follow Arrange-Act-Assert pattern
- Mock external dependencies
- Test edge cases and error conditions

**Example**:

```rust
#[test]
fn test_animation_value_interpolation() {
    // Arrange
    let start = AnimationValue::Number(0.0);
    let end = AnimationValue::Number(100.0);
    let progress = 0.5;

    // Act
    let result = interpolate_animation_value(start, end, progress);

    // Assert
    assert_eq!(result, AnimationValue::Number(50.0));
}

#[test]
fn test_animation_value_interpolation_edge_cases() {
    // Test edge cases
    assert_eq!(
        interpolate_animation_value(AnimationValue::Number(0.0), AnimationValue::Number(100.0), 0.0),
        AnimationValue::Number(0.0)
    );

    assert_eq!(
        interpolate_animation_value(AnimationValue::Number(0.0), AnimationValue::Number(100.0), 1.0),
        AnimationValue::Number(100.0)
    );
}
```

### Integration Tests

**Purpose**: Test how components work together

**Guidelines**:

- Test component interactions
- Use real DOM elements when possible
- Test animation lifecycle
- Verify side effects

**Example**:

```rust
#[wasm_bindgen_test]
async fn test_motion_div_animation_lifecycle() {
    // Arrange
    let test_app = TestApp::new();
    let element = test_app.create_element("div");

    // Act
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    test_app.mount(motion_div);

    // Assert
    test_app.wait_for_animation(150);
    test_app.assert_style_equals(&element, "opacity", "1");
}
```

### E2E Tests

**Purpose**: Test complete user scenarios

**Guidelines**:

- Test real user workflows
- Use actual browser automation
- Test performance and accessibility
- Verify visual consistency

**Example**:

```rust
#[test]
async fn test_user_drag_interaction() {
    let page = create_test_page().await;

    // User drags element
    page.drag_element("#draggable", 100, 100).await;

    // Verify element moved
    let position = page.get_element_position("#draggable").await;
    assert_eq!(position.x, 100);
    assert_eq!(position.y, 100);

    // Verify visual feedback
    assert!(page.has_class("#draggable", "dragging"));
}
```

## Test Writing Best Practices

### 1. Test Naming

**Good Names**:

```rust
#[test]
fn test_animation_value_interpolation_linear_progress() { }

#[test]
fn test_gesture_detection_pinch_zoom_scale() { }

#[test]
fn test_layout_animation_flip_transform_calculation() { }
```

**Bad Names**:

```rust
#[test]
fn test1() { }

#[test]
fn test_animation() { }

#[test]
fn test_stuff() { }
```

### 2. Test Structure

**Follow Arrange-Act-Assert**:

```rust
#[test]
fn test_feature_behavior() {
    // Arrange - Set up test data and conditions
    let input = create_test_input();
    let expected = create_expected_output();

    // Act - Execute the function under test
    let result = function_under_test(input);

    // Assert - Verify the result
    assert_eq!(result, expected);
}
```

### 3. Test Data Management

**Use Test Factories**:

```rust
pub struct TestDataFactory;

impl TestDataFactory {
    pub fn create_animation_target() -> AnimationTarget {
        let mut target = AnimationTarget::new();
        target.insert("opacity".to_string(), AnimationValue::Number(1.0));
        target.insert("scale".to_string(), AnimationValue::Number(1.5));
        target
    }

    pub fn create_transition() -> Transition {
        Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            delay: None,
            repeat: RepeatConfig::None,
        }
    }
}
```

### 4. Property-Based Testing

**Use Proptest for Mathematical Functions**:

```rust
use proptest::prelude::*;

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

        // Property: progress 0 should give start
        prop_assert_eq!(interpolate(start, end, 0.0), start);

        // Property: progress 1 should give end
        prop_assert_eq!(interpolate(start, end, 1.0), end);
    }
}
```

## TDD Checklist

### Before Writing Code

- [ ] Do I understand the requirement?
- [ ] Can I write a test for this behavior?
- [ ] What are the edge cases?
- [ ] What are the error conditions?

### During Development

- [ ] Did I write the test first?
- [ ] Does the test fail for the right reason?
- [ ] Did I write minimal code to pass?
- [ ] Did I refactor after making it pass?
- [ ] Are all tests still passing?

### After Implementation

- [ ] Do I have good test coverage?
- [ ] Are the tests readable and maintainable?
- [ ] Do the tests serve as documentation?
- [ ] Can I refactor with confidence?

## Common TDD Patterns

### 1. Test Doubles (Mocks/Stubs)

**For External Dependencies**:

```rust
pub trait AnimationEngine {
    fn start_animation(&self, config: AnimationConfig) -> AnimationHandle;
}

pub struct MockAnimationEngine {
    pub start_animation_calls: Vec<AnimationConfig>,
}

impl AnimationEngine for MockAnimationEngine {
    fn start_animation(&self, config: AnimationConfig) -> AnimationHandle {
        self.start_animation_calls.push(config);
        AnimationHandle(1)
    }
}

#[test]
fn test_animation_start() {
    let mock_engine = MockAnimationEngine::new();
    let animator = Animator::new(mock_engine);

    animator.start_animation(create_test_config());

    assert_eq!(mock_engine.start_animation_calls.len(), 1);
}
```

### 2. Test Builders

**For Complex Test Data**:

```rust
pub struct AnimationConfigBuilder {
    duration: Option<f64>,
    ease: Easing,
    delay: Option<f64>,
}

impl AnimationConfigBuilder {
    pub fn new() -> Self {
        Self {
            duration: None,
            ease: Easing::Linear,
            delay: None,
        }
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_ease(mut self, ease: Easing) -> Self {
        self.ease = ease;
        self
    }

    pub fn build(self) -> AnimationConfig {
        AnimationConfig {
            duration: self.duration,
            ease: self.ease,
            delay: self.delay,
        }
    }
}

#[test]
fn test_animation_config() {
    let config = AnimationConfigBuilder::new()
        .with_duration(0.5)
        .with_ease(Easing::EaseInOut)
        .build();

    assert_eq!(config.duration, Some(0.5));
    assert_eq!(config.ease, Easing::EaseInOut);
}
```

### 3. Test Helpers

**For Common Test Operations**:

```rust
pub struct TestHelper {
    document: web_sys::Document,
}

impl TestHelper {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Self { document }
    }

    pub fn create_element(&self, tag: &str) -> web_sys::Element {
        self.document.create_element(tag).unwrap()
    }

    pub fn wait_for_animation(&self, duration_ms: u64) {
        // Implementation for waiting
    }

    pub fn assert_style_equals(&self, element: &web_sys::Element, property: &str, expected: &str) {
        let actual = self.get_computed_style(element, property);
        assert_eq!(actual, expected);
    }
}
```

## TDD Anti-Patterns to Avoid

### 1. Testing Implementation Details

```rust
// Bad - Testing internal implementation
#[test]
fn test_internal_counter() {
    let animator = Animator::new();
    assert_eq!(animator.internal_counter, 0);
}

// Good - Testing behavior
#[test]
fn test_animation_starts() {
    let animator = Animator::new();
    let handle = animator.start_animation(config);
    assert!(handle.is_valid());
}
```

### 2. Over-Mocking

```rust
// Bad - Mocking everything
#[test]
fn test_animation() {
    let mock_engine = MockEngine::new();
    let mock_dom = MockDOM::new();
    let mock_timer = MockTimer::new();
    // Too many mocks make tests brittle
}

// Good - Mock only external dependencies
#[test]
fn test_animation() {
    let mock_engine = MockEngine::new();
    let animator = Animator::new(mock_engine);
    // Test the real logic with minimal mocking
}
```

### 3. Test Duplication

```rust
// Bad - Duplicated test setup
#[test]
fn test_animation_start() {
    let config = AnimationConfig {
        duration: Some(0.5),
        ease: Easing::Linear,
        // ... repeated setup
    };
    // test logic
}

#[test]
fn test_animation_stop() {
    let config = AnimationConfig {
        duration: Some(0.5),
        ease: Easing::Linear,
        // ... same setup repeated
    };
    // test logic
}

// Good - Use test fixtures
fn create_test_config() -> AnimationConfig {
    AnimationConfig {
        duration: Some(0.5),
        ease: Easing::Linear,
        // ... setup once
    }
}
```

## TDD Tools and Setup

### Required Tools

```toml
[dev-dependencies]
# Core testing
wasm-bindgen-test = "0.3"
test-case = "3.1"
pretty_assertions = "1.4"
approx = "0.5"

# Property-based testing
proptest = "1.4"
quickcheck = "1.0"

# Mocking
mockall = "0.12"

# Benchmarking
criterion = { version = "0.5", features = ["html_reports"] }

# Coverage
cargo-tarpaulin = "0.27"
cargo-llvm-cov = "0.5"
```

### Test Configuration

```toml
# .cargo/config.toml
[coverage]
minimum = 85

[coverage.ignore]
paths = [
    "tests/*",
    "benches/*",
    "examples/*",
]

[coverage.thresholds]
core = 95
dom = 85
gestures = 80
layout = 75
```

## TDD Metrics and Monitoring

### Key Metrics

- **Test Coverage**: 85%+ overall, 95%+ for core
- **Test Execution Time**: <30 seconds for unit tests
- **Test Reliability**: 99%+ pass rate
- **TDD Adoption**: 90%+ features developed test-first

### Monitoring Tools

- Coverage reports with `cargo-tarpaulin`
- Test performance with `cargo nextest`
- Test trends with CI/CD analytics
- Quality gates with coverage thresholds

## Conclusion

This TDD process guide provides the foundation for implementing Test-Driven Development in Leptos Motion. By following these guidelines, we can achieve:

- Higher code quality
- Better design decisions
- Increased confidence in refactoring
- Comprehensive test coverage
- Living documentation

**Next Steps**: Begin implementing TDD practices with the test improvement action plan, starting with critical test fixes and TDD process adoption.
