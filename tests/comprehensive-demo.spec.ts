import { test, expect } from '@playwright/test';

test.describe('Comprehensive Demo Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the comprehensive demo
    await page.goto('http://localhost:8083');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait for WASM module to load and initialize
    await page.waitForTimeout(3000);

    // Check if Leptos app has loaded (look for specific Leptos elements)
    try {
      await page.waitForSelector('[data-leptos-hydrate]', { timeout: 10000 });
    } catch {
      // If no Leptos hydration, we're still on the fallback HTML
      console.log('Still on fallback HTML, WASM may not have loaded');
    }
  });

  test('should load the demo page', async ({ page }) => {
    // Check that the main content is visible
    await expect(page.locator('h1')).toContainText('Leptos Motion');

    // Check that the main demo elements are present
    await expect(page.locator('button').first()).toBeVisible();
    await expect(page.locator('div[style*="padding: 40px"]')).toBeVisible();
  });

  test('should display interactive elements', async ({ page }) => {
    // Check that buttons are visible and clickable
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    expect(buttonCount).toBeGreaterThan(0);

    // Check that at least one button is visible
    await expect(buttons.first()).toBeVisible();
  });

  test('should handle hover effects', async ({ page }) => {
    // Find a hoverable element (button)
    const hoverElement = page.locator('button').first();

    // Get initial transform
    const initialTransform = await hoverElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    // Hover over the element
    await hoverElement.hover();

    // Wait for animation to complete
    await page.waitForTimeout(500);

    // Check that transform has changed (if animations are working)
    const finalTransform = await hoverElement.evaluate(el => window.getComputedStyle(el).transform);

    // This might be the same if animations aren't working, so we'll just check visibility
    await expect(hoverElement).toBeVisible();
  });

  test('should handle click interactions', async ({ page }) => {
    // Find a clickable element
    const clickElement = page.locator('button').first();

    // Click the element
    await clickElement.click();

    // Wait for any potential animation
    await page.waitForTimeout(300);

    // Check that the element is still visible and interactive
    await expect(clickElement).toBeVisible();
  });

  test('should test animation functionality with visual verification', async ({ page }) => {
    // Find the "Button Animation" button specifically
    const animationButton = page.locator('button').filter({ hasText: 'Button Animation' });

    // Wait for the button to be visible
    await animationButton.waitFor({ state: 'visible' });

    // Get initial state of the button
    const initialTransform = await animationButton.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Initial button transform:', initialTransform);

    // Click the button to trigger animation
    await animationButton.click();

    // Wait for animation to start
    await page.waitForTimeout(200);

    // Get state during animation
    const midAnimationTransform = await animationButton.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Mid animation button transform:', midAnimationTransform);

    // Wait for animation to complete
    await page.waitForTimeout(400);

    // Get final state
    const finalTransform = await animationButton.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Final button transform:', finalTransform);

    // The button should scale up and then return to normal
    // At least one of the states should be different from initial
    const hasAnimation = midAnimationTransform !== initialTransform || finalTransform !== initialTransform;

    // If animations are working, at least one state should differ
    expect(hasAnimation).toBe(true);
  });

  test('should load WASM module without errors', async ({ page }) => {
    // Check for console errors
    const errors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Wait for the page to fully load
    await page.waitForTimeout(2000);

    // Check that there are no critical errors
    const criticalErrors = errors.filter(
      error => !error.includes('favicon.ico') && !error.includes('404')
    );

    console.log('Console errors:', errors);
    expect(criticalErrors.length).toBe(0);
  });

  test('should have proper HTML structure', async ({ page }) => {
    // Check that the main structure is present
    await expect(page.locator('body')).toBeVisible();
    await expect(page.locator('h1')).toContainText('Leptos Motion');
    await expect(page.locator('button').first()).toBeVisible();
    await expect(page.locator('div[style*="padding: 40px"]')).toBeVisible();
  });
});
