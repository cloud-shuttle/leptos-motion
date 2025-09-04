import { test, expect } from '@playwright/test';

test('Debug - Basic page load', async ({ page }) => {
  console.log('Starting debug test...');

  // Listen for console messages
  page.on('console', msg => console.log('PAGE LOG:', msg.text()));
  page.on('pageerror', error => console.log('PAGE ERROR:', error.message));

  // Navigate to the page
  await page.goto('/', { waitUntil: 'networkidle' });
  console.log('Page loaded');

  // Check if the page has content
  const content = await page.content();
  console.log('Page content length:', content.length);
  console.log('Page HTML:', content.substring(0, 500)); // First 500 chars

  // Wait a bit for JS to execute
  await page.waitForTimeout(3000);

  // Check if app div exists (even if empty)
  const appDiv = await page.locator('#app');
  const appExists = await appDiv.count();
  console.log('App div exists:', appExists > 0);

  if (appExists > 0) {
    const appContent = await appDiv.innerHTML();
    console.log('App div content:', appContent);
  }

  // Take a screenshot for debugging
  await page.screenshot({ path: 'debug-screenshot.png' });
  console.log('Screenshot saved');

  // Basic assertion
  expect(appExists).toBeGreaterThan(0);
  console.log('Test completed successfully');
});
