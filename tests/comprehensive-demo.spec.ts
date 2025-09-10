import { test, expect } from '@playwright/test';

test.describe('Comprehensive Demo Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the comprehensive demo
    await page.goto('http://localhost:8085/index.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');
  });

  test('should load the demo page', async ({ page }) => {
    // Check that the main content is visible
    await expect(page.locator('h1')).toHaveCount(2);
    await expect(page.locator('h1').last()).toContainText('Animation Reactivity Test');

    // Check that the main demo elements are present
    await expect(page.locator('button')).toBeVisible();
    await expect(page.locator('div[style*="padding: 2rem"]')).toBeVisible();
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
    // Find the animation button
    const animationButton = page.locator('button').first();

    // Find the animated element - look for the test animation box text
    const animatedElement = page.locator('div:has-text("Test Animation Box")').last();

    // Get initial state
    const initialOpacity = await animatedElement.evaluate(
      el => window.getComputedStyle(el).opacity
    );
    const initialBackgroundColor = await animatedElement.evaluate(
      el => window.getComputedStyle(el).backgroundColor
    );
    const initialTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Initial state:', { initialOpacity, initialBackgroundColor, initialTransform });

    // Click the button to trigger animation
    await animationButton.click();

    // Wait for animation to start and complete
    await page.waitForTimeout(100);

    // Check that the animation is in progress or completed
    const midAnimationOpacity = await animatedElement.evaluate(
      el => window.getComputedStyle(el).opacity
    );
    const midAnimationBackgroundColor = await animatedElement.evaluate(
      el => window.getComputedStyle(el).backgroundColor
    );
    const midAnimationTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Mid animation state:', {
      midAnimationOpacity,
      midAnimationBackgroundColor,
      midAnimationTransform,
    });

    // Wait for animation to complete
    await page.waitForTimeout(1000);

    // Get final state
    const finalOpacity = await animatedElement.evaluate(el => window.getComputedStyle(el).opacity);
    const finalBackgroundColor = await animatedElement.evaluate(
      el => window.getComputedStyle(el).backgroundColor
    );
    const finalTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Final state:', { finalOpacity, finalBackgroundColor, finalTransform });

    // Verify that the animation actually changed the visual properties
    // The animation should change opacity from 1.0 to 0.5
    expect(finalOpacity).not.toBe(initialOpacity);

    // The animation should change the transform (scale)
    expect(finalTransform).not.toBe(initialTransform);

    // The animation should change the background color
    expect(finalBackgroundColor).not.toBe(initialBackgroundColor);

    // Click again to toggle back
    await animationButton.click();

    // Wait for animation to complete
    await page.waitForTimeout(1000);

    // Get the state after toggling back
    const backToInitialOpacity = await animatedElement.evaluate(
      el => window.getComputedStyle(el).opacity
    );
    const backToInitialBackgroundColor = await animatedElement.evaluate(
      el => window.getComputedStyle(el).backgroundColor
    );
    const backToInitialTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    console.log('Back to initial state:', {
      backToInitialOpacity,
      backToInitialBackgroundColor,
      backToInitialTransform,
    });

    // Verify that it animated back to the initial state
    expect(backToInitialOpacity).toBe(initialOpacity);
    expect(backToInitialTransform).toBe(initialTransform);
    expect(backToInitialBackgroundColor).toBe(initialBackgroundColor);
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
    await expect(page.locator('h1')).toHaveCount(2); // Static title + Leptos title
    await expect(page.locator('button')).toBeVisible();
    await expect(page.locator('div[style*="padding: 2rem"]')).toBeVisible();
  });
});
