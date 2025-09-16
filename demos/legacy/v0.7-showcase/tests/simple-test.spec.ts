import { test, expect } from '@playwright/test';

test.describe('Simple Test', () => {
  test('check if simple div appears', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Check if the app div has content
    const appContent = await page.evaluate(() => {
      const app = document.getElementById('app');
      return {
        exists: !!app,
        hasChildren: app ? app.children.length > 0 : false,
        innerHTML: app ? app.innerHTML.substring(0, 500) : 'No app div',
      };
    });

    console.log('App content:', appContent);

    // Check for the simple div
    const simpleDiv = await page.locator('.w-20.h-20.bg-green-500').count();
    console.log('Simple divs found:', simpleDiv);

    // Take a screenshot
    await page.screenshot({ path: 'simple-test-screenshot.png', fullPage: true });
  });
});
