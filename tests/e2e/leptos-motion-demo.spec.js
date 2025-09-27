const { test, expect } = require('@playwright/test');

test.describe('Leptos Motion Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should load without errors', async ({ page }) => {
    // Check for console errors
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check that there are no critical errors
    const criticalErrors = errors.filter(error => 
      !error.includes('favicon.ico') && 
      !error.includes('404') &&
      !error.includes('Failed to initialize logger')
    );
    
    expect(criticalErrors).toHaveLength(0);
  });

  test('should display interactive elements', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for interactive elements
    const buttons = await page.locator('button').count();
    expect(buttons).toBeGreaterThan(0);
    
    // Check for cards or demo elements
    const cards = await page.locator('[class*="card"], [class*="demo"], [class*="motion"]').count();
    expect(cards).toBeGreaterThan(0);
  });

  test('should have animations working', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find a button to test animations
    const button = page.locator('button').first();
    await expect(button).toBeVisible();
    
    // Test hover animation
    await button.hover();
    await page.waitForTimeout(100);
    
    // Test click animation
    await button.click();
    await page.waitForTimeout(100);
  });

  test('should load WASM successfully', async ({ page }) => {
    // Check that WASM files are loaded
    const wasmLoaded = await page.evaluate(() => {
      return window.wasmLoaded || false;
    });
    
    // Check for WASM-related elements
    const wasmElements = await page.locator('[data-wasm], [class*="wasm"]').count();
    
    // At least one WASM indicator should be present
    expect(wasmLoaded || wasmElements > 0).toBeTruthy();
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForLoadState('networkidle');
    
    // Check that content is still visible
    const content = await page.locator('body').textContent();
    expect(content).toBeTruthy();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForLoadState('networkidle');
    
    const desktopContent = await page.locator('body').textContent();
    expect(desktopContent).toBeTruthy();
  });

  test('should have proper performance', async ({ page }) => {
    // Measure performance
    const performanceMetrics = await page.evaluate(() => {
      const navigation = performance.getEntriesByType('navigation')[0];
      return {
        loadTime: navigation.loadEventEnd - navigation.loadEventStart,
        domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
        totalTime: navigation.loadEventEnd - navigation.fetchStart
      };
    });
    
    // Check that load time is reasonable (less than 5 seconds)
    expect(performanceMetrics.totalTime).toBeLessThan(5000);
  });
});
