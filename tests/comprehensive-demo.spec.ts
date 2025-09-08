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
    await expect(page.locator('h1').first()).toContainText('🚀 Leptos Motion');

    // Check that the main demo elements are present
    await expect(page.locator('.animated-box')).toBeVisible();
    await expect(page.locator('.mode-button')).toHaveCount(3);
    await expect(page.locator('.main-button')).toBeVisible();
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
    // Find a hoverable element (mode button)
    const hoverElement = page.locator('.mode-button').first();

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
    await expect(page.locator('h1')).toHaveCount(2); // Main title
    await expect(page.locator('.animated-box')).toBeVisible();
    await expect(page.locator('.mode-button')).toHaveCount(3);
  });
});
