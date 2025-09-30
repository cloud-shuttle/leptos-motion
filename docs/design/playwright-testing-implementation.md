# Playwright Testing Implementation Design

## Overview

This document outlines the comprehensive implementation of Playwright-based end-to-end testing for Leptos Motion demos, providing automated validation of animation functionality across different browsers and environments.

## Current State

### ✅ Existing Infrastructure
- Playwright configuration files present
- Test structure established in `examples/comprehensive-showcase/tests/`
- Package.json with test scripts configured
- Basic test scenarios outlined

### ❌ Current Issues
- Server startup failures prevent test execution
- Limited test coverage for animation functionality
- No performance or accessibility testing
- Manual testing only (no CI/CD automation)

## Solution Architecture

### Test Pyramid Implementation

```
┌─────────────────────────────────────────┐
│      Manual Visual Testing              │  <- Human verification
│      (Animation quality, UX)            │
├─────────────────────────────────────────┤
│   Integration Tests (E2E)               │  <- Playwright
│   (Full application flow, animations)   │
├─────────────────────────────────────────┤
│   Component Tests                       │  <- Rust unit tests
│   (MotionDiv, Animation logic)          │
├─────────────────────────────────────────┤
│   Unit Tests                            │  <- Rust unit tests
│   (AnimationValue, Easing, etc)         │
└─────────────────────────────────────────┘
```

### Test Categories

#### 1. Functional Tests
- **Page Loading**: Verify demos load correctly
- **Animation Rendering**: Confirm MotionDiv animations work
- **User Interactions**: Test hover, click, drag behaviors
- **Responsive Design**: Validate across screen sizes

#### 2. Performance Tests
- **Load Time**: Measure page load performance
- **Animation Frame Rate**: Monitor 60fps target
- **Memory Usage**: Track memory consumption
- **Bundle Size**: Validate WASM loading

#### 3. Compatibility Tests
- **Cross-Browser**: Chrome, Firefox, Safari, Edge
- **Mobile Devices**: iOS Safari, Android Chrome
- **Accessibility**: WCAG compliance, keyboard navigation

#### 4. Regression Tests
- **Animation Consistency**: Ensure animations don't break
- **API Compatibility**: Validate MotionDiv API stability
- **Performance Baselines**: Detect performance regressions

## Implementation Strategy

### Phase 1: Core Test Infrastructure

#### 1.1 Fix Server Startup Issues

**Problem**: Trunk server fails to start due to environment conflicts.

**Solution**: Implement environment variable overrides.

```typescript
// playwright.config.ts
export default defineConfig({
  webServer: {
    command: 'NO_COLOR= TRUNK_COLOR=auto trunk serve --address 127.0.0.1 --port 3000',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
  // ... rest of config
});
```

#### 1.2 Test File Structure

```
examples/comprehensive-showcase/tests/
├── functional/
│   ├── page-loading.spec.ts
│   ├── animation-rendering.spec.ts
│   └── user-interactions.spec.ts
├── performance/
│   ├── load-times.spec.ts
│   ├── frame-rate.spec.ts
│   └── memory-usage.spec.ts
├── compatibility/
│   ├── cross-browser.spec.ts
│   ├── mobile.spec.ts
│   └── accessibility.spec.ts
└── regression/
    ├── animation-consistency.spec.ts
    └── api-compatibility.spec.ts
```

### Phase 2: Functional Test Implementation

#### 2.1 Page Loading Tests

```typescript
// tests/functional/page-loading.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Page Loading', () => {
  test.beforeEach(async ({ page }) => {
    // Set up test environment
    await page.goto('/');
  });

  test('homepage loads successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/Leptos Motion/);
    await expect(page.locator('#app')).toBeVisible();
  });

  test('WASM files load correctly', async ({ page, request }) => {
    // Test WASM file availability
    const wasmResponse = await request.get('/comprehensive_showcase_bg.wasm');
    expect(wasmResponse.status()).toBe(200);
    expect(wasmResponse.headers()['content-type']).toBe('application/wasm');
  });

  test('JavaScript initializes properly', async ({ page }) => {
    // Wait for WASM to initialize
    await page.waitForFunction(() => {
      return window.__LEPTOS_INITIALIZED__ === true;
    });
  });
});
```

#### 2.2 Animation Rendering Tests

```typescript
// tests/functional/animation-rendering.spec.ts
test.describe('Animation Rendering', () => {
  test('MotionDiv animations trigger correctly', async ({ page }) => {
    // Navigate to animation demo
    await page.goto('/');

    // Find animation trigger button
    const button = page.locator('button').filter({ hasText: 'Button Animation' });

    // Get initial position
    const initialBox = await page.locator('.motion-box').boundingBox();

    // Trigger animation
    await button.click();

    // Wait for animation
    await page.waitForTimeout(500);

    // Verify animation occurred
    const animatedBox = await page.locator('.motion-box').boundingBox();
    expect(animatedBox?.x).not.toBe(initialBox?.x);
  });

  test('multiple animations can run simultaneously', async ({ page }) => {
    // Test concurrent animations
    const buttons = page.locator('button');
    await buttons.nth(0).click(); // Scale animation
    await buttons.nth(1).click(); // Translate animation
    await buttons.nth(2).click(); // Rotate animation

    // Verify all animations are running
    await page.waitForTimeout(1000);
    // Add assertions for animation states
  });
});
```

#### 2.3 User Interaction Tests

```typescript
// tests/functional/user-interactions.spec.ts
test.describe('User Interactions', () => {
  test('hover animations work', async ({ page }) => {
    const animatedElement = page.locator('.hover-target');

    // Get initial state
    const initialScale = await animatedElement.evaluate(el =>
      window.getComputedStyle(el).transform
    );

    // Trigger hover
    await animatedElement.hover();

    // Wait for animation
    await page.waitForTimeout(300);

    // Verify hover animation
    const hoverScale = await animatedElement.evaluate(el =>
      window.getComputedStyle(el).transform
    );

    expect(hoverScale).not.toBe(initialScale);
  });

  test('drag interactions work', async ({ page }) => {
    const draggable = page.locator('.draggable');

    // Get initial position
    const initialBox = await draggable.boundingBox();

    // Perform drag
    await draggable.dragTo(page.locator('body'), {
      targetPosition: { x: 100, y: 50 }
    });

    // Verify drag occurred
    const finalBox = await draggable.boundingBox();
    expect(finalBox?.x).toBeGreaterThan(initialBox?.x || 0);
  });
});
```

### Phase 3: Performance Testing

#### 3.1 Load Time Performance

```typescript
// tests/performance/load-times.spec.ts
test.describe('Load Time Performance', () => {
  test('page loads within acceptable time', async ({ page }) => {
    const startTime = Date.now();

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Wait for WASM initialization
    await page.waitForFunction(() => {
      const app = document.getElementById('app');
      return app && app.children.length > 0;
    }, { timeout: 10000 });

    const loadTime = Date.now() - startTime;
    console.log(`Page load time: ${loadTime}ms`);

    // Assert acceptable load time
    expect(loadTime).toBeLessThan(10000); // 10 seconds max
  });

  test('WASM loads efficiently', async ({ page, request }) => {
    const startTime = Date.now();

    // Load WASM file
    const response = await request.get('/comprehensive_showcase_bg.wasm');
    expect(response.status()).toBe(200);

    const loadTime = Date.now() - startTime;
    console.log(`WASM load time: ${loadTime}ms`);

    // WASM should load within 2 seconds
    expect(loadTime).toBeLessThan(2000);
  });
});
```

#### 3.2 Animation Performance

```typescript
// tests/performance/frame-rate.spec.ts
test.describe('Animation Performance', () => {
  test('maintains 60fps during animations', async ({ page }) => {
    // Start frame rate monitoring
    const frameRates: number[] = [];

    page.on('framenavigated', () => {
      // Frame rate calculation logic
      const now = performance.now();
      // Implementation for frame rate tracking
    });

    // Trigger animations
    await page.locator('button').filter({ hasText: 'Start Animation' }).click();

    // Run animations for 5 seconds
    await page.waitForTimeout(5000);

    // Calculate average frame rate
    const avgFrameRate = frameRates.reduce((a, b) => a + b, 0) / frameRates.length;

    console.log(`Average frame rate: ${avgFrameRate}fps`);
    expect(avgFrameRate).toBeGreaterThan(50); // 50+ fps acceptable
  });

  test('animations complete within expected time', async ({ page }) => {
    const startTime = Date.now();

    // Trigger 0.5 second animation
    await page.locator('.animate-button').click();

    // Wait for animation to complete
    await page.waitForFunction(() => {
      // Check if animation completed
      return true; // Implementation needed
    });

    const animationTime = Date.now() - startTime;

    // Allow some tolerance for animation timing
    expect(animationTime).toBeGreaterThan(450); // At least 450ms
    expect(animationTime).toBeLessThan(800); // No more than 800ms
  });
});
```

### Phase 4: Compatibility Testing

#### 4.1 Cross-Browser Testing

```typescript
// playwright.config.ts - expanded configuration
export default defineConfig({
  projects: [
    { name: 'chromium', use: devices['Desktop Chrome'] },
    { name: 'firefox', use: devices['Desktop Firefox'] },
    { name: 'webkit', use: devices['Desktop Safari'] },
    { name: 'edge', use: devices['Desktop Edge'] },
    { name: 'chrome-mobile', use: devices['Pixel 5'] },
    { name: 'safari-mobile', use: devices['iPhone 12'] },
  ],
  // ... other config
});

// tests/compatibility/cross-browser.spec.ts
test.describe('Cross-Browser Compatibility', () => {
  test('animations work in all browsers', async ({ page, browserName }) => {
    await page.goto('/');

    // Trigger animation
    await page.locator('button').first().click();

    // Verify animation works (browser-specific checks if needed)
    const animatedElement = page.locator('.animated');
    await expect(animatedElement).toHaveCSS('transform', /scale|translate|rotate/);

    console.log(`Animation test passed on ${browserName}`);
  });
});
```

#### 4.2 Accessibility Testing

```typescript
// tests/compatibility/accessibility.spec.ts
test.describe('Accessibility', () => {
  test('respects reduced motion preferences', async ({ page }) => {
    // Set reduced motion preference
    await page.emulateMedia({ reducedMotion: 'reduce' });

    await page.goto('/');

    // Trigger animation
    await page.locator('button').click();

    // Verify animations are disabled or simplified
    // This requires checking actual animation behavior
    const hasAnimation = await page.evaluate(() => {
      // Check if animations are running
      return false; // Implementation needed
    });

    expect(hasAnimation).toBe(false);
  });

  test('keyboard navigation works', async ({ page }) => {
    await page.goto('/');

    // Test tab navigation
    await page.keyboard.press('Tab');
    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();

    // Test keyboard activation
    await page.keyboard.press('Enter');
    // Verify action occurred
  });

  test('screen reader compatibility', async ({ page }) => {
    // Test ARIA labels and roles
    const buttons = page.locator('button');
    for (const button of await buttons.all()) {
      const ariaLabel = await button.getAttribute('aria-label');
      expect(ariaLabel).toBeTruthy();
    }
  });
});
```

### Phase 5: CI/CD Integration

#### 5.1 GitHub Actions Workflow

```yaml
# .github/workflows/playwright-tests.yml
name: Playwright Tests
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Setup Trunk
        uses: jetli/trunk-action@v0.4.0

      - name: Install Playwright
        run: cd examples/comprehensive-showcase && npm ci

      - name: Install Playwright Browsers
        run: cd examples/comprehensive-showcase && npx playwright install

      - name: Run Playwright Tests
        run: cd examples/comprehensive-showcase && npx playwright test

      - name: Upload Test Results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: playwright-results
          path: examples/comprehensive-showcase/test-results/
```

#### 5.2 Performance Regression Detection

```typescript
// tests/performance/regression-baseline.spec.ts
test.describe('Performance Regression Baseline', () => {
  let baselineMetrics: any;

  test.beforeAll(async () => {
    // Load baseline metrics from previous runs
    baselineMetrics = await loadBaselineMetrics();
  });

  test('load time regression check', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const loadTime = Date.now() - startTime;

    // Check against baseline (allow 5% regression)
    const maxAcceptableTime = baselineMetrics.loadTime * 1.05;
    expect(loadTime).toBeLessThan(maxAcceptableTime);
  });

  test('animation performance regression', async ({ page }) => {
    // Measure animation performance
    const metrics = await measureAnimationPerformance(page);

    // Compare against baseline
    expect(metrics.frameRate).toBeGreaterThan(baselineMetrics.frameRate * 0.95);
  });
});
```

## Test Execution Strategy

### Local Development
```bash
# Run all tests
cd examples/comprehensive-showcase
npm test

# Run specific test categories
npx playwright test --grep "functional"
npx playwright test --grep "performance"

# Debug mode
npx playwright test --debug
npx playwright test --ui
```

### CI/CD Execution
- **On Push/PR**: Run all tests on ubuntu-latest
- **Scheduled**: Weekly full compatibility test matrix
- **Release**: Extended performance and compatibility testing

## Monitoring & Reporting

### Test Results Dashboard
- **Pass/Fail Status**: Real-time test results
- **Performance Trends**: Load time and frame rate charts
- **Browser Compatibility**: Matrix of supported browsers
- **Failure Analysis**: Common failure patterns

### Alerting
- **Test Failures**: Immediate notification on test failures
- **Performance Regression**: Alerts when metrics exceed thresholds
- **Browser Compatibility**: Notification when browser support breaks

## Success Metrics

### Coverage Metrics
- **Test Count**: 50+ comprehensive test cases
- **Browser Coverage**: 6+ browser/device combinations
- **Functionality Coverage**: 90%+ of user interactions tested
- **Performance Coverage**: Load time, frame rate, memory usage

### Quality Metrics
- **Test Reliability**: 95%+ test pass rate
- **Performance Stability**: < 5% performance variation
- **Cross-Browser Consistency**: Identical behavior across browsers
- **False Positive Rate**: < 2% flaky tests

### Efficiency Metrics
- **Test Execution Time**: < 10 minutes for full suite
- **CI/CD Pipeline Time**: < 15 minutes total
- **Debugging Time**: < 5 minutes to identify failures
- **Maintenance Overhead**: < 2 hours/week

## Risk Mitigation

### Technical Risks
1. **Browser API Changes**: Regular browser updates can break tests
2. **Performance Variability**: Different environments affect performance metrics
3. **Flaky Tests**: Network or timing issues can cause false failures

### Mitigation Strategies
1. **Test Isolation**: Each test is independent and self-contained
2. **Retry Logic**: Automatic retry for flaky tests
3. **Baseline Adjustments**: Regular baseline metric updates
4. **Cross-Environment Testing**: Test across different infrastructures

## Future Enhancements

### Advanced Features
- **Visual Regression Testing**: Screenshot comparison for UI changes
- **Performance Profiling**: Detailed performance analysis tools
- **Load Testing**: Concurrent user simulation
- **Real Device Testing**: Cloud-based device testing

### AI-Powered Testing
- **Test Generation**: AI-assisted test case creation
- **Failure Analysis**: Automated root cause analysis
- **Performance Prediction**: ML-based performance forecasting

## Conclusion

This comprehensive Playwright testing implementation provides robust, automated validation of Leptos Motion demos across multiple dimensions: functionality, performance, compatibility, and accessibility. The phased approach ensures systematic implementation while maintaining high quality and reliability standards.

**Expected Outcome**: Complete E2E testing coverage with automated CI/CD integration, enabling confident production deployments with validated animation functionality. 🚀
