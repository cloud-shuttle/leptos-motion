import { test, expect } from '@playwright/test';

test('debug browser loading issues', async ({ page }) => {
  const errors: string[] = [];
  const logs: string[] = [];

  // Capture console errors and logs
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
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

  // Wait for potential WASM loading
  await page.waitForTimeout(5000);

  // Check if the app div has content
  const appDiv = page.locator('#app');
  const appContent = await appDiv.innerHTML();
  console.log('App div content length:', appContent.length);
  console.log('App div content:', appContent);

  // Check for any showcase cards
  const showcaseCards = page.locator('.showcase-card');
  const cardCount = await showcaseCards.count();
  console.log('Showcase cards found:', cardCount);

  // Check for any buttons
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log('Buttons found:', buttonCount);

  // Log all console messages
  console.log('=== Console Logs ===');
  logs.forEach(log => console.log(log));

  // Log all errors
  console.log('=== Errors ===');
  errors.forEach(error => console.log(error));

  // Check if WASM loaded
  const wasmLoaded = await page.evaluate(() => {
    return typeof window !== 'undefined' && typeof window.wasmLoaded !== 'undefined'
      ? window.wasmLoaded
      : false;
  });
  console.log('WASM loaded flag:', wasmLoaded);

  // Check if the page title is correct
  const title = await page.title();
  console.log('Page title:', title);

  // Basic assertions
  expect(title).toBe('Leptos Motion v0.8.0 - Motion.dev Parity Showcase');

  // The app should have some content
  if (appContent.length === 0) {
    console.log('❌ App div is empty - WASM might not be loading');
  } else {
    console.log('✅ App div has content');
  }

  if (cardCount === 0) {
    console.log('❌ No showcase cards found - components might not be rendering');
  } else {
    console.log('✅ Showcase cards found');
  }

  console.log('=== Debug Summary ===');
  console.log('App content length:', appContent.length);
  console.log('Showcase cards:', cardCount);
  console.log('Buttons:', buttonCount);
  console.log('Errors:', errors.length);
  console.log('Logs:', logs.length);
});
