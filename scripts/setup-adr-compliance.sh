#!/bin/bash

# ADR Compliance Setup Script
# This script sets up the foundation for ADR compliance

set -e

echo "🚀 Setting up ADR Compliance Foundation..."

# Create necessary directories
echo "📁 Creating directory structure..."
mkdir -p docs/tdd
mkdir -p docs/testing
mkdir -p docs/standards
mkdir -p tests/{unit,integration,e2e}
mkdir -p scripts
mkdir -p .github/workflows

# Create TDD documentation
echo "📝 Creating TDD documentation..."
cat > docs/tdd/README.md << 'EOF'
# Test-Driven Development (TDD) Guide

## TDD Process
1. **Red**: Write a failing test first
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve code while keeping tests green
4. **Repeat**: Continue the cycle for each feature

## TDD Checklist
- [ ] Write failing test first (Red)
- [ ] Ensure test fails for the right reason
- [ ] Write minimal code to pass test (Green)
- [ ] Refactor while keeping tests green
- [ ] Review test coverage and quality

## Examples
See `examples.md` for TDD examples and patterns.
EOF

cat > docs/tdd/examples.md << 'EOF'
# TDD Examples

## Basic TDD Example

### Step 1: Red - Write Failing Test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let animation = Animation::new("fade-in");
        assert_eq!(animation.name(), "fade-in");
    }
}
```

### Step 2: Green - Write Minimal Code
```rust
pub struct Animation {
    name: String,
}

impl Animation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}
```

### Step 3: Refactor - Improve Code
```rust
pub struct Animation {
    name: String,
    duration: f64,
}

impl Animation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: 1.0, // Default duration
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn duration(&self) -> f64 {
        self.duration
    }
}
```
EOF

# Create testing documentation
echo "🧪 Creating testing documentation..."
cat > docs/testing/README.md << 'EOF'
# Testing Strategy

## Testing Pyramid
- **Unit Tests (70%)**: Fast, focused tests for individual functions
- **Integration Tests (20%)**: Test interactions between components
- **E2E Tests (10%)**: Test complete user workflows

## Coverage Requirements
- **Minimum Coverage**: 95% line coverage
- **Branch Coverage**: 90% branch coverage
- **Function Coverage**: 100% public function coverage

## Test Organization
```
tests/
├── unit/                    # Unit tests
├── integration/             # Integration tests
└── e2e/                     # End-to-end tests
```
EOF

# Create Rust standards documentation
echo "🦀 Creating Rust standards documentation..."
cat > docs/standards/rust.md << 'EOF'
# Rust Coding Standards

## Code Quality Requirements
- **Formatting**: All code must be formatted with `rustfmt`
- **Linting**: All code must pass `clippy` with strict settings
- **Documentation**: All public APIs must be documented
- **Testing**: 100% test coverage for all public functions

## Documentation Standards
```rust
/// Processes animation data with the given configuration.
///
/// # Arguments
/// * `data` - The animation data to process
/// * `config` - Configuration for the processing
///
/// # Returns
/// * `Ok(ProcessedData)` - Successfully processed data
/// * `Err(ProcessingError)` - Error during processing
///
/// # Examples
/// ```
/// let data = AnimationData::new();
/// let config = ProcessingConfig::default();
/// let result = process_animation_data(data, config)?;
/// ```
pub fn process_animation_data(
    data: AnimationData,
    config: ProcessingConfig,
) -> Result<ProcessedData, ProcessingError> {
    // Implementation
}
```
EOF

# Create strict clippy configuration
echo "🔧 Creating strict clippy configuration..."
cat > .clippy.toml << 'EOF'
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

# Set specific lint levels
warn = [
    "clippy::missing_docs_in_private_items",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]
EOF

# Create enhanced pre-commit configuration
echo "🪝 Creating enhanced pre-commit configuration..."
cat > .pre-commit-config.yaml << 'EOF'
repos:
  - repo: local
    hooks:
      - id: cargo-test
        name: Run cargo test
        entry: cargo test
        language: system
        pass_filenames: false
        always_run: true
      - id: cargo-clippy
        name: Run cargo clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        pass_filenames: false
        always_run: true
      - id: cargo-fmt
        name: Run cargo fmt
        entry: cargo fmt --all -- --check
        language: system
        pass_filenames: false
        always_run: true
      - id: coverage-check
        name: Check test coverage
        entry: cargo tarpaulin --out Html --output-dir coverage --fail-under 95
        language: system
        pass_filenames: false
        always_run: true
EOF

# Create coverage reporting script
echo "📊 Creating coverage reporting script..."
cat > scripts/coverage-report.sh << 'EOF'
#!/bin/bash

echo "🔍 Generating coverage report..."
cargo tarpaulin --out Html --output-dir coverage

echo "📈 Coverage summary:"
if [ -f "coverage/tarpaulin-report.html" ]; then
    echo "Coverage report generated at: coverage/tarpaulin-report.html"
else
    echo "❌ Coverage report generation failed"
    exit 1
fi

echo "📋 Files with low coverage:"
find . -name "*.rs" -path "*/src/*" -exec grep -l "coverage.*[0-9][0-9]%" {} \; 2>/dev/null || echo "No low coverage files found"

echo "✅ Coverage report complete"
EOF

chmod +x scripts/coverage-report.sh

# Create quality gates CI workflow
echo "🔄 Creating quality gates CI workflow..."
cat > .github/workflows/quality-gates.yml << 'EOF'
name: Quality Gates
on: [push, pull_request]

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
          
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Run tests
        run: cargo test --workspace
        
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Run rustfmt
        run: cargo fmt --all -- --check
        
      - name: Generate coverage
        run: cargo tarpaulin --out Html --output-dir coverage --fail-under 95
        
      - name: Security audit
        run: cargo audit
EOF

# Create TDD checklist
echo "✅ Creating TDD checklist..."
cat > docs/tdd/checklist.md << 'EOF'
# TDD Checklist

## Before Writing Code
- [ ] Write failing test first (Red)
- [ ] Ensure test fails for the right reason
- [ ] Document expected behavior in test

## During Implementation
- [ ] Write minimal code to pass test (Green)
- [ ] Ensure all tests pass
- [ ] Refactor while keeping tests green

## After Implementation
- [ ] Review test coverage
- [ ] Ensure tests are maintainable
- [ ] Document any deviations from TDD

## Code Review Checklist
- [ ] Tests were written first
- [ ] All tests pass
- [ ] Code coverage is adequate
- [ ] Tests are readable and maintainable
- [ ] No production code without tests
EOF

# Create PR template
echo "📝 Creating PR template..."
cat > .github/pull_request_template.md << 'EOF'
# Pull Request

## Description
Brief description of changes

## TDD Compliance
- [ ] Tests were written first (Red-Green-Refactor)
- [ ] All tests pass
- [ ] Test coverage is adequate (95%+)
- [ ] No production code without tests

## Code Quality
- [ ] Code follows Rust standards
- [ ] All clippy warnings resolved
- [ ] Code is properly documented
- [ ] Performance considerations addressed

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated (if applicable)
- [ ] E2E tests added/updated (if applicable)
- [ ] Manual testing completed

## Documentation
- [ ] Public APIs documented
- [ ] Examples updated (if applicable)
- [ ] README updated (if applicable)

## Related Issues
Closes #(issue number)
EOF

echo "🎉 ADR Compliance foundation setup complete!"
echo ""
echo "Next steps:"
echo "1. Install cargo-tarpaulin: cargo install cargo-tarpaulin"
echo "2. Run coverage report: ./scripts/coverage-report.sh"
echo "3. Review TDD documentation: docs/tdd/README.md"
echo "4. Start implementing TDD for new features"
echo "5. Gradually improve test coverage to 95%+"
echo ""
echo "📚 Documentation created:"
echo "  - docs/tdd/README.md - TDD guide and examples"
echo "  - docs/testing/README.md - Testing strategy"
echo "  - docs/standards/rust.md - Rust coding standards"
echo "  - .clippy.toml - Strict clippy configuration"
echo "  - .pre-commit-config.yaml - Enhanced pre-commit hooks"
echo "  - .github/workflows/quality-gates.yml - Quality gates CI"
echo ""
echo "🚀 Ready to start ADR compliance implementation!"

