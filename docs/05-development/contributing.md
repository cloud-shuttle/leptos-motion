# Contributing to Leptos Motion

Thank you for your interest in contributing to Leptos Motion! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)
- [Community](#community)

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic knowledge of Rust and Leptos
- Familiarity with animation concepts

### Areas for Contribution

We welcome contributions in the following areas:

- **Bug fixes** - Report and fix bugs
- **Feature development** - Implement new features
- **Documentation** - Improve docs and examples
- **Testing** - Add tests and improve coverage
- **Performance** - Optimize animations and reduce bundle size
- **Examples** - Create new examples and demos
- **Website** - Improve the showcase website

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/leptos-motion.git
cd leptos-motion

# Add the upstream remote
git remote add upstream https://github.com/cloud-shuttle/leptos-motion.git
```

### 2. Install Dependencies

```bash
# Install Rust toolchain
rustup default stable
rustup target add wasm32-unknown-unknown

# Install development tools
cargo install trunk
cargo install mdbook
cargo install cargo-audit
cargo install cargo-outdated
```

### 3. Build the Project

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### 4. Build Examples

```bash
# Build basic examples
cargo build --examples

# Build website (when WASM issues are resolved)
cd website
trunk build
```

## Code Style

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Write meaningful commit messages

### Documentation

- Document all public APIs
- Include examples in documentation
- Use clear, concise language
- Follow the existing documentation style

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:

```
feat(gestures): add drag gesture support
fix(core): resolve animation timing issue
docs(examples): add new animation examples
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test --package leptos-motion-core

# Run tests with output
cargo test -- --nocapture

# Run WASM tests (when available)
cargo test --target wasm32-unknown-unknown
```

### Writing Tests

- Write unit tests for all new functionality
- Include integration tests for complex features
- Test edge cases and error conditions
- Ensure good test coverage

### Test Structure

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
}
```

## Documentation

### Code Documentation

- Document all public functions, types, and modules
- Use doc comments (`///`) for public APIs
- Include examples in documentation
- Explain complex algorithms and design decisions

### Book Documentation

The documentation is built with mdBook. To contribute:

```bash
# Build the book
cd docs/book
mdbook build

# Serve locally for development
mdbook serve --open
```

### Examples

- Keep examples simple and focused
- Include comments explaining key concepts
- Ensure examples work and are up-to-date
- Add new examples for new features

## Pull Request Process

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clear, well-documented code
- Add tests for new functionality
- Update documentation as needed
- Follow the code style guidelines

### 3. Test Your Changes

```bash
# Run all tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build examples
cargo build --examples
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat(scope): description of changes"
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:

- Clear title and description
- Link to related issues
- Summary of changes
- Testing instructions
- Screenshots/videos for UI changes

### 6. PR Review Process

- All PRs require review from maintainers
- Address review comments promptly
- Keep PRs focused and reasonably sized
- Update PR description if needed

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major** (x.0.0): Breaking changes
- **Minor** (0.x.0): New features, backward compatible
- **Patch** (0.0.x): Bug fixes, backward compatible

### Release Steps

1. **Prepare Release**
   - Update version numbers in `Cargo.toml` files
   - Update `CHANGELOG.md`
   - Create release branch

2. **Test Release**
   - Run full test suite
   - Build all examples
   - Test documentation

3. **Create Tag**

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

4. **Automated Release**
   - GitHub Actions will automatically:
     - Run tests
     - Publish to crates.io
     - Create GitHub release
     - Deploy documentation

## Community

### Getting Help

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Documentation**: Check the book and API docs first

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Pull Requests**: Code contributions and reviews

### Recognition

Contributors will be recognized in:

- GitHub contributors list
- Release notes
- Project documentation
- Community acknowledgments

## Development Guidelines

### Architecture

- Keep crates modular and focused
- Minimize dependencies between crates
- Use feature flags for optional functionality
- Maintain backward compatibility

### Performance

- Optimize for 60fps animations
- Minimize bundle size
- Use GPU acceleration when possible
- Profile and benchmark changes

### Accessibility

- Support `prefers-reduced-motion`
- Ensure keyboard navigation works
- Provide alternative interactions
- Test with screen readers

### Browser Support

- Support modern browsers (last 2 versions)
- Graceful degradation for older browsers
- Test across different browsers
- Consider mobile performance

## Questions?

If you have questions about contributing:

1. Check the documentation first
2. Search existing issues and discussions
3. Create a new discussion for general questions
4. Open an issue for bugs or feature requests

Thank you for contributing to Leptos Motion! 🎭
