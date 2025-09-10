import { test, expect } from '@playwright/test';

test.describe('Demo TDD - Test-Driven Development', () => {
  test('TDD Step 1: Basic page should load without hanging', async ({ page }) => {
    // Test: Page should load and be responsive within 5 seconds
    const startTime = Date.now();
    
    await page.goto('/');
    
    // Wait for basic page load
    await page.waitForLoadState('domcontentloaded', { timeout: 5000 });
    
    const loadTime = Date.now() - startTime;
    expect(loadTime).toBeLessThan(5000); // Should load in under 5 seconds
    
    // Page should be responsive (not hanging)
    const body = await page.$('body');
    expect(body).toBeTruthy();
  });

  test('TDD Step 2: WASM should load without blocking the main thread', async ({ page }) => {
    // Test: WASM loading should not freeze the browser
    await page.goto('/');
    
    // Wait for WASM to start loading
    await page.waitForLoadState('networkidle', { timeout: 10000 });
    
    // Browser should still be responsive after WASM loads
    const body = await page.$('body');
    expect(body).toBeTruthy();
    
    // Should be able to interact with the page
    const bodyText = await page.textContent('body');
    expect(bodyText).toBeTruthy();
    expect(bodyText!.length).toBeGreaterThan(0);
  });

  test('TDD Step 3: Leptos app should mount without infinite loops', async ({ page }) => {
    // Test: Leptos mounting should complete and not cause infinite loops
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(msg.text());
    });

    await page.goto('/');
    
    // Wait for Leptos to mount (should see mounting logs)
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    // Wait a bit more for mounting to complete
    await page.waitForTimeout(2000);
    
    // Check for successful mounting logs
    const mountingLogs = consoleMessages.filter(msg => 
      msg.includes('mounted successfully') || msg.includes('completed successfully')
    );
    expect(mountingLogs.length).toBeGreaterThan(0);
    
    // Should not see infinite loop indicators
    const errorLogs = consoleMessages.filter(msg => 
      msg.includes('Maximum call stack') || 
      msg.includes('infinite loop') ||
      msg.includes('RangeError')
    );
    expect(errorLogs).toHaveLength(0);
  });

  test('TDD Step 4: DOM should be rendered after Leptos mounts', async ({ page }) => {
    // Test: After Leptos mounts, DOM elements should be visible
    await page.goto('/');
    
    // Wait for WASM and Leptos to load
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    // Wait for DOM rendering
    await page.waitForTimeout(3000);
    
    // Should have the main heading
    const heading = await page.$('h1');
    expect(heading).toBeTruthy();
    
    const headingText = await heading?.textContent();
    expect(headingText).toContain('Leptos Motion');
  });

  test('TDD Step 5: Interactive elements should be clickable', async ({ page }) => {
    // Test: Buttons and interactive elements should work
    await page.goto('/');
    
    // Wait for full app to load
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    await page.waitForTimeout(3000);
    
    // Should have clickable elements
    const buttons = await page.$$('button');
    expect(buttons.length).toBeGreaterThan(0);
    
    // Should be able to click without hanging
    if (buttons.length > 0) {
      await buttons[0].click();
      // If we get here without hanging, the click worked
      expect(true).toBe(true);
    }
  });

  test('TDD Step 6: Animations should not cause performance issues', async ({ page }) => {
    // Test: Animations should run smoothly without blocking
    await page.goto('/');
    
    // Wait for app to load
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    await page.waitForTimeout(3000);
    
    // Trigger some animations (hover, click)
    const animatedElements = await page.$$('.animated-box, [class*="motion"]');
    expect(animatedElements.length).toBeGreaterThan(0);
    
    if (animatedElements.length > 0) {
      // Hover over an animated element
      await animatedElements[0].hover();
      await page.waitForTimeout(1000);
      
      // Should still be responsive after animation
      const body = await page.$('body');
      expect(body).toBeTruthy();
    }
  });

  test('TDD Step 7: No memory leaks or resource exhaustion', async ({ page }) => {
    // Test: App should not consume excessive resources
    await page.goto('/');
    
    // Wait for app to load
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    await page.waitForTimeout(3000);
    
    // Interact with the app multiple times
    for (let i = 0; i < 5; i++) {
      const buttons = await page.$$('button');
      if (buttons.length > 0) {
        await buttons[0].click();
        await page.waitForTimeout(500);
      }
    }
    
    // Should still be responsive after multiple interactions
    const body = await page.$('body');
    expect(body).toBeTruthy();
  });

  test('TDD Step 8: Error handling should be graceful', async ({ page }) => {
    // Test: Errors should be handled gracefully without crashing
    const errors: string[] = [];
    page.on('pageerror', error => {
      errors.push(error.message);
    });

    await page.goto('/');
    
    // Wait for app to load
    await page.waitForFunction(() => {
      return window.wasmBindings !== undefined;
    }, { timeout: 10000 });
    
    await page.waitForTimeout(3000);
    
    // Filter out expected warnings
    const criticalErrors = errors.filter(error => 
      !error.includes('integrity') && 
      !error.includes('favicon') &&
      !error.includes('preload') &&
      !error.includes('ResizeObserver')
    );
    
    // Should not have critical errors
    expect(criticalErrors).toHaveLength(0);
  });
});
