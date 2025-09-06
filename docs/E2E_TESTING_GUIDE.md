# E2E Testing Guide for Leptos Motion

## Overview

This guide covers the comprehensive end-to-end (E2E) testing framework for the Leptos Motion library. E2E tests verify complete user workflows in real browser environments, ensuring the animation system works correctly across different browsers, devices, and user scenarios.

## E2E Testing Architecture

### Test Types

1. **WASM E2E Tests** (`tests/e2e/browser_tests.rs`)
   - Tests using `wasm-bindgen-test` for browser-based testing
   - Direct integration with Leptos components
   - Real DOM manipulation and event handling

2. **Playwright E2E Tests** (`tests/e2e/playwright_*.spec.ts`)
   - Cross-browser testing with Playwright
   - Visual regression testing
   - Performance monitoring
   - Mobile device testing

3. **Complete Workflow Tests** (`tests/e2e/complete_workflows.rs`)
   - End-to-end user scenarios
   - Multi-step interactions
   - Real-world application workflows

### Test Configuration

The E2E testing framework includes:

- **E2E Configuration** (`tests/e2e/e2e_config.rs`)
  - Test environment setup
  - Performance monitoring
  - Browser viewport configuration
  - Timeout and retry settings

- **Test Utilities** (`tests/e2e/e2e_config.rs`)
  - Helper functions for common operations
  - Assertion utilities
  - Performance measurement tools
  - Cross-browser compatibility helpers

## Test Workflows

### 1. E-commerce Product Interaction

Tests complete product browsing and interaction workflows:

```rust
#[wasm_bindgen_test]
async fn test_ecommerce_product_workflow() {
    // Create product grid with 6 products
    // Test hover animations on each product
    // Verify click interactions and state changes
    // Ensure smooth transitions between states
}
```

**What it tests:**

- Product hover animations (scale, shadow effects)
- Click feedback animations
- State transitions (normal → hover → active → normal)
- Performance with multiple animated elements

### 2. Form Validation and Submission

Tests complete form interaction workflows:

```rust
#[wasm_bindgen_test]
async fn test_form_validation_workflow() {
    // Create form with multiple fields
    // Test validation error animations
    // Verify focus state animations
    // Test successful submission flow
}
```

**What it tests:**

- Field focus animations (border color, box shadow)
- Validation error animations (opacity, height transitions)
- Form submission feedback
- Accessibility compliance

### 3. Image Gallery with Lightbox

Tests complete gallery interaction workflows:

```rust
#[wasm_bindgen_test]
async fn test_image_gallery_workflow() {
    // Create thumbnail grid
    // Test lightbox open/close animations
    // Verify thumbnail hover effects
    // Test navigation between images
}
```

**What it tests:**

- Thumbnail hover animations
- Lightbox entrance/exit animations
- Modal overlay animations
- Image transition effects

### 4. Dashboard Navigation

Tests complete dashboard interaction workflows:

```rust
#[wasm_bindgen_test]
async fn test_dashboard_workflow() {
    // Create dashboard layout
    // Test sidebar navigation animations
    // Verify content section transitions
    // Test responsive behavior
}
```

**What it tests:**

- Sidebar menu hover effects
- Content section entrance animations
- Navigation state transitions
- Layout responsiveness

### 5. Mobile Gesture Handling

Tests complete mobile interaction workflows:

```rust
#[wasm_bindgen_test]
async fn test_mobile_gesture_workflow() {
    // Create mobile viewport
    // Test swipe gestures
    // Verify touch interactions
    // Test page indicators
}
```

**What it tests:**

- Touch gesture recognition
- Swipe navigation animations
- Mobile-specific interactions
- Touch feedback animations

## Running E2E Tests

### Prerequisites

1. **Node.js and npm** (for Playwright tests)
2. **Rust and Cargo** (for WASM tests)
3. **wasm-bindgen-cli** (for WASM compilation)
4. **Python 3** (for test server)

### Installation

```bash
# Install Playwright
npm install -D @playwright/test
npx playwright install

# Install WASM testing tools
cargo install wasm-bindgen-cli
```

### Running Tests

#### Automated Test Runner

```bash
# Run all E2E tests
./scripts/run-e2e-tests.sh
```

This script:

- Starts a test server
- Runs WASM E2E tests
- Runs Playwright tests across all browsers
- Generates comprehensive reports
- Cleans up resources

#### Manual Test Execution

```bash
# Run WASM E2E tests only
cargo test --package leptos-motion-core --lib --test browser_tests

# Run Playwright tests
npx playwright test

# Run specific browser tests
npx playwright test --project=chromium
npx playwright test --project=firefox
npx playwright test --project=webkit

# Run mobile tests
npx playwright test --project="Mobile Chrome"
npx playwright test --project="Mobile Safari"
```

### Test Reports

E2E tests generate multiple report formats:

1. **HTML Report** (`target/e2e-reports/html/index.html`)
   - Interactive test results
   - Screenshots and videos
   - Performance metrics
   - Browser compatibility matrix

2. **JSON Report** (`target/e2e-reports/results.json`)
   - Machine-readable test results
   - CI/CD integration
   - Automated analysis

3. **JUnit Report** (`target/e2e-reports/results.xml`)
   - CI/CD integration
   - Test result aggregation
   - Build system integration

4. **Summary Report** (`target/e2e-reports/e2e_summary.md`)
   - Human-readable summary
   - Performance metrics
   - Recommendations

## Performance Testing

### Metrics Monitored

1. **Frame Rate (FPS)**
   - Target: 60+ FPS during animations
   - Monitoring: Real-time FPS tracking
   - Thresholds: 30 FPS minimum, 60 FPS preferred

2. **Memory Usage**
   - Target: < 100MB for typical workflows
   - Monitoring: JavaScript heap usage
   - Thresholds: 50MB normal, 100MB maximum

3. **Animation Performance**
   - Target: < 16ms per frame (60 FPS)
   - Monitoring: Animation frame timing
   - Thresholds: 16ms per frame, 100ms total animation

4. **Interaction Responsiveness**
   - Target: < 100ms response time
   - Monitoring: Event handling latency
   - Thresholds: 50ms preferred, 100ms maximum

### Performance Test Example

```typescript
test('Performance Under Load', async ({ page }) => {
  // Create 100 animated elements
  await page.evaluate(() => {
    // Create performance test elements
  });

  // Measure performance during interactions
  const performance = await helper.measurePerformance(async () => {
    // Perform rapid interactions
  });

  // Assert performance thresholds
  expect(performance.fps).toBeGreaterThan(30);
  expect(performance.memoryUsage).toBeLessThan(100);
});
```

## Accessibility Testing

### Reduced Motion Support

Tests verify that animations respect user preferences:

```typescript
test('Accessibility and Reduced Motion', async ({ page }) => {
  // Set reduced motion preference
  await page.addInitScript(() => {
    // Mock reduced motion preference
  });

  // Test that animations are minimal or disabled
  const opacityChange = Math.abs(parseFloat(finalOpacity) - parseFloat(initialOpacity));
  expect(opacityChange).toBeLessThan(0.1);
});
```

### Keyboard Navigation

Tests verify keyboard accessibility:

- Tab navigation through interactive elements
- Enter/Space key activation
- Arrow key navigation
- Escape key handling

### Screen Reader Compatibility

Tests verify screen reader support:

- ARIA labels and descriptions
- Semantic HTML structure
- Focus management
- Announcement timing

## Cross-Browser Testing

### Supported Browsers

1. **Desktop Browsers**
   - Chrome/Chromium (latest)
   - Firefox (latest)
   - Safari/WebKit (latest)

2. **Mobile Browsers**
   - Mobile Chrome (Android)
   - Mobile Safari (iOS)

### Browser-Specific Testing

```typescript
test('Cross-Browser Compatibility', async ({ page, browserName }) => {
  // Test basic functionality
  await testElement.hover();
  await helper.assertAnimationCompleted(testElement, 'transform', 'scale(1.1)');

  // Browser-specific assertions
  if (browserName === 'chromium') {
    // Chrome-specific tests
  } else if (browserName === 'firefox') {
    // Firefox-specific tests
  } else if (browserName === 'webkit') {
    // Safari-specific tests
  }
});
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: |
          npm install
          npx playwright install
          cargo install wasm-bindgen-cli

      - name: Run E2E tests
        run: ./scripts/run-e2e-tests.sh

      - name: Upload test results
        uses: actions/upload-artifact@v3
        with:
          name: e2e-test-results
          path: target/e2e-reports/
```

### Test Result Integration

- **Pass/Fail Status**: CI/CD pipeline integration
- **Performance Metrics**: Automated performance regression detection
- **Visual Regression**: Screenshot comparison
- **Coverage Reports**: Test coverage analysis

## Best Practices

### Test Design

1. **Realistic Scenarios**
   - Test complete user workflows
   - Use realistic data and interactions
   - Simulate real-world usage patterns

2. **Performance Focus**
   - Monitor performance metrics
   - Test under load conditions
   - Verify smooth animations

3. **Accessibility First**
   - Test with reduced motion
   - Verify keyboard navigation
   - Check screen reader compatibility

### Test Maintenance

1. **Regular Updates**
   - Update tests for new features
   - Maintain browser compatibility
   - Review performance thresholds

2. **Debugging**
   - Use screenshots and videos
   - Check browser console logs
   - Verify test environment setup

3. **Optimization**
   - Minimize test execution time
   - Use parallel execution
   - Optimize test data

## Troubleshooting

### Common Issues

1. **Test Timeouts**
   - Increase timeout values
   - Check for infinite animations
   - Verify element visibility

2. **Browser Compatibility**
   - Check browser-specific CSS
   - Verify JavaScript compatibility
   - Test on different devices

3. **Performance Issues**
   - Monitor memory usage
   - Check for memory leaks
   - Optimize animation performance

### Debug Tools

1. **Playwright Inspector**

   ```bash
   npx playwright test --debug
   ```

2. **Browser DevTools**
   - Performance tab for animation analysis
   - Memory tab for memory leak detection
   - Console for error logging

3. **Test Reports**
   - HTML reports for visual debugging
   - Screenshots for visual regression
   - Videos for interaction debugging

## Future Enhancements

### Planned Features

1. **Visual Regression Testing**
   - Automated screenshot comparison
   - Visual diff detection
   - Design system validation

2. **Advanced Performance Testing**
   - Real user monitoring (RUM)
   - Core Web Vitals tracking
   - Performance budgets

3. **Enhanced Mobile Testing**
   - Device-specific testing
   - Touch gesture recognition
   - Mobile performance optimization

4. **Accessibility Automation**
   - Automated accessibility scanning
   - WCAG compliance testing
   - Screen reader automation

## Conclusion

The E2E testing framework for Leptos Motion provides comprehensive coverage of real-world user scenarios, ensuring the animation library works correctly across all supported browsers and devices. By testing complete workflows, monitoring performance, and verifying accessibility, we ensure a high-quality user experience for all users.

The framework is designed to be maintainable, scalable, and integrated with modern CI/CD pipelines, providing confidence in the library's reliability and performance.
