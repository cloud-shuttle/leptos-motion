# Contributing to Leptos Motion

Welcome to Leptos Motion! We're excited to have you contribute to this animation library for Rust and Leptos.

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust with rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install required tools
cargo install wasm-pack cargo-nextest cargo-watch
npm install -g playwright @percy/cli backstopjs
```

### Development Setup

```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-motion.git
cd leptos-motion

# Install dependencies and build
cargo build --all-features

# Run tests to verify setup
cargo test --lib
wasm-pack test --headless --chrome
```

## 📋 Development Workflow

### 1. Issue First
Before starting work, please:
- Check existing issues and discussions
- Create an issue for bugs or feature requests
- Get consensus on approach for significant changes
- Assign yourself to avoid duplicate work

### 2. Branch Strategy
```bash
# Create feature branch
git checkout -b feature/animation-spring-physics
git checkout -b fix/gesture-memory-leak
git checkout -b docs/api-examples
```

### 3. Development Process
```bash
# Start development with watch mode
cargo watch -x "test --lib" -x "clippy -- -D warnings"

# Run comprehensive tests
./scripts/test-local.sh

# Run specific test suites
cargo nextest run --lib                    # Unit tests
wasm-pack test --headless --chrome        # WASM tests
cargo bench                               # Performance tests
npm run test:visual                       # Visual tests
```

## 🎯 Contribution Areas

### Core Engine Development
- **Animation Engine**: RAF and WAAPI implementations
- **Spring Physics**: Mathematical simulation improvements
- **Performance**: Optimization and benchmarking
- **Browser Compatibility**: Cross-platform testing

### Component System
- **Motion Components**: New component types
- **Gesture System**: Touch and pointer interactions
- **Layout Animations**: FLIP technique implementations
- **Scroll Effects**: Intersection Observer integration

### Developer Experience
- **Documentation**: Examples, guides, and API docs
- **Error Messages**: Helpful error reporting
- **Type Safety**: Compile-time validation
- **Tooling**: Development and build tools

### Testing & Quality
- **Test Coverage**: Unit, integration, and E2E tests
- **Performance Testing**: Benchmarks and profiling
- **Visual Testing**: Screenshot regression
- **Cross-Browser**: Compatibility validation

## 📝 Code Standards

### Rust Code Style
```rust
// Use standard formatting
cargo fmt

// Follow clippy suggestions
cargo clippy -- -D warnings

// Document all public APIs
/// Calculates spring position at given time
/// 
/// # Arguments
/// * `time` - Time in seconds since animation start
/// 
/// # Returns
/// Position value between 0.0 and 1.0
pub fn calculate_position(&self, time: f64) -> f64 {
    // Implementation
}
```

### Testing Requirements
```rust
// Unit tests for all public functions
#[test]
fn test_spring_physics() {
    let spring = SpringSimulator::new(100.0, 10.0, 1.0);
    assert_relative_eq!(spring.calculate_position(1.0), 0.95, epsilon = 0.01);
}

// Property-based tests for mathematical functions
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_easing_bounded(t in 0.0..=1.0) {
        let result = ease_in_out_quad(t);
        prop_assert!(result >= 0.0 && result <= 1.0);
    }
}

// Integration tests for components
#[wasm_bindgen_test]
async fn test_motion_component_lifecycle() {
    // Test component mounting, animation, and cleanup
}
```

### Performance Standards
- No performance regressions in benchmarks
- Memory usage must be bounded and predictable
- 60fps target for reasonable animation loads
- Bundle size within established limits

## 🧪 Testing Guidelines

### Test Categories

#### Unit Tests (Fast, Isolated)
```bash
# Run frequently during development
cargo test --lib
cargo nextest run --lib
```
- Test pure Rust functions
- Mathematical correctness
- Edge cases and error handling
- No external dependencies

#### Integration Tests (Component Level)
```bash
# Test component interactions
wasm-pack test --headless --chrome --firefox
cargo test --test integration
```
- Component rendering and behavior
- Animation lifecycle management
- Cross-browser WASM compatibility

#### E2E Tests (User Scenarios)
```bash
# Full user workflows
npm run test:e2e
./scripts/test-cross-browser.sh
```
- Real browser interactions
- Performance in realistic scenarios
- Visual regression detection

#### Performance Tests
```bash
# Benchmark critical paths
cargo bench
./scripts/profile-memory.sh
```
- Frame rate maintenance
- Memory usage patterns
- Bundle size validation

### Writing Good Tests
- **Descriptive Names**: `test_spring_overshoots_target_by_expected_amount`
- **Single Responsibility**: One behavior per test
- **Deterministic**: No flaky or random failures
- **Fast Execution**: Unit tests under 100ms
- **Resource Cleanup**: No memory leaks or hanging resources

## 🚦 Pull Request Process

### Before Submitting
- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] Passes all linting (`cargo clippy -- -D warnings`)
- [ ] All tests pass locally
- [ ] Documentation updated for public APIs
- [ ] Performance benchmarks show no regressions
- [ ] Visual tests pass (if UI changes)

### PR Description Template
```markdown
## Summary
Brief description of changes and motivation.

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (existing functionality affected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring (no external behavior change)

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] E2E tests added/updated
- [ ] Performance tests added/updated
- [ ] Manual testing completed

## Performance Impact
- Bundle size: +/- X KB
- Memory usage: +/- X MB
- Frame rate: +/- X fps impact

## Screenshots (if applicable)
Before/after comparisons for visual changes.

## Documentation
- [ ] API documentation updated
- [ ] Examples added/updated
- [ ] README updated (if needed)

## Breaking Changes
List any breaking changes and migration steps.
```

### Review Process
1. **Automated Checks**: CI must pass
2. **Code Review**: At least one maintainer approval
3. **Testing Review**: Test coverage and quality
4. **Performance Review**: No significant regressions
5. **Documentation Review**: Clear and complete docs

## 🎨 Documentation Standards

### API Documentation
```rust
/// Creates a new spring animation configuration.
///
/// # Arguments
/// * `stiffness` - Spring stiffness (higher = faster)
/// * `damping` - Spring damping (higher = less oscillation)
/// * `mass` - Object mass (higher = more inertia)
///
/// # Examples
/// ```rust
/// let spring = SpringConfig {
///     stiffness: 100.0,
///     damping: 10.0,
///     mass: 1.0,
/// };
/// ```
///
/// # Performance
/// Spring calculations run in O(1) time per frame.
pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self
```

### Example Code
- **Complete**: Self-contained working examples
- **Realistic**: Practical use cases, not toy examples
- **Commented**: Explain the why, not just the what
- **Tested**: All examples must compile and run

### Guides and Tutorials
- **Progressive**: Build complexity gradually
- **Visual**: Include animations and screenshots
- **Interactive**: CodePen/StackBlitz examples when possible
- **Current**: Keep examples updated with API changes

## 🐛 Bug Reports

### Before Reporting
- Search existing issues for duplicates
- Test with the latest version
- Provide minimal reproduction case
- Check if it's actually a Leptos or browser issue

### Bug Report Template
```markdown
## Bug Description
Clear description of the problem.

## Reproduction Steps
1. Step one
2. Step two
3. Expected vs actual behavior

## Environment
- Leptos Motion version: X.Y.Z
- Leptos version: X.Y.Z
- Rust version: X.Y.Z
- Browser: Chrome/Firefox/Safari X.Y
- OS: macOS/Windows/Linux

## Code Example
Minimal reproduction case (preferably online).

## Additional Context
Screenshots, error messages, related issues.
```

## 💡 Feature Requests

### Before Requesting
- Check existing issues and discussions
- Consider if it fits the project's scope and goals
- Think about API design and implementation complexity
- Provide concrete use cases

### Feature Request Template
```markdown
## Feature Summary
Brief description of the proposed feature.

## Motivation
Why is this feature needed? What problem does it solve?

## Detailed Design
How should this feature work? API examples?

## Use Cases
Specific scenarios where this would be useful.

## Implementation Notes
Technical considerations or constraints.

## Alternatives
Other solutions you've considered.
```

## 🏗️ Architecture Contributions

### Design Principles
When contributing to the architecture, consider:

- **Performance**: Will this maintain 60fps performance?
- **Type Safety**: Can we catch errors at compile time?
- **Developer Experience**: Is the API intuitive and helpful?
- **Bundle Size**: What's the size impact?
- **Browser Support**: Does this work across target browsers?

### API Design Guidelines
- **Consistent**: Follow established patterns
- **Composable**: Features should work together
- **Extensible**: Allow for future enhancements
- **Safe**: Prevent common mistakes
- **Documented**: Self-explanatory where possible

## 📈 Performance Contributions

### Benchmarking
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_spring_calculation(c: &mut Criterion) {
    c.bench_function("spring_position", |b| {
        let spring = SpringSimulator::new(100.0, 10.0, 1.0);
        b.iter(|| spring.calculate_position(0.5))
    });
}

criterion_group!(benches, bench_spring_calculation);
criterion_main!(benches);
```

### Performance Reviews
- Run benchmarks before and after changes
- Profile memory usage for significant changes
- Test frame rate impact with stress tests
- Consider bundle size implications

## 🌐 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and contribute
- Follow GitHub's community guidelines

### Communication Channels
- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Discord/Matrix**: Real-time chat (link in README)
- **Blog/Newsletter**: Updates and announcements

## 🎁 Recognition

Contributors are recognized through:
- **Git history**: All contributions are credited
- **Release notes**: Significant contributions highlighted
- **Contributors list**: Maintained in README
- **Special thanks**: Outstanding contributions called out

## 📚 Resources

### Learning Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Book](https://book.leptos.dev/)
- [WASM Book](https://rustwasm.github.io/docs/book/)
- [Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API)

### Development Tools
- [rust-analyzer](https://rust-analyzer.github.io/) - LSP for Rust
- [Tauri](https://tauri.app/) - Desktop app testing
- [Chrome DevTools](https://developers.google.com/web/tools/chrome-devtools) - Performance profiling

Thank you for contributing to Leptos Motion! 🚀

---

*Questions? Open an issue or join our community discussions.*