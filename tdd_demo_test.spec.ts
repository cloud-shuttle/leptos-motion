import { test, expect, Page } from '@playwright/test';

test.describe('TDD Reactive Animation Demo', () => {
  let page: Page;

  test.beforeEach(async ({ browser }) => {
    page = await browser.newPage();

    // Enable console logging to capture debug info
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Capture any errors
    page.on('pageerror', error => {
      console.error(`Page error: ${error.message}`);
    });
  });

  test.afterEach(async () => {
    await page.close();
  });

  test('should load the demo page successfully', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('TDD Reactive Animation Demo');

    // Check if the status element is present
    await expect(page.locator('#status')).toBeVisible();
  });

  test('should load WASM module and initialize demo', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for the demo to initialize
    await page.waitForTimeout(3000);

    // Check if the loading spinner is hidden
    await expect(page.locator('#loading')).not.toBeVisible();

    // Check if the demo container is visible
    await expect(page.locator('#demo')).toBeVisible();

    // Check if the error container is not visible
    await expect(page.locator('#error')).not.toBeVisible();
  });

  test('should display console output', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for initialization
    await page.waitForTimeout(3000);

    // Check if console output is visible
    await expect(page.locator('#console')).toBeVisible();

    // Check if there are console messages
    const consoleLines = page.locator('.console-line');
    await expect(consoleLines).toHaveCount.greaterThan(0);

    // Check for success message
    await expect(page.locator('.console-line.success')).toContainText(
      'TDD Reactive Animation Demo loaded successfully'
    );
  });

  test('should display TDD implementation features', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for demo to load
    await page.waitForTimeout(3000);

    // Check if features are displayed
    const features = page.locator('.feature');
    await expect(features).toHaveCount(6);

    // Check specific features
    await expect(features.nth(0)).toContainText('Signal Tracking');
    await expect(features.nth(1)).toContainText('Reactive Targets');
    await expect(features.nth(2)).toContainText('Multiple Elements');
    await expect(features.nth(3)).toContainText('WASM Optimized');
    await expect(features.nth(4)).toContainText('TDD Validated');
    await expect(features.nth(5)).toContainText('Performance');
  });

  test('should handle demo loading errors gracefully', async () => {
    // Test with a non-existent page
    await page.goto('http://localhost:8080/nonexistent.html');

    // Should not crash
    await page.waitForTimeout(1000);

    // Go back to the main demo
    await page.goto('http://localhost:8080/index.html');
    await page.waitForTimeout(3000);

    // Should still work
    await expect(page.locator('#demo')).toBeVisible();
  });

  test('should validate WASM files are accessible', async () => {
    // Test WASM module
    const wasmResponse = await page.request.get(
      'http://localhost:8080/pkg/comprehensive_demo_bg.wasm'
    );
    expect(wasmResponse.status()).toBe(200);
    expect(wasmResponse.headers()['content-type']).toContain('application/wasm');

    // Test JS bindings
    const jsResponse = await page.request.get('http://localhost:8080/pkg/comprehensive_demo.js');
    expect(jsResponse.status()).toBe(200);
    expect(jsResponse.headers()['content-type']).toContain('application/javascript');
  });

  test('should test the test suite page', async () => {
    await page.goto('http://localhost:8080/test_demo.html');

    // Wait for page to load
    await page.waitForLoadState('networkidle');

    // Check if test page loads
    await expect(page.locator('h1')).toContainText('TDD Reactive Animation Demo Test');

    // Check if demo iframe is present
    await expect(page.locator('#demo-frame')).toBeVisible();

    // Wait for tests to run automatically
    await page.waitForTimeout(5000);

    // Check if test results are displayed
    const testResults = page.locator('.test-result');
    await expect(testResults).toHaveCount.greaterThan(0);

    // Check for specific test results
    await expect(page.locator('.test-pass')).toHaveCount.greaterThan(0);
  });

  test('should run manual tests via test suite', async () => {
    await page.goto('http://localhost:8080/test_demo.html');

    // Wait for page to load
    await page.waitForLoadState('networkidle');

    // Click the "Run Tests" button
    await page.click('button:has-text("Run Tests")');

    // Wait for tests to complete
    await page.waitForTimeout(3000);

    // Check if test results are displayed
    const testResults = page.locator('.test-result');
    await expect(testResults).toHaveCount.greaterThan(0);

    // Check for pass results
    const passResults = page.locator('.test-pass');
    await expect(passResults).toHaveCount.greaterThan(0);

    // Check for specific test names
    await expect(page.locator('.test-result')).toContainText('Demo HTML Load');
    await expect(page.locator('.test-result')).toContainText('WASM Module');
    await expect(page.locator('.test-result')).toContainText('JS Bindings');
  });

  test('should validate TDD implementation components', async () => {
    await page.goto('http://localhost:8080/test_demo.html');

    // Wait for page to load
    await page.waitForLoadState('networkidle');

    // Run tests
    await page.click('button:has-text("Run Tests")');
    await page.waitForTimeout(3000);

    // Check for TDD-specific test results
    await expect(page.locator('.test-result')).toContainText('TDD Implementation');
    await expect(page.locator('.test-result')).toContainText('Signal Tracking');
    await expect(page.locator('.test-result')).toContainText('Reactive MotionDiv');
    await expect(page.locator('.test-result')).toContainText('Effect Dependencies');
    await expect(page.locator('.test-result')).toContainText('WASM Memory Management');
  });

  test('should handle iframe demo loading', async () => {
    await page.goto('http://localhost:8080/test_demo.html');

    // Wait for page to load
    await page.waitForLoadState('networkidle');

    // Get the iframe
    const iframe = page.frameLocator('#demo-frame');

    // Wait for iframe to load
    await page.waitForTimeout(3000);

    // Check if iframe content is loaded
    await expect(iframe.locator('body')).toBeVisible();
  });

  test('should validate performance characteristics', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for demo to load
    await page.waitForTimeout(3000);

    // Measure page load performance
    const performanceMetrics = await page.evaluate(() => {
      const navigation = performance.getEntriesByType(
        'navigation'
      )[0] as PerformanceNavigationTiming;
      return {
        loadTime: navigation.loadEventEnd - navigation.loadEventStart,
        domContentLoaded:
          navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
        totalTime: navigation.loadEventEnd - navigation.fetchStart,
      };
    });

    // Validate performance is reasonable
    expect(performanceMetrics.totalTime).toBeLessThan(10000); // Less than 10 seconds
    expect(performanceMetrics.domContentLoaded).toBeLessThan(5000); // Less than 5 seconds
  });

  test('should check for JavaScript errors', async () => {
    const errors: string[] = [];

    page.on('pageerror', error => {
      errors.push(error.message);
    });

    await page.goto('http://localhost:8080/index.html');

    // Wait for demo to load
    await page.waitForTimeout(3000);

    // Check for any JavaScript errors
    expect(errors).toHaveLength(0);
  });

  test('should validate responsive design', async () => {
    await page.goto('http://localhost:8080/index.html');

    // Wait for demo to load
    await page.waitForTimeout(3000);

    // Test different viewport sizes
    await page.setViewportSize({ width: 375, height: 667 }); // Mobile
    await expect(page.locator('#demo')).toBeVisible();

    await page.setViewportSize({ width: 768, height: 1024 }); // Tablet
    await expect(page.locator('#demo')).toBeVisible();

    await page.setViewportSize({ width: 1920, height: 1080 }); // Desktop
    await expect(page.locator('#demo')).toBeVisible();
  });
});
