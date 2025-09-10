import { test, expect } from '@playwright/test';

test('Debug demo loading', async ({ page }) => {
  // Listen to console messages
  const consoleMessages: string[] = [];
  page.on('console', msg => {
    consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
  });

  // Listen to network requests
  const requests: string[] = [];
  page.on('request', request => {
    requests.push(`${request.method()} ${request.url()}`);
  });

  // Listen to page errors
  const errors: string[] = [];
  page.on('pageerror', error => {
    errors.push(`Page error: ${error.message}`);
  });

  await page.goto('/');

  // Wait a bit for everything to load
  await page.waitForTimeout(5000);

  // Log what we found
  console.log('Console messages:', consoleMessages);
  console.log('Network requests:', requests);
  console.log('Page errors:', errors);

  // Check if the page has any content
  const bodyText = await page.textContent('body');
  console.log('Body text:', bodyText?.substring(0, 200));

  // Check if there's an h1 element
  const h1 = await page.$('h1');
  console.log('H1 element exists:', !!h1);
  if (h1) {
    console.log('H1 text:', await h1.textContent());
  }
});
