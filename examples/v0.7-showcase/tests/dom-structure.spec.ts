import { test, expect } from '@playwright/test';

test('check DOM structure after WASM loads', async ({ page }) => {
  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

  // Wait for WASM to load and render
  await page.waitForTimeout(3000);

  // Check if the app div has content
  const appDiv = page.locator('#app');
  const appContent = await appDiv.innerHTML();

  console.log('App div content length:', appContent.length);
  console.log('App div content:', appContent);

  // Check for showcase cards
  const showcaseCards = page.locator('.showcase-card');
  const cardCount = await showcaseCards.count();
  console.log('Showcase cards found:', cardCount);

  // Check for buttons
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log('Buttons found:', buttonCount);

  // Check for motion divs
  const motionDivs = page.locator('[data-motion]');
  const motionCount = await motionDivs.count();
  console.log('Motion divs found:', motionCount);

  // Basic assertions
  expect(appContent.length).toBeGreaterThan(0);
  expect(cardCount).toBeGreaterThan(0);
  expect(buttonCount).toBeGreaterThan(0);

  console.log('✅ DOM structure looks correct');
});
