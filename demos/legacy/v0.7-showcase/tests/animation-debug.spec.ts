import { test, expect } from '@playwright/test';

test.describe('Animation Debug', () => {
  test('debug animation properties', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Find the Spring Physics demo
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    await expect(springDemo).toBeVisible();

    // Find the animated element
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    await expect(animatedElement).toBeVisible();

    // Get initial styles
    const initialStyles = await animatedElement.evaluate(el => {
      const computed = window.getComputedStyle(el);
      return {
        transform: computed.transform,
        transition: computed.transition,
        backgroundColor: computed.backgroundColor,
        position: computed.position,
        left: computed.left,
        top: computed.top,
      };
    });

    console.log('Initial styles:', initialStyles);

    // Check if the element has data-motion attribute
    const hasDataMotion = await animatedElement.evaluate(el => el.hasAttribute('data-motion'));
    console.log('Has data-motion attribute:', hasDataMotion);

    // Check inline styles
    const inlineStyles = await animatedElement.evaluate(el => el.getAttribute('style'));
    console.log('Inline styles:', inlineStyles);

    // Click the animate button
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();

    // Wait for animation
    await page.waitForTimeout(1500);

    // Get final styles
    const finalStyles = await animatedElement.evaluate(el => {
      const computed = window.getComputedStyle(el);
      return {
        transform: computed.transform,
        transition: computed.transition,
        backgroundColor: computed.backgroundColor,
        position: computed.position,
        left: computed.left,
        top: computed.top,
      };
    });

    console.log('Final styles:', finalStyles);

    // Check if styles changed
    const stylesChanged =
      initialStyles.transform !== finalStyles.transform ||
      initialStyles.backgroundColor !== finalStyles.backgroundColor;
    console.log('Styles changed:', stylesChanged);

    // Get bounding box
    const finalBox = await animatedElement.boundingBox();
    console.log('Final bounding box:', finalBox);
  });
});
