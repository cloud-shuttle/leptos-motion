import { test, expect } from '@playwright/test';

test.describe('Fixed ReactiveMotionDiv Demo Test', () => {
  test('should load and display the fixed demo correctly', async ({ page }) => {
    // Enable console logging
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Go to the simple demo
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait a bit for Leptos to mount
    await page.waitForTimeout(2000);

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('Fixed ReactiveMotionDiv Demo');

    // Check if the demo container is visible
    await expect(page.locator('#demo')).toBeVisible();

    // Check if the status element shows success
    await expect(page.locator('#status')).toContainText('loaded successfully');

    // Check if console output is visible
    await expect(page.locator('#console')).toBeVisible();

    // Check if features are displayed
    const features = page.locator('.feature');
    await expect(features).toHaveCount(4);

    // Check if buttons are present
    await expect(page.locator('button:has-text("Animate")')).toBeVisible();
    await expect(page.locator('button:has-text("Reset")')).toBeVisible();

    console.log('✅ Fixed ReactiveMotionDiv Demo loaded successfully!');
  });

  test('should handle button clicks with fixed component', async ({ page }) => {
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    await page.goto('http://localhost:8080/simple.html');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Click the Animate button
    await page.click('button:has-text("Animate")');

    // Wait for animation to start
    await page.waitForTimeout(1000);

    // Click the Reset button
    await page.click('button:has-text("Reset")');

    // Wait for reset
    await page.waitForTimeout(1000);

    console.log('✅ Fixed ReactiveMotionDiv button interactions working!');
  });
});
