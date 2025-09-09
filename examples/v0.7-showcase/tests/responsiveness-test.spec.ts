import { test, expect } from '@playwright/test';

test.describe('Page Responsiveness Tests', () => {
  test('page should remain responsive after WASM loads', async ({ page }) => {
    // Navigate to the demo
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Test 1: Right-click should work
    await page.locator('body').click({ button: 'right' });
    // If right-click works, we should be able to see context menu or no error

    // Test 2: DOM interactions should work
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    expect(buttonCount).toBeGreaterThan(0);

    // Test 3: Clicking buttons should work
    const firstButton = buttons.first();
    await firstButton.click();

    // Test 4: Page should not be frozen
    const startTime = Date.now();
    await page.evaluate(() => {
      // Simple JavaScript execution test
      return 'page is responsive';
    });
    const endTime = Date.now();
    const responseTime = endTime - startTime;

    // Response time should be reasonable (less than 1 second)
    expect(responseTime).toBeLessThan(1000);

    console.log(`Page response time: ${responseTime}ms`);
  });

  test('should detect unresponsive page', async ({ page }) => {
    // This test specifically looks for signs of an unresponsive page
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Test if page becomes unresponsive
    const isResponsive = await page.evaluate(() => {
      return new Promise(resolve => {
        const startTime = Date.now();

        // Try to execute a simple operation
        setTimeout(() => {
          const endTime = Date.now();
          const responseTime = endTime - startTime;

          // If response time is too high, page is likely unresponsive
          resolve(responseTime < 1000);
        }, 100);
      });
    });

    expect(isResponsive).toBe(true);
  });

  test('should detect motion component issues', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Check if motion components are present
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();

    if (motionCount > 0) {
      console.log(`Found ${motionCount} motion components`);

      // Test interaction with motion components
      const firstMotionDiv = motionDivs.first();

      // Try to click on motion component
      await firstMotionDiv.click();

      // Check if page is still responsive after interaction
      const isStillResponsive = await page.evaluate(() => {
        return new Promise(resolve => {
          setTimeout(() => {
            resolve(true);
          }, 100);
        });
      });

      expect(isStillResponsive).toBe(true);
    }
  });

  test('should detect console errors', async ({ page }) => {
    const errors: string[] = [];

    // Capture console errors
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Capture page errors
    page.on('pageerror', error => {
      errors.push(`Page error: ${error.message}`);
    });

    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Check for critical errors
    const criticalErrors = errors.filter(
      error =>
        error.includes('unresponsive') ||
        error.includes('blocked') ||
        error.includes('infinite loop') ||
        error.includes('stack overflow')
    );

    expect(criticalErrors).toHaveLength(0);

    if (errors.length > 0) {
      console.log('Console errors found:', errors);
    }
  });

  test('should test button interactions', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Find all buttons
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    expect(buttonCount).toBeGreaterThan(0);

    // Test each button
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const buttonText = await button.textContent();

      console.log(`Testing button: ${buttonText}`);

      // Click button
      await button.click();

      // Wait a bit to see if page becomes unresponsive
      await page.waitForTimeout(500);

      // Test if page is still responsive
      const isResponsive = await page.evaluate(() => {
        return new Promise(resolve => {
          setTimeout(() => {
            resolve(true);
          }, 100);
        });
      });

      expect(isResponsive).toBe(true);
    }
  });

  test('should detect memory leaks', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Get initial memory usage
    const initialMemory = await page.evaluate(() => {
      return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
    });

    // Perform some interactions
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < Math.min(buttonCount, 5); i++) {
      await buttons.nth(i).click();
      await page.waitForTimeout(200);
    }

    // Wait for potential memory cleanup
    await page.waitForTimeout(2000);

    // Get final memory usage
    const finalMemory = await page.evaluate(() => {
      return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
    });

    if (initialMemory > 0 && finalMemory > 0) {
      const memoryIncrease = finalMemory - initialMemory;
      const memoryIncreasePercent = (memoryIncrease / initialMemory) * 100;

      console.log(
        `Memory increase: ${memoryIncrease} bytes (${memoryIncreasePercent.toFixed(2)}%)`
      );

      // Memory increase should be reasonable (less than 50%)
      expect(memoryIncreasePercent).toBeLessThan(50);
    }
  });
});
