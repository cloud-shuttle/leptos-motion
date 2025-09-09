import { test, expect } from '@playwright/test';

test('verify reactive animation fix', async ({ page }) => {
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
  
  // Get initial styles
  const initialStyles = await animatedElement.evaluate(el => {
    const computed = window.getComputedStyle(el);
    return {
      transform: computed.transform,
      backgroundColor: computed.backgroundColor,
      transition: computed.transition
    };
  });
  
  console.log('Initial styles:', initialStyles);
  
  // Click the animate button
  await animateButton.click();
  
  // Wait a bit for the animation to apply
  await page.waitForTimeout(100);
  
  // Get final styles
  const finalStyles = await animatedElement.evaluate(el => {
    const computed = window.getComputedStyle(el);
    return {
      transform: computed.transform,
      backgroundColor: computed.backgroundColor,
      transition: computed.transition
    };
  });
  
  console.log('Final styles:', finalStyles);
  
  // Check if the styles changed (indicating the animation is working)
  const stylesChanged = 
    initialStyles.transform !== finalStyles.transform ||
    initialStyles.backgroundColor !== finalStyles.backgroundColor;
  
  console.log('Styles changed:', stylesChanged);
  
  // The fix should make styles change when the button is clicked
  expect(stylesChanged).toBe(true);
  
  // Verify the transition is present
  expect(finalStyles.transition).toContain('0.5s');
});
