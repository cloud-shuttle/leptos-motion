# Leptos Motion Scripts

This directory contains all the automation scripts for the Leptos Motion project, organized by purpose.

## 📁 Directory Structure

```
scripts/
├── development/     # Development workflow scripts
├── testing/         # Testing and benchmarking scripts
├── quality/         # Code quality and validation scripts
├── release/         # Release preparation scripts
└── README.md        # This file
```

## 🛠️ Development Scripts

### `development/pre-commit.sh`

Runs pre-commit hooks to ensure code quality before commits.

```bash
./scripts/development/pre-commit.sh
```

**What it does:**

- Runs cargo fmt to format code
- Runs cargo clippy for linting
- Runs tests to ensure nothing is broken
- Validates Cargo.toml files
- Checks for large files

## 🧪 Testing Scripts

### `testing/test-all.sh`

Runs the complete test suite across all crates and examples.

```bash
./scripts/testing/test-all.sh
```

**What it does:**

- Runs unit tests for all crates
- Runs integration tests
- Tests all examples compilation
- Generates test coverage reports

### `testing/e2e-tests.sh`

Runs end-to-end tests using Playwright.

```bash
./scripts/testing/e2e-tests.sh
```

**What it does:**

- Builds all examples
- Runs Playwright tests
- Generates test reports
- Validates browser compatibility

### `testing/benchmark.sh`

Runs performance benchmarks and generates reports.

```bash
./scripts/testing/benchmark.sh
```

**What it does:**

- Runs Criterion benchmarks
- Measures animation performance
- Generates benchmark reports
- Tracks performance regressions

## 🔍 Quality Scripts

### `quality/test-quality.sh`

Comprehensive code quality checks.

```bash
./scripts/quality/test-quality.sh
```

**What it does:**

- Runs cargo clippy with strict settings
- Checks for security vulnerabilities
- Validates documentation
- Runs cargo audit

### `quality/check-version-consistency.py`

Ensures version consistency across all Cargo.toml files.

```bash
python3 scripts/quality/check-version-consistency.py
```

**What it does:**

- Validates workspace version consistency
- Checks dependency versions
- Ensures proper version formatting

### `quality/check-large-files.sh`

Identifies and reports large files that might impact performance.

```bash
./scripts/quality/check-large-files.sh
```

**What it does:**

- Scans for files larger than 1MB
- Reports potential performance issues
- Suggests optimization opportunities

### `quality/validate-cargo-toml.sh`

Validates all Cargo.toml files for correctness.

```bash
./scripts/quality/validate-cargo-toml.sh
```

**What it does:**

- Validates TOML syntax
- Checks required fields
- Ensures proper dependency declarations

## 🚀 Release Scripts

_Release scripts will be added as needed for the release process._

## 📋 Usage Guidelines

### Running Scripts

All scripts are designed to be run from the project root:

```bash
# From project root
./scripts/development/pre-commit.sh

# Or with full path
./scripts/testing/test-all.sh
```

### Script Requirements

Most scripts require:

- Rust toolchain (latest stable)
- Cargo
- Python 3.8+ (for Python scripts)
- Node.js (for E2E tests)

### Environment Variables

Some scripts support environment variables:

```bash
# Run tests with verbose output
VERBOSE=1 ./scripts/testing/test-all.sh

# Run benchmarks with specific configuration
BENCHMARK_CONFIG=release ./scripts/testing/benchmark.sh
```

## 🔧 Customization

### Adding New Scripts

When adding new scripts:

1. Place them in the appropriate subdirectory
2. Make them executable: `chmod +x script-name.sh`
3. Update this README with documentation
4. Follow the existing naming conventions

### Script Standards

All scripts should:

- Be executable and start with a shebang
- Include error handling
- Provide clear output and progress indicators
- Exit with appropriate status codes
- Be documented in this README

## 🐛 Troubleshooting

### Common Issues

**Script not executable:**

```bash
chmod +x scripts/path/to/script.sh
```

**Permission denied:**

```bash
# Make sure you're running from project root
pwd  # Should show /path/to/leptos-motion
```

**Python script errors:**

```bash
# Ensure Python 3.8+ is available
python3 --version
```

### Getting Help

If a script fails:

1. Check the error output
2. Ensure all dependencies are installed
3. Verify you're running from the project root
4. Check the script's documentation above

## 📝 Contributing

When modifying scripts:

1. Test your changes thoroughly
2. Update this README if needed
3. Follow existing patterns and conventions
4. Add appropriate error handling

## 🔄 Maintenance

Scripts are maintained alongside the main codebase:

- Updated with each release
- Tested on multiple platforms
- Documented with examples
- Version controlled with the project

---

**Last Updated**: Version 0.3.0 - December 2024

For issues with scripts, please open a GitHub issue or submit a pull request.
