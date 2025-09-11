# ADR-005: Playwright Testing Strategy

## Status
Accepted

## Context
The leptos-motion library includes interactive demos and examples that need comprehensive end-to-end testing to ensure they work correctly across different browsers and provide the intended user experience. We need a robust testing strategy for our web-based demos and examples.

Key considerations:
- Demos must work correctly across different browsers
- Visual behavior and animations need to be tested
- User interactions must be validated
- Performance of animations in real browsers
- Cross-platform compatibility
- CI/CD integration for automated testing

## Decision
We will use **Playwright** as our primary end-to-end testing framework for all demos and web-based examples.

### Playwright Testing Strategy

#### 1. Comprehensive Demo Testing
- **All Demos**: Every demo and example must have Playwright tests
- **Cross-Browser**: Test on Chromium, Firefox, and WebKit
- **Visual Regression**: Screenshot comparison for visual consistency
- **Interaction Testing**: Validate user interactions and animations
- **Performance Testing**: Measure animation performance and frame rates

#### 2. Test Categories

##### Visual Regression Tests
```typescript
import { test, expect } from '@playwright/test';

test('webgl demo visual regression', async ({ page }) => {
  await page.goto('/examples/webgl-demo/demo.html');
  
  // Wait for demo to load
  await page.waitForSelector('.demo-container');
  
  // Take screenshot for visual regression
  await expect(page).toHaveScreenshot('webgl-demo-initial.png');
  
  // Test animation states
  await page.click('[data-testid="play-button"]');
  await page.waitForTimeout(1000);
  await expect(page).toHaveScreenshot('webgl-demo-playing.png');
});
```

##### Interaction Tests
```typescript
test('slider controls work correctly', async ({ page }) => {
  await page.goto('/examples/webgl-demo/demo.html');
  
  // Test bloom intensity slider
  const bloomSlider = page.locator('[data-testid="bloom-intensity"]');
  await bloomSlider.fill('0.5');
  
  // Verify the value is applied
  await expect(bloomSlider).toHaveValue('0.5');
  
  // Test that the visual effect changes
  const bloomEffect = page.locator('.bloom-effect');
  await expect(bloomEffect).toHaveCSS('opacity', '0.167'); // 0.5/3.0
});
```

##### Performance Tests
```typescript
test('animation performance', async ({ page }) => {
  await page.goto('/examples/webgl-demo/demo.html');
  
  // Start performance monitoring
  await page.evaluate(() => {
    window.performance.mark('animation-start');
  });
  
  // Trigger animation
  await page.click('[data-testid="play-button"]');
  
  // Wait for animation to complete
  await page.waitForFunction(() => {
    return window.performance.getEntriesByType('mark').length > 0;
  });
  
  // Measure performance
  const performanceMetrics = await page.evaluate(() => {
    const marks = window.performance.getEntriesByType('mark');
    const measures = window.performance.getEntriesByType('measure');
    return { marks, measures };
  });
  
  // Assert performance requirements
  expect(performanceMetrics.measures.length).toBeGreaterThan(0);
});
```

##### Accessibility Tests
```typescript
test('demo accessibility', async ({ page }) => {
  await page.goto('/examples/webgl-demo/demo.html');
  
  // Test keyboard navigation
  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  
  // Verify focus is visible
  const focusedElement = page.locator(':focus');
  await expect(focusedElement).toBeVisible();
  
  // Test screen reader compatibility
  const ariaLabels = await page.locator('[aria-label]').all();
  expect(ariaLabels.length).toBeGreaterThan(0);
});
```

#### 3. Test Organization

```
tests/
├── e2e/
│   ├── webgl-demo.spec.ts
│   ├── studio-demo.spec.ts
│   ├── comprehensive-demo.spec.ts
│   └── accessibility.spec.ts
├── fixtures/
│   ├── test-data.json
│   └── expected-screenshots/
└── utils/
    ├── test-helpers.ts
    └── performance-utils.ts
```

#### 4. CI/CD Integration

```yaml
# GitHub Actions
- name: Install Playwright
  run: pnpm install @playwright/test

- name: Install Playwright browsers
  run: pnpm exec playwright install

- name: Run Playwright tests
  run: pnpm exec playwright test

- name: Upload test results
  uses: actions/upload-artifact@v3
  if: always()
  with:
    name: playwright-report
    path: playwright-report/
```

## Consequences

### Positive
- **Comprehensive Coverage**: All demos are thoroughly tested
- **Cross-Browser Compatibility**: Ensures demos work across all major browsers
- **Visual Consistency**: Screenshot testing prevents visual regressions
- **User Experience Validation**: Tests validate actual user interactions
- **Performance Monitoring**: Continuous performance testing
- **Accessibility Assurance**: Ensures demos are accessible
- **Automated Testing**: Reduces manual testing overhead

### Negative
- **Test Maintenance**: Tests need to be updated when demos change
- **CI/CD Time**: E2E tests increase build times
- **Flaky Tests**: Browser tests can be flaky due to timing issues
- **Resource Usage**: Running multiple browsers requires more CI resources
- **Complexity**: E2E tests can be complex to write and debug

### Neutral
- **Test Infrastructure**: Need to maintain Playwright configuration
- **Screenshot Management**: Need to manage expected screenshots

## Implementation Notes

### Playwright Configuration
```typescript
// playwright.config.ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],
  webServer: {
    command: 'pnpm run serve',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

### Test Utilities
```typescript
// tests/utils/test-helpers.ts
export class DemoTestHelper {
  static async waitForAnimationComplete(page: Page, selector: string) {
    await page.waitForFunction(
      (sel) => {
        const element = document.querySelector(sel);
        return element && !element.classList.contains('animating');
      },
      selector
    );
  }
  
  static async takeAnimationScreenshot(page: Page, name: string) {
    await page.waitForTimeout(100); // Wait for animation to settle
    await page.screenshot({ path: `screenshots/${name}.png` });
  }
  
  static async measureAnimationPerformance(page: Page) {
    return await page.evaluate(() => {
      const entries = performance.getEntriesByType('measure');
      return entries.map(entry => ({
        name: entry.name,
        duration: entry.duration,
        startTime: entry.startTime,
      }));
    });
  }
}
```

### Demo Test Structure
```typescript
// tests/e2e/webgl-demo.spec.ts
import { test, expect } from '@playwright/test';
import { DemoTestHelper } from '../utils/test-helpers';

test.describe('WebGL Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/examples/webgl-demo/demo.html');
    await page.waitForSelector('.demo-container');
  });
  
  test('demo loads correctly', async ({ page }) => {
    await expect(page.locator('.demo-container')).toBeVisible();
    await expect(page.locator('[data-testid="play-button"]')).toBeVisible();
  });
  
  test('controls work correctly', async ({ page }) => {
    const bloomSlider = page.locator('[data-testid="bloom-intensity"]');
    await bloomSlider.fill('0.8');
    await expect(bloomSlider).toHaveValue('0.8');
  });
  
  test('visual regression', async ({ page }) => {
    await expect(page).toHaveScreenshot('webgl-demo-initial.png');
  });
  
  test('performance meets requirements', async ({ page }) => {
    await page.click('[data-testid="play-button"]');
    const metrics = await DemoTestHelper.measureAnimationPerformance(page);
    expect(metrics.length).toBeGreaterThan(0);
  });
});
```

### Package.json Scripts
```json
{
  "scripts": {
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:e2e:debug": "playwright test --debug",
    "test:e2e:headed": "playwright test --headed",
    "test:e2e:report": "playwright show-report"
  }
}
```

## References
- [Playwright Documentation](https://playwright.dev/)
- [Playwright Best Practices](https://playwright.dev/docs/best-practices)
- [Visual Regression Testing](https://playwright.dev/docs/test-snapshots)
- [Performance Testing with Playwright](https://playwright.dev/docs/test-performance)
