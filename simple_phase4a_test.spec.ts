import { test, expect } from '@playwright/test';

test.describe('Simple Phase 4A Test', () => {
  test('should load and display the simple test correctly', async ({ page }) => {
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
    await expect(page.locator('h1')).toContainText('AnimateFn Test');

    // Check if the test description is visible
    await expect(
      page.locator(
        'text=Testing if animate prop works (using signal-based instead of function-based)'
      )
    ).toBeVisible();

    // Check if the function box is present (be more specific to avoid multiple matches)
    await expect(page.locator('div').filter({ hasText: 'Function' }).first()).toBeVisible();

    console.log('✅ Simple Phase 4A Test loaded successfully!');
  });
});
