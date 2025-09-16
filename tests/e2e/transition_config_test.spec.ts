import { test, expect } from '@playwright/test';

test.describe('TransitionConfig Test', () => {
  test('should load and display the transition config test correctly', async ({ page }) => {
    // Navigate to the demo page
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the WASM module to load and initialize
    await page.waitForTimeout(3000);

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('TransitionConfig Test');

    // Check if the test description is visible
    await expect(page.locator('text=Testing timing controls for animations')).toBeVisible();

    // Check if both animation boxes are present
    await expect(page.locator('text=Fast Animation (100ms)')).toBeVisible();
    await expect(page.locator('text=Slow Animation (2000ms)')).toBeVisible();

    // Check if the animation boxes contain their text
    await expect(page.locator('div').filter({ hasText: 'Fast' }).first()).toBeVisible();
    await expect(page.locator('div').filter({ hasText: 'Slow' }).first()).toBeVisible();

    console.log('✅ TransitionConfig Test loaded successfully!');
  });
});
