# Leptos Motion Tests

This directory contains organized test suites for the leptos-motion library.

## Structure

### Unit Tests (`tests/unit/`)
- **`animation_engine_test.rs`** - Core animation engine unit tests
- **`css-animation-alternative.rs`** - CSS animation alternative implementations

### Integration Tests (`tests/integration/`)
- **`*.html`** - HTML test files for integration testing
- **`test_wasm.js`** - WASM integration tests
- **`test.txt`** - Test data and configurations

### End-to-End Tests (`tests/e2e/`)
- **`*.spec.ts`** - Playwright end-to-end test specifications

## Running Tests

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Serve the test files
cd tests/integration
python -m http.server 8000
# Then open the HTML files in your browser
```

### End-to-End Tests
```bash
# Install Playwright if not already installed
npm install -g playwright
playwright install

# Run E2E tests
npx playwright test tests/e2e/
```

## Test Organization

- **Unit tests** focus on individual components and functions
- **Integration tests** verify component interactions and WASM functionality
- **E2E tests** ensure the complete user experience works correctly
