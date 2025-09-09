import { test, expect } from '@playwright/test';

test.describe('Demo Inspection', () => {
  test('inspect what is actually rendered', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Take a screenshot
    await page.screenshot({ path: 'demo-screenshot.png', fullPage: true });

    // Log all showcase cards
    const showcaseCards = await page.locator('.showcase-card').all();
    console.log(`Found ${showcaseCards.length} showcase cards`);

    for (let i = 0; i < showcaseCards.length; i++) {
      const card = showcaseCards[i];
      const title = await card.locator('h3').textContent();
      console.log(`Card ${i}: ${title}`);
      
      // Check what elements are inside each card
      const elements = await card.locator('*').all();
      console.log(`  - Contains ${elements.length} elements`);
      
      // Look for any divs with classes
      const divs = await card.locator('div').all();
      for (const div of divs) {
        const classes = await div.getAttribute('class');
        if (classes && classes.includes('w-')) {
          console.log(`  - Found div with classes: ${classes}`);
        }
      }
    }

    // Check if ReactiveMotionDiv components are being rendered
    const motionDivs = await page.locator('[data-motion]').all();
    console.log(`Found ${motionDivs.length} motion divs`);

    // Check for any elements with inline styles
    const styledElements = await page.locator('[style]').all();
    console.log(`Found ${styledElements.length} elements with inline styles`);

    for (const element of styledElements) {
      const style = await element.getAttribute('style');
      if (style && style.includes('transform')) {
        console.log(`  - Found element with transform: ${style}`);
      }
    }
  });
});
