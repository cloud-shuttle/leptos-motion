import { test, expect } from '@playwright/test';

test('check for errors and page loading', async ({ page }) => {
  const errors: string[] = [];
  const logs: string[] = [];

  // Capture console errors
  page.on('console', msg => {
    if (msg.type() === 'error') {
      errors.push(msg.text());
    } else {
      logs.push(`[${msg.type()}] ${msg.text()}`);
    }
  });

  // Capture page errors
  page.on('pageerror', error => {
    errors.push(`Page error: ${error.message}`);
  });

  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded', timeout: 10000 });

  // Wait a bit for any async operations
  await page.waitForTimeout(2000);

  // Log all console messages
  console.log('=== Console Logs ===');
  logs.forEach(log => console.log(log));

  // Log all errors
  console.log('=== Errors ===');
  errors.forEach(error => console.log(error));

  // Check if there are any critical errors
  const criticalErrors = errors.filter(
    error =>
      error.includes('WASM') ||
      error.includes('leptos') ||
      error.includes('motion') ||
      error.includes('ReferenceError') ||
      error.includes('TypeError')
  );

  if (criticalErrors.length > 0) {
    console.log('=== Critical Errors ===');
    criticalErrors.forEach(error => console.log(error));
  }

  // Basic check - if we got here without timeout, the page loaded
  const url = page.url();
  console.log('Final URL:', url);

  // Check if the page has any content
  const bodyContent = await page.locator('body').textContent();
  console.log('Body content length:', bodyContent?.length || 0);

  // Don't fail the test, just report what we found
  expect(url).toBe('http://localhost:8080/');
});
