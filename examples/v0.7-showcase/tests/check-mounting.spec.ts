import { test, expect } from '@playwright/test';

test.describe('Check App Mounting', () => {
  test('check where the app is mounting', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Manually initialize
    await page.evaluate(async () => {
      const wasmModule = await import('./pkg/v0_7_showcase.js');
      if (typeof wasmModule.main === 'function') {
        wasmModule.main();
      }
    });

    await page.waitForTimeout(3000);

    // Check all possible mounting locations
    const mountingInfo = await page.evaluate(() => {
      const app = document.getElementById('app');
      const body = document.body;
      
      return {
        appDiv: {
          exists: !!app,
          hasChildren: app ? app.children.length > 0 : false,
          innerHTML: app ? app.innerHTML.substring(0, 200) : 'No app div'
        },
        body: {
          hasChildren: body.children.length,
          innerHTML: body.innerHTML.substring(0, 500)
        },
        showcaseGrids: document.querySelectorAll('.showcase-grid').length,
        showcaseCards: document.querySelectorAll('.showcase-card').length,
        allDivs: document.querySelectorAll('div').length
      };
    });

    console.log('Mounting info:', mountingInfo);

    // Check if there are any elements with motion-related classes
    const motionElements = await page.evaluate(() => {
      const elements = document.querySelectorAll('*');
      const motionElements = [];
      
      for (const el of elements) {
        const classes = el.getAttribute('class') || '';
        if (classes.includes('w-20') || classes.includes('w-24') || 
            classes.includes('bg-green-500') || classes.includes('bg-purple-500')) {
          motionElements.push({
            tagName: el.tagName,
            classes: classes,
            innerHTML: el.innerHTML.substring(0, 100)
          });
        }
      }
      
      return motionElements;
    });

    console.log('Motion elements found:', motionElements);

    // Take a screenshot
    await page.screenshot({ path: 'mounting-check-screenshot.png', fullPage: true });
  });
});
