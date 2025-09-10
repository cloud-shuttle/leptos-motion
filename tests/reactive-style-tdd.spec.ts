import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
});

test('should apply reactive styles when signal changes', async ({ page }) => {
  // Find the test element
  const testElement = page.locator('#test-element');

  // Get initial state
  const initialOpacity = await testElement.evaluate(el => window.getComputedStyle(el).opacity);
  const initialBackgroundColor = await testElement.evaluate(
    el => window.getComputedStyle(el).backgroundColor
  );
  const initialTransform = await testElement.evaluate(el => window.getComputedStyle(el).transform);

  console.log('Initial state:', {
    opacity: initialOpacity,
    backgroundColor: initialBackgroundColor,
    transform: initialTransform,
  });

  // Click the button to change the signal
  const button = page.locator('button');
  await button.click();

  // Wait for the style to update
  await page.waitForTimeout(100);

  // Get state after signal change
  const afterClickOpacity = await testElement.evaluate(el => window.getComputedStyle(el).opacity);
  const afterClickBackgroundColor = await testElement.evaluate(
    el => window.getComputedStyle(el).backgroundColor
  );
  const afterClickTransform = await testElement.evaluate(
    el => window.getComputedStyle(el).transform
  );

  console.log('After click state:', {
    opacity: afterClickOpacity,
    backgroundColor: afterClickBackgroundColor,
    transform: afterClickTransform,
  });

  // The test should fail initially because styles aren't updating
  // This is our failing test that we need to make pass
  expect(afterClickOpacity).not.toBe(initialOpacity);
  expect(afterClickBackgroundColor).not.toBe(initialBackgroundColor);
  expect(afterClickTransform).not.toBe(initialTransform);
});

test('should have proper HTML structure', async ({ page }) => {
  // Check that the main structure is present
  await expect(page.locator('body')).toBeVisible();
  await expect(page.locator('h1').last()).toContainText('Reactive Style Test (TDD)');
  await expect(page.locator('button')).toBeVisible();
  await expect(page.locator('#test-element')).toBeVisible();
});
