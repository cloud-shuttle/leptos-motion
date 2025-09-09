import { test, expect } from '@playwright/test';

test('debug inline styles during animation', async ({ page }) => {
  // Navigate to the demo
  await page.goto('http://localhost:8080');

  // Wait for the app to load
  await page.waitForSelector('.showcase-card', { timeout: 10000 });

  // Find the spring physics demo button
  const animateButton = page.locator('button').filter({ hasText: 'Animate' }).first();
  await expect(animateButton).toBeVisible();

  // Get the animated element
  const animatedElement = page.locator('.bg-green-500.rounded-full').first();
  await expect(animatedElement).toBeVisible();

  // Get initial inline styles
  const initialInlineStyle = await animatedElement.getAttribute('style');
  console.log('Initial inline style:', initialInlineStyle);

  // Click the animate button
  await animateButton.click();

  // Wait a bit for the animation to apply
  await page.waitForTimeout(200);

  // Get final inline styles
  const finalInlineStyle = await animatedElement.getAttribute('style');
  console.log('Final inline style:', finalInlineStyle);

  // Check if the inline styles changed
  const inlineStylesChanged = initialInlineStyle !== finalInlineStyle;
  console.log('Inline styles changed:', inlineStylesChanged);

  // Also check computed styles
  const initialComputed = await animatedElement.evaluate(el => {
    const computed = window.getComputedStyle(el);
    return {
      transform: computed.transform,
      backgroundColor: computed.backgroundColor,
    };
  });

  await page.waitForTimeout(100);

  const finalComputed = await animatedElement.evaluate(el => {
    const computed = window.getComputedStyle(el);
    return {
      transform: computed.transform,
      backgroundColor: computed.backgroundColor,
    };
  });

  console.log('Initial computed styles:', initialComputed);
  console.log('Final computed styles:', finalComputed);

  // The fix should make inline styles change when the button is clicked
  expect(inlineStylesChanged).toBe(true);
});
