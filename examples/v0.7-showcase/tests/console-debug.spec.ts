import { test, expect } from '@playwright/test';

test.describe('Console Debug', () => {
  test('capture console logs during animation', async ({ page }) => {
    const logs: string[] = [];

    // Capture all console messages
    page.on('console', msg => {
      const message = `[${msg.type()}] ${msg.text()}`;
      logs.push(message);
      console.log(message);
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Find the Spring Physics demo
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    await expect(springDemo).toBeVisible();

    // Find the animated element
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    await expect(animatedElement).toBeVisible();

    console.log('About to click animate button');

    // Click the animate button
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();

    console.log('Clicked animate button, waiting for animation');

    // Wait for animation
    await page.waitForTimeout(2000);

    console.log('Animation should be complete');

    // Print all logs
    console.log('\n=== All Console Logs ===');
    logs.forEach(log => console.log(log));

    // Check if animation logs appeared
    const animationLogs = logs.filter(
      log =>
        log.includes('Animation triggered') ||
        log.includes('Returning active animation') ||
        log.includes('Returning idle animation')
    );

    console.log('\n=== Animation Logs ===');
    animationLogs.forEach(log => console.log(log));

    expect(animationLogs.length).toBeGreaterThan(0);
  });
});
