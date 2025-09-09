import { test, expect } from '@playwright/test';

test.describe('WASM Debug', () => {
  test('debug WASM loading issues', async ({ page }) => {
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

    // Capture network failures
    page.on('response', response => {
      if (!response.ok()) {
        const errorMsg = `Network Error: ${response.status()} ${response.url()}`;
        errors.push(errorMsg);
        console.log(errorMsg);
      }
    });

    await page.goto('/');
    
    // Wait for network to be idle
    await page.waitForLoadState('networkidle');
    
    // Wait longer for WASM to potentially load
    await page.waitForTimeout(5000);

    // Try to manually load the WASM module
    const wasmLoadResult = await page.evaluate(async () => {
      try {
        const module = await import('./pkg/v0_7_showcase.js');
        console.log('WASM module imported successfully');
        return { success: true, error: null };
      } catch (error) {
        console.log('WASM module import failed:', error);
        return { success: false, error: error.message };
      }
    });

    console.log('WASM load result:', wasmLoadResult);

    // Check if the app div gets populated
    const appContent = await page.evaluate(() => {
      const app = document.getElementById('app');
      return {
        exists: !!app,
        hasChildren: app ? app.children.length > 0 : false,
        innerHTML: app ? app.innerHTML.substring(0, 200) : 'No app div'
      };
    });

    console.log('App content:', appContent);

    // Check for any elements that might indicate the app loaded
    const hasShowcaseGrid = await page.locator('.showcase-grid').count();
    console.log('Showcase grid elements found:', hasShowcaseGrid);

    // Print all logs
    console.log('\n=== All Console Logs ===');
    logs.forEach(log => console.log(log));

    console.log('\n=== All Errors ===');
    errors.forEach(error => console.log(error));

    // Take a screenshot
    await page.screenshot({ path: 'wasm-debug-screenshot.png', fullPage: true });
  });
});
