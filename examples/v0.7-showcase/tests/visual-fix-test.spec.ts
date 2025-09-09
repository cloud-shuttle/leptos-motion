import { test, expect } from '@playwright/test';

test('test visual animation fix', async ({ page }) => {
  const logs: string[] = [];

  // Capture console logs
  page.on('console', msg => {
    logs.push(`[${msg.type()}] ${msg.text()}`);
  });

  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

  // Wait for the app to initialize
  await page.waitForTimeout(3000);

  // Find the animated element
  const animatedElement = page.locator('.bg-green-500.rounded-full').first();

  // Get initial inline style
  const initialStyle = await animatedElement.getAttribute('style');
  console.log('Initial inline style:', initialStyle);

  // Try to find and click a button
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log(`Found ${buttonCount} buttons`);

  if (buttonCount > 0) {
    // Click the first button
    await buttons.first().click();
    console.log('Clicked button');

    // Wait for animation to process
    await page.waitForTimeout(1000);

    // Get final inline style
    const finalStyle = await animatedElement.getAttribute('style');
    console.log('Final inline style:', finalStyle);

    // Check if styles changed
    const stylesChanged = initialStyle !== finalStyle;
    console.log('Styles changed:', stylesChanged);

    // Check animation logs
    const animationLogs = logs.filter(
      log => log.includes('Animation triggered') || log.includes('Returning')
    );
    console.log('Animation logs:', animationLogs);

    // The fix should make styles change when the button is clicked
    expect(stylesChanged).toBe(true);

    console.log('✅ VISUAL ANIMATION FIX CONFIRMED: Styles are changing!');
  } else {
    console.log('No buttons found');
  }
});
