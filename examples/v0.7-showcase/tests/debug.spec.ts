import { test, expect } from '@playwright/test';

test.describe('Debug Demo Issues', () => {
  test('check for JavaScript errors and WASM loading', async ({ page }) => {
    const errors: string[] = [];
    
    // Listen for console errors
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
        console.log('Console error:', msg.text());
      }
    });

    // Listen for page errors
    page.on('pageerror', error => {
      errors.push(error.message);
      console.log('Page error:', error.message);
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Wait a bit more for WASM to initialize
    await page.waitForTimeout(2000);

    console.log('Total errors found:', errors.length);
    if (errors.length > 0) {
      console.log('Errors:', errors);
    }

    // Check if WASM loaded
    const wasmLoaded = await page.evaluate(() => {
      return typeof window !== 'undefined' && 
             typeof (window as any).wasm !== 'undefined';
    });
    console.log('WASM loaded:', wasmLoaded);

    // Check if Leptos app mounted
    const appMounted = await page.evaluate(() => {
      const app = document.getElementById('app');
      return app && app.children.length > 0;
    });
    console.log('App mounted:', appMounted);

    // Check what's in the app div
    const appContent = await page.evaluate(() => {
      const app = document.getElementById('app');
      return app ? app.innerHTML.substring(0, 500) : 'No app div found';
    });
    console.log('App content preview:', appContent);

    // Check if any ReactiveMotionDiv components exist
    const motionDivs = await page.evaluate(() => {
      const divs = document.querySelectorAll('div');
      let count = 0;
      divs.forEach(div => {
        if (div.getAttribute('class')?.includes('w-20') || 
            div.getAttribute('class')?.includes('w-24')) {
          count++;
        }
      });
      return count;
    });
    console.log('Potential motion divs found:', motionDivs);

    // Take a screenshot for visual inspection
    await page.screenshot({ path: 'debug-screenshot.png', fullPage: true });
  });
});
