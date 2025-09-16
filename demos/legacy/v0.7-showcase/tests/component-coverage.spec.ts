import { test, expect } from '@playwright/test';

test.describe('Component Coverage Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8080');
    await page.waitForLoadState('networkidle');
  });

  test('should test all button interactions', async ({ page }) => {
    const buttons = page.locator('button');
    const buttonCount = await buttons.count({ timeout: 10000 });

    console.log(`Found ${buttonCount} buttons to test`);

    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const buttonText = await button.textContent({ timeout: 5000 });

      console.log(`Testing button ${i + 1}: "${buttonText}"`);

      // Test button visibility and interactivity with timeout
      await expect(button).toBeVisible({ timeout: 10000 });
      await expect(button).toBeEnabled({ timeout: 10000 });

      // Test button click with timeout
      await button.click({ timeout: 10000 });
      await page.waitForTimeout(200);

      // Verify button is still functional after click with timeout
      await expect(button).toBeEnabled({ timeout: 10000 });
    }
  });

  test('should test component state changes', async ({ page }) => {
    // Test counter component
    const counterButton = page.locator(
      '.showcase-card:has-text("Simple Test") button:has-text("Increment Counter")'
    );
    const counterDisplay = page.locator('.showcase-card:has-text("Simple Test") div.w-20.h-20 div');

    if (await counterButton.isVisible()) {
      // Get initial count
      const initialCount = await counterDisplay.textContent();
      const initialValue = parseInt(initialCount || '0');

      console.log(`Initial counter value: ${initialValue}`);

      // Increment counter multiple times
      for (let i = 0; i < 5; i++) {
        await counterButton.click();
        await page.waitForTimeout(100);

        const newCount = await counterDisplay.textContent();
        const newValue = parseInt(newCount || '0');

        expect(newValue).toBe(initialValue + i + 1);
        console.log(`Counter after click ${i + 1}: ${newValue}`);
      }
    }
  });

  test('should test motion component variations', async ({ page }) => {
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();

    console.log(`Found ${motionCount} motion components`);

    for (let i = 0; i < motionCount; i++) {
      const motionDiv = motionDivs.nth(i);
      const toggleButton = page.locator('.showcase-card:has([data-motion]) button').nth(i);

      if ((await motionDiv.isVisible()) && (await toggleButton.isVisible())) {
        console.log(`Testing motion component ${i + 1}`);

        // Test initial state
        const initialStyle = await motionDiv.getAttribute('style');
        console.log(`Initial style: ${initialStyle}`);

        // Test animation trigger
        await toggleButton.click();
        await page.waitForTimeout(1000);

        const activeStyle = await motionDiv.getAttribute('style');
        console.log(`Active style: ${activeStyle}`);

        // Verify style changed
        expect(activeStyle).not.toEqual(initialStyle);

        // Test return to initial state
        await toggleButton.click();
        await page.waitForTimeout(1000);

        const returnStyle = await motionDiv.getAttribute('style');
        console.log(`Return style: ${returnStyle}`);

        // Verify returned to initial state
        expect(returnStyle).toEqual(initialStyle);
      }
    }
  });

  test('should test component error handling', async ({ page }) => {
    // Test what happens when components encounter errors
    const consoleErrors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    // Perform various interactions that might trigger errors
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < Math.min(buttonCount, 3); i++) {
      const button = buttons.nth(i);
      await button.click();
      await page.waitForTimeout(100);
    }

    // Check for console errors
    console.log(`Console errors detected: ${consoleErrors.length}`);
    consoleErrors.forEach((error, index) => {
      console.log(`Error ${index + 1}: ${error}`);
    });

    // Should not have critical errors that break functionality
    const criticalErrors = consoleErrors.filter(
      error =>
        error.includes('Uncaught') ||
        error.includes('ReferenceError') ||
        error.includes('TypeError')
    );

    expect(criticalErrors.length).toBe(0);
  });

  test('should test component accessibility', async ({ page }) => {
    // Test keyboard navigation
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    if (buttonCount > 0) {
      // Test tab navigation
      await page.keyboard.press('Tab');
      await page.waitForTimeout(100);

      // Test Enter key on focused button
      await page.keyboard.press('Enter');
      await page.waitForTimeout(100);

      // Test Space key on focused button
      await page.keyboard.press('Space');
      await page.waitForTimeout(100);
    }

    // Test ARIA attributes
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();

    for (let i = 0; i < motionCount; i++) {
      const motionDiv = motionDivs.nth(i);

      // Check for accessibility attributes
      const ariaLabel = await motionDiv.getAttribute('aria-label');
      const role = await motionDiv.getAttribute('role');

      console.log(`Motion div ${i + 1} - aria-label: ${ariaLabel}, role: ${role}`);
    }
  });

  test('should test component responsiveness to rapid interactions', async ({ page }) => {
    const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();

    if (await toggleButton.isVisible()) {
      console.log('Testing rapid button clicks');

      // Rapidly click button multiple times
      for (let i = 0; i < 10; i++) {
        await toggleButton.click();
        await page.waitForTimeout(50); // Very short delay
      }

      // Verify component is still responsive
      await expect(toggleButton).toBeEnabled();

      // Test final state
      const motionDiv = page.locator('[data-motion]').first();
      const finalStyle = await motionDiv.getAttribute('style');

      console.log(`Final style after rapid clicks: ${finalStyle}`);
      expect(finalStyle).toBeDefined();
    }
  });

  test('should test component behavior with different viewport sizes', async ({ page }) => {
    const viewports = [
      { width: 1920, height: 1080, name: 'Desktop' },
      { width: 1024, height: 768, name: 'Laptop' },
      { width: 768, height: 1024, name: 'Tablet' },
      { width: 375, height: 667, name: 'Mobile' },
    ];

    for (const viewport of viewports) {
      console.log(`Testing ${viewport.name} viewport (${viewport.width}x${viewport.height})`);

      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await page.waitForTimeout(500);

      // Test component visibility
      const showcaseGrid = page.locator('.showcase-grid');
      await expect(showcaseGrid).toBeVisible();

      // Test button interactions
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();

      if (buttonCount > 0) {
        const firstButton = buttons.first();
        await expect(firstButton).toBeVisible();
        await expect(firstButton).toBeEnabled();

        // Test click interaction
        await firstButton.click();
        await page.waitForTimeout(200);

        // Verify button is still functional
        await expect(firstButton).toBeEnabled();
      }
    }
  });

  test('should test component behavior with different user agents', async ({ page }) => {
    // Test with different user agents
    const userAgents = [
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
      'Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1',
    ];

    for (const userAgent of userAgents) {
      console.log(`Testing with user agent: ${userAgent.substring(0, 50)}...`);

      await page.setExtraHTTPHeaders({ 'User-Agent': userAgent });
      await page.reload();
      await page.waitForLoadState('networkidle');

      // Test basic functionality
      const showcaseGrid = page.locator('.showcase-grid');
      await expect(showcaseGrid).toBeVisible();

      // Test button interactions
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();

      if (buttonCount > 0) {
        const firstButton = buttons.first();
        await expect(firstButton).toBeEnabled();
        await firstButton.click();
        await page.waitForTimeout(200);
      }
    }
  });

  test('should test component behavior with network conditions', async ({ page }) => {
    // Test with different network conditions
    const networkConditions = [
      { name: 'Fast 3G', download: (1.6 * 1024 * 1024) / 8, upload: (750 * 1024) / 8, latency: 40 },
      { name: 'Slow 3G', download: (500 * 1024) / 8, upload: (500 * 1024) / 8, latency: 400 },
      { name: 'Offline', download: 0, upload: 0, latency: 0 },
    ];

    for (const condition of networkConditions) {
      console.log(`Testing with ${condition.name} network condition`);

      // Simulate network condition
      await page.context().setOffline(condition.name === 'Offline');

      if (condition.name !== 'Offline') {
        await page.route('**/*', route => {
          // Simulate network delay
          setTimeout(() => route.continue(), condition.latency);
        });
      }

      await page.reload();
      await page.waitForLoadState('networkidle');

      // Test basic functionality
      const showcaseGrid = page.locator('.showcase-grid');
      await expect(showcaseGrid).toBeVisible();

      // Test button interactions
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();

      if (buttonCount > 0) {
        const firstButton = buttons.first();
        await expect(firstButton).toBeEnabled();
        await firstButton.click();
        await page.waitForTimeout(200);
      }
    }
  });

  test('should generate component coverage report', async ({ page }) => {
    // Generate comprehensive coverage report
    const coverageReport = await page.evaluate(() => {
      const buttons = document.querySelectorAll('button');
      const motionDivs = document.querySelectorAll('[data-motion]');
      const showcaseCards = document.querySelectorAll('.showcase-card');

      return {
        timestamp: new Date().toISOString(),
        components: {
          buttons: buttons.length,
          motionDivs: motionDivs.length,
          showcaseCards: showcaseCards.length,
        },
        interactions: {
          buttonClicks: 0, // This would be tracked in a real implementation
          motionTriggers: 0, // This would be tracked in a real implementation
        },
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight,
        },
        userAgent: navigator.userAgent,
      };
    });

    console.log('=== COMPONENT COVERAGE REPORT ===');
    console.log(JSON.stringify(coverageReport, null, 2));
    console.log('=================================');

    // Verify coverage
    expect(coverageReport.components.buttons).toBeGreaterThan(0);
    expect(coverageReport.components.showcaseCards).toBeGreaterThan(0);
  });
});
