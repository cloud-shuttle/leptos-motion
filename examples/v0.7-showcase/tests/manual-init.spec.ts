import { test, expect } from '@playwright/test';

test.describe('Manual WASM Initialization', () => {
  test('manually initialize the WASM app', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Try to manually call the main function
    const initResult = await page.evaluate(async () => {
      try {
        // Import the WASM module
        const wasmModule = await import('./pkg/v0_7_showcase.js');
        
        // Check if main function exists
        if (typeof wasmModule.main === 'function') {
          console.log('Calling main function manually');
          wasmModule.main();
          return { success: true, error: null };
        } else {
          console.log('No main function found in module');
          console.log('Available exports:', Object.keys(wasmModule));
          return { success: false, error: 'No main function found' };
        }
      } catch (error) {
        console.log('Error calling main:', error);
        return { success: false, error: error.message };
      }
    });

    console.log('Manual init result:', initResult);

    // Wait a bit for the app to potentially mount
    await page.waitForTimeout(2000);

    // Check if the app mounted
    const appContent = await page.evaluate(() => {
      const app = document.getElementById('app');
      return {
        exists: !!app,
        hasChildren: app ? app.children.length > 0 : false,
        innerHTML: app ? app.innerHTML.substring(0, 500) : 'No app div'
      };
    });

    console.log('App content after manual init:', appContent);

    // Check for showcase grid
    const showcaseGrid = await page.locator('.showcase-grid').count();
    console.log('Showcase grid elements found:', showcaseGrid);

    // Take a screenshot
    await page.screenshot({ path: 'manual-init-screenshot.png', fullPage: true });
  });
});
