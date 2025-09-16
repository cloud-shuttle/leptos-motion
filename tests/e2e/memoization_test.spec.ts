import { test, expect } from '@playwright/test';

test.describe('Memoization Test', () => {
  test('should load and display the memoization test correctly', async ({ page }) => {
    // Navigate to the demo page
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the WASM module to load and initialize
    await page.waitForTimeout(3000);

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('Memoization Test');

    // Check if the test description is visible
    await expect(
      page.locator('text=Testing memoization with create_memo for performance optimization')
    ).toBeVisible();

    // Check if both buttons are present
    await expect(page.locator('button').filter({ hasText: 'Increment Counter' })).toBeVisible();
    await expect(
      page.locator('button').filter({ hasText: 'Update Expensive Value' })
    ).toBeVisible();

    // Check if both animation boxes are present
    await expect(page.locator('text=Memoized Animation')).toBeVisible();
    await expect(page.locator('text=Non-Memoized (for comparison)')).toBeVisible();

    // Check if the animation boxes contain their text
    await expect(page.locator('div').filter({ hasText: 'Memoized' }).first()).toBeVisible();
    await expect(page.locator('div').filter({ hasText: 'Non-Memoized' }).first()).toBeVisible();

    // Test button interactions
    const incrementButton = page.locator('button').filter({ hasText: 'Increment Counter' });
    const expensiveButton = page.locator('button').filter({ hasText: 'Update Expensive Value' });

    // Click increment button and verify counter updates
    await incrementButton.click();
    await expect(incrementButton).toContainText('Increment Counter: 1');

    // Click expensive value button and verify it updates
    await expensiveButton.click();
    await expect(expensiveButton).toContainText('Update Expensive Value: 0.1');

    console.log('✅ Memoization Test loaded successfully!');
  });
});
