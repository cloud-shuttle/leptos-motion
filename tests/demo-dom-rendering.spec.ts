import { test, expect } from '@playwright/test';

test.describe('Demo DOM Rendering Tests', () => {
  test('should render basic HTML structure', async ({ page }) => {
    await page.goto('/');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Check that basic HTML structure exists
    const body = await page.$('body');
    expect(body).toBeTruthy();

    // Check that the page has some content
    const bodyText = await page.textContent('body');
    expect(bodyText).toBeTruthy();
    expect(bodyText!.length).toBeGreaterThan(0);
  });

  test('should have WASM loading script', async ({ page }) => {
    await page.goto('/');

    // Check that the WASM script is present
    const scriptTags = await page.$$('script[type="module"]');
    expect(scriptTags.length).toBeGreaterThan(0);

    // Check that the script contains the correct WASM file reference
    const scriptContent = await page.content();
    expect(scriptContent).toContain('comprehensive-demo-cbb9f9e91b5563c5.js');
    expect(scriptContent).toContain('comprehensive-demo-cbb9f9e91b5563c5_bg.wasm');
  });

  test('should load WASM files successfully', async ({ page }) => {
    const requests: string[] = [];
    page.on('request', request => {
      requests.push(request.url());
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check that WASM files were requested
    const wasmJsRequest = requests.find(url =>
      url.includes('comprehensive-demo-cbb9f9e91b5563c5.js')
    );
    const wasmBgRequest = requests.find(url =>
      url.includes('comprehensive-demo-cbb9f9e91b5563c5_bg.wasm')
    );

    expect(wasmJsRequest).toBeTruthy();
    expect(wasmBgRequest).toBeTruthy();
  });

  test('should have console logs indicating successful mounting', async ({ page }) => {
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(msg.text());
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check for successful mounting logs
    const mountingLogs = consoleMessages.filter(
      msg =>
        msg.includes('Leptos Motion') &&
        (msg.includes('mounted successfully') || msg.includes('completed successfully'))
    );

    expect(mountingLogs.length).toBeGreaterThan(0);
  });

  test('should have DOM elements after WASM loads', async ({ page }) => {
    await page.goto('/');

    // Wait for WASM to load and mount
    await page.waitForFunction(
      () => {
        return window.wasmBindings !== undefined;
      },
      { timeout: 10000 }
    );

    // Wait a bit more for DOM rendering
    await page.waitForTimeout(2000);

    // Check that we have some DOM elements
    const allElements = await page.$$('*');
    expect(allElements.length).toBeGreaterThan(10); // Should have more than just basic HTML

    // Check for specific elements that should be rendered
    const divs = await page.$$('div');
    expect(divs.length).toBeGreaterThan(0);
  });

  test('should have interactive elements', async ({ page }) => {
    await page.goto('/');

    // Wait for WASM to load
    await page.waitForFunction(
      () => {
        return window.wasmBindings !== undefined;
      },
      { timeout: 10000 }
    );

    // Wait for DOM rendering
    await page.waitForTimeout(2000);

    // Check for buttons or interactive elements
    const buttons = await page.$$('button');
    const clickableElements = await page.$$('[onclick], [role="button"], button, a');

    expect(clickableElements.length).toBeGreaterThan(0);
  });

  test('should not have JavaScript errors', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', error => {
      errors.push(error.message);
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Filter out expected warnings
    const criticalErrors = errors.filter(
      error =>
        !error.includes('integrity') && !error.includes('favicon') && !error.includes('preload')
    );

    expect(criticalErrors).toHaveLength(0);
  });

  test('should have proper CSS styles', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check that the page has styles
    const body = await page.$('body');
    const computedStyle = await body?.evaluate(el => {
      const styles = window.getComputedStyle(el);
      return {
        fontFamily: styles.fontFamily,
        backgroundColor: styles.backgroundColor,
        color: styles.color,
      };
    });

    expect(computedStyle).toBeTruthy();
    expect(computedStyle!.fontFamily).toBeTruthy();
  });

  test('should be responsive', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Test different viewport sizes
    await page.setViewportSize({ width: 320, height: 568 }); // Mobile
    await page.waitForTimeout(500);

    const mobileBody = await page.$('body');
    expect(mobileBody).toBeTruthy();

    await page.setViewportSize({ width: 1024, height: 768 }); // Desktop
    await page.waitForTimeout(500);

    const desktopBody = await page.$('body');
    expect(desktopBody).toBeTruthy();
  });
});
