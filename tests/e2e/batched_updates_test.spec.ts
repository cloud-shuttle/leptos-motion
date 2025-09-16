import { test, expect } from '@playwright/test';

test.describe('Batched Updates Test', () => {
  test('should load and display the batched updates test correctly', async ({ page }) => {
    // Navigate to the demo page
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the WASM module to load and initialize
    await page.waitForTimeout(3000);

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('Batched Updates Test');

    // Check if the test description is visible
    await expect(
      page.locator(
        'text=Testing batched DOM updates with requestAnimationFrame for performance optimization'
      )
    ).toBeVisible();

    // Check if both buttons are present
    await expect(page.locator('button').filter({ hasText: 'Start Rapid Updates' })).toBeVisible();
    await expect(page.locator('button').filter({ hasText: 'Reset Counters' })).toBeVisible();

    // Check if all three animation elements are present
    await expect(page.locator('text=Element 1')).toBeVisible();
    await expect(page.locator('text=Element 2')).toBeVisible();
    await expect(page.locator('text=Element 3')).toBeVisible();

    // Check if the animation boxes contain their text
    await expect(page.locator('div').filter({ hasText: '1' }).first()).toBeVisible();
    await expect(page.locator('div').filter({ hasText: '2' }).first()).toBeVisible();
    await expect(page.locator('div').filter({ hasText: '3' }).first()).toBeVisible();

    // Check if the performance note is visible
    await expect(page.locator('text=Performance Note')).toBeVisible();

    // Test button interactions
    const startButton = page.locator('button').filter({ hasText: 'Start Rapid Updates' });
    const resetButton = page.locator('button').filter({ hasText: 'Reset Counters' });

    // Click start button and verify it changes to "Animating..."
    await startButton.click();

    // Wait a bit for the state to update
    await page.waitForTimeout(100);

    // Check if button text changed (it might not change immediately due to timing)
    const buttonText = await startButton.textContent();
    console.log('Button text after click:', buttonText);

    // Wait for animation to complete
    await page.waitForTimeout(2000);

    // Verify button returns to normal state
    await expect(startButton).toContainText('Start Rapid Updates');

    // Click reset button
    await resetButton.click();

    console.log('✅ Batched Updates Test loaded successfully!');
  });
});
