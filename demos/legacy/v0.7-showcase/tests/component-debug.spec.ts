import { test, expect } from '@playwright/test';

test.describe('Component Debug', () => {
  test('debug ReactiveMotionDiv component', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Find the Spring Physics demo
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    await expect(springDemo).toBeVisible();

    // Find the animated element
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    await expect(animatedElement).toBeVisible();

    // Check all attributes of the element
    const attributes = await animatedElement.evaluate(el => {
      const attrs = {};
      for (let i = 0; i < el.attributes.length; i++) {
        const attr = el.attributes[i];
        attrs[attr.name] = attr.value;
      }
      return attrs;
    });

    console.log('Element attributes:', attributes);

    // Check if it's a ReactiveMotionDiv by looking for specific attributes
    const hasDataMotion = attributes['data-motion'] !== undefined;
    const hasStyle = attributes['style'] !== undefined;

    console.log('Has data-motion:', hasDataMotion);
    console.log('Has style attribute:', hasStyle);

    // Check the element's tag name
    const tagName = await animatedElement.evaluate(el => el.tagName);
    console.log('Tag name:', tagName);

    // Check if the element has any special classes or data attributes
    const className = await animatedElement.evaluate(el => el.className);
    console.log('Class name:', className);

    // Check if there are any elements with data-motion attribute
    const motionElements = await page.locator('[data-motion]').count();
    console.log('Elements with data-motion attribute:', motionElements);

    // Check if there are any elements with specific motion-related attributes
    const motionDivs = await page.locator('div[data-motion]').count();
    console.log('Divs with data-motion attribute:', motionDivs);
  });
});
