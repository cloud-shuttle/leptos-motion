import { test, expect } from '@playwright/test';

test.describe('Minimal Test Demo', () => {
  test('should load and display the minimal demo correctly', async ({ page }) => {
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
    await expect(page.locator('h1')).toContainText('Minimal Test Demo');

    // Check if the test button is visible
    await expect(page.locator('button:has-text("Test Button")')).toBeVisible();

    console.log('✅ Minimal Test Demo loaded successfully!');
  });
});
