import { test, expect } from '@playwright/test';

test('simple page check', async ({ page }) => {
  // Navigate to the demo
  await page.goto('http://localhost:8080');
  
  // Wait for the page to load
  await page.waitForLoadState('networkidle');
  
  // Check if the page title is correct
  const title = await page.title();
  console.log('Page title:', title);
  
  // Check if the app div exists
  const appDiv = page.locator('#app');
  const appExists = await appDiv.count() > 0;
  console.log('App div exists:', appExists);
  
  if (appExists) {
    const appContent = await appDiv.innerHTML();
    console.log('App content length:', appContent.length);
    console.log('App content preview:', appContent.substring(0, 200));
  }
  
  // Check for any showcase cards
  const showcaseCards = page.locator('.showcase-card');
  const cardCount = await showcaseCards.count();
  console.log('Showcase cards found:', cardCount);
  
  // Check for any buttons
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log('Buttons found:', buttonCount);
  
  // Check for any animated elements
  const animatedElements = page.locator('.bg-green-500');
  const animatedCount = await animatedElements.count();
  console.log('Animated elements found:', animatedCount);
  
  // Basic assertions
  expect(appExists).toBe(true);
  expect(cardCount).toBeGreaterThan(0);
});
