import { test, expect } from '@playwright/test';

test('Debug WASM Loading', async ({ page }) => {
  // Capture console logs
  const logs: string[] = [];
  const errors: string[] = [];

  page.on('console', msg => {
    const text = msg.text();
    if (msg.type() === 'error') {
      errors.push(text);
    } else {
      logs.push(text);
    }
    console.log(`[${msg.type()}] ${text}`);
  });

  page.on('pageerror', error => {
    errors.push(`Page Error: ${error.message}`);
    console.error('Page Error:', error);
  });

  // Navigate to the debug page
  await page.goto('http://localhost:8080/debug.html');

  // Wait for the page to load
  await page.waitForLoadState('networkidle');

  // Wait a bit for WASM to initialize
  await page.waitForTimeout(5000);

  // Check if WASM loaded
  const wasmStatus = await page.textContent('#wasm-status');
  console.log('WASM Status:', wasmStatus);

  // Check console logs
  const consoleLogs = await page.textContent('#console-logs');
  console.log('Console Logs:', consoleLogs);

  // Check error logs
  const errorLogs = await page.textContent('#error-logs');
  console.log('Error Logs:', errorLogs);

  // Check page content
  const pageContent = await page.textContent('#page-content');
  console.log('Page Content:', pageContent);

  // Take a screenshot
  await page.screenshot({ path: 'debug-screenshot.png' });

  // Log all captured logs
  console.log('\n=== CAPTURED LOGS ===');
  logs.forEach(log => console.log('LOG:', log));

  console.log('\n=== CAPTURED ERRORS ===');
  errors.forEach(error => console.log('ERROR:', error));
});
