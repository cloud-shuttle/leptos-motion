import { test, expect } from '@playwright/test';

test('inspect DOM structure to understand the issue', async ({ page }) => {
  const logs: string[] = [];

  // Capture console logs
  page.on('console', msg => {
    logs.push(`[${msg.type()}] ${msg.text()}`);
  });

  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

  // Wait for the app to initialize
  await page.waitForTimeout(3000);

  // Check what's actually in the DOM
  const bodyContent = await page.locator('body').textContent();
  console.log('Body content length:', bodyContent?.length || 0);
  console.log('Body content preview:', bodyContent?.substring(0, 500));

  // Check for any divs
  const allDivs = page.locator('div');
  const divCount = await allDivs.count();
  console.log(`Found ${divCount} divs`);

  // Check for elements with green background
  const greenElements = page.locator('[class*="green"]');
  const greenCount = await greenElements.count();
  console.log(`Found ${greenCount} elements with "green" in class`);

  // Check for elements with bg-green-500
  const bgGreenElements = page.locator('.bg-green-500');
  const bgGreenCount = await bgGreenElements.count();
  console.log(`Found ${bgGreenCount} elements with bg-green-500 class`);

  // Check for rounded elements
  const roundedElements = page.locator('.rounded-full');
  const roundedCount = await roundedElements.count();
  console.log(`Found ${roundedCount} elements with rounded-full class`);

  // Check for any buttons
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log(`Found ${buttonCount} buttons`);

  // Check for any elements with inline styles
  const styledElements = page.locator('[style]');
  const styledCount = await styledElements.count();
  console.log(`Found ${styledCount} elements with inline styles`);

  // Check animation logs
  const animationLogs = logs.filter(
    log => log.includes('Animation triggered') || log.includes('Returning')
  );
  console.log('Animation logs:', animationLogs);

  // Basic assertions
  expect(divCount).toBeGreaterThan(0);
  expect(buttonCount).toBeGreaterThan(0);

  console.log('✅ DOM inspection complete');
});
