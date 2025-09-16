import { test, expect } from '@playwright/test';

test.describe('Mounting Debug', () => {
  test('debug mounting issues', async ({ page }) => {
    const logs: string[] = [];
    const errors: string[] = [];

    // Capture all console messages
    page.on('console', msg => {
      const message = `[${msg.type()}] ${msg.text()}`;
      logs.push(message);
      console.log(message);

      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Capture page errors
    page.on('pageerror', error => {
      const errorMsg = `Page Error: ${error.message}`;
      errors.push(errorMsg);
      console.log(errorMsg);
    });

    await page.goto('/');

    // Check if app div exists before WASM loads
    const appDivBefore = await page.evaluate(() => {
      const app = document.getElementById('app');
      return {
        exists: !!app,
        innerHTML: app ? app.innerHTML : 'No app div',
      };
    });
    console.log('App div before WASM load:', appDivBefore);

    // Wait for network to be idle
    await page.waitForLoadState('networkidle');

    // Wait for WASM to potentially load
    await page.waitForTimeout(3000);

    // Check if app div exists after WASM loads
    const appDivAfter = await page.evaluate(() => {
      const app = document.getElementById('app');
      return {
        exists: !!app,
        hasChildren: app ? app.children.length > 0 : false,
        innerHTML: app ? app.innerHTML.substring(0, 200) : 'No app div',
      };
    });
    console.log('App div after WASM load:', appDivAfter);

    // Try to manually call main function and see what happens
    const manualResult = await page.evaluate(async () => {
      try {
        const wasmModule = await import('./pkg/v0_7_showcase.js');

        // Check if main function exists
        if (typeof wasmModule.main === 'function') {
          console.log('Calling main function manually');
          wasmModule.main();

          // Wait a bit for mounting
          await new Promise(resolve => setTimeout(resolve, 1000));

          const app = document.getElementById('app');
          return {
            success: true,
            appExists: !!app,
            appHasChildren: app ? app.children.length > 0 : false,
            appInnerHTML: app ? app.innerHTML.substring(0, 200) : 'No app div',
          };
        } else {
          return { success: false, error: 'No main function found' };
        }
      } catch (error) {
        console.log('Error in manual call:', error);
        return { success: false, error: error.message };
      }
    });

    console.log('Manual result:', manualResult);

    // Print all logs
    console.log('\n=== All Console Logs ===');
    logs.forEach(log => console.log(log));

    console.log('\n=== All Errors ===');
    errors.forEach(error => console.log(error));

    // Take a screenshot
    await page.screenshot({ path: 'mounting-debug-screenshot.png', fullPage: true });
  });
});
